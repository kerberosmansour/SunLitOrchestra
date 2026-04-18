use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use clap::Parser;

use sldo_common::color::{divider, fail, header, info, success, warn};
use sldo_common::copilot::ClaudeInvocation;
use sldo_common::logging::{ensure_log_dir, LogFile};
use sldo_common::preflight;
use sldo_common::toolflags;

/// Cooldown between steps (seconds).
const COOLDOWN_SECS: u64 = 3;

/// Default output path relative to the repo directory.
const DEFAULT_OUTPUT_SUBPATH: &str = "docs/RUNBOOK.md";

/// Fallback template when the template file is not found on disk.
const FALLBACK_TEMPLATE: &str = r#"# [Runbook Title] — [Project Name]

> **Purpose**: [One-sentence description]
> **How to use**: Work through milestones sequentially.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|

---

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
## Milestone Plan

### Milestone N — [Title]

**Goal**: [description]
**Context**: [description]

#### Files Most Likely Touched
#### Step-by-Step
#### BDD Acceptance Scenarios
#### Regression Tests
#### E2E Runtime Validation
#### Smoke Tests

---

## Threat Model
## Documentation Update Table
"#;

/// Generate a milestone-based runbook from a requirements prompt and a repository.
///
/// Two-step process:
///   Step 1 — Generate the runbook (resumable: continues from existing partial output).
///   Step 2 — Review & correct (ensures nothing is forgotten).
#[derive(Parser, Debug)]
#[command(name = "sldo-plan", about = "Generate a milestone-based runbook using Claude Code CLI.")]
struct Cli {
    /// Path to a text/markdown file describing the desired changes.
    prompt_file: PathBuf,

    /// Path to the target repository to plan changes for.
    repo_dir: PathBuf,

    /// Output runbook path (default: <repo>/docs/RUNBOOK.md).
    #[arg(short, long, default_value = None)]
    output: Option<PathBuf>,

    /// Copilot model to use.
    #[arg(short, long, default_value = "claude-opus-4.6")]
    model: String,
}

/// Resolve the output path: if provided, resolve relative to repo_dir;
/// otherwise default to `<repo_dir>/docs/RUNBOOK.md`.
fn resolve_output_path(output: Option<&Path>, repo_dir: &Path) -> PathBuf {
    match output {
        Some(p) if p.is_absolute() => p.to_path_buf(),
        Some(p) => repo_dir.join(p),
        None => repo_dir.join(DEFAULT_OUTPUT_SUBPATH),
    }
}

/// Read the runbook template from disk, falling back to a built-in template.
pub fn read_template(template_path: &Path) -> String {
    match std::fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(_) => FALLBACK_TEMPLATE.to_string(),
    }
}

// ─── Step 1: Generate ────────────────────────────────────────────────────────

/// Build the Step 1 prompt: generate the runbook in resumable chunks.
///
/// If the output file already exists with partial content, the prompt instructs
/// Copilot to continue from where it left off rather than starting over.
/// Includes a threat model section for application features (not CI/CD or cloud).
pub fn build_generate_prompt(
    prompt_content: &str,
    template: &str,
    output_path: &Path,
    existing_content: Option<&str>,
) -> String {
    let resume_section = match existing_content {
        Some(content) if content.len() > 100 => format!(
            r#"

## RESUME FROM EXISTING DRAFT

A previous run was interrupted. The file `{output}` already contains a partial
runbook ({len} bytes). Read it, then **continue from where it left off**.
Do NOT rewrite sections that are already complete — only fill in the remaining
sections and milestones. Overwrite `{output}` with the combined result."#,
            output = output_path.display(),
            len = content.len(),
        ),
        _ => String::new(),
    };

    format!(
        r#"You are an expert software architect and planning agent. Your job is to analyse a
repository and produce a detailed, actionable runbook of milestones that an AI
coding agent can follow to implement the requested changes.

## INPUT

### User Requirements

<requirements>
{prompt_content}
</requirements>

### Runbook Template

The output MUST follow this exact template structure. Fill in every placeholder.
Do not remove any sections — populate them all with concrete, repo-specific content.

<template>
{template}
</template>

## YOUR TASK

1. **Explore the repository thoroughly.** Read key files: README, Cargo.toml /
   package.json / pyproject.toml / Makefile (whichever exist), directory structure,
   architecture docs, existing test structure. Understand the tech stack, project
   layout, build commands, and test commands.

2. **Analyse the requirements** against the current codebase. Identify which files
   and modules need to change, what new files are needed, correct build/test
   commands, dependencies to add, and existing tests that must not break.

3. **Decompose into milestones.** Each milestone must be:
   - Small enough for one focused coding session (1-3 hours of AI agent work)
   - Self-contained: builds, tests pass, and the app works at the end
   - Ordered by dependency — earlier milestones provide foundations for later ones
   - Concrete: reference real file paths, real function names, real test commands
   - Iterative: the application should work after each milestone based on its
     current capabilities — no milestone should leave the app in a broken state

4. **For each milestone, fill in the template completely** with concrete,
   repo-specific content: Goal, Context, Files Most Likely Touched, Step-by-Step,
   BDD Acceptance Scenarios, E2E Runtime Validation, Regression Tests, Smoke Tests.

5. **Include a Threat Model section** in the runbook. This must cover:
   - Security threats to the **application features** being built
   - Input validation, authentication, authorization, data exposure risks
   - Dependency supply-chain risks for new crates/packages being added
   - Do NOT threat-model CI/CD pipelines, cloud infrastructure, or deployment
   - For each threat: description, severity (high/medium/low), and mitigation
     that should be implemented within the milestones

6. **Write the completed runbook** to: `{output}`
{resume_section}

## HARD RULES

- Explore the repo BEFORE writing the runbook. Do not guess file paths or commands.
- Every file path must be real (or clearly marked as new).
- Every build/test command must be the actual command for this project.
- Milestones must be strictly sequential — no circular dependencies.
- Each milestone must leave the project in a buildable, testable, working state.
- Do NOT implement any code changes. PLANNING only.
- Do NOT modify any existing source files in the repository.
- Do NOT commit or push anything.
- The runbook file at `{output}` is the ONLY file you should create or modify."#,
        output = output_path.display(),
    )
}

// ─── Step 2: Review & Correct ────────────────────────────────────────────────

/// Build the Step 2 prompt: re-read the runbook and fix anything forgotten.
///
/// Checks for: .gitignore updates, cargo fmt / cargo clippy / cargo audit steps,
/// smoke tests proving each milestone is iterative and the app works.
pub fn build_review_prompt(output_path: &Path) -> String {
    format!(
        r#"You are a meticulous review agent. A runbook has been generated at `{output}`.
Your job is to re-read it and fix anything that was forgotten or is incomplete.

## YOUR TASK

1. **Read the entire runbook** at `{output}`.

2. **Check every milestone** for these commonly forgotten items and add them
   if missing:

   a. **`.gitignore`** — If the milestone creates new build artifacts, output
      directories, or generated files, the step-by-step must include updating
      `.gitignore`. Review what files the milestone produces and ensure they
      are covered.

   b. **`cargo fmt`** (or the project's equivalent formatter) — Every milestone's
      step-by-step must end with running the formatter. If the project uses
      Rust, that means `cargo fmt --all`. For JS/TS, `npx prettier --write .`
      or equivalent.

   c. **`cargo clippy`** (or equivalent linter) — Every milestone must run the
      linter after implementation. For Rust: `cargo clippy --workspace -- -D warnings`.

   d. **`cargo audit`** (or equivalent) — If the milestone adds new dependencies,
      the step-by-step must include `cargo audit` (or `npm audit`, etc.) to
      check for known vulnerabilities.

   e. **Smoke test proving the app works** — Every milestone must have at least
      one smoke test that proves the application works at its current level of
      capability. This is NOT just "cargo test passes" — it must be a concrete
      verification that a user-facing feature or system behavior works. The app
      must be functional after each milestone, not just compilable.

   f. **Iterative capability** — Verify that milestones are truly iterative.
      After milestone 1, the app should do something useful. After milestone 2,
      it should do more. Each milestone should extend — not just scaffold.

3. **Check the Threat Model section** exists and covers application-level threats
   (not CI/CD or cloud platform threats). If missing or incomplete, add it.

4. **Check that all template placeholders are filled in.** No `[description]`,
   `[file path]`, `[One-sentence`, or similar placeholder text should remain.

5. **Write the corrected runbook** back to `{output}`. If nothing needs fixing,
   write it back unchanged.

## HARD RULES

- Do NOT add new milestones or change the milestone structure.
- Only add missing checklist items, fix forgotten steps, and fill placeholders.
- Do NOT implement any code. REVIEW only.
- Do NOT modify any source files in the repository.
- The runbook file at `{output}` is the ONLY file you should modify."#,
        output = output_path.display(),
    )
}

// ─── Validation ──────────────────────────────────────────────────────────────

/// Required sections that a valid runbook must contain.
const REQUIRED_SECTIONS: &[&str] = &[
    "Milestone Tracker",
    "Pre-Milestone Protocol",
    "Post-Milestone Protocol",
    "Background Context",
    "Current State",
    "BDD Acceptance Scenarios",
    "Threat Model",
];

/// Placeholder patterns that indicate unfilled template content.
const PLACEHOLDER_PATTERNS: &[&str] = &[
    "[file path]",
    "[description]",
    "[One-sentence",
    "[Milestone 1 title]",
    "[Milestone 2 title]",
    "[Milestone 3 title]",
];

/// Validate a generated runbook file.
///
/// Returns a list of issue descriptions. An empty list means the runbook is valid.
pub fn validate_runbook(path: &Path) -> Vec<String> {
    let mut issues = Vec::new();

    // Check file exists
    if !path.exists() {
        issues.push(format!("Runbook file was not created at: {}", path.display()));
        return issues;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            issues.push(format!("Failed to read runbook: {}", e));
            return issues;
        }
    };

    // Check size
    if content.len() < 500 {
        issues.push(format!(
            "Runbook is suspiciously small ({} bytes). May be incomplete.",
            content.len()
        ));
    }

    // Check required sections
    for section in REQUIRED_SECTIONS {
        if !content.contains(section) {
            issues.push(format!("Missing section: {}", section));
        }
    }

    // Check milestone tracker has entries
    let milestone_re = regex::Regex::new(r"^\|\s*\d+\s*\|").unwrap();
    let milestone_count = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with('|')
                && milestone_re.is_match(trimmed)
                && (trimmed.contains("`not_started`")
                    || trimmed.contains("`in_progress`")
                    || trimmed.contains("`done`"))
        })
        .count();

    if milestone_count == 0 {
        issues.push("No milestones found in tracker table.".to_string());
    }

    // Check for unfilled template placeholders
    let placeholder_count: usize = PLACEHOLDER_PATTERNS
        .iter()
        .map(|p| content.matches(p).count())
        .sum();

    if placeholder_count > 0 {
        issues.push(format!(
            "Found {} possible unfilled placeholder(s).",
            placeholder_count
        ));
    }

    // Check that no milestones are marked done (fresh runbook should be all not_started)
    let done_count = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with('|')
                && milestone_re.is_match(trimmed)
                && trimmed.contains("`done`")
        })
        .count();

    if done_count > 0 {
        issues.push(format!(
            "Some milestones are marked 'done' — they should all be 'not_started'. Found {} done.",
            done_count
        ));
    }

    issues
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Resolve paths
    let prompt_file = cli.prompt_file.canonicalize().with_context(|| {
        format!("Prompt file not found: {}", cli.prompt_file.display())
    })?;

    let repo_dir = cli.repo_dir.canonicalize().with_context(|| {
        format!("Repository directory not found: {}", cli.repo_dir.display())
    })?;

    let output_path = resolve_output_path(cli.output.as_deref(), &repo_dir);

    header("Milestone Planner — Runbook Generator");
    info(&format!("Prompt file: {}", prompt_file.display()));
    info(&format!("Repository:  {}", repo_dir.display()));
    info(&format!("Output:      {}", output_path.display()));
    info(&format!("Model:       {}", cli.model));

    // ── Pre-flight checks ────────────────────────────────────────────────
    header("Pre-flight checks");

    let claude_path = preflight::check_claude_installed()?;
    success(&format!("claude CLI found: {}", claude_path.display()));

    preflight::check_file_exists(&prompt_file, "Prompt file")?;
    success(&format!("Prompt file found: {}", prompt_file.display()));

    if !repo_dir.is_dir() {
        anyhow::bail!("Repository directory not found: {}", repo_dir.display());
    }
    success(&format!("Repository directory: {}", repo_dir.display()));

    let branch = preflight::check_git_safety(&repo_dir)?;
    success(&format!("Branch '{}' — safe to proceed.", branch));

    let template_path = repo_dir.join("docs/runbook-template.md");
    let template = read_template(&template_path);
    if template_path.exists() {
        success(&format!("Runbook template: {}", template_path.display()));
    } else {
        warn("Runbook template not found — using built-in template.");
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let log_dir = ensure_log_dir(&repo_dir)?;

    let prompt_content = std::fs::read_to_string(&prompt_file)
        .with_context(|| format!("Failed to read prompt file: {}", prompt_file.display()))?;

    let start = Instant::now();

    // ── Step 1: Generate the runbook (resumable) ─────────────────────────
    divider();
    header("Step 1/2 — Generate runbook");

    let existing_content = std::fs::read_to_string(&output_path).ok();
    if let Some(ref content) = existing_content {
        if content.len() > 100 {
            info(&format!(
                "Found existing partial runbook ({} bytes) — will resume.",
                content.len()
            ));
        }
    }

    let generate_prompt = build_generate_prompt(
        &prompt_content,
        &template,
        &output_path,
        existing_content.as_deref(),
    );

    let log_file = LogFile::new(&log_dir, "plan-step1-generate.log")?;
    log_file.append("Step 1: Generate runbook")?;

    let invocation = ClaudeInvocation {
        prompt: generate_prompt,
        model: cli.model.clone(),
        allow_flags: toolflags::plan_allow_flags(),
        deny_flags: toolflags::plan_deny_flags(),
        working_dir: repo_dir.clone(),
    };

    let exit_code = invocation.run(&log_file)?;
    if exit_code != 0 {
        warn(&format!("claude exited with code {} during generation.", exit_code));
    }

    // Quick validation — if the file wasn't even created, bail early
    if !output_path.exists() {
        fail("Runbook file was not created. Check logs for details.");
        fail(&format!("Logs: {}/", log_dir.display()));
        anyhow::bail!("Step 1 failed: runbook not created.");
    }

    let issues = validate_runbook(&output_path);
    if issues.is_empty() {
        success("Runbook generated — all sections present.");
    } else {
        for issue in &issues {
            warn(issue);
        }
        warn("Proceeding to review step which may fix these issues.");
    }

    info(&format!("Cooling down {}s before review…", COOLDOWN_SECS));
    thread::sleep(Duration::from_secs(COOLDOWN_SECS));

    // ── Step 2: Review & correct ─────────────────────────────────────────
    divider();
    header("Step 2/2 — Review & correct");

    let review_prompt = build_review_prompt(&output_path);

    let log_file = LogFile::new(&log_dir, "plan-step2-review.log")?;
    log_file.append("Step 2: Review & correct")?;

    let invocation = ClaudeInvocation {
        prompt: review_prompt,
        model: cli.model.clone(),
        allow_flags: toolflags::plan_allow_flags(),
        deny_flags: toolflags::plan_deny_flags(),
        working_dir: repo_dir.clone(),
    };

    let exit_code = invocation.run(&log_file)?;
    if exit_code != 0 {
        warn(&format!("claude exited with code {} during review.", exit_code));
    }

    // Final validation
    let issues = validate_runbook(&output_path);
    if !issues.is_empty() {
        for issue in &issues {
            warn(issue);
        }
        warn(&format!(
            "Runbook has {} remaining issue(s) after review. Manual inspection recommended.",
            issues.len()
        ));
    }

    // ── Summary ──────────────────────────────────────────────────────────
    let elapsed = start.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;

    header("Planning Complete");

    if output_path.exists() {
        success(&format!("Runbook written to: {}", output_path.display()));

        let content = std::fs::read_to_string(&output_path).unwrap_or_default();
        let milestone_re = regex::Regex::new(r"^\|\s*\d+\s*\|").unwrap();
        let milestone_count = content
            .lines()
            .filter(|line| milestone_re.is_match(line.trim_start()))
            .count();

        info(&format!("Milestones planned: {}", milestone_count));

        info("Milestone Tracker:");
        for line in content.lines().filter(|l| l.trim_start().starts_with('|')).take(20) {
            println!("{}", line);
        }

        success("You can now run the milestones with:");
        info("  sldo-run <runbook> <repo-dir>");
    } else {
        fail("Runbook was not generated. Check logs for details.");
        fail(&format!("Logs: {}/", log_dir.display()));
    }

    info(&format!("Total wall time: {}m {}s", minutes, seconds));

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        fail(&format!("{:#}", e));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // ── CLI argument parsing ─────────────────────────────────────────────

    #[test]
    fn help_flag_exits_zero() {
        let result = Cli::try_parse_from(["sldo-plan", "--help"]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn missing_args_errors() {
        let result = Cli::try_parse_from(["sldo-plan"]);
        assert!(result.is_err());
    }

    #[test]
    fn default_output_is_none() {
        let cli = Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo"]).unwrap();
        assert!(cli.output.is_none());

        let output = resolve_output_path(cli.output.as_deref(), &cli.repo_dir);
        assert_eq!(output, PathBuf::from("/tmp/repo/docs/RUNBOOK.md"));
    }

    #[test]
    fn custom_output() {
        let cli =
            Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo", "-o", "custom.md"])
                .unwrap();
        let output = resolve_output_path(cli.output.as_deref(), &cli.repo_dir);
        assert_eq!(output, PathBuf::from("/tmp/repo/custom.md"));
    }

    #[test]
    fn custom_model() {
        let cli =
            Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo", "-m", "gpt-4"]).unwrap();
        assert_eq!(cli.model, "gpt-4");
    }

    #[test]
    fn default_model() {
        let cli = Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo"]).unwrap();
        assert_eq!(cli.model, "claude-opus-4.6");
    }

    // ── Template reading ─────────────────────────────────────────────────

    #[test]
    fn template_exists_returns_content() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        let template_path = workspace_root.join("docs/runbook-template.md");

        let content = read_template(&template_path);
        assert!(
            content.contains("Milestone Tracker"),
            "Template should contain 'Milestone Tracker'"
        );
    }

    #[test]
    fn template_missing_returns_fallback() {
        let path = Path::new("/nonexistent/path/to/template.md");
        let content = read_template(path);
        assert!(
            content.contains("Milestone Tracker"),
            "Fallback template should contain 'Milestone Tracker'"
        );
    }

    // ── Generate prompt (Step 1) ─────────────────────────────────────────

    #[test]
    fn generate_prompt_fresh() {
        // Given: no existing content
        let prompt = build_generate_prompt(
            "Add search functionality",
            "# Template\n## Milestone Tracker",
            Path::new("/tmp/RUNBOOK.md"),
            None,
        );

        // Then: contains key sections, no resume section
        assert!(prompt.contains("Add search functionality"));
        assert!(prompt.contains("Runbook Template"));
        assert!(prompt.contains("YOUR TASK"));
        assert!(prompt.contains("Threat Model"));
        assert!(!prompt.contains("RESUME FROM EXISTING DRAFT"));
    }

    #[test]
    fn generate_prompt_resumes_from_partial() {
        // Given: existing partial content > 100 bytes
        let existing = "x".repeat(200);
        let prompt = build_generate_prompt(
            "Add search",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
            Some(&existing),
        );

        // Then: contains resume section
        assert!(prompt.contains("RESUME FROM EXISTING DRAFT"));
        assert!(prompt.contains("200 bytes"));
    }

    #[test]
    fn generate_prompt_ignores_tiny_existing() {
        // Given: existing content <= 100 bytes (too small to be meaningful)
        let existing = "tiny";
        let prompt = build_generate_prompt(
            "Add search",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
            Some(existing),
        );

        // Then: no resume section — starts fresh
        assert!(!prompt.contains("RESUME FROM EXISTING DRAFT"));
    }

    #[test]
    fn generate_prompt_requires_threat_model_for_app_features() {
        let prompt = build_generate_prompt(
            "Add auth",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
            None,
        );

        assert!(prompt.contains("Threat Model"));
        assert!(prompt.contains("application features"));
        assert!(prompt.contains("Do NOT threat-model CI/CD"));
    }

    // ── Review prompt (Step 2) ───────────────────────────────────────────

    #[test]
    fn review_prompt_checks_for_forgotten_items() {
        let prompt = build_review_prompt(Path::new("/tmp/RUNBOOK.md"));

        assert!(prompt.contains(".gitignore"));
        assert!(prompt.contains("cargo fmt"));
        assert!(prompt.contains("cargo clippy"));
        assert!(prompt.contains("cargo audit"));
        assert!(prompt.contains("Smoke test"));
        assert!(prompt.contains("Threat Model"));
    }

    #[test]
    fn review_prompt_references_output_path() {
        let prompt = build_review_prompt(Path::new("/my/output/RUNBOOK.md"));
        assert!(prompt.contains("/my/output/RUNBOOK.md"));
    }

    // ── Runbook validation ───────────────────────────────────────────────

    #[test]
    fn valid_runbook_returns_empty_issues() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_valid");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        let content = format!(
            r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `not_started` | | | |
| 2 | Second | `not_started` | | | |

## Pre-Milestone Protocol
Something here.

## Post-Milestone Protocol
Something here.

## Background Context

### Current State
Described here.

## BDD Acceptance Scenarios
Scenarios here.

## Threat Model
Threats here.

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(issues.is_empty(), "Expected no issues, got: {:?}", issues);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_file_returns_issue() {
        let path = Path::new("/nonexistent/path/to/RUNBOOK.md");
        let issues = validate_runbook(path);
        assert!(!issues.is_empty());
        assert!(
            issues[0].contains("not created") || issues[0].contains("not found"),
            "Expected 'not created' issue, got: {}",
            issues[0]
        );
    }

    #[test]
    fn small_file_returns_issue() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_small");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        fs::write(&runbook_path, "x".repeat(100)).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(
            issues.iter().any(|i| i.contains("small") || i.contains("suspicious")),
            "Expected size issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_section_returns_issue() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_section");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        let content = format!(
            r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `not_started` | | | |

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
### Current State
## Threat Model

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(
            issues.iter().any(|i| i.contains("Missing section") && i.contains("BDD Acceptance Scenarios")),
            "Expected missing BDD section issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_threat_model_returns_issue() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_threat");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        let content = format!(
            r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `not_started` | | | |

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
### Current State
## BDD Acceptance Scenarios

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(
            issues.iter().any(|i| i.contains("Missing section") && i.contains("Threat Model")),
            "Expected missing Threat Model issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn unfilled_placeholders_returns_issue() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_placeholder");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        let content = format!(
            r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `not_started` | | | |

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
### Current State
[description]

## BDD Acceptance Scenarios
## Threat Model

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(
            issues.iter().any(|i| i.contains("placeholder")),
            "Expected placeholder issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn done_milestones_returns_issue() {
        let tmp = std::env::temp_dir().join("sldo_test_validate_done");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        let content = format!(
            r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `done` | 2026-01-01 | 2026-01-02 | |
| 2 | Second | `not_started` | | | |

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
### Current State
## BDD Acceptance Scenarios
## Threat Model

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        let issues = validate_runbook(&runbook_path);
        assert!(
            issues.iter().any(|i| i.contains("done")),
            "Expected done milestone issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    // ── Output path resolution ───────────────────────────────────────────

    #[test]
    fn resolve_output_default() {
        let output = resolve_output_path(None, Path::new("/tmp/repo"));
        assert_eq!(output, PathBuf::from("/tmp/repo/docs/RUNBOOK.md"));
    }

    #[test]
    fn resolve_output_relative() {
        let output = resolve_output_path(Some(Path::new("custom.md")), Path::new("/tmp/repo"));
        assert_eq!(output, PathBuf::from("/tmp/repo/custom.md"));
    }

    #[test]
    fn resolve_output_absolute() {
        let output =
            resolve_output_path(Some(Path::new("/abs/path.md")), Path::new("/tmp/repo"));
        assert_eq!(output, PathBuf::from("/abs/path.md"));
    }
}
