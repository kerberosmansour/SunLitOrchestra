//! M1 structural-contract tests for Engineering Skill Improvements.
//!
//! M1 seeds the shared `references/templates/` library and wires the
//! security-engineering-facing skills to cite those shared disciplines.
//! These tests assert documented shape only; they do not execute slash-command
//! runtime behavior.

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

const TEMPLATE_FILES: &[&str] = &[
    "intake-checklist",
    "restate-and-confirm",
    "citation-discipline",
    "tool-safety-section",
    "output-frontmatter",
    "escalation",
    "eval-cases",
    "heuristic-numbers-discipline",
    "rate-limiting-discipline",
    "fallback-discipline",
    "version-pinning-discipline",
];

const UPDATED_SKILLS: &[&str] = &[
    "skills/slo-sast/SKILL.md",
    "skills/slo-tla/SKILL.md",
    "skills/slo-rulegen/SKILL.md",
    "skills/slo-verify/SKILL.md",
    "skills/slo-research/SKILL.md",
];

const SOURCE_HIERARCHY: &str = r#"1. Tool vendor official documentation at a pinned version or dated page.
2. Tool repository README, CHANGELOG, or release notes at a pinned commit.
3. Upstream advisory database, standards body, or regulator documentation.
4. Named academic paper or conference talk with author and year.
5. Vendor blog post or maintainer discussion as secondary context only.
6. Never Stack Overflow, random forums, unsourced commentary, or model memory."#;

#[test]
fn templates_directory_has_all_eleven_files() {
    let root = repo_root();
    for slug in TEMPLATE_FILES {
        let rel = format!("references/templates/{slug}.md");
        let body = read(root.join(&rel));
        assert!(
            body.starts_with("---\n"),
            "{rel} must start with YAML frontmatter"
        );
        assert!(
            body.contains(&format!("name: {slug}")),
            "{rel} frontmatter must declare name: {slug}"
        );
        assert!(
            body.contains("status: stable-template"),
            "{rel} must declare stable-template status"
        );
        assert!(body.len() > 500, "{rel} must not be placeholder-sized");
    }
}

#[test]
fn citation_discipline_source_hierarchy_verbatim() {
    let body = read(repo_root().join("references/templates/citation-discipline.md"));
    assert!(
        body.contains(SOURCE_HIERARCHY),
        "citation-discipline.md must contain the locked six-tier source hierarchy verbatim"
    );
    assert!(
        body.contains("Unverifiable claims are removed, not weakened"),
        "citation-discipline.md must lock the remove-not-weaken rule"
    );
}

#[test]
fn every_other_template_cites_citation_discipline() {
    let root = repo_root();
    for slug in TEMPLATE_FILES
        .iter()
        .copied()
        .filter(|slug| *slug != "citation-discipline")
    {
        let rel = format!("references/templates/{slug}.md");
        let body = read(root.join(&rel));
        assert!(
            body.contains("references/templates/citation-discipline.md")
                || body.contains("citation-discipline.md"),
            "{rel} must cite citation-discipline.md"
        );
    }
}

#[test]
fn five_security_engineering_skills_cite_templates() {
    let root = repo_root();
    for rel in UPDATED_SKILLS {
        let body = read(root.join(rel));
        assert!(
            body.contains("references/templates/"),
            "{rel} must cite at least one shared template"
        );
    }
}

#[test]
fn targeted_skill_updates_preserve_frontmatter_names() {
    let root = repo_root();
    for rel in UPDATED_SKILLS {
        let body = read(root.join(rel));
        let expected_name = rel
            .trim_start_matches("skills/")
            .trim_end_matches("/SKILL.md");
        assert!(
            body.contains(&format!("name: {expected_name}")),
            "{rel} must preserve its frontmatter skill name"
        );
    }
}

#[test]
fn m1_lessons_document_source_verification_spike() {
    let body = read(repo_root().join("docs/slo/lessons/eng-imp-m1.md"));
    assert!(
        body.contains("Source-verification spike"),
        "eng-imp-m1 lessons must document the source-verification spike"
    );
    for marker in [
        "Semgrep",
        "GitHub Actions",
        "cargo audit",
        "ZAP",
        "Dastardly",
        "human review",
    ] {
        assert!(
            body.contains(marker),
            "eng-imp-m1 lessons must include spike marker `{marker}`"
        );
    }
}
