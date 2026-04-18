//! Planning Tauri commands — validates inputs, builds prompt, invokes Claude Code CLI,
//! and emits streaming events to the frontend.
//! Also provides runbook read/save commands for the Markdown editor (M4).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use sldo_common::copilot::ClaudeInvocation;
use sldo_common::logging::{ensure_log_dir, LogFile};
use sldo_common::preflight;
use sldo_common::runbook;

use crate::events::{PlanCompleteEvent, PlanErrorEvent, PlanProgressEvent};
use crate::state::{AppState, PlanningSession};

/// Data returned by the `read_runbook` command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunbookData {
    /// Raw Markdown content of the runbook file.
    pub content: String,
    /// Parsed milestone rows from the tracker table.
    pub milestones: Vec<MilestoneRowDto>,
    /// Absolute path to the runbook file.
    pub path: String,
}

/// Serializable milestone row DTO for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneRowDto {
    pub number: u32,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lessons_file: Option<String>,
}

impl From<&runbook::MilestoneRow> for MilestoneRowDto {
    fn from(row: &runbook::MilestoneRow) -> Self {
        Self {
            number: row.number,
            title: row.title.clone(),
            status: row.status.to_string(),
            started: row.started.clone(),
            completed: row.completed.clone(),
            lessons_file: row.lessons_file.clone(),
        }
    }
}

/// Read a runbook file and return its content and parsed milestones.
#[tauri::command]
pub fn read_runbook(path: String) -> Result<RunbookData, String> {
    let file_path = Path::new(&path);

    if !file_path.exists() {
        return Err(format!("Runbook file not found: {}", path));
    }

    let content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read runbook: {}", e))?;

    let milestones: Vec<MilestoneRowDto> = runbook::parse_tracker(&content)
        .iter()
        .map(MilestoneRowDto::from)
        .collect();

    Ok(RunbookData {
        content,
        milestones,
        path,
    })
}

/// Save runbook content to disk and re-parse to validate.
/// Returns a list of validation warnings (empty if valid).
#[tauri::command]
pub fn save_runbook(path: String, content: String) -> Result<Vec<String>, String> {
    let file_path = Path::new(&path);

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Write content to disk
    std::fs::write(file_path, &content)
        .map_err(|e| format!("Failed to write runbook: {}", e))?;

    // Validate the saved content
    let mut warnings = Vec::new();

    // Check required sections
    let required_sections = [
        "Milestone Tracker",
        "Pre-Milestone Protocol",
        "Post-Milestone Protocol",
        "Background Context",
    ];
    for section in &required_sections {
        if !content.contains(section) {
            warnings.push(format!("Missing section: {}", section));
        }
    }

    // Check milestone table is parseable
    let milestones = runbook::parse_tracker(&content);
    if milestones.is_empty() {
        warnings.push("No milestones found in tracker table".to_string());
    }

    // Check file size
    if content.len() < 500 {
        warnings.push(format!("Runbook is suspiciously small ({} bytes)", content.len()));
    }

    Ok(warnings)
}

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

/// Build the planning prompt for a given iteration.
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

<requirements>
{prompt_content}
</requirements>

### Runbook Template

<template>
{template}
</template>

## YOUR TASK

1. Explore the repository thoroughly.
2. Analyse the requirements against the codebase.
3. Decompose into milestones.
4. Fill in the template completely.
5. Write the completed runbook to: `{output}`

## HARD RULES

- Explore the repo BEFORE writing the runbook.
- Every file path must be real.
- Milestones must be strictly sequential.
- Do NOT implement code changes — this is PLANNING only.
- Write the runbook to `{output}`."#,
        output = output_path.display(),
    );

    if iteration > 1 {
        prompt.push_str(&format!(
            r#"

## REFINEMENT PASS {iteration}

Read the existing draft at `{output}` and improve it."#,
            output = output_path.display(),
        ));
    }

    prompt
}

/// Read the runbook template from disk, falling back to a built-in template.
fn read_template(template_path: &Path) -> String {
    match std::fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(_) => FALLBACK_TEMPLATE.to_string(),
    }
}

/// Resolve the output path.
fn resolve_output_path(output: Option<&str>, repo_dir: &Path) -> PathBuf {
    match output {
        Some(p) => {
            let path = PathBuf::from(p);
            if path.is_absolute() {
                path
            } else {
                repo_dir.join(path)
            }
        }
        None => repo_dir.join(DEFAULT_OUTPUT_SUBPATH),
    }
}

/// Required sections for runbook validation.
const REQUIRED_SECTIONS: &[&str] = &[
    "Milestone Tracker",
    "Pre-Milestone Protocol",
    "Post-Milestone Protocol",
    "Background Context",
];

/// Validate a generated runbook file, returning a list of issues.
pub fn validate_runbook(path: &Path) -> Vec<String> {
    let mut issues = Vec::new();

    if !path.exists() {
        issues.push(format!(
            "Runbook file was not created at: {}",
            path.display()
        ));
        return issues;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            issues.push(format!("Failed to read runbook: {}", e));
            return issues;
        }
    };

    if content.len() < 500 {
        issues.push(format!(
            "Runbook is suspiciously small ({} bytes).",
            content.len()
        ));
    }

    for section in REQUIRED_SECTIONS {
        if !content.contains(section) {
            issues.push(format!("Missing section: {}", section));
        }
    }

    issues
}

/// ISO 8601 timestamp for event payloads.
fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Start a planning session. Runs preflight checks, then spawns an async task
/// that invokes Copilot and emits streaming events.
///
/// Returns immediately with the output path. Progress is delivered via events:
/// - `plan-progress` — each line of Copilot output
/// - `plan-complete` — planning finished successfully
/// - `plan-error` — planning failed
#[tauri::command]
pub async fn start_planning(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    prompt: String,
    repo_dir: String,
    output_path: Option<String>,
) -> Result<String, String> {
    let repo = PathBuf::from(&repo_dir);
    let output = resolve_output_path(output_path.as_deref(), &repo);

    // Preflight: claude installed
    preflight::check_claude_installed().map_err(|e| e.to_string())?;

    // Preflight: git safety
    preflight::check_git_safety(&repo).map_err(|e| e.to_string())?;

    // Update session state
    {
        let mut session = state.current_session.lock().map_err(|e| e.to_string())?;
        *session = Some(PlanningSession {
            id: format!("plan-{}", chrono::Utc::now().timestamp_millis()),
            prompt: prompt.clone(),
            runbook_path: None,
            in_progress: true,
        });
    }

    let settings = {
        let s = state.settings.lock().map_err(|e| e.to_string())?;
        s.clone()
    };

    let output_str = output.display().to_string();

    // Spawn on a blocking thread so the synchronous process I/O does not
    // starve the tokio async runtime.
    let app_handle = app.clone();
    let prompt_clone = prompt.clone();
    let output_clone = output.clone();
    let repo_clone = repo.clone();

    tokio::task::spawn_blocking(move || {
        let result: Result<Result<(), anyhow::Error>, _> =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_planning_sync(
                    &app_handle,
                    &prompt_clone,
                    &repo_clone,
                    &output_clone,
                    &settings.model,
                    settings.max_iterations,
                    &settings.allow_flags,
                    &settings.deny_flags,
                )
            }));

        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                let _ = app_handle.emit(
                    "plan-error",
                    PlanErrorEvent {
                        error: e.to_string(),
                    },
                );
            }
            Err(_) => {
                let _ = app_handle.emit(
                    "plan-error",
                    PlanErrorEvent {
                        error: "Planning task panicked".to_string(),
                    },
                );
            }
        }

        // Mark session as no longer in progress
        if let Some(state) = app_handle.try_state::<AppState>() {
            if let Ok(mut session) = state.current_session.lock() {
                if let Some(ref mut s) = *session {
                    s.in_progress = false;
                }
            }
        }
    });

    Ok(output_str)
}

/// Synchronous planning execution (runs inside spawn_blocking).
fn run_planning_sync(
    app: &AppHandle,
    prompt: &str,
    repo_dir: &Path,
    output_path: &Path,
    model: &str,
    max_iterations: u32,
    allow_flags: &[String],
    deny_flags: &[String],
) -> Result<(), anyhow::Error> {
    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let log_dir = ensure_log_dir(repo_dir)?;

    // Read template
    let template_path = repo_dir.join("docs/runbook-template.md");
    let template = read_template(&template_path);

    for iteration in 1..=max_iterations {
        let log_file = LogFile::new(&log_dir, &format!("plan-iteration-{}.log", iteration))?;

        let planning_prompt =
            build_planning_prompt(iteration, prompt, &template, output_path);

        let invocation = ClaudeInvocation {
            prompt: planning_prompt,
            model: model.to_string(),
            allow_flags: allow_flags.to_vec(),
            deny_flags: deny_flags.to_vec(),
            working_dir: repo_dir.to_path_buf(),
        };

        let app_clone = app.clone();
        let exit_code = invocation.run_with_callback(&log_file, |line, stream| {
            let _ = app_clone.emit(
                "plan-progress",
                PlanProgressEvent {
                    line: line.to_string(),
                    stream: stream.to_string(),
                    timestamp: now_iso(),
                },
            );
        })?;

        if exit_code != 0 {
            let _ = app.emit(
                "plan-progress",
                PlanProgressEvent {
                    line: format!("claude exited with code {}", exit_code),
                    stream: "stderr".to_string(),
                    timestamp: now_iso(),
                },
            );
        }

        // Validate
        let issues = validate_runbook(output_path);
        if issues.is_empty() {
            // Runbook is valid, emit complete
            let _ = app.emit(
                "plan-complete",
                PlanCompleteEvent {
                    runbook_path: output_path.display().to_string(),
                    validation_issues: vec![],
                },
            );

            // Update session with runbook path
            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(mut session) = state.current_session.lock() {
                    if let Some(ref mut s) = *session {
                        s.runbook_path = Some(output_path.to_path_buf());
                    }
                }
            }

            return Ok(());
        }

        // If last iteration and still has issues, emit complete with warnings
        if iteration == max_iterations {
            let _ = app.emit(
                "plan-complete",
                PlanCompleteEvent {
                    runbook_path: output_path.display().to_string(),
                    validation_issues: issues,
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_planning_prompt_contains_requirements() {
        // Given: A prompt and template
        let prompt = build_planning_prompt(
            1,
            "Build a REST API",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
        );
        // Then: Contains the user requirements
        assert!(prompt.contains("Build a REST API"));
        assert!(prompt.contains("# Template"));
        assert!(prompt.contains("RUNBOOK.md"));
    }

    #[test]
    fn build_planning_prompt_iteration_2_has_refinement() {
        // Given: Iteration 2
        let prompt = build_planning_prompt(
            2,
            "Build a REST API",
            "# Template",
            Path::new("/tmp/RUNBOOK.md"),
        );
        // Then: Contains refinement instructions
        assert!(prompt.contains("REFINEMENT PASS 2"));
    }

    #[test]
    fn resolve_output_path_default() {
        // Given: No output path specified
        let resolved = resolve_output_path(None, Path::new("/repo"));
        // Then: Defaults to docs/RUNBOOK.md
        assert_eq!(resolved, PathBuf::from("/repo/docs/RUNBOOK.md"));
    }

    #[test]
    fn resolve_output_path_relative() {
        // Given: A relative output path
        let resolved = resolve_output_path(Some("output/plan.md"), Path::new("/repo"));
        // Then: Resolved relative to repo_dir
        assert_eq!(resolved, PathBuf::from("/repo/output/plan.md"));
    }

    #[test]
    fn resolve_output_path_absolute() {
        // Given: An absolute output path
        let resolved = resolve_output_path(Some("/tmp/plan.md"), Path::new("/repo"));
        // Then: Used as-is
        assert_eq!(resolved, PathBuf::from("/tmp/plan.md"));
    }

    #[test]
    fn validate_runbook_missing_file() {
        // Given: A non-existent path
        let issues = validate_runbook(Path::new("/nonexistent/RUNBOOK.md"));
        // Then: Reports the file is missing
        assert!(!issues.is_empty());
        assert!(issues[0].contains("not created"));
    }

    #[test]
    fn validate_runbook_valid_content() {
        // Given: A temp file with valid runbook content
        let tmp = std::env::temp_dir().join("sldo_test_validate_runbook");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("RUNBOOK.md");
        let content = format!(
            "{}\n{}\n{}\n{}\n{}",
            "# Test Runbook\n\n## Milestone Tracker\n\n| # | M | S |\n",
            "## Pre-Milestone Protocol\n\nDo stuff.\n",
            "## Post-Milestone Protocol\n\nDo more stuff.\n",
            "## Background Context\n\n### Current State\n\nSome state.\n",
            "Extra content to make it over 500 bytes. ".repeat(15)
        );
        std::fs::write(&path, &content).unwrap();

        // When: validate_runbook is called
        let issues = validate_runbook(&path);

        // Then: No issues (all required sections present, size OK)
        assert!(issues.is_empty(), "Unexpected issues: {:?}", issues);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_template_fallback() {
        // Given: A non-existent template path
        let template = read_template(Path::new("/nonexistent/template.md"));
        // Then: Falls back to built-in template
        assert!(template.contains("Milestone Tracker"));
    }

    #[test]
    fn claude_not_installed_returns_error_mentioning_claude() {
        // Given: We check for claude
        // When: claude may not be installed
        let result = preflight::check_claude_installed();
        // Then: If error, message mentions "claude"
        if let Err(e) = result {
            let msg = e.to_string().to_lowercase();
            assert!(msg.contains("claude"), "Error should mention claude: {}", msg);
        }
    }

    #[test]
    fn protected_branch_rejected() {
        // Given: A repo on main branch — we use a temp dir with git init on main
        let tmp = std::env::temp_dir().join("sldo_test_protected_branch");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        // Initialize a git repo on main branch
        let _ = std::process::Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(&tmp)
            .output();
        let _ = std::process::Command::new("git")
            .args(["commit", "--allow-empty", "-m", "init"])
            .current_dir(&tmp)
            .output();

        // When: check_git_safety is called
        let result = preflight::check_git_safety(&tmp);

        // Then: Error mentions "protected" or "main"
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            msg.contains("protected") || msg.contains("main"),
            "Error should mention protected branch: {}",
            msg
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_runbook_returns_content_and_milestones() {
        // Given: A temp file with valid runbook content
        let tmp = std::env::temp_dir().join("sldo_test_read_runbook");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("RUNBOOK.md");
        let content = r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Setup | `done` | 2026-01-01 | 2026-01-02 | |
| 2 | Core | `in_progress` | 2026-01-03 | | |
"#;
        std::fs::write(&path, content).unwrap();

        // When: read_runbook is called
        let result = read_runbook(path.display().to_string());

        // Then: Returns content and parsed milestones
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.content.contains("Test Runbook"));
        assert_eq!(data.milestones.len(), 2);
        assert_eq!(data.milestones[0].title, "Setup");
        assert_eq!(data.milestones[0].status, "done");
        assert_eq!(data.milestones[1].title, "Core");
        assert_eq!(data.milestones[1].status, "in_progress");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_runbook_missing_file_returns_error() {
        // Given: A non-existent path
        let result = read_runbook("/nonexistent/RUNBOOK.md".to_string());

        // Then: Returns error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn save_runbook_writes_and_validates() {
        // Given: Valid runbook content
        let tmp = std::env::temp_dir().join("sldo_test_save_runbook");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("RUNBOOK.md");
        let content = format!(
            "{}\n{}\n{}\n{}\n{}",
            "# Test Runbook\n\n## Milestone Tracker\n\n| # | Milestone | Status | Started | Completed | Lessons File |\n|---|---|---|---|---|---|\n| 1 | Setup | `done` | 2026-01-01 | 2026-01-02 | |",
            "\n## Pre-Milestone Protocol\n\nDo stuff.\n",
            "## Post-Milestone Protocol\n\nDo more stuff.\n",
            "## Background Context\n\n### Current State\n\nSome state.\n",
            "Extra content to make it over 500 bytes. ".repeat(15)
        );

        // When: save_runbook is called
        let result = save_runbook(path.display().to_string(), content.clone());

        // Then: File written, no warnings
        assert!(result.is_ok());
        let warnings = result.unwrap();
        assert!(warnings.is_empty(), "Unexpected warnings: {:?}", warnings);

        // And: File on disk matches
        let on_disk = std::fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, content);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn save_runbook_invalid_content_returns_warnings() {
        // Given: Content missing milestone table
        let tmp = std::env::temp_dir().join("sldo_test_save_runbook_invalid");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("RUNBOOK.md");
        let content = "# Just a title\n\nSome text.\n";

        // When: save_runbook is called
        let result = save_runbook(path.display().to_string(), content.to_string());

        // Then: Returns warnings
        assert!(result.is_ok());
        let warnings = result.unwrap();
        assert!(!warnings.is_empty(), "Expected warnings for invalid content");
        // Should mention missing sections
        assert!(
            warnings.iter().any(|w| w.contains("Missing section")),
            "Expected 'Missing section' warning, got: {:?}",
            warnings
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn milestone_row_dto_from_runbook_row() {
        // Given: A runbook MilestoneRow
        use sldo_common::runbook::{MilestoneRow, MilestoneStatus};
        let row = MilestoneRow {
            number: 1,
            title: "Setup".to_string(),
            status: MilestoneStatus::Done,
            started: Some("2026-01-01".to_string()),
            completed: Some("2026-01-02".to_string()),
            lessons_file: None,
        };

        // When: Convert to DTO
        let dto = MilestoneRowDto::from(&row);

        // Then: Fields match
        assert_eq!(dto.number, 1);
        assert_eq!(dto.title, "Setup");
        assert_eq!(dto.status, "done");
        assert_eq!(dto.started.as_deref(), Some("2026-01-01"));
        assert_eq!(dto.completed.as_deref(), Some("2026-01-02"));
        assert!(dto.lessons_file.is_none());
    }
}
