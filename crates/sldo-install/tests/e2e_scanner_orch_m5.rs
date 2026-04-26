//! M5 structural-contract tests for the scanner-orchestration runbook.
//!
//! M5 closes the auto-tuning loop: re-derivation trigger detection + diff PR
//! generation + dogfood E2E. Tests assert the triggers reference doc and
//! SKILL.md document the contract correctly. Runtime behavior (actual `gh pr
//! create` invocations, real drift detection) is exercised via smoke tests.

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

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_md() -> String {
    read(&repo_root().join("skills/slo-sast/SKILL.md"))
}

fn triggers_doc() -> String {
    read(&repo_root().join("references/sast/scanner-orch-rederivation-triggers.md"))
}

// ---------------------------------------------------------------------------
// Triggers reference doc — predicate set + PR format + rate-limit policy.
// ---------------------------------------------------------------------------

#[test]
fn triggers_doc_exists() {
    let doc = triggers_doc();
    assert!(
        doc.len() > 800,
        "triggers doc is suspiciously short ({} bytes)",
        doc.len()
    );
}

#[test]
fn triggers_doc_documents_all_four_predicates() {
    let doc = triggers_doc();
    let predicates = [
        "Threat-model SHA changed",
        "pin bumped",
        "Stack added",
        "CWEs claimed changed",
    ];
    for p in predicates {
        assert!(
            doc.contains(p),
            "triggers doc must document predicate `{p}`"
        );
    }
}

#[test]
fn triggers_doc_documents_compound_trigger_coalescing() {
    let doc = triggers_doc();
    assert!(
        doc.to_lowercase().contains("compound trigger"),
        "triggers doc must document compound-trigger coalescing"
    );
    assert!(
        doc.contains("single") || doc.contains("SINGLE"),
        "triggers doc must specify exactly one PR per invocation even with multiple triggers"
    );
}

#[test]
fn triggers_doc_documents_rate_limit_per_invocation() {
    let doc = triggers_doc();
    // ENG-4: rate limit = max 1 PR per invocation; cross-invocation is user's responsibility.
    assert!(
        doc.contains("Maximum 1 PR per skill invocation")
            || doc.contains("max 1 PR per invocation")
            || doc.contains("Maximum 1 PR per invocation"),
        "triggers doc must document the per-invocation rate limit (ENG-4)"
    );
    assert!(
        doc.to_lowercase().contains("user's responsibility")
            || doc.to_lowercase().contains("cross-invocation rate"),
        "triggers doc must document that cross-invocation rate is the user's responsibility"
    );
}

#[test]
fn triggers_doc_documents_pr_title_format() {
    let doc = triggers_doc();
    assert!(
        doc.contains("[scanner-orch] re-derive:"),
        "triggers doc must document the PR title prefix `[scanner-orch] re-derive:`"
    );
    assert!(
        doc.contains("70 char") || doc.contains("70-char") || doc.contains("Length cap"),
        "triggers doc must document the 70-char title cap"
    );
}

#[test]
fn triggers_doc_documents_argv_list_discipline() {
    let doc = triggers_doc();
    // SEC-6: argv-list only.
    assert!(
        doc.contains("argv-list"),
        "triggers doc must document argv-list discipline for `gh pr create`"
    );
}

#[test]
fn triggers_doc_forbids_repo_flag() {
    let doc = triggers_doc();
    // SEC-8: NO --repo flag in gh pr create.
    assert!(
        doc.contains("NO `--repo` flag")
            || doc.contains("no `--repo`")
            || doc.to_lowercase().contains("no `--repo` flag"),
        "triggers doc must explicitly forbid the `--repo` flag (SEC-8)"
    );
}

#[test]
fn triggers_doc_forbids_merge_flags() {
    let doc = triggers_doc();
    // No --auto, --squash, --rebase, --admin, --merge, no gh pr merge.
    let forbidden = ["--auto", "--squash", "--rebase", "--merge"];
    for f in forbidden {
        assert!(
            doc.contains(f),
            "triggers doc must enumerate forbidden flag `{f}`"
        );
    }
    assert!(
        doc.contains("gh pr merge"),
        "triggers doc must forbid `gh pr merge` invocation"
    );
}

#[test]
fn triggers_doc_marked_stable() {
    let doc = triggers_doc();
    assert!(
        doc.contains("`stable`"),
        "triggers doc must be marked `stable`"
    );
}

// ---------------------------------------------------------------------------
// SKILL.md M5 section — re-derivation flow + PR creation.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_m5_method() {
    let skill = skill_md();
    assert!(
        skill.contains("Method (M5"),
        "SKILL.md must have a Method (M5) section"
    );
}

#[test]
fn skill_md_documents_re_derivation_flow() {
    let skill = skill_md();
    let flow_signals = [
        "Re-derivation trigger evaluation",
        "scanner-orch-rederivation-triggers.md",
        "no drift detected",
        "PR creation",
    ];
    for sig in flow_signals {
        assert!(
            skill.contains(sig),
            "SKILL.md M5 must document `{sig}`"
        );
    }
}

#[test]
fn skill_md_documents_argv_list_for_gh_pr_create() {
    let skill = skill_md();
    // Section-bounded check.
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    assert!(
        m5_section.contains("argv-list"),
        "SKILL.md M5 must document argv-list discipline for `gh pr create`"
    );
}

#[test]
fn skill_md_documents_no_repo_flag_in_m5() {
    let skill = skill_md();
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    assert!(
        m5_section.contains("--repo"),
        "SKILL.md M5 must explicitly mention the `--repo` flag (to forbid it per SEC-8)"
    );
    assert!(
        m5_section.to_lowercase().contains("no `--repo`")
            || m5_section.to_lowercase().contains("never"),
        "SKILL.md M5 must explicitly forbid the `--repo` flag"
    );
}

#[test]
fn skill_md_documents_dogfood_copy_not_symlink() {
    let skill = skill_md();
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    // ENG-6: dogfood subtree uses file-content copy, NOT symlinks.
    assert!(
        m5_section.contains("file-content copy") || m5_section.contains("NOT symlinks"),
        "SKILL.md M5 must document the copy-not-symlink discipline for the dogfood fixture (ENG-6)"
    );
    assert!(
        m5_section.contains("TempDir") || m5_section.contains("tempdir"),
        "SKILL.md M5 must document tempdir-based dogfood isolation"
    );
}

#[test]
fn skill_md_documents_no_auto_merge() {
    let skill = skill_md();
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    assert!(
        m5_section.contains("Auto-merge") || m5_section.contains("auto-merge"),
        "SKILL.md M5 anti-patterns must explicitly forbid auto-merge"
    );
}

#[test]
fn skill_md_documents_max_one_pr_per_invocation() {
    let skill = skill_md();
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    // ENG-4 framing.
    assert!(
        m5_section.contains("Max 1 PR per invocation")
            || m5_section.contains("max 1 PR per invocation")
            || m5_section.contains("exactly one"),
        "SKILL.md M5 must document the max-1-PR-per-invocation rule (ENG-4)"
    );
}

// ---------------------------------------------------------------------------
// Defensive-design framing carried into M5.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_m5_pr_body_no_threat_model_prose() {
    let skill = skill_md();
    let m5_start = skill.find("Method (M5").expect("M5 section must exist");
    let m5_section = &skill[m5_start..];
    // No content from threat-model prose flows into the PR body — same
    // template-skeleton discipline as M3's workflow YAML.
    assert!(
        m5_section.to_lowercase().contains("template-skeleton")
            || m5_section.to_lowercase().contains("manifest-derived"),
        "SKILL.md M5 must document that PR body uses template-skeleton with manifest-derived values only"
    );
}

// ---------------------------------------------------------------------------
// Regressions — M1 + M2 + M3 + M4 sections still present.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_m1_through_m4_sections_still_present() {
    let skill = skill_md();
    let prior_sections = [
        "Method (M1 — parser scaffold)",
        "Method (M2 — stack detection",
        "Method (M3",
        "Method (M4",
        "Threat-model parser scope rule",
        "Stack detection",
        "Workflow safety contract",
        "Manifest schema v1.0",
        "Preview-mode UX",
    ];
    for sec in prior_sections {
        assert!(
            skill.contains(sec),
            "SKILL.md must preserve prior milestone section `{sec}` after M5 additions"
        );
    }
}

#[test]
fn existing_references_sast_unmodified_by_m5() {
    let cases = [
        ("references/sast/threat-model-parser-contract.md", "tm-scanner-orchestration-abuse-1"),
        ("references/sast/scanner-orch-pinned-rules-sha.md", "40-character"),
        ("references/sast/stack-detection-contract.md", "polyglot"),
        ("references/sast/scanner-orch-workflow-template.yml", "pull_request"),
        ("references/sast/scanner-orch-action-shas.md", "actions/checkout"),
        ("references/sast/scanner-orch-manifest-schema.md", "defensive design"),
        ("references/sast/AUTHORING.md", "Trail of Bits"),
    ];
    for (path, sentinel) in cases {
        let content = read(&repo_root().join(path));
        assert!(
            content.contains(sentinel),
            "{path} appears modified — expected sentinel `{sentinel}` not found"
        );
    }
}
