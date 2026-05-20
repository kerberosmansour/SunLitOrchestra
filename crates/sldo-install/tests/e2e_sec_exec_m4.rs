//! M4 structural-contract tests for secure execution controls.
//!
//! M4 adds the Pulumi TypeScript / Hulumi secure-IaC lane.

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
fn cloud_iac_reference_exists_with_hulumi_and_pulumi_rules() {
    let body =
        read(repo_root().join("skills/slo-execute/references/cloud-iac-secure-construction.md"));
    for needle in [
        "Pulumi TypeScript",
        "Hulumi explicit or detected",
        "@hulumi/baseline.aws.SecureBucket",
        "HulumiHardeningPack",
        "pulumi.runtime.setMocks",
        "Policy as Code",
        "no plaintext secrets in state",
        "preview evidence",
        "drift evidence",
    ] {
        assert!(
            body.contains(needle),
            "cloud-IaC reference missing `{needle}`"
        );
    }
}

#[test]
fn cloud_threat_model_hands_off_to_secure_iac_lane() {
    let skill = read(repo_root().join("skills/slo-cloud-threat-model/SKILL.md"));
    assert!(skill.contains("secure-IaC lane"));
    assert!(skill.contains("cloud-iac-secure-construction.md"));
}

#[test]
fn secure_construction_matrix_covers_cloud_platforms() {
    let matrix = read(repo_root().join("skills/slo-plan/references/secure-construction-matrix.md"));
    for needle in ["AWS", "GitHub", "Cloudflare", "OIDC", "SecureBucket"] {
        assert!(
            matrix.contains(needle),
            "matrix missing cloud/platform `{needle}`"
        );
    }
}
