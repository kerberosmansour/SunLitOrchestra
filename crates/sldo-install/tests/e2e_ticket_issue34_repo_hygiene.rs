//! Structural-contract tests for GitHub issue #34.
//!
//! These tests pin the repo hygiene and branch discipline expected from
//! `/slo-execute` and `/slo-ticket-execute`: both skills must inspect git
//! state before edits, avoid default/protected branch writes, preserve dirty
//! default-branch work by moving it to a task branch, and record branch
//! remediation evidence.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn assert_contains_all(body: &str, path: &str, markers: &[&str]) {
    for marker in markers {
        assert!(
            body.contains(marker),
            "{path} must document `{marker}` for issue #34"
        );
    }
}

#[test]
fn slo_execute_has_repo_hygiene_gate_before_file_edits() {
    let path = "skills/slo-execute/SKILL.md";
    let body = read(repo_root().join(path));

    assert_contains_all(
        &body,
        path,
        &[
            "Repo hygiene gate",
            "before file edits",
            "git status --short --branch",
            "git rev-parse --abbrev-ref HEAD",
            "origin/HEAD",
            "main",
            "master",
            "default/protected branch",
            "slo/<runbook-prefix>-m<N>",
        ],
    );
}

#[test]
fn slo_ticket_execute_has_matching_repo_hygiene_gate() {
    let path = "skills/slo-ticket-execute/SKILL.md";
    let body = read(repo_root().join(path));

    assert_contains_all(
        &body,
        path,
        &[
            "Repo hygiene gate",
            "before file edits",
            "git status --short --branch",
            "git rev-parse --abbrev-ref HEAD",
            "origin/HEAD",
            "main",
            "master",
            "default/protected branch",
            "ticket/<issue>-<slug>",
            "issue workpad",
        ],
    );
}

#[test]
fn both_execution_skills_preserve_dirty_default_branch_work() {
    let root = repo_root();
    for path in &[
        "skills/slo-execute/SKILL.md",
        "skills/slo-ticket-execute/SKILL.md",
    ] {
        let body = read(root.join(path));
        assert_contains_all(
            &body,
            path,
            &[
                "uncommitted work",
                "default branch",
                "switch",
                "new branch",
                "record the remediation",
            ],
        );
    }
}

#[test]
fn both_execution_skills_document_repo_hygiene_evidence_fields() {
    let root = repo_root();
    for path in &[
        "skills/slo-execute/SKILL.md",
        "skills/slo-ticket-execute/SKILL.md",
    ] {
        let body = read(root.join(path));
        assert_contains_all(
            &body,
            path,
            &[
                "branch before",
                "branch after",
                "dirty-tree state",
                "remediation needed",
            ],
        );
    }
}

#[test]
fn branch_names_are_task_scoped_not_agent_scoped() {
    let root = repo_root();
    for path in &[
        "skills/slo-execute/SKILL.md",
        "skills/slo-ticket-execute/SKILL.md",
    ] {
        let body = read(root.join(path));
        assert_contains_all(
            &body,
            path,
            &["Do not include the agent name", "host name", "model name"],
        );
    }
}

#[test]
fn execution_skills_keep_commits_and_pushes_out_of_execute_by_default() {
    let root = repo_root();
    for path in &[
        "skills/slo-execute/SKILL.md",
        "skills/slo-ticket-execute/SKILL.md",
    ] {
        let body = read(root.join(path));
        assert_contains_all(
            &body,
            path,
            &[
                "Execution may prepare the working tree",
                "commits and pushes",
                "explicitly asks",
            ],
        );
    }
}
