//! M2 structural-contract tests for the engineering skill-improvements runbook.
//!
//! M2 decomposes `/slo-sast` from a monolithic SKILL.md into a thin dispatcher
//! plus five skill-local methodology references. These tests keep the cut honest:
//! the dispatcher stays small, the references travel with the skill directory,
//! and the security-sensitive MUST / MUST NOT rules remain present verbatim.

use std::fs;
use std::path::{Path, PathBuf};

const METHODOLOGY_FILES: &[(&str, &str)] = &[
    ("methodology-m1-parser.md", "slo-sast-methodology-m1-parser"),
    (
        "methodology-m2-stack-detect.md",
        "slo-sast-methodology-m2-stack-detect",
    ),
    (
        "methodology-m3-emission.md",
        "slo-sast-methodology-m3-emission",
    ),
    (
        "methodology-m4-manifest.md",
        "slo-sast-methodology-m4-manifest",
    ),
    (
        "methodology-m5-pr-creation.md",
        "slo-sast-methodology-m5-pr-creation",
    ),
];

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

fn skill_md() -> String {
    read(repo_root().join("skills/slo-sast/SKILL.md"))
}

fn methodology_path(file: &str) -> PathBuf {
    repo_root().join("skills/slo-sast/references").join(file)
}

fn methodology(file: &str) -> String {
    read(methodology_path(file))
}

fn combined_operating_text() -> String {
    let mut combined = skill_md();
    for (file, _) in METHODOLOGY_FILES {
        let path = methodology_path(file);
        if path.exists() {
            combined.push('\n');
            combined.push_str(&read(path));
        }
    }
    combined
}

#[test]
fn slo_sast_skill_md_at_or_under_100_lines_without_exception() {
    let skill = skill_md();
    let line_count = skill.lines().count();

    assert!(
        line_count <= 100,
        "skills/slo-sast/SKILL.md must be a thin dispatcher (<= 100 lines), saw {line_count}"
    );
    assert!(
        !skill.contains("# soft-cap-exception:"),
        "M2 must meet the hard <= 100-line target without a soft-cap exception"
    );
}

#[test]
fn methodology_files_exist_with_frontmatter_and_local_references_path() {
    for (file, name) in METHODOLOGY_FILES {
        let path = methodology_path(file);
        assert!(
            path.exists(),
            "missing skill-local methodology file {}",
            path.display()
        );

        let body = read(&path);
        assert!(
            body.starts_with("---\n"),
            "{file} must begin with YAML frontmatter"
        );
        assert!(
            body.contains(&format!("name: {name}")),
            "{file} must declare frontmatter name `{name}`"
        );
        assert!(
            body.contains("source_skill: skills/slo-sast/SKILL.md"),
            "{file} must record the source skill for install/readability continuity"
        );
    }

    assert!(
        repo_root().join("skills/slo-sast/references").is_dir(),
        "methodology files must live under skills/slo-sast/references/ so they travel with the installed skill symlink"
    );
}

#[test]
fn thin_skill_points_to_every_methodology_contract() {
    let skill = skill_md();
    let required_links = [
        "references/methodology-m1-parser.md",
        "references/methodology-m2-stack-detect.md",
        "references/methodology-m3-emission.md",
        "references/methodology-m4-manifest.md",
        "references/methodology-m5-pr-creation.md",
    ];

    for link in required_links {
        assert!(
            skill.contains(link),
            "thin SKILL.md must cite `{link}` instead of inlining that milestone contract"
        );
    }

    for old_section in [
        "Method (M1",
        "Method (M2",
        "Method (M3",
        "Method (M4",
        "Method (M5",
    ] {
        assert!(
            skill.contains(old_section),
            "existing scanner-orch tests and human readers still need a small method dispatch marker `{old_section}`"
        );
    }
}

#[test]
fn security_disciplines_preserved_in_stage_methodologies() {
    let m2 = methodology("methodology-m2-stack-detect.md");
    let m3 = methodology("methodology-m3-emission.md");
    let m4 = methodology("methodology-m4-manifest.md");
    let m5 = methodology("methodology-m5-pr-creation.md");

    for needle in [
        "All subprocess invocations are argv-list form",
        "`serde_yaml_ng` default settings",
        "Reject any individual YAML file > 1 MiB before parse",
        "git rev-parse HEAD",
        "selection_strategy: \"default-fallback\"",
    ] {
        assert!(m2.contains(needle), "M2 methodology missing `{needle}`");
    }

    for needle in [
        "pull_request_target",
        "permissions:` is `{}`",
        "fetch-depth: 0",
        "SEMGREP_RULES",
        "No `secrets.*` references",
        "No `--autofix` flag",
        "No `--severity` flag",
        "O_NOFOLLOW",
        "Byte-identical copy",
    ] {
        assert!(m3.contains(needle), "M3 methodology missing `{needle}`");
    }

    for needle in [
        "regex-validated or comes from a closed enumeration",
        "Rollback contract on user-decline",
        "Manifest schema v1.0",
        "Defensive design, not regulatory mandate",
    ] {
        assert!(m4.contains(needle), "M4 methodology missing `{needle}`");
    }

    for needle in [
        "gh pr create",
        "NO `--repo` flag",
        "Max 1 PR per invocation",
        "file-content copy",
        "Auto-merge in any form",
    ] {
        assert!(m5.contains(needle), "M5 methodology missing `{needle}`");
    }
}

#[test]
fn must_and_must_not_rules_survive_decomposition_verbatim() {
    let combined = combined_operating_text();
    let rules = [
        "The pinned value MUST match regex `^[0-9a-f]{40}$`",
        "The emitted workflow MUST satisfy ALL of:",
        "`on:` block contains `pull_request` and MUST NOT contain `pull_request_target`. Hard ban.",
        "No `--severity` flag (rule selection is the only severity gate per research synthesis).",
        "Every value is **regex-validated or comes from a closed enumeration**",
        "Every re-derivation surfaces as a human-review PR.",
        "**NO swallowing of `gh` errors**",
    ];

    for rule in rules {
        assert!(
            combined.contains(rule),
            "decomposition lost required rule: {rule}"
        );
    }
}

#[test]
fn existing_references_sast_authority_docs_are_not_moved_or_edited() {
    let cases = [
        (
            "references/sast/threat-model-parser-contract.md",
            "tm-scanner-orchestration-abuse-1",
        ),
        (
            "references/sast/scanner-orch-pinned-rules-sha.md",
            "40-character",
        ),
        ("references/sast/stack-detection-contract.md", "polyglot"),
        (
            "references/sast/scanner-orch-workflow-template.yml",
            "pull_request",
        ),
        (
            "references/sast/scanner-orch-action-shas.md",
            "actions/checkout",
        ),
        (
            "references/sast/scanner-orch-manifest-schema.md",
            "defensive design",
        ),
        ("references/sast/AUTHORING.md", "Trail of Bits"),
    ];

    for (path, sentinel) in cases {
        let content = read(repo_root().join(path));
        assert!(
            content.contains(sentinel),
            "{path} must remain in place and keep sentinel `{sentinel}`"
        );
    }
}
