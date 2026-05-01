//! M2 structural-contract tests for the loops-and-lessons-closure runbook.
//!
//! These tests assert that `docs/LOOPS-BUSINESS.md` exists, has the four
//! documented loop sections (user-interview, GTM, pricing, founder-check),
//! opens with a "Start here" outcome-first orienter, follows M1's
//! per-loop schema, and is cross-linked from each cited business SKILL.md.
//!
//! Backwards-compat: M1's LOOPS-ENGINEERING.md and ARCHITECTURE cross-link
//! must remain valid.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` Milestone 2.

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

const LOOP_SECTIONS: &[&str] = &[
    "User-interview loop",
    "GTM loop",
    "Pricing loop",
    "Founder-check loop",
];

// Business skills cited under at least one loop in LOOPS-BUSINESS.md.
// Every skill listed here must carry a back-link to LOOPS-BUSINESS.md
// in its SKILL.md so the cross-reference invariant holds bidirectionally.
const CITED_BUSINESS_SKILLS: &[&str] = &[
    "slo-talk-to-users",
    "slo-gtm",
    "slo-product",
    "slo-marketing",
    "slo-sales-funnel",
    "slo-launch",
    "slo-pricing",
    "slo-metrics",
    "slo-fundraise",
    "slo-founder-check",
    "slo-cofounder",
    "slo-hire",
    "slo-equity",
    "slo-legal",
    "slo-accounting",
];

#[test]
fn loops_business_doc_exists_and_has_required_sections() {
    let doc = read(&repo_root().join("docs/LOOPS-BUSINESS.md"));

    for section in LOOP_SECTIONS {
        assert!(
            doc.contains(section),
            "LOOPS-BUSINESS.md missing required section: {section}"
        );
    }

    for marker in &[
        "Trigger",
        "Exit condition",
        "Skills involved",
        "User-visible outcome",
    ] {
        assert!(
            doc.contains(marker),
            "LOOPS-BUSINESS.md missing per-loop schema marker: {marker}"
        );
    }
}

#[test]
fn loops_business_doc_has_start_here_orienter() {
    let doc = read(&repo_root().join("docs/LOOPS-BUSINESS.md"));

    assert!(
        doc.contains("Start here"),
        "LOOPS-BUSINESS.md missing 'Start here' outcome-first orienter"
    );
    assert!(
        doc.contains("User-interview loop") || doc.contains("user-interview loop"),
        "Start here orienter must reference the user-interview loop entrypoint"
    );
}

#[test]
fn every_cited_business_skill_has_cross_reference() {
    let root = repo_root();
    for skill_name in CITED_BUSINESS_SKILLS {
        let skill_path = root.join(format!("skills/{skill_name}/SKILL.md"));
        let body = read(&skill_path);
        assert!(
            body.contains("LOOPS-BUSINESS.md"),
            "skills/{skill_name}/SKILL.md missing cross-reference to LOOPS-BUSINESS.md \
             (skill is cited in a business loop section, so the back-link is required)"
        );
    }
}

// Backward-compat: M1's LOOPS-ENGINEERING.md must still be present and the
// ARCHITECTURE.md cross-link to it must remain. This guards against M2
// silently breaking M1's invariants.
#[test]
fn m1_engineering_loops_doc_unchanged_and_cross_linked() {
    let eng = read(&repo_root().join("docs/LOOPS-ENGINEERING.md"));
    assert!(
        eng.contains("Sprint loop") && eng.contains("Lessons loop"),
        "M1's LOOPS-ENGINEERING.md must still contain Sprint loop and Lessons loop"
    );
    let arch = read(&repo_root().join("docs/ARCHITECTURE.md"));
    assert!(
        arch.contains("LOOPS-ENGINEERING.md") && arch.contains("LOOPS-BUSINESS.md"),
        "ARCHITECTURE.md must cross-link to BOTH LOOPS-ENGINEERING.md and LOOPS-BUSINESS.md"
    );
}

// PII-discipline rule: LOOPS-BUSINESS.md must not contain real-looking
// interview quotes. The runbook's `tm-loops-abuse-2` mitigation is the
// structural rule "all examples use Alice / Bob pseudonyms".
#[test]
fn loops_business_doc_uses_pseudonyms_in_examples() {
    let doc = read(&repo_root().join("docs/LOOPS-BUSINESS.md"));
    // If the doc references any interview quote at all, it must use the
    // pseudonym pair. We assert presence of pseudonyms only when an
    // example marker is present; otherwise the test passes trivially.
    let has_example_marker =
        doc.contains("Example:") || doc.contains("e.g., \"") || doc.contains("interview quote");
    if has_example_marker {
        assert!(
            doc.contains("Alice") || doc.contains("Bob"),
            "LOOPS-BUSINESS.md interview-quote examples must use Alice / Bob pseudonyms \
             (per tm-loops-abuse-2 mitigation)"
        );
    }
}
