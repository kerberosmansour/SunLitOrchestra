//! M7 E2E: /slo-execute and /slo-verify skill contracts.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn skill(name: &str) -> String {
    fs::read_to_string(repo_root().join("skills").join(name).join("SKILL.md"))
        .unwrap_or_else(|e| panic!("{} SKILL.md missing: {e}", name))
}

#[test]
fn execute_frontmatter_valid() {
    let body = skill("slo-execute");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-execute"));
}

#[test]
fn execute_enforces_allow_list_rule() {
    let body = skill("slo-execute");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("allow-list") || lower.contains("allowed to change"),
        "execute must reference the milestone's file allow-list"
    );
    assert!(
        lower.contains("stop") || lower.contains("pause") || lower.contains("refuse"),
        "execute must stop/pause when an out-of-scope edit is needed"
    );
    assert!(
        lower.contains("surface the conflict") || lower.contains("ask the user"),
        "execute must surface the conflict to the user, not widen silently"
    );
}

#[test]
fn execute_enforces_bdd_first() {
    let body = skill("slo-execute");
    let lower = body.to_lowercase();
    // BDD-first must be explicit.
    assert!(lower.contains("bdd") && lower.contains("first"));
    assert!(
        lower.contains("fail for the") || lower.contains("fail for expected"),
        "execute must require tests fail for the right reason before implementation"
    );
}

#[test]
fn execute_restates_constraints_before_coding() {
    let body = skill("slo-execute");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("restate") && lower.contains("constraint"),
        "execute must restate constraints before coding"
    );
}

#[test]
fn verify_frontmatter_valid() {
    let body = skill("slo-verify");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-verify"));
}

#[test]
fn verify_adds_regression_test_before_fix() {
    let body = skill("slo-verify");
    let lower = body.to_lowercase();
    assert!(lower.contains("regression test"));
    assert!(
        lower.contains("before") && (lower.contains("fix") || lower.contains("bug")),
        "verify must require regression test committed BEFORE the fix"
    );
}

#[test]
fn verify_exercises_empty_and_degraded_states() {
    let body = skill("slo-verify");
    let lower = body.to_lowercase();
    assert!(lower.contains("empty") && lower.contains("state"));
    assert!(
        lower.contains("partial failure") || lower.contains("degraded"),
        "verify must exercise degraded/partial-failure states explicitly"
    );
}

#[test]
fn verify_has_playwright_for_ui() {
    let body = skill("slo-verify");
    let lower = body.to_lowercase();
    assert!(lower.contains("playwright"));
    assert!(
        lower.contains("if ui") || lower.contains("ui surface") || lower.contains("ui path"),
        "verify must gate Playwright on UI-surface presence"
    );
}

#[test]
fn verify_does_not_fix_bugs_itself() {
    let body = skill("slo-verify");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("hand the bug back") || lower.contains("do not fix it yourself")
            || lower.contains("separation of concerns"),
        "verify must not repair bugs itself — separation of concerns with /slo-execute"
    );
}
