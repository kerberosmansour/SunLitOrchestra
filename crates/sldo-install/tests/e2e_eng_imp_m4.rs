//! M4 structural-contract tests for the engineering skill-improvements runbook.
//!
//! M4 decomposes `/slo-plan` and adds a soft line-cap guard so future skills do
//! not drift back into monoliths without an explicit, reviewable exception.

use std::fs;
use std::path::{Path, PathBuf};

const SOFT_SKILL_CAP: usize = 200;
const HARD_PLAN_CAP: usize = 80;
const METHODOLOGY_CAP: usize = 500;
const TEMPLATE_CAP: usize = 300;

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

fn skill_md(skill: &str) -> String {
    read(repo_root().join("skills").join(skill).join("SKILL.md"))
}

fn skill_exception_reason(body: &str) -> Option<String> {
    exception_reason(body, "# soft-cap-exception:")
}

fn methodology_exception_reason(body: &str) -> Option<String> {
    exception_reason(body, "# methodology-cap-exception:")
}

fn template_exception_reason(body: &str) -> Option<String> {
    exception_reason(body, "# template-cap-exception:")
}

fn exception_reason(body: &str, marker: &str) -> Option<String> {
    body.lines()
        .find_map(|line| line.trim().strip_prefix(marker).map(str::trim))
        .map(str::to_string)
}

fn assert_reason_non_empty(reason: Option<String>, path: &Path, marker: &str) {
    let Some(reason) = reason else {
        panic!(
            "{} soft cap exceeded; add `{marker} <reason>`",
            path.display()
        );
    };

    assert!(
        !reason.is_empty(),
        "{} exception reason required for `{marker}`",
        path.display()
    );
}

#[test]
fn slo_plan_skill_md_decomposed() {
    let body = skill_md("slo-plan");
    let line_count = body.lines().count();

    assert!(
        line_count <= HARD_PLAN_CAP,
        "skills/slo-plan/SKILL.md must be <= {HARD_PLAN_CAP} lines after Step 2 extraction, saw {line_count}"
    );
    assert!(
        body.contains("NEVER generate a whole runbook in one shot"),
        "discipline rule must remain in SKILL.md"
    );
    assert!(
        body.contains("references/methodology-milestone-authoring.md"),
        "thin SKILL.md must cite the milestone-authoring methodology"
    );
    assert!(
        body.contains("Data classification")
            && body.contains("Proactive controls in play")
            && body.contains("Abuse acceptance scenarios"),
        "security Contract Block row names must remain visible for old tests and human readers"
    );
}

#[test]
fn methodology_milestone_authoring_exists_with_frontmatter() {
    let path = repo_root()
        .join("skills/slo-plan/references")
        .join("methodology-milestone-authoring.md");
    assert!(
        path.exists(),
        "missing skill-local methodology file {}",
        path.display()
    );

    let body = read(&path);
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-plan-methodology-milestone-authoring"));
    assert!(body.contains("source_skill: skills/slo-plan/SKILL.md"));

    for needle in [
        "For milestone N, write the full section:",
        "**Goal**",
        "**Contract Block**",
        "**Data classification**",
        "**Proactive controls in play**",
        "**Abuse acceptance scenarios**",
        "**Evidence Log**",
        "**Definition of Done**",
    ] {
        assert!(
            body.contains(needle),
            "milestone-authoring methodology missing `{needle}`"
        );
    }
}

#[test]
fn soft_line_cap_runs_for_every_skill_md() {
    let skills_dir = repo_root().join("skills");
    let mut checked = 0;

    for entry in fs::read_dir(&skills_dir).expect("skills dir missing") {
        let entry = entry.expect("cannot read skills entry");
        let path = entry.path().join("SKILL.md");
        if !path.exists() {
            continue;
        }

        checked += 1;
        let body = read(&path);
        if body.lines().count() > SOFT_SKILL_CAP {
            assert_reason_non_empty(
                skill_exception_reason(&body),
                &path,
                "# soft-cap-exception:",
            );
        }
    }

    assert!(
        checked >= 30,
        "soft-cap test did not inspect the skill pack"
    );
}

#[test]
fn pragma_reason_required_for_skill_soft_cap() {
    let too_long = (0..=SOFT_SKILL_CAP)
        .map(|_| "line")
        .collect::<Vec<_>>()
        .join("\n");
    assert!(skill_exception_reason(&too_long).is_none());

    let empty = format!("---\n# soft-cap-exception: \n---\n{too_long}");
    assert_eq!(skill_exception_reason(&empty).as_deref(), Some(""));

    let with_reason =
        format!("---\n# soft-cap-exception: carries generated template contract\n---\n{too_long}");
    assert_eq!(
        skill_exception_reason(&with_reason).as_deref(),
        Some("carries generated template contract")
    );
}

#[test]
fn methodology_files_stay_under_cap_or_explain_exception() {
    let mut checked = 0;
    for skill in fs::read_dir(repo_root().join("skills")).expect("skills dir missing") {
        let references = skill.expect("cannot read skill").path().join("references");
        if !references.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&references).expect("cannot read references dir") {
            let path = entry.expect("cannot read references entry").path();
            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !file_name.starts_with("methodology-")
                || path.extension().and_then(|e| e.to_str()) != Some("md")
            {
                continue;
            }

            checked += 1;
            let body = read(&path);
            if body.lines().count() > METHODOLOGY_CAP {
                assert_reason_non_empty(
                    methodology_exception_reason(&body),
                    &path,
                    "# methodology-cap-exception:",
                );
            }
        }
    }

    assert!(
        checked >= 10,
        "methodology-cap test did not inspect the decomposed methodology files"
    );
}

#[test]
fn shared_templates_stay_under_cap_or_explain_exception() {
    let templates = repo_root().join("references/templates");
    let mut checked = 0;

    for entry in fs::read_dir(&templates).expect("references/templates missing") {
        let path = entry.expect("cannot read template entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        checked += 1;
        let body = read(&path);
        if body.lines().count() > TEMPLATE_CAP {
            assert_reason_non_empty(
                template_exception_reason(&body),
                &path,
                "# template-cap-exception:",
            );
        }
    }

    assert!(
        checked >= 8,
        "template-cap test did not inspect shared templates"
    );
}
