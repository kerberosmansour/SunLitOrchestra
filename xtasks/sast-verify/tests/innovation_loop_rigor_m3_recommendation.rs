//! M3 structural contract: evidence strength gates routes and handoff completeness.

use std::path::{Path, PathBuf};

const CURATE: &str = "skills/slo-curate/SKILL.md";
const DEMO: &str = "skills/slo-demo/SKILL.md";
const TEMPLATE: &str = "docs/slo/templates/experiment-book-template_v_1.md";
const SPEC: &str = "docs/slo/design/innovation-loop-experiment-book-spec.md";
const INTERFACES: &str = "docs/slo/design/innovation-loop-interfaces.md";
const CONTRACTS: &[&str] = &[CURATE, DEMO, TEMPLATE, SPEC, INTERFACES];

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

fn assert_all_contain(needles: &[&str], contract: &str) {
    for rel in CONTRACTS {
        let body = read(rel).to_lowercase();
        for needle in needles {
            assert!(
                body.contains(&needle.to_lowercase()),
                "{rel} must define {contract} sentinel `{needle}`"
            );
        }
    }
}

#[test]
fn confidence_vocabulary_is_frozen_across_contracts() {
    assert_all_contain(
        &["exploratory | confirmatory | engineering_ready"],
        "the confidence vocabulary",
    );
}

#[test]
fn engineering_routes_require_evidence_not_a_label() {
    assert_all_contain(
        &[
            "promote_to_ticket",
            "promote_to_runbook",
            "current Validation Record",
            "engineering_ready",
            "ablation",
            "cannot self-upgrade",
        ],
        "the engineering route gate",
    );
}

#[test]
fn exploratory_routes_disclose_confirmation_gaps() {
    assert_all_contain(
        &[
            "promote_to_idea",
            "promote_to_research",
            "may be exploratory",
            "confirmation gaps",
        ],
        "the exploratory route policy",
    );
}

#[test]
fn recommendation_packet_is_method_complete() {
    assert_all_contain(
        &[
            "RecommendationPacket",
            "protocol version",
            "baseline",
            "candidate interventions",
            "benchmark arms",
            "split ids",
            "primary metrics",
            "secondary metrics",
            "ablation summary",
            "failure taxonomy",
            "replication instructions",
            "exact commands",
            "environment",
            "limitations",
            "uncertainty",
            "confidence",
            "exact engineering question",
            "evidence pointers",
        ],
        "the recommendation packet field set",
    );
}

#[test]
fn ablations_and_residual_failures_are_structured() {
    assert_all_contain(
        &[
            "Ablation Matrix",
            "removed or replaced",
            "actual delta",
            "Failure Taxonomy",
            "failure family",
            "residual impact",
        ],
        "ablation and failure analysis",
    );
}

#[test]
fn evidence_excerpts_are_literal_not_route_authority() {
    assert_all_contain(
        &[
            "~~~text",
            "literal data",
            "never select",
            "disposition",
            "confidence",
            "route",
        ],
        "the evidence/control boundary",
    );
}

#[test]
fn legacy_promotion_packet_is_a_compatible_subset() {
    assert_all_contain(
        &[
            "PromotionPacket",
            "compatible subset",
            "downgrade",
            "blocks engineering routes",
        ],
        "legacy handoff compatibility",
    );
}

#[test]
fn frozen_routes_and_human_control_remain() {
    let curate = read(CURATE).to_lowercase();
    assert!(curate.contains("exactly one disposition"));
    for route in [
        "promote_to_idea",
        "promote_to_ticket",
        "promote_to_research",
        "promote_to_runbook",
        "needs_more_play",
        "blocked_by_unknown",
        "killed_but_reusable",
        "archive_no_action",
    ] {
        assert!(
            curate.contains(route),
            "curate must preserve route `{route}`"
        );
    }

    let demo = read(DEMO).to_lowercase();
    assert!(demo.contains("suggestion"));
    assert!(demo.contains("never auto-invoke"));
}
