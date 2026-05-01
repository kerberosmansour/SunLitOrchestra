//! M4 structural-contract tests for the loops-and-lessons-closure runbook.
//!
//! These tests assert that `/slo-execute`'s SKILL.md has been extended with
//! a pre-flight Step 1.5 that reads open prior-retro issues for the
//! current runbook's prefix, that the runbook template gains a new
//! optional "Carry-forward from prior retros" section with the
//! `micro | milestone | fresh-runbook` lane vocabulary, and that
//! existing pre-flight Step 1 (read previous milestone's lessons file)
//! is preserved.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` Milestone 4.

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
fn slo_execute_pre_flight_extended() {
    let skill = read(&repo_root().join("skills/slo-execute/SKILL.md"));

    // The new pre-flight step must mention prior-retro issues so the
    // carry-forward query is anchored in prose.
    assert!(
        skill.contains("prior-retro") || skill.contains("retro-derived"),
        "/slo-execute SKILL.md must mention prior-retro issues in pre-flight"
    );

    // Existing Step 1 (read previous milestone's lessons file) must be
    // preserved — additive, not replaced.
    assert!(
        skill.contains("previous milestone")
            || skill.contains("lessons file from the previous milestone"),
        "/slo-execute SKILL.md must preserve existing 'read previous milestone's lessons file' step"
    );

    // The new step must cite M3's marker so the query stays aligned.
    assert!(
        skill.contains("retro-derived"),
        "/slo-execute SKILL.md must cite the `retro-derived` label (M3 marker)"
    );
}

#[test]
fn slo_execute_no_auto_extend_allowlist() {
    let skill = read(&repo_root().join("skills/slo-execute/SKILL.md"));
    let lower = skill.to_lowercase();
    // Carry-forward must NOT auto-extend the allow-list — discipline rule.
    assert!(
        lower.contains("user decides each milestone")
            || lower.contains("never auto-extend")
            || lower.contains("does not auto-extend")
            || lower.contains("user decides"),
        "/slo-execute SKILL.md must state that carry-forward never auto-extends the allow-list"
    );
}

#[test]
fn slo_execute_gh_issue_list_argv_list_documented() {
    let skill = read(&repo_root().join("skills/slo-execute/SKILL.md"));
    // The query must be documented in argv-list form — inherits M3 discipline.
    assert!(
        skill.contains("gh issue list"),
        "/slo-execute SKILL.md must document `gh issue list` for the carry-forward query"
    );
    assert!(
        skill.contains("argv-list") || skill.contains("argv list"),
        "/slo-execute SKILL.md carry-forward query must follow argv-list discipline"
    );
}

#[test]
fn runbook_template_carry_forward_section() {
    let template = read(&repo_root().join("docs/slo/templates/runbook-template_v_3_template.md"));
    assert!(
        template.contains("Carry-forward from prior retros"),
        "runbook-template_v_3_template.md must add the optional 'Carry-forward from prior retros' section"
    );
}

#[test]
fn runbook_template_carry_forward_lane_column() {
    let template = read(&repo_root().join("docs/slo/templates/runbook-template_v_3_template.md"));
    // The lane vocabulary must appear in the template so authors know
    // the contract.
    for lane in &["micro", "milestone", "fresh-runbook"] {
        assert!(
            template.contains(lane),
            "runbook template carry-forward section must document lane: {lane}"
        );
    }
}

#[test]
fn runbook_template_carry_forward_section_is_optional() {
    let template = read(&repo_root().join("docs/slo/templates/runbook-template_v_3_template.md"));
    let lower = template.to_lowercase();
    // The new section must be marked optional so existing runbooks
    // remain valid without it.
    assert!(
        lower.contains("optional"),
        "runbook template carry-forward section must be marked optional (backward compat)"
    );
}

#[test]
fn this_runbook_has_carry_forward_section() {
    // Dogfood: the loops runbook itself must include a "Carry-forward
    // from prior retros" section after M4 closes — even if empty.
    let runbook = read(
        &repo_root().join("docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md"),
    );
    assert!(
        runbook.contains("Carry-forward from prior retros"),
        "this runbook must include a dogfood 'Carry-forward from prior retros' section"
    );
}

// Compatibility guard: M3's reference file and SKILL.md changes must
// remain valid. M4 must not silently drift the marker.
#[test]
fn m3_marker_unchanged_at_m4() {
    let ref_file = read(
        &repo_root().join("skills/slo-retro/references/issue-filing-discipline.md"),
    );
    assert!(
        ref_file.contains("retro-derived"),
        "M4 must not drift M3's locked marker (`retro-derived`)"
    );
}
