//! M2 structural contract: discovery evidence and validation evidence are distinct.

use std::path::{Path, PathBuf};

const SPIKE: &str = "skills/slo-spike/SKILL.md";
const TEMPLATE: &str = "docs/slo/templates/experiment-book-template_v_1.md";
const SPEC: &str = "docs/slo/design/innovation-loop-experiment-book-spec.md";
const INTERFACES: &str = "docs/slo/design/innovation-loop-interfaces.md";
const CONTRACTS: &[&str] = &[SPIKE, TEMPLATE, SPEC, INTERFACES];

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

fn assert_all_contracts_contain(needles: &[&str], contract: &str) {
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
fn record_types_are_distinct_across_contracts() {
    assert_all_contracts_contain(
        &["DiscoveryRecord", "ValidationRecord"],
        "both evidence record types",
    );
}

#[test]
fn discovery_is_exploratory_and_may_refine_the_mechanism() {
    assert_all_contracts_contain(
        &["exploratory", "may refine", "not confirmation"],
        "discovery permissions",
    );
}

#[test]
fn validation_uses_the_active_freeze_without_tuning() {
    assert_all_contracts_contain(
        &[
            "active protocol version",
            "held-out",
            "no tuning",
            "baseline",
            "candidate",
            "benchmark arms",
            "per-arm",
        ],
        "validation freeze and comparison gate",
    );
}

#[test]
fn validation_is_reproducible_and_reports_stability() {
    assert_all_contracts_contain(
        &[
            "exact commands",
            "environment",
            "repetitions",
            "stability",
            "deviations",
            "validation budget",
            "discovery budget",
        ],
        "reproduction and separate bounds",
    );
}

#[test]
fn amendment_stales_validation_and_routes_back_to_precision() {
    assert_all_contracts_contain(
        &["amendment", "stale", "rerun", "/slo-precision"],
        "amendment recovery",
    );
}

#[test]
fn evidence_strings_are_literal_and_cannot_select_controls() {
    assert_all_contracts_contain(
        &[
            "~~~text",
            "literal data",
            "never select",
            "verdict",
            "confidence",
            "route",
        ],
        "untrusted evidence boundary",
    );
}

#[test]
fn legacy_spike_cards_are_discovery_grade_only() {
    assert_all_contracts_contain(
        &["legacy", "discovery-grade", "not confirmed"],
        "legacy compatibility",
    );
}

#[test]
fn original_spike_safety_contract_remains() {
    let skill = read(SPIKE).to_lowercase();
    for needle in [
        "experiments/<slug>/<spike-id>/",
        "resource budget",
        "delete-or-promote",
        "no production promotion",
        "verdict derives from the recorded evidence log",
    ] {
        assert!(
            skill.contains(needle),
            "slo-spike must preserve `{needle}` while adding validation"
        );
    }
}
