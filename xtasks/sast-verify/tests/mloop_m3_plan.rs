//! M3 structural-contract test (measurement-loop runbook).
//!
//! Asserts the v4 template Measurement Contract section + Contract Block row +
//! the `/slo-plan` requirement (incl. the ENG-1 value-bearing definition):
//!
//! - BOTH v4 template copies (skill-primary `skills/slo-plan/references/...`
//!   and repo-mirror `docs/slo/templates/...`) carry an optional Measurement
//!   Contract section with all 10 documented fields + the optional/legacy
//!   framing, AND a `Measurement deliverables` Contract Block row.
//! - The two copies stay byte-identical (the documented primary/mirror sync).
//! - `/slo-plan` SKILL.md requires the Measurement Contract for value-bearing
//!   features, defines "value-bearing" deterministically, and flags the gap
//!   without invalidating legacy runbooks.
//! - Existing template sections §6 / §10 / §17 are not renumbered/removed.

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

const TEN_FIELDS: &[&str] = &[
    "Value hypothesis",
    "Review windows",
    "Primary leading metric",
    "Primary lagging metric",
    "Guardrails",
    "Telemetry deliverables",
    "Rollout plan",
    "Diagnosis plan",
    "Experiment plan",
    "Privacy controls",
];

fn assert_measurement_contract_in(label: &str, content: &str) {
    assert!(
        content.contains("Measurement Contract"),
        "{label}: must carry a Measurement Contract section"
    );
    for field in TEN_FIELDS {
        assert!(
            content.contains(field),
            "{label}: Measurement Contract must include the `{field}` field"
        );
    }
    assert!(
        content.contains("Measurement deliverables"),
        "{label}: §17 Contract Block must add a `Measurement deliverables` row"
    );
    // Optional / legacy-valid framing (mirrors §10 Carry-forward precedent).
    let lc = content.to_lowercase();
    assert!(
        lc.contains("optional") && lc.contains("legacy runbooks"),
        "{label}: Measurement Contract must be optional-by-shape (legacy runbooks remain valid)"
    );
}

#[test]
fn measurement_contract_in_both_template_copies() {
    assert_measurement_contract_in("repo mirror", &read(TEMPLATE_MIRROR));
    assert_measurement_contract_in("skill primary", &read(TEMPLATE_PRIMARY));
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
fn slo_plan_requires_measurement_contract() {
    let content = read("skills/slo-plan/SKILL.md");
    let lc = content.to_lowercase();
    assert!(
        content.contains("Measurement Contract") && lc.contains("value-bearing"),
        "/slo-plan must require a Measurement Contract for value-bearing features"
    );
    assert!(
        lc.contains("flag") && lc.contains("legacy"),
        "/slo-plan must flag a missing contract without invalidating legacy runbooks"
    );
}

#[test]
fn slo_plan_defines_value_bearing() {
    // ENG-1: the trigger must be deterministic.
    let lc = read("skills/slo-plan/SKILL.md").to_lowercase();
    assert!(
        lc.contains("user-facing")
            && lc.contains("excludes")
            && (lc.contains("docs-only") || lc.contains("refactor")),
        "/slo-plan must define value-bearing deterministically (user-facing capability; excludes refactor/docs-only/test-only) — ENG-1"
    );
}

#[test]
fn template_existing_sections_not_renumbered() {
    let t = read(TEMPLATE_MIRROR);
    for heading in [
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
fn template_section5_kani_subblock_preserved() {
    // Regression: the new section is inserted near §5 without disturbing the
    // Kani sub-block that kani_m3_integration.rs pins.
    let lc = read(TEMPLATE_MIRROR).to_lowercase();
    assert!(
        lc.contains("kani proof obligation"),
        "v4 template §5 Kani proof-obligation sub-block must survive the Measurement Contract insertion"
    );
}
