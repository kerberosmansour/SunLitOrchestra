//! `check-clean` subcommand — false-positive scan over a known-clean subset.
//!
//! Per `/slo-critique` eng-1: the default clean-dir is
//! `xtasks/sast-verify/tests/fixtures/clean_subset/`, NOT the host crate's
//! `src/`. Scanning host `src/` is opt-in via explicit `--clean-dir src/` (for
//! "find actual unfixed bugs" use cases) and acceptable to fail on real bugs.
//!
//! Exit codes (per interfaces §1):
//! - `0` zero matches in clean-dir
//! - `2` ≥ 1 false positive
//! - `4` semgrep CLI missing

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::{semgrep_runner, GlobalOpts};

pub fn run(rule_path: &Path, clean_dir: Option<&Path>, opts: &GlobalOpts) -> Result<i32> {
    if which::which("semgrep").is_err() && opts.semgrep_bin.is_none() {
        emit(opts, "fail", "semgrep_not_found", "semgrep CLI not on PATH");
        return Ok(4);
    }

    let target = resolve_clean_dir(clean_dir);
    if !target.exists() {
        emit(
            opts,
            "fail",
            "clean_dir_missing",
            &format!(
                "clean dir {} does not exist; default is xtasks/sast-verify/tests/fixtures/clean_subset/ (per /slo-critique eng-1)",
                target.display()
            ),
        );
        return Ok(2);
    }

    let rule_str = rule_path.to_string_lossy();
    let target_str = target.to_string_lossy();
    let args = ["--config", rule_str.as_ref(), "--json", target_str.as_ref()];
    let run = match semgrep_runner::run(&args, opts) {
        Ok(r) => r,
        Err(e) => {
            emit(opts, "fail", "semgrep_invocation_failed", &format!("{e:#}"));
            return Ok(4);
        }
    };

    // Parse the --json output strictly. Per sec-2 we never substring-match
    // raw stdout for verdict.
    let json = match semgrep_runner::parse_json_output(&run.stdout, "check-clean") {
        Ok(v) => v,
        Err(e) => {
            emit(opts, "fail", "json_parse_failed", &format!("{e:#}"));
            return Ok(2);
        }
    };

    let results = json
        .get("results")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    if results == 0 {
        emit(
            opts,
            "pass",
            "zero_fp",
            &format!("scanned {}", target.display()),
        );
        Ok(0)
    } else {
        // Build a short summary of FP locations for the operator.
        let summary = json
            .get("results")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .take(5)
                    .map(|r| {
                        let path = r.get("path").and_then(|v| v.as_str()).unwrap_or("?");
                        let line = r
                            .get("start")
                            .and_then(|s| s.get("line"))
                            .and_then(|n| n.as_u64())
                            .unwrap_or(0);
                        format!("{path}:{line}")
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        emit(
            opts,
            "fail",
            "false_positive",
            &format!(
                "rule fired on {results} site(s) in clean-dir {} (first 5: {summary})",
                target.display()
            ),
        );
        Ok(2)
    }
}

fn resolve_clean_dir(override_dir: Option<&Path>) -> PathBuf {
    if let Some(d) = override_dir {
        return d.to_path_buf();
    }
    // Default to xtasks/sast-verify/tests/fixtures/clean_subset/ relative to the
    // workspace root. Walk up looking for the directory.
    let cwd = std::env::current_dir().unwrap_or_default();
    let mut candidate = cwd.clone();
    for _ in 0..6 {
        let check = candidate
            .join("xtasks")
            .join("sast-verify")
            .join("tests")
            .join("fixtures")
            .join("clean_subset");
        if check.is_dir() {
            return check;
        }
        if !candidate.pop() {
            break;
        }
    }
    cwd.join("xtasks/sast-verify/tests/fixtures/clean_subset")
}

fn emit(opts: &GlobalOpts, status: &str, label: &str, detail: &str) {
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "check-clean",
            "status": status,
            "label": label,
            "detail": detail,
        });
        println!("{v}");
    } else if status == "pass" {
        println!("check-clean: zero FP — {detail}");
    } else {
        eprintln!("check-clean: {status} ({label}): {detail}");
    }
}
