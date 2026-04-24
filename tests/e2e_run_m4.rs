//! E2E runtime validation tests for Milestone 4 — sldo-run binary.
//!
//! These tests verify that the sldo-run binary correctly parses arguments,
//! constructs prompts, detects build/test commands, and parses the tracker.

use std::path::Path;
use std::process::Command;

#[test]
fn run_help_flag() {
    // Given: The sldo-run binary is built
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-run";
    // When: sldo-run --help is run
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-run --help");
    // Then: Process exits 0, stdout contains "Usage"
    assert!(
        output.status.success(),
        "sldo-run --help should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage") || stdout.contains("usage"),
        "sldo-run --help should contain 'Usage', got: {}",
        stdout
    );
}

#[test]
fn run_missing_args_exits_nonzero() {
    // Given: No arguments
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-run";
    // When: sldo-run is run with no args
    let output = Command::new(&binary)
        .output()
        .expect("failed to execute sldo-run");
    // Then: Process exits non-zero
    assert!(
        !output.status.success(),
        "sldo-run with no args should exit non-zero"
    );
}

#[test]
fn run_detects_cargo_in_own_repo() {
    // Given: This repo has a Cargo.toml
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // When: detect_build_commands is called on this repo
    let build_cmds = sldo_common::detect::detect_build_commands(project_dir);
    // Then: Build commands include "cargo build --workspace"
    assert!(
        build_cmds.contains(&"cargo build --workspace".to_string()),
        "Expected 'cargo build --workspace' in {:?}",
        build_cmds
    );
}

#[test]
fn run_parses_tracker_from_real_runbook() {
    // Given: The actual runbook at docs/RUNBOOK-RUST-REWRITE.md
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runbook_path = Path::new(manifest_dir).join("docs/RUNBOOK-RUST-REWRITE.md");
    let content = std::fs::read_to_string(&runbook_path)
        .expect("Failed to read docs/RUNBOOK-RUST-REWRITE.md");
    // When: parse_tracker is called
    let rows = sldo_common::runbook::parse_tracker(&content);
    // Then: Finds 5 milestones
    assert_eq!(
        rows.len(),
        5,
        "Expected 5 milestones in the tracker, got {}",
        rows.len()
    );
}
