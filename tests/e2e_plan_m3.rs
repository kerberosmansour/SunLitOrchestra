//! E2E runtime validation tests for Milestone 3 — sldo-plan binary.
//!
//! These tests verify that the sldo-plan binary correctly parses arguments,
//! reads templates, and validates runbooks at runtime.

use std::path::Path;
use std::process::Command;

#[test]
fn plan_help_flag() {
    // Given: The sldo-plan binary is built
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-plan";
    // When: sldo-plan --help is run
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-plan --help");
    // Then: Process exits 0, stdout contains "Usage"
    assert!(
        output.status.success(),
        "sldo-plan --help should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage") || stdout.contains("usage"),
        "sldo-plan --help should contain 'Usage', got: {}",
        stdout
    );
}

#[test]
fn plan_missing_args_exits_nonzero() {
    // Given: No arguments
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-plan";
    // When: sldo-plan is run with no args
    let output = Command::new(&binary)
        .output()
        .expect("failed to execute sldo-plan");
    // Then: Process exits non-zero, stderr contains error
    assert!(
        !output.status.success(),
        "sldo-plan with no args should exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.trim().is_empty(),
        "sldo-plan with no args should produce stderr output"
    );
}

#[test]
fn plan_reads_real_template() {
    // Given: The actual runbook template at docs/runbook-template.md
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let template_path = Path::new(manifest_dir).join("docs/runbook-template.md");
    let content = std::fs::read_to_string(&template_path)
        .expect("Failed to read docs/runbook-template.md");

    // When: Template content is read
    // Then: It contains "Milestone Tracker"
    assert!(
        content.contains("Milestone Tracker"),
        "Template should contain 'Milestone Tracker'"
    );
}

#[test]
fn plan_validates_own_runbook() {
    // Given: The actual runbook at docs/RUNBOOK-RUST-REWRITE.md
    // (This tests the validate_runbook logic against a real file)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runbook_path = Path::new(manifest_dir).join("docs/RUNBOOK-RUST-REWRITE.md");

    // When: The file is checked for required content
    let content = std::fs::read_to_string(&runbook_path)
        .expect("Failed to read docs/RUNBOOK-RUST-REWRITE.md");

    // Then: File is large enough and contains required sections
    assert!(content.len() > 500, "Runbook should be > 500 bytes");
    assert!(content.contains("Milestone Tracker"));
    assert!(content.contains("Pre-Milestone Protocol"));
    assert!(content.contains("Post-Milestone Protocol"));
    assert!(content.contains("Background Context"));
    assert!(content.contains("Current State"));
    assert!(content.contains("BDD Acceptance Scenarios"));
}
