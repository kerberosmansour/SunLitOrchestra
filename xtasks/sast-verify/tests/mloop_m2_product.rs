//! M2 structural-contract test (measurement-loop runbook).
//!
//! Asserts the `/slo-product metrics` feature-measurement-spec contract + the
//! single authorized biz-schema key addition:
//!
//! - `mode_arg: metrics` gains a `Feature measurement specification` section
//!   (primary leading + lagging metric, guardrails, telemetry requirements,
//!   experiment backlog) and instructs setting `feature_measurement_spec: true`.
//! - The telemetry-requirements sub-block mandates pseudonymised identifiers
//!   (C8 / data minimisation).
//! - `references/biz/artifact-schema.md` registers exactly one new optional
//!   `feature_measurement_spec: bool` key (default-absent = false), and NO key
//!   cluster crept in (forbidden-key denylist absent).
//! - The PM/financial split is preserved: the CAC/LTV/NDR -> /slo-metrics
//!   cross-reference is intact.
//! - The `tier` enum stays exactly `confidential | public` (no enum drift).

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

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

fn product_md() -> String {
    read(&workspace_root().join("skills/slo-product/SKILL.md"))
}

fn artifact_schema_md() -> String {
    read(&workspace_root().join("references/biz/artifact-schema.md"))
}

#[test]
fn slo_product_feature_spec_section_present() {
    let content = product_md();
    assert!(
        content.contains("Feature measurement specification"),
        "`mode_arg: metrics` must gain a `Feature measurement specification` section"
    );
    for sentinel in [
        "Primary leading metric",
        "Primary lagging metric",
        "Telemetry requirements",
        "Experiment backlog",
    ] {
        assert!(
            content.contains(sentinel),
            "feature measurement spec must include `{sentinel}`"
        );
    }
    assert!(
        content.contains("feature_measurement_spec: true"),
        "SKILL.md must instruct setting `feature_measurement_spec: true` when the spec is present"
    );
}

#[test]
fn slo_product_telemetry_requires_pseudonymisation() {
    let content = product_md().to_lowercase();
    assert!(
        content.contains("pseudonym"),
        "the telemetry-requirements sub-block must mandate pseudonymised identifiers (C8 / minimisation)"
    );
}

#[test]
fn slo_product_measurement_key_registered() {
    let schema = artifact_schema_md();
    assert!(
        schema.contains("feature_measurement_spec"),
        "artifact-schema.md must register the `feature_measurement_spec` key"
    );
    let lc = schema.to_lowercase();
    assert!(
        lc.contains("bool"),
        "`feature_measurement_spec` must be typed `bool`"
    );
    assert!(
        lc.contains("absent") && lc.contains("false"),
        "`feature_measurement_spec` must document default-absent = false (backward compatible)"
    );
}

#[test]
fn slo_product_split_preserved() {
    let content = product_md();
    // CAC/LTV/NDR remain enumerated as belonging to /slo-metrics (cross-ref),
    // and the structural redirect is intact.
    for sentinel in ["CAC", "LTV", "NDR", "/slo-metrics"] {
        assert!(
            content.contains(sentinel),
            "PM/financial split must be preserved: `{sentinel}` cross-reference intact"
        );
    }
}

#[test]
fn artifact_schema_tier_enum_unchanged() {
    let schema = artifact_schema_md();
    assert!(
        schema.contains("`confidential` | `public`"),
        "the `tier` enum must stay exactly `confidential | public` (no enum drift)"
    );
}

#[test]
fn slo_product_single_key_bound() {
    // Exactly one new key; no telemetry-key cluster crept in.
    let schema = artifact_schema_md();
    assert!(schema.contains("feature_measurement_spec"));
    for forbidden in ["telemetry_schema", "event_names", "measurement_spec_json"] {
        assert!(
            !schema.contains(forbidden),
            "no key cluster: `{forbidden}` must NOT be added (single-key bound)"
        );
    }
}
