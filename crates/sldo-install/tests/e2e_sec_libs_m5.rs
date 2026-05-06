//! M5 structural-contract tests for `/slo-sec-libs`.
//!
//! These tests pin the dogfood report shape and preserve the M1-M4
//! confirmation-gated filing discipline.

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

fn report_path() -> PathBuf {
    repo_root().join("docs/sec-libs-dogfood-2026-05-06.md")
}

fn report() -> String {
    read(&report_path())
}

fn skill_path() -> PathBuf {
    repo_root().join("skills/slo-sec-libs")
}

fn skill() -> String {
    read(&skill_path().join("SKILL.md"))
}

#[test]
fn dogfood_report_exists_with_frontmatter() {
    let path = report_path();
    assert!(path.is_file(), "missing {}", path.display());
    let report = report();
    assert!(report.starts_with("---\n"));
    assert!(report.contains("title: \"slo-sec-libs dogfood 2026-05-06\""));
    assert!(
        report.contains("target_runbook: \"docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md\"")
    );
    assert!(report.contains("target_milestone: \"M3\""));
    assert!(report.contains("filed_issue_status: \"deferred-pending-confirmation\""));
}

#[test]
fn dogfood_report_has_candidate_shortlist() {
    let report = report();
    assert!(report.contains("## Candidate Shortlist"));
    assert!(report.contains("RUNBOOK-SLO-SECURITY-EMBEDDING.md"));
    assert!(report.contains("RUNBOOK-SAST-RULEGEN-A.md"));
    assert!(report.contains("RUNBOOK-SCANNER-ORCHESTRATION.md"));
    assert!(report.contains("security_libs_required: true"));
    assert!(report.contains("security_libs_required: false"));
}

#[test]
fn dogfood_report_has_reader_evidence() {
    let report = report();
    assert!(report.contains("## M1 Reader Result"));
    assert!(report.contains("1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f"));
    assert!(report.contains("c8ef1b206ff8d06fcdc373118b69084056c18b40ad8feef4b1a334b0ae857e5b"));
    assert!(report.contains("0de8f573392692e0c18c7c5462e154fff2578e8489cb4f6c8ebfb0505f12bdb9"));
    assert!(report.contains("c29d75d4903838c51d35497d3e9bb78d8161c3b9"));
    assert!(report.contains("ac3b4ccc641cbe4f12107196de9237a1e5503ab5"));
    assert!(report.contains("4 components, 2 claims"));
    assert!(report.contains("11 components, 11 claims"));
    assert!(report.contains("refused them because `/tmp` is a symlink"));
}

#[test]
fn dogfood_report_references_target_m3() {
    let report = report();
    assert!(report.contains("## Target Rows"));
    assert!(report.contains("M3, `/slo-critique` security persona rewrite"));
    assert!(report.contains("tm-slo-sec-abuse-3"));
    assert!(report.contains("m3-req-class-schema"));
    assert!(report.contains("m3-req-variant-analysis"));
    assert!(report.contains("m3-abuse-prompt-boundary"));
}

#[test]
fn dogfood_report_has_matched_section_with_catalog_refs() {
    let report = report();
    assert!(report.contains("## Matched"));
    assert!(report.contains("matched:"));
    for bom_ref in [
        "component:security_core",
        "component:secure_boundary",
        "component:security_events",
    ] {
        assert!(
            report.contains(bom_ref),
            "missing matched catalog_bom_ref `{bom_ref}`"
        );
    }
    assert!(report.contains("catalog_bom_ref"));
    assert!(report.contains("claim:security_core:capabilities"));
    assert!(report.contains("claim:secure_boundary:capabilities"));
    assert!(report.contains("claim:security_events:capabilities"));
}

#[test]
fn dogfood_report_has_unmatched_section() {
    let report = report();
    assert!(report.contains("## Unmatched"));
    assert!(report.contains("unmatched:"));
    assert!(report.contains("gap-agent-prompt-boundary"));
    assert!(report.contains("gap-variant-analysis-schema"));
    assert!(report.contains("desired_capability"));
    assert!(report.contains("agent-prompt-injection-boundary"));
    assert!(report.contains("variant-analysis-result-schema"));
}

#[test]
fn dogfood_report_has_filed_status_without_unconfirmed_live_url() {
    let report = report();
    assert!(report.contains("## Filed"));
    assert!(report.contains("filed:"));
    assert!(report.contains("N/A - deferred-pending-confirmation"));
    assert!(report.contains("Live filing requires explicit per-issue confirmation"));
    assert!(report.contains("No `gh issue create` command was run"));
    assert!(report.contains("kerberosmansour/slo-security-intake"));
    assert!(report.contains("kerberosmansour/SunLitSecurityLibraries"));
}

#[test]
fn dogfood_report_preserves_canonical_security_libraries_name() {
    let report = report();
    let deprecated_name = ["SunLit", "SecureLibraries"].join("");
    let deprecated_repo = format!("kerberosmansour/{deprecated_name}");
    assert!(report.contains("SunLitSecurityLibraries"));
    assert!(report.contains("Legacy secure-libraries owner spelling was not used"));
    assert!(!report.contains(&deprecated_name));
    assert!(!report.contains(&deprecated_repo));
}

#[test]
fn m1_through_m4_deliverables_still_present() {
    let root = skill_path();
    let skill = skill();
    assert!(skill.contains("--read-declarations <path>"));
    assert!(skill.contains("--match <runbook.md> --catalog <catalog.json>"));
    assert!(skill.contains("--file-gaps <m2-output.json> --intake-dir <path>"));
    assert!(skill.contains("--file-upstream --upstream-dir <path>"));
    assert!(root.join("scripts/read-declarations.py").is_file());
    assert!(root.join("references/methodology-m1-reader.md").is_file());
    assert!(root.join("references/methodology-m2-matcher.md").is_file());
    assert!(root.join("references/capability-gap-schema.md").is_file());
    assert!(root
        .join("references/upstream-filing-discipline.md")
        .is_file());
}

#[test]
fn sec_libs_tool_deny_flags_unchanged() {
    let flags = toolflags::sec_libs_deny_flags();
    let combined = flags.join(",");
    assert!(combined.contains("WebFetch"));
    assert!(combined.contains("WebSearch"));
}
