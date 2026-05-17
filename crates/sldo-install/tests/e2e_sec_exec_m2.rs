//! M2 structural-contract tests for secure execution controls.
//!
//! M2 makes `/slo-plan` proactive-control rows actionable implementation
//! constraints instead of decorative labels.

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

#[test]
fn plan_skill_requires_actionable_controls_and_matrix() {
    let skill = read(repo_root().join("skills/slo-plan/SKILL.md"));
    assert!(skill.contains("actionable implementation constraints"));
    assert!(skill.contains("references/secure-construction-matrix.md"));
}

#[test]
fn secure_construction_matrix_covers_core_surfaces() {
    let body = read(repo_root().join("skills/slo-plan/references/secure-construction-matrix.md"));
    for needle in [
        "Touched surface",
        "Secure-construction default",
        "Rust request body",
        "safe_types",
        "HTTP API route",
        "Pulumi TypeScript cloud resource",
        "GitHub Actions",
        "Capability gap",
    ] {
        assert!(body.contains(needle), "matrix missing `{needle}`");
    }
}

#[test]
fn vocabulary_distinguishes_hulumi_from_generic_pulumi() {
    let vocab = read(repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"));
    for needle in [
        "Hulumi explicit",
        "Generic Pulumi TypeScript",
        "Do not force Hulumi",
        "policy as code",
        "unit tests",
    ] {
        assert!(vocab.contains(needle), "vocabulary missing `{needle}`");
    }
}

#[test]
fn typescript_and_java_fallback_forbids_model_memory_claims() {
    let vocab = read(repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"));
    for needle in [
        "TypeScript / Java fallback",
        "do not invent library capability claims",
        "official framework docs",
        "OWASP ASVS",
    ] {
        assert!(vocab.contains(needle), "fallback missing `{needle}`");
    }
}

