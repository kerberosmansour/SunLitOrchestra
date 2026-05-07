//! M1 structural-contract tests for the Fowler AI architecture SLO improvements.
//!
//! M1 extends `/slo-architect` with two additive design artifacts:
//! a reversibility matrix and a brownfield code map. These tests assert the
//! Markdown contract shape; they do not invoke an agent runtime.

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

fn architect_skill() -> String {
    read(repo_root().join("skills/slo-architect/SKILL.md"))
}

#[test]
fn architect_documents_reversibility_output() {
    let skill = architect_skill();

    assert!(
        skill.contains("docs/slo/design/<slug>-reversibility.md"),
        "/slo-architect must document the additive reversibility-matrix output"
    );
    assert!(
        skill.to_lowercase().contains("hard-to-change"),
        "reversibility output must name hard-to-change decisions"
    );
    assert!(
        skill.contains("Rollback / migration path") || skill.contains("rollback / migration path"),
        "reversibility output must require rollback or migration evidence"
    );
}

#[test]
fn architect_documents_brownfield_code_map_output() {
    let skill = architect_skill();

    assert!(
        skill.contains("docs/slo/design/<slug>-code-map.md"),
        "/slo-architect must document the additive brownfield code-map output"
    );
    for needle in [
        "Four-object",
        "Exemplar",
        "Anti-exemplar",
        "dangerous seams",
    ] {
        assert!(
            skill.to_lowercase().contains(&needle.to_lowercase()),
            "brownfield code-map contract missing `{needle}`"
        );
    }
}

#[test]
fn existing_architect_outputs_remain_documented() {
    let skill = architect_skill();

    for output in [
        "ARCHITECTURE.md",
        "docs/slo/design/<slug>-stack-decision.md",
        "docs/slo/design/<slug>-interfaces.md",
        "SECURITY.md",
        "docs/slo/design/<slug>-threat-model.md",
    ] {
        assert!(
            skill.contains(output),
            "existing /slo-architect output must remain documented: {output}"
        );
    }
}

#[test]
fn architect_output_count_wording_is_updated() {
    let skill = architect_skill();

    assert!(
        !skill.contains("Five files"),
        "stale `Five files` wording must be removed after adding two outputs"
    );
    assert!(
        skill.contains("Seven files"),
        "output intro should state the new count clearly"
    );
}

#[test]
fn brownfield_code_map_has_greenfield_na_path() {
    let skill = architect_skill();

    assert!(
        skill.contains("N/A — greenfield; no existing codebase to map"),
        "greenfield projects must have an explicit code-map N/A path"
    );
    assert!(
        skill.contains("non-empty") || skill.contains("not empty"),
        "code-map prose must distinguish non-empty brownfield repos from greenfield repos"
    );
}

#[test]
fn architect_preserves_user_string_fence_rule() {
    let skill = architect_skill();

    assert!(
        skill.contains("~~~text"),
        "user-provided strings must remain fenced as literal text"
    );
    assert!(
        skill.contains("User-provided strings") || skill.contains("user-provided strings"),
        "skill must keep the user-provided string injection defense visible"
    );
}

#[test]
fn architect_evals_cover_reversibility_and_ambiguous_brownfield_context() {
    let happy = read(repo_root().join("skills/slo-architect/evals/happy-path.md"));
    let high_risk = read(repo_root().join("skills/slo-architect/evals/high-risk-case.md"));

    assert!(
        happy.contains("reversibility") && happy.contains("code map"),
        "happy-path eval should expect reversibility and code-map outputs"
    );
    assert!(
        high_risk.contains("ambiguous brownfield") || high_risk.contains("brownfield context"),
        "high-risk eval should cover missing or ambiguous brownfield context"
    );
}
