//! M4 structural-contract tests for the Fowler AI architecture SLO improvements.
//!
//! M4 strengthens `/slo-critique` by adding architecture-coherence checks to
//! the engineering persona. These tests assert the checks cite the new M1-M3
//! artifacts and preserve the existing finding-table discipline.

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
fn critique_skill_mentions_architecture_coherence_in_eng_pass() {
    let skill = read(repo_root().join("skills/slo-critique/SKILL.md"));
    let lower = skill.to_lowercase();

    assert!(
        lower.contains("architecture coherence"),
        "critique skill must name the architecture coherence pass"
    );
    for needle in [
        "four-object summary",
        "reversibility",
        "exemplar",
        "ai tolerance",
    ] {
        assert!(
            lower.contains(needle),
            "critique skill missing coherence input `{needle}`"
        );
    }
    assert!(
        skill.contains(
            "| id | persona | category | runbook section | finding | concrete scenario | recommendation |",
        ),
        "M4 must preserve the existing critique finding-table schema"
    );
}

#[test]
fn eng_persona_names_all_coherence_inputs() {
    let persona = read(repo_root().join("skills/slo-critique/personas/eng.md"));
    let lower = persona.to_lowercase();

    for needle in [
        "architecture coherence",
        "four-object summary",
        "reversibility",
        "exemplar",
        "anti-exemplar",
        "ai tolerance",
        "code-map",
    ] {
        assert!(
            lower.contains(needle),
            "eng persona missing architecture coherence input `{needle}`"
        );
    }
}

#[test]
fn eng_persona_rejects_vague_architecture_concerns() {
    let persona = read(repo_root().join("skills/slo-critique/personas/eng.md"));
    let lower = persona.to_lowercase();

    assert!(
        lower.contains("architecture feels messy") || lower.contains("vague architecture"),
        "eng persona must reject vague architecture concerns explicitly"
    );
    for needle in ["actor", "action", "bad outcome"] {
        assert!(
            lower.contains(needle),
            "eng persona must require concrete scenario element `{needle}`"
        );
    }
}

#[test]
fn critique_evals_cover_concrete_and_rejected_coherence_findings() {
    let happy = read(repo_root().join("skills/slo-critique/evals/happy-path.md"));
    let high_risk = read(repo_root().join("skills/slo-critique/evals/high-risk-case.md"));
    let combined = format!("{happy}\n{high_risk}").to_lowercase();

    for needle in [
        "exemplar mismatch",
        "missing reversibility",
        "ai tolerance",
        "vague architecture concern",
        "actor",
        "bad outcome",
    ] {
        assert!(
            combined.contains(needle),
            "critique evals missing M4 coverage for `{needle}`"
        );
    }
}

#[test]
fn legacy_security_m3_guard_allows_eng_persona_changes_after_m4() {
    let legacy = read(repo_root().join("crates/sldo-install/tests/e2e_slo_sec_m3.rs"));

    assert!(
        legacy.contains("eng_persona_architecture_coherence_allowed_after_m4"),
        "legacy M3 guard must document that M4 intentionally changes eng.md"
    );
    assert!(
        !legacy.contains("fn eng_persona_unchanged"),
        "legacy M3 guard must no longer pin eng.md byte-for-byte after M4"
    );
}
