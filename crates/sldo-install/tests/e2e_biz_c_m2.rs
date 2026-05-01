//! M2 structural-contract tests for Runbook C — /slo-hire (with mandatory IR35 triage gate).

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
fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];
const FOUR_ROLE_SHAPES: &[&str] = &["swe", "ae", "designer", "ops"];

#[test]
fn slo_hire_skill_md_has_required_frontmatter() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(s.starts_with("---\n"));
}

#[test]
fn slo_hire_is_generator_archetype() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(s.contains("Generator with"));
    for pid in FOUR_PREDICATE_IDS {
        let c = s.matches(pid).count();
        assert!(c <= 1, "/slo-hire cites `{pid}` {c} times");
    }
}

#[test]
fn slo_hire_documents_four_role_shapes() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    for r in FOUR_ROLE_SHAPES {
        assert!(s.contains(r), "/slo-hire must enumerate role shape `{r}`");
    }
}

#[test]
fn slo_hire_outputs_to_confidential_tier() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(s.contains("docs/biz/hires/"));
    assert!(s.contains("tier: confidential"));
}

#[test]
fn slo_hire_mandatory_ir35_triage_gate() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(
        s.contains("ir35-cest-factors.md"),
        "/slo-hire must cite the IR35 reference"
    );
    let signals = [
        "MANDATORY",
        "Mandatory",
        "seven IR35",
        "seven hard-block-to-lawyer IR35",
    ];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(any, "/slo-hire must enforce the mandatory IR35 triage gate");
}

#[test]
fn slo_hire_documents_seven_ir35_triggers() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    let signals = [
        "Substitution clause",
        "Full-time",
        "Exclusive engagement",
        "CEST",
    ];
    let count = signals.iter().filter(|x| s.contains(**x)).count();
    assert!(
        count >= 3,
        "/slo-hire must enumerate IR35 triggers (found {count})"
    );
}

#[test]
fn slo_hire_rejects_tax_efficiency_framing() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    let signals = ["tax efficiency", "REJECT", "REFUSE", "by-preference"];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(
        any,
        "/slo-hire must reject 'call them a contractor for tax efficiency' framing"
    );
}

#[test]
fn slo_hire_unknown_mode_rejection() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(s.contains("Unknown mode_arg"));
}

#[test]
fn slo_hire_has_uk_only_error() {
    let s = read(&repo_root().join("skills/slo-hire/SKILL.md"));
    assert!(s.contains("v1 supports UK only"));
}
