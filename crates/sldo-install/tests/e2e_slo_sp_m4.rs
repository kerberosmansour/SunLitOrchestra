//! M4 E2E: /slo-architect and /slo-plan skill contracts.

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
fn architect_frontmatter_valid() {
    let body = skill("slo-architect");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-architect"));
}

#[test]
fn architect_sets_tla_required_flag() {
    let body = skill("slo-architect");
    assert!(
        body.contains("tla_required"),
        "architect skill must decide tla_required true/false"
    );
    // Must state both paths explicitly.
    assert!(body.contains("true") && body.contains("false"));
}

#[test]
fn architect_declares_interfaces_lockdown() {
    let body = skill("slo-architect");
    assert!(
        body.contains("interfaces.md") || body.contains("Public APIs"),
        "architect must lock down interfaces for downstream milestones"
    );
}

#[test]
fn plan_frontmatter_valid() {
    let body = skill("slo-plan");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-plan"));
}

#[test]
fn plan_refuses_one_shot_generation() {
    let body = skill("slo-plan");
    // The core discipline of this skill.
    assert!(
        body.to_lowercase().contains("one-shot") || body.to_lowercase().contains("one shot")
            || body.to_lowercase().contains("in one go"),
        "plan skill must explicitly refuse one-shot runbook generation"
    );
    assert!(
        body.to_lowercase().contains("refuse"),
        "plan skill must use 'refuse' language for the one-shot anti-pattern"
    );
}

#[test]
fn plan_caps_milestones_at_five() {
    let body = skill("slo-plan");
    assert!(
        body.contains("5 milestones") || body.contains("five milestones") || body.contains("2\u{2013}5"),
        "plan skill must cap milestones per runbook"
    );
}

#[test]
fn plan_references_v3_template() {
    let body = skill("slo-plan");
    assert!(
        body.contains("v3") || body.contains("runbook-template"),
        "plan skill must reference the v3 template as the output contract"
    );
}
