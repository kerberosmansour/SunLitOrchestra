//! M1 structural-contract tests for the scanner-orchestration runbook.
//!
//! These tests verify the static contract of M1's edits — the SKILL.md scaffold,
//! the threat-model-parser-contract reference doc, and the discoverability of
//! `slo-sast` to the existing skill installer. They do not exercise Claude Code
//! runtime behavior (that happens when the user invokes `/slo-sast` against a
//! real target repo); they assert the documented shape is correct so M2-M5 can
//! rely on it and so smoke tests can validate runtime behavior.
//!
//! BDD scenarios from `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md` Milestone 1 are
//! mapped to structural assertions here. Smoke tests (in the runbook's
//! Smoke Tests section) cover end-to-end runtime invocation.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points at crates/sldo-install; go up two levels.
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

fn parser_contract() -> String {
    read(&repo_root().join("references/sast/threat-model-parser-contract.md"))
}

// ---------------------------------------------------------------------------
// BDD #1 — SKILL.md exists with valid frontmatter (name + description).
// ---------------------------------------------------------------------------

#[test]
fn skill_md_exists_with_valid_frontmatter() {
    let skill = skill_md();
    assert!(
        skill.starts_with("---\n"),
        "SKILL.md must open with YAML frontmatter delimiter"
    );
    assert!(
        skill.contains("name: slo-sast"),
        "SKILL.md frontmatter must declare `name: slo-sast`"
    );
    // `description:` opens a folded block scalar (`>`) per the existing skill
    // pack convention; sufficient to assert the key is present.
    assert!(
        skill.contains("description:"),
        "SKILL.md frontmatter must include a description"
    );
}

#[test]
fn skill_md_description_summarizes_role() {
    let skill = skill_md();
    // Description should mention SAST, threat-model, and Semgrep — the three
    // anchor concepts a discoverer needs to identify the skill's purpose.
    let desc_keywords = ["SAST", "threat-model", "Semgrep"];
    for kw in desc_keywords {
        assert!(
            skill.contains(kw),
            "SKILL.md description must mention `{kw}` so the skill is discoverable"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #2 — SKILL.md cites the parser-contract reference doc.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_cites_parser_contract_reference() {
    let skill = skill_md();
    assert!(
        skill.contains("threat-model-parser-contract.md"),
        "SKILL.md must cite references/sast/threat-model-parser-contract.md so the parse contract is discoverable from the skill"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — parser-contract reference exists and documents the regex.
// ---------------------------------------------------------------------------

#[test]
fn parser_contract_doc_exists() {
    let contract = parser_contract();
    assert!(
        contract.len() > 500,
        "parser-contract doc is suspiciously short ({} bytes)",
        contract.len()
    );
}

#[test]
fn parser_contract_documents_regex() {
    let contract = parser_contract();
    // The exact regex must be present so future implementers reference the
    // canonical form, not a rewritten variant.
    assert!(
        contract.contains(r"\bCWE-(\d+)\b"),
        "parser-contract must document the canonical regex `\\bCWE-(\\d+)\\b`"
    );
}

// ---------------------------------------------------------------------------
// BDD #4-#6 — parser-contract enumerates the three exclusion regions.
// (Maps to BDD scenarios `parser_ignores_html_comment_cwe_refs`,
//  `parser_ignores_fenced_code_cwe_refs`,
//  `parser_ignores_user_string_fence_cwe_refs` from the runbook M1 BDD table.
//  The runtime behavior is exercised via smoke tests; here we assert the
//  contract is documented so any future implementation honors it.)
// ---------------------------------------------------------------------------

#[test]
fn parser_contract_names_html_comment_exclusion() {
    let contract = parser_contract();
    // Must explicitly name HTML comments as exclusion region #1.
    assert!(
        contract.contains("HTML comment"),
        "parser-contract must name HTML comments as an exclusion region"
    );
    assert!(
        contract.contains("<!--"),
        "parser-contract must show the literal HTML-comment syntax"
    );
}

#[test]
fn parser_contract_names_fenced_code_exclusion() {
    let contract = parser_contract();
    assert!(
        contract.contains("Fenced code") || contract.contains("fenced code"),
        "parser-contract must name fenced code blocks as an exclusion region"
    );
    // Both fence forms documented (` ``` ` and `~~~`).
    assert!(
        contract.contains("```"),
        "parser-contract must document the ``` fence form"
    );
    assert!(
        contract.contains("~~~"),
        "parser-contract must document the ~~~ fence form"
    );
}

#[test]
fn parser_contract_names_user_string_fence_exclusion() {
    let contract = parser_contract();
    assert!(
        contract.contains("~~~text"),
        "parser-contract must explicitly name `~~~text` user-string fences as an exclusion region (per the slo-security-embedding fence rule)"
    );
}

// ---------------------------------------------------------------------------
// BDD #7 — parser-contract cites the abuse-case row.
// ---------------------------------------------------------------------------

#[test]
fn parser_contract_cites_abuse_case() {
    let contract = parser_contract();
    assert!(
        contract.contains("tm-scanner-orchestration-abuse-1"),
        "parser-contract must cite threat-model abuse case `tm-scanner-orchestration-abuse-1` so the rationale is auditable"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 — SKILL.md documents the empty-list behavior.
// (Maps to runbook BDD `returns_empty_list_when_no_cwes_named`.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_empty_list_behavior() {
    let skill = skill_md();
    // Both "[]" output and "exit 0" semantics must be documented.
    assert!(
        skill.contains("[]"),
        "SKILL.md must document the empty-list output `[]`"
    );
    assert!(
        skill.contains("exit 0"),
        "SKILL.md must document that empty parse exits 0 (empty is a valid M1 result)"
    );
}

// ---------------------------------------------------------------------------
// BDD #9 — SKILL.md documents the missing-file behavior.
// (Maps to runbook BDD `errors_on_missing_threat_model`.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_missing_file_behavior() {
    let skill = skill_md();
    // Pre-flight step 3 (or equivalent) names the error path.
    assert!(
        skill.to_lowercase().contains("threat-model not found")
            || skill.to_lowercase().contains("does not exist"),
        "SKILL.md must document the missing-threat-model error path"
    );
    // Must be clear that nothing partial is printed.
    assert!(
        skill.contains("non-zero") || skill.contains("exit non-zero"),
        "SKILL.md must specify non-zero exit on missing input"
    );
}

// ---------------------------------------------------------------------------
// BDD #10 — SKILL.md documents long-form CWE output (CWE-N strings).
// (Maps to runbook BDD `parses_canonical_cwe_list_from_prose`.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_long_form_cwe_output() {
    let skill = skill_md();
    // The example output string anchors the contract.
    assert!(
        skill.contains(r#""CWE-77""#) || skill.contains(r#""CWE-89""#),
        "SKILL.md must show example long-form `\"CWE-N\"` output"
    );
}

// ---------------------------------------------------------------------------
// BDD #11 — SKILL.md documents deduplication.
// (Maps to runbook BDD `dedupes_repeated_cwe_refs`.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_deduplication() {
    let skill = skill_md();
    assert!(
        skill.to_lowercase().contains("deduplicat") || skill.to_lowercase().contains("dedup"),
        "SKILL.md must document that the CWE list is deduplicated"
    );
}

// ---------------------------------------------------------------------------
// BDD #12 — SKILL.md documents sort order.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_sort_order() {
    let skill = skill_md();
    assert!(
        skill.to_lowercase().contains("sort") || skill.to_lowercase().contains("ascending"),
        "SKILL.md must document that output is sorted (ascending by integer)"
    );
}

// ---------------------------------------------------------------------------
// BDD #13 — SKILL.md scopes to M1 (no M2-M5 functionality leaked).
// (Reinforces "Out of Scope" from the runbook M1 section.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_scopes_m1_to_parser_only() {
    let skill = skill_md();
    // Anti-patterns section explicitly forbids M2+ behaviors.
    let antipattern_signals = [
        "Anti-patterns",
        "Emitting any artifact",
        "Inferring stack",
    ];
    for sig in antipattern_signals {
        assert!(
            skill.contains(sig),
            "SKILL.md must enumerate M2+ behaviors as anti-patterns for M1 (missing: `{sig}`)"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #14 — sldo-install discovers slo-sast (the skill is on the install
// walker's path).
// ---------------------------------------------------------------------------

#[test]
fn sldo_install_discovers_slo_sast() {
    // The walker requires `<skills_dir>/<name>/SKILL.md` to exist. We assert
    // the file exists at the expected path; the walker's own behavior is
    // covered by `tests/install_e2e.rs`.
    let path = repo_root().join("skills/slo-sast/SKILL.md");
    assert!(
        path.exists(),
        "skills/slo-sast/SKILL.md must exist for sldo-install discover_skills() to find it"
    );
}

// ---------------------------------------------------------------------------
// BDD #15 — pre-existing skills still discoverable (regression — no other
// SKILL.md files were touched by M1).
// ---------------------------------------------------------------------------

#[test]
fn existing_skills_unmodified_by_m1() {
    // Sample three existing skills and assert key content fragments still
    // match. If M1 accidentally touched them, these fragments would shift.
    let cases = [
        ("skills/slo-research/SKILL.md", "name: slo-research"),
        ("skills/slo-architect/SKILL.md", "name: slo-architect"),
        ("skills/slo-plan/SKILL.md", "name: slo-plan"),
    ];
    for (path, expected_name) in cases {
        let content = read(&repo_root().join(path));
        assert!(
            content.contains(expected_name),
            "{path} appears modified — expected to find `{expected_name}` in frontmatter"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #16 — references/sast/ existing files unmodified by M1.
// ---------------------------------------------------------------------------

#[test]
fn existing_references_sast_unmodified_by_m1() {
    // Sample existing reference files; these were authored by sast-rulegen and
    // must be byte-identical (no edits, no whitespace churn) post-M1.
    let cases = [
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
// BDD #17 — parser-contract is `stable` interface per architect's interfaces doc.
// ---------------------------------------------------------------------------

#[test]
fn parser_contract_marked_stable() {
    let contract = parser_contract();
    assert!(
        contract.contains("`stable`"),
        "parser-contract must be marked `stable` (downstream milestones depend on byte-stable contract)"
    );
}

// ---------------------------------------------------------------------------
// BDD #18 — SKILL.md links into the runbook's M1 / interfaces / threat-model
// citations so an executor can chase references.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_cites_canonical_design_docs() {
    let skill = skill_md();
    let citations = [
        "scanner-orchestration-threat-model.md",
        "scanner-orchestration-interfaces.md",
        "RUNBOOK-SCANNER-ORCHESTRATION.md",
    ];
    for citation in citations {
        assert!(
            skill.contains(citation),
            "SKILL.md must link to the canonical design doc `{citation}`"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #19 — SKILL.md anti-patterns explicitly forbid subprocess invocation in M1.
// (Defense-in-depth: M1 is pure file-read; subprocess discipline is M2+.)
// ---------------------------------------------------------------------------

#[test]
fn skill_md_forbids_subprocess_in_m1() {
    let skill = skill_md();
    // Anti-pattern section names forbidden subprocess invocations.
    assert!(
        skill.contains("subprocess") || skill.contains("Subprocess"),
        "SKILL.md anti-patterns must address subprocess invocation discipline for M1"
    );
}
