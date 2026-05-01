//! M2 structural-contract tests for the biz-skill-pack Runbook A.
//!
//! Verifies:
//! - `/slo-accounting` SKILL.md has required frontmatter + four-mode contract
//!   + four-predicate-id citations.
//! - The five new shared references parse correctly:
//!   `references/biz/{artifact-schema,jurisdiction-uk,ico-duaa-index,
//!   ico-enforcement-reality,open-template-anchors}.md`.
//! - Cross-skill citation: every advisor SKILL.md (`slo-legal`, `slo-accounting`)
//!   cites all four predicate IDs from `references/biz/triage-gate.md`.
//! - The triage-gate.md predicate-id set is unchanged from M1 (immutability).
//! - artifact-schema.md tier-enum is constrained to {confidential, public}.
//! - jurisdiction-uk.md carries the canonical "v1 supports UK only" error string.
//! - ico-duaa-index.md carries the three DUAA 2026 dates.
//! - ico-enforcement-reality.md carries the non-normative disclaimer + does NOT
//!   contain forbidden phrases that would contradict gate-4.
//! - open-template-anchors.md documents the oneNDA CC BY-ND 4.0 verbatim rule.
//! - `references/biz/` still NOT discovered as a skill (regression on M1
//!   invariant).

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

const FOUR_MODES: &[&str] = &["draft", "translate", "triage", "prepare"];

const ADVISOR_SKILLS: &[&str] = &["slo-legal", "slo-accounting"];

// ---------------------------------------------------------------------------
// BDD #1 — slo-accounting SKILL.md has required frontmatter.
// ---------------------------------------------------------------------------

#[test]
fn slo_accounting_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-accounting/SKILL.md"));
    assert!(
        skill.starts_with("---\n"),
        "SKILL.md must start with `---\\n`"
    );
    let after_open = &skill[4..];
    let close_idx = after_open
        .find("\n---\n")
        .expect("SKILL.md frontmatter must close with `---`");
    let frontmatter = &after_open[..close_idx];
    assert!(
        frontmatter.contains("name: slo-accounting"),
        "frontmatter must contain `name: slo-accounting`"
    );
    assert!(
        frontmatter.contains("description:"),
        "frontmatter must contain a `description:` key"
    );
}

// ---------------------------------------------------------------------------
// BDD #2 — slo-accounting documents the four advisor modes.
// ---------------------------------------------------------------------------

#[test]
fn slo_accounting_skill_md_documents_four_modes() {
    let skill = read(&repo_root().join("skills/slo-accounting/SKILL.md"));
    for mode in FOUR_MODES {
        assert!(
            skill.contains(mode),
            "slo-accounting SKILL.md must mention mode `{mode}`"
        );
    }
    assert!(
        skill.contains("## Modes") || skill.contains("# Modes"),
        "slo-accounting SKILL.md must have a `## Modes` heading"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — slo-accounting cites all four predicate IDs.
// ---------------------------------------------------------------------------

#[test]
fn slo_accounting_skill_md_cites_all_four_predicate_ids() {
    let skill = read(&repo_root().join("skills/slo-accounting/SKILL.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(
            skill.contains(pid),
            "slo-accounting SKILL.md must cite predicate id `{pid}`"
        );
    }
    assert!(
        skill.contains("references/biz/triage-gate.md"),
        "slo-accounting SKILL.md must reference `references/biz/triage-gate.md`"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — Cross-skill advisor pattern: every advisor SKILL.md cites every
// predicate ID. This is the milestone's load-bearing test — proves the
// advisor pattern replicates without divergence.
// ---------------------------------------------------------------------------

#[test]
fn cross_skill_advisor_pattern_replicated() {
    for skill_name in ADVISOR_SKILLS {
        let path = repo_root().join("skills").join(skill_name).join("SKILL.md");
        let body = read(&path);
        for pid in FOUR_PREDICATE_IDS {
            assert!(
                body.contains(pid),
                "advisor skill `{skill_name}` SKILL.md must cite predicate id `{pid}` (cross-skill citation contract)"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// BDD #5 — triage-gate.md predicate-id set unchanged from M1 (immutability).
// ---------------------------------------------------------------------------

#[test]
fn triage_gate_predicate_set_unchanged_from_m1() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));

    // The exact four predicate IDs must be present.
    for pid in FOUR_PREDICATE_IDS {
        assert!(
            gate.contains(pid),
            "triage-gate.md must contain predicate id `{pid}` (immutability — M1 contract)"
        );
    }

    // The file must NOT contain a fifth predicate id matching the `gate-N-`
    // pattern beyond the four. We approximate by looking for `gate-5-`,
    // `gate-6-`, etc.
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(
            !gate.contains(&candidate),
            "triage-gate.md must NOT contain predicate id pattern `{candidate}` — adding a fifth gate is a /slo-architect decision, not a milestone option"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #6 — artifact-schema.md tier value constrained to enum.
// ---------------------------------------------------------------------------

#[test]
fn artifact_schema_tier_value_constrained_to_enum() {
    let schema = read(&repo_root().join("references/biz/artifact-schema.md"));

    // Both enum values must be documented.
    assert!(
        schema.contains("`confidential`"),
        "artifact-schema.md must document `confidential` as a tier value"
    );
    assert!(
        schema.contains("`public`"),
        "artifact-schema.md must document `public` as a tier value"
    );

    // Schema must explicitly declare these are the ONLY two values.
    let exclusivity_signals = [
        "Exactly two permitted values",
        "exactly two permitted values",
        "No free-form",
        "no free-form",
        "two values:",
    ];
    let any_exclusive = exclusivity_signals.iter().any(|s| schema.contains(s));
    assert!(
        any_exclusive,
        "artifact-schema.md must declare the tier enum is constrained to two values (looked for any of {exclusivity_signals:?})"
    );
}

// ---------------------------------------------------------------------------
// BDD #7 — jurisdiction-uk.md has the canonical "v1 supports UK only" error.
// ---------------------------------------------------------------------------

#[test]
fn jurisdiction_uk_md_has_canonical_error_string() {
    let juris = read(&repo_root().join("references/biz/jurisdiction-uk.md"));
    assert!(
        juris.contains("v1 supports UK only"),
        "jurisdiction-uk.md must contain the canonical error string `v1 supports UK only`"
    );
    assert!(
        juris.contains("v2 architectural pivot"),
        "jurisdiction-uk.md must reference `v2 architectural pivot` as the future expansion path"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 — ico-duaa-index.md carries the three 2026 DUAA dates.
// ---------------------------------------------------------------------------

#[test]
fn ico_duaa_index_carries_2026_dates() {
    let duaa = read(&repo_root().join("references/biz/ico-duaa-index.md"));
    let expected_dates = ["2025-06-19", "2026-02-05", "2026-06-19"];
    for date in &expected_dates {
        assert!(
            duaa.contains(date),
            "ico-duaa-index.md must carry the DUAA date `{date}`"
        );
    }
    assert!(
        duaa.contains("ico.org.uk"),
        "ico-duaa-index.md must cite ico.org.uk as the source"
    );
    assert!(
        duaa.contains("£17.5M") || duaa.contains("17.5M"),
        "ico-duaa-index.md must document the £17.5M PECR ceiling"
    );
}

// ---------------------------------------------------------------------------
// BDD #9 — ico-enforcement-reality.md is descriptive-not-normative.
// ---------------------------------------------------------------------------

#[test]
fn ico_enforcement_reality_doc_does_not_contradict_gate_4() {
    let reality = read(&repo_root().join("references/biz/ico-enforcement-reality.md"));

    // Non-normative disclaimer must appear early in the doc (within the
    // first 30 lines).
    let first_30: String = reality.lines().take(30).collect::<Vec<_>>().join("\n");
    let disclaimer_signals = [
        "DESCRIPTIVE NOT NORMATIVE",
        "descriptive not normative",
        "descriptive provenance",
        "NOT authorization",
        "not authorization",
    ];
    let any_disclaimer = disclaimer_signals.iter().any(|s| first_30.contains(s));
    assert!(
        any_disclaimer,
        "ico-enforcement-reality.md must lead with a non-normative disclaimer in the first 30 lines (looked for any of {disclaimer_signals:?})"
    );

    // The doc must NOT contain phrases that would authorize relaxing gate-4.
    let forbidden_phrases = [
        "narrow gate-4",
        "relax gate-4",
        "drop the broad block",
        "drafting privacy notices is safe",
        "draft privacy notices",
    ];
    for phrase in &forbidden_phrases {
        assert!(
            !reality.contains(phrase),
            "ico-enforcement-reality.md must NOT contain `{phrase}` — the doc is descriptive provenance, not authorization to weaken gate-4"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #10 — open-template-anchors.md documents the oneNDA license rule.
// ---------------------------------------------------------------------------

#[test]
fn open_template_anchors_documents_onenda_license() {
    let anchors = read(&repo_root().join("references/biz/open-template-anchors.md"));
    assert!(
        anchors.contains("CC BY-ND 4.0"),
        "open-template-anchors.md must cite CC BY-ND 4.0 as oneNDA's license"
    );
    let verbatim_signals = ["verbatim", "byte-for-byte", "unmodified", "No Derivatives"];
    let any_verbatim = verbatim_signals.iter().any(|s| anchors.contains(s));
    assert!(
        any_verbatim,
        "open-template-anchors.md must document the oneNDA verbatim-render obligation (looked for any of {verbatim_signals:?})"
    );
    assert!(
        anchors.contains("references/biz/templates/onenda-uk.md"),
        "open-template-anchors.md must point to the placeholder file path"
    );
}

// ---------------------------------------------------------------------------
// BDD #11 — references/biz/ still NOT discovered as a skill (regression).
// ---------------------------------------------------------------------------

#[test]
fn references_biz_dir_still_not_discovered_as_skill() {
    let skills_dir = repo_root().join("skills");
    let references_biz = repo_root().join("references/biz");

    assert!(
        skills_dir.exists() && skills_dir.is_dir(),
        "skills/ directory must exist"
    );
    assert!(
        references_biz.exists() && references_biz.is_dir(),
        "references/biz/ directory must exist"
    );
    let misplaced = skills_dir.join("biz");
    assert!(
        !misplaced.exists(),
        "skills/biz/ must NOT exist — references/biz/ lives outside skills/"
    );
    let misplaced_underscore = skills_dir.join("_biz-shared");
    assert!(
        !misplaced_underscore.exists(),
        "skills/_biz-shared/ must NOT exist"
    );
    // M1 + M2 advisor skills must exist under skills/.
    for skill_name in ADVISOR_SKILLS {
        let path = skills_dir.join(skill_name).join("SKILL.md");
        assert!(path.exists(), "skills/{skill_name}/SKILL.md must exist");
    }
}
