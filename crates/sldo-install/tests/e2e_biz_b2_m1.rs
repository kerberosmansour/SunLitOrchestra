//! M1 structural-contract tests for biz-skill-pack Runbook B2 — /slo-launch.

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
fn slo_launch_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-launch"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_launch_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    assert!(skill.contains("Generator pattern") || skill.contains("archetype: generator"));
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 1, "/slo-launch cites `{pid}` {count} times; generators should reference advisor predicates at most once");
    }
}

#[test]
fn slo_launch_documents_four_stages() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    let stages = ["Silent", "Friends & family", "communities", "broader press"];
    for s in &stages {
        assert!(skill.contains(s), "/slo-launch must enumerate stage `{s}`");
    }
}

#[test]
fn slo_launch_documents_pitch_validator() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    let signals = ["one-sentence", "**For**", "we provide"];
    let count = signals.iter().filter(|s| skill.contains(**s)).count();
    assert!(count >= 2, "/slo-launch must document the one-sentence pitch validator (found {count} of {signals:?})");
}

#[test]
fn slo_launch_outputs_to_public_tier() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    assert!(skill.contains("docs/biz-public/launch"));
    assert!(skill.contains("tier: public"));
}

#[test]
fn slo_launch_routes_direct_marketing_to_legal() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    assert!(skill.contains("/slo-legal triage") || skill.contains("slo-legal triage"));
    assert!(skill.contains("PECR"));
}

#[test]
fn slo_launch_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-launch/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}
