//! Crate-local integration test for `cargo xtask sast-verify gate`.
//!
//! Asserts that every rule pair in `.semgrep/rust/` passes the deterministic
//! gate. Skipped when `semgrep` is not on PATH (common on CI runners that
//! haven't installed it; the workflow per Runbook A M3 installs it explicitly).

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // CARGO_MANIFEST_DIR for xtasks/sast-verify is .../SunLitOrchestrate/xtasks/sast-verify
    // Walk up two levels.
    p.pop();
    p.pop();
    p
}

fn semgrep_on_path() -> bool {
    Command::new("semgrep")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn xtask_help_lists_five_subcommands() {
    let root = workspace_root();
    let bin_path = root.join("target").join("release").join("sast-verify");
    if !bin_path.exists() {
        // Build hasn't run; skip rather than fail the test suite during early dev.
        eprintln!("skipping: {} not found; run `cargo build -p sast-verify --release` first", bin_path.display());
        return;
    }
    let output = Command::new(&bin_path)
        .arg("--help")
        .output()
        .expect("invoke sast-verify --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for sub in &[
        "validate",
        "test",
        "check-coverage",
        "check-clean",
        "gate",
        "detect-tier",
    ] {
        assert!(
            stdout.contains(sub),
            "--help should mention `{}` subcommand; got:\n{}",
            sub,
            stdout
        );
    }
}

#[test]
fn gate_passes_for_all_authored_rules() {
    if !semgrep_on_path() {
        eprintln!("skipping: semgrep not on PATH");
        return;
    }
    let root = workspace_root();
    let bin_path = root.join("target").join("release").join("sast-verify");
    if !bin_path.exists() {
        eprintln!("skipping: {} not found", bin_path.display());
        return;
    }

    let rules_dir = root.join(".semgrep").join("rust");
    if !rules_dir.is_dir() {
        eprintln!("skipping: {} not present (rule pack not yet authored)", rules_dir.display());
        return;
    }

    let mut tested = 0;
    let mut failed: Vec<String> = vec![];

    for entry in std::fs::read_dir(&rules_dir).expect("read .semgrep/rust dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().map(|e| e == "yaml").unwrap_or(false) {
            tested += 1;
            let output = Command::new(&bin_path)
                .arg("gate")
                .arg(&path)
                .current_dir(&root)
                .output()
                .expect("invoke sast-verify gate");
            if !output.status.success() {
                failed.push(format!(
                    "{}: exit {} | stderr={}",
                    path.file_name().unwrap().to_string_lossy(),
                    output.status.code().unwrap_or(-1),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
    }

    assert!(tested >= 1, "expected at least one rule in .semgrep/rust/");
    assert!(
        failed.is_empty(),
        "{}/{} rules failed gate:\n{}",
        failed.len(),
        tested,
        failed.join("\n")
    );
}

#[test]
fn detect_tier_runs_returns_confidential_or_public() {
    let root = workspace_root();
    let bin_path = root.join("target").join("release").join("sast-verify");
    if !bin_path.exists() {
        eprintln!("skipping: {} not found", bin_path.display());
        return;
    }
    let output = Command::new(&bin_path)
        .arg("detect-tier")
        .current_dir(&root)
        .output()
        .expect("invoke sast-verify detect-tier");
    assert!(output.status.success(), "detect-tier should always exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.trim() == "Confidential" || stdout.trim() == "Public",
        "detect-tier stdout must be one of {{Confidential, Public}}; got: {}",
        stdout
    );
    // Per /slo-critique eng-4: default-deny — Confidential is the v1 outcome
    // for every successfully-parsed remote.
    assert_eq!(
        stdout.trim(),
        "Confidential",
        "v1 default-deny: detect-tier returns Confidential for every remote shape"
    );
}
