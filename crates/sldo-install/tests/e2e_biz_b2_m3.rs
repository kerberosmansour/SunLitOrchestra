//! M3 structural-contract tests for biz-skill-pack Runbook B2 — /slo-pricing.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
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

#[test]
fn slo_pricing_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-pricing"));
}

#[test]
fn slo_pricing_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    assert!(skill.contains("Generator pattern"));
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 1, "/slo-pricing cites `{pid}` {count} times; generator should reference advisor predicates ≤ 1");
    }
}

#[test]
fn slo_pricing_documents_value_equation() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    let signals = ["25-33%", "value-equation", "price = 25"];
    let count = signals.iter().filter(|s| skill.contains(**s)).count();
    assert!(count >= 2, "/slo-pricing must document the 25-33% value-equation framing (found {count})");
}

#[test]
fn slo_pricing_caps_tiers_at_three() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    let signals = ["3 tiers max", "AT MOST 3 tiers", "4+ tiers"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-pricing must cap tiers at 3");
}

#[test]
fn slo_pricing_documents_50_percent_experiment() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    let signals = ["Increase price by 50%", "increase price by 50%", "× 1.5", "1.5×"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-pricing must document the canonical 'increase by 50%' experiment");
}

#[test]
fn slo_pricing_routes_seis_eis_to_fundraise() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    assert!(skill.contains("SEIS") || skill.contains("EIS"));
    assert!(skill.contains("/slo-fundraise") || skill.contains("slo-fundraise"));
}

#[test]
fn slo_pricing_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}

#[test]
fn slo_pricing_outputs_to_public_tier() {
    let skill = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    assert!(skill.contains("docs/biz-public/pricing.md"));
    assert!(skill.contains("tier: public"));
}
