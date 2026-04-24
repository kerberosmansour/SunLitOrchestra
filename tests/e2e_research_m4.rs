//! E2E runtime validation tests for Milestone 4 — dossier format, writer &
//! validator.
//!
//! These tests drive the `sldo-research` binary as a subprocess and verify
//! that after the M3 research loop runs, an M4 dossier is written at the
//! resolved `--output` path, contains every required section header, and the
//! binary exits 0 under the shim-claude harness inherited from M3.

use std::path::{Path, PathBuf};
use std::process::Command;

fn binary() -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-research"
}

fn unique_tmp(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sldo_research_e2e_m4_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

/// Build a minimal POSIX shim at `<dir>/claude` that prints `marker` and
/// exits 0. Mirrors the M3 harness pattern.
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

// ── Happy path: default output ────────────────────────────────────────────

#[test]
fn test_default_output_path_respected() {
    // Given: a working claude shim and the default --output value
    let (mut cmd, cwd, shim) = shimmed_cmd("default_output", "shim findings line");
    // When: run with --max-iterations 1
    let output = cmd
        .arg("--prompt")
        .arg("research the best async runtimes")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: default output path exists under the CWD
    assert!(
        output.status.success(),
        "binary should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let dossier = cwd.join("output").join("research-dossier.md");
    assert!(
        dossier.exists(),
        "expected default dossier at {}",
        dossier.display()
    );
    cleanup(&[&cwd, &shim]);
}

// ── Custom output path ────────────────────────────────────────────────────

#[test]
fn test_custom_output_path_respected() {
    // Given: --output custom.md
    let (mut cmd, cwd, shim) = shimmed_cmd("custom_output", "shim findings line");
    let custom = cwd.join("custom.md");
    // When
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&custom)
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then
    assert!(output.status.success());
    assert!(custom.exists(), "expected dossier at {}", custom.display());
    cleanup(&[&cwd, &shim]);
}

// ── Nested output directory is created ────────────────────────────────────

#[test]
fn test_nested_output_dir_created() {
    // Given: --output points at a path whose parent does not yet exist
    let (mut cmd, cwd, shim) = shimmed_cmd("nested_output", "shim out");
    let nested = cwd.join("deep").join("new").join("dir").join("x.md");
    // When
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&nested)
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then
    assert!(output.status.success());
    assert!(nested.exists(), "expected dossier at {}", nested.display());
    cleanup(&[&cwd, &shim]);
}

// ── Dossier always written, even when findings are empty ─────────────────

#[test]
fn test_dossier_created_when_findings_are_empty() {
    // Given: a shim that prints nothing (empty output) — findings will be empty
    let (mut cmd, cwd, shim) = shimmed_cmd("empty_findings", "");
    let dossier = cwd.join("empty-dossier.md");
    // When
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: binary still exits 0 and a dossier was written with the required
    //       section skeleton.
    assert!(
        output.status.success(),
        "binary should still exit 0 with empty findings; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dossier.exists());
    let content = std::fs::read_to_string(&dossier).unwrap();
    for section in [
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
            content.contains(section),
            "dossier missing required section: {}",
            section
        );
    }
    cleanup(&[&cwd, &shim]);
}

// ── Findings text makes it into the dossier's Key Findings body ──────────

#[test]
fn test_findings_appear_under_key_findings() {
    // Given: a claude shim emitting a unique marker line
    let marker = "DOSSIER-MARKER-M4";
    let (mut cmd, cwd, shim) = shimmed_cmd("findings_under_key", marker);
    let dossier = cwd.join("out.md");
    // When
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: dossier contains the marker and it appears after the Key Findings header
    assert!(output.status.success());
    let content = std::fs::read_to_string(&dossier).unwrap();
    let idx_key = content
        .find("## Key Findings")
        .expect("dossier should contain '## Key Findings' header");
    let idx_marker = content
        .find(marker)
        .unwrap_or_else(|| panic!("dossier should contain marker {}", marker));
    assert!(
        idx_marker > idx_key,
        "marker should appear after the Key Findings header"
    );
    cleanup(&[&cwd, &shim]);
}

// ── Regression: M1 CLI surface unchanged ─────────────────────────────────

#[test]
fn test_help_flag_unchanged_after_m4() {
    // Given: the M1/M2/M3 CLI surface
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");
    // Then: exit 0; all documented flags still present
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for flag in [
        "--prompt",
        "--repo-dir",
        "--output",
        "--model",
        "--max-iterations",
        "--max-searches",
    ] {
        assert!(stdout.contains(flag), "--help missing flag: {}", flag);
    }
}
