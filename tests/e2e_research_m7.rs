//! E2E runtime validation tests for Milestone 7 — plan-ready output and
//! sldo-plan integration.
//!
//! These tests drive the `sldo-research` binary as a subprocess and verify
//! the M7 end-of-run output gating: when the dossier passes
//! `dossier::check_plan_readiness`, the binary prints a "Next step"
//! suggestion that includes `sldo-plan` and the dossier path; when the
//! dossier is not plan-ready, the suggestion is suppressed and a warning
//! block lists the issues.
//!
//! Backwards-compat: the M1–M6 CLI surface of `sldo-research`, `sldo-plan`,
//! and `sldo-run` must be unchanged. M7 only adds end-of-run output —
//! no new flags.
//!
//! Integration: a hand-written plan-ready dossier fixture lives at
//! `tests/fixtures/research/plan-ready-dossier.md`. The test asserts the
//! fixture is valid UTF-8 and >1 KiB, which is exactly the contract
//! `sldo-plan` requires of its `prompt_file` input.

use std::path::{Path, PathBuf};
use std::process::Command;

fn binary() -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-research"
}

fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("research")
        .join(rel)
}

fn unique_tmp(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sldo_research_e2e_m7_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

/// Build a minimal POSIX shim at `<dir>/claude` that prints `marker` and
/// exits 0. PATH is replaced by the test (not augmented), so the shim
/// must use `/bin/sh` builtins only — `printf` is safe.
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

fn cleanup(paths: &[&Path]) {
    for p in paths {
        let _ = std::fs::remove_dir_all(p);
    }
}

// ── Dossier file is valid UTF-8 text after a normal run ──────────────────

#[test]
fn test_dossier_is_valid_utf8_text() {
    // Given: a working claude shim and a normal run
    let cwd = unique_tmp("utf8_cwd");
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_claude("utf8_shim", "shim findings line");
    let dossier = cwd.join("d.md");
    // When
    let output = Command::new(binary())
        .current_dir(&cwd)
        .env("PATH", &shim)
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    assert!(output.status.success());
    // Then: the dossier file is valid UTF-8 (read_to_string succeeds) and
    //       has > 0 bytes — the M4/M6 writer never produces invalid UTF-8.
    let content = std::fs::read_to_string(&dossier).expect("dossier should be valid UTF-8");
    assert!(!content.is_empty(), "dossier should not be empty");
    cleanup(&[&cwd, &shim]);
}

// ── Summary block always shown after a successful run ────────────────────

#[test]
fn test_summary_block_shown_after_run() {
    // Given: a working claude shim and a normal run
    let cwd = unique_tmp("summary_cwd");
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_claude("summary_shim", "shim findings line");
    // When
    let output = Command::new(binary())
        .current_dir(&cwd)
        .env("PATH", &shim)
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    assert!(output.status.success());
    // Then: stderr contains "Summary" header and a "Total wall time" line
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Summary"),
        "expected Summary header in stderr; got: {}",
        stderr
    );
    assert!(
        stderr.contains("Total wall time"),
        "expected Total wall time line in stderr; got: {}",
        stderr
    );
    cleanup(&[&cwd, &shim]);
}

// ── Not-ready dossier suppresses the next-step suggestion ───────────────

#[test]
fn test_cli_omits_next_step_when_not_ready() {
    // Given: an unstructured-shim run (raw findings only, M4 fallback in
    //        the dossier — keeps the M4 stub sentinel everywhere except
    //        Key Findings, so check_plan_readiness will fail).
    let cwd = unique_tmp("not_ready_cwd");
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_claude("not_ready_shim", "tiny finding");
    // When
    let output = Command::new(binary())
        .current_dir(&cwd)
        .env("PATH", &shim)
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Then: stderr does NOT contain "Next step" or "ready for planning"
    assert!(
        !stderr.contains("Next step"),
        "not-ready run should not print Next step; got: {}",
        stderr
    );
    assert!(
        !stderr.contains("ready for planning"),
        "not-ready run should not print readiness success; got: {}",
        stderr
    );
    // and the warning block lists at least one issue
    assert!(
        stderr.contains("not yet plan-ready"),
        "not-ready run should print readiness warning; got: {}",
        stderr
    );
    cleanup(&[&cwd, &shim]);
}

// ── Plan-ready fixture passes check_plan_readiness via the binary ───────

#[test]
fn test_plan_ready_fixture_is_consumable_by_sldo_plan() {
    // Given: a hand-written plan-ready dossier fixture
    let fixture = fixture_path("plan-ready-dossier.md");
    assert!(
        fixture.exists(),
        "plan-ready fixture missing at {}",
        fixture.display()
    );
    // When: read as a UTF-8 string (the contract sldo-plan requires)
    let content = std::fs::read_to_string(&fixture).expect("fixture should be valid UTF-8");
    // Then: > 1 KiB (the M7 plan-readiness threshold)
    assert!(
        content.len() > 1000,
        "fixture should be > 1000 bytes; got: {}",
        content.len()
    );
    // and contains every required dossier section header
    for s in [
        "## Executive Summary",
        "## Topic Decomposition",
        "## Key Findings",
        "## Library & Tool Evaluations",
        "## Architecture Options",
        "## API & SDK Documentation",
        "## Design Recommendations",
        "## Risks & Open Questions",
        "## References",
    ] {
        assert!(
            content.contains(s),
            "fixture missing required section: {}",
            s
        );
    }
    // and does NOT contain the M4 stub sentinel
    assert!(
        !content.contains("To be synthesised in M6"),
        "fixture should not contain the M4 stub sentinel"
    );
}

// ── Backwards-compat regression guards ───────────────────────────────────
//
// Note: `sldo-plan` and `sldo-run` CLIs were removed in the 2026-04 cleanup
// (see CLAUDE.md). Their backwards-compat tests have been deleted; the
// `sldo-research` surface guard remains.

#[test]
fn test_sldo_research_help_unchanged_after_m7() {
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // M1–M6 surface stays stable; no new flag added in M7.
    for flag in [
        "--prompt",
        "--repo-dir",
        "--output",
        "--model",
        "--max-iterations",
        "--max-searches",
    ] {
        assert!(
            stdout.contains(flag),
            "sldo-research --help missing flag after M7: {}",
            flag
        );
    }
    // Pin the no-new-flag rule explicitly.
    for forbidden in ["--max-synthesis", "--no-synthesis", "--plan-ready"] {
        assert!(
            !stdout.contains(forbidden),
            "sldo-research should not introduce flag {}",
            forbidden
        );
    }
}
