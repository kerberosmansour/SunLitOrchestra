//! ticket-92 structural-contract test: /slo-ticket-plan must encode the
//! `secrets.<NAME>` → IAM role → trust-policy chase, and the ticket-contract
//! template must carry the matching Contract Block row in both copies.
//!
//! Source: https://github.com/kerberosmansour/SunLitOrchestra/issues/92
//! Source lesson: docs/slo/lessons/security-codex-findings-2026-05-19-m1.md
//!   "Mistakes made" + "Rules for the next milestone"
//!
//! The four assertions are BDD-first: they fail on the pre-edit baseline and
//! pass after the additive insertions in:
//!   - skills/slo-ticket-plan/SKILL.md (Method + Gates)
//!   - skills/slo-ticket-plan/references/ticket-contract-template_v_1.md
//!   - docs/slo/templates/ticket-contract-template_v_1.md

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

const SKILL: &str = "skills/slo-ticket-plan/SKILL.md";
const TEMPLATE_SKILL_COPY: &str =
    "skills/slo-ticket-plan/references/ticket-contract-template_v_1.md";
const TEMPLATE_DOCS_COPY: &str = "docs/slo/templates/ticket-contract-template_v_1.md";

const METHOD_TRIGGER_PHRASES: &[&str] = &[
    "secrets→role→trust-policy chase",
    "role-to-assume:",
    "trust policy",
];

const GATES_REFUSAL_PHRASE: &str =
    "Refuse a ticket contract that proposes extending or amending an IAM trust policy";

const TEMPLATE_ROW_LABEL: &str = "IAM secrets→role→trust-policy mapping";

fn extract_method_section(skill: &str) -> &str {
    let start = skill
        .find("## Method")
        .expect("SKILL.md missing `## Method` heading — refusing to assert without anchor");
    let after = &skill[start..];
    let end_offset = after[1..]
        .find("\n## ")
        .map(|i| i + 1)
        .unwrap_or(after.len());
    &after[..end_offset]
}

fn extract_gates_section(skill: &str) -> &str {
    let start = skill
        .find("## Gates")
        .expect("SKILL.md missing `## Gates` heading — refusing to assert without anchor");
    let after = &skill[start..];
    let end_offset = after[1..]
        .find("\n## ")
        .map(|i| i + 1)
        .unwrap_or(after.len());
    &after[..end_offset]
}

#[test]
fn method_step_present() {
    let skill = read(SKILL);
    let method = extract_method_section(&skill);
    for phrase in METHOD_TRIGGER_PHRASES {
        assert!(
            method.contains(phrase),
            "ticket-92: /slo-ticket-plan Method must contain the IAM chase trigger phrase \
             `{phrase}`. Issue #92 acceptance criterion: \
             SKILL.md Method requires the secrets→role→trust-policy chase whenever the file \
             allow-list/read-list includes IAM trust-policy JSON, AWS OIDC config, or a workflow \
             YAML with `role-to-assume:`. Source: docs/slo/tickets/ticket-92-iam-trust-policy-chase.md."
        );
    }
}

#[test]
fn gates_refusal_present() {
    let skill = read(SKILL);
    let gates = extract_gates_section(&skill);
    assert!(
        gates.contains(GATES_REFUSAL_PHRASE),
        "ticket-92: /slo-ticket-plan Gates must contain the exact IAM trust-policy refusal \
         phrase `{GATES_REFUSAL_PHRASE}`. Issue #92 acceptance criterion: \
         SKILL.md Gates refuses to ship a ticket contract that mentions extending a trust policy \
         without naming the exact role ARN(s) and the exact secrets.<NAME> → role mapping table. \
         Source: docs/slo/tickets/ticket-92-iam-trust-policy-chase.md."
    );
    let lower = gates.to_lowercase();
    assert!(
        lower.contains("secrets") && lower.contains("role") && lower.contains("trust polic"),
        "ticket-92: /slo-ticket-plan Gates refusal must reference the secrets→role→trust-policy \
         mapping; the words 'secrets', 'role', and 'trust polic[y/ies]' must all appear in the \
         Gates section. Found Gates section: {gates}"
    );
}

#[test]
fn template_row_present_in_both() {
    for path in [TEMPLATE_SKILL_COPY, TEMPLATE_DOCS_COPY] {
        let body = read(path);
        assert!(
            body.contains(TEMPLATE_ROW_LABEL),
            "ticket-92: ticket-contract template at `{path}` must contain a Contract Block row \
             labeled `{TEMPLATE_ROW_LABEL}`. Issue #92 acceptance criterion: \
             references/ticket-contract-template_v_1.md Contract Block gains a new row capturing \
             the IAM trust-policy chase requirement. Source: \
             docs/slo/tickets/ticket-92-iam-trust-policy-chase.md."
        );
    }
}

#[test]
fn templates_byte_identical() {
    let a = read(TEMPLATE_SKILL_COPY);
    let b = read(TEMPLATE_DOCS_COPY);
    assert_eq!(
        a, b,
        "ticket-92: the two ticket-contract template copies must remain byte-identical so the \
         skill-local copy (used post-install in target repos) cannot drift from the human-browsable \
         mirror under docs/slo/templates/. Drift would let one surface gain the IAM-chase row \
         while the other does not, reintroducing the ticket-180 failure mode."
    );
}
