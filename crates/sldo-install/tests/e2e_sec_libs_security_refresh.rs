//! Structural tests for the post-M5 `/slo-sec-libs` security-doc refresh.
//!
//! These tests pin the security properties that issue #4 asked to refresh
//! after Phase 4 shipped.

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

fn security_md() -> String {
    read(&repo_root().join("SECURITY.md"))
}

fn threat_model() -> String {
    read(&repo_root().join("docs/slo/design/slo-security-embedding-threat-model.md"))
}

fn deprecated_owner_name() -> String {
    ["SunLit", "SecureLibraries"].join("")
}

#[test]
fn security_md_documents_sec_libs_reader_boundary() {
    let doc = security_md();
    assert!(doc.contains("## /slo-sec-libs"));
    assert!(doc.contains("Declaration reader trust boundary"));
    assert!(doc.contains("CycloneDX 1.6"));
    assert!(doc.contains("1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f"));
    assert!(doc.contains("strict-jsonschema"));
    assert!(doc.contains("Refuse symlink path segments"));
    assert!(doc.contains("10 MiB"));
    assert!(doc.contains("200-level JSON nesting cap"));
    assert!(doc.contains("NFKC"));
}

#[test]
fn security_md_documents_sec_libs_matcher_and_filing_discipline() {
    let doc = security_md();
    assert!(doc.contains("Matcher evidence discipline"));
    assert!(doc.contains("catalog_bom_ref"));
    assert!(doc.contains("MUST NOT invent capabilities"));
    assert!(doc.contains("Filing discipline"));
    assert!(doc.contains("kerberosmansour/slo-security-intake"));
    assert!(doc.contains("kerberosmansour/hulumi"));
    assert!(doc.contains("kerberosmansour/SunLitSecurityLibraries"));
    assert!(doc.contains("--file-upstream --upstream-dir <path>"));
    assert!(doc.contains("explicit per-issue confirmation"));
    assert!(doc.contains("no `--repo` flag"));
    assert!(doc.contains("no `gh auth login`"));
    assert!(doc.contains("40 issues per session per hour"));
    assert!(doc.contains("deferred-pending-confirmation"));
}

#[test]
fn threat_model_has_sec_libs_phase4_stride_surface() {
    let doc = threat_model();
    assert!(doc.contains("Updated 2026-05-06 after `/slo-sec-libs` M5 dogfood"));
    assert!(doc.contains("| `/slo-sec-libs` declaration reader + matcher + filer (Phase 4) |"));
    assert!(doc.contains("strict schema validation + symlink refusal + catalog-grounded matching"));
    assert!(doc.contains("10 MiB cap"));
    assert!(doc.contains("200-depth cap"));
    assert!(doc.contains("catalog_bom_ref"));
    assert!(doc.contains("no `--repo`"));
    assert!(doc.contains("per-issue confirmation"));
}

#[test]
fn threat_model_has_sec_libs_abuse_cases() {
    let doc = threat_model();
    for row_id in [
        "tm-slo-sec-abuse-5",
        "tm-slo-sec-abuse-6",
        "tm-slo-sec-abuse-7",
        "tm-slo-sec-abuse-8",
    ] {
        assert!(
            doc.contains(row_id),
            "missing /slo-sec-libs abuse case `{row_id}`"
        );
    }
    assert!(doc.contains("strict reader schema"));
    assert!(doc.contains("close-but-not-exact rows become `unmatched`"));
    assert!(doc.contains("deferred-pending-confirmation"));
}

#[test]
fn threat_model_maps_sec_libs_controls_and_residuals() {
    let doc = threat_model();
    assert!(
        doc.contains("`/slo-sec-libs` declaration reader schema + size/depth/symlink/NFKC checks")
    );
    assert!(doc.contains("`/slo-sec-libs` catalog-grounded matcher (`catalog_bom_ref` required)"));
    assert!(doc.contains("`/slo-sec-libs` confirmation-gated filing and no `--repo` discipline"));
    assert!(doc.contains("`/slo-sec-libs` dogfood report"));
    assert!(doc.contains("Capability-match poisoning via target-runbook prose"));
    assert!(doc.contains("Deferred M5 capability-gap filings"));
    assert!(doc.contains("docs/sec-libs-dogfood-2026-05-06.md"));
}

#[test]
fn security_refresh_uses_canonical_library_owner() {
    let combined = format!("{}\n{}", security_md(), threat_model());
    assert!(combined.contains("SunLitSecurityLibraries"));
    let deprecated = deprecated_owner_name();
    assert!(!combined.contains(&deprecated));
    assert!(!combined.contains(&format!("kerberosmansour/{deprecated}")));
}
