//! M5 structural-contract tests for Business Skill Improvements.
//!
//! Verifies additive artifact schema fields, cross-skill citations, predicate
//! immutability, and PR-only baseline refresh-loop configuration.

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

const ADVISOR_CITATIONS: &[(&str, &str)] = &[
    (
        "skills/slo-legal/SKILL.md",
        "references/biz/legal-intake-contract.md",
    ),
    (
        "skills/slo-accounting/SKILL.md",
        "references/biz/accounting-intake-contract.md",
    ),
    (
        "skills/slo-equity/SKILL.md",
        "references/biz/equity-intake-contract.md",
    ),
    (
        "skills/slo-fundraise/SKILL.md",
        "references/biz/fundraise-intake-contract.md",
    ),
    (
        "skills/slo-hire/SKILL.md",
        "references/biz/hire-intake-contract.md",
    ),
];

const GENERATOR_CITATIONS: &[(&str, &str)] = &[
    (
        "skills/slo-metrics/SKILL.md",
        "references/biz/saas-kpi-targets-baseline.md",
    ),
    (
        "skills/slo-pricing/SKILL.md",
        "references/biz/value-equation-pricing.md",
    ),
    (
        "skills/slo-sales-funnel/SKILL.md",
        "references/biz/outbound-conversion-baselines.md",
    ),
    (
        "skills/slo-product/SKILL.md",
        "references/biz/product-prioritization-frameworks.md",
    ),
    (
        "skills/slo-launch/SKILL.md",
        "references/biz/launch-success-thresholds.md",
    ),
    (
        "skills/slo-marketing/SKILL.md",
        "references/biz/uk-marketing-statute-anchors.md",
    ),
    (
        "skills/slo-talk-to-users/SKILL.md",
        "references/biz/mom-test-canonical-questions.md",
    ),
];

const NEW_OPTIONAL_FIELDS: &[&str] = &[
    "baseline_ref",
    "intake_summary",
    "gates_evaluation",
    "restated_and_confirmed",
    "restated_at",
    "agent_version",
    "agent_session_id",
    "conversation_turn_count",
    "intake_duration_seconds",
];

const PRE_M5_SCHEMA_FIELDS: &[&str] = &[
    "name",
    "created",
    "tier",
    "archetype",
    "skill",
    "mode",
    "mode_arg",
    "pii_scan_override",
    "tier_override_reason",
    "pecr_triage_completed",
    "pecr_triage_doc",
    "pecr_triage_blocker",
    "jurisdiction",
    "cost_baseline_ref",
    "triage_gate_passed",
    "gates_fired",
    "lawyer_review_recommended",
    "expires_or_review_by",
    "template_source",
    "template_license",
];

#[test]
fn artifact_schema_has_new_optional_fields() {
    let schema = read(&repo_root().join("references/biz/artifact-schema.md"));

    for field in NEW_OPTIONAL_FIELDS {
        let row = schema
            .lines()
            .find(|line| line.contains(&format!("`{field}`")))
            .unwrap_or_else(|| panic!("artifact-schema.md missing `{field}` row"));
        assert!(
            row.contains("optional"),
            "`{field}` must be documented as optional for backward compatibility; row was {row:?}"
        );
    }

    for required_phrase in [
        "path + retrieval-date",
        "F1-F6",
        "pass / fail / insufficient-info",
        "anti-pattern detector",
    ] {
        assert!(
            schema.contains(required_phrase),
            "artifact-schema.md missing M5 schema rationale `{required_phrase}`"
        );
    }
}

#[test]
fn schema_additive_only_preserves_existing_fields() {
    let schema = read(&repo_root().join("references/biz/artifact-schema.md"));

    for field in PRE_M5_SCHEMA_FIELDS {
        assert!(
            schema.contains(&format!("`{field}`")),
            "schema changes must be additive; existing field `{field}` disappeared"
        );
    }

    assert!(
        schema.contains("Exactly two permitted values: `confidential` \\| `public`"),
        "tier enum contract must remain unchanged"
    );
    assert!(
        schema.contains("Exactly one permitted value in v1: `uk`"),
        "jurisdiction enum contract must remain unchanged"
    );
}

#[test]
fn cross_skill_citation_test() {
    for (skill, contract) in ADVISOR_CITATIONS {
        let body = read(&repo_root().join(skill));
        assert!(
            body.contains(contract),
            "{skill} must cite its conversational intake contract `{contract}`"
        );
        assert!(
            body.contains("references/biz/uk-regulator-enumeration.md"),
            "{skill} must cite the M1 closed regulator enumeration"
        );
        assert!(
            body.contains("Restate-and-confirm"),
            "{skill} must preserve restate-and-confirm discipline"
        );
    }

    for (skill, baseline) in GENERATOR_CITATIONS {
        let body = read(&repo_root().join(skill));
        assert!(
            body.contains(baseline),
            "{skill} must cite generator baseline `{baseline}`"
        );
        assert!(
            body.contains("baseline_ref:"),
            "{skill} must show `baseline_ref:` in output frontmatter"
        );
    }
}

#[test]
fn triage_gate_predicate_set_unchanged_from_m1() {
    let body = read(&repo_root().join("references/biz/triage-gate.md"));
    let mut ids = body
        .lines()
        .filter_map(|line| {
            let start = line.find("`gate-")?;
            let rest = &line[start + 1..];
            let end = rest.find('`')?;
            Some(rest[..end].to_string())
        })
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();

    assert_eq!(
        ids,
        vec![
            "gate-1-regulated",
            "gate-2-deal-value-over-5k",
            "gate-3-counterparty-has-lawyer-or-their-paper",
            "gate-4-gdpr-document",
        ],
        "the four hard-block predicate IDs are immutable"
    );
}

#[test]
fn refresh_loop_opens_pr_not_auto_merge() {
    let body = read(&repo_root().join(".sldo/refresh-loop.toml"));

    for required in [
        "name = \"biz-kpi-baseline-refresh\"",
        "cadence = \"annual\"",
        "open_pr = true",
        "auto_merge = false",
        "max_prs_per_run = 1",
        "gh_pr_create",
    ] {
        assert!(
            body.contains(required),
            "refresh-loop.toml missing `{required}`"
        );
    }

    for forbidden in [
        "auto_merge = true",
        "merge = true",
        "gh pr merge",
        "--auto",
        "--squash",
        "--rebase",
        "--admin",
        "--merge",
    ] {
        assert!(
            !body.contains(forbidden),
            "refresh loop must be PR-only and must not contain `{forbidden}`"
        );
    }
}
