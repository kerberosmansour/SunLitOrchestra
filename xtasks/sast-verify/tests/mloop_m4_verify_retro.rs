//! M4 structural-contract test (measurement-loop runbook).
//!
//! Asserts the `/slo-verify` measurement pass + `/slo-retro` Results-vs-thesis
//! section, AND drives the catch->remediate->green failure-bar demonstration
//! over a committed synthetic-PII fixture pair.
//!
//! Failure bar (non-vacuous): the mechanized subset of the measurement pass
//! (event presence, telemetry PII/masking, feature_measurement_spec flag<->
//! section cross-check, unfenced template-injection) must FAIL on `bad.md`
//! (catching tm-measurement-loop-abuse-1/2/3 + the missing-event defect) and
//! PASS on `remediated.md`. Both asserted in one test (vacuity guard).

use regex::Regex;
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

fn verify_md() -> String {
    read("skills/slo-verify/SKILL.md")
}
fn retro_md() -> String {
    read("skills/slo-retro/SKILL.md")
}
fn fixture(name: &str) -> String {
    read(&format!(
        "xtasks/sast-verify/tests/fixtures/mloop_failure_bar/{name}"
    ))
}

// --- mechanized subset of the measurement pass (the checkable checks) -------

/// Text outside any `~~~ ... ~~~` fence (odd split segments are inside a fence).
fn outside_fences(text: &str) -> String {
    text.split("~~~")
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, s)| s)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns the names of the checks that FAIL on this artifact text.
fn run_measurement_checks(text: &str) -> Vec<&'static str> {
    let mut failures = Vec::new();

    // (b) telemetry PII / masking — same email regex as Pass 4.
    let email = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap();
    if email.is_match(text) {
        failures.push("telemetry PII / masking");
    }

    // (e) feature_measurement_spec flag<->section cross-check.
    let flag_true = text.contains("feature_measurement_spec: true");
    let has_spec_section = text.contains("Feature measurement specification");
    if flag_true && !has_spec_section {
        failures.push("flag<->section cross-check");
    }

    // (f) unfenced user-string / template-injection.
    let unfenced = outside_fences(text).to_lowercase();
    if unfenced.contains("system:") || unfenced.contains("ignore all prior") {
        failures.push("template-injection");
    }

    // (a) event presence — at least one named behavioural event token.
    let event = Regex::new(
        r"\b[a-z][a-z_]*_(started|loaded|viewed|selected|confirmed|failed|cancelled|contacted|clicked|impression|event)\b",
    )
    .unwrap();
    if !event.is_match(text) {
        failures.push("event presence");
    }

    failures
}

// --- failure-bar demonstration ---------------------------------------------

#[test]
fn failure_bar_bad_fixture_is_caught() {
    let failures = run_measurement_checks(&fixture("bad.md"));
    for expected in [
        "telemetry PII / masking",
        "flag<->section cross-check",
        "template-injection",
        "event presence",
    ] {
        assert!(
            failures.contains(&expected),
            "bad.md must be caught by the `{expected}` check; failures were {failures:?}"
        );
    }
}

#[test]
fn failure_bar_remediated_fixture_is_green() {
    let failures = run_measurement_checks(&fixture("remediated.md"));
    assert!(
        failures.is_empty(),
        "remediated.md must pass every mechanized check; still failing: {failures:?}"
    );
}

#[test]
fn failure_bar_is_non_vacuous() {
    // The guardrail is real only if the bad fixture fails AND the remediated
    // fixture passes — asserted together.
    assert!(
        !run_measurement_checks(&fixture("bad.md")).is_empty(),
        "vacuity guard: bad.md must fail at least one check"
    );
    assert!(
        run_measurement_checks(&fixture("remediated.md")).is_empty(),
        "vacuity guard: remediated.md must pass all checks"
    );
}

#[test]
fn failure_bar_fixtures_marked_synthetic() {
    for name in ["bad.md", "remediated.md"] {
        assert!(
            fixture(name).contains("SYNTHETIC PII"),
            "{name} must carry a `SYNTHETIC PII` header (SEC-2)"
        );
    }
}

// --- pass / retro prose contract -------------------------------------------

const SIX_CHECKS: &[&str] = &[
    "event presence",
    "telemetry PII / masking",
    "failure-path emission",
    "replay tagging",
    "flag↔section cross-check",
    "template-injection",
];

#[test]
fn slo_verify_measurement_pass_six_checks_present() {
    let v = verify_md();
    for check in SIX_CHECKS {
        assert!(
            v.contains(check),
            "/slo-verify measurement pass must document the `{check}` check"
        );
    }
}

#[test]
fn slo_verify_prose_mechanical_lockstep() {
    // ENG-2: the prose must declare the mechanized subset as hard signals
    // demonstrated by the failure-bar fixture, so prose and test cannot drift.
    let v = verify_md().to_lowercase();
    assert!(
        v.contains("mechanically-demonstrated") && v.contains("failure-bar"),
        "/slo-verify must tie the hard checks to the mechanically-demonstrated failure-bar fixture (ENG-2 lockstep)"
    );
}

#[test]
fn slo_verify_measurement_pass_tool_optional() {
    let v = verify_md().to_lowercase();
    assert!(
        v.contains("skipped") && v.contains("telemetry"),
        "the measurement pass must be tool-optional: absent telemetry context emits a `skipped` row, not a hard fail"
    );
}

#[test]
fn slo_retro_results_vs_thesis_present() {
    assert!(
        retro_md().contains("## Results vs thesis"),
        "/slo-retro lessons template must add a `## Results vs thesis` section"
    );
}

#[test]
fn slo_verify_passes_not_renumbered() {
    let v = verify_md();
    for heading in [
        "### Pass 1. Happy path",
        "### Pass 4. Security",
        "### Pass 5. AI tolerance",
    ] {
        assert!(
            v.contains(heading),
            "existing /slo-verify pass heading `{heading}` must be preserved (measurement pass is additive)"
        );
    }
}
