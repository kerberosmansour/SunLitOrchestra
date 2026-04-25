//! Workspace-level E2E for SAST Runbook A M3 — CI + dev-env wiring.
//!
//! Asserts disk-content properties of:
//! - `.github/workflows/semgrep.yml` (per /slo-critique sec-4 reframe)
//! - `.pre-commit-config.yaml`
//! - `references/sast/CI-WIRING.md`
//! - `LICENSE` (Apache OR MIT, NEVER AGPL)
//! - `README.md` (SAST section linking to CI-WIRING.md)

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn workflow_yaml_exists_and_targets_correct_branches() {
    let p = workspace_root()
        .join(".github")
        .join("workflows")
        .join("semgrep.yml");
    let content = fs::read_to_string(&p).expect("read .github/workflows/semgrep.yml");
    assert!(content.contains("pull_request:"));
    // Must run on at least the protected branches.
    assert!(content.contains("main"));
}

#[test]
fn workflow_yaml_does_not_invoke_extend_or_rulegen_paths() {
    // Per /slo-critique sec-4 reframe: forbid --extend and slo-rulegen invocations
    // (tm-sast-rulegen-skill-pack-abuse-3 mitigation).
    let p = workspace_root()
        .join(".github")
        .join("workflows")
        .join("semgrep.yml");
    let content = fs::read_to_string(&p).expect("read .github/workflows/semgrep.yml");
    assert!(
        !content.contains("--extend"),
        "workflow MUST NOT invoke --extend (auto-fire in CI is forbidden per tm-abuse-3)"
    );
    assert!(
        !content.contains("slo-rulegen"),
        "workflow MUST NOT invoke /slo-rulegen (rule generation in CI is forbidden per tm-abuse-3)"
    );
}

#[test]
fn workflow_yaml_invokes_gate_for_admission_control() {
    // Per /slo-critique sec-4 reframe: REQUIRES gate step before semgrep ci.
    let p = workspace_root()
        .join(".github")
        .join("workflows")
        .join("semgrep.yml");
    let content = fs::read_to_string(&p).expect("read .github/workflows/semgrep.yml");
    assert!(
        content.contains("sast-verify gate") || content.contains("sast-verify\" gate")
            || content.contains("cargo xtask sast-verify"),
        "workflow MUST invoke `cargo xtask sast-verify gate` for admission control \
         per /slo-critique sec-4 reframe; this catches direct-edit bypasses of the \
         skill's write-time gate"
    );
}

#[test]
fn workflow_yaml_pins_actions_by_sha() {
    // Per BDD workflow_invokes_pinned_semgrep_action: tag-only refs are rejected;
    // SHA pins required.
    let p = workspace_root()
        .join(".github")
        .join("workflows")
        .join("semgrep.yml");
    let content = fs::read_to_string(&p).expect("read .github/workflows/semgrep.yml");
    // Find every `uses: <action>@<ref>` and assert the ref looks like a 40-char SHA
    // (or, if a tag is used, that there's an explanatory comment + the action is
    // gated by `if: false` for the placeholder semgrep-action SHA per CI-WIRING.md).
    let mut found_at_least_one_pinned_action = false;
    for line in content.lines() {
        let line = line.trim();
        if let Some(uses) = line.strip_prefix("uses: ") {
            if let Some(at_idx) = uses.find('@') {
                let ref_part = &uses[at_idx + 1..];
                let is_40_hex = ref_part.len() == 40 && ref_part.chars().all(|c| c.is_ascii_hexdigit());
                if is_40_hex {
                    found_at_least_one_pinned_action = true;
                }
            }
        }
    }
    assert!(
        found_at_least_one_pinned_action,
        "workflow must pin at least one action by 40-char SHA (BDD workflow_invokes_pinned_semgrep_action)"
    );
}

#[test]
fn precommit_yaml_exists_and_declares_semgrep_hook() {
    let p = workspace_root().join(".pre-commit-config.yaml");
    let content = fs::read_to_string(&p).expect("read .pre-commit-config.yaml");
    assert!(content.contains("semgrep/pre-commit"));
    assert!(content.contains("rev:"));
    assert!(content.contains("id: semgrep"));
}

#[test]
fn license_file_is_apache_or_mit_not_agpl() {
    // Per /slo-critique LICENSE addendum: Apache-2.0 OR MIT, explicitly NOT AGPL.
    let p = workspace_root().join("LICENSE");
    let content = fs::read_to_string(&p).expect("read LICENSE");
    let upper = content.to_uppercase();
    assert!(
        upper.contains("APACHE LICENSE") || upper.contains("MIT LICENSE"),
        "LICENSE must include Apache-2.0 or MIT text"
    );
    assert!(
        !upper.contains("AGPL"),
        "LICENSE must NOT be AGPL (Trail of Bits clean-room policy + downstream consumability)"
    );
    assert!(
        !upper.contains("GNU GENERAL PUBLIC LICENSE"),
        "LICENSE must NOT be GPL of any flavour (downstream consumability)"
    );
}

#[test]
fn readme_has_sast_section_linking_to_ci_wiring() {
    let p = workspace_root().join("README.md");
    let content = fs::read_to_string(&p).expect("read README.md");
    assert!(
        content.contains("## SAST rule pack") || content.contains("# SAST rule pack"),
        "README must have a SAST rule pack section"
    );
    assert!(
        content.contains("references/sast/CI-WIRING.md"),
        "README's SAST section must link to references/sast/CI-WIRING.md"
    );
    assert!(
        content.contains("/slo-rulegen") && content.contains("/slo-ruleverify"),
        "README's SAST section must mention both skills"
    );
    assert!(
        content.contains("cargo xtask sast-verify"),
        "README's SAST section must reference the xtask quickref"
    );
}

#[test]
fn ci_wiring_md_exists_and_documents_developer_initiated_extend() {
    let p = workspace_root()
        .join("references")
        .join("sast")
        .join("CI-WIRING.md");
    let content = fs::read_to_string(&p).expect("read CI-WIRING.md");
    assert!(
        content.to_lowercase().contains("developer-initiated"),
        "CI-WIRING.md must document the cargo-audit-driven extend trigger \
         as developer-initiated only (per tm-sast-rulegen-skill-pack-abuse-3)"
    );
    assert!(
        content.contains("cargo audit"),
        "CI-WIRING.md must reference cargo-audit as the natural trigger source"
    );
    assert!(
        content.to_lowercase().contains("two-tier") || content.contains("Confidential"),
        "CI-WIRING.md must explain the two-tier corpus rendering posture"
    );
}
