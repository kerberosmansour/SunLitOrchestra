//! M3 structural-contract tests for the scanner-orchestration runbook.
//!
//! M3 lands the workflow YAML safety contract via a static template at
//! references/sast/scanner-orch-workflow-template.yml. The skill emits the
//! template verbatim with only action-SHA substitution. Template-correctness
//! (asserted here) implies emission-correctness — that's the architectural
//! defense against tm-scanner-orchestration-abuse-3.

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

fn workflow_template() -> String {
    read(&repo_root().join("references/sast/scanner-orch-workflow-template.yml"))
}

fn action_shas_doc() -> String {
    read(&repo_root().join("references/sast/scanner-orch-action-shas.md"))
}

// ---------------------------------------------------------------------------
// Workflow-template safety contract (the load-bearing assertions for M3).
// ---------------------------------------------------------------------------

#[test]
fn workflow_template_exists() {
    let tpl = workflow_template();
    assert!(
        tpl.len() > 500,
        "workflow template is suspiciously short ({} bytes)",
        tpl.len()
    );
}

#[test]
fn workflow_template_uses_pull_request_not_pull_request_target() {
    let tpl = workflow_template();
    assert!(
        tpl.contains("pull_request:") || tpl.contains("pull_request\n"),
        "workflow template must use `on: pull_request`"
    );
    // The hard ban — pull_request_target must NEVER appear except as part of
    // a comment explaining why it's banned.
    let lines_with_prt: Vec<_> = tpl
        .lines()
        .filter(|l| l.contains("pull_request_target"))
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    assert!(
        lines_with_prt.is_empty(),
        "workflow template MUST NOT use pull_request_target (load-bearing defense against tm-scanner-orchestration-abuse-3); offending lines: {lines_with_prt:?}"
    );
}

#[test]
fn workflow_template_has_empty_workflow_scope_permissions() {
    let tpl = workflow_template();
    // `permissions: {}` at workflow scope (top-level, not inside a job).
    // Detect via line position relative to `jobs:` — must appear before.
    let permissions_idx = tpl.find("permissions: {}");
    let jobs_idx = tpl.find("jobs:");
    assert!(
        permissions_idx.is_some(),
        "workflow template must declare `permissions: {{}}` at workflow scope"
    );
    if let (Some(p), Some(j)) = (permissions_idx, jobs_idx) {
        assert!(
            p < j,
            "workflow-scope `permissions: {{}}` must appear before `jobs:` (saw permissions at {p}, jobs at {j})"
        );
    }
}

#[test]
fn workflow_template_per_job_permissions_minimal() {
    let tpl = workflow_template();
    // contents: read for analysis. security-events: write for SARIF upload.
    assert!(
        tpl.contains("contents: read"),
        "workflow template must declare `contents: read` for the analysis job"
    );
    assert!(
        tpl.contains("security-events: write"),
        "workflow template must declare `security-events: write` for the SARIF upload"
    );
}

#[test]
fn workflow_template_uses_sha_placeholders_only() {
    let tpl = workflow_template();
    // Only two third-party `uses:` lines should be present, both pinned to
    // SHA placeholders that the skill substitutes from action-shas.md.
    let placeholders = ["{{CHECKOUT_SHA}}", "{{UPLOAD_SARIF_SHA}}"];
    for ph in placeholders {
        assert!(
            tpl.contains(ph),
            "workflow template must use the `{ph}` placeholder for SHA substitution"
        );
    }
    // No raw tag references on actions/checkout or upload-sarif.
    let bad_patterns = [
        "actions/checkout@v",
        "actions/checkout@main",
        "actions/checkout@master",
        "github/codeql-action/upload-sarif@v",
        "github/codeql-action/upload-sarif@main",
    ];
    for bad in bad_patterns {
        let lines: Vec<_> = tpl.lines().filter(|l| l.contains(bad)).collect();
        // Allow occurrences in comments (lines starting with `#`).
        let non_comment: Vec<_> = lines
            .iter()
            .filter(|l| !l.trim_start().starts_with('#'))
            .collect();
        assert!(
            non_comment.is_empty(),
            "workflow template MUST NOT use unpinned `{bad}` (offending: {non_comment:?})"
        );
    }
}

#[test]
fn workflow_template_checkout_has_fetch_depth_zero() {
    let tpl = workflow_template();
    // The actions/checkout step must have `fetch-depth: 0` (default 1 breaks
    // semgrep ci diff-aware scans per Semgrep KB).
    assert!(
        tpl.contains("fetch-depth: 0"),
        "workflow template must declare `fetch-depth: 0` on actions/checkout (default 1 breaks semgrep ci diff-aware scans)"
    );
}

#[test]
fn workflow_template_uses_semgrep_rules_env_var() {
    let tpl = workflow_template();
    assert!(
        tpl.contains("SEMGREP_RULES:"),
        "workflow template must use `SEMGREP_RULES` env var for Semgrep config"
    );
    // No --config flag in any step (CLI reference says it's "not supported in
    // ci mode" — the env-var path is the future-proof choice).
    let config_flag_lines: Vec<_> = tpl
        .lines()
        .filter(|l| l.contains("--config"))
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    assert!(
        config_flag_lines.is_empty(),
        "workflow template MUST NOT use the `--config` flag (offending: {config_flag_lines:?})"
    );
}

#[test]
fn workflow_template_no_secrets_in_analysis_job() {
    let tpl = workflow_template();
    // pull_request event isolates fork-PR contributors from secrets; no
    // `secrets.*` reference should appear in the analysis job. Note: a
    // SEMGREP_APP_TOKEN reference WOULD be a secret reference, intentionally
    // omitted from the template (stand-alone Semgrep, no AppSec Platform).
    let secret_refs: Vec<_> = tpl
        .lines()
        .filter(|l| l.contains("${{ secrets.") || l.contains("secrets."))
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    assert!(
        secret_refs.is_empty(),
        "workflow template MUST NOT reference secrets.* in the analysis job (offending: {secret_refs:?})"
    );
}

#[test]
fn workflow_template_no_autofix_flag() {
    let tpl = workflow_template();
    // --autofix is forbidden — defends against compromised-rule autofix
    // backdoors per tm-scanner-orchestration-abuse-2.
    let autofix_lines: Vec<_> = tpl
        .lines()
        .filter(|l| l.contains("--autofix"))
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    assert!(
        autofix_lines.is_empty(),
        "workflow template MUST NOT use `--autofix` (defends against compromised-rule autofix backdoors)"
    );
}

// ---------------------------------------------------------------------------
// Action-SHAs reference doc.
// ---------------------------------------------------------------------------

#[test]
fn action_shas_doc_exists() {
    let doc = action_shas_doc();
    assert!(
        doc.len() > 500,
        "action-SHAs doc is suspiciously short ({} bytes)",
        doc.len()
    );
}

#[test]
fn action_shas_doc_lists_both_required_actions() {
    let doc = action_shas_doc();
    assert!(
        doc.contains("actions/checkout"),
        "action-SHAs doc must list `actions/checkout`"
    );
    assert!(
        doc.contains("github/codeql-action/upload-sarif"),
        "action-SHAs doc must list `github/codeql-action/upload-sarif`"
    );
}

#[test]
fn action_shas_doc_documents_sha_only_enforcement() {
    let doc = action_shas_doc();
    assert!(
        doc.contains("40-character") || doc.contains("40-char"),
        "action-SHAs doc must document the 40-char SHA-only constraint"
    );
}

#[test]
fn action_shas_doc_documents_refresh_cadence() {
    let doc = action_shas_doc();
    assert!(
        doc.to_lowercase().contains("refresh cadence")
            || doc.to_lowercase().contains("90 days"),
        "action-SHAs doc must document a refresh cadence"
    );
}

#[test]
fn action_shas_doc_documents_placeholder_state() {
    let doc = action_shas_doc();
    let placeholder = "0000000000000000000000000000000000000000";
    assert!(
        doc.contains(placeholder) || doc.to_lowercase().contains("placeholder"),
        "action-SHAs doc must document the placeholder state for unbumped SHAs"
    );
}

// ---------------------------------------------------------------------------
// SKILL.md — Emission section + safety properties + CWE-list independence.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_emission_method() {
    let skill = skill_md();
    assert!(
        skill.contains("Method (M3"),
        "SKILL.md must have a Method (M3) section"
    );
    let emission_signals = [
        "Emission",
        "scanner-orch-workflow-template.yml",
        "scanner-orch-action-shas.md",
    ];
    for sig in emission_signals {
        assert!(
            skill.contains(sig),
            "SKILL.md M3 section must mention `{sig}`"
        );
    }
}

#[test]
fn skill_md_documents_workflow_safety_contract() {
    let skill = skill_md();
    let safety_signals = [
        "pull_request_target",  // explicitly named as forbidden
        "permissions:",          // permissions discipline mentioned
        "fetch-depth: 0",        // checkout depth mandate
        "SEMGREP_RULES",         // env var instead of --config
    ];
    for sig in safety_signals {
        assert!(
            skill.contains(sig),
            "SKILL.md M3 section must document workflow safety property `{sig}`"
        );
    }
}

#[test]
fn skill_md_documents_symlink_traversal_defense() {
    let skill = skill_md();
    // SEC-1: defense against symlink traversal at every emit site.
    assert!(
        skill.to_lowercase().contains("symlink"),
        "SKILL.md M3 section must document symlink-traversal defense (SEC-1)"
    );
    assert!(
        skill.contains("O_NOFOLLOW") || skill.contains("not a symlink"),
        "SKILL.md M3 must specify the symlink-rejection mechanism"
    );
}

#[test]
fn skill_md_documents_cwe_independence_of_workflow() {
    let skill = skill_md();
    // The workflow YAML is byte-identical across CWE-disjoint threat models.
    // Same applies to .semgrep.yml (per SEC-4 ask).
    assert!(
        skill.to_lowercase().contains("byte-identical")
            || skill.to_lowercase().contains("static skeleton")
            || skill.to_lowercase().contains("cwe-list independence"),
        "SKILL.md M3 section must document the architectural property that emitted workflow YAML is independent of CWE list"
    );
}

// ---------------------------------------------------------------------------
// Regressions — M1 + M2 contracts still in place.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_m1_m2_sections_still_present() {
    let skill = skill_md();
    let prior_sections = [
        "Method (M1 — parser scaffold)",
        "Method (M2 — stack detection",
        "Threat-model parser scope rule",
        "Stack detection",
        "Registry fetch",
        "Rule filter",
    ];
    for sec in prior_sections {
        assert!(
            skill.contains(sec),
            "SKILL.md must preserve prior milestone section `{sec}` after M3 additions"
        );
    }
}

#[test]
fn existing_references_sast_unmodified_by_m3() {
    let cases = [
        ("references/sast/threat-model-parser-contract.md", "tm-scanner-orchestration-abuse-1"),
        ("references/sast/scanner-orch-pinned-rules-sha.md", "40-character"),
        ("references/sast/stack-detection-contract.md", "polyglot"),
        ("references/sast/AUTHORING.md", "Trail of Bits"),
        ("references/sast/MIN-SEMGREP-VERSION.md", "Semgrep"),
    ];
    for (path, sentinel) in cases {
        let content = read(&repo_root().join(path));
        assert!(
            content.contains(sentinel),
            "{path} appears modified — expected sentinel `{sentinel}` not found"
        );
    }
}
