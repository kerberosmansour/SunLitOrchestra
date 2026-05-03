//! M1 structural-contract tests for Business Skill Improvements.
//!
//! Verifies the authority-file hardening layer:
//! - UK regulator enumeration rows have source-verification metadata.
//! - Three statute-anchor files exist and carry verbatim `quoted_text:` blocks.
//! - HMRC VCM and ICO DUAA references use quoted authority text.
//! - Statute/regulator sources stay on official authority domains.
//! - The four advisor hard-block predicate IDs remain immutable.

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

const REGULATOR_IDS: &[&str] = &[
    "hmrc",
    "companies-house",
    "pensions-regulator",
    "ico",
    "ofcom",
    "fca",
    "pra",
    "cma",
    "asa",
    "tsi",
    "mhra",
    "cqc",
    "gmc",
    "nmc",
    "hcpc",
    "hse",
    "ehrc",
    "ofgem",
    "ofwat",
    "ofsted",
    "caa",
    "environment-agency",
    "charity-commission",
    "oisc",
    "solicitors-regulation-authority",
];

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

#[test]
fn regulator_enum_source_verified() {
    let body = read(&repo_root().join("references/biz/uk-regulator-enumeration.md"));
    assert!(
        body.contains("## Source verification register"),
        "uk-regulator-enumeration.md must have a source verification register"
    );

    for id in REGULATOR_IDS {
        let row_prefix = format!("| `{id}` |");
        let row = body
            .lines()
            .find(|line| line.starts_with(&row_prefix))
            .unwrap_or_else(|| panic!("missing source-verification row for `{id}`"));
        assert!(
            row.contains("https://www.legislation.gov.uk/")
                || row.contains("https://www.gov.uk/")
                || row.contains("https://www.asa.org.uk/"),
            "source-verification row for `{id}` must cite an official authority URL: {row}"
        );
        assert!(
            row.contains("2026-05-03"),
            "source-verification row for `{id}` must carry the M1 last_checked date: {row}"
        );
        assert!(
            row.contains("2027-05-03"),
            "source-verification row for `{id}` must carry next_review_due: {row}"
        );
        assert!(
            row.contains("high") || row.contains("medium") || row.contains("low"),
            "source-verification row for `{id}` must carry a confidence value: {row}"
        );
        assert!(
            !row.contains("pending-verification"),
            "source-verification row for `{id}` must not ship pending-verification"
        );
    }
}

#[test]
fn statute_anchor_files_have_verbatim_quotes() {
    for rel in [
        "references/biz/uk-employment-statute-anchors.md",
        "references/biz/uk-consumer-statute-anchors.md",
        "references/biz/uk-marketing-statute-anchors.md",
    ] {
        let path = repo_root().join(rel);
        assert!(path.exists(), "{rel} missing");
        let body = read(&path);
        assert!(
            body.matches("quoted_text:").count() >= 3,
            "{rel} must contain at least three quoted_text blocks"
        );
        assert!(
            body.contains("source_url:"),
            "{rel} must cite source_url for each authority block"
        );
        assert!(
            !body.to_lowercase().contains("approximately says"),
            "{rel} must not use paraphrase framing"
        );
    }
}

#[test]
fn hmrc_vcm_index_verbatim() {
    let body = read(&repo_root().join("references/biz/hmrc-vcm-index.md"));
    for marker in ["VCM34080", "VCM3000", "VCM31000", "Abingdon Health"] {
        assert!(body.contains(marker), "hmrc-vcm-index.md missing {marker}");
    }
    assert!(
        body.matches("quoted_text:").count() >= 4,
        "hmrc-vcm-index.md must have quoted_text blocks for VCM34080, VCM3000, VCM31000, and Abingdon Health"
    );
    assert!(
        body.contains("https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm34080"),
        "hmrc-vcm-index.md must cite VCM34080 gov.uk URL"
    );
}

#[test]
fn ico_duaa_verbatim() {
    let body = read(&repo_root().join("references/biz/ico-duaa-index.md"));
    assert!(
        body.contains("https://www.legislation.gov.uk/ukpga/2025/18"),
        "ico-duaa-index.md must cite DUAA 2025 legislation.gov.uk URL"
    );
    assert!(
        body.contains("quoted_text:"),
        "ico-duaa-index.md must include verbatim DUAA quoted_text"
    );
    assert!(
        body.contains("commencement"),
        "ico-duaa-index.md must retain commencement coverage"
    );
}

#[test]
fn no_unauthoritative_sources_in_m1_authority_files() {
    for rel in [
        "references/biz/uk-regulator-enumeration.md",
        "references/biz/uk-employment-statute-anchors.md",
        "references/biz/uk-consumer-statute-anchors.md",
        "references/biz/uk-marketing-statute-anchors.md",
        "references/biz/hmrc-vcm-index.md",
        "references/biz/ico-duaa-index.md",
    ] {
        let body = read(&repo_root().join(rel));
        for forbidden in [
            "stackoverflow.com",
            "medium.com",
            "rossmartin.co.uk",
            "vendor blog",
            "random commentary",
        ] {
            assert!(
                !body.contains(forbidden),
                "{rel} must not cite unauthoritative source `{forbidden}`"
            );
        }
    }
}

#[test]
fn triage_gate_predicate_set_unchanged_from_m1() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(gate.contains(pid), "triage-gate.md missing `{pid}`");
    }
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(
            !gate.contains(&candidate),
            "triage-gate.md must not contain extra predicate pattern `{candidate}`"
        );
    }
}
