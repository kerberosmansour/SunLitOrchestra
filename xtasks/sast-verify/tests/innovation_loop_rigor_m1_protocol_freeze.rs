//! Experiment-rigor M1: `/slo-precision` freezes the confirmatory protocol.
//!
//! These are structural contracts for the interactive Markdown skill pack. They
//! prove the protocol is represented consistently; they do not claim that a
//! live model followed it on a real experiment.

use std::path::{Path, PathBuf};

const PRECISION: &str = "skills/slo-precision/SKILL.md";
const TEMPLATE: &str = "docs/slo/templates/experiment-book-template_v_1.md";
const SPEC: &str = "docs/slo/design/innovation-loop-experiment-book-spec.md";
const INTERFACES: &str = "docs/slo/design/innovation-loop-interfaces.md";

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("sast-verify lives two levels below the workspace root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|error| panic!("failed to read {rel}: {error}"))
}

fn assert_all(rel_paths: &[&str], needles: &[&str]) {
    for rel in rel_paths {
        let body = read(rel).to_lowercase();
        for needle in needles {
            assert!(
                body.contains(&needle.to_lowercase()),
                "{rel} must contain `{needle}`"
            );
        }
    }
}

#[test]
fn protocol_freeze_fields_are_locked_across_contracts() {
    assert_all(
        &[PRECISION, TEMPLATE, SPEC, INTERFACES],
        &[
            "ProtocolFreeze",
            "protocol version",
            "frozen at",
            "hypothesis",
            "baseline",
            "candidate interventions",
            "benchmark arms",
            "split IDs",
            "primary metrics",
            "secondary metrics",
            "analysis plan",
            "scoring method",
            "repetition / stability rule",
            "accept rule",
            "kill rule",
            "resource budget",
            "risk envelope",
        ],
    );
}

#[test]
fn protocol_amendments_are_append_only_and_stale_validation() {
    assert_all(
        &[PRECISION, TEMPLATE, SPEC, INTERFACES],
        &[
            "ProtocolAmendment",
            "append-only",
            "old value",
            "new value",
            "reason",
            "impact",
            "stale",
            "rerun",
        ],
    );
}

#[test]
fn protocol_source_strings_are_literal_and_cannot_select_controls() {
    assert_all(
        &[PRECISION, TEMPLATE, SPEC],
        &["~~~text", "literal data", "never select", "control fields"],
    );
}

#[test]
fn incomplete_and_legacy_protocols_never_imply_confirmation() {
    assert_all(
        &[PRECISION, TEMPLATE, SPEC, INTERFACES],
        &["legacy", "degraded", "not confirmed"],
    );
    assert_all(
        &[PRECISION, TEMPLATE, SPEC],
        &["incomplete", "blocks validation"],
    );
}

#[test]
fn confirmatory_plan_is_finite_not_try_until_good() {
    assert_all(
        &[PRECISION, TEMPLATE, SPEC],
        &["sample budget", "finite", "run until good"],
    );
}

#[test]
fn experiment_book_v1_section_order_is_unchanged() {
    let template = read(TEMPLATE);
    let headings = [
        "## 0. Experiment Metadata",
        "## 1. Experiment Tracker",
        "## 2. Global Experiment Rules",
        "## 3. Sandbox Charter",
        "## 4. Play Log",
        "## 5. Pattern Catalog",
        "## 6. Precision Model",
        "## 7. Spike Cards and Evidence",
        "## 8. Curation Decision",
        "## 9. Demo Pack",
        "## 10. Handoff Contract",
        "## 11. Compost / Lessons",
    ];
    let mut previous = 0;
    for heading in headings {
        let position = template
            .find(heading)
            .unwrap_or_else(|| panic!("missing `{heading}`"));
        assert!(position >= previous, "`{heading}` is out of order");
        previous = position;
    }
}
