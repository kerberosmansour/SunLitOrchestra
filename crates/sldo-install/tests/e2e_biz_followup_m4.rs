//! Follow-up M4 — biz-pack-judgment-tests (DESIGN + STUB).
//!
//! Combined critique flagged the load-bearing residual: structural tests
//! assert policy is documented; they don't exercise LLM-judgment classification
//! of marginal cases. This follow-up ships:
//!   1. v1 fixture set under references/biz/judgment-fixtures/<skill>/*.md
//!   2. README.md documenting the fixture format + harness expansion plan
//!   3. Structural-contract tests (non-ignored) asserting fixtures parse + are well-formed
//!   4. Runtime harness stub (#[ignore], not run by default) that future
//!      runbooks expand into real `claude`-CLI invocations
//!
//! The structural tests run as part of `cargo test -p sldo-install`. The
//! #[ignore] runtime tests run via `cargo test -- --ignored` once the harness
//! is fleshed out; today they panic with a "not yet implemented" message.

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

const ADVISOR_SKILLS: &[&str] = &["slo-legal", "slo-accounting", "slo-equity", "slo-fundraise"];

const REQUIRED_FIXTURE_FRONTMATTER_KEYS: &[&str] = &[
    "name:",
    "target_skill:",
    "target_mode:",
    "expected_gates_fired:",
    "expected_routing:",
    "must_refuse:",
    "must_route_to:",
    "fixture_class:",
    "adversarial:",
    "critique_provenance:",
];

const ALLOWED_FIXTURE_CLASSES: &[&str] = &[
    "gdpr",
    "ir35",
    "tax-efficiency-pushback",
    "deal-value",
    "counterparty-paper",
    "seis-eis",
    "preferential-rights",
    "control-fixture",
    "other",
];

const ALLOWED_ROUTING: &[&str] = &[
    "lawyer",
    "accountant",
    "dpo",
    "lawyer_and_accountant",
    "accountant_and_dpo",
    "lawyer_and_dpo",
    "none",
    "none-permit",
];

// ---------------------------------------------------------------------------
// Structural test 1 — fixture directory exists with subdirs per advisor skill.
// ---------------------------------------------------------------------------

#[test]
fn judgment_fixtures_directory_layout_correct() {
    let fixtures = repo_root().join("references/biz/judgment-fixtures");
    assert!(
        fixtures.is_dir(),
        "references/biz/judgment-fixtures/ must exist"
    );
    assert!(
        fixtures.join("README.md").exists(),
        "judgment-fixtures/README.md must exist"
    );
    for skill in ADVISOR_SKILLS {
        let subdir = fixtures.join(skill);
        assert!(subdir.is_dir(), "judgment-fixtures/{skill}/ must exist");
    }
}

// ---------------------------------------------------------------------------
// Structural test 2 — every fixture has the required frontmatter keys.
// ---------------------------------------------------------------------------

#[test]
fn all_fixtures_have_required_frontmatter() {
    let fixtures = repo_root().join("references/biz/judgment-fixtures");
    for skill in ADVISOR_SKILLS {
        let dir = fixtures.join(skill);
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let body = read(&path);
                for key in REQUIRED_FIXTURE_FRONTMATTER_KEYS {
                    assert!(
                        body.contains(key),
                        "fixture `{}` missing required frontmatter key `{key}`",
                        path.display()
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Structural test 3 — fixture frontmatter values are within allowed enums.
// ---------------------------------------------------------------------------

#[test]
fn all_fixtures_have_valid_enum_values() {
    let fixtures = repo_root().join("references/biz/judgment-fixtures");
    for skill in ADVISOR_SKILLS {
        let dir = fixtures.join(skill);
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.extension().map(|e| e == "md").unwrap_or(false) {
                continue;
            }
            let body = read(&path);

            // target_skill must equal the directory name.
            let target_line = body
                .lines()
                .find(|l| l.trim_start().starts_with("target_skill:"))
                .unwrap_or("");
            let target_skill = target_line
                .split("target_skill:")
                .nth(1)
                .unwrap_or("")
                .trim();
            assert_eq!(
                target_skill,
                *skill,
                "fixture `{}` declares target_skill `{target_skill}` but lives in `{skill}/`",
                path.display()
            );

            // fixture_class must be in allowed set.
            let class_line = body
                .lines()
                .find(|l| l.trim_start().starts_with("fixture_class:"))
                .unwrap_or("");
            let class_value = class_line
                .split("fixture_class:")
                .nth(1)
                .unwrap_or("")
                .trim();
            assert!(
                ALLOWED_FIXTURE_CLASSES.contains(&class_value),
                "fixture `{}` has fixture_class `{class_value}` not in {ALLOWED_FIXTURE_CLASSES:?}",
                path.display()
            );

            // must_route_to must be in allowed routing set.
            let route_line = body
                .lines()
                .find(|l| l.trim_start().starts_with("must_route_to:"))
                .unwrap_or("");
            let route_value = route_line
                .split("must_route_to:")
                .nth(1)
                .unwrap_or("")
                .trim();
            assert!(
                ALLOWED_ROUTING.contains(&route_value),
                "fixture `{}` has must_route_to `{route_value}` not in {ALLOWED_ROUTING:?}",
                path.display()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Structural test 4 — at least one fixture per critical class is present.
// ---------------------------------------------------------------------------

#[test]
fn critical_fixture_classes_seeded_in_v1_set() {
    let fixtures = repo_root().join("references/biz/judgment-fixtures");
    let mut classes_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for skill in ADVISOR_SKILLS {
        let dir = fixtures.join(skill);
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.extension().map(|e| e == "md").unwrap_or(false) {
                continue;
            }
            let body = read(&path);
            let class_line = body
                .lines()
                .find(|l| l.trim_start().starts_with("fixture_class:"))
                .unwrap_or("");
            let class_value = class_line
                .split("fixture_class:")
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            classes_seen.insert(class_value);
        }
    }

    // The v1 fixture set must cover at minimum these critical classes.
    let critical = [
        "gdpr",
        "ir35",
        "tax-efficiency-pushback",
        "seis-eis",
        "preferential-rights",
    ];
    for c in &critical {
        assert!(
            classes_seen.contains(*c),
            "v1 fixture set missing critical class `{c}` — combined critique flagged this surface as load-bearing residual"
        );
    }
}

// ---------------------------------------------------------------------------
// Structural test 5 — adversarial pushback fixture (tax-efficiency) exists.
// ---------------------------------------------------------------------------

#[test]
fn tax_efficiency_pushback_fixture_present() {
    let path =
        repo_root().join("references/biz/judgment-fixtures/slo-legal/tax-efficiency-pushback.md");
    assert!(path.exists(), "tax-efficiency-pushback.md fixture must exist (combined critique B1+B2+C f5 explicitly cited this scenario)");
    let body = read(&path);
    assert!(
        body.contains("adversarial: true"),
        "tax-efficiency-pushback.md must be marked adversarial: true"
    );
    assert!(
        body.contains("must_refuse: true"),
        "tax-efficiency-pushback.md expected behavior is must_refuse: true"
    );
}

// ---------------------------------------------------------------------------
// Runtime harness — implemented in `e2e_biz_judgment_runtime_m{1,2}.rs`
// (see `docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md`). The function below
// remains as a forwarder so external tooling that targets this name by
// convention still has a callable entry — but it does no work; pointers
// only.
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn runtime_harness_invokes_claude_cli_per_fixture() {
    eprintln!("Runtime harness moved. Run instead:");
    eprintln!("  BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install \\");
    eprintln!("      --test e2e_biz_judgment_runtime_m2 -- --ignored");
    eprintln!("(or _m1 for the single-fixture proof). See docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md.");
}
