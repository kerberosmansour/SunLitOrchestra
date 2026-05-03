//! M4 structural-contract tests for Business Skill Improvements.
//!
//! Verifies KPI / heuristic baseline provenance and generator skill citations.

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

const BASELINE_FILES: &[&str] = &[
    "references/biz/saas-kpi-targets-baseline.md",
    "references/biz/outbound-conversion-baselines.md",
    "references/biz/product-prioritization-frameworks.md",
    "references/biz/value-equation-pricing.md",
    "references/biz/mom-test-canonical-questions.md",
    "references/biz/launch-success-thresholds.md",
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

#[test]
fn baseline_files_exist_with_source_verified_rows() {
    for rel in BASELINE_FILES {
        let body = read(&repo_root().join(rel));
        assert!(
            body.contains("retrieved: 2026-05-03"),
            "{rel} must carry M4 retrieval date"
        );
        for required in [
            "source_url:",
            "last_checked: 2026-05-03",
            "confidence:",
            "methodology_note:",
            "applicability_caveat:",
        ] {
            assert!(body.contains(required), "{rel} missing `{required}`");
        }
    }
}

#[test]
fn each_generator_skill_md_cites_baseline() {
    for (skill, baseline) in GENERATOR_CITATIONS {
        let body = read(&repo_root().join(skill));
        assert!(
            body.contains(baseline),
            "{skill} must cite baseline/reference file `{baseline}`"
        );
        assert!(
            body.contains("baseline_ref:"),
            "{skill} output frontmatter must document baseline_ref discipline"
        );
    }
}

#[test]
fn generator_skill_md_documents_stale_warning() {
    for (skill, _) in GENERATOR_CITATIONS {
        let body = read(&repo_root().join(skill));
        assert!(
            body.contains("stale warning") || body.contains("stale-warning"),
            "{skill} must document the >12 month stale warning"
        );
        assert!(
            body.contains("refuse at +24 months") || body.contains("refuses at +24 months"),
            "{skill} must document refusal at +24 months"
        );
    }
}

#[test]
fn authoritative_sources_only() {
    for rel in BASELINE_FILES {
        let body = read(&repo_root().join(rel));
        for forbidden in [
            "medium.com",
            "substack.com",
            "random commentary",
            "unsourced",
            "training memory",
        ] {
            assert!(
                !body.contains(forbidden),
                "{rel} must not use forbidden source `{forbidden}`"
            );
        }
    }
}

#[test]
fn launch_thresholds_reframe_unsourceable_numbers() {
    let body = read(&repo_root().join("references/biz/launch-success-thresholds.md"));
    assert!(
        body.contains("set your own threshold"),
        "launch baseline must reframe unsourceable launch numbers"
    );
    assert!(
        body.contains("threshold_owner: founder"),
        "launch baseline must assign threshold ownership to the founder"
    );
}
