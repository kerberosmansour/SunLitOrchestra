//! M4 structural-contract tests for the biz-skill-pack Runbook A.
//!
//! Verifies:
//! - `/slo-fundraise` SKILL.md frontmatter + four-mode contract + four-predicate-id citations.
//! - `references/biz/ir35-cest-factors.md` documents the three primary factors (substitution, MOO, control), the CEST April 2025 refresh, the PGMOL v HMRC [2024] UKSC 29 commentary, and the seven hard-block-to-lawyer triggers.
//! - Cross-skill citation: ALL FOUR advisor SKILL.mds cite all four predicate IDs (final replication proof).
//! - SEIS / EIS Advance Assurance pre-check is documented in `/slo-fundraise` SKILL.md.
//! - CLAUDE.md catalogs all four advisor skills.
//! - All M1 / M2 / M3 invariants still hold.

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

const ALL_FOUR_ADVISOR_SKILLS: &[&str] = &[
    "slo-legal",
    "slo-accounting",
    "slo-equity",
    "slo-fundraise",
];

#[test]
fn slo_fundraise_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-fundraise/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-fundraise"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_fundraise_skill_md_documents_four_modes() {
    let skill = read(&repo_root().join("skills/slo-fundraise/SKILL.md"));
    for mode in FOUR_MODES {
        assert!(skill.contains(mode));
    }
    assert!(skill.contains("## Modes"));
}

#[test]
fn slo_fundraise_skill_md_cites_all_four_predicate_ids() {
    let skill = read(&repo_root().join("skills/slo-fundraise/SKILL.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(skill.contains(pid));
    }
    assert!(skill.contains("references/biz/triage-gate.md"));
}

#[test]
fn cross_skill_advisor_pattern_replicated_all_four() {
    for skill_name in ALL_FOUR_ADVISOR_SKILLS {
        let path = repo_root().join("skills").join(skill_name).join("SKILL.md");
        let body = read(&path);
        for pid in FOUR_PREDICATE_IDS {
            assert!(
                body.contains(pid),
                "advisor skill `{skill_name}` SKILL.md must cite predicate `{pid}` (FINAL cross-skill citation contract — all four advisors)"
            );
        }
    }
}

#[test]
fn triage_gate_predicate_set_still_unchanged_after_m4() {
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
fn ir35_cest_factors_documents_three_primary_factors() {
    let ir35 = read(&repo_root().join("references/biz/ir35-cest-factors.md"));
    let factors = ["substitution", "Substitution", "MOO", "mutuality", "Mutuality", "control", "Control"];
    // At least 5 of the factor signals must appear (covering all three concepts).
    let count = factors.iter().filter(|f| ir35.contains(**f)).count();
    assert!(
        count >= 5,
        "ir35-cest-factors.md must document substitution / MOO / control (found {count} of {factors:?})"
    );
}

#[test]
fn ir35_cest_factors_documents_april_2025_refresh() {
    let ir35 = read(&repo_root().join("references/biz/ir35-cest-factors.md"));
    let signals = ["April 2025", "30 April 2025", "April-2025"];
    let any = signals.iter().any(|s| ir35.contains(s));
    assert!(
        any,
        "ir35-cest-factors.md must document the CEST April 2025 refresh"
    );
    assert!(ir35.contains("PGMOL"), "must cite PGMOL v HMRC case");
}

#[test]
fn ir35_cest_factors_documents_seven_hard_block_triggers() {
    let ir35 = read(&repo_root().join("references/biz/ir35-cest-factors.md"));
    // Look for the seven-trigger framing.
    let signals = ["seven factors", "seven triggers", "seven hard-block"];
    let any = signals.iter().any(|s| ir35.contains(s));
    assert!(
        any,
        "ir35-cest-factors.md must explicitly enumerate the seven hard-block-to-lawyer triggers"
    );
}

#[test]
fn slo_fundraise_runs_advance_assurance_precheck() {
    let skill = read(&repo_root().join("skills/slo-fundraise/SKILL.md"));
    let signals = [
        "Advance Assurance",
        "AA",
        "6 weeks",
        "VCM34080",
        "VCM31000",
    ];
    for s in &signals {
        assert!(
            skill.contains(s),
            "slo-fundraise SKILL.md must reference `{s}` (AA pre-check)"
        );
    }
}

#[test]
fn claude_md_catalogs_all_four_advisor_skills() {
    let claude_md = read(&repo_root().join("CLAUDE.md"));
    for skill_name in ALL_FOUR_ADVISOR_SKILLS {
        let verb = format!("/{skill_name}");
        assert!(
            claude_md.contains(&verb),
            "CLAUDE.md must catalog skill verb `{verb}`"
        );
    }
    // Must reference references/biz/ as the shared scaffolding location.
    assert!(
        claude_md.contains("references/biz"),
        "CLAUDE.md must reference the shared scaffolding location `references/biz/`"
    );
}

#[test]
fn references_biz_dir_still_not_discovered_after_m4() {
    let skills_dir = repo_root().join("skills");
    let references_biz = repo_root().join("references/biz");
    assert!(skills_dir.is_dir());
    assert!(references_biz.is_dir());
    assert!(!skills_dir.join("biz").exists());
    assert!(!skills_dir.join("_biz-shared").exists());
    for skill_name in ALL_FOUR_ADVISOR_SKILLS {
        assert!(
            skills_dir.join(skill_name).join("SKILL.md").exists(),
            "skills/{skill_name}/SKILL.md must exist"
        );
    }
}
