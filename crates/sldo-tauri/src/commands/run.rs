//! Execution Tauri commands — runs milestones from a runbook, emitting
//! streaming events for agent output, build/test results, and progress.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering;

use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager};

use sldo_common::copilot::CopilotInvocation;
use sldo_common::detect;
use sldo_common::logging::{ensure_log_dir, LogFile};
use sldo_common::runbook;

use crate::events::{
    BuildTestResultEvent, ExecutionCompleteEvent, ExecutionProgressEvent,
    MilestoneCompletedEvent, MilestoneStartedEvent,
};
use crate::state::AppState;

/// Build a summary of commands for embedding in the execution prompt.
fn build_cmd_summary(label: &str, cmds: &[String]) -> String {
    if cmds.is_empty() {
        return format!("- No {} commands configured.", label);
    }
    cmds.iter()
        .map(|cmd| format!("- `{}`", cmd))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Build the execution prompt, matching the pattern from `sldo-run`.
pub fn build_execution_prompt(
    runbook_path: &str,
    build_cmds: &[String],
    test_cmds: &[String],
    attempt: u32,
) -> String {
    let build_summary = build_cmd_summary("build", build_cmds);
    let test_summary = build_cmd_summary("test", test_cmds);
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut prompt = format!(
        r#"You are an expert software engineer working on a project.

## YOUR TASK

1. Read the runbook at `{runbook_path}`. Look at the **Milestone Tracker** table.
2. Find the first milestone whose status is NOT `done` (i.e. `not_started` or `in_progress`).
3. Complete that milestone — and ONLY that milestone.
4. If an architecture doc exists (e.g. `docs/ARCHITECTURE.md`), read it for context.
5. If a lessons file exists for the previous milestone, read it and apply any corrections.

## Workflow

- **Verify baseline**: Run the project's build and test commands before changing anything.
  Build commands:
{build_summary}
  Test commands:
{test_summary}
- **Update tracker**: Set the milestone status to `in_progress` and Started date to {today}.
- **Write tests FIRST**: Create BDD and E2E test files from the milestone's scenario tables.
- **Implement**: Follow the Step-by-Step section. Make all tests pass.
- **Post-milestone**: Run smoke tests, write lessons-learned, update tracker to `done`.
- **Final check**: Run all build and test commands one last time — everything must be green.
- **STOP**: Do not proceed to the next milestone."#
    );

    if attempt > 1 {
        let mut retry_cmds = String::new();
        for cmd in build_cmds {
            retry_cmds.push_str(&format!(
                "\n- Run `{}` to check the current build state.",
                cmd
            ));
        }
        for cmd in test_cmds {
            retry_cmds.push_str(&format!(
                "\n- Run `{}` to check the current test state.",
                cmd
            ));
        }

        prompt.push_str(&format!(
            r#"

## RETRY CONTEXT — Attempt {attempt}

A previous session did not fully complete a milestone. Please:
1. Check the current project state:{retry_cmds}
2. Read existing code and test files to understand what was already done.
3. Fix any issues and complete the next incomplete milestone.
4. Run all build and test commands to verify everything passes.
5. STOP after that one milestone is complete."#
        ));
    }

    prompt
}

/// Run a single verification command, returning (success, output).
fn run_verification_command(cmd: &str) -> (bool, String) {
    match Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}{}", stdout, stderr);
            (output.status.success(), combined)
        }
        Err(e) => (false, format!("Failed to run command: {}", e)),
    }
}

/// Start execution of runbook milestones. Spawns background work on tokio
/// and emits events as execution progresses.
#[tauri::command]
pub async fn start_execution(
    app: AppHandle,
    runbook_path: String,
    repo_dir: String,
) -> Result<String, String> {
    let state = app.state::<AppState>();

    // Check if execution is already running
    if state.execution_running.load(Ordering::Relaxed) {
        return Err("Execution is already running".to_string());
    }

    let repo = Path::new(&repo_dir);
    if !repo.exists() {
        return Err(format!("Repository directory not found: {}", repo_dir));
    }

    let rb_path = if Path::new(&runbook_path).is_absolute() {
        std::path::PathBuf::from(&runbook_path)
    } else {
        repo.join(&runbook_path)
    };

    if !rb_path.exists() {
        return Err(format!("Runbook not found: {}", rb_path.display()));
    }

    // Parse tracker to check initial state
    let content = fs::read_to_string(&rb_path)
        .map_err(|e| format!("Failed to read runbook: {}", e))?;
    let rows = runbook::parse_tracker(&content);

    if runbook::all_done(&rows) {
        let total = rows.len() as u32;
        let _ = app.emit(
            "execution-complete",
            ExecutionCompleteEvent {
                all_done: true,
                milestones_completed: total,
                total,
            },
        );
        return Ok("All milestones already done".to_string());
    }

    // Detect build/test commands
    let build_cmds = detect::detect_build_commands(repo);
    let test_cmds = detect::detect_test_commands(repo);

    // Reset cancellation flag and set running
    state.cancel_execution.store(false, Ordering::Relaxed);
    state.execution_running.store(true, Ordering::Relaxed);

    let cancel_flag = state.cancel_execution.clone();
    let running_flag = state.execution_running.clone();
    let runbook_str = runbook_path.clone();
    let repo_dir_owned = repo_dir.clone();

    // Spawn execution loop on a blocking thread so synchronous process I/O
    // does not starve the tokio async runtime.
    tokio::task::spawn_blocking(move || {
        let repo = Path::new(&repo_dir_owned);

        let log_dir = match ensure_log_dir(repo) {
            Ok(d) => d,
            Err(e) => {
                let _ = app.emit(
                    "execution-complete",
                    ExecutionCompleteEvent {
                        all_done: false,
                        milestones_completed: 0,
                        total: 0,
                    },
                );
                eprintln!("Failed to create log dir: {}", e);
                running_flag.store(false, Ordering::Relaxed);
                return;
            }
        };

        let (allow_flags, deny_flags, model, max_attempts, cooldown_secs) = {
            let state_ref = app.state::<AppState>();
            let settings = state_ref.settings.lock().unwrap();
            (
                settings.allow_flags.clone(),
                settings.deny_flags.clone(),
                settings.model.clone(),
                settings.max_attempts,
                settings.cooldown_secs,
            )
        };
        let mut attempt: u32 = 0;
        let mut milestones_completed: u32 = 0;

        while attempt < max_attempts {
            // Check cancellation
            if cancel_flag.load(Ordering::Relaxed) {
                break;
            }

            // Re-read runbook each iteration (it may have been modified)
            let content = match fs::read_to_string(&rb_path) {
                Ok(c) => c,
                Err(_) => break,
            };
            let rows = runbook::parse_tracker(&content);
            let total = rows.len() as u32;

            if runbook::all_done(&rows) {
                milestones_completed = total;
                let _ = app.emit(
                    "execution-complete",
                    ExecutionCompleteEvent {
                        all_done: true,
                        milestones_completed: total,
                        total,
                    },
                );
                break;
            }

            let next = match runbook::next_incomplete(&rows) {
                Some(row) => row.clone(),
                None => break,
            };

            attempt += 1;

            // Emit milestone-started
            let _ = app.emit(
                "milestone-started",
                MilestoneStartedEvent {
                    milestone_number: next.number,
                    title: next.title.clone(),
                    attempt,
                },
            );

            // Build prompt
            let prompt = build_execution_prompt(
                &runbook_str,
                &build_cmds,
                &test_cmds,
                attempt,
            );

            // Create log file
            let log_filename =
                format!("milestone-{}-attempt-{}.log", next.number, attempt);
            let log_file = match LogFile::new(&log_dir, &log_filename) {
                Ok(lf) => lf,
                Err(_) => break,
            };

            let _ = log_file.append(&format!(
                "Attempt {} — Milestone {}: {}",
                attempt, next.number, next.title
            ));

            // Invoke Copilot, streaming output as events
            let app_handle = app.clone();
            let invocation = CopilotInvocation {
                prompt,
                model: model.clone(),
                allow_flags: allow_flags.clone(),
                deny_flags: deny_flags.clone(),
                working_dir: repo.to_path_buf(),
            };

            let exit_code = invocation.run_with_callback(&log_file, |line, stream| {
                let _ = app_handle.emit(
                    "execution-progress",
                    ExecutionProgressEvent {
                        line: line.to_string(),
                        stream: stream.to_string(),
                        timestamp: Utc::now().to_rfc3339(),
                    },
                );
            });

            match exit_code {
                Ok(code) => {
                    let _ = log_file.append(&format!("copilot exited with code {}", code));
                }
                Err(e) => {
                    let _ = log_file.append(&format!("copilot error: {}", e));
                }
            }

            // Run build verification
            for cmd in &build_cmds {
                let (success, output) = run_verification_command(cmd);
                let _ = app.emit(
                    "build-test-result",
                    BuildTestResultEvent {
                        command: cmd.clone(),
                        success,
                        output: output.chars().take(5000).collect(),
                    },
                );
            }

            // Run test verification
            for cmd in &test_cmds {
                let (success, output) = run_verification_command(cmd);
                let _ = app.emit(
                    "build-test-result",
                    BuildTestResultEvent {
                        command: cmd.clone(),
                        success,
                        output: output.chars().take(5000).collect(),
                    },
                );
            }

            // Emit milestone-completed
            let _ = app.emit(
                "milestone-completed",
                MilestoneCompletedEvent {
                    milestone_number: next.number,
                    success: true,
                },
            );

            // Re-check if done after this attempt
            if let Ok(content) = fs::read_to_string(&rb_path) {
                let rows = runbook::parse_tracker(&content);
                milestones_completed = rows
                    .iter()
                    .filter(|r| r.status == runbook::MilestoneStatus::Done)
                    .count() as u32;
            }

            // Cooldown between attempts
            if !cancel_flag.load(Ordering::Relaxed) && attempt < max_attempts {
                std::thread::sleep(std::time::Duration::from_secs(cooldown_secs));
            }
        }

        // Final state
        let total = match fs::read_to_string(&rb_path) {
            Ok(content) => {
                let rows = runbook::parse_tracker(&content);
                let t = rows.len() as u32;
                let done = rows
                    .iter()
                    .filter(|r| r.status == runbook::MilestoneStatus::Done)
                    .count() as u32;
                let _ = app.emit(
                    "execution-complete",
                    ExecutionCompleteEvent {
                        all_done: runbook::all_done(&rows),
                        milestones_completed: done,
                        total: t,
                    },
                );
                t
            }
            Err(_) => {
                let _ = app.emit(
                    "execution-complete",
                    ExecutionCompleteEvent {
                        all_done: false,
                        milestones_completed,
                        total: 0,
                    },
                );
                0
            }
        };

        let _ = total; // suppress unused warning
        running_flag.store(false, Ordering::Relaxed);
    });

    Ok("Execution started".to_string())
}

/// Cancel a running execution by setting the cancellation flag.
#[tauri::command]
pub fn cancel_execution(app: AppHandle) -> Result<String, String> {
    let state = app.state::<AppState>();

    if !state.execution_running.load(Ordering::Relaxed) {
        return Err("No execution is currently running".to_string());
    }

    state.cancel_execution.store(true, Ordering::Relaxed);
    Ok("Cancellation requested".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Feature: Execution prompt construction ──────────────────────────

    #[test]
    fn build_execution_prompt_contains_runbook_path() {
        // Given: A runbook path and commands
        let prompt = build_execution_prompt(
            "docs/RUNBOOK.md",
            &["cargo build".to_string()],
            &["cargo test".to_string()],
            1,
        );
        // Then: The prompt contains the runbook path
        assert!(prompt.contains("docs/RUNBOOK.md"));
    }

    #[test]
    fn build_execution_prompt_contains_build_commands() {
        // Given: Build commands
        let prompt = build_execution_prompt(
            "RUNBOOK.md",
            &["cargo build --workspace".to_string()],
            &[],
            1,
        );
        // Then: The prompt contains the build command
        assert!(prompt.contains("cargo build --workspace"));
    }

    #[test]
    fn build_execution_prompt_retry_includes_attempt() {
        // Given: Attempt > 1
        let prompt = build_execution_prompt("RUNBOOK.md", &[], &[], 3);
        // Then: Contains retry context with attempt number
        assert!(prompt.contains("Attempt 3"));
        assert!(prompt.contains("RETRY CONTEXT"));
    }

    #[test]
    fn build_execution_prompt_first_attempt_no_retry() {
        // Given: First attempt
        let prompt = build_execution_prompt("RUNBOOK.md", &[], &[], 1);
        // Then: No retry context
        assert!(!prompt.contains("RETRY CONTEXT"));
    }

    #[test]
    fn build_execution_prompt_no_commands_shows_placeholder() {
        // Given: No build or test commands
        let prompt = build_execution_prompt("RUNBOOK.md", &[], &[], 1);
        // Then: Shows "No ... commands configured"
        assert!(prompt.contains("No build commands configured"));
        assert!(prompt.contains("No test commands configured"));
    }

    // ── Feature: Verification command execution ─────────────────────────

    #[test]
    fn run_verification_command_success() {
        // Given: A command that succeeds
        let (success, output) = run_verification_command("echo hello");
        // Then: It succeeds and captures output
        assert!(success);
        assert!(output.contains("hello"));
    }

    #[test]
    fn run_verification_command_failure() {
        // Given: A command that fails
        let (success, _output) = run_verification_command("false");
        // Then: It reports failure
        assert!(!success);
    }
}
