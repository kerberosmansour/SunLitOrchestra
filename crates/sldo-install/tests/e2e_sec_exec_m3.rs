//! M3 structural-contract tests for secure execution controls.
//!
//! M3 aligns `/slo-verify` Pass 4 with `/slo-sast` and `/slo-dast-tuner`.

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
fn verify_skill_documents_security_test_selector() {
    let skill = read(repo_root().join("skills/slo-verify/SKILL.md"));
    assert!(skill.contains("Security-test selector"));
    assert!(skill.contains("threat model"));
    assert!(skill.contains("touched surface"));
    assert!(skill.contains("/slo-dast-tuner"));
}

#[test]
fn pass4_dast_guidance_converges_on_zaprun() {
    let commands = read(repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(commands.contains("zaprun"));
    assert!(commands.contains("/slo-dast-tuner"));
    assert!(commands.contains("unauthenticated") && commands.contains("coverage failure"));

    for forbidden in [
        "zap-api-scan.py",
        "zap-baseline.py",
        "zap-full-scan.py",
        "dastardly:latest",
    ] {
        assert!(
            !commands.contains(forbidden),
            "Pass 4 must not teach direct `{forbidden}` DAST invocation"
        );
    }
}

#[test]
fn dast_tuner_cross_links_verify_selector() {
    let skill = read(repo_root().join("skills/slo-dast-tuner/SKILL.md"));
    assert!(skill.contains("Security-test selector"));
    assert!(skill.contains("skills/slo-verify/references/security-pass-commands.md"));
}
