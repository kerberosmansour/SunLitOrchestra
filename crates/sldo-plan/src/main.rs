use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use clap::Parser;

use sldo_common::color::{divider, fail, header, info, success, warn};
use sldo_common::copilot::CopilotInvocation;
use sldo_common::logging::{ensure_log_dir, LogFile};
use sldo_common::preflight;
use sldo_common::toolflags;

/// Default cooldown between planning iterations (seconds).
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

## Documentation Update Table
"#;

/// Generate a milestone-based runbook from a requirements prompt and a repository.
///
/// Takes a prompt file describing desired changes and a repository directory,
/// then uses GitHub Copilot CLI to analyse the repo and produce a runbook
/// markdown file following the project's runbook template.
#[derive(Parser, Debug)]
#[command(name = "sldo-plan", about = "Generate a milestone-based runbook from a requirements prompt and a repository.")]
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

    /// Max planning refinement iterations.
    #[arg(short = 'n', long, default_value_t = 3)]
    max_iterations: u32,
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

/// Build the planning prompt for a given iteration.
///
/// For iteration 1, produces the initial planning prompt.
/// For iteration > 1, appends a refinement section referencing the output file.
pub fn build_planning_prompt(
    iteration: u32,
    prompt_content: &str,
    template: &str,
    output_path: &Path,
) -> String {
    let mut prompt = format!(
        r#"You are an expert software architect and planning agent. Your job is to analyse a
repository and produce a detailed, actionable runbook of milestones that an AI
coding agent can follow to implement the requested changes.

## INPUT

### User Requirements

The user has described the following desired changes:

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

1. **Explore the repository thoroughly.** Read key files: README, package.json /
   Cargo.toml / pyproject.toml / Makefile (whichever exist), directory structure,
   architecture docs, existing test structure, CI config. Understand the tech stack,
   project layout, build commands, and test commands.

2. **Analyse the requirements** against the current codebase. Identify:
   - Which files and modules need to change
   - What new files need to be created
   - What the correct build and test commands are
   - What dependencies might need to be added
   - What existing tests and features must not break

3. **Decompose into milestones.** Each milestone should be:
   - Small enough for one focused coding session (1-3 hours of AI agent work)
   - Self-contained: builds, tests pass, and the new feature works at the end
   - Ordered by dependency — earlier milestones provide foundations for later ones
   - Concrete: reference real file paths, real function names, real test commands

4. **For each milestone, fill in the template completely:**
   - **Goal**: One clear sentence.
   - **Context**: Reference specific files, modules, and current state.
   - **Files Most Likely Touched**: Real file paths with specific changes.
   - **Step-by-Step**: Numbered, actionable steps. Not vague — reference files and functions.
   - **BDD Acceptance Scenarios**: Concrete Given/When/Then with realistic test data.
   - **E2E Runtime Validation**: Tests that prove the system works at runtime.
   - **Regression Tests**: List pre-existing tests that must still pass.
   - **Smoke Tests**: Manual or scripted verification steps.

5. **Fill in background sections:**
   - **Current State**: Describe the real repo state (reference actual files).
   - **Problem**: Number the actual gaps the requirements describe.
   - **Target Architecture**: ASCII diagram or description of the end state.
   - **Key Design Principles**: Derive from the codebase style and requirements.
   - **What to Keep / What to Change**: Concrete lists of files and modules.
   - **Documentation Update Table**: What docs need updating per milestone.

6. **Populate protocols with real commands:**
   - Replace placeholder test/build commands with the actual commands for this repo.
   - Use the correct package manager, test runner, and build system.

7. **Write the completed runbook** to: `{output}`

## HARD RULES

- Explore the repo BEFORE writing the runbook. Do not guess file paths or commands.
- Every file path in the runbook must be a real path in the repo (or a clearly marked new file).
- Every build/test command must be the actual command for this project's tech stack.
- Milestones must be strictly sequential — no circular dependencies.
- Each milestone must leave the project in a buildable, testable state.
- Do NOT implement any code changes. This is a PLANNING session only.
- Do NOT modify any existing source files in the repository.
- Do NOT commit or push anything.
- Write the runbook output to `{output}`.
- The runbook file is the ONLY file you should create or modify."#,
        output = output_path.display(),
    );

    if iteration > 1 {
        prompt.push_str(&format!(
            r#"

## REFINEMENT PASS {iteration}

A previous planning pass has already written a draft runbook to `{output}`.
Please:

1. **Read the existing draft** at `{output}`.
2. **Re-explore the repo** to verify all file paths and commands in the draft are accurate.
3. **Improve the runbook**:
   - Fix any incorrect file paths, commands, or tech stack references.
   - Add missing BDD scenarios or E2E tests.
   - Ensure step-by-step instructions are concrete enough for an AI agent to follow.
   - Verify milestone ordering makes sense (no forward dependencies).
   - Fill in any placeholder text that was left from the template.
4. **Overwrite** `{output}` with the improved version."#,
            output = output_path.display(),
        ));
    }

    prompt
}

/// Required sections that a valid runbook must contain.
const REQUIRED_SECTIONS: &[&str] = &[
    "Milestone Tracker",
    "Pre-Milestone Protocol",
    "Post-Milestone Protocol",
    "Background Context",
    "Current State",
    "BDD Acceptance Scenarios",
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
    let milestone_count = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with('|')
                && regex::Regex::new(r"^\|\s*\d+\s*\|")
                    .unwrap()
                    .is_match(trimmed)
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
                && regex::Regex::new(r"^\|\s*\d+\s*\|")
                    .unwrap()
                    .is_match(trimmed)
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

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Resolve paths
    let prompt_file = cli.prompt_file.canonicalize().with_context(|| {
        format!(
            "Prompt file not found: {}",
            cli.prompt_file.display()
        )
    })?;

    let repo_dir = cli.repo_dir.canonicalize().with_context(|| {
        format!(
            "Repository directory not found: {}",
            cli.repo_dir.display()
        )
    })?;

    let output_path = resolve_output_path(cli.output.as_deref(), &repo_dir);

    header("Milestone Planner — Runbook Generator");
    info(&format!("Prompt file:    {}", prompt_file.display()));
    info(&format!("Repository:     {}", repo_dir.display()));
    info(&format!("Output:         {}", output_path.display()));
    info(&format!("Model:          {}", cli.model));
    info(&format!("Max iterations: {}", cli.max_iterations));

    // Pre-flight checks
    header("Pre-flight checks");

    let copilot_path = preflight::check_copilot_installed()?;
    success(&format!("copilot CLI found: {}", copilot_path.display()));

    preflight::check_file_exists(&prompt_file, "Prompt file")?;
    let prompt_size = std::fs::metadata(&prompt_file)?.len();
    success(&format!(
        "Prompt file found: {} ({} bytes)",
        prompt_file.display(),
        prompt_size
    ));

    if !repo_dir.is_dir() {
        anyhow::bail!("Repository directory not found: {}", repo_dir.display());
    }
    success(&format!("Repository directory: {}", repo_dir.display()));

    let branch = preflight::check_git_safety(&repo_dir)?;
    info(&format!("Current git branch: {}", branch));
    success(&format!(
        "Branch '{}' is not main/master — safe to proceed.",
        branch
    ));

    // Resolve template path relative to binary or repo
    let template_path = repo_dir.join("docs/runbook-template.md");
    let template = read_template(&template_path);
    if template_path.exists() {
        success(&format!("Runbook template found: {}", template_path.display()));
    } else {
        warn(&format!(
            "Runbook template not found at {} — will use built-in template.",
            template_path.display()
        ));
    }

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    success(&format!("Output will be written to: {}", output_path.display()));

    // Set up logging
    let log_dir = ensure_log_dir(&repo_dir)?;
    success(&format!("Log directory: {}", log_dir.display()));

    // Read prompt content
    let prompt_content = std::fs::read_to_string(&prompt_file)
        .with_context(|| format!("Failed to read prompt file: {}", prompt_file.display()))?;

    let start = Instant::now();

    for iteration in 1..=cli.max_iterations {
        let log_file = LogFile::new(&log_dir, &format!("plan-iteration-{}.log", iteration))?;

        divider();
        if iteration == 1 {
            info(&format!(
                "Iteration {}/{} — Initial planning pass",
                iteration, cli.max_iterations
            ));
        } else {
            info(&format!(
                "Iteration {}/{} — Refinement pass",
                iteration, cli.max_iterations
            ));
        }
        divider();

        let prompt = build_planning_prompt(iteration, &prompt_content, &template, &output_path);

        log_file.append(&format!(
            "═══════════════════════════════════════════════════\nPlanning iteration {}",
            iteration
        ))?;

        // Invoke Copilot CLI
        let invocation = CopilotInvocation {
            prompt,
            model: cli.model.clone(),
            allow_flags: toolflags::plan_allow_flags(),
            deny_flags: toolflags::plan_deny_flags(),
            working_dir: repo_dir.clone(),
        };

        let exit_code = invocation.run(&log_file)?;

        if exit_code != 0 {
            warn(&format!("Copilot exited with code {}.", exit_code));
        }

        // Validate the output
        let issues = validate_runbook(&output_path);
        if issues.is_empty() {
            success(&format!(
                "Runbook generated successfully after {} iteration(s).",
                iteration
            ));
            break;
        }

        for issue in &issues {
            warn(issue);
        }
        warn(&format!(
            "Runbook validation found {} issue(s). Will attempt refinement.",
            issues.len()
        ));

        if iteration < cli.max_iterations {
            info(&format!(
                "Cooling down {}s before refinement…",
                COOLDOWN_SECS
            ));
            thread::sleep(Duration::from_secs(COOLDOWN_SECS));
        }
    }

    let elapsed = start.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;

    header("Planning Complete");

    if output_path.exists() {
        success(&format!("Runbook written to: {}", output_path.display()));

        let content = std::fs::read_to_string(&output_path).unwrap_or_default();
        let milestone_count = content
            .lines()
            .filter(|line| {
                regex::Regex::new(r"^\|\s*\d+\s*\|")
                    .unwrap()
                    .is_match(line.trim_start())
            })
            .count();

        info(&format!("Milestones planned: {}", milestone_count));

        info("Milestone Tracker:");
        for line in content.lines().filter(|l| l.trim_start().starts_with('|')).take(20) {
            println!("{}", line);
        }

        success("You can now run the milestones with:");
        info("  sldo-run <runbook> <repo-dir>");
    } else {
        fail(&format!(
            "Runbook was not generated after {} iterations.",
            cli.max_iterations
        ));
        fail(&format!("Check logs in {}/ for details.", log_dir.display()));
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
        // Given: N/A
        // When: sldo-plan --help is invoked
        let result = Cli::try_parse_from(["sldo-plan", "--help"]);
        // Then: clap returns help error (which exits 0)
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn missing_args_errors() {
        // Given: No arguments
        // When: sldo-plan is invoked with no args
        let result = Cli::try_parse_from(["sldo-plan"]);
        // Then: clap returns error
        assert!(result.is_err());
    }

    #[test]
    fn default_output_is_none() {
        // Given: sldo-plan prompt.txt /tmp/repo
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo"]).unwrap();
        // Then: output defaults to None (resolved later to <repo>/docs/RUNBOOK.md)
        assert!(cli.output.is_none());

        let output = resolve_output_path(cli.output.as_deref(), &cli.repo_dir);
        assert_eq!(output, PathBuf::from("/tmp/repo/docs/RUNBOOK.md"));
    }

    #[test]
    fn custom_output() {
        // Given: sldo-plan prompt.txt /tmp/repo -o custom.md
        // When: Args are parsed
        let cli =
            Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo", "-o", "custom.md"])
                .unwrap();
        // Then: output is resolved relative to repo_dir
        let output = resolve_output_path(cli.output.as_deref(), &cli.repo_dir);
        assert_eq!(output, PathBuf::from("/tmp/repo/custom.md"));
    }

    #[test]
    fn custom_model() {
        // Given: sldo-plan prompt.txt /tmp/repo -m gpt-4
        // When: Args are parsed
        let cli =
            Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo", "-m", "gpt-4"]).unwrap();
        // Then: model is "gpt-4"
        assert_eq!(cli.model, "gpt-4");
    }

    #[test]
    fn default_model() {
        // Given: sldo-plan prompt.txt /tmp/repo (no -m flag)
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo"]).unwrap();
        // Then: model defaults to "claude-opus-4.6"
        assert_eq!(cli.model, "claude-opus-4.6");
    }

    #[test]
    fn default_max_iterations() {
        // Given: sldo-plan prompt.txt /tmp/repo (no -n flag)
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo"]).unwrap();
        // Then: max_iterations defaults to 3
        assert_eq!(cli.max_iterations, 3);
    }

    #[test]
    fn custom_max_iterations() {
        // Given: sldo-plan prompt.txt /tmp/repo -n 5
        // When: Args are parsed
        let cli =
            Cli::try_parse_from(["sldo-plan", "prompt.txt", "/tmp/repo", "-n", "5"]).unwrap();
        // Then: max_iterations is 5
        assert_eq!(cli.max_iterations, 5);
    }

    // ── Template reading ─────────────────────────────────────────────────

    #[test]
    fn template_exists_returns_content() {
        // Given: docs/runbook-template.md is on disk
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        let template_path = workspace_root.join("docs/runbook-template.md");

        // When: read_template() is called
        let content = read_template(&template_path);

        // Then: Returns file contents containing "Milestone Tracker"
        assert!(
            content.contains("Milestone Tracker"),
            "Template should contain 'Milestone Tracker'"
        );
    }

    #[test]
    fn template_missing_returns_fallback() {
        // Given: Template path does not exist
        let path = Path::new("/nonexistent/path/to/template.md");

        // When: read_template() is called
        let content = read_template(path);

        // Then: Returns fallback template string containing "Milestone Tracker"
        assert!(
            content.contains("Milestone Tracker"),
            "Fallback template should contain 'Milestone Tracker'"
        );
    }

    // ── Prompt construction ──────────────────────────────────────────────

    #[test]
    fn first_iteration_prompt() {
        // Given: iteration=1, prompt="Add search", template="..."
        let prompt = build_planning_prompt(
            1,
            "Add search functionality",
            "# Template\n## Milestone Tracker",
            Path::new("/tmp/RUNBOOK.md"),
        );

        // Then: Contains key sections, does NOT contain "REFINEMENT PASS"
        assert!(prompt.contains("User Requirements"));
        assert!(prompt.contains("Runbook Template"));
        assert!(prompt.contains("YOUR TASK"));
        assert!(prompt.contains("Add search functionality"));
        assert!(!prompt.contains("REFINEMENT PASS"));
    }

    #[test]
    fn refinement_iteration_prompt() {
        // Given: iteration=2, output="/tmp/RUNBOOK.md"
        let prompt = build_planning_prompt(
            2,
            "Add search",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
        );

        // Then: Contains "REFINEMENT PASS 2" and the output path
        assert!(prompt.contains("REFINEMENT PASS 2"));
        assert!(prompt.contains("/tmp/RUNBOOK.md"));
    }

    // ── Runbook validation ───────────────────────────────────────────────

    #[test]
    fn valid_runbook_returns_empty_issues() {
        // Given: File >500 bytes with all required sections, milestones all not_started
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

{}
"#,
            "x".repeat(300) // pad to > 500 bytes
        );
        fs::write(&runbook_path, &content).unwrap();

        // When: validate_runbook(path) is called
        let issues = validate_runbook(&runbook_path);

        // Then: Returns empty issues list
        assert!(
            issues.is_empty(),
            "Expected no issues, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_file_returns_issue() {
        // Given: Path does not exist
        let path = Path::new("/nonexistent/path/to/RUNBOOK.md");

        // When: validate_runbook(path) is called
        let issues = validate_runbook(path);

        // Then: Returns issue "file was not created"
        assert!(!issues.is_empty());
        assert!(
            issues[0].contains("not created") || issues[0].contains("not found"),
            "Expected 'not created' issue, got: {}",
            issues[0]
        );
    }

    #[test]
    fn small_file_returns_issue() {
        // Given: File is 100 bytes
        let tmp = std::env::temp_dir().join("sldo_test_validate_small");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let runbook_path = tmp.join("RUNBOOK.md");

        fs::write(&runbook_path, "x".repeat(100)).unwrap();

        // When: validate_runbook(path) is called
        let issues = validate_runbook(&runbook_path);

        // Then: Returns issue about suspicious size
        assert!(
            issues.iter().any(|i| i.contains("small") || i.contains("suspicious")),
            "Expected size issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_section_returns_issue() {
        // Given: File lacks "BDD Acceptance Scenarios"
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

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        // When: validate_runbook(path) is called
        let issues = validate_runbook(&runbook_path);

        // Then: Returns issue about missing section
        assert!(
            issues.iter().any(|i| i.contains("Missing section") && i.contains("BDD Acceptance Scenarios")),
            "Expected missing BDD section issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn unfilled_placeholders_returns_issue() {
        // Given: File contains [description]
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

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        // When: validate_runbook(path) is called
        let issues = validate_runbook(&runbook_path);

        // Then: Returns issue about placeholders
        assert!(
            issues.iter().any(|i| i.contains("placeholder")),
            "Expected placeholder issue, got: {:?}",
            issues
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn done_milestones_returns_issue() {
        // Given: A runbook with a milestone marked done
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

{}
"#,
            "x".repeat(300)
        );
        fs::write(&runbook_path, &content).unwrap();

        // When: validate_runbook(path) is called
        let issues = validate_runbook(&runbook_path);

        // Then: Returns issue about done milestones
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
        // Given: No output specified
        let output = resolve_output_path(None, Path::new("/tmp/repo"));
        // Then: defaults to <repo>/docs/RUNBOOK.md
        assert_eq!(output, PathBuf::from("/tmp/repo/docs/RUNBOOK.md"));
    }

    #[test]
    fn resolve_output_relative() {
        // Given: Relative output path
        let output = resolve_output_path(Some(Path::new("custom.md")), Path::new("/tmp/repo"));
        // Then: resolved relative to repo_dir
        assert_eq!(output, PathBuf::from("/tmp/repo/custom.md"));
    }

    #[test]
    fn resolve_output_absolute() {
        // Given: Absolute output path
        let output =
            resolve_output_path(Some(Path::new("/abs/path.md")), Path::new("/tmp/repo"));
        // Then: used as-is
        assert_eq!(output, PathBuf::from("/abs/path.md"));
    }
}
