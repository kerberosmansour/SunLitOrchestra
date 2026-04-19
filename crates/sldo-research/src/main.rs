use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use sldo_common::color::{divider, header, info, success, warn};
use sldo_common::logging::ensure_log_dir;
use sldo_common::preflight;

mod dossier;
mod prompt;
mod research;

/// Cooldown (seconds) inserted between deepening invocations. Mirrors
/// `sldo-plan`'s COOLDOWN_SECS constant — gives Claude rate limits a chance to
/// recover between calls.
const COOLDOWN_SECS: u64 = 5;

/// Generate a research dossier using Claude Code CLI.
///
/// Takes a research prompt (via file or inline arg), explores the topic using
/// Claude Code, and produces a structured dossier ready for use with sldo-plan.
#[derive(Parser, Debug)]
#[command(
    name = "sldo-research",
    about = "Generate a research dossier using Claude Code CLI."
)]
struct Cli {
    /// Path to a file containing the research prompt.
    prompt_file: Option<PathBuf>,

    /// Inline research prompt (alternative to prompt_file).
    #[arg(long)]
    prompt: Option<String>,

    /// Target repository to research in context of.
    #[arg(long = "repo-dir")]
    repo_dir: Option<PathBuf>,

    /// Output dossier path.
    #[arg(short, long, default_value = "output/research-dossier.md")]
    output: PathBuf,

    /// Claude model to use.
    #[arg(short, long, default_value = "claude-opus-4-7")]
    model: String,

    /// Maximum research deepening iterations.
    #[arg(long, default_value_t = 3)]
    max_iterations: u32,

    /// Maximum web search invocations.
    #[arg(long, default_value_t = 5)]
    max_searches: u32,
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Validate exactly one prompt source
    match (&cli.prompt_file, &cli.prompt) {
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either a prompt file or --prompt, not both.");
        }
        (None, None) => {
            anyhow::bail!("Provide a prompt file or --prompt <text>.");
        }
        _ => {}
    }

    header("Research Dossier Generator");
    info(&format!("Output:         {}", cli.output.display()));
    info(&format!("Model:          {}", cli.model));
    info(&format!("Max iterations: {}", cli.max_iterations));
    info(&format!("Max searches:   {}", cli.max_searches));

    // ── Pre-flight checks ────────────────────────────────────────────────
    header("Pre-flight checks");

    let claude_path = preflight::check_claude_installed()?;
    success(&format!("claude CLI found: {}", claude_path.display()));

    // Validate prompt source and read content
    let prompt_content = match (&cli.prompt_file, &cli.prompt) {
        (Some(file), None) => {
            let path = file
                .canonicalize()
                .with_context(|| format!("Prompt file not found: {}", file.display()))?;
            preflight::check_file_exists(&path, "Prompt file")?;
            success(&format!("Prompt file: {}", path.display()));
            std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read prompt file: {}", path.display()))?
        }
        (None, Some(text)) => {
            success("Inline prompt provided.");
            text.clone()
        }
        _ => unreachable!(),
    };

    // Validate repo dir if provided
    let canonical_repo_dir = if let Some(repo) = &cli.repo_dir {
        let repo = repo
            .canonicalize()
            .with_context(|| format!("Repository directory not found: {}", repo.display()))?;
        if !repo.is_dir() {
            anyhow::bail!("Repository directory not found: {}", repo.display());
        }
        success(&format!("Repository: {}", repo.display()));
        let branch = preflight::check_git_safety(&repo)?;
        success(&format!("Branch '{}' — safe to proceed.", branch));
        Some(repo)
    } else {
        None
    };

    divider();

    // ── Prompt construction (M2) ─────────────────────────────────────────
    let exploration_prompt =
        prompt::build_exploration_prompt(&prompt_content, canonical_repo_dir.as_deref());
    let first_line = exploration_prompt.lines().next().unwrap_or("");
    info(&format!(
        "Exploration prompt: {} bytes",
        exploration_prompt.len()
    ));
    info(&format!("Exploration prompt first line: {}", first_line));

    // ── Research loop (M3) ───────────────────────────────────────────────
    let working_dir = canonical_repo_dir
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    let log_dir = ensure_log_dir(&working_dir)?;

    let cfg = research::ResearchConfig {
        prompt_content: prompt_content.clone(),
        repo_dir: canonical_repo_dir,
        output_path: cli.output.clone(),
        model: cli.model.clone(),
        max_iterations: cli.max_iterations,
        cooldown_secs: COOLDOWN_SECS,
        log_dir,
    };

    divider();
    header("Research loop");
    let findings = research::research_loop(&cfg)?;
    info(&format!(
        "Research accumulated {} bytes of findings",
        findings.raw.len()
    ));

    // ── Dossier write & validation (M4) ──────────────────────────────────
    divider();
    header("Dossier");
    dossier::write_dossier(
        &cli.output,
        &prompt_content,
        &findings.raw,
        findings.repo_context.as_deref(),
    )?;
    let dossier_bytes = std::fs::metadata(&cli.output).map(|m| m.len()).unwrap_or(0);
    success(&format!(
        "Dossier written: {} ({} bytes)",
        cli.output.display(),
        dossier_bytes
    ));

    let issues = dossier::validate_dossier(&cli.output);
    if issues.is_empty() {
        success("Dossier validation passed.");
    } else {
        warn(&format!(
            "Dossier validation reported {} issue(s):",
            issues.len()
        ));
        for issue in &issues {
            warn(&format!("  - {}", issue));
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        sldo_common::color::fail(&format!("{:#}", e));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── CLI argument parsing ─────────────────────────────────────────────

    #[test]
    fn help_flag_exits_zero() {
        // Given: binary is built
        // When: --help is passed
        // Then: clap returns DisplayHelp error (which maps to exit 0)
        let result = Cli::try_parse_from(["sldo-research", "--help"]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            clap::error::ErrorKind::DisplayHelp
        );
    }

    #[test]
    fn no_args_parses_no_prompt() {
        // Given: no args
        // When: CLI is parsed
        // Then: prompt_file and prompt are both None (run() will reject this)
        let cli = Cli::try_parse_from(["sldo-research"]).unwrap();
        assert!(cli.prompt_file.is_none());
        assert!(cli.prompt.is_none());
    }

    #[test]
    fn inline_prompt_accepted() {
        // Given: --prompt arg
        // When: CLI is parsed
        // Then: prompt field is set, prompt_file is None
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "add OAuth2"]).unwrap();
        assert_eq!(cli.prompt.as_deref(), Some("add OAuth2"));
        assert!(cli.prompt_file.is_none());
    }

    #[test]
    fn prompt_file_accepted() {
        // Given: positional prompt_file arg
        // When: CLI is parsed
        // Then: prompt_file is set, prompt is None
        let cli = Cli::try_parse_from(["sldo-research", "prompt.txt"]).unwrap();
        assert_eq!(cli.prompt_file, Some(PathBuf::from("prompt.txt")));
        assert!(cli.prompt.is_none());
    }

    #[test]
    fn both_prompt_sources_parsed_for_rejection() {
        // Given: both prompt_file and --prompt provided
        // When: CLI is parsed (clap allows it; run() rejects it)
        // Then: both fields are set — run() will detect the conflict
        let cli = Cli::try_parse_from(["sldo-research", "file.txt", "--prompt", "text"]).unwrap();
        assert!(cli.prompt_file.is_some());
        assert!(cli.prompt.is_some());
    }

    #[test]
    fn default_output_path() {
        // Given: no --output arg
        // When: CLI is parsed
        // Then: output defaults to output/research-dossier.md
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test"]).unwrap();
        assert_eq!(cli.output, PathBuf::from("output/research-dossier.md"));
    }

    #[test]
    fn custom_output_path() {
        // Given: --output custom.md
        // When: CLI is parsed
        // Then: output is custom.md
        let cli =
            Cli::try_parse_from(["sldo-research", "--prompt", "test", "--output", "custom.md"])
                .unwrap();
        assert_eq!(cli.output, PathBuf::from("custom.md"));
    }

    #[test]
    fn default_model() {
        // Given: no --model arg
        // When: CLI is parsed
        // Then: model is claude-opus-4-7
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test"]).unwrap();
        assert_eq!(cli.model, "claude-opus-4-7");
    }

    #[test]
    fn custom_model() {
        // Given: -m flag
        // When: CLI is parsed
        // Then: model is set to the given value
        let cli = Cli::try_parse_from([
            "sldo-research",
            "--prompt",
            "test",
            "-m",
            "claude-haiku-4-5",
        ])
        .unwrap();
        assert_eq!(cli.model, "claude-haiku-4-5");
    }

    #[test]
    fn default_max_iterations() {
        // Given: no --max-iterations arg
        // When: CLI is parsed
        // Then: max_iterations is 3
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test"]).unwrap();
        assert_eq!(cli.max_iterations, 3);
    }

    #[test]
    fn default_max_searches() {
        // Given: no --max-searches arg
        // When: CLI is parsed
        // Then: max_searches is 5
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test"]).unwrap();
        assert_eq!(cli.max_searches, 5);
    }

    #[test]
    fn repo_dir_parsed() {
        // Given: --repo-dir /tmp
        // When: CLI is parsed
        // Then: repo_dir is Some(/tmp)
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test", "--repo-dir", "/tmp"])
            .unwrap();
        assert_eq!(cli.repo_dir, Some(PathBuf::from("/tmp")));
    }

    #[test]
    fn repo_dir_defaults_to_none() {
        // Given: no --repo-dir
        // When: CLI is parsed
        // Then: repo_dir is None
        let cli = Cli::try_parse_from(["sldo-research", "--prompt", "test"]).unwrap();
        assert!(cli.repo_dir.is_none());
    }
}
