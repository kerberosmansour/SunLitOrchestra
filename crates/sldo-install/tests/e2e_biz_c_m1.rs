//! M1 structural-contract tests for Runbook C — /slo-cofounder.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}
fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated", "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper", "gate-4-gdpr-document",
];

#[test]
fn slo_cofounder_skill_md_has_required_frontmatter() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.starts_with("---\n"));
    let after = &s[4..]; let close = after.find("\n---\n").unwrap();
    let fm = &after[..close];
    assert!(fm.contains("name: slo-cofounder"));
}

#[test]
fn slo_cofounder_is_generator_archetype() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.contains("Generator pattern"));
    for pid in FOUR_PREDICATE_IDS {
        let c = s.matches(pid).count();
        assert!(c <= 1, "/slo-cofounder cites `{pid}` {c} times");
    }
}

#[test]
fn slo_cofounder_outputs_to_confidential_tier() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.contains("docs/biz/cofounder/"));
    assert!(s.contains("tier: confidential"));
}

#[test]
fn slo_cofounder_documents_stress_over_skills() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.contains("stress > skills") || s.contains("Stress > skills") || s.contains("stress-handling"));
}

#[test]
fn slo_cofounder_documents_4_week_paid_trial() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    let signals = ["4-week paid", "4 weeks paid", "four-week paid"];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(any, "/slo-cofounder must document 4-week paid trial");
}

#[test]
fn slo_cofounder_routes_equity_to_slo_equity() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.contains("/slo-equity") || s.contains("slo-equity"));
}

#[test]
fn slo_cofounder_has_uk_only_error() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    assert!(s.contains("v1 supports UK only"));
}

#[test]
fn slo_cofounder_documents_monthly_1_1_agenda() {
    let s = read(&repo_root().join("skills/slo-cofounder/SKILL.md"));
    let signals = ["monthly 1:1", "1:1 agenda", "monthly one-on-one"];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(any, "/slo-cofounder must document monthly 1:1 agenda");
}
