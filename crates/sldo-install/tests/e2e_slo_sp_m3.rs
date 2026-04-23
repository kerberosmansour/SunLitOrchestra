//! M3 E2E: /slo-research skill contract.
//!
//! Checks the static skill: valid frontmatter, mentions the sldo-research
//! backend, and includes a which-style preflight hint. Runtime behavior is
//! tested when Claude Code actually invokes the skill.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn skill_body() -> String {
    fs::read_to_string(repo_root().join("skills").join("slo-research").join("SKILL.md"))
        .expect("slo-research SKILL.md missing")
}

#[test]
fn frontmatter_valid() {
    let body = skill_body();
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-research"));
    assert!(body.contains("description:"));
}

#[test]
fn references_sldo_research_backend() {
    let body = skill_body();
    assert!(
        body.contains("sldo-research"),
        "skill must shell out to the Rust backend"
    );
}

#[test]
fn has_preflight_cascade_for_missing_binary() {
    let body = skill_body();
    // The skill must tell users how to fix a missing binary — not silently fall through.
    assert!(
        body.contains("which sldo-research") || body.contains("which::which"),
        "skill must include a which-style detection step"
    );
    assert!(
        body.contains("cargo install"),
        "skill must include an install hint"
    );
}

#[test]
fn delegates_third_party_api_docs_to_chub() {
    let body = skill_body();
    assert!(
        body.to_lowercase().contains("chub") || body.to_lowercase().contains("get-api-docs"),
        "skill must explicitly delegate third-party library API docs to get-api-docs / chub"
    );
}
