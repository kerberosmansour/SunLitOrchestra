//! M4 end-to-end contract: honest synthetic dogfood, threats, and public docs.

use serde::Deserialize;
use std::path::{Path, PathBuf};

const GALLERY: &str = "docs/slo/experiments/example-context-validator/EXPERIMENT.md";
const THREAT_MD: &str = "docs/slo/design/innovation-loop-threat-model.md";
const THREAT_JSON: &str = "docs/slo/design/innovation-loop-threat-model.slo.json";
const README: &str = "README.md";
const ARCH: &str = "docs/ARCHITECTURE.md";
const LOOPS: &str = "docs/LOOPS-ENGINEERING.md";
const CATALOG: &str = "docs/skill-pack-catalog.md";
const SECURITY: &str = "SECURITY.md";

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {rel}: {e}"))
}

fn assert_contains_all(rel: &str, needles: &[&str]) {
    let body = read(rel).to_lowercase();
    for needle in needles {
        assert!(
            body.contains(&needle.to_lowercase()),
            "{rel} must contain `{needle}`"
        );
    }
}

#[derive(Deserialize)]
struct ThreatModel {
    provenance: Provenance,
    abuse_cases: Vec<AbuseCase>,
}

#[derive(Deserialize)]
struct Provenance {
    produced_by: String,
    producer_skill_sha: String,
    generated_at: String,
    inputs: Vec<ProvenanceInput>,
}

#[derive(Deserialize)]
struct ProvenanceInput {
    path: String,
    sha: String,
}

#[derive(Deserialize)]
struct AbuseCase {
    id: String,
    control: String,
    status: String,
}

#[test]
fn gallery_walks_the_complete_rigorous_path() {
    assert_contains_all(
        GALLERY,
        &[
            "Protocol Freeze",
            "PF-1",
            "Discovery Record",
            "DiscoveryRecord",
            "Validation Record",
            "ValidationRecord",
            "active protocol version",
            "held-out",
            "no tuning",
            "per-arm results",
            "Ablation Matrix",
            "Failure Taxonomy",
            "Replication instructions",
            "Exact commands",
            "Environment",
            "Limitations",
            "Uncertainty",
            "Confidence",
            "confirmatory",
            "RecommendationPacket",
            "promote_to_idea",
        ],
    );
}

#[test]
fn gallery_does_not_overstate_synthetic_evidence() {
    assert_contains_all(
        GALLERY,
        &[
            "synthetic fixture evidence",
            "not real-world validation",
            "not independently replicated",
            "generalization gap",
            "suggestion",
            "never auto-invoke",
        ],
    );
}

#[test]
fn threat_ids_are_contiguous_and_provenance_is_current() {
    let raw = read(THREAT_JSON);
    let threat: ThreatModel = serde_json::from_str(&raw).expect("threat JSON must parse");
    let ids: Vec<String> = threat
        .abuse_cases
        .iter()
        .map(|case| case.id.clone())
        .collect();
    let expected: Vec<String> = (1..=8)
        .map(|n| format!("tm-innovation-loop-abuse-{n}"))
        .collect();
    assert_eq!(ids, expected, "threat IDs must remain contiguous 1..8");
    assert!(threat
        .abuse_cases
        .iter()
        .all(|case| case.status == "active"));

    let md = read(THREAT_MD);
    for id in &expected {
        assert!(md.contains(id), "Markdown threat model missing `{id}`");
    }

    assert_eq!(threat.provenance.produced_by, "slo-execute");
    assert_eq!(threat.provenance.generated_at, "2026-07-13");
    assert!(threat.provenance.producer_skill_sha.len() >= 7);
    assert!(
        threat.provenance.inputs.iter().any(|input| {
            input.path.ends_with("RUNBOOK-EXPERIMENT-RIGOR.md") && input.sha.len() >= 7
        }),
        "provenance must name the runbook that produced rows 7–8"
    );
}

#[test]
fn threat_controls_cover_mutation_and_evidence_leakage() {
    assert_contains_all(
        THREAT_MD,
        &[
            "tm-innovation-loop-abuse-7",
            "post-result protocol mutation",
            "ProtocolAmendment",
            "stale",
            "rerun",
            "tm-innovation-loop-abuse-8",
            "discovery/validation evidence leakage",
            "held-out",
            "no tuning",
            "downgrade confidence",
        ],
    );

    let threat: ThreatModel =
        serde_json::from_str(&read(THREAT_JSON)).expect("threat JSON must parse");
    let controls = threat
        .abuse_cases
        .iter()
        .skip(6)
        .map(|case| case.control.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    for needle in [
        "protocolamendment",
        "stale",
        "rerun",
        "held-out",
        "no tuning",
        "downgrade confidence",
    ] {
        assert!(controls.contains(needle), "new controls missing `{needle}`");
    }
}

#[test]
fn public_docs_describe_the_same_rigorous_loop() {
    for rel in [README, ARCH, LOOPS] {
        assert_contains_all(
            rel,
            &[
                "Innovation Sandbox",
                "Protocol Freeze",
                "Validation Record",
                "RecommendationPacket",
            ],
        );
    }
    assert_contains_all(
        CATALOG,
        &[
            "/slo-precision",
            "Protocol Freeze",
            "/slo-spike",
            "DiscoveryRecord",
            "ValidationRecord",
            "/slo-curate",
            "engineering_ready",
            "/slo-demo",
            "RecommendationPacket",
        ],
    );
    assert_contains_all(
        SECURITY,
        &[
            "Protocol integrity",
            "Evidence separation",
            "tm-innovation-loop-abuse-7",
            "tm-innovation-loop-abuse-8",
        ],
    );
}

#[test]
fn loop_documentation_has_one_canonical_section() {
    let count = read(LOOPS)
        .lines()
        .filter(|line| *line == "## Innovation Sandbox loop")
        .count();
    assert_eq!(
        count, 1,
        "LOOPS must have one canonical Innovation Sandbox section"
    );
}

#[test]
fn frozen_book_spine_and_route_remain_human_controlled() {
    let gallery = read(GALLERY);
    let mut previous = 0;
    for section in 0..=11 {
        let marker = format!("## {section}.");
        let pos = gallery
            .find(&marker)
            .unwrap_or_else(|| panic!("gallery missing section marker `{marker}`"));
        assert!(
            pos >= previous,
            "gallery section `{marker}` is out of order"
        );
        previous = pos;
    }
    assert!(gallery.contains("promote_to_idea"));
    assert!(gallery.to_lowercase().contains("never auto-invoke"));
}
