//! M1 structural-contract tests for secure execution controls.
//!
//! M1 adds a secure-construction pre-flight to `/slo-execute` so agents choose
//! secure building blocks before writing tests or code.

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
fn execute_skill_has_secure_construction_preflight_before_bdd() {
    let skill = read(repo_root().join("skills/slo-execute/SKILL.md"));
    let preflight = skill
        .find("Secure-construction pre-flight")
        .expect("/slo-execute must document the secure-construction pre-flight");
    let bdd = skill
        .find("Write BDD tests first")
        .expect("/slo-execute must still document BDD-first");

    assert!(
        preflight < bdd,
        "secure-construction pre-flight must happen before BDD/code writing"
    );
    assert!(skill.contains("references/secure-construction-preflight.md"));
}

#[test]
fn secure_construction_reference_exists_with_gap_routing() {
    let path = repo_root().join("skills/slo-execute/references/secure-construction-preflight.md");
    let body = read(&path);

    for needle in [
        "surface map",
        "SunLitSecurityLibraries",
        "/slo-sec-libs",
        "capability gap",
        "residual risk",
        "N/A - no new security-relevant surface",
    ] {
        assert!(
            body.contains(needle),
            "{} missing `{needle}`",
            path.display()
        );
    }
}

#[test]
fn secure_construction_reference_blocks_silent_hand_rolled_security() {
    let body = read(repo_root().join("skills/slo-execute/references/secure-construction-preflight.md"));
    for needle in [
        "hand-rolled crypto",
        "hand-rolled auth",
        "raw path",
        "string-built SQL",
        "shell argument",
        "explicit justification",
    ] {
        assert!(body.contains(needle), "missing guardrail `{needle}`");
    }
}

