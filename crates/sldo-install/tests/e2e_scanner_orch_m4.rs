//! M4 structural-contract tests for the scanner-orchestration runbook.
//!
//! M4 lands the manifest schema v1.0 + initial-baseline preview-mode UX.
//! Tests assert the schema reference doc and SKILL.md document the contract
//! correctly. Runtime behavior (actual JSON emission, preview-mode stdin
//! interaction) is exercised via smoke tests + /slo-verify.

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
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_md() -> String {
    read(&repo_root().join("skills/slo-sast/SKILL.md"))
}

fn manifest_schema_doc() -> String {
    read(&repo_root().join("references/sast/scanner-orch-manifest-schema.md"))
}

// ---------------------------------------------------------------------------
// Manifest schema v1.0 — required fields documented.
// ---------------------------------------------------------------------------

#[test]
fn manifest_schema_doc_exists() {
    let doc = manifest_schema_doc();
    assert!(
        doc.len() > 800,
        "manifest schema doc is suspiciously short ({} bytes)",
        doc.len()
    );
}

#[test]
fn manifest_schema_documents_all_v1_fields() {
    let doc = manifest_schema_doc();
    let required_fields = [
        "schema_version",
        "generated_at",
        "generated_by_skill_version",
        "threat_model_path",
        "threat_model_sha",
        "semgrep_rules_sha",
        "semgrep_version",
        "detected_stack",
        "selection_strategy",
        "cwes_claimed",
        "cwes_actually_covered",
        "cwes_uncovered",
        "selected_rules",
    ];
    for field in required_fields {
        assert!(
            doc.contains(field),
            "manifest schema doc must document field `{field}`"
        );
    }
}

#[test]
fn manifest_schema_documents_per_rule_fields() {
    let doc = manifest_schema_doc();
    let per_rule_fields = [
        "selected_rules[].path",
        "selected_rules[].rule_id",
        "selected_rules[].source_sha",
        "selected_rules[].source",
        "selected_rules[].metadata_cwe",
        "selected_rules[].metadata_technology",
    ];
    for field in per_rule_fields {
        assert!(
            doc.contains(field),
            "manifest schema doc must document per-rule field `{field}`"
        );
    }
}

#[test]
fn manifest_schema_documents_validation_rules() {
    let doc = manifest_schema_doc();
    // Regex validation discipline must be documented.
    assert!(
        doc.contains(r"^CWE-\d+$"),
        "manifest schema must document CWE field regex validation"
    );
    assert!(
        doc.contains(r"^[0-9a-f]{40}$"),
        "manifest schema must document SHA field regex validation"
    );
}

#[test]
fn manifest_schema_documents_set_algebra() {
    let doc = manifest_schema_doc();
    // cwes_uncovered = cwes_claimed \ cwes_actually_covered must be documented
    // explicitly as the construction rule.
    assert!(
        doc.contains("cwes_uncovered = cwes_claimed \\ cwes_actually_covered"),
        "manifest schema must document the set-algebra rule for cwes_uncovered"
    );
}

#[test]
fn manifest_schema_marked_stable() {
    let doc = manifest_schema_doc();
    assert!(
        doc.contains("`stable`") || doc.contains("**`stable`**"),
        "manifest schema v1.0 must be marked `stable`"
    );
}

// ---------------------------------------------------------------------------
// Defensive-design framing — no overpromising language.
// ---------------------------------------------------------------------------

#[test]
fn manifest_schema_uses_defensive_design_framing() {
    let doc = manifest_schema_doc();
    // Must explicitly state "defensive design, not regulatory mandate".
    assert!(
        doc.to_lowercase().contains("defensive design"),
        "manifest schema must use the `defensive design` framing"
    );
    assert!(
        doc.to_lowercase().contains("not regulatory mandate")
            || doc.to_lowercase().contains("not a regulatory mandate"),
        "manifest schema must explicitly disclaim regulatory-mandate framing"
    );
}

#[test]
fn manifest_schema_pci_citation_correct() {
    let doc = manifest_schema_doc();
    // Must cite PCI DSS 6.2.3 (v4.0.1), NOT 6.3.2.
    assert!(
        doc.contains("6.2.3") || doc.contains("PCI DSS 6.2.3"),
        "manifest schema must cite PCI DSS 6.2.3 (v4.0.1)"
    );
    // 6.3.2 may appear ONLY in a context that explains the v3.2.1 → v4.0.1
    // renumbering or the SBOM-inventory distinction. Allow it but verify
    // there's no naked "PCI DSS 6.3.2" claim.
    let bad_claim = "PCI DSS 6.3.2 evidence";
    assert!(
        !doc.contains(bad_claim),
        "manifest schema must not make a naked `{bad_claim}` claim"
    );
}

#[test]
fn manifest_schema_avoids_overpromising_strings() {
    let doc = manifest_schema_doc();
    // The forbidden strings: must NOT appear except in the prohibition list itself.
    // Detect by checking for occurrences and excluding lines that begin with `❌`.
    let forbidden = ["PCI-compliant", "regulatory mandate"];
    for f in forbidden {
        // Count lines containing the forbidden phrase that don't start with ❌
        // (which marks the prohibition list).
        let positive_uses: Vec<_> = doc
            .lines()
            .filter(|l| l.contains(f))
            .filter(|l| {
                !l.contains("❌")
                    && !l.contains("not regulatory")
                    && !l.contains("not a regulatory")
            })
            .collect();
        assert!(
            positive_uses.is_empty(),
            "manifest schema must not positively use overpromising phrase `{f}` (offending: {positive_uses:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// SKILL.md — Manifest + Preview-Mode sections.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_documents_manifest_method() {
    let skill = skill_md();
    assert!(
        skill.contains("Method (M4"),
        "SKILL.md must have a Method (M4) section"
    );
    assert!(
        skill.contains("Manifest schema v1.0") || skill.contains("manifest.json"),
        "SKILL.md M4 must document manifest emission"
    );
    assert!(
        skill.contains("scanner-orch-manifest-schema.md"),
        "SKILL.md M4 must cite the manifest schema reference doc"
    );
}

#[test]
fn skill_md_documents_preview_mode() {
    let skill = skill_md();
    assert!(
        skill.contains("Preview-mode") || skill.contains("preview-mode"),
        "SKILL.md M4 must document preview-mode UX"
    );
    // Preview-mode discipline: detect first install vs re-derivation, gate on user input.
    assert!(
        skill.contains("First install") || skill.contains("first install"),
        "SKILL.md M4 must distinguish first-install path"
    );
    assert!(
        skill.contains("Re-derivation") || skill.contains("re-derivation"),
        "SKILL.md M4 must distinguish re-derivation path"
    );
}

#[test]
fn skill_md_documents_mixed_state_trigger() {
    let skill = skill_md();
    // ENG-3: pre-existing workflow / .semgrep.yml triggers preview even
    // without an existing manifest.
    assert!(
        skill.to_lowercase().contains("mixed pre-existing state")
            || skill.to_lowercase().contains("pre-existing")
                && skill.to_lowercase().contains("workflow"),
        "SKILL.md M4 must document the ENG-3 mixed-state trigger (pre-existing workflow / .semgrep.yml triggers preview)"
    );
}

#[test]
fn skill_md_documents_rollback_on_decline() {
    let skill = skill_md();
    assert!(
        skill.to_lowercase().contains("rollback"),
        "SKILL.md M4 must document the rollback contract on user-decline"
    );
}

#[test]
fn skill_md_documents_symlink_defense_at_manifest_writes() {
    let skill = skill_md();
    // SEC-1 variant: manifest write site also resists symlink traversal.
    let m4_section_start = skill
        .find("Method (M4")
        .expect("M4 section must exist for this test to be meaningful");
    let m4_through_end = &skill[m4_section_start..];
    assert!(
        m4_through_end.to_lowercase().contains("symlink"),
        "SKILL.md M4 section must document symlink defense at manifest write site"
    );
}

#[test]
fn skill_md_documents_overpromise_anti_pattern() {
    let skill = skill_md();
    let m4_section_start = skill.find("Method (M4").expect("M4 section must exist");
    let m4_through_end = &skill[m4_section_start..];
    assert!(
        m4_through_end.to_lowercase().contains("overpromising")
            || m4_through_end.to_lowercase().contains("defensive design")
            || m4_through_end.to_lowercase().contains("not regulatory"),
        "SKILL.md M4 section must document the no-overpromising anti-pattern (defensive design, not regulatory mandate)"
    );
}

// ---------------------------------------------------------------------------
// Regressions — M1 + M2 + M3 sections still present.
// ---------------------------------------------------------------------------

#[test]
fn skill_md_m1_m2_m3_sections_still_present() {
    let skill = skill_md();
    let prior_sections = [
        "Method (M1 — parser scaffold)",
        "Method (M2 — stack detection",
        "Method (M3",
        "Threat-model parser scope rule",
        "Stack detection",
        "Workflow safety contract",
    ];
    for sec in prior_sections {
        assert!(
            skill.contains(sec),
            "SKILL.md must preserve prior milestone section `{sec}` after M4 additions"
        );
    }
}

#[test]
fn existing_references_sast_unmodified_by_m4() {
    let cases = [
        (
            "references/sast/threat-model-parser-contract.md",
            "tm-scanner-orchestration-abuse-1",
        ),
        (
            "references/sast/scanner-orch-pinned-rules-sha.md",
            "40-character",
        ),
        ("references/sast/stack-detection-contract.md", "polyglot"),
        (
            "references/sast/scanner-orch-workflow-template.yml",
            "pull_request",
        ),
        (
            "references/sast/scanner-orch-action-shas.md",
            "actions/checkout",
        ),
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
