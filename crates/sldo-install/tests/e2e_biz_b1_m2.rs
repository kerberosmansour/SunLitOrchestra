//! M2 structural-contract tests for biz-skill-pack Runbook B1.
//! Verifies /slo-gtm SKILL.md structure + generator-archetype non-citation of advisor predicates.

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

#[test]
fn slo_gtm_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-gtm"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_gtm_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    assert!(
        skill.contains("Generator pattern") || skill.contains("archetype: generator"),
        "/slo-gtm must declare itself as a generator"
    );
    // Generator: max one mention per advisor predicate id.
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 1, "/slo-gtm cites `{pid}` {count} times; generators reference advisor predicates at most once");
    }
}

#[test]
fn slo_gtm_documents_motion_choice() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    let motions = ["PLG", "sales-led", "community-led", "hybrid"];
    for m in &motions {
        assert!(skill.contains(m), "/slo-gtm must enumerate motion `{m}`");
    }
}

#[test]
fn slo_gtm_outputs_to_public_tier() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    assert!(skill.contains("docs/biz-public/gtm/"));
    assert!(skill.contains("tier: public"));
}

#[test]
fn slo_gtm_routes_direct_marketing_to_legal_triage() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    assert!(
        skill.contains("/slo-legal triage") || skill.contains("slo-legal triage"),
        "/slo-gtm must route direct-marketing channels to /slo-legal triage for PECR considerations"
    );
    assert!(skill.contains("PECR"), "must reference PECR considerations");
}

#[test]
fn slo_gtm_documents_segment_cap() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    let signals = ["3 segments", "three segments", "more than 3 segments"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-gtm must enforce a segment cap (3 max)");
}

#[test]
fn slo_gtm_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-gtm/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}
