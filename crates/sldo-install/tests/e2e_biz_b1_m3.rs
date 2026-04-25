//! M3 structural-contract tests for biz-skill-pack Runbook B1.
//! Verifies /slo-product SKILL.md structure + three mode_arg paths + disambiguation from /slo-metrics.

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

const THREE_MODES: &[&str] = &["roadmap", "metrics", "okrs"];

#[test]
fn slo_product_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-product"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_product_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    assert!(skill.contains("Generator with a mode arg") || skill.contains("Generator pattern"));
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 1, "/slo-product cites `{pid}` {count} times; generator should reference advisor predicates at most once");
    }
}

#[test]
fn slo_product_documents_three_mode_args() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    for m in THREE_MODES {
        // Mode keyword must appear in a mode-defining context (heading or table).
        assert!(skill.contains(m), "/slo-product must enumerate mode `{m}`");
    }
    // The skill must enumerate the three output paths under docs/biz-public/product/.
    let output_paths = [
        "docs/biz-public/product/roadmap.md",
        "docs/biz-public/product/metrics.md",
        "docs/biz-public/product/okrs.md",
    ];
    for p in &output_paths {
        assert!(skill.contains(p), "/slo-product must declare output path `{p}`");
    }
}

#[test]
fn slo_product_documents_unknown_mode_rejection() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    let signals = [
        "Unknown mode_arg",
        "unknown mode_arg",
        "Refuse unknown mode_arg",
    ];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-product must document rejection of unknown mode_arg values");
}

#[test]
fn slo_product_disambiguates_from_slo_metrics() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    // Must mention slo-metrics by name.
    assert!(
        skill.contains("/slo-metrics") || skill.contains("slo-metrics"),
        "/slo-product must reference /slo-metrics for disambiguation"
    );
    // Must enumerate the financial KPIs that belong to /slo-metrics, not here.
    let financial_kpis = ["CAC", "LTV", "NDR", "burn multiple"];
    let count = financial_kpis.iter().filter(|k| skill.contains(**k)).count();
    assert!(
        count >= 3,
        "/slo-product must enumerate at least 3 financial KPIs that belong to /slo-metrics (not here); found {count}"
    );
}

#[test]
fn slo_product_documents_north_star_singular() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    assert!(skill.contains("north-star") || skill.contains("North Star") || skill.contains("North-star"));
    let signals = ["ONE metric", "single", "Not multiple"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-product must enforce ONE north-star metric (not multiple)");
}

#[test]
fn slo_product_okrs_caps_objectives_at_three() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    let signals = ["3 objectives MAX", "3 objectives max", "more than 3 objectives", "Cut to 3"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-product okrs mode must cap objectives at 3");
}

#[test]
fn slo_product_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-product/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}
