//! Workspace-level E2E for SAST Runbook A M2 — extend-mode contract.
//!
//! M2 ships:
//! - The full extend.md prompt body (was a skeleton in M1).
//! - The skill's extend-mode section in slo-rulegen/SKILL.md.
//! - The atomic-write contract per /slo-critique eng-5 (described; implemented
//!   skill-side, not xtask-side).
//! - Test fixtures at tests/fixtures/extend_mode/{good_bug,malicious_bug}/.
//!
//! These tests assert structural properties of the M2-shipped artifacts. They
//! do NOT invoke `/slo-rulegen --extend` end-to-end, because that requires
//! Claude Code's slash-command runtime which isn't available from a `cargo test`
//! invocation. The tests cover what's verifiable from disk: prompt content,
//! fixture presence, and tier-detect behaviour.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn extend_md_is_no_longer_a_skeleton() {
    let p = workspace_root()
        .join("references")
        .join("sast")
        .join("prompts")
        .join("extend.md");
    let content = fs::read_to_string(&p).expect("read extend.md");
    // Skeleton sentinels from M1.
    assert!(
        !content.contains("M2 fills this in"),
        "extend.md should no longer contain the M1 skeleton sentinel"
    );
    assert!(
        !content.contains("NOT YET IMPLEMENTED"),
        "extend.md should no longer say NOT YET IMPLEMENTED"
    );
    // Body content sentinels (M2 must include these).
    assert!(
        content.contains("Render-as-untrusted contract"),
        "extend.md must document the ~~~text fence rendering"
    );
    assert!(
        content.contains("~~~text"),
        "extend.md must literally contain the ~~~text fence in its template"
    );
    assert!(
        content.contains("Atomic-write contract"),
        "extend.md must document the atomic-write contract per /slo-critique eng-5"
    );
}

#[test]
fn extend_md_forbids_webfetch_and_websearch_in_prose() {
    let p = workspace_root()
        .join("references")
        .join("sast")
        .join("prompts")
        .join("extend.md");
    let content = fs::read_to_string(&p).expect("read extend.md");
    assert!(
        content.contains("WebFetch") && content.contains("WebSearch"),
        "extend.md must explicitly forbid WebFetch and WebSearch (sec-5)"
    );
    assert!(
        content.contains("FORBIDDEN") || content.contains("MUST NOT"),
        "extend.md must use imperative language for the toolflag denial"
    );
}

#[test]
fn extend_md_cites_threat_model_row_for_prompt_injection() {
    let p = workspace_root()
        .join("references")
        .join("sast")
        .join("prompts")
        .join("extend.md");
    let content = fs::read_to_string(&p).expect("read extend.md");
    assert!(
        content.contains("tm-sast-rulegen-skill-pack-abuse-1"),
        "extend.md must cite the threat-model row for prompt-injection (tm-abuse-1)"
    );
}

#[test]
fn good_bug_fixture_exists_and_has_required_files() {
    let dir = workspace_root()
        .join("tests")
        .join("fixtures")
        .join("extend_mode")
        .join("good_bug");
    assert!(
        dir.is_dir(),
        "tests/fixtures/extend_mode/good_bug/ must exist"
    );
    let summary = dir.join("bug-summary.md");
    let diff = dir.join("fix.diff");
    assert!(summary.exists(), "bug-summary.md must exist");
    assert!(diff.exists(), "fix.diff must exist");
    let summary_content = fs::read_to_string(&summary).unwrap();
    assert!(
        summary_content.contains("CWE-755") || summary_content.contains("panic"),
        "good_bug summary should reference the panic-DoS class"
    );
}

#[test]
fn malicious_bug_fixture_contains_prompt_injection_attempt() {
    let dir = workspace_root()
        .join("tests")
        .join("fixtures")
        .join("extend_mode")
        .join("malicious_bug");
    assert!(
        dir.is_dir(),
        "tests/fixtures/extend_mode/malicious_bug/ must exist"
    );
    let summary = dir.join("bug-summary.md");
    assert!(summary.exists(), "bug-summary.md must exist");
    let content = fs::read_to_string(&summary).unwrap();
    assert!(
        content.contains("ignore prior instructions"),
        "malicious_bug summary must contain the prompt-injection attempt sentinel for resistance testing"
    );
    assert!(
        content.to_lowercase().contains("attacker.example") || content.to_lowercase().contains("attacker"),
        "malicious_bug summary must reference an attacker URL (the exfil sink the skill MUST refuse)"
    );
}

#[test]
fn skill_md_documents_extend_mode_contract() {
    let p = workspace_root()
        .join("skills")
        .join("slo-rulegen")
        .join("SKILL.md");
    let content = fs::read_to_string(&p).expect("read slo-rulegen SKILL.md");
    assert!(
        content.contains("Extend-mode contract"),
        "slo-rulegen SKILL.md must have an `Extend-mode contract` section"
    );
    assert!(
        content.contains("--bug-summary") && content.contains("--fix-diff") && content.contains("--file-paths"),
        "Extend-mode contract must document --bug-summary / --fix-diff / --file-paths flags"
    );
    assert!(
        content.contains("Atomic-write")
            || content.contains("atomic-write")
            || content.contains("tempfile::TempDir"),
        "Extend-mode contract must reference the atomic-write discipline"
    );
}

#[test]
fn skill_md_forbids_webfetch_and_websearch_in_extend_section() {
    let p = workspace_root()
        .join("skills")
        .join("slo-rulegen")
        .join("SKILL.md");
    let content = fs::read_to_string(&p).expect("read slo-rulegen SKILL.md");
    assert!(
        content.contains("Tools you MUST NOT use"),
        "slo-rulegen SKILL.md must have a `## Tools you MUST NOT use` section per /slo-critique sec-5"
    );
    assert!(
        content.contains("WebFetch") && content.contains("WebSearch"),
        "Tools-MUST-NOT-use section must list WebFetch and WebSearch"
    );
}
