//! M1 structural-contract test (outcome-first runbook).
//!
//! Asserts the v4 template's Outcome First Engineering contract surface:
//!
//! - §5C Outcome Validation Contract, inserted AFTER §5B and BEFORE §6 (no renumber).
//! - §17 sub-sections: Outcome Scenarios (`oc-<slug>-N`), Critical User Journeys
//!   (`cuj-<slug>-N`), Core Capability Regression Matrix (resolution never blank).
//! - §11 Outcome test layer ("Outcome Tests (highest authority)") + §6.12
//!   "Outcome outranks unit" rule (the authority inversion).
//! - Theme B (ENG-2 + SEC-2): §5C Front-to-End steps are PER-LAYER
//!   `applicable | not_applicable(reason)` with >=1 real cross-layer assertion.
//! - tm-outcome-first-abuse-1: the `~~~text` fence + no-control-field rule.
//! - Both template copies carry §5C; the v3 template does NOT (back-compat).
//!
//! Byte-identity of the two v4 copies is enforced by the existing
//! `svl_m1` / `svl_m3` / `mloop_m3_plan` template-sync tests; this file adds the
//! belt-and-braces "both copies contain §5C" check on top.

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
const TEMPLATE_V3: &str = "docs/slo/templates/runbook-template_v_3_template.md";

const SECTION_ANCHORS: &[&str] = &[
    "## 5C. Outcome Validation Contract",
    "#### Outcome Scenarios",
    "#### Critical User Journeys",
    "#### Core Capability Regression Matrix",
    "Outcome Tests (highest authority)",
    "### 6.12 Outcome outranks unit",
];

#[test]
fn template_has_outcome_first_sections() {
    let c = read(TEMPLATE_PRIMARY);
    for anchor in SECTION_ANCHORS {
        assert!(
            c.contains(anchor),
            "skill-primary v4 template must contain `{anchor}`"
        );
    }
}

#[test]
fn both_template_copies_have_5c() {
    assert!(
        read(TEMPLATE_PRIMARY).contains("## 5C. Outcome Validation Contract"),
        "skill-primary copy must carry §5C"
    );
    assert!(
        read(TEMPLATE_MIRROR).contains("## 5C. Outcome Validation Contract"),
        "repo-mirror copy must carry §5C"
    );
}

#[test]
fn template_5c_after_5b_no_renumber() {
    let c = read(TEMPLATE_PRIMARY);
    for h in [
        "## 5A.",
        "## 5B.",
        "## 6. Global Execution Rules",
        "## 11.",
        "## 17. Milestone Plan",
    ] {
        assert!(
            c.contains(h),
            "existing heading `{h}` must be preserved (no renumber)"
        );
    }
    let i5b = c.find("## 5B.").expect("§5B present");
    let i5c = c
        .find("## 5C. Outcome Validation Contract")
        .expect("§5C present");
    let i6 = c.find("## 6. Global Execution Rules").expect("§6 present");
    assert!(
        i5b < i5c && i5c < i6,
        "§5C must be inserted AFTER §5B and BEFORE §6 (insertion, not renumber)"
    );
}

#[test]
fn template_regression_matrix_resolution_never_blank() {
    let c = read(TEMPLATE_PRIMARY);
    assert!(
        c.contains("Core Capability Regression Matrix"),
        "§17 must add the Core Capability Regression Matrix sub-section"
    );
    assert!(
        c.to_lowercase().contains("never blank"),
        "the Regression Matrix resolution column must be documented as never blank \
         (pass | not_applicable | waived_with_reason)"
    );
}

#[test]
fn template_per_layer_front_to_end_with_cross_layer_assertion() {
    // Theme B (ENG-2 + SEC-2): no monolithic Front-to-End block.
    let lc = read(TEMPLATE_PRIMARY).to_lowercase();
    assert!(
        lc.contains("not_applicable(reason)")
            || (lc.contains("applicable") && lc.contains("per-layer")),
        "§5C Front-to-End steps must be PER-LAYER applicable | not_applicable(reason)"
    );
    assert!(
        lc.contains("cross-layer assertion"),
        "§5C must require >=1 real cross-layer assertion (a mock-only assertion is non-conformant)"
    );
}

#[test]
fn template_carries_authored_string_fence_rule() {
    // tm-outcome-first-abuse-1: authored §5C/§17 strings cannot inject.
    let c = read(TEMPLATE_PRIMARY);
    assert!(
        c.contains("~~~text"),
        "§5C must carry the ~~~text fence rule for authored strings"
    );
    let lc = c.to_lowercase();
    assert!(
        lc.contains("never selects control fields") || lc.contains("descriptive markdown only"),
        "authored outcome text must be descriptive-only and never select control fields"
    );
}

#[test]
fn template_defines_frozen_id_schemes() {
    let c = read(TEMPLATE_PRIMARY);
    assert!(
        c.contains("oc-<slug>-") && c.contains("cuj-<slug>-"),
        "template must define the frozen oc-<slug>-N and cuj-<slug>-N id schemes"
    );
}

#[test]
fn v3_template_untouched_no_5c() {
    let c = read(TEMPLATE_V3);
    assert!(
        !c.contains("Outcome Validation Contract"),
        "the v3 template must remain untouched (no §5C)"
    );
}
