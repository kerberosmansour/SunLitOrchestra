//! M6 E2E: /slo-critique orchestrator + four persona files.

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

fn critique_dir() -> PathBuf {
    repo_root().join("skills").join("slo-critique")
}

fn read(path: &str) -> String {
    fs::read_to_string(critique_dir().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn critique_skill_frontmatter_valid() {
    let body = read("SKILL.md");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-critique"));
}

#[test]
fn critique_lists_four_personas_in_rotation_order() {
    // Look at the bold-marker list entries specifically, not any mention in prose.
    let body = read("SKILL.md");
    let ceo_idx = body.find("**CEO**").expect("rotation list missing **CEO**");
    let eng_idx = body.find("**Eng lead**").expect("rotation list missing **Eng lead**");
    let sec_idx = body.find("**Security**").expect("rotation list missing **Security**");
    let des_idx = body.find("**Design**").expect("rotation list missing **Design**");
    assert!(ceo_idx < eng_idx, "CEO must come before Eng in the rotation list");
    assert!(eng_idx < sec_idx, "Eng must come before Security in the rotation list");
    assert!(sec_idx < des_idx, "Security must come before Design in the rotation list");
}

#[test]
fn critique_requires_concrete_exploit_scenario() {
    let body = read("SKILL.md");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("concrete") && (lower.contains("scenario") || lower.contains("exploit")),
        "critique must require concrete scenarios, not theoretical findings"
    );
}

#[test]
fn critique_skips_design_when_no_ui() {
    let body = read("SKILL.md");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("no ui") || lower.contains("n/a"),
        "critique must handle the no-UI case explicitly"
    );
}

#[test]
fn ceo_persona_has_four_modes() {
    let body = read("personas/ceo.md");
    let lower = body.to_lowercase();
    for mode in &["expansion", "hold", "reduction"] {
        assert!(lower.contains(mode), "CEO persona missing mode: {mode}");
    }
}

#[test]
fn eng_persona_covers_failure_modes_and_assumptions() {
    let body = read("personas/eng.md");
    let lower = body.to_lowercase();
    assert!(lower.contains("assumption"));
    assert!(lower.contains("failure mode"));
    assert!(lower.contains("orthogonal") || lower.contains("scope creep") || lower.contains("test gap"));
}

#[test]
fn security_persona_has_owasp_and_stride() {
    let body = read("personas/security.md");
    let lower = body.to_lowercase();
    assert!(lower.contains("owasp"));
    assert!(lower.contains("stride"));
    // Every finding needs an exploit scenario.
    assert!(lower.contains("exploit") && (lower.contains("attacker") || lower.contains("step-by-step")));
}

#[test]
fn design_persona_documents_no_ui_skip() {
    let body = read("personas/design.md");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("n/a") || lower.contains("no ui") || lower.contains("skip"),
        "design persona must explicitly handle no-UI runbook"
    );
}
