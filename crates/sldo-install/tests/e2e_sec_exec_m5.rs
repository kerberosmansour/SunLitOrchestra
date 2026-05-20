//! M5 structural-contract tests for secure execution controls.
//!
//! M5 dogfoods the loop and pins the upstream gap-handling evidence.

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

#[test]
fn dogfood_fixture_exists_with_rust_and_pulumi_surfaces() {
    let root = repo_root().join("tests/fixtures/secure-execution-controls");
    assert!(root.join("README.md").is_file());
    assert!(root.join("rust-boundary.md").is_file());
    assert!(root.join("pulumi-typescript.ts").is_file());
    let readme = read(root.join("README.md"));
    assert!(readme.contains("Rust boundary"));
    assert!(readme.contains("Pulumi TypeScript"));
}

#[test]
fn dogfood_report_records_matches_gaps_and_upstream_changes() {
    let report = read(repo_root().join("docs/slo/verify/sec-exec-dogfood.md"));
    for needle in [
        "matched:",
        "secure_boundary",
        "@hulumi/baseline.aws.SecureBucket",
        "gap:",
        "kerberosmansour/SunLitSecurityLibraries",
        "kerberosmansour/hulumi",
        "upstream issue",
        "upstream change status",
        "applied",
        "N/A - no smoke service",
    ] {
        assert!(report.contains(needle), "dogfood report missing `{needle}`");
    }
}

#[test]
fn loops_doc_mentions_secure_construction_loop() {
    let loops = read(repo_root().join("docs/LOOPS-ENGINEERING.md"));
    assert!(loops.contains("Secure-construction loop"));
    assert!(loops.contains("/slo-execute"));
    assert!(loops.contains("/slo-sec-libs"));
}
