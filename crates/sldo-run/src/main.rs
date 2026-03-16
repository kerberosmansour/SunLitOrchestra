use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use clap::Parser;

use sldo_common::color::{divider, fail, header, info, success, warn};
use sldo_common::copilot::CopilotInvocation;
use sldo_common::detect;
use sldo_common::logging::{ensure_log_dir, LogFile};
use sldo_common::preflight;
use sldo_common::runbook;
use sldo_common::toolflags;

/// Drive GitHub Copilot CLI through milestones in a runbook, one at a time,
/// verifying build and tests after each pass.
#[derive(Parser, Debug)]
#[command(name = "sldo-run", about = "Drive GitHub Copilot CLI through runbook milestones")]
struct Cli {
    /// Path to the runbook markdown file (relative to repo or absolute)
    runbook: PathBuf,

    /// Path to the target repository
    repo_dir: PathBuf,

    /// Copilot model to use
    #[arg(short, long, default_value = "claude-opus-4.6")]
    model: String,

    /// Max Copilot invocations before giving up
    #[arg(short = 'a', long, default_value_t = 150)]
    max_attempts: u32,

    /// Pause between retries in seconds
    #[arg(short = 'c', long, default_value_t = 5)]
    cooldown: u64,

    /// Custom build verification command (repeatable)
    #[arg(long = "build-cmd", action = clap::ArgAction::Append)]
    build_cmds: Vec<String>,

    /// Custom test verification command (repeatable)
    #[arg(long = "test-cmd", action = clap::ArgAction::Append)]
    test_cmds: Vec<String>,
}

/// Build a summary of commands for embedding in the prompt.
fn build_cmd_summary(label: &str, cmds: &[String]) -> String {
    if cmds.is_empty() {
        return format!("- No {} commands configured.", label);
    }
    cmds.iter()
        .map(|cmd| format!("- `{}`", cmd))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Build the execution prompt, mirroring the Bash `build_prompt()` function.
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
3. Complete that milestone — and ONLY that milestone — following the runbook's Pre-Milestone Protocol, the milestone's Step-by-Step, BDD Acceptance Scenarios, E2E Runtime Validation, Smoke Tests, and Post-Milestone Protocol.
4. If an architecture doc exists (e.g. `docs/ARCHITECTURE.md`), read it for context.
5. If a lessons file exists for the previous milestone (check the Milestone Tracker), read it and apply any corrections.

## Workflow

- **Verify baseline**: Run the project's build and test commands before changing anything. Fix any failures first.
  Build commands:
{build_summary}
  Test commands:
{test_summary}
- **Update tracker**: Set the milestone status to `in_progress` and Started date to {today}.
- **Write tests FIRST**: Create BDD and E2E test files from the milestone's scenario tables, following the test file conventions described in the runbook.
- **Implement**: Follow the Step-by-Step section. Make all tests pass.
- **Post-milestone**: Run smoke tests, write lessons-learned per the Post-Milestone Protocol, update tracker to `done`, update documentation per the Documentation Update table.
- **Final check**: Run all build and test commands one last time — everything must be green.
- **STOP**: Do not proceed to the next milestone. Your session ends here.

## Hard rules
- Write BDD tests BEFORE production code.
- Do NOT skip any step in the Pre-Milestone or Post-Milestone protocols.
- Do NOT touch code or tests belonging to other milestones.
- All pre-existing tests must still pass.
- Follow existing code style and naming conventions.
- Do not commit secrets, API keys, or credentials.
- Do NOT work on any subsequent milestone.
- Do NOT delete or modify any files in `.copilot-logs/` — the automation script writes its logs there.
- When running smoke tests that need a temporary directory, create it under `output/` and clean up only that directory afterward."#
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

A previous Copilot session did not fully complete a milestone. Please:
1. Check the current project state:{retry_cmds}
2. Read existing code and test files to understand what was already done.
3. Fix any issues and complete the next incomplete milestone.
4. Run all build and test commands to verify everything passes.
5. STOP after that one milestone is complete."#
        ));
    }

    prompt
}

/// Run a set of verification commands, returning true if all succeed.
pub fn verify_commands(label: &str, cmds: &[String], log_file: &LogFile) -> bool {
    if cmds.is_empty() {
        return true;
    }

    let mut all_ok = true;
    for cmd in cmds {
        info(&format!("Running {}: {}", label, cmd));
        let result = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let _ = log_file.append(&format!("{} stdout: {}", cmd, stdout));
                if !stderr.is_empty() {
                    let _ = log_file.append(&format!("{} stderr: {}", cmd, stderr));
                }

                if output.status.success() {
                    success(&format!("{} OK: {}", label, cmd));
                } else {
                    warn(&format!("{} failed: {}. Will retry.", label, cmd));
                    all_ok = false;
                }
            }
            Err(e) => {
                fail(&format!("Failed to run {}: {} — {}", label, cmd, e));
                let _ = log_file.append(&format!("FAILED to run {}: {}", cmd, e));
                all_ok = false;
            }
        }
    }
    all_ok
}

/// Print the current tracker state with coloured output.
fn print_tracker_state(rows: &[runbook::MilestoneRow]) {
    for row in rows {
        let line = format!(
            "  | {} | {} | {} |",
            row.number, row.title, row.status
        );
        if row.status == runbook::MilestoneStatus::Done {
            success(&line);
        } else {
            warn(&line);
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Resolve repo directory to absolute path
    let repo_dir = cli
        .repo_dir
        .canonicalize()
        .with_context(|| format!("Repository directory not found: {}", cli.repo_dir.display()))?;

    header("Automated Milestone Runner");
    info(&format!("Repository:   {}", repo_dir.display()));
    info(&format!("Runbook:      {}", cli.runbook.display()));
    info(&format!("Model:        {}", cli.model));
    info(&format!("Max attempts: {}", cli.max_attempts));

    // Preflight checks
    header("Pre-flight checks");
    preflight::check_copilot_installed()?;
    success("copilot CLI found");

    // Resolve runbook path
    let runbook_path = if cli.runbook.is_absolute() {
        cli.runbook.clone()
    } else {
        repo_dir.join(&cli.runbook)
    };
    preflight::check_file_exists(&runbook_path, "Runbook")?;
    success(&format!("Runbook found: {}", runbook_path.display()));

    let branch = preflight::check_git_safety(&repo_dir)?;
    success(&format!(
        "Branch '{}' is not main/master — safe to proceed.",
        branch
    ));

    // Detect or use provided build/test commands
    let mut build_cmds = cli.build_cmds;
    let mut test_cmds = cli.test_cmds;

    if build_cmds.is_empty() || test_cmds.is_empty() {
        info("Auto-detecting build and test commands…");
        if build_cmds.is_empty() {
            build_cmds = detect::detect_build_commands(&repo_dir);
        }
        if test_cmds.is_empty() {
            test_cmds = detect::detect_test_commands(&repo_dir);
        }
    }

    if !build_cmds.is_empty() {
        info("Build commands:");
        for cmd in &build_cmds {
            info(&format!("  {}", cmd));
        }
    } else {
        warn("No build commands detected. Use --build-cmd to specify manually.");
    }
    if !test_cmds.is_empty() {
        info("Test commands:");
        for cmd in &test_cmds {
            info(&format!("  {}", cmd));
        }
    } else {
        warn("No test commands detected. Use --test-cmd to specify manually.");
    }

    let log_dir = ensure_log_dir(&repo_dir)?;
    success(&format!("Log directory: {}", log_dir.display()));

    // Runbook path string for the prompt (relative to repo if possible)
    let runbook_str = cli.runbook.to_string_lossy().to_string();

    let allow_flags = toolflags::run_allow_flags();
    let deny_flags = toolflags::run_deny_flags();

    let start_time = Instant::now();
    let mut attempt: u32 = 0;

    while attempt < cli.max_attempts {
        // Read runbook and parse tracker
        let content = fs::read_to_string(&runbook_path)
            .with_context(|| format!("Failed to read runbook: {}", runbook_path.display()))?;
        let rows = runbook::parse_tracker(&content);

        if runbook::all_done(&rows) {
            success("All milestones in the runbook are marked done!");
            break;
        }

        attempt += 1;
        let next = runbook::next_incomplete(&rows);
        let (ms_num, ms_title) = match next {
            Some(row) => (row.number, row.title.clone()),
            None => {
                success("All milestones in the runbook are marked done!");
                break;
            }
        };

        divider();
        info(&format!(
            "Attempt {}/{} — Milestone {}: {}",
            attempt, cli.max_attempts, ms_num, ms_title
        ));

        print_tracker_state(&rows);
        divider();

        // Build prompt
        let prompt =
            build_execution_prompt(&runbook_str, &build_cmds, &test_cmds, attempt);

        // Log
        let log_filename = format!("milestone-{}-attempt-{}.log", ms_num, attempt);
        let log_file = LogFile::new(&log_dir, &log_filename)?;
        log_file.append(&format!(
            "Attempt {} — Milestone {}: {}",
            attempt, ms_num, ms_title
        ))?;

        // Invoke copilot
        let invocation = CopilotInvocation {
            prompt,
            model: cli.model.clone(),
            allow_flags: allow_flags.clone(),
            deny_flags: deny_flags.clone(),
            working_dir: repo_dir.clone(),
        };

        let exit_code = invocation.run(&log_file)?;
        log_file.append(&format!("copilot exited with code {}", exit_code))?;

        // Verify build + tests
        verify_commands("Build", &build_cmds, &log_file);
        verify_commands("Test", &test_cmds, &log_file);

        if attempt < cli.max_attempts {
            info(&format!("Cooling down {}s…", cli.cooldown));
            thread::sleep(Duration::from_secs(cli.cooldown));
        }
    }

    // Final tracker state
    let content = fs::read_to_string(&runbook_path)
        .with_context(|| format!("Failed to read runbook: {}", runbook_path.display()))?;
    let rows = runbook::parse_tracker(&content);

    header("Final Tracker State");
    for row in &rows {
        let line = format!(
            "  | {} | {} | {} |",
            row.number, row.title, row.status
        );
        if row.status == runbook::MilestoneStatus::Done {
            success(&line);
        } else {
            fail(&line);
        }
    }

    let elapsed = start_time.elapsed();
    let hours = elapsed.as_secs() / 3600;
    let minutes = (elapsed.as_secs() % 3600) / 60;

    if runbook::all_done(&rows) {
        success("All milestones completed!");
    } else {
        fail(&format!(
            "Not all milestones completed after {} attempts.",
            cli.max_attempts
        ));
    }
    info(&format!("Total wall time: {}h {}m", hours, minutes));

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
    use clap::Parser;

    // ── Feature: CLI argument parsing ───────────────────────────────────────

    #[test]
    fn help_flag() {
        // Given: N/A
        // When: sldo-run --help is parsed
        let result = Cli::try_parse_from(["sldo-run", "--help"]);
        // Then: clap returns an error of kind DisplayHelp (exit 0)
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn missing_args() {
        // Given: No arguments
        // When: sldo-run is parsed with no args
        let result = Cli::try_parse_from(["sldo-run"]);
        // Then: Returns error (missing required args)
        assert!(result.is_err());
    }

    #[test]
    fn default_model() {
        // Given: sldo-run runbook.md /tmp/repo
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-run", "runbook.md", "/tmp/repo"]).unwrap();
        // Then: model is "claude-opus-4.6"
        assert_eq!(cli.model, "claude-opus-4.6");
    }

    #[test]
    fn custom_max_attempts() {
        // Given: sldo-run rb.md . -a 50
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-run", "rb.md", ".", "-a", "50"]).unwrap();
        // Then: max_attempts is 50
        assert_eq!(cli.max_attempts, 50);
    }

    #[test]
    fn multiple_build_cmds() {
        // Given: sldo-run rb.md . --build-cmd "make" --build-cmd "npm run build"
        // When: Args are parsed
        let cli = Cli::try_parse_from([
            "sldo-run",
            "rb.md",
            ".",
            "--build-cmd",
            "make",
            "--build-cmd",
            "npm run build",
        ])
        .unwrap();
        // Then: build_cmds has 2 entries
        assert_eq!(cli.build_cmds.len(), 2);
        assert_eq!(cli.build_cmds[0], "make");
        assert_eq!(cli.build_cmds[1], "npm run build");
    }

    #[test]
    fn default_cooldown() {
        // Given: sldo-run runbook.md /tmp/repo (no -c flag)
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-run", "runbook.md", "/tmp/repo"]).unwrap();
        // Then: cooldown is 5
        assert_eq!(cli.cooldown, 5);
    }

    #[test]
    fn custom_cooldown() {
        // Given: sldo-run rb.md . -c 10
        // When: Args are parsed
        let cli = Cli::try_parse_from(["sldo-run", "rb.md", ".", "-c", "10"]).unwrap();
        // Then: cooldown is 10
        assert_eq!(cli.cooldown, 10);
    }

    #[test]
    fn multiple_test_cmds() {
        // Given: sldo-run rb.md . --test-cmd "cargo test" --test-cmd "npm test"
        // When: Args are parsed
        let cli = Cli::try_parse_from([
            "sldo-run",
            "rb.md",
            ".",
            "--test-cmd",
            "cargo test",
            "--test-cmd",
            "npm test",
        ])
        .unwrap();
        // Then: test_cmds has 2 entries
        assert_eq!(cli.test_cmds.len(), 2);
    }

    // ── Feature: Prompt construction ────────────────────────────────────────

    #[test]
    fn first_attempt_prompt() {
        // Given: attempt=1, runbook_path="docs/RUNBOOK.md"
        let build_cmds = vec!["cargo build --workspace".to_string()];
        let test_cmds = vec!["cargo test --workspace".to_string()];
        // When: build_execution_prompt is called
        let prompt = build_execution_prompt("docs/RUNBOOK.md", &build_cmds, &test_cmds, 1);
        // Then: Contains "Read the runbook", build/test commands, does NOT contain "RETRY CONTEXT"
        assert!(prompt.contains("Read the runbook"), "Should contain 'Read the runbook'");
        assert!(prompt.contains("cargo build --workspace"), "Should contain build cmd");
        assert!(prompt.contains("cargo test --workspace"), "Should contain test cmd");
        assert!(
            !prompt.contains("RETRY CONTEXT"),
            "First attempt should not contain RETRY CONTEXT"
        );
    }

    #[test]
    fn retry_prompt() {
        // Given: attempt=3
        let build_cmds = vec!["cargo build --workspace".to_string()];
        let test_cmds = vec!["cargo test --workspace".to_string()];
        // When: build_execution_prompt is called with attempt=3
        let prompt = build_execution_prompt("docs/RUNBOOK.md", &build_cmds, &test_cmds, 3);
        // Then: Contains "RETRY CONTEXT — Attempt 3"
        assert!(
            prompt.contains("RETRY CONTEXT — Attempt 3"),
            "Retry prompt should contain 'RETRY CONTEXT — Attempt 3'"
        );
    }

    // ── Feature: Command verification ───────────────────────────────────────

    #[test]
    fn successful_command() {
        // Given: Command is "true"
        let tmp = std::env::temp_dir().join("sldo_test_verify_ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let log = sldo_common::logging::LogFile::new(&tmp, "test.log").unwrap();
        // When: verify_commands is called
        let result = verify_commands("test", &["true".to_string()], &log);
        // Then: Returns true
        assert!(result, "verify_commands('true') should return true");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn failed_command() {
        // Given: Command is "false"
        let tmp = std::env::temp_dir().join("sldo_test_verify_fail");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let log = sldo_common::logging::LogFile::new(&tmp, "test.log").unwrap();
        // When: verify_commands is called
        let result = verify_commands("test", &["false".to_string()], &log);
        // Then: Returns false
        assert!(!result, "verify_commands('false') should return false");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn empty_command_list() {
        // Given: No commands
        let tmp = std::env::temp_dir().join("sldo_test_verify_empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let log = sldo_common::logging::LogFile::new(&tmp, "test.log").unwrap();
        // When: verify_commands is called with empty list
        let result = verify_commands("test", &[], &log);
        // Then: Returns true (vacuously)
        assert!(result, "verify_commands with empty list should return true");
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

