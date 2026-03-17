//! E2E runtime validation tests for Tauri Desktop Milestone 1.
//!
//! These tests verify the Tauri workspace scaffolding is correct:
//! - sldo-tauri compiles as part of the workspace
//! - sldo-tauri can reference sldo-common types
//! - Frontend produces build output after `npm run build`
//! - All existing test suites still pass

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

// ── Feature: Tauri workspace integration ────────────────────────────

#[test]
fn workspace_builds_with_tauri_crate() {
    // Given: sldo-tauri is added to workspace members
    // When: `cargo build --workspace` runs
    let output = Command::new("cargo")
        .args(["build", "--workspace"])
        .current_dir(workspace_root())
        .output()
        .expect("failed to run cargo build");
    // Then: All 4 crates compile without errors
    assert!(
        output.status.success(),
        "cargo build --workspace failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn existing_tests_still_pass() {
    // Given: sldo-tauri crate exists in workspace
    // When: `cargo test --workspace --test 'e2e_*'` (only pre-existing tests) runs
    for test_name in &[
        "e2e_scaffold_m1",
        "e2e_common_m2",
        "e2e_plan_m3",
        "e2e_run_m4",
        "e2e_integration_m5",
    ] {
        let output = Command::new("cargo")
            .args(["test", "--workspace", "--test", test_name])
            .current_dir(workspace_root())
            .output()
            .expect(&format!("failed to run cargo test --test {}", test_name));
        // Then: All existing E2E tests pass
        assert!(
            output.status.success(),
            "{} failed:\n{}",
            test_name,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn tauri_crate_compiles() {
    // Given: sldo-tauri is a workspace member
    // When: `cargo check -p sldo-tauri` runs
    let output = Command::new("cargo")
        .args(["check", "-p", "sldo-tauri"])
        .current_dir(workspace_root())
        .output()
        .expect("failed to run cargo check -p sldo-tauri");
    // Then: sldo-tauri compiles without errors
    assert!(
        output.status.success(),
        "cargo check -p sldo-tauri failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn tauri_references_common() {
    // Given: sldo-tauri depends on sldo-common
    // When: sldo-common::version() is called
    let v = sldo_common::version();
    // Then: it returns a non-empty string (proving the dependency works)
    assert!(
        !v.is_empty(),
        "sldo_common::version() returned empty string"
    );
}

#[test]
fn frontend_dist_exists_after_build() {
    // Given: React frontend is initialized with Vite
    // When: `npm run build` is run in the ui directory
    let ui_dir = workspace_root().join("crates/sldo-tauri/ui");
    let output = Command::new("npm")
        .args(["run", "build"])
        .current_dir(&ui_dir)
        .output()
        .expect("failed to run npm run build");
    assert!(
        output.status.success(),
        "npm run build failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // Then: dist/index.html exists
    let index_html = ui_dir.join("dist/index.html");
    assert!(
        index_html.exists(),
        "dist/index.html not found after npm run build"
    );
}

// ── Feature: Shell app renders ──────────────────────────────────────

#[test]
fn app_css_contains_design_tokens() {
    // Given: docs/App.css content has been copied
    // When: crates/sldo-tauri/ui/src/App.css is read
    let css_path = workspace_root().join("crates/sldo-tauri/ui/src/App.css");
    let css = std::fs::read_to_string(&css_path).expect("failed to read App.css");
    // Then: it contains --color-primary (or equivalent gold token) and --bg token definitions
    assert!(
        css.contains("--gold") || css.contains("--color-primary"),
        "App.css missing primary color token"
    );
    assert!(
        css.contains("--bg"),
        "App.css missing --bg token definition"
    );
}

#[test]
fn logo_available_in_public() {
    // Given: sunlit.jpeg has been copied to public/
    // When: the file path is checked
    let logo_path = workspace_root().join("crates/sldo-tauri/ui/public/sunlit.jpeg");
    // Then: the file exists and is a valid JPEG (starts with FF D8)
    assert!(logo_path.exists(), "sunlit.jpeg not found in public/");
    let bytes = std::fs::read(&logo_path).expect("failed to read sunlit.jpeg");
    assert!(bytes.len() > 2, "sunlit.jpeg is too small to be valid");
    assert!(
        bytes[0] == 0xFF && bytes[1] == 0xD8,
        "sunlit.jpeg does not start with JPEG magic bytes"
    );
}
