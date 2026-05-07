//! M3 structural-contract tests for the Fowler AI architecture SLO improvements.
//!
//! M3 adds an AI nondeterminism tolerance contract across architect, plan, and
//! verify. These tests assert the required fields, gated N/A path, and verify
//! pass documentation without invoking an AI runtime.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn ai_reference() -> String {
    read(repo_root().join("skills/slo-plan/references/ai-tolerance-contract.md"))
}

fn skill_local_v4() -> String {
    read(repo_root().join("skills/slo-plan/references/runbook-template_v_4_template.md"))
}

fn docs_v4() -> String {
    read(repo_root().join("docs/slo/templates/runbook-template_v_4_template.md"))
}

#[test]
fn ai_tolerance_reference_exists_with_required_fields() {
    let body = ai_reference();

    for field in [
        "Accepted variance",
        "Deterministic boundary",
        "Eval evidence",
        "Retry / fallback",
        "Must-never outcomes",
        "Sample budget",
    ] {
        assert!(
            body.contains(field),
            "AI tolerance reference missing `{field}`"
        );
    }
}

#[test]
fn ai_tolerance_reference_requires_bounded_samples() {
    let body = ai_reference();

    assert!(
        body.contains("bounded") && body.contains("sample"),
        "AI tolerance contract must require bounded sample/eval counts"
    );
}

#[test]
fn v4_templates_emit_ai_tolerance_row_with_non_ai_na_path() {
    for (label, body) in [("skill-local", skill_local_v4()), ("docs", docs_v4())] {
        assert!(
            body.contains("AI tolerance contract"),
            "{label} v4 template missing AI tolerance row"
        );
        assert!(
            body.contains("N/A — no AI component"),
            "{label} v4 template missing non-AI N/A path"
        );
    }
}

#[test]
fn plan_methodology_and_skill_cite_ai_tolerance_reference() {
    let skill = read(repo_root().join("skills/slo-plan/SKILL.md"));
    let methodology =
        read(repo_root().join("skills/slo-plan/references/methodology-milestone-authoring.md"));

    for (label, body) in [("skill", skill), ("methodology", methodology)] {
        assert!(
            body.contains("references/ai-tolerance-contract.md")
                || body.contains("ai-tolerance-contract.md"),
            "{label} must cite the AI tolerance reference"
        );
        assert!(
            body.contains("N/A — no AI component"),
            "{label} must document the non-AI N/A path"
        );
    }
}

#[test]
fn verify_has_ai_tolerance_pass_after_normal_passes() {
    let verify = read(repo_root().join("skills/slo-verify/SKILL.md"));

    let pass4 = verify
        .find("### Pass 4. Security")
        .expect("verify skill missing Pass 4 security heading");
    let ai_pass = verify
        .find("AI tolerance")
        .expect("verify skill missing AI tolerance pass");

    assert!(
        ai_pass > pass4,
        "AI tolerance pass must be documented after normal runtime/security passes"
    );
    for needle in [
        "accepted variance",
        "deterministic boundary",
        "must-never outcomes",
    ] {
        assert!(
            verify.to_lowercase().contains(needle),
            "verify AI pass missing `{needle}`"
        );
    }
}

#[test]
fn architect_links_ai_component_to_tolerance_contract() {
    let architect = read(repo_root().join("skills/slo-architect/SKILL.md"));

    assert!(
        architect.contains("ai_component: true"),
        "architect must preserve the ai_component gate"
    );
    assert!(
        architect.contains("AI tolerance"),
        "architect must link AI component architecture to downstream tolerance contracts"
    );
}
