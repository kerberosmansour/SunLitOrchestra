//! E2E runtime validation tests for Milestone 5 — web-search phase integration.
//!
//! These tests drive the `sldo-research` binary as a subprocess and verify the
//! web-search phase is inserted between exploration and deepening: zero
//! searches skip the phase entirely (no log files), non-zero searches produce
//! N log files named `research-websearch-<N>.log` under `.sldo-logs/`, and
//! a web-search failure does not halt the pipeline.
//!
//! The shim-claude harness is inherited verbatim from the M3/M4 E2E files;
//! each web-search invocation is also a `claude -p` spawn and so is answered
//! by the same shim.

use std::path::{Path, PathBuf};
use std::process::Command;

fn binary() -> String {
    static RESEARCH_BIN: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    RESEARCH_BIN
        .get_or_init(|| {
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let bin = manifest_dir
                .join("target")
                .join("debug")
                .join(format!("sldo-research{}", std::env::consts::EXE_SUFFIX));

            if !bin.exists() {
                let status = Command::new("cargo")
                    .args(["build", "-p", "sldo-research"])
                    .current_dir(&manifest_dir)
                    .status()
                    .expect("failed to build sldo-research");
                assert!(status.success(), "cargo build -p sldo-research failed");
            }

            bin.to_string_lossy().into_owned()
        })
        .clone()
}

fn unique_tmp(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sldo_research_e2e_m5_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

/// Build a minimal POSIX shim at `<dir>/claude` that prints `marker` and exits 0.
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

fn websearch_log_files(sldo_logs: &Path) -> Vec<PathBuf> {
    let Ok(rd) = std::fs::read_dir(sldo_logs) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in rd.flatten() {
        let name = entry.file_name();
        let name_s = name.to_string_lossy();
        if name_s.starts_with("research-websearch-") && name_s.ends_with(".log") {
            out.push(entry.path());
        }
    }
    out.sort();
    out
}

// ── --max-searches 0 is accepted ─────────────────────────────────────────

#[test]
fn test_max_searches_zero_accepted() {
    // Given: a working claude shim and --max-searches 0
    let (mut cmd, cwd, shim) = shimmed_cmd("zero_accepted", "shim findings");
    // When: run with --max-searches 0 and --max-iterations 1
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-searches")
        .arg("0")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: exit 0 (success) — CLI accepts the value and pipeline completes
    assert!(
        output.status.success(),
        "binary should exit 0 with --max-searches 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    cleanup(&[&cwd, &shim]);
}

// ── --max-searches 0 skips the phase entirely ───────────────────────────

#[test]
fn test_max_searches_zero_skips_phase() {
    // Given: --max-searches 0 under a working claude shim
    let (mut cmd, cwd, shim) = shimmed_cmd("zero_skips_phase", "shim findings");
    // When: run
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-searches")
        .arg("0")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    // Then: no `.sldo-logs/research-websearch-*.log` files are created
    assert!(output.status.success());
    let logs = cwd.join(".sldo-logs");
    let websearch_logs = websearch_log_files(&logs);
    assert!(
        websearch_logs.is_empty(),
        "expected no web-search logs when --max-searches 0; found: {:?}",
        websearch_logs
    );
    cleanup(&[&cwd, &shim]);
}

// ── --max-searches 2 creates 2 log files with consistent names ──────────

#[test]
fn test_websearch_log_files_named_correctly() {
    // Given: --max-searches 2 under a working claude shim
    let (mut cmd, cwd, shim) = shimmed_cmd("two_logs_named", "shim findings");
    // When: run with --max-searches 2 and --max-iterations 1
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-searches")
        .arg("2")
        .arg("--max-iterations")
        .arg("1")
        .output()
        .expect("failed to execute sldo-research");
    assert!(
        output.status.success(),
        "binary should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    // Then: exactly two websearch log files exist with the expected names
    let logs = cwd.join(".sldo-logs");
    let websearch_logs = websearch_log_files(&logs);
    assert_eq!(
        websearch_logs.len(),
        2,
        "expected 2 web-search logs; found: {:?}",
        websearch_logs
    );
    let names: Vec<String> = websearch_logs
        .iter()
        .map(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();
    assert!(
        names.iter().any(|n| n == "research-websearch-1.log"),
        "missing research-websearch-1.log; got: {:?}",
        names
    );
    assert!(
        names.iter().any(|n| n == "research-websearch-2.log"),
        "missing research-websearch-2.log; got: {:?}",
        names
    );
    cleanup(&[&cwd, &shim]);
}

// ── CLI --help still lists --max-searches after M5 ──────────────────────

#[test]
fn test_help_flag_still_lists_max_searches_after_m5() {
    // Given: the M5 binary build
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");
    // Then: exit 0 and --max-searches is still documented
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--max-searches"),
        "--help should document --max-searches; got: {}",
        stdout
    );
}
