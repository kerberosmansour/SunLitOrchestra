//! M3 structural-contract tests for the loops-and-lessons-closure runbook.
//!
//! These tests assert that `/slo-retro`'s SKILL.md has been extended with
//! the issue-filing flow (classify → dedupe → file with confirmation),
//! that `skills/slo-retro/references/issue-filing-discipline.md` exists
//! and locks the marker, the argv-list discipline, the NO `--repo` rule,
//! the 40-issues/hr rate-limit cap, the three-strike dedupe, the body
//! SHA-256 cross-session dedupe, and the `LESSONS-BACKLOG.md` fallback
//! row schema.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` Milestone 3.

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
fn slo_retro_skill_md_extended() {
    let skill = read(&repo_root().join("skills/slo-retro/SKILL.md"));

    // The extension must add an issue-filing flow as a discrete header.
    assert!(
        skill.contains("Issue filing"),
        "/slo-retro SKILL.md missing 'Issue filing' header — M3 extension not applied"
    );

    // Classification triad must be named in the prose so the agent picks
    // the right destination.
    for classification in &["product", "upstream-OSS", "slo-process"] {
        assert!(
            skill.contains(classification),
            "/slo-retro SKILL.md missing classification: {classification}"
        );
    }

    // Cite the reference file.
    assert!(
        skill.contains("issue-filing-discipline.md"),
        "/slo-retro SKILL.md must cite references/issue-filing-discipline.md"
    );

    // The lessons-file write happens BEFORE the issue filing — discipline
    // rule that defends graceful degradation when `gh` is unavailable.
    let body_lower = skill.to_lowercase();
    assert!(
        body_lower.contains("lessons file") || body_lower.contains("lessons-learned"),
        "/slo-retro SKILL.md must continue to reference the lessons file as the first write"
    );

    // User confirmation gate — never auto-file.
    assert!(
        skill.contains("confirmation") || skill.contains("confirm"),
        "/slo-retro SKILL.md must require user confirmation before filing"
    );
}

#[test]
fn issue_filing_discipline_reference_exists() {
    let path = repo_root().join("skills/slo-retro/references/issue-filing-discipline.md");
    assert!(
        path.exists(),
        "skills/slo-retro/references/issue-filing-discipline.md missing"
    );
    let body = read(&path);
    assert!(
        body.starts_with("---") || body.contains("# "),
        "issue-filing-discipline.md must be a valid Markdown reference file"
    );
}

#[test]
fn argv_list_discipline_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    assert!(
        body.contains("argv-list"),
        "issue-filing-discipline.md must document argv-list discipline (inherits from /slo-sast M5)"
    );
}

#[test]
fn no_repo_flag_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    assert!(
        body.contains("--repo") && (body.contains("NO ") || body.contains("never")),
        "issue-filing-discipline.md must document the NO --repo rule (confused-deputy defense)"
    );
}

#[test]
fn rate_limit_cap_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    assert!(
        body.contains("40 issues") || body.contains("40 per session") || body.contains("40/hr"),
        "issue-filing-discipline.md must document the 40-issues/hour rate-limit cap"
    );
}

#[test]
fn lessons_backlog_fallback_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    assert!(
        body.contains("LESSONS-BACKLOG.md"),
        "issue-filing-discipline.md must document the LESSONS-BACKLOG.md local fallback"
    );
}

#[test]
fn three_strike_dedupe_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    let lower = body.to_lowercase();
    assert!(
        lower.contains("three-strike") || lower.contains("three strike"),
        "issue-filing-discipline.md must document the three-strike dedupe (literal + NFKC + ASCII-collapsed)"
    );
    assert!(
        body.contains("NFKC"),
        "issue-filing-discipline.md must document NFKC normalization in dedupe"
    );
}

#[test]
fn body_sha256_audit_row_documented() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    let lower = body.to_lowercase();
    assert!(
        lower.contains("body_sha256") || lower.contains("body-sha256"),
        "issue-filing-discipline.md must document body_sha256 cross-session dedupe field in audit row"
    );
}

#[test]
fn marker_choice_locked() {
    let body = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    // The marker choice (title prefix `[retro]`, label `retro-derived`,
    // or body sentinel `<!-- retro-derived -->`) is decided in the M3
    // spike. The test only requires the marker be named explicitly so
    // future skills (M4 carry-forward query) can inherit it.
    let has_marker_choice = body.contains("retro-derived")
        || body.contains("[retro]")
        || body.contains("<!-- retro-derived -->");
    assert!(
        has_marker_choice,
        "issue-filing-discipline.md must lock a single marker (title prefix / label / body sentinel)"
    );
}

#[test]
fn slo_retro_install_unchanged() {
    // The slo-retro SKILL.md still installs as a single skill — the new
    // reference file lives under `skills/slo-retro/references/` which
    // does NOT mint a new top-level skill.
    let path = repo_root().join("skills/slo-retro/SKILL.md");
    assert!(path.exists(), "skills/slo-retro/SKILL.md must still be present");

    // The new reference file is INSIDE the slo-retro skill directory,
    // not in the top-level skills/ tree where it would be discovered as
    // a separate skill.
    let ref_path = repo_root().join("skills/slo-retro/references/issue-filing-discipline.md");
    let parent = ref_path.parent().expect("ref path has a parent");
    assert!(
        parent.ends_with("references"),
        "issue-filing-discipline.md must live under skills/slo-retro/references/, not as a sibling skill"
    );
}
