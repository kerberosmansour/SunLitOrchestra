//! M2 structural-contract tests for the Fowler AI architecture SLO improvements.
//!
//! M2 extends `/slo-plan` and the v4 runbook template with exemplar rows and
//! true refactoring discipline. These tests assert the Markdown contract shape
//! and keep the skill-local template mirror aligned with the docs mirror.

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

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_local_v4() -> String {
    read(repo_root().join("skills/slo-plan/references/runbook-template_v_4_template.md"))
}

fn docs_v4() -> String {
    read(repo_root().join("docs/slo/templates/runbook-template_v_4_template.md"))
}

#[test]
fn plan_emits_exemplar_rows_in_both_v4_templates() {
    for (label, body) in [("skill-local", skill_local_v4()), ("docs", docs_v4())] {
        for row in ["Exemplar code to copy", "Anti-exemplar code not to copy"] {
            assert!(
                body.contains(row),
                "{label} v4 template missing Contract Block row `{row}`"
            );
        }
    }
}

#[test]
fn refactor_budget_row_is_preserved() {
    for (label, body) in [("skill-local", skill_local_v4()), ("docs", docs_v4())] {
        assert!(
            body.contains("Refactor budget"),
            "{label} v4 template must preserve the existing `Refactor budget` row"
        );
    }
}

#[test]
fn refactoring_discipline_reference_exists_and_defines_true_refactor() {
    let path = repo_root().join("skills/slo-plan/references/refactoring-discipline.md");
    assert!(
        path.exists(),
        "missing skill-local refactoring discipline reference at {}",
        path.display()
    );

    let body = read(&path);
    for needle in [
        "behavior-preserving",
        "pre-test",
        "microstep",
        "post-test proof",
    ] {
        assert!(
            body.contains(needle),
            "refactoring-discipline.md missing `{needle}`"
        );
    }
}

#[test]
fn slo_plan_cites_exemplars_and_refactoring_discipline() {
    let skill = read(repo_root().join("skills/slo-plan/SKILL.md"));

    for needle in [
        "Exemplar code to copy",
        "Anti-exemplar code not to copy",
        "references/refactoring-discipline.md",
    ] {
        assert!(
            skill.contains(needle),
            "/slo-plan SKILL.md missing `{needle}`"
        );
    }
}

#[test]
fn milestone_authoring_methodology_requires_new_rows() {
    let body =
        read(repo_root().join("skills/slo-plan/references/methodology-milestone-authoring.md"));

    for needle in [
        "Exemplar code to copy",
        "Anti-exemplar code not to copy",
        "refactoring-discipline.md",
    ] {
        assert!(
            body.contains(needle),
            "milestone-authoring methodology missing `{needle}`"
        );
    }
}

#[test]
fn docs_only_milestones_have_na_path_for_exemplars() {
    let methodology =
        read(repo_root().join("skills/slo-plan/references/methodology-milestone-authoring.md"));

    assert!(
        methodology.contains("N/A — docs-only"),
        "docs-only milestones must have an explicit exemplar-row N/A path"
    );
}
