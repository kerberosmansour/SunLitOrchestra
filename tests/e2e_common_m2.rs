//! E2E runtime validation tests for Milestone 2 — Shared library.
//!
//! These tests verify that the shared library modules work correctly at runtime
//! against real-world inputs (actual runbook, actual git repo, actual project).

use std::path::Path;

#[test]
fn parse_v3_template_tracker() {
    // Given: The v3 runbook template (the canonical contract /slo-plan emits)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let template_path = Path::new(manifest_dir).join("docs/templates/runbook-template_v_3_template.md");
    let content = std::fs::read_to_string(&template_path)
        .expect("Failed to read docs/templates/runbook-template_v_3_template.md");

    // When: parse_tracker is called on the template content
    let rows = sldo_common::runbook::parse_tracker(&content);

    // Then: Returns rows (the template has placeholder milestones)
    assert!(
        !rows.is_empty(),
        "parse_tracker on the v3 template should return milestone rows"
    );
}

#[test]
fn parse_actual_runbook_tracker() {
    // Given: A real shipped runbook (biz-pack Runbook A — 4 advisor skills, 4 milestones)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runbook_path = Path::new(manifest_dir).join("docs/RUNBOOK-BIZ-SKILL-PACK-A.md");
    let content = std::fs::read_to_string(&runbook_path)
        .expect("Failed to read docs/RUNBOOK-BIZ-SKILL-PACK-A.md");

    // When: parse_tracker is called on the actual runbook
    let rows = sldo_common::runbook::parse_tracker(&content);

    // Then: Returns at least one milestone row
    assert!(
        !rows.is_empty(),
        "parse_tracker on a real runbook should return milestone rows"
    );
}

#[test]
fn git_checks_on_own_repo() {
    // Given: The SunLitOrchestrate repository
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest_dir);

    // When: is_git_repo is called
    // Then: Returns true
    assert!(
        sldo_common::git::is_git_repo(repo),
        "SunLitOrchestrate root should be a git repo"
    );

    // When: current_branch is called
    let branch = sldo_common::git::current_branch(repo).expect("Should get current branch");
    // Then: Returns a non-empty string
    assert!(
        !branch.is_empty(),
        "current_branch should return a non-empty string"
    );
}

#[test]
fn detect_commands_on_own_repo() {
    // Given: The SunLitOrchestrate project (has Cargo.toml)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_dir = Path::new(manifest_dir);

    // When: detect_build_commands is called
    let build_cmds = sldo_common::detect::detect_build_commands(project_dir);

    // Then: Returns commands including "cargo build --workspace"
    assert!(
        build_cmds.contains(&"cargo build --workspace".to_string()),
        "Expected 'cargo build --workspace' in {:?}",
        build_cmds
    );
}
