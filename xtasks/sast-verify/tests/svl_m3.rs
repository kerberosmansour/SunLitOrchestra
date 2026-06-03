//! M3 structural-contract test (secure-value-loop runbook).
//!
//! Asserts the additive milestone-status vocabulary + the Operator Readiness
//! Gate, across the v4 template comment, `/slo-execute`, `/slo-resume`, and the
//! canonical doc. The Rust enum totality + the F-ENG-1 silent-completion fix
//! are covered by `sldo-common`'s own unit tests (`runbook.rs mod tests`);
//! this file pins the contract-text surface.

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
const EXECUTE_SKILL: &str = "skills/slo-execute/SKILL.md";
const RESUME_SKILL: &str = "skills/slo-resume/SKILL.md";
const CANONICAL_DOC: &str = "docs/SECURE-VALUE-LOOP.md";

const OLD_FOUR: &[&str] = &["not_started", "in_progress", "blocked", "done"];
const NEW_FIVE: &[&str] = &[
    "human_review_required",
    "blocked_by_operator",
    "blocked_by_upstream",
    "issue_filed",
    "accepted_risk",
];

#[test]
fn status_enum_extended_additively_old_values_present() {
    for content in [read(TEMPLATE_MIRROR), read(TEMPLATE_PRIMARY)] {
        for old in OLD_FOUR {
            assert!(
                content.contains(old),
                "v4 template status comment must still list the original `{old}` value (additive, not replaced)"
            );
        }
        for new in NEW_FIVE {
            assert!(
                content.contains(new),
                "v4 template status comment must list the additive `{new}` honest exit state"
            );
        }
    }
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
fn unknown_status_maps_to_blocked_rule_documented() {
    let lc = read(TEMPLATE_MIRROR).to_lowercase();
    assert!(
        lc.contains("unrecognise") || lc.contains("does not recognise") || lc.contains("unknown"),
        "template must document the unknown-status rule"
    );
    assert!(
        lc.contains("treat it as `blocked`")
            || lc.contains("treat as `blocked`")
            || (lc.contains("blocked") && lc.contains("never silently")),
        "template must state unknown status → blocked, never silently done"
    );
}

#[test]
fn execute_global_entry_has_operator_readiness_gate() {
    let c = read(EXECUTE_SKILL);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("operator readiness gate"),
        "/slo-execute must add an Operator Readiness Gate to pre-flight"
    );
    assert!(
        c.contains("safe_to_continue_without_blockers") && lc.contains("fail closed"),
        "the gate must read safe_to_continue_without_blockers and fail closed"
    );
    assert!(
        c.contains("blocked_by_operator"),
        "the gate must set status blocked_by_operator when a prerequisite is unmet"
    );
}

#[test]
fn resume_recognizes_new_states() {
    let c = read(RESUME_SKILL);
    for new in NEW_FIVE {
        assert!(
            c.contains(new),
            "/slo-resume must recognise the additive status `{new}` (read-only orientation)"
        );
    }
    let lc = c.to_lowercase();
    assert!(
        lc.contains("unknown") && lc.contains("blocked"),
        "/slo-resume must apply the unknown→blocked fail-safe"
    );
}

#[test]
fn operator_action_label_documented() {
    let d = read(CANONICAL_DOC);
    assert!(
        d.contains("operator-action-required") && d.contains("security-review-required"),
        "SECURE-VALUE-LOOP.md must document both GitHub labels (proposal §9 #9/#10)"
    );
}
