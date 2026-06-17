//! M2 structural-contract test (outcome-first runbook).
//!
//! Asserts `/slo-plan` enforces the Outcome Validation Contract:
//!
//! - `slo-plan/SKILL.md` requires §5C + outcome sections for value-bearing
//!   milestones (peer to §5A/§5B, same deterministic "value-bearing" trigger).
//! - the specificity gate covers outcome-shape (vacuous / single-`And` / mock-only
//!   / monolithic-Front-to-End are refused).
//! - forward-looking: flags the gap, does not invalidate legacy runbooks.
//! - the new authoring reference `references/outcome-validation-contract.md`
//!   carries the per-layer Front-to-End + >=1 cross-layer assertion (theme B),
//!   the never-blank resolution enum, the `oc-`/`cuj-` id rules, and anti-theatre.
//! - `slo-plan/SKILL.md` SHA-256 == pinned baseline (founder SHA-pin direction).

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

const PLAN_SKILL: &str = "skills/slo-plan/SKILL.md";
const OUTCOME_REF: &str = "skills/slo-plan/references/outcome-validation-contract.md";

/// SHA-256 of `skills/slo-plan/SKILL.md` after the M2 Outcome Validation Contract
/// requirement was added (outcome-first runbook M2). Founder SHA-pin direction
/// (2026-06-17): edited orchestration SKILL.md files are pinned byte-identical;
/// the constant moves in lockstep with the edit, inside M2's allow-list.
const PLAN_SKILL_SHA256: &str = "337581d179cf441d5ae446a2ab2751c2894ad8d7201356591d206b0bfdcd4f3d";

#[test]
fn plan_requires_outcome_contract_for_value_bearing() {
    let c = read(PLAN_SKILL);
    let lc = c.to_lowercase();
    assert!(
        c.contains("Outcome Validation Contract") && lc.contains("value-bearing"),
        "/slo-plan must require the Outcome Validation Contract for value-bearing milestones"
    );
}

#[test]
fn plan_specificity_gate_covers_outcome_shape() {
    // Theme B + tm-outcome-first-abuse-2: vacuous / mock-only / monolithic refused.
    let lc = read(PLAN_SKILL).to_lowercase();
    assert!(
        lc.contains("vacuous") || lc.contains("mock-only") || lc.contains("theatre"),
        "/slo-plan must refuse vacuous/mock-only/theatre outcome scenarios"
    );
    assert!(
        lc.contains("per-layer"),
        "/slo-plan must require per-layer Front-to-End (no monolithic block)"
    );
}

#[test]
fn plan_forward_looking_not_retroactive() {
    let lc = read(PLAN_SKILL).to_lowercase();
    assert!(
        lc.contains("flag") && lc.contains("legacy"),
        "/slo-plan must flag a missing §5C without invalidating legacy runbooks"
    );
}

#[test]
fn outcome_validation_contract_reference_complete() {
    let c = read(OUTCOME_REF);
    let lc = c.to_lowercase();
    for needle in [
        "per-layer",
        "cross-layer assertion",
        "oc-<slug>-",
        "cuj-<slug>-",
    ] {
        assert!(
            c.contains(needle),
            "outcome-validation-contract.md must document `{needle}`"
        );
    }
    assert!(
        lc.contains("never blank") && c.contains("waived_with_reason"),
        "reference must document the never-blank resolution enum"
    );
    assert!(
        lc.contains("theatre") || lc.contains("anti-theatre"),
        "reference must document the anti-theatre rule"
    );
}

#[test]
fn plan_skill_md_sha_pinned() {
    let hex = sha256_of(PLAN_SKILL);
    assert_eq!(
        hex, PLAN_SKILL_SHA256,
        "skills/slo-plan/SKILL.md SHA-256 changed (expected {PLAN_SKILL_SHA256}, got {hex}) — \
         update the baseline constant in lockstep within the same milestone (founder SHA-pin direction)."
    );
}
