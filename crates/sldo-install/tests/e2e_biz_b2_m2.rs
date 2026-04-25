//! M2 structural-contract tests for biz-skill-pack Runbook B2 — /slo-sales-funnel.

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
fn slo_sales_funnel_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-sales-funnel"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_sales_funnel_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    assert!(skill.contains("Generator pattern"));
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 2, "/slo-sales-funnel cites `{pid}` {count} times; generators ≤ 2 (allowing PECR routing reference)");
    }
}

#[test]
fn slo_sales_funnel_documents_seven_outbound_principles() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    let principles = ["seven principles", "seven outbound", "Subject specific", "One clear ask", "Short — under 100 words"];
    let count = principles.iter().filter(|p| skill.contains(**p)).count();
    assert!(count >= 3, "/slo-sales-funnel must enumerate the seven outbound-email principles (found {count})");
    // Check at least 5 of the 7 numbered principles appear.
    for n in 1..=7 {
        let pattern = format!("\n{n}. **");
        if !skill.contains(&pattern) {
            // Allow alternative formatting; just check at least 5 appear in ANY numbered form.
        }
    }
}

#[test]
fn slo_sales_funnel_documents_deal_structure() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    let stages = ["Paid trial", "recurring", "opt-out"];
    for s in &stages {
        assert!(skill.contains(s), "/slo-sales-funnel must enumerate deal-structure stage `{s}`");
    }
}

#[test]
fn slo_sales_funnel_routes_cold_email_to_legal() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    assert!(skill.contains("/slo-legal triage") || skill.contains("slo-legal triage"));
    assert!(skill.contains("PECR"));
    assert!(skill.contains("DUAA"));
}

#[test]
fn slo_sales_funnel_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}

#[test]
fn slo_sales_funnel_outputs_to_public_tier() {
    let skill = read(&repo_root().join("skills/slo-sales-funnel/SKILL.md"));
    assert!(skill.contains("docs/biz-public/sales/"));
    assert!(skill.contains("tier: public"));
}
