//! M4 structural-contract tests for biz-skill-pack Runbook B2 — /slo-metrics + cross-skill financial-KPI test + CLAUDE.md catalog.

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

const TWO_MODES: &[&str] = &["consumer", "b2b"];

const ALL_B2_GENERATORS: &[&str] = &[
    "slo-launch",
    "slo-sales-funnel",
    "slo-pricing",
    "slo-metrics",
];

const ALL_PACK_SKILLS_THROUGH_B2: &[&str] = &[
    // Runbook A advisors
    "slo-legal",
    "slo-accounting",
    "slo-equity",
    "slo-fundraise",
    // Runbook B1 generators
    "slo-talk-to-users",
    "slo-gtm",
    "slo-product",
    "slo-marketing",
    // Runbook B2 generators
    "slo-launch",
    "slo-sales-funnel",
    "slo-pricing",
    "slo-metrics",
];

#[test]
fn slo_metrics_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-metrics"));
}

#[test]
fn slo_metrics_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(
        skill.contains("Generator with")
            || skill.contains("Generator pattern")
            || skill.contains("archetype: generator")
    );
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 1, "/slo-metrics cites `{pid}` {count} times; generator should reference advisor predicates ≤ 1");
    }
}

#[test]
fn slo_metrics_documents_two_mode_args() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    for m in TWO_MODES {
        assert!(skill.contains(m), "/slo-metrics must enumerate mode `{m}`");
    }
}

#[test]
fn slo_metrics_disambiguates_from_slo_product() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(skill.contains("/slo-product metrics") || skill.contains("slo-product metrics"));
    // Must explicitly enumerate PM-side metrics that belong to /slo-product.
    let pm_signals = [
        "DAU",
        "activation rate",
        "Retention curves",
        "feature-adoption",
    ];
    let count = pm_signals.iter().filter(|s| skill.contains(**s)).count();
    assert!(
        count >= 3,
        "/slo-metrics must enumerate PM-side metrics that belong to /slo-product (found {count})"
    );
}

#[test]
fn slo_metrics_documents_all_canonical_financial_kpis() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    let kpis = [
        "CAC",
        "LTV",
        "NDR",
        "burn multiple",
        "MoM revenue growth",
        "Gross margin",
        "Runway",
        "ARR",
    ];
    for kpi in &kpis {
        assert!(
            skill.contains(kpi),
            "/slo-metrics must document financial KPI `{kpi}`"
        );
    }
}

#[test]
fn slo_metrics_b2b_target_ndr_110() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(
        skill.contains("110%"),
        "/slo-metrics b2b mode must cite NDR ≥ 110% target"
    );
}

#[test]
fn slo_metrics_consumer_target_15_mom() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(
        skill.contains("15%"),
        "/slo-metrics consumer mode must cite ≥ 15% MoM target"
    );
}

#[test]
fn slo_metrics_burn_multiple_threshold_2() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    let signals = ["≤ 2", "<= 2"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(
        any,
        "/slo-metrics must cite burn multiple ≤ 2 healthy threshold (Bessemer convention)"
    );
}

#[test]
fn slo_metrics_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-metrics/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}

#[test]
fn cross_skill_financial_kpi_citation() {
    // /slo-pricing and /slo-metrics carry financial-KPI surfaces. Each must
    // cite at least one canonical financial KPI. /slo-fundraise carries
    // fundraise-process surface (SEIS/EIS, SAFE math), NOT KPI-dashboard
    // surface — it's intentionally excluded.
    let canonical_kpis = ["CAC", "LTV", "NDR", "burn multiple", "ARR"];
    let financial_skills = ["skills/slo-pricing/SKILL.md", "skills/slo-metrics/SKILL.md"];
    for skill_path in &financial_skills {
        let body = read(&repo_root().join(skill_path));
        let count = canonical_kpis.iter().filter(|k| body.contains(**k)).count();
        assert!(
            count >= 1,
            "skill `{skill_path}` carries financial-KPI surface; must cite at least one canonical financial KPI from {canonical_kpis:?} (found {count})"
        );
    }
}

#[test]
fn claude_md_catalogs_all_b2_generators() {
    let claude_md = read(&repo_root().join("CLAUDE.md"));
    for skill_name in ALL_B2_GENERATORS {
        let verb = format!("/{skill_name}");
        assert!(
            claude_md.contains(&verb),
            "CLAUDE.md must catalog B2 generator `{verb}`"
        );
    }
}

#[test]
fn references_biz_dir_still_not_discovered_after_b2_m4() {
    let skills_dir = repo_root().join("skills");
    assert!(skills_dir.is_dir());
    assert!(!skills_dir.join("biz").exists());
    for skill_name in ALL_PACK_SKILLS_THROUGH_B2 {
        assert!(
            skills_dir.join(skill_name).join("SKILL.md").exists(),
            "skills/{skill_name}/SKILL.md must exist"
        );
    }
}
