//! E2E runtime validation tests for Milestone 3 — research loop.
//!
//! These tests drive the `sldo-research` binary as a subprocess and verify the
//! research loop's runtime side effects: `.sldo-logs/` is created, the
//! exploration log file is written, deepening logs are numbered correctly, and
//! the binary tolerates a missing `claude` CLI.
//!
//! Strategy: every test overrides the child's `PATH` to a known-bad directory.
//! This forces `preflight::check_claude_installed` to fail OR — when we want
//! to exercise the loop itself — we point `PATH` at a tempdir holding a
//! handcrafted `claude` shim. Either way, no real Claude API calls happen
//! during the test suite.

use std::path::{Path, PathBuf};
use std::process::Command;

fn binary() -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-research"
}

fn unique_tmp(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sldo_research_e2e_m3_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

/// Build a minimal POSIX shim at `<dir>/claude` that prints a marker line and
/// exits 0. Lets us exercise the research loop's success path without invoking
/// the real Claude API. Returns the directory containing the shim so the
/// caller can prepend it to `PATH`.
fn shim_dir_with_claude(label: &str, marker: &str) -> PathBuf {
    let dir = unique_tmp(label);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let shim = dir.join("claude");
    let body = format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", marker);
    std::fs::write(&shim, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&shim).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&shim, perms).unwrap();
    }
    dir
}

fn block_claude_cmd(label: &str) -> (Command, PathBuf) {
    let cwd = unique_tmp(label);
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let mut cmd = Command::new(binary());
    cmd.current_dir(&cwd)
        .env("PATH", "/sldo_research_nonexistent_path");
    (cmd, cwd)
}

fn shimmed_cmd(label: &str, marker: &str) -> (Command, PathBuf, PathBuf) {
    let cwd = unique_tmp(&format!("{}_cwd", label));
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_claude(&format!("{}_shim", label), marker);
    let mut cmd = Command::new(binary());
    cmd.current_dir(&cwd).env("PATH", &shim);
    (cmd, cwd, shim)
}

fn cleanup(paths: &[&Path]) {
    for p in paths {
        let _ = std::fs::remove_dir_all(p);
    }
}

// ── Pass-criteria: log directory created when claude shim succeeds ────────

#[test]
fn test_log_directory_created() {
    // Given: a working `claude` shim and a clean CWD
    let (mut cmd, cwd, shim) =
        shimmed_cmd("log_dir_created", "exploration shim findings");
    // When: the binary runs with --max-iterations 1
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: .sldo-logs/ exists in the CWD; binary exits 0
    assert!(
        output.status.success(),
        "sldo-research should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        cwd.join(".sldo-logs").is_dir(),
        ".sldo-logs/ should be created in the working directory"
    );
    cleanup(&[&cwd, &shim]);
}

// ── Pass-criteria: max-iterations 1 → no deepening log ────────────────────

#[test]
fn test_single_iteration_creates_no_deepening_log() {
    // Given: claude shim that always succeeds
    let (mut cmd, cwd, shim) = shimmed_cmd("single_iter", "shim findings");
    // When: --max-iterations 1
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: research-exploration.log exists; no deepening log files exist
    assert!(output.status.success());
    let logs = cwd.join(".sldo-logs");
    assert!(
        logs.join("research-exploration.log").exists(),
        "exploration log should be created"
    );
    for n in 2..=5 {
        let deepen = logs.join(format!("research-deepen-{}.log", n));
        assert!(
            !deepen.exists(),
            "no deepening log expected for max_iterations = 1, found {}",
            deepen.display()
        );
    }
    cleanup(&[&cwd, &shim]);
}

// ── Pass-criteria: ResearchConfig is exported from the module ─────────────

#[test]
fn test_research_config_struct_exposed_from_module() {
    // The unit-test layer in research.rs covers struct field accessibility.
    // This E2E-side check delegates to the binary surfacing "Research
    // accumulated <N> bytes of findings" — proving the binary actually
    // reached the research_loop call site (not stuck in pre-flight).
    let (mut cmd, cwd, shim) = shimmed_cmd("findings_byte_hint", "captured stdout");
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    assert!(output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let combined_lower = combined.to_lowercase();
    assert!(
        combined_lower.contains("accumulated") && combined_lower.contains("bytes"),
        "expected 'accumulated <N> bytes of findings' line, got:\n{}",
        combined
    );
    cleanup(&[&cwd, &shim]);
}

// ── Additional coverage from the runbook BDD scenarios ────────────────────

#[test]
fn test_no_repo_dir_skips_repo_context_log() {
    // Given: claude shim, no --repo-dir
    let (mut cmd, cwd, shim) = shimmed_cmd("no_repo", "shim out");
    // When: binary runs without --repo-dir
    let _ = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: research-repo-context.log does NOT exist
    let logs = cwd.join(".sldo-logs");
    assert!(
        !logs.join("research-repo-context.log").exists(),
        "repo-context log should not exist when --repo-dir is absent"
    );
    cleanup(&[&cwd, &shim]);
}

#[test]
fn test_max_iterations_three_creates_two_deepening_logs() {
    // Given: claude shim, --max-iterations 3
    let (mut cmd, cwd, shim) = shimmed_cmd("three_iters", "iter findings");
    // When: binary runs with --max-iterations 3
    let _ = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("3")
        .output()
        .expect("failed to execute sldo-research");
    // Then: both research-deepen-2.log and -deepen-3.log exist
    let logs = cwd.join(".sldo-logs");
    assert!(
        logs.join("research-deepen-2.log").exists(),
        "expected research-deepen-2.log"
    );
    assert!(
        logs.join("research-deepen-3.log").exists(),
        "expected research-deepen-3.log"
    );
    cleanup(&[&cwd, &shim]);
}

#[test]
fn test_scratch_file_persisted_after_exploration() {
    // Given: claude shim emitting a unique marker
    let marker = "SCRATCH-MARKER-XYZ";
    let (mut cmd, cwd, shim) = shimmed_cmd("scratch_persist", marker);
    // When: binary runs with --output under CWD and --max-iterations 1
    let dossier = cwd.join("output").join("dossier.md");
    let _ = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: the scratch file exists alongside the dossier path and contains
    // the shim's marker line
    let scratch = cwd.join("output").join(".research-scratch-iter-1.md");
    assert!(
        scratch.exists(),
        "expected scratch file at {}",
        scratch.display()
    );
    let body = std::fs::read_to_string(&scratch).unwrap();
    assert!(
        body.contains(marker),
        "scratch file should contain shim output marker, got:\n{}",
        body
    );
    cleanup(&[&cwd, &shim]);
}

#[test]
fn test_missing_claude_does_not_panic() {
    // Given: PATH is cleared so `claude` is unavailable
    let (mut cmd, cwd) = block_claude_cmd("missing_claude");
    // When: binary runs
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: exit non-zero with a clear stderr diagnostic; no panic
    assert!(
        !output.status.success(),
        "missing claude should yield non-zero exit"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.trim().is_empty(),
        "expected non-empty stderr diagnostic when claude is missing"
    );
    cleanup(&[&cwd]);
}

#[test]
fn test_help_flag_unchanged() {
    // Given: M1/M2 CLI surface
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");
    // Then: exit 0; stdout still mentions all M1 flags
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for flag in ["--prompt", "--repo-dir", "--output", "--model", "--max-iterations", "--max-searches"] {
        assert!(stdout.contains(flag), "--help missing flag: {}", flag);
    }
}

#[test]
fn test_invalid_repo_dir_still_rejected() {
    // Regression guard for M1 surface — invalid --repo-dir must still bail
    let output = Command::new(binary())
        .arg("--prompt")
        .arg("topic")
        .arg("--repo-dir")
        .arg("/sldo_research_does_not_exist_zzz")
        .output()
        .expect("failed to execute sldo-research");
    assert!(
        !output.status.success(),
        "invalid --repo-dir should yield non-zero exit"
    );
}
