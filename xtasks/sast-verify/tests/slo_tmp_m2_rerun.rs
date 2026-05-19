//! M2 structural-contract test (slo-threat-model-producer runbook).
//!
//! Asserts the `.slo.json` re-run / supersession algorithm and a live
//! `status: superseded` row in a dedicated demo fixture (closes the wedge
//! retro's coverage gap):
//!
//! - The re-run algorithm is documented in `/slo-architect` Step 3.5
//!   (per-file binding per M1 lesson): detect existing `.slo.json`, diff,
//!   supersede-don't-renumber, surface the diff + overwrite/merge/skip,
//!   never silent clobber.
//! - ENG-2 (locked by the M2 contract): this test OWNS the demo-fixture
//!   strict-parse — `slo_tm_m1_schema` only loads its hardcoded original
//!   fixture and will not read the demo. The structs below are private to
//!   this test (mirror `slo_tm_m1_schema.rs` shape, `deny_unknown_fields`).
//! - Demo fixture invariants: ≥1 `status: superseded` row with non-empty
//!   `superseded_by` + `supersede_reason`; ids contiguous from 1.

use regex::Regex;
use serde::Deserialize;
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
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

const ARCHITECT_SKILL: &str = "skills/slo-architect/SKILL.md";
const DEMO_FIXTURE: &str = "docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json";

const ALLOWED_CLASSIFICATIONS: &[&str] = &["public", "internal", "confidential", "restricted"];
const ALLOWED_ABUSE_STATUS: &[&str] = &["active", "superseded"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ThreatModel {
    slo_schema_version: String,
    #[allow(dead_code)]
    slug: String,
    sensitivity: String,
    #[allow(dead_code)]
    otm_compatible: bool,
    provenance: Provenance,
    #[allow(dead_code)]
    stride: Vec<StrideCell>,
    abuse_cases: Vec<AbuseCase>,
    #[allow(dead_code)]
    residual_risks: Vec<ResidualRisk>,
    #[allow(dead_code)]
    compliance: Vec<ComplianceRow>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    #[allow(dead_code)]
    produced_by: String,
    producer_skill_sha: String,
    #[allow(dead_code)]
    generated_at: String,
    inputs: Vec<ProvInput>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvInput {
    #[allow(dead_code)]
    path: String,
    sha: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct StrideCell {
    component: String,
    class: String,
    state: String,
    control_or_reason: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AbuseCase {
    id: String,
    #[allow(dead_code)]
    surface: String,
    #[allow(dead_code)]
    attacker: String,
    #[allow(dead_code)]
    attack_step: String,
    #[allow(dead_code)]
    attacker_outcome: String,
    #[allow(dead_code)]
    control: String,
    status: String,
    superseded_by: Option<String>,
    supersede_reason: Option<String>,
    classification: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct ResidualRisk {
    risk: String,
    exploit_path: String,
    compensating_control: String,
    accepted_residual: bool,
    owner: String,
    review_by: String,
    classification: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct ComplianceRow {
    control: String,
    soc2: String,
    asvs: String,
}

fn load_demo() -> ThreatModel {
    let raw = read(DEMO_FIXTURE);
    serde_json::from_str::<ThreatModel>(&raw)
        .unwrap_or_else(|e| panic!("demo fixture {DEMO_FIXTURE} failed strict parse: {e}"))
}

/// BDD `rerun_algorithm_documented` — per-file (M1 lesson). The re-run
/// algorithm is documented in Step 3.5 of slo-architect/SKILL.md.
#[test]
fn rerun_algorithm_documented_in_step35() {
    let skill = read(ARCHITECT_SKILL);
    let lc = skill.to_lowercase();
    // The mirror of item 7's Markdown re-run wording, applied to .slo.json.
    for required in [
        "supersede",
        "renumber",
        "overwrite",
        "merge",
        "skip",
        "no silent clobber",
        "diff",
    ] {
        assert!(
            lc.contains(&required.to_lowercase()),
            "Step 3.5 must document the .slo.json re-run algorithm phrase \
             {required:?} (mirroring the Markdown item 7 wording)"
        );
    }
    // Bind to the .slo.json target so the prose isn't satisfied by the
    // pre-existing Markdown item 7 alone (per-file + per-target).
    assert!(
        skill.contains("-threat-model.slo.json"),
        "the re-run algorithm prose must explicitly target the .slo.json artifact"
    );
}

/// BDD `live_superseded_row_validates` (parse half) + ENG-2 (this test
/// owns the demo-fixture strict-parse).
#[test]
fn demo_fixture_strict_parses_with_own_structs() {
    let tm = load_demo();
    assert_eq!(
        tm.slo_schema_version, "0.1.0",
        "demo fixture must use the merged frozen schema version"
    );
    assert!(
        ALLOWED_CLASSIFICATIONS.contains(&tm.sensitivity.as_str()),
        "top-level sensitivity {:?} not in allowed enum",
        tm.sensitivity
    );
    assert!(
        tm.provenance.producer_skill_sha.len() >= 7,
        "provenance.producer_skill_sha must be a real git sha"
    );
    assert!(
        !tm.provenance.inputs.is_empty() && tm.provenance.inputs.iter().all(|i| i.sha.len() >= 7),
        "provenance.inputs must be non-empty + each carry a sha"
    );
    for ac in &tm.abuse_cases {
        assert!(
            ALLOWED_CLASSIFICATIONS.contains(&ac.classification.as_str()),
            "abuse {} classification {:?} not in allowed enum",
            ac.id,
            ac.classification
        );
        assert!(
            ALLOWED_ABUSE_STATUS.contains(&ac.status.as_str()),
            "abuse {} status {:?} not in allowed enum",
            ac.id,
            ac.status
        );
    }
}

/// BDD `live_superseded_row_validates` (superseded-row half) +
/// `schema_invalid_superseded_rejected`.
#[test]
fn demo_fixture_has_live_superseded_row() {
    let tm = load_demo();
    let superseded: Vec<&AbuseCase> = tm
        .abuse_cases
        .iter()
        .filter(|a| a.status == "superseded")
        .collect();
    assert!(
        !superseded.is_empty(),
        "the demo fixture must carry ≥1 status:superseded abuse case row \
         (this is the whole point of M2 — close the wedge-retro coverage gap)"
    );
    for ac in &superseded {
        assert!(
            ac.superseded_by.as_deref().is_some_and(|s| !s.is_empty()),
            "superseded {} must carry non-empty superseded_by (supersede-don't-renumber)",
            ac.id
        );
        assert!(
            ac.supersede_reason
                .as_deref()
                .is_some_and(|s| !s.is_empty()),
            "superseded {} must carry non-empty supersede_reason",
            ac.id
        );
        // The replacement must be a real id present in the fixture.
        let replacement = ac.superseded_by.as_ref().unwrap();
        assert!(
            tm.abuse_cases.iter().any(|other| &other.id == replacement),
            "superseded {} points to {replacement}, which is not present in abuse_cases",
            ac.id
        );
    }
}

/// BDD `renumber_on_rerun_fails` + `silent_drop_of_superseded_fails` —
/// frozen-ID contiguity for the demo fixture.
#[test]
fn demo_fixture_ids_match_demo_slug_and_are_contiguous() {
    let tm = load_demo();
    let re = Regex::new(r"^tm-slo-threat-model-producer-demo-abuse-(\d+)$").unwrap();
    let mut nums: Vec<u32> = tm
        .abuse_cases
        .iter()
        .map(|ac| {
            let c = re
                .captures(&ac.id)
                .unwrap_or_else(|| panic!("demo id {:?} drifted from demo slug pattern", ac.id));
            c[1].parse().expect("trailing id segment is an integer")
        })
        .collect();
    let original = nums.clone();
    nums.sort_unstable();
    nums.dedup();
    assert_eq!(
        nums.len(),
        original.len(),
        "demo fixture ids must be unique (no reuse of a frozen id)"
    );
    let max = *nums.iter().max().expect("non-empty");
    assert_eq!(
        nums,
        (1..=max).collect::<Vec<_>>(),
        "demo fixture ids must be contiguous from 1 \
         (a gap means a superseded row was silently dropped — keep it with status=superseded)"
    );
}
