//! M3 structural-contract tests for the biz-skill-pack Runbook A.
//!
//! Verifies:
//! - `/slo-equity` SKILL.md frontmatter + four-mode contract + four-predicate-id citations.
//! - `references/biz/hmrc-vcm-index.md` carries VCM34080 / VCM3000 / VCM31000 with retrieval date.
//! - Cross-skill citation: every advisor SKILL.md (slo-legal + slo-accounting + slo-equity) cites all four predicate IDs.
//! - Triage-gate predicate-id set still unchanged.
//! - SEIS / EIS Advance Assurance lead-time (≥ 6 weeks) is documented in hmrc-vcm-index.md.
//! - Abingdon Health case-law citation present in hmrc-vcm-index.md (preferential-rights trigger).
//! - references/biz/ still NOT discovered.

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
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

const FOUR_MODES: &[&str] = &["draft", "translate", "triage", "prepare"];

const ADVISOR_SKILLS: &[&str] = &["slo-legal", "slo-accounting", "slo-equity"];

#[test]
fn slo_equity_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-equity/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-equity"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_equity_skill_md_documents_four_modes() {
    let skill = read(&repo_root().join("skills/slo-equity/SKILL.md"));
    for mode in FOUR_MODES {
        assert!(skill.contains(mode), "missing mode `{mode}`");
    }
    assert!(skill.contains("## Modes"), "missing `## Modes` heading");
}

#[test]
fn slo_equity_skill_md_cites_all_four_predicate_ids() {
    let skill = read(&repo_root().join("skills/slo-equity/SKILL.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(skill.contains(pid), "missing predicate `{pid}`");
    }
    assert!(skill.contains("references/biz/triage-gate.md"));
}

#[test]
fn cross_skill_advisor_pattern_replicated_three_skills() {
    for skill_name in ADVISOR_SKILLS {
        let path = repo_root().join("skills").join(skill_name).join("SKILL.md");
        let body = read(&path);
        for pid in FOUR_PREDICATE_IDS {
            assert!(
                body.contains(pid),
                "advisor skill `{skill_name}` must cite predicate `{pid}` (cross-skill citation contract — three advisors now)"
            );
        }
    }
}

#[test]
fn triage_gate_predicate_set_still_unchanged() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(gate.contains(pid));
    }
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(!gate.contains(&candidate), "must not contain `{candidate}`");
    }
}

#[test]
fn hmrc_vcm_index_carries_three_manual_sections() {
    let vcm = read(&repo_root().join("references/biz/hmrc-vcm-index.md"));
    let sections = ["VCM34080", "VCM3000", "VCM31000"];
    for section in &sections {
        assert!(vcm.contains(section), "missing HMRC manual section `{section}`");
    }
    assert!(vcm.contains("gov.uk"), "must cite gov.uk URLs");
    assert!(vcm.contains("retrieved"), "must include retrieval-date frontmatter or body line");
}

#[test]
fn hmrc_vcm_index_documents_advance_assurance_lead_time() {
    let vcm = read(&repo_root().join("references/biz/hmrc-vcm-index.md"));
    let lead_time_signals = ["6 weeks", "six weeks", "≥ 6 weeks", ">= 6 weeks"];
    let any = lead_time_signals.iter().any(|s| vcm.contains(s));
    assert!(
        any,
        "hmrc-vcm-index.md must document the ≥ 6 weeks AA lead time floor (looked for any of {lead_time_signals:?})"
    );
}

#[test]
fn hmrc_vcm_index_cites_abingdon_health_case() {
    let vcm = read(&repo_root().join("references/biz/hmrc-vcm-index.md"));
    assert!(
        vcm.contains("Abingdon Health"),
        "hmrc-vcm-index.md must cite the Abingdon Health case for the preferential-rights disqualification trigger"
    );
}

#[test]
fn slo_equity_runs_seis_eis_pre_check() {
    let skill = read(&repo_root().join("skills/slo-equity/SKILL.md"));
    // The skill must reference the four pre-check questions or the HMRC manual paragraphs.
    let signals = ["VCM34080", "VCM3000", "Advance Assurance", "Abingdon Health"];
    for s in &signals {
        assert!(
            skill.contains(s),
            "slo-equity SKILL.md must reference `{s}` (SEIS/EIS pre-check)"
        );
    }
}

#[test]
fn references_biz_dir_still_not_discovered_after_m3() {
    let skills_dir = repo_root().join("skills");
    let references_biz = repo_root().join("references/biz");
    assert!(skills_dir.is_dir());
    assert!(references_biz.is_dir());
    assert!(!skills_dir.join("biz").exists());
    assert!(!skills_dir.join("_biz-shared").exists());
    for skill_name in ADVISOR_SKILLS {
        assert!(
            skills_dir.join(skill_name).join("SKILL.md").exists(),
            "skills/{skill_name}/SKILL.md must exist"
        );
    }
}
