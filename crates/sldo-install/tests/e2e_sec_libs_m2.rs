//! M2 structural-contract tests for `/slo-sec-libs`.
//!
//! These tests assert the matcher contract. Runtime smoke checks are performed
//! manually against fixture runbook/catalog pairs because M2 is host-driven and
//! does not add a new executable matcher dependency.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use sldo_common::toolflags;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_path() -> PathBuf {
    repo_root().join("skills/slo-sec-libs")
}

fn unmatched_catalog_ids<'a>(
    catalog_ids: &[&'a str],
    matched_ids: &[&'a str],
    selected_ids: &[&'a str],
) -> Vec<&'a str> {
    let allowed: BTreeSet<_> = catalog_ids.iter().copied().collect();
    matched_ids
        .iter()
        .chain(selected_ids.iter())
        .copied()
        .filter(|id| !allowed.contains(id))
        .collect()
}

#[test]
fn methodology_m2_exists() {
    let methodology_path = skill_path().join("references/methodology-m2-matcher.md");
    let body = read(&methodology_path);
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-sec-libs-m2-matcher"));
    assert!(body.contains("# /slo-sec-libs M2 Matcher Methodology"));
}

#[test]
fn skill_dispatch_documents_match_mode() {
    let skill = read(&skill_path().join("SKILL.md"));
    assert!(skill.contains("--match <runbook.md> --catalog <catalog.json>"));
    assert!(skill.contains("methodology-m2-matcher.md"));
    assert!(skill.contains("do not file issues"));
}

#[test]
fn tiebreaker_rule_documented() {
    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("Specificity Score"));
    assert!(methodology.contains("more parametric evidence"));
    assert!(methodology.contains("preferred-by-specificity"));
}

#[test]
fn tie_disposition_documented() {
    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("disposition: \"tie\""));
    assert!(methodology.contains("selected_catalog_bom_ref is omitted"));
}

#[test]
fn conservative_tiebreaker_documented() {
    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("preferred-conservative"));
    assert!(methodology.contains("conservative tiebreaker applied"));
    assert!(methodology.contains("If comparability is unclear, use `tie`"));
}

#[test]
fn output_schema_contains_matched_unmatched_and_diagnostics() {
    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("\"matched\""));
    assert!(methodology.contains("\"unmatched\""));
    assert!(methodology.contains("\"diagnostics\""));
    assert!(methodology.contains("\"catalog_bom_ref\""));
    assert!(methodology.contains("\"selected_catalog_bom_ref\""));
}

#[test]
fn every_matched_entry_references_catalog_id() {
    let invalid = unmatched_catalog_ids(
        &["component:hulumi-auth", "component:slsl-argon2id"],
        &["component:slsl-argon2id"],
        &["component:slsl-argon2id"],
    );
    assert!(invalid.is_empty());
}

#[test]
fn fabricated_catalog_id_is_rejected_by_structural_guard() {
    let invalid = unmatched_catalog_ids(
        &["component:hulumi-auth", "component:slsl-argon2id"],
        &["component:made-up"],
        &["component:slsl-argon2id"],
    );
    assert_eq!(invalid, vec!["component:made-up"]);

    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("fabricated-catalog-id-refused"));
    assert!(methodology.contains("tm-slo-sec-libs-abuse-7"));
}

#[test]
fn empty_states_documented() {
    let methodology = read(&skill_path().join("references/methodology-m2-matcher.md"));
    assert!(methodology.contains("no controls to match"));
    assert!(methodology.contains("no candidate libraries"));
}

#[test]
fn m1_reader_contract_still_present() {
    let root = skill_path();
    let skill = read(&root.join("SKILL.md"));
    assert!(root.join("scripts/read-declarations.py").is_file());
    assert!(root.join("references/methodology-m1-reader.md").is_file());
    assert!(skill.contains("--read-declarations <path>"));
}

#[test]
fn sec_libs_tool_deny_flags_unchanged() {
    let flags = toolflags::sec_libs_deny_flags();
    let combined = flags.join(",");
    assert!(combined.contains("WebFetch"));
    assert!(combined.contains("WebSearch"));
}
