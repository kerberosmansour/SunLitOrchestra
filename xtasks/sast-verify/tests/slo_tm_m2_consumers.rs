//! M2 structural-contract test (slo-threat-model runbook).
//!
//! Asserts the consumer read-side contract is wired into the canonical
//! portable critique/verify path:
//!
//! - Both `skills/slo-verify/SKILL.md` and `skills/slo-critique/SKILL.md`
//!   carry the read-side contract: read the frozen
//!   `<slug>-threat-model.slo.json`, HALT rather than re-derive abuse-case
//!   IDs, and treat `accepted_residual` as not-a-finding vs missing coverage.
//! - SEC-1: both specify the explicit `~~~text` literal-fence rule for
//!   `.slo.json` string fields (the rule, not merely the words "literal
//!   data").
//! - The degraded-mode (no file) vs hard-halt (invalid file) boundary is
//!   stated in both.
//! - ENG-1: the pre-existing `slo-critique` critique anchors survive the
//!   edit (additive-only) and the read-side block cites the schema doc so
//!   `sap_imp_m1_citations` enforces the path.
//! - F-ENG-6 lockstep: the SHA-256 of `slo-critique/SKILL.md` equals the
//!   pinned constant in `sap_imp_m5_agents.rs` (this test fails until the
//!   constant is updated in the same milestone as the SKILL.md edit).

use regex::Regex;
use sha2::{Digest, Sha256};
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

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

const VERIFY_SKILL: &str = "skills/slo-verify/SKILL.md";
const CRITIQUE_SKILL: &str = "skills/slo-critique/SKILL.md";
const SCHEMA_DOC_CITATION: &str = "references/security/threat-model-schema.md";

/// Phrases that constitute the read-side contract. Each must appear in BOTH
/// consumer SKILL.md files.
fn read_side_contract_phrases() -> Vec<&'static str> {
    vec![
        "-threat-model.slo.json",
        "halt",
        "re-derive",
        "accepted_residual",
        "missing coverage",
    ]
}

/// BDD `both_skills_carry_read_side_contract`.
#[test]
fn both_skills_carry_read_side_contract() {
    for skill_path in [VERIFY_SKILL, CRITIQUE_SKILL] {
        let body = read(skill_path);
        for phrase in read_side_contract_phrases() {
            assert!(
                body.contains(phrase),
                "{} is missing read-side contract phrase {:?}",
                skill_path,
                phrase
            );
        }
    }
}

/// BDD `instruction_shaped_field_not_executed` — SEC-1: the explicit fence
/// rule must be specified, not merely the words "literal data".
#[test]
fn both_skills_specify_the_fence_rule() {
    for skill_path in [VERIFY_SKILL, CRITIQUE_SKILL] {
        let body = read(skill_path);
        assert!(
            body.contains("~~~text"),
            "{}: SEC-1 requires the explicit ~~~text literal-fence rule for \
             .slo.json string fields, not just a 'literal data' description",
            skill_path
        );
        assert!(
            body.contains("never") && (body.contains("instruction") || body.contains("prompt")),
            "{}: must state .slo.json string fields are never interpreted as \
             instructions/prompt",
            skill_path
        );
    }
}

/// BDD `missing_json_uses_documented_degraded_mode` +
/// `invalid_json_hard_halts`.
#[test]
fn degraded_vs_hard_halt_boundary_is_stated() {
    for skill_path in [VERIFY_SKILL, CRITIQUE_SKILL] {
        let body = read(skill_path);
        assert!(
            body.contains("degraded mode") || body.contains("degraded-mode"),
            "{}: must state the documented degraded mode for an absent .slo.json",
            skill_path
        );
        assert!(
            body.contains("hard halt") || body.contains("hard-halt"),
            "{}: must state the hard halt for a schema-invalid .slo.json",
            skill_path
        );
    }
}

/// BDD `critique_edit_is_additive_only` (ENG-1): pre-existing critique
/// anchors survive; the read-side block cites the schema doc.
#[test]
fn critique_edit_is_additive_only() {
    let body = read(CRITIQUE_SKILL);
    assert!(
        body.contains("## Rotation order"),
        "ENG-1: slo-critique/SKILL.md lost the '## Rotation order' heading — \
         the edit must be additive, not a reflow"
    );
    for persona in ["CEO", "Eng lead", "Security", "Design"] {
        assert!(
            body.contains(persona),
            "ENG-1: slo-critique/SKILL.md lost persona anchor {:?} — the edit \
             must be additive",
            persona
        );
    }
    assert!(
        body.contains(SCHEMA_DOC_CITATION),
        "ENG-1: the read-side block must cite {} so sap_imp_m1_citations \
         enforces the path",
        SCHEMA_DOC_CITATION
    );
}

/// BDD `feng6_constant_in_lockstep` — the pinned constant must track the
/// current SKILL.md bytes. Fails until M2 updates the constant in the same
/// milestone as the SKILL.md edit.
#[test]
fn feng6_sha_constant_in_lockstep() {
    let critique_bytes =
        std::fs::read(workspace_root().join(CRITIQUE_SKILL)).expect("read slo-critique/SKILL.md");
    let mut hasher = Sha256::new();
    hasher.update(&critique_bytes);
    let actual: String = hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();

    let m5 = read("xtasks/sast-verify/tests/sap_imp_m5_agents.rs");
    let re = Regex::new(r#"CRITIQUE_SKILL_SHA256:\s*&str\s*=\s*"([0-9a-f]{64})""#).unwrap();
    let pinned = re
        .captures(&m5)
        .map(|c| c[1].to_string())
        .expect("CRITIQUE_SKILL_SHA256 constant not found in sap_imp_m5_agents.rs");

    assert_eq!(
        actual, pinned,
        "F-ENG-6 lockstep broken: slo-critique/SKILL.md SHA-256 ({actual}) does \
         not match the pinned constant ({pinned}). The constant MUST be updated \
         in the same milestone as the SKILL.md edit, with the recorded F-ENG-6 \
         amendment."
    );
}
