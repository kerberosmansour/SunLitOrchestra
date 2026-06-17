//! M4 structural-contract test (outcome-first runbook).
//!
//! The three back-end consumers enforce the outcome contract:
//! - `slo-retro` refuses to close a value-bearing milestone with an unproven
//!   outcome/journey/regression row, and gains a `## Outcome vs promise` section.
//! - `slo-execute` writes Outcome Scenario + Critical User Journey tests first.
//! - `slo-critique` flags vacuous/mock-only outcome scenarios as `ask`, while
//!   preserving `## Rotation order` + the four persona anchors.
//!
//! SHA discipline (founder direction): `slo-retro` + `slo-execute` get fresh
//! pins here. `slo-critique` has a SINGLE source-of-truth constant
//! (`CRITIQUE_SKILL_SHA256` in `sap_imp_m5_agents.rs`); `slo_tm_m2_consumers.rs`
//! DERIVES it by regex (no second constant), so M4 bumps ONLY that one constant
//! (revises the plan's ENG-4 "two-constant" premise — see DW-003). This test
//! cross-checks that the live `slo-critique` SHA matches the single constant and
//! that `slo_tm_m2` still derives from it (tm-outcome-first-abuse-4: only the
//! constant VALUE may change, the pin logic must not be weakened).

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

const RETRO_SKILL: &str = "skills/slo-retro/SKILL.md";
const EXECUTE_SKILL: &str = "skills/slo-execute/SKILL.md";
const CRITIQUE_SKILL: &str = "skills/slo-critique/SKILL.md";
const SAP_IMP_M5: &str = "xtasks/sast-verify/tests/sap_imp_m5_agents.rs";
const SLO_TM_M2: &str = "xtasks/sast-verify/tests/slo_tm_m2_consumers.rs";

/// SHA-256 of `skills/slo-retro/SKILL.md` after the M4 outcome-gate edit.
const RETRO_SKILL_SHA256: &str = "5c6362645a89189ca03c2ccb94bcc3412735e85bedc7ced7ecf6960a14d857b9";
/// SHA-256 of `skills/slo-execute/SKILL.md` after the M4 outcome-first edit.
const EXECUTE_SKILL_SHA256: &str =
    "b85d47907704efab2702a9293df37cdddc7417a9c0f94bd7d289116576ec4607";

#[test]
fn retro_refuses_unproven_outcome() {
    let lc = read(RETRO_SKILL).to_lowercase();
    assert!(
        lc.contains("outcome validation") && lc.contains("unproven"),
        "/slo-retro must refuse a value-bearing milestone with an unproven Outcome Validation row"
    );
}

#[test]
fn retro_has_outcome_vs_promise() {
    assert!(
        read(RETRO_SKILL).contains("## Outcome vs promise"),
        "/slo-retro lessons template must add a `## Outcome vs promise` section"
    );
}

#[test]
fn execute_writes_outcome_tests_first() {
    let c = read(EXECUTE_SKILL);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("outcome scenario")
            && lc.contains("critical user journey")
            && lc.contains("first"),
        "/slo-execute Step 1 must write Outcome Scenario + Critical User Journey tests first"
    );
    assert!(
        c.contains("cuj-<slug>-") || c.contains("oc-<slug>-"),
        "/slo-execute should reference the frozen oc-/cuj- id schemes"
    );
}

#[test]
fn critique_flags_outcome_theatre() {
    let lc = read(CRITIQUE_SKILL).to_lowercase();
    assert!(
        (lc.contains("vacuous") || lc.contains("mock-only") || lc.contains("theatre"))
            && lc.contains("outcome scenario"),
        "/slo-critique must flag vacuous/mock-only outcome scenarios as `ask`"
    );
}

#[test]
fn critique_anchors_preserved() {
    // Must not break sap_imp_m5 / slo_tm_m2_consumers anchor assertions.
    let c = read(CRITIQUE_SKILL);
    assert!(
        c.contains("## Rotation order"),
        "slo-critique must keep `## Rotation order`"
    );
    for persona in ["CEO", "Eng lead", "Security", "Design"] {
        assert!(
            c.contains(persona),
            "slo-critique must keep persona anchor `{persona}`"
        );
    }
}

#[test]
fn retro_execute_sha_pinned() {
    assert_eq!(
        sha256_of(RETRO_SKILL),
        RETRO_SKILL_SHA256,
        "slo-retro SKILL.md SHA drift"
    );
    assert_eq!(
        sha256_of(EXECUTE_SKILL),
        EXECUTE_SKILL_SHA256,
        "slo-execute SKILL.md SHA drift"
    );
}

#[test]
fn critique_single_source_of_truth_consistent() {
    // Reality discovered in M4 (revises the plan's ENG-4 premise): the slo-critique
    // SHA has exactly ONE constant — `CRITIQUE_SKILL_SHA256` in sap_imp_m5_agents.rs.
    // slo_tm_m2_consumers.rs DOES NOT store a second constant; its
    // `feng6_sha_constant_in_lockstep` DERIVES the value by regex over sap_imp_m5.
    // So there is no two-constant half-update risk. This test guards the real
    // invariant: the live SHA matches the single constant, and slo_tm_m2 keeps
    // deriving from it (so no divergent second baseline can be introduced) —
    // which is the tm-outcome-first-abuse-4 surface.
    let live = sha256_of(CRITIQUE_SKILL);
    assert!(
        read(SAP_IMP_M5).contains(&live),
        "sap_imp_m5_agents.rs CRITIQUE_SKILL_SHA256 ({live}) is stale — bump the single constant in the same milestone (tm-outcome-first-abuse-4)"
    );
    let tm2 = read(SLO_TM_M2);
    assert!(
        tm2.contains("sap_imp_m5_agents.rs") && tm2.contains("CRITIQUE_SKILL_SHA256"),
        "slo_tm_m2_consumers.rs must keep DERIVING the slo-critique SHA from sap_imp_m5's single constant (no divergent second baseline)"
    );
}
