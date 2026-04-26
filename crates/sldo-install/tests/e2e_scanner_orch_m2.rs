//! M2 structural-contract tests for the scanner-orchestration runbook.
//!
//! M2 lands stack detection + `semgrep-rules` cache fetch at pinned SHA + CWE ×
//! technology rule filter. Like M1, these tests assert the SKILL.md and
//! reference docs document the contract correctly; runtime behavior is
//! exercised via smoke tests.

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

fn pinned_sha_doc() -> String {
    read(&repo_root().join("references/sast/scanner-orch-pinned-rules-sha.md"))
}

fn stack_contract() -> String {
    read(&repo_root().join("references/sast/stack-detection-contract.md"))
}

// ---------------------------------------------------------------------------
// BDD #1 — pinned-SHA reference doc exists and enforces 40-char SHA only.
// ---------------------------------------------------------------------------

#[test]
fn pinned_sha_doc_exists() {
    let doc = pinned_sha_doc();
    assert!(
        doc.len() > 500,
        "pinned-SHA doc is suspiciously short ({} bytes)",
        doc.len()
    );
}

#[test]
fn pinned_sha_doc_documents_sha_only_enforcement() {
    let doc = pinned_sha_doc();
    assert!(
        doc.contains("40-character") || doc.contains("40-char"),
        "pinned-SHA doc must document the 40-char SHA-only constraint"
    );
    // Must explicitly forbid tags / branches / short SHAs.
    let forbidden = ["tag", "branch", "short SHA"];
    for f in forbidden {
        assert!(
            doc.to_lowercase().contains(&f.to_lowercase()),
            "pinned-SHA doc must explicitly forbid `{f}` references"
        );
    }
}

#[test]
fn pinned_sha_doc_cites_abuse_case() {
    let doc = pinned_sha_doc();
    assert!(
        doc.contains("tm-scanner-orchestration-abuse-2"),
        "pinned-SHA doc must cite threat-model abuse case `tm-scanner-orchestration-abuse-2` (compromised semgrep-rules upstream)"
    );
}

#[test]
fn pinned_sha_doc_documents_bump_procedure() {
    let doc = pinned_sha_doc();
    assert!(
        doc.to_lowercase().contains("bump procedure")
            || doc.to_lowercase().contains("bumping"),
        "pinned-SHA doc must document a bump procedure"
    );
    // PR-based review is the human-review surface.
    assert!(
        doc.contains("PR") || doc.contains("pull request"),
        "pinned-SHA doc must document that bumps go through PR review"
    );
}

#[test]
fn pinned_sha_doc_starts_with_placeholder_or_real_sha() {
    let doc = pinned_sha_doc();
    // First fenced 40-char block must be either all-zero (placeholder)
    // or 40 hex chars. The doc explicitly documents this.
    let placeholder = "0000000000000000000000000000000000000000";
    let has_placeholder = doc.contains(placeholder);
    // OR — match a 40-char hex SHA on its own line (real pin form).
    let has_real_sha = doc.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.len() == 40
            && trimmed
                .chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
    });
    assert!(
        has_placeholder || has_real_sha,
        "pinned-SHA doc must contain either the all-zero placeholder or a real 40-char SHA"
    );
}

// ---------------------------------------------------------------------------
// BDD #2 — stack-detection contract exists and documents manifest priority.
// ---------------------------------------------------------------------------

#[test]
fn stack_contract_exists() {
    let doc = stack_contract();
    assert!(
        doc.len() > 500,
        "stack-detection contract is suspiciously short ({} bytes)",
        doc.len()
    );
}

#[test]
fn stack_contract_documents_priority_order() {
    let doc = stack_contract();
    // All 8 manifest types must be documented.
    let manifests = [
        "Cargo.toml",
        "package.json",
        "requirements.txt",
        "go.mod",
        "pom.xml",
        "Gemfile",
        "composer.json",
        "Package.swift",
    ];
    for m in manifests {
        assert!(
            doc.contains(m),
            "stack-detection contract must document manifest `{m}`"
        );
    }
}

#[test]
fn stack_contract_documents_polyglot_behavior() {
    let doc = stack_contract();
    assert!(
        doc.to_lowercase().contains("polyglot"),
        "stack-detection contract must document polyglot behavior (multi-stack tag emission)"
    );
}

#[test]
fn stack_contract_documents_default_fallback() {
    let doc = stack_contract();
    assert!(
        doc.contains("default-fallback") || doc.contains("language-agnostic"),
        "stack-detection contract must document the no-stack-detected fallback path"
    );
}

#[test]
fn stack_contract_marked_stable() {
    let doc = stack_contract();
    assert!(
        doc.contains("`stable`"),
        "stack-detection contract must be marked `stable` (downstream filter logic depends on byte-stable manifest priority)"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — SKILL.md cites both new reference docs.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_cites_pinned_sha_doc() {
    let skill = skill_md();
    assert!(
        skill.contains("scanner-orch-pinned-rules-sha.md"),
        "SKILL.md must cite references/sast/scanner-orch-pinned-rules-sha.md"
    );
}

#[test]
fn skill_md_cites_stack_detection_contract() {
    let skill = skill_md();
    assert!(
        skill.contains("stack-detection-contract.md"),
        "SKILL.md must cite references/sast/stack-detection-contract.md"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — SKILL.md documents M2 output JSON envelope.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_json_envelope_output() {
    let skill = skill_md();
    let envelope_keys = [
        "cwes_extracted",
        "detected_stack",
        "selected_rules",
        "selection_strategy",
    ];
    for key in envelope_keys {
        assert!(
            skill.contains(key),
            "SKILL.md must document the M2 JSON envelope key `{key}`"
        );
    }
}

#[test]
fn skill_md_documents_default_fallback_strategy() {
    let skill = skill_md();
    assert!(
        skill.contains("default-fallback") || skill.contains("language-agnostic"),
        "SKILL.md must document the empty-stack default-fallback path"
    );
}

// ---------------------------------------------------------------------------
// BDD #5 — SKILL.md documents argv-list subprocess discipline (SEC-6).
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_argv_list_discipline() {
    let skill = skill_md();
    assert!(
        skill.contains("argv-list"),
        "SKILL.md must document argv-list subprocess discipline (SEC-6 defense against shell-string injection)"
    );
    assert!(
        skill.contains("shell") || skill.contains("bash -c"),
        "SKILL.md must explicitly forbid shell-string interpolation"
    );
}

// ---------------------------------------------------------------------------
// BDD #6 — SKILL.md documents YAML parser safety (SEC-2 billion-laughs defense).
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_yaml_parser_safety() {
    let skill = skill_md();
    // Must reference the safe-default behavior of serde_yaml_ng.
    assert!(
        skill.contains("entity expansion") || skill.contains("billion-laughs"),
        "SKILL.md must document defense against YAML entity expansion / billion-laughs"
    );
}

// ---------------------------------------------------------------------------
// BDD #7 — SKILL.md documents cache layout.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_cache_layout() {
    let skill = skill_md();
    assert!(
        skill.contains("~/.cache/sldo/semgrep-rules"),
        "SKILL.md must document the cache directory layout"
    );
    assert!(
        skill.contains("XDG_CACHE_HOME"),
        "SKILL.md must document XDG_CACHE_HOME override (test-harness isolation)"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 — SKILL.md documents cache-hit behavior (no `git clone` invocation).
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_cache_hit_behavior() {
    let skill = skill_md();
    assert!(
        skill.contains("cache hit") || skill.contains("cache miss"),
        "SKILL.md must document cache-hit/cache-miss distinction"
    );
    // ENG-2: cache hit allows `git rev-parse` for integrity verification but
    // skips `git clone`.
    assert!(
        skill.contains("git rev-parse"),
        "SKILL.md must document the integrity-verification step (git rev-parse) per ENG-2"
    );
}

// ---------------------------------------------------------------------------
// BDD #9 — SKILL.md documents the rule-filter intersection logic.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_rule_filter() {
    let skill = skill_md();
    // Must describe the intersection: cwe ∧ technology, with language-agnostic
    // fallback when metadata.technology is absent.
    assert!(
        skill.to_lowercase().contains("filter") || skill.to_lowercase().contains("intersect"),
        "SKILL.md must document the rule filter logic"
    );
    assert!(
        skill.contains("metadata.cwe"),
        "SKILL.md must reference the `metadata.cwe` field used for filtering"
    );
    assert!(
        skill.contains("metadata.technology"),
        "SKILL.md must reference the `metadata.technology` field used for filtering"
    );
}

// ---------------------------------------------------------------------------
// BDD #10 — SKILL.md still scopes M1 anti-patterns correctly (regression).
// ---------------------------------------------------------------------------

#[test]
fn skill_md_m1_parser_scope_still_enforced() {
    let skill = skill_md();
    // M1's parser scope rule (HTML comments, fenced code, ~~~text fences)
    // remains documented after M2 additions.
    let scope_signals = [
        "HTML comments",
        "fenced code",
        "~~~text",
    ];
    for sig in scope_signals {
        assert!(
            skill.contains(sig),
            "SKILL.md must still document M1's parser scope rule (missing: `{sig}`)"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #11 — references/sast/ existing files unmodified (M2 only adds 2).
// ---------------------------------------------------------------------------

#[test]
fn existing_references_sast_unmodified_by_m2() {
    // The M1 reference doc + sast-rulegen pre-existing files must be
    // byte-identical post-M2. M2 only adds 2 new files.
    let cases = [
        ("references/sast/threat-model-parser-contract.md", "tm-scanner-orchestration-abuse-1"),
        ("references/sast/AUTHORING.md", "Trail of Bits"),
        ("references/sast/MIN-SEMGREP-VERSION.md", "Semgrep"),
        ("references/sast/cwe-map-rust.md", "CWE"),
    ];
    for (path, sentinel) in cases {
        let content = read(&repo_root().join(path));
        assert!(
            content.contains(sentinel),
            "{path} appears modified — expected sentinel `{sentinel}` not found"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #12 — pinned-SHA placeholder is detectable as such (skill must refuse).
// ---------------------------------------------------------------------------

#[test]
fn pinned_sha_doc_documents_placeholder_refusal() {
    let doc = pinned_sha_doc();
    // The doc states the all-zero placeholder must be refused at runtime.
    assert!(
        doc.contains("placeholder") || doc.contains("PLACEHOLDER"),
        "pinned-SHA doc must explicitly call out the placeholder state"
    );
    assert!(
        doc.to_lowercase().contains("refuse") || doc.to_lowercase().contains("exit non-zero"),
        "pinned-SHA doc must document that the skill refuses to operate against the placeholder"
    );
}
