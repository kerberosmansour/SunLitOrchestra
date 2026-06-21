//! M3 structural-contract test (outcome-first runbook).
//!
//! Asserts `/slo-verify` gains a leading, highest-authority **Pass 0: Outcome
//! Validation**, inserted WITHOUT renumbering Passes 1–6 (DW-001):
//!
//! - a "Pass 0" / "Outcome Validation" section exists and is positioned BEFORE
//!   "### Pass 1. Happy path".
//! - the authority-override rule is stated (a Pass 0 failure fails the milestone
//!   even if Passes 1–6 are green).
//! - Pass 0 runs front-to-end over the highest APPLICABLE layer chain (theme B),
//!   never a single mock, and reuses the existing regression-test-first flow.
//! - existing Pass 4/5/6 headings are unchanged (no renumber).
//! - the reference `references/outcome-validation-pass.md` documents the
//!   procedure (front-to-end ordering, regression-matrix re-run, finding flow).
//! - `slo-verify/SKILL.md` SHA-256 == pinned baseline (founder SHA-pin direction).

use sha2::{Digest, Sha256};
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

fn sha256_of(rel: &str) -> String {
    let bytes = std::fs::read(workspace_root().join(rel)).expect("read for sha");
    let mut h = Sha256::new();
    h.update(&bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

const VERIFY_SKILL: &str = "skills/slo-verify/SKILL.md";
const PASS_REF: &str = "skills/slo-verify/references/outcome-validation-pass.md";

/// SHA-256 of `skills/slo-verify/SKILL.md` after the M3 Pass 0 insertion
/// (outcome-first runbook M3). Founder SHA-pin direction (2026-06-17): edited
/// orchestration SKILL.md files are pinned byte-identical; the constant moves in
/// lockstep with the edit, inside M3's allow-list.
const VERIFY_SKILL_SHA256: &str =
    "755e2016440f4ca4fe1459cc3678dd2a3f12fcaa25b68287c9c4c9375d8de2c8";

#[test]
fn verify_has_pass_0_outcome_validation_before_pass_1() {
    let c = read(VERIFY_SKILL);
    assert!(
        c.contains("### Pass 0. Outcome Validation"),
        "/slo-verify must add a leading `### Pass 0. Outcome Validation` section"
    );
    let i0 = c.find("### Pass 0. Outcome Validation").expect("Pass 0");
    let i1 = c.find("### Pass 1. Happy path").expect("Pass 1");
    assert!(
        i0 < i1,
        "Pass 0 must be positioned BEFORE Pass 1 (leading pass)"
    );
}

#[test]
fn pass_0_is_highest_authority() {
    let lc = read(VERIFY_SKILL).to_lowercase();
    assert!(
        lc.contains("highest authority"),
        "Pass 0 must be declared the highest authority"
    );
    assert!(
        lc.contains("even if passes 1–6 are green") || lc.contains("even if passes 1-6 are green"),
        "Pass 0 must state the override rule (fails the milestone even if Passes 1–6 are green)"
    );
}

#[test]
fn pass_0_runs_front_to_end_not_mock() {
    let lc = read(VERIFY_SKILL).to_lowercase();
    assert!(
        lc.contains("front-to-end") && lc.contains("highest applicable layer"),
        "Pass 0 must run front-to-end over the highest applicable layer chain (theme B)"
    );
    assert!(
        lc.contains("never a single mock") || lc.contains("mock-only"),
        "Pass 0 must reject a mock-only assertion (tm-outcome-first-abuse-2)"
    );
}

#[test]
fn pass_0_reuses_bug_found_flow() {
    let lc = read(VERIFY_SKILL).to_lowercase();
    assert!(
        lc.contains("reuses the existing") || lc.contains("regression-test-first"),
        "Pass 0 must reuse the existing regression-test-first bug-found flow, not fork a new one"
    );
}

#[test]
fn verify_passes_not_renumbered() {
    let c = read(VERIFY_SKILL);
    for h in [
        "### Pass 1. Happy path",
        "### Pass 4. Security",
        "### Pass 5. AI tolerance",
        "### Pass 6. Measurement",
    ] {
        assert!(
            c.contains(h),
            "existing pass `{h}` must keep its number (no renumber, DW-001)"
        );
    }
}

#[test]
fn outcome_validation_pass_reference_complete() {
    let c = read(PASS_REF);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("front-to-end") && lc.contains("regression matrix"),
        "reference must document the front-to-end procedure + regression-matrix re-run"
    );
    assert!(
        lc.contains("regression-test-first") || lc.contains("regression test first"),
        "reference must document the regression-test-first finding flow"
    );
}

#[test]
fn verify_skill_md_sha_pinned() {
    let hex = sha256_of(VERIFY_SKILL);
    assert_eq!(
        hex, VERIFY_SKILL_SHA256,
        "skills/slo-verify/SKILL.md SHA-256 changed (expected {VERIFY_SKILL_SHA256}, got {hex}) — \
         update the baseline constant in lockstep within M3 (founder SHA-pin direction)."
    );
}
