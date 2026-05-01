//! M5 E2E: /slo-tla skill contract + auxiliary files.
//!
//! Tests that would invoke TLC directly require Java + the tla2tools jar,
//! which this crate does NOT fetch. A broken-mutex smoke test against TLC
//! is gated behind `#[ignore]` and run manually after the skill has been
//! bootstrapped once.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn skill_dir() -> PathBuf {
    repo_root().join("skills").join("slo-tla")
}

fn skill_body() -> String {
    fs::read_to_string(skill_dir().join("SKILL.md")).expect("slo-tla SKILL.md missing")
}

#[test]
fn tla_frontmatter_valid() {
    let body = skill_body();
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-tla"));
}

#[test]
fn tla_has_jvm_cascade() {
    let body = skill_body();
    // JVM check must come first and fail loud.
    assert!(body.contains("which java"), "must check for java via which");
    assert!(
        body.contains("brew install") || body.contains("adoptium") || body.contains("apt install"),
        "must provide install hint when java is missing"
    );
}

#[test]
fn tla_has_checksum_verify_step() {
    let body = skill_body();
    assert!(body.contains("sha") || body.contains("SHA-256"));
    assert!(
        body.contains("shasum") || body.contains("sha256sum"),
        "skill must show a concrete checksum-verify command"
    );
    // Explicit abort path on mismatch.
    assert!(
        body.contains("mismatch") || body.contains("MISMATCH") || body.contains("FAILS"),
        "skill must handle checksum mismatch explicitly"
    );
}

#[test]
fn tla_requires_bounds_declaration() {
    let body = skill_body();
    assert!(
        body.to_lowercase().contains("bound"),
        "skill must force bounds declaration on every verified claim"
    );
    assert!(
        body.contains("N=") || body.contains("N,M,K") || body.contains("N actors"),
        "skill must give concrete bound notation"
    );
}

#[test]
fn tla_requires_fairness_for_liveness() {
    let body = skill_body();
    assert!(
        body.to_lowercase().contains("fairness"),
        "skill must force fairness assumptions on liveness properties"
    );
}

#[test]
fn tla_has_apalache_hint_path() {
    let body = skill_body();
    assert!(
        body.to_lowercase().contains("apalache"),
        "skill must describe when to escalate to Apalache"
    );
}

#[test]
fn tla_cache_at_sldo_tla() {
    let body = skill_body();
    assert!(
        body.contains("~/.sldo/tla/") || body.contains(".sldo/tla"),
        "skill must use ~/.sldo/tla/ as the jar cache location"
    );
}

#[test]
fn tools_toml_has_required_fields() {
    let tools_path = skill_dir().join("tools.toml");
    let body = fs::read_to_string(&tools_path).expect("tools.toml missing");
    // TLC entry
    assert!(body.contains("[tlc]"));
    assert!(body.contains("version"));
    assert!(body.contains("url"));
    assert!(body.contains("sha256"));
    // The URL must be to a pinned release, not `latest`.
    assert!(
        !body.contains("/releases/latest"),
        "tools.toml must pin to a specific release tag, not `latest`"
    );
    // Apalache entry for lazy fallback.
    assert!(body.contains("[apalache]"));
}

#[test]
fn tla_template_file_exists() {
    let tmpl = skill_dir()
        .join("templates")
        .join("basic-state-machine.tla.tmpl");
    assert!(
        tmpl.exists(),
        "basic-state-machine template must be present"
    );
    let body = fs::read_to_string(&tmpl).unwrap();
    assert!(body.contains("MODULE"));
    assert!(body.contains("VARIABLES"));
    assert!(body.contains("Init"));
    assert!(body.contains("Next"));
}

#[test]
fn counterexample_translator_doc_exists() {
    let doc = skill_dir().join("counterexample-translator.md");
    assert!(doc.exists(), "counterexample-translator.md required");
    let body = fs::read_to_string(&doc).unwrap();
    // Must explicitly forbid the dump-state-vectors anti-pattern.
    assert!(body.to_lowercase().contains("anti-pattern") || body.contains("Anti-patterns"));
    assert!(
        body.contains("state vector") || body.contains("Don't dump"),
        "translator doc must call out the raw-state-dump anti-pattern"
    );
}

#[test]
fn tla_refuses_verified_without_bounds() {
    let body = skill_body();
    // The gate section must include this exact failure case.
    let lower = body.to_lowercase();
    assert!(
        lower.contains("bound is not stated")
            || lower.contains("bounds") && lower.contains("refuse"),
        "skill must refuse to mark verified when bounds are missing"
    );
}
