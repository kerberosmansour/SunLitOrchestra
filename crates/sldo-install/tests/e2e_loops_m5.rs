//! M5 structural-contract tests for the loops-and-lessons-closure runbook.
//!
//! These tests assert that `/slo-resume`'s SKILL.md has been extended to
//! read the optional "Carry-forward from prior retros" section, classify
//! the recommended next action with a lane (`micro | milestone | fresh-runbook`),
//! keep the orientation output compact (top-3 inline, remainder summarized),
//! and stay strictly read-only (no auto-starting the next skill).
//!
//! Backwards-compat: M4's runbook template change must remain valid;
//! `/slo-resume` continues to handle runbooks WITHOUT the carry-forward
//! section.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` Milestone 5.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

#[test]
fn slo_resume_reads_carry_forward_section() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    assert!(
        skill.contains("Carry-forward from prior retros"),
        "/slo-resume SKILL.md must reference 'Carry-forward from prior retros' as part of orientation"
    );
}

#[test]
fn slo_resume_lane_vocabulary_documented() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    for lane in &["micro", "milestone", "fresh-runbook"] {
        assert!(
            skill.contains(lane),
            "/slo-resume SKILL.md must document lane vocabulary: {lane}"
        );
    }
}

#[test]
fn slo_resume_output_stays_short() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    let lower = skill.to_lowercase();
    // The output rules must explicitly cap inline at top 3 and reference
    // a single-screen / short / compact discipline. M4's pre-flight uses
    // identical caps; M5 inherits.
    assert!(
        lower.contains("top 3") || lower.contains("top-3"),
        "/slo-resume SKILL.md must cap inline carry-forward at top 3"
    );
    assert!(
        lower.contains("one screen")
            || lower.contains("compact")
            || lower.contains("short message"),
        "/slo-resume SKILL.md must require compact / one-screen / short orientation output"
    );
}

#[test]
fn slo_resume_no_auto_start_preserved() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    let lower = skill.to_lowercase();
    // The read-only / no auto-start rule is the central invariant.
    assert!(
        lower.contains("do not start")
            || lower.contains("does not start")
            || lower.contains("no auto-start")
            || lower.contains("read-only")
            || lower.contains("not modify state"),
        "/slo-resume SKILL.md must preserve the read-only / no auto-start rule"
    );
}

#[test]
fn slo_resume_existing_tracker_first_behavior_preserved() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    // Existing tracker-only orientation must keep working when no
    // carry-forward section is present (backward compat).
    assert!(
        skill.contains("Milestone Tracker"),
        "/slo-resume SKILL.md must continue to read the Milestone Tracker first"
    );
}

#[test]
fn slo_resume_handles_empty_carry_forward() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    let lower = skill.to_lowercase();
    // Empty carry-forward must not crash orientation.
    assert!(
        lower.contains("empty") || lower.contains("no carry-forward") || lower.contains("no rows"),
        "/slo-resume SKILL.md must document empty-carry-forward / empty-section handling"
    );
}

#[test]
fn slo_resume_blocked_milestone_handled() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    // Blocked rows still print blocker; user decides — preserved invariant.
    assert!(
        skill.contains("blocked") || skill.contains("blocker"),
        "/slo-resume SKILL.md must still handle blocked tracker rows"
    );
}

#[test]
fn slo_resume_fence_wraps_quoted_issue_bodies() {
    let skill = read(&repo_root().join("skills/slo-resume/SKILL.md"));
    // Abuse-case mitigation `tm-loops-abuse-9`: any quoted carry-forward
    // body snippet is fence-wrapped so prompt-injection is not honored.
    assert!(
        skill.contains("~~~text") || skill.contains("fence"),
        "/slo-resume SKILL.md must fence-wrap any quoted carry-forward body snippet"
    );
}

#[test]
fn slo_resume_no_new_skill_minted() {
    // Sanity: no `slo-help` (or similar) skill folder created — the
    // runbook is explicit that this milestone strengthens /slo-resume
    // rather than minting a new public verb.
    let path = repo_root().join("skills/slo-help");
    assert!(
        !path.exists(),
        "M5 must NOT mint a new /slo-help skill — strengthen the existing /slo-resume entrypoint instead"
    );
}

// Compatibility guard: M4's runbook template change must remain.
#[test]
fn m4_template_carry_forward_section_unchanged() {
    let template = read(&repo_root().join("docs/runbook-template_v_3_template.md"));
    assert!(
        template.contains("Carry-forward from prior retros"),
        "M5 must not silently remove M4's runbook template carry-forward section"
    );
    for lane in &["micro", "milestone", "fresh-runbook"] {
        assert!(
            template.contains(lane),
            "M5 must not silently drift M4's lane vocabulary: {lane}"
        );
    }
}
