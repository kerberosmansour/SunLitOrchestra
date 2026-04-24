//! E2E integration tests for Milestone 5 — Integration tests, docs & migration.
//!
//! These tests exercise the full plan and run workflows using a mock claude
//! script, verify CLI flag parity between Rust and Bash, and validate that
//! safety guards work at runtime.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Return the workspace root directory.
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Return path to a built binary under target/debug/.
fn binary_path(name: &str) -> PathBuf {
    workspace_root().join("target/debug").join(name)
}

/// Create a temporary git repo on a feature branch, returning its path.
/// The repo is initialised with a dummy commit so that branch checks pass.
fn create_temp_git_repo(suffix: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("sldo_e2e_m5_{}", suffix));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    // git init + initial commit on main
    Command::new("git")
        .args(["init"])
        .current_dir(&tmp)
        .output()
        .expect("git init failed");
    Command::new("git")
        .args(["checkout", "-b", "main"])
        .current_dir(&tmp)
        .output()
        .expect("git checkout main failed");

    // Configure git user for commits
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&tmp)
        .output()
        .unwrap();

    // Create initial commit
    fs::write(tmp.join("README.md"), "# Test").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(&tmp)
        .output()
        .unwrap();

    // Switch to feature branch
    Command::new("git")
        .args(["checkout", "-b", "feature/test"])
        .current_dir(&tmp)
        .output()
        .expect("git checkout feature/test failed");

    tmp
}

/// Set up a mock claude directory with the mock script renamed as `claude`.
/// Each call uses a unique suffix to avoid race conditions in parallel tests.
fn setup_mock_claude_dir(suffix: &str) -> PathBuf {
    let mock_dir = std::env::temp_dir().join(format!("sldo_e2e_m5_mock_{}", suffix));
    let _ = fs::remove_dir_all(&mock_dir);
    fs::create_dir_all(&mock_dir).unwrap();

    let mock_src = workspace_root().join("tests/fixtures/mock-claude.sh");
    let mock_dst = mock_dir.join("claude");
    fs::copy(&mock_src, &mock_dst).unwrap();
    fs::set_permissions(&mock_dst, fs::Permissions::from_mode(0o755)).unwrap();

    mock_dir
}

/// Build a PATH string that puts our mock claude first.
fn path_with_mock(mock_dir: &Path) -> String {
    let existing_path = std::env::var("PATH").unwrap_or_default();
    format!("{}:{}", mock_dir.display(), existing_path)
}

// ── Feature: End-to-end planning ────────────────────────────────────────────

#[test]
fn plan_end_to_end_with_mock() {
    // Given: Mock claude on PATH, prompt file exists, temp git repo on feature branch
    let mock_dir = setup_mock_claude_dir("plan_e2e");
    let repo = create_temp_git_repo("plan_e2e");
    let prompt_path = workspace_root().join("tests/fixtures/sample-prompt.txt");

    // Copy template to temp repo so sldo-plan can find it
    fs::create_dir_all(repo.join("docs")).unwrap();
    let template_src = workspace_root().join("docs/runbook-template.md");
    if template_src.exists() {
        fs::copy(&template_src, repo.join("docs/runbook-template.md")).unwrap();
    }

    let output_path = repo.join("docs/RUNBOOK.md");

    // When: sldo-plan is run with mock claude
    let output = Command::new(binary_path("sldo-plan"))
        .args([
            prompt_path.to_str().unwrap(),
            repo.to_str().unwrap(),
            "-o",
            "docs/RUNBOOK.md",
        ])
        .env("PATH", path_with_mock(&mock_dir))
        .output()
        .expect("failed to execute sldo-plan");

    // Then: Runbook file exists at docs/RUNBOOK.md, contains "Milestone Tracker", exit code 0
    assert!(
        output.status.success(),
        "sldo-plan should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output_path.exists(),
        "Runbook should exist at {}",
        output_path.display()
    );
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(
        content.contains("Milestone Tracker"),
        "Runbook should contain 'Milestone Tracker'"
    );

    // Cleanup
    let _ = fs::remove_dir_all(&repo);
    let _ = fs::remove_dir_all(&mock_dir);
}

#[test]
fn plan_rejects_protected_branch() {
    // Given: Repo is on `main` branch
    let mock_dir = setup_mock_claude_dir("plan_main");
    let tmp = std::env::temp_dir().join("sldo_e2e_m5_plan_main");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    // git init on main (do NOT switch to feature branch)
    Command::new("git")
        .args(["init"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["checkout", "-b", "main"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    fs::write(tmp.join("README.md"), "# Test").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(&tmp)
        .output()
        .unwrap();

    let prompt_path = workspace_root().join("tests/fixtures/sample-prompt.txt");

    // When: sldo-plan is run on main branch
    let output = Command::new(binary_path("sldo-plan"))
        .args([
            prompt_path.to_str().unwrap(),
            tmp.to_str().unwrap(),
        ])
        .env("PATH", path_with_mock(&mock_dir))
        .output()
        .expect("failed to execute sldo-plan");

    // Then: Exits non-zero with error about protected branch
    assert!(
        !output.status.success(),
        "sldo-plan should reject main branch"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("main") || stderr.contains("protected"),
        "Error should mention main/protected branch, got: {}",
        stderr
    );

    let _ = fs::remove_dir_all(&tmp);
    let _ = fs::remove_dir_all(&mock_dir);
}

// ── Feature: End-to-end execution ───────────────────────────────────────────

#[test]
fn run_end_to_end_with_mock() {
    // Given: Mock claude on PATH, runbook has 1 not_started milestone
    let mock_dir = setup_mock_claude_dir("run_e2e");
    let repo = create_temp_git_repo("run_e2e");

    // Copy sample runbook into the repo
    fs::create_dir_all(repo.join("docs")).unwrap();
    let runbook_path = repo.join("docs/RUNBOOK.md");
    fs::copy(
        workspace_root().join("tests/fixtures/sample-runbook.md"),
        &runbook_path,
    )
    .unwrap();

    // When: sldo-run is run with mock claude (just 1 attempt to avoid long test)
    let output = Command::new(binary_path("sldo-run"))
        .args([
            "docs/RUNBOOK.md",
            repo.to_str().unwrap(),
            "-a",
            "1",
            "-c",
            "0",
        ])
        .env("PATH", path_with_mock(&mock_dir))
        .output()
        .expect("failed to execute sldo-run");

    // Then: Mock claude was invoked (process ran), exit code 0
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // The binary should have run without crashing (exit 0)
    assert!(
        output.status.success(),
        "sldo-run should exit 0, stdout: {}\nstderr: {}",
        stdout,
        stderr
    );

    let _ = fs::remove_dir_all(&repo);
    let _ = fs::remove_dir_all(&mock_dir);
}

#[test]
fn run_detects_all_done() {
    // Given: Runbook has all milestones `done`
    let mock_dir = setup_mock_claude_dir("run_done");
    let repo = create_temp_git_repo("run_all_done");

    fs::create_dir_all(repo.join("docs")).unwrap();
    let runbook_path = repo.join("docs/RUNBOOK.md");

    let all_done_runbook = r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `done` | 2026-03-01 | 2026-03-01 | |
| 2 | Second | `done` | 2026-03-02 | 2026-03-02 | |
"#;
    fs::write(&runbook_path, all_done_runbook).unwrap();

    // When: sldo-run is run against the all-done runbook
    let output = Command::new(binary_path("sldo-run"))
        .args([
            "docs/RUNBOOK.md",
            repo.to_str().unwrap(),
            "-a",
            "1",
            "-c",
            "0",
        ])
        .env("PATH", path_with_mock(&mock_dir))
        .output()
        .expect("failed to execute sldo-run");

    // Then: Prints "All milestones" message, exits immediately
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);
    assert!(
        output.status.success(),
        "sldo-run with all-done runbook should exit 0"
    );
    assert!(
        combined.contains("All milestones") || combined.contains("all milestones"),
        "Should print all-milestones message, got stdout: {}\nstderr: {}",
        stdout,
        stderr
    );

    let _ = fs::remove_dir_all(&repo);
    let _ = fs::remove_dir_all(&mock_dir);
}

#[test]
fn run_rejects_protected_branch() {
    // Given: Repo is on `main` branch
    let mock_dir = setup_mock_claude_dir("run_main");
    let tmp = std::env::temp_dir().join("sldo_e2e_m5_run_main");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    Command::new("git")
        .args(["init"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["checkout", "-b", "main"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    fs::write(tmp.join("README.md"), "# Test").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&tmp)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(&tmp)
        .output()
        .unwrap();

    // Put a runbook in the repo
    fs::create_dir_all(tmp.join("docs")).unwrap();
    fs::copy(
        workspace_root().join("tests/fixtures/sample-runbook.md"),
        tmp.join("docs/RUNBOOK.md"),
    )
    .unwrap();

    // When: sldo-run is run on main branch
    let output = Command::new(binary_path("sldo-run"))
        .args(["docs/RUNBOOK.md", tmp.to_str().unwrap()])
        .env("PATH", path_with_mock(&mock_dir))
        .output()
        .expect("failed to execute sldo-run");

    // Then: Exit non-zero on main branch
    assert!(
        !output.status.success(),
        "sldo-run should reject main branch"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("main") || stderr.contains("protected"),
        "Error should mention main/protected, got: {}",
        stderr
    );

    let _ = fs::remove_dir_all(&tmp);
    let _ = fs::remove_dir_all(&mock_dir);
}

// ── Feature: CLI parity ─────────────────────────────────────────────────────

#[test]
fn cli_flag_parity_plan() {
    // Given: Both sldo-plan and plan-milestones.sh are available
    let plan_binary = binary_path("sldo-plan");

    // When: sldo-plan --help is run
    let output = Command::new(&plan_binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-plan --help");

    let help_text = String::from_utf8_lossy(&output.stdout);

    // Then: All flags from the Bash script are present in the Rust help:
    // -o/--output, -m/--model, -h/--help
    assert!(
        help_text.contains("--output") || help_text.contains("-o"),
        "Plan help should mention --output/-o flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("--model") || help_text.contains("-m"),
        "Plan help should mention --model/-m flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("-h") || help_text.contains("--help"),
        "Plan help should mention -h/--help flag, got: {}",
        help_text
    );
    // Check positional args
    assert!(
        help_text.contains("prompt") || help_text.contains("PROMPT"),
        "Plan help should mention prompt-file positional arg, got: {}",
        help_text
    );
    assert!(
        help_text.contains("repo") || help_text.contains("REPO"),
        "Plan help should mention repo-dir positional arg, got: {}",
        help_text
    );
}

#[test]
fn cli_flag_parity_run() {
    // Given: Both sldo-run and run-milestones.sh are available
    let run_binary = binary_path("sldo-run");

    // When: sldo-run --help is run
    let output = Command::new(&run_binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-run --help");

    let help_text = String::from_utf8_lossy(&output.stdout);

    // Then: All flags from the Bash script are present in the Rust help:
    // -m/--model, -a/--max-attempts, -c/--cooldown, --build-cmd, --test-cmd, -h/--help
    assert!(
        help_text.contains("--model") || help_text.contains("-m"),
        "Run help should mention --model/-m flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("--max-attempts") || help_text.contains("-a"),
        "Run help should mention --max-attempts/-a flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("--cooldown") || help_text.contains("-c"),
        "Run help should mention --cooldown/-c flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("--build-cmd"),
        "Run help should mention --build-cmd flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("--test-cmd"),
        "Run help should mention --test-cmd flag, got: {}",
        help_text
    );
    assert!(
        help_text.contains("-h") || help_text.contains("--help"),
        "Run help should mention -h/--help flag, got: {}",
        help_text
    );
    // Check positional args
    assert!(
        help_text.contains("runbook") || help_text.contains("RUNBOOK"),
        "Run help should mention runbook positional arg, got: {}",
        help_text
    );
    assert!(
        help_text.contains("repo") || help_text.contains("REPO"),
        "Run help should mention repo-dir positional arg, got: {}",
        help_text
    );
}
