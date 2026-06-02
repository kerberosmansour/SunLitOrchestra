//! M4 structural-contract test (secure-value-loop runbook).
//!
//! Asserts the Detected Work Ledger discipline in `/slo-execute`, the
//! reconciliation to existing `/slo-retro` lanes (NO new lane verb — the
//! enforceable F-SEC-1 invariant), and `/slo-verify` recording Bundle A–F
//! security tests as first-class evidence rows. Also a regression guard that
//! the `/slo-verify` threat-model read-side contract phrases survive the edit
//! (slo_tm_m2_consumers.rs content-checks them).

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

const EXECUTE_SKILL: &str = "skills/slo-execute/SKILL.md";
const RETRO_SKILL: &str = "skills/slo-retro/SKILL.md";
const VERIFY_SKILL: &str = "skills/slo-verify/SKILL.md";

const FIVE_DISPOSITIONS: &[&str] = &[
    "fix_now",
    "file_github_issue",
    "operator_action",
    "upstream_feedback",
    "accepted_risk",
];

#[test]
fn execute_has_detected_work_ledger_discipline() {
    let c = read(EXECUTE_SKILL);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("detected work ledger"),
        "/slo-execute must add the Detected Work Ledger discipline"
    );
    for d in FIVE_DISPOSITIONS {
        assert!(
            c.contains(d),
            "/slo-execute must document the `{d}` disposition"
        );
    }
    assert!(
        (lc.contains("undisposed") || lc.contains("without a disposition") || lc.contains("never") )
            && lc.contains("done"),
        "/slo-execute must refuse `done` while a ledger row is undisposed (no finding left merely 'observed')"
    );
}

#[test]
fn dispositions_route_to_existing_retro_lanes_no_new_verb() {
    // F-SEC-1: the enforceable invariant is "no NEW /slo-retro lane verb".
    let retro = read(RETRO_SKILL);
    // The three existing lane verbs must still be the lane vocabulary.
    for lane in ["product", "upstream-OSS", "slo-process"] {
        assert!(
            retro.contains(lane),
            "/slo-retro must still carry the existing lane verb `{lane}`"
        );
    }
    // The ledger reconciliation must be explicit about introducing no new verb.
    let lc = retro.to_lowercase();
    assert!(
        lc.contains("ledger") && (lc.contains("no new lane") || lc.contains("existing lane")),
        "/slo-retro must reconcile the ledger dispositions to existing lanes (no new lane verb) — F-SEC-1"
    );
}

#[test]
fn retro_rereads_ledger() {
    let lc = read(RETRO_SKILL).to_lowercase();
    assert!(
        lc.contains("detected work ledger") || lc.contains("ledger"),
        "/slo-retro must re-read the Detected Work Ledger and dispose its rows"
    );
}

#[test]
fn verify_records_bundle_evidence_rows() {
    let c = read(VERIFY_SKILL);
    for bundle in [
        "Bundle A", "Bundle B", "Bundle C", "Bundle D", "Bundle E", "Bundle F",
    ] {
        assert!(
            c.contains(bundle),
            "/slo-verify must reference {bundle} as a security-test selection input"
        );
    }
    // Result vocabulary — never blank.
    assert!(
        c.contains("not_applicable") && c.contains("waived_with_reason"),
        "/slo-verify Bundle evidence rows must use pass | not_applicable | waived_with_reason (never blank)"
    );
}

/// Regression (slo_tm_m2_consumers.rs content-check): the threat-model
/// read-side contract phrases must survive the M4 edit to /slo-verify.
#[test]
fn verify_read_side_contract_phrases_survive() {
    let body = read(VERIFY_SKILL);
    for phrase in [
        "-threat-model.slo.json",
        "halt",
        "re-derive",
        "accepted_residual",
        "missing coverage",
        "~~~text",
    ] {
        assert!(
            body.contains(phrase),
            "/slo-verify must retain the read-side contract phrase {phrase:?} (additive edit only)"
        );
    }
}
