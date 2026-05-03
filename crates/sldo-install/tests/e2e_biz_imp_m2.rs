//! M2 structural-contract tests for Business Skill Improvements.
//!
//! Verifies the conversational intake layer:
//! - Five intake contracts exist and frame intake as conversation, not a form.
//! - F1/F4/F5 shared gate fields are byte-identical across the five contracts.
//! - The legal starter was renamed from `legal-intake-form.md`.
//! - Each advisor skill cites its intake contract and restate-confirm discipline.
//! - The four advisor hard-block predicate IDs remain immutable.

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

fn section<'a>(body: &'a str, start: &str, end: &str) -> &'a str {
    let start_idx = body
        .find(start)
        .unwrap_or_else(|| panic!("missing section marker `{start}`"));
    let after_start = &body[start_idx..];
    let end_idx = after_start
        .find(end)
        .unwrap_or_else(|| panic!("missing end marker `{end}` after `{start}`"));
    after_start[..end_idx].trim()
}

const CONTRACTS: &[(&str, &str)] = &[
    ("legal", "references/biz/legal-intake-contract.md"),
    ("accounting", "references/biz/accounting-intake-contract.md"),
    ("equity", "references/biz/equity-intake-contract.md"),
    ("fundraise", "references/biz/fundraise-intake-contract.md"),
    ("hire", "references/biz/hire-intake-contract.md"),
];

const ADVISOR_SKILLS: &[(&str, &str)] = &[
    ("legal", "skills/slo-legal/SKILL.md"),
    ("accounting", "skills/slo-accounting/SKILL.md"),
    ("equity", "skills/slo-equity/SKILL.md"),
    ("fundraise", "skills/slo-fundraise/SKILL.md"),
    ("hire", "skills/slo-hire/SKILL.md"),
];

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

#[test]
fn five_intake_contracts_exist_with_conversational_framing() {
    for (name, rel) in CONTRACTS {
        let path = repo_root().join(rel);
        assert!(path.exists(), "{name} intake contract missing at {rel}");
        let body = read(&path);
        assert!(
            body.contains("Conversation is the UX"),
            "{rel} must state the Conversation is the UX framing"
        );
        assert!(
            body.contains("## Restate-and-confirm step"),
            "{rel} must define a restate-and-confirm step"
        );
        for field in ["### F1.", "### F2.", "### F3.", "### F4.", "### F5.", "### F6."] {
            assert!(body.contains(field), "{rel} missing required field {field}");
        }
        for forbidden in [
            "submit a form",
            "fill in this form",
            "founder fills in",
            "founder should fill",
        ] {
            assert!(
                !body.to_lowercase().contains(forbidden),
                "{rel} must not reframe intake as a founder-filled form (`{forbidden}`)"
            );
        }
    }
}

#[test]
fn f1_f4_f5_verbatim_across_sisters() {
    let root = repo_root();
    let legal = read(&root.join("references/biz/legal-intake-contract.md"));
    let canonical_f1 = section(&legal, "### F1.", "### F2.");
    let canonical_f4 = section(&legal, "### F4.", "### F5.");
    let canonical_f5 = section(&legal, "### F5.", "### F6.");

    for (name, rel) in CONTRACTS.iter().skip(1) {
        let body = read(&root.join(rel));
        assert_eq!(
            canonical_f1,
            section(&body, "### F1.", "### F2."),
            "{name} F1 must match legal-intake-contract.md verbatim"
        );
        assert_eq!(
            canonical_f4,
            section(&body, "### F4.", "### F5."),
            "{name} F4 must match legal-intake-contract.md verbatim"
        );
        assert_eq!(
            canonical_f5,
            section(&body, "### F5.", "### F6."),
            "{name} F5 must match legal-intake-contract.md verbatim"
        );
    }
}

#[test]
fn every_advisor_skill_md_cites_contract_and_restate_discipline() {
    for (name, rel) in ADVISOR_SKILLS {
        let body = read(&repo_root().join(rel));
        let contract_ref = format!("references/biz/{name}-intake-contract.md");
        assert!(
            body.contains(&contract_ref),
            "{rel} must cite its intake contract `{contract_ref}`"
        );
        assert!(
            body.contains("Restate-and-confirm")
                || body.contains("restate-and-confirm")
                || body.contains("restate_confirm"),
            "{rel} must document restate-and-confirm discipline"
        );
        assert!(
            body.contains("refuse") && body.contains("ambigu"),
            "{rel} must document refusal-on-ambiguity"
        );
    }
}

#[test]
fn every_advisor_skill_md_cites_m1_authority_files() {
    let authority_refs = [
        "references/biz/uk-regulator-enumeration.md",
        "references/biz/uk-employment-statute-anchors.md",
        "references/biz/uk-consumer-statute-anchors.md",
        "references/biz/uk-marketing-statute-anchors.md",
        "references/biz/hmrc-vcm-index.md",
        "references/biz/ico-duaa-index.md",
    ];

    for (_, rel) in ADVISOR_SKILLS {
        let body = read(&repo_root().join(rel));
        assert!(
            body.contains("references/biz/uk-regulator-enumeration.md"),
            "{rel} must cite the closed regulator enum"
        );
        assert!(
            authority_refs.iter().any(|authority| body.contains(authority)),
            "{rel} must cite at least one source-verified M1 authority file"
        );
    }
}

#[test]
fn legal_intake_form_renamed() {
    let root = repo_root();
    assert!(
        !root.join("references/biz/legal-intake-form.md").exists(),
        "legal-intake-form.md must be renamed, not left as a duplicate"
    );
    assert!(
        root.join("references/biz/legal-intake-contract.md").exists(),
        "legal-intake-contract.md must exist after rename"
    );
}

#[test]
fn triage_gate_predicate_set_unchanged_after_m2() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(gate.contains(pid), "triage-gate.md missing `{pid}`");
    }
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(
            !gate.contains(&candidate),
            "triage-gate.md must not contain extra predicate pattern `{candidate}`"
        );
    }
}
