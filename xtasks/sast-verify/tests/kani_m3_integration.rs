//! M3 structural-contract test (kani-verification runbook).
//!
//! Asserts the integration seams are present and additive:
//!
//! - `/slo-architect` documents the `kani_required` frontmatter key with a
//!   default-when-absent of `false`.
//! - The v4 runbook template §5 carries the Kani proof-obligation sub-block.
//! - An existing overview lacking `kani_required` still parses (additivity).
//! - `/slo-execute` and `/slo-verify` carry their Kani-obligation hook prose
//!   (ENG-4 — all six edited files asserted, none silently dropped).
//! - `/slo-retro` refuses to close on a blank Kani Evidence-Log row.
//!
//! Prose-gate substrings are matched case-insensitively (kani-m2 lesson).

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read_lc(rel: &str) -> String {
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {rel}: {e}"))
        .to_lowercase()
}

#[test]
fn architect_documents_kani_required_with_default() {
    let t = read_lc("skills/slo-architect/SKILL.md");
    assert!(
        t.contains("kani_required"),
        "slo-architect must document the kani_required key"
    );
    assert!(
        t.contains("default when absent: `false`") || t.contains("default-when-absent is `false`"),
        "slo-architect must state kani_required defaults to false when absent (additivity)"
    );
}

#[test]
fn template_section5_has_kani_subblock() {
    let t = read_lc("docs/slo/templates/runbook-template_v_4_template.md");
    assert!(
        t.contains("kani proof obligation"),
        "v4 template §5 must carry the Kani proof-obligation sub-block"
    );
}

#[test]
fn existing_overview_without_key_still_parses() {
    // Additivity: an overview authored before kani_required must remain valid
    // YAML and simply lack the key (readers default it to false).
    let rel = "docs/slo/design/sast-rulegen-skill-pack-overview.md";
    let content = std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {rel}: {e}"));
    let fm = content
        .strip_prefix("---\n")
        .and_then(|s| s.split("\n---").next())
        .expect("overview missing frontmatter");
    let yaml: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(fm).expect("pre-existing overview frontmatter must still parse");
    assert!(
        yaml.as_mapping()
            .map(|m| !m.contains_key(serde_yaml_ng::Value::String("kani_required".into())))
            .unwrap_or(false),
        "this pre-existing overview should lack kani_required (proving the key is additive, not required)"
    );
}

#[test]
fn execute_skill_documents_kani_obligation_hook() {
    // ENG-4: execute hook prose must be present, not silently dropped.
    let t = read_lc("skills/slo-execute/SKILL.md");
    assert!(
        t.contains("kani-obligation") && t.contains("cargo kani"),
        "slo-execute must document the Kani-obligation hook (write #[cfg(kani)] harness, run cargo kani, remediate) — ENG-4"
    );
}

#[test]
fn verify_skill_documents_kani_scope_check() {
    // ENG-4: verify hook prose must be present.
    let t = read_lc("skills/slo-verify/SKILL.md");
    assert!(
        t.contains("kani harnesses") && t.contains("stated bounds"),
        "slo-verify must document the Kani scope check (harnesses green at stated bounds + scope honesty) — ENG-4"
    );
}

#[test]
fn retro_refuses_blank_kani_evidence() {
    let t = read_lc("skills/slo-retro/SKILL.md");
    assert!(
        t.contains("kani") && t.contains("blank") && t.contains("evidence"),
        "slo-retro must state it refuses to close on a blank Kani Evidence-Log row"
    );
}
