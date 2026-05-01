//! M1 structural-contract tests for biz-skill-pack Runbook B1.
//!
//! Verifies:
//! - `/slo-talk-to-users` SKILL.md frontmatter, generator archetype, no advisor predicate citations.
//! - `references/biz/artifact-schema.md` documents the new `archetype` and `mode_arg` keys.
//! - `skills/slo-verify/SKILL.md` Pass 4 documents the PII-pattern scan with at least three regex types.
//! - `references/biz/` still NOT discovered as a skill.

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

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

#[test]
fn slo_talk_to_users_skill_md_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-talk-to-users/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(
        frontmatter.contains("name: slo-talk-to-users"),
        "frontmatter must declare `name: slo-talk-to-users`"
    );
    assert!(
        frontmatter.contains("description:"),
        "frontmatter must contain a description"
    );

    // The skill body must declare archetype: generator (the canonical line) somewhere.
    assert!(
        skill.contains("Generator pattern") || skill.contains("archetype: generator"),
        "slo-talk-to-users SKILL.md must declare itself as a generator (looked for `Generator pattern` or `archetype: generator`)"
    );

    // Generator skills MUST NOT cite the four advisor predicate IDs in any
    // operative-policy context. We allow at most ONE mention per ID (e.g., a
    // forwarding pointer to /slo-legal); more than that suggests the skill is
    // misclassifying itself.
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(
            count <= 1,
            "generator skill `slo-talk-to-users` cites predicate `{pid}` {count} times; generators should reference advisor predicates at most once (forwarding only). Predicate citation is the advisor cluster's contract."
        );
    }
}

#[test]
fn artifact_schema_documents_archetype_and_mode_arg() {
    let schema = read(&repo_root().join("references/biz/artifact-schema.md"));
    assert!(
        schema.contains("`archetype`"),
        "artifact-schema.md must document the `archetype` key"
    );
    assert!(
        schema.contains("`advisor`") && schema.contains("`generator`"),
        "artifact-schema.md must document both `advisor` and `generator` enum values for archetype"
    );
    assert!(
        schema.contains("`mode_arg`"),
        "artifact-schema.md must document the `mode_arg` key"
    );
    assert!(
        schema.contains("`pii_scan_override`"),
        "artifact-schema.md must document the `pii_scan_override` boolean"
    );
    assert!(
        schema.contains("`tier_override_reason`"),
        "artifact-schema.md must document the `tier_override_reason` string"
    );
}

#[test]
fn slo_verify_pass_4_documents_pii_scan() {
    let verify = read(&repo_root().join("skills/slo-verify/SKILL.md"));

    // The Pass 4 section must include the PII-scan addition.
    assert!(
        verify.contains("PII-pattern scan") || verify.contains("PII pattern scan"),
        "slo-verify SKILL.md Pass 4 must document the PII-pattern scan"
    );

    // The scan must explicitly list at least three regex types.
    let regex_types = ["Email addresses", "National Insurance", "sort code"];
    for rt in &regex_types {
        assert!(
            verify.contains(rt),
            "slo-verify Pass 4 PII-scan must enumerate regex type `{rt}`"
        );
    }

    // The override mechanism must be documented.
    assert!(
        verify.contains("pii_scan_override"),
        "slo-verify Pass 4 must document the `pii_scan_override` frontmatter override mechanism"
    );

    // Scan scope must be explicit: docs/biz-public/ only, not docs/biz/.
    assert!(
        verify.contains("docs/biz-public/"),
        "slo-verify Pass 4 PII-scan must declare scope = `docs/biz-public/`"
    );
}

#[test]
fn slo_verify_pass_4_existing_substeps_unchanged() {
    let verify = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    // The existing Pass 4 sub-steps from slo-security-embedding M4 must remain.
    let preserved_signals = [
        "Stack detection",
        "Tool-optional rule",
        "Tool-error vs. finding",
        "DAST conditional",
        "Bug-found flow",
    ];
    for s in &preserved_signals {
        assert!(
            verify.contains(s),
            "slo-verify Pass 4 must preserve existing sub-step `{s}` (regression on slo-security-embedding M4)"
        );
    }
}

#[test]
fn slo_talk_to_users_outputs_to_confidential_tier() {
    let skill = read(&repo_root().join("skills/slo-talk-to-users/SKILL.md"));
    assert!(
        skill.contains("docs/biz/users/"),
        "slo-talk-to-users must document `docs/biz/users/` as the output dir"
    );
    assert!(
        skill.contains("tier: confidential"),
        "slo-talk-to-users must declare `tier: confidential` in the output frontmatter"
    );
    // Founder repo discipline: gitignore + write-time warning.
    assert!(
        skill.contains(".gitignore"),
        "slo-talk-to-users must reference founder repo `.gitignore` discipline"
    );
    assert!(
        skill.contains("WRITE-TIME WARNING") || skill.contains("write-time warning"),
        "slo-talk-to-users must document the write-time warning"
    );
}

#[test]
fn slo_talk_to_users_documents_mom_test_discipline() {
    let skill = read(&repo_root().join("skills/slo-talk-to-users/SKILL.md"));
    assert!(
        skill.contains("Mom Test"),
        "slo-talk-to-users must cite Mom Test discipline (no leading questions)"
    );
    // The skill must enumerate at least one anti-pattern.
    let anti_patterns = ["leading", "Wouldn't it be great", "How much would you pay"];
    let count = anti_patterns.iter().filter(|p| skill.contains(**p)).count();
    assert!(
        count >= 2,
        "slo-talk-to-users must enumerate at least 2 anti-patterns from Mom Test discipline (found {count})"
    );
}

#[test]
fn slo_talk_to_users_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-talk-to-users/SKILL.md"));
    assert!(
        skill.contains("v1 supports UK only"),
        "slo-talk-to-users must reuse the canonical UK-only error from jurisdiction-uk.md"
    );
}

#[test]
fn references_biz_dir_still_not_discovered_after_b1_m1() {
    let skills_dir = repo_root().join("skills");
    let references_biz = repo_root().join("references/biz");
    assert!(skills_dir.is_dir());
    assert!(references_biz.is_dir());
    assert!(!skills_dir.join("biz").exists());
    assert!(!skills_dir.join("_biz-shared").exists());
    assert!(
        skills_dir
            .join("slo-talk-to-users")
            .join("SKILL.md")
            .exists(),
        "skills/slo-talk-to-users/SKILL.md must exist"
    );
}
