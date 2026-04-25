//! `sast-verify` xtask — deterministic gate for SAST rule packs.
//!
//! Wraps `semgrep --validate` and `semgrep --test` plus a YAML-parse-and-count
//! check (`check-coverage`) and a known-clean-subset false-positive scan
//! (`check-clean`). The `gate` subcommand composes the four primitives in order
//! (validate → test → check-coverage → check-clean) and is the SINGLE
//! deterministic entry point that `/slo-rulegen` and `/slo-ruleverify` shell
//! out to before authorising any rule write.
//!
//! Never reaches the network. Never invokes `slo-rulegen` or any rule-generation
//! path — the xtask is a write-time gate, not a CI rule-gen runner.
//!
//! Exit codes 0–7 are owned by this binary; ≥64 reserved for unrecoverable
//! crashes (panic / signal). See `docs/design/sast-rulegen-skill-pack-interfaces.md` §1.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod check_clean;
mod check_coverage;
mod gate;
mod semgrep_runner;
mod test_cmd;
mod tier_detect;
mod validate;
mod validate_file_paths;
mod yaml_schema;

#[derive(Parser, Debug)]
#[command(
    name = "sast-verify",
    about = "Deterministic gate for SAST rule packs (validate / test / check-coverage / check-clean / gate)",
    version
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,

    /// Override the `semgrep` binary path; defaults to `which semgrep`.
    #[arg(long, global = true)]
    semgrep_bin: Option<PathBuf>,

    /// Pass through to `semgrep --timeout`. Default 30s; minimum 5.
    #[arg(long, global = true, default_value_t = 30)]
    timeout_secs: u64,

    /// Pass through to `semgrep --max-target-bytes`. Default 1 MB.
    #[arg(long, global = true, default_value_t = 1_000_000)]
    max_target_bytes: u64,

    /// Emit structured JSON to stdout instead of human-readable.
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Validate Semgrep rule YAML syntax (wraps `semgrep --validate`).
    Validate {
        /// Path to a rule YAML file or directory of rules.
        rule_path: PathBuf,
    },
    /// Run paired-fixture fire/silent test (wraps `semgrep --validate` then `semgrep --test`).
    /// Refuses to run `--test` on an invalid rule per Semgrep issue #10319.
    Test {
        /// Path to the rule YAML; paired `<rule-id>.rs` fixture must exist as a sibling.
        rule_path: PathBuf,
    },
    /// Assert `pattern-either` arm count against per-CWE minimum/maximum.
    /// Reads `references/sast/variations/cwe-<NNN>.md` for the floor; default ceiling 25 arms.
    CheckCoverage {
        /// Path to the rule YAML.
        rule_path: PathBuf,
        /// Override the references directory (default: workspace `references/sast/`).
        #[arg(long)]
        references_dir: Option<PathBuf>,
    },
    /// Run rule against a known-clean subset; fail on any false positive.
    /// Default clean dir: `xtasks/sast-verify/tests/fixtures/clean_subset/`.
    /// Per `/slo-critique` eng-1: NEVER defaults to host crate's `src/` (self-poisoning gate).
    CheckClean {
        /// Path to the rule YAML.
        rule_path: PathBuf,
        /// Optional clean-dir override; explicit `--clean-dir src/` is the host-`src/` opt-in.
        #[arg(long)]
        clean_dir: Option<PathBuf>,
    },
    /// Gate composing validate → test → check-coverage → check-clean in order.
    /// Short-circuits on first failure; exit code propagates from the failing sub-step.
    /// This is the contract `/slo-rulegen` and `/slo-ruleverify` invoke.
    Gate {
        /// Path to the rule YAML.
        rule_path: PathBuf,
        /// Override the references directory.
        #[arg(long)]
        references_dir: Option<PathBuf>,
        /// Override the clean dir.
        #[arg(long)]
        clean_dir: Option<PathBuf>,
    },
    /// Detect the current repo's confidentiality tier from `git remote get-url origin`.
    /// Returns `Confidential` by default (default-deny on parse failure / no remote / unknown host).
    /// Used by `/slo-rulegen --extend` to choose the corpus output tier.
    DetectTier {
        /// Repo root to inspect (default: current working directory).
        #[arg(long)]
        repo_dir: Option<PathBuf>,
    },
    /// Validate a comma-separated list of file paths against a repo root.
    /// Each path must canonicalize to a location under the repo root.
    /// Rejects absolute paths, `..` segments, missing files, and symlinks
    /// pointing outside the repo. Used by `/slo-rulegen --extend` to gate
    /// `--file-paths` input before any LLM invocation. Exit 4 on rejection.
    ValidateFilePaths {
        /// Comma-separated list of repo-relative paths to validate.
        csv: String,
        /// Repo root to check against (default: current working directory).
        #[arg(long)]
        repo_dir: Option<PathBuf>,
    },
}

#[derive(Debug, Clone)]
pub struct GlobalOpts {
    pub semgrep_bin: Option<PathBuf>,
    pub timeout_secs: u64,
    pub max_target_bytes: u64,
    pub json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Clamp timeout (per interfaces §1: minimum 5).
    let timeout_secs = cli.timeout_secs.max(5);

    let opts = GlobalOpts {
        semgrep_bin: cli.semgrep_bin.clone(),
        timeout_secs,
        max_target_bytes: cli.max_target_bytes,
        json: cli.json,
    };

    let result: Result<i32> = match cli.cmd {
        Cmd::Validate { rule_path } => validate::run(&rule_path, &opts),
        Cmd::Test { rule_path } => test_cmd::run(&rule_path, &opts),
        Cmd::CheckCoverage {
            rule_path,
            references_dir,
        } => check_coverage::run(&rule_path, references_dir.as_deref(), &opts),
        Cmd::CheckClean {
            rule_path,
            clean_dir,
        } => check_clean::run(&rule_path, clean_dir.as_deref(), &opts),
        Cmd::Gate {
            rule_path,
            references_dir,
            clean_dir,
        } => gate::run(
            &rule_path,
            references_dir.as_deref(),
            clean_dir.as_deref(),
            &opts,
        ),
        Cmd::DetectTier { repo_dir } => tier_detect::run(repo_dir.as_deref(), &opts),
        Cmd::ValidateFilePaths { csv, repo_dir } => {
            validate_file_paths::run(&csv, repo_dir.as_deref(), &opts)
        }
    };

    match result {
        Ok(code) => ExitCode::from(code as u8),
        Err(err) => {
            eprintln!("sast-verify: error: {err:#}");
            ExitCode::from(64)
        }
    }
}
