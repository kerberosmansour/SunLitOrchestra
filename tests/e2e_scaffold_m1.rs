//! E2E runtime validation tests for Milestone 1 — Cargo workspace scaffolding.
//!
//! These tests verify that the workspace builds, binaries run, and the shared
//! library crate exports the expected API.

use std::process::Command;

#[test]
fn workspace_builds() {
    // Given: A fresh clone of the repo
    // When: `cargo build --workspace` is run
    let output = Command::new("cargo")
        .args(["build", "--workspace"])
        .output()
        .expect("failed to execute cargo build");
    // Then: Exit code 0
    assert!(
        output.status.success(),
        "cargo build --workspace failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn plan_binary_runs() {
    // Given: Binaries are built
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-plan";
    // When: sldo-plan --help is executed
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-plan");
    // Then: Process exits 0 with non-empty stdout
    assert!(
        output.status.success(),
        "sldo-plan --help exited with non-zero: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "sldo-plan --help should print usage"
    );
}

#[test]
fn run_binary_runs() {
    // Given: Binaries are built
    let binary = env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-run";
    // When: sldo-run --help is executed
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("failed to execute sldo-run");
    // Then: Process exits 0 with non-empty stdout
    assert!(
        output.status.success(),
        "sldo-run --help exited with non-zero: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "sldo-run --help should print usage"
    );
}

#[test]
fn common_version_exists() {
    // Given: The sldo-common library crate is available
    // When: version() is called
    let v = sldo_common::version();
    // Then: it returns a non-empty string
    assert!(!v.is_empty(), "sldo_common::version() must return a non-empty string");
}
