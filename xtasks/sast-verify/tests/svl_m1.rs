//! M1 structural-contract test (secure-value-loop runbook).
//!
//! Asserts the canonical envelope doc + the v4 template §5B "Secure Value and
//! Security Contract" section + the tightened proactive-controls citation:
//!
//! - `docs/SECURE-VALUE-LOOP.md` exists and carries the operating rule, the
//!   Bundle A–F table, the ledger↔/slo-retro lane mapping, the additive status
//!   values, the `~~~text` fence rule with its named generation surfaces
//!   (F-SEC-3), and the agent one-page prompt.
//! - BOTH v4 template copies carry an optional §5B section with all five
//!   sub-blocks (Value Wedge, Security Definition of Ready, Threat Model
//!   Summary, Security Test Plan, Detected Work Ledger) + optional/legacy
//!   framing.
//! - The two copies stay byte-identical (the documented primary/mirror sync).
//! - The Contract Block proactive-controls row cites OWASP Proactive Controls
//!   **2024 by name**, not a bare number (the 2018→2024 renumber drift fix).
//! - Existing template sections §5A / §6 / §10 / §17 are not renumbered/removed.

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    let p = workspace_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {}: {}", p.display(), e))
}

const TEMPLATE_MIRROR: &str = "docs/slo/templates/runbook-template_v_4_template.md";
const TEMPLATE_PRIMARY: &str = "skills/slo-plan/references/runbook-template_v_4_template.md";
const CANONICAL_DOC: &str = "docs/SECURE-VALUE-LOOP.md";

/// The five §5B sub-block headings the contract must carry.
const FIVE_SUBBLOCKS: &[&str] = &[
    "Value Wedge",
    "Security Definition of Ready",
    "Threat Model Summary",
    "Security Test Plan",
    "Detected Work Ledger",
];

fn assert_section5b_in(label: &str, content: &str) {
    assert!(
        content.contains("## 5B. Secure Value and Security Contract"),
        "{label}: must carry the §5B Secure Value and Security Contract section (letter-suffixed, no renumber)"
    );
    for sub in FIVE_SUBBLOCKS {
        assert!(
            content.contains(sub),
            "{label}: §5B must include the `{sub}` sub-block"
        );
    }
    // Optional / legacy-valid framing (mirrors §5A + §10 precedent).
    let lc = content.to_lowercase();
    assert!(
        lc.contains("optional") && lc.contains("legacy runbooks"),
        "{label}: §5B must be optional-by-shape (legacy runbooks remain valid)"
    );
}

#[test]
fn secure_value_section_present_in_both_template_copies() {
    assert_section5b_in("repo mirror", &read(TEMPLATE_MIRROR));
    assert_section5b_in("skill primary", &read(TEMPLATE_PRIMARY));
}

#[test]
fn template_copies_stay_byte_identical() {
    assert_eq!(
        read(TEMPLATE_PRIMARY),
        read(TEMPLATE_MIRROR),
        "the skill-primary v4 template and the repo mirror must stay byte-identical"
    );
}

#[test]
fn proactive_controls_row_named_and_editioned() {
    // The 2018→2024 renumber drift fix: cite by name + edition, never bare `Cn`.
    let t = read(TEMPLATE_MIRROR);
    assert!(
        t.contains("OWASP Proactive Controls 2024"),
        "Contract Block proactive-controls row must name the 2024 edition explicitly"
    );
    assert!(
        t.contains("Implement Access Control") || t.contains("Address Security from the Start"),
        "proactive-controls row must cite at least one 2024 control by name, not only a bare number"
    );
}

#[test]
fn existing_sections_not_renumbered() {
    let t = read(TEMPLATE_MIRROR);
    for heading in [
        "## 5A. Measurement Contract",
        "## 6. Global Execution Rules",
        "## 10. Carry-forward from prior retros",
        "## 17. Milestone Plan",
    ] {
        assert!(
            t.contains(heading),
            "existing template heading `{heading}` must be preserved (no renumber/removal)"
        );
    }
}

#[test]
fn canonical_doc_present_and_complete() {
    let d = read(CANONICAL_DOC);
    // Operating rule.
    assert!(
        d.contains("smallest valuable, secure, testable, unblocked, reviewable"),
        "SECURE-VALUE-LOOP.md must state the operating rule"
    );
    // Bundle A–F table.
    for bundle in [
        "Bundle A", "Bundle B", "Bundle C", "Bundle D", "Bundle E", "Bundle F",
    ] {
        assert!(
            d.contains(bundle),
            "SECURE-VALUE-LOOP.md must define {bundle}"
        );
    }
    // Standards cited by name+edition (anti-drift).
    assert!(
        d.contains("OWASP Proactive Controls 2024") && d.contains("NIST SSDF"),
        "SECURE-VALUE-LOOP.md must cite SSDF + OWASP Proactive Controls 2024 by name"
    );
}

#[test]
fn canonical_doc_ledger_lane_mapping_and_status_values() {
    let d = read(CANONICAL_DOC);
    // Five ledger dispositions.
    for disp in [
        "fix_now",
        "file_github_issue",
        "operator_action",
        "upstream_feedback",
        "accepted_risk",
    ] {
        assert!(
            d.contains(disp),
            "ledger disposition `{disp}` must be documented"
        );
    }
    // Reuse existing /slo-retro lanes — the no-new-taxonomy contract.
    assert!(
        d.contains("upstream-OSS") && d.contains("slo-process"),
        "ledger dispositions must map onto existing /slo-retro lanes (upstream-OSS, slo-process)"
    );
    // Five additive status values.
    for status in [
        "human_review_required",
        "blocked_by_operator",
        "blocked_by_upstream",
        "issue_filed",
        "accepted_risk",
    ] {
        assert!(
            d.contains(status),
            "additive status value `{status}` must be documented"
        );
    }
    // Unknown-status fail-safe.
    let lc = d.to_lowercase();
    assert!(
        lc.contains("unknown") && lc.contains("blocked"),
        "SECURE-VALUE-LOOP.md must document the unknown-status → blocked fail-safe"
    );
}

#[test]
fn canonical_doc_fence_rule_and_named_surfaces() {
    // F-SEC-3: the fence rule must name the concrete generation surfaces it
    // protects, not over-claim over inert author prose.
    let d = read(CANONICAL_DOC);
    assert!(
        d.contains("~~~text"),
        "SECURE-VALUE-LOOP.md must document the ~~~text fence rule (tm-secure-value-loop-abuse-1)"
    );
    assert!(
        d.contains("/slo-ship") && d.contains("/slo-resume"),
        "the fence rule must name its concrete generation surfaces (/slo-ship PR body, /slo-resume snippets) — F-SEC-3"
    );
}

#[test]
fn canonical_doc_has_agent_prompt() {
    let lc = read(CANONICAL_DOC).to_lowercase();
    assert!(
        lc.contains("one-page agent prompt") || lc.contains("agent prompt"),
        "SECURE-VALUE-LOOP.md must carry the one-page agent prompt"
    );
}
