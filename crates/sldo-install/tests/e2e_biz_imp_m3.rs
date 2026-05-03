//! M3 structural-contract tests for Business Skill Improvements.
//!
//! Verifies numeric verification discipline for SAFE worksheets, cap-table
//! snapshots, and pricing value-equation outputs.

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

const NUMERIC_SKILLS: &[&str] = &[
    "skills/slo-fundraise/SKILL.md",
    "skills/slo-equity/SKILL.md",
    "skills/slo-pricing/SKILL.md",
];

#[test]
fn safe_worksheet_has_verification() {
    let body = read(&repo_root().join("skills/slo-fundraise/SKILL.md"));
    for required in [
        "M3 numeric verification",
        "safe-worksheet",
        "runnable Python snippet",
        "SPDX-License-Identifier: MIT",
        "expected-results table",
        "two-pass verification",
        "refuse to write",
    ] {
        assert!(
            body.contains(required),
            "slo-fundraise/SKILL.md must document SAFE verification marker `{required}`"
        );
    }
    for tolerance in ["±£1", "±0.01%", "±1"] {
        assert!(
            body.contains(tolerance),
            "SAFE worksheet verification must document tolerance `{tolerance}`"
        );
    }
}

#[test]
fn cap_table_totals_re_derived() {
    let body = read(&repo_root().join("skills/slo-equity/SKILL.md"));
    for required in [
        "M3 numeric verification",
        "cap-table-snapshot",
        "re-derive every Total row",
        "sum-down",
        "weighted-product",
        "refuse to write",
    ] {
        assert!(
            body.contains(required),
            "slo-equity/SKILL.md must document cap-table verification marker `{required}`"
        );
    }
    for tolerance in ["±£1", "±0.01%", "±1"] {
        assert!(
            body.contains(tolerance),
            "cap-table verification must document tolerance `{tolerance}`"
        );
    }
}

#[test]
fn value_equation_emits_computation() {
    let body = read(&repo_root().join("skills/slo-pricing/SKILL.md"));
    for required in [
        "M3 numeric verification",
        "runnable Python snippet",
        "price = round(value × ratio, -2)",
        "reciprocal verification",
        "price=value×0.25",
        "value=price/0.25",
        "25-33%",
        "refuse to write",
    ] {
        assert!(
            body.contains(required),
            "slo-pricing/SKILL.md must document pricing verification marker `{required}`"
        );
    }
}

#[test]
fn python_snippets_stdlib_only() {
    for rel in NUMERIC_SKILLS {
        let body = read(&repo_root().join(rel));
        for forbidden in [
            "import requests",
            "import pandas",
            "import numpy",
            "import httpx",
            "from requests",
            "from pandas",
            "from numpy",
            "from httpx",
        ] {
            assert!(
                !body.contains(forbidden),
                "{rel} Python examples must stay stdlib-only; found `{forbidden}`"
            );
        }
    }
}

#[test]
fn refusal_on_mismatch_documented() {
    for rel in NUMERIC_SKILLS {
        let body = read(&repo_root().join(rel));
        assert!(
            body.contains("mismatch") && body.contains("refuse to write"),
            "{rel} must document mismatch handling and refusal to write"
        );
    }
}
