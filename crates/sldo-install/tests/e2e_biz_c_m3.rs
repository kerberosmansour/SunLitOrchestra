//! M3 structural-contract tests for Runbook C — /slo-founder-check + final pack-level catalog test.

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

const ALL_FIFTEEN_BIZ_SKILLS: &[&str] = &[
    "slo-legal", "slo-accounting", "slo-equity", "slo-fundraise",
    "slo-talk-to-users", "slo-gtm", "slo-product", "slo-marketing",
    "slo-launch", "slo-sales-funnel", "slo-pricing", "slo-metrics",
    "slo-cofounder", "slo-hire", "slo-founder-check",
];

#[test]
fn slo_founder_check_has_required_frontmatter() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    assert!(s.starts_with("---\n"));
}

#[test]
fn slo_founder_check_is_generator_archetype() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    assert!(s.contains("Generator"));
    for pid in FOUR_PREDICATE_IDS {
        let c = s.matches(pid).count();
        assert!(c <= 1);
    }
}

#[test]
fn slo_founder_check_outputs_to_confidential_tier() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    assert!(s.contains("docs/biz/founder-check.md"));
    assert!(s.contains("tier: confidential"));
}

#[test]
fn slo_founder_check_documents_12_question_self_assessment() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    let signals = ["12-question", "12 question", "12-Question"];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(any, "/slo-founder-check must document 12-question self-assessment");
}

#[test]
fn slo_founder_check_documents_worst_case_runway_tiers() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    let tiers = ["Tier 1", "Tier 2", "Tier 3", "Tier 4"];
    for t in &tiers {
        assert!(s.contains(t), "/slo-founder-check must document worst-case-runway `{t}`");
    }
}

#[test]
fn slo_founder_check_has_yc_application_prep() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    let signals = ["YC application prep", "YC's standard application", "YC application"];
    let any = signals.iter().any(|x| s.contains(x));
    assert!(any, "/slo-founder-check must offer optional YC application prep");
}

#[test]
fn slo_founder_check_has_uk_only_error() {
    let s = read(&repo_root().join("skills/slo-founder-check/SKILL.md"));
    assert!(s.contains("v1 supports UK only"));
}

#[test]
fn pack_level_all_fifteen_biz_skills_exist() {
    let skills_dir = repo_root().join("skills");
    for name in ALL_FIFTEEN_BIZ_SKILLS {
        assert!(skills_dir.join(name).join("SKILL.md").exists(), "skills/{name}/SKILL.md must exist");
    }
}

#[test]
fn pack_level_claude_md_catalogs_all_fifteen() {
    let claude = read(&repo_root().join("CLAUDE.md"));
    for name in ALL_FIFTEEN_BIZ_SKILLS {
        let verb = format!("/{name}");
        assert!(claude.contains(&verb), "CLAUDE.md must catalog all 15 biz skills; missing `{verb}`");
    }
}

#[test]
fn pack_level_references_biz_dir_still_not_discovered() {
    let skills_dir = repo_root().join("skills");
    assert!(skills_dir.is_dir());
    assert!(!skills_dir.join("biz").exists());
    assert!(!skills_dir.join("_biz-shared").exists());
}

#[test]
fn pack_level_triage_gate_predicate_set_unchanged_through_runbook_c() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(gate.contains(pid));
    }
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(!gate.contains(&candidate));
    }
}

#[test]
fn pack_level_advisor_skills_still_cite_all_four_predicates() {
    let advisors = ["slo-legal", "slo-accounting", "slo-equity", "slo-fundraise"];
    for name in &advisors {
        let body = read(&repo_root().join("skills").join(name).join("SKILL.md"));
        for pid in FOUR_PREDICATE_IDS {
            assert!(body.contains(pid), "advisor `{name}` must still cite predicate `{pid}` (regression on Runbook A's contract)");
        }
    }
}
