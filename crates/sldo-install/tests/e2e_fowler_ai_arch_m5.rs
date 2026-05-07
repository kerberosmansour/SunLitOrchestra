//! M5 structural-contract tests for the Fowler AI architecture SLO improvements.
//!
//! M5 keeps the ticket-sized flow at parity with the new sprint-flow contract
//! rows without turning tickets into full multi-milestone runbooks, and checks
//! the lightweight orientation docs.

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

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_ticket_template() -> String {
    read(repo_root().join("skills/slo-ticket-plan/references/ticket-contract-template_v_1.md"))
}

fn docs_ticket_template() -> String {
    read(repo_root().join("docs/slo/templates/ticket-contract-template_v_1.md"))
}

#[test]
fn ticket_templates_have_compact_parity_rows_in_both_mirrors() {
    for (label, body) in [
        ("skill-local", skill_ticket_template()),
        ("docs", docs_ticket_template()),
    ] {
        for row in [
            "Reversibility / rollback path",
            "Exemplar code to copy",
            "Anti-exemplar code not to copy",
            "Refactoring discipline",
            "AI tolerance contract",
        ] {
            assert!(
                body.contains(row),
                "{label} ticket template missing `{row}`"
            );
        }
        assert!(
            body.contains("N/A - no AI component"),
            "{label} ticket template missing non-AI N/A path"
        );
        for na_path in ["N/A - docs-only", "N/A - no refactoring performed"] {
            assert!(
                body.contains(na_path),
                "{label} ticket template missing compact N/A path `{na_path}`"
            );
        }
    }
}

#[test]
fn ticket_template_stays_compact_not_full_runbook_copy() {
    let body = docs_ticket_template();

    assert!(
        body.lines().count() <= 230,
        "ticket template should stay compact; current line count exceeds cap"
    );
    for forbidden in [
        "Milestone Tracker",
        "Global Entry Rules",
        "End-to-End Architecture Diagram",
    ] {
        assert!(
            !body.contains(forbidden),
            "ticket template copied full runbook-only section `{forbidden}`"
        );
    }
}

#[test]
fn ticket_plan_consumes_new_parity_rows() {
    let plan = read(repo_root().join("skills/slo-ticket-plan/SKILL.md"));
    let lower = plan.to_lowercase();

    for needle in [
        "exemplar",
        "anti-exemplar",
        "reversibility",
        "refactoring discipline",
        "ai tolerance",
        "n/a",
    ] {
        assert!(
            lower.contains(needle),
            "ticket-plan skill missing parity instruction `{needle}`"
        );
    }
}

#[test]
fn ticket_execute_and_verify_restate_new_constraints() {
    let execute = read(repo_root().join("skills/slo-ticket-execute/SKILL.md")).to_lowercase();
    let verify = read(repo_root().join("skills/slo-ticket-verify/SKILL.md")).to_lowercase();

    for (label, body) in [("execute", execute), ("verify", verify)] {
        for needle in ["exemplar", "reversibility", "refactoring", "ai tolerance"] {
            assert!(
                body.contains(needle),
                "ticket-{label} missing parity constraint `{needle}`"
            );
        }
    }
}

#[test]
fn catalog_and_architecture_have_concise_orientation() {
    let catalog = read(repo_root().join("docs/skill-pack-catalog.md")).to_lowercase();
    let architecture = read(repo_root().join("docs/ARCHITECTURE.md")).to_lowercase();
    let combined = format!("{catalog}\n{architecture}");

    for needle in ["reversibility", "exemplar", "ai tolerance"] {
        assert!(
            combined.contains(needle),
            "catalog/architecture docs missing concise orientation `{needle}`"
        );
    }
    assert!(
        !catalog.contains("accepted variance:")
            && !architecture.contains("accepted variance:")
            && !catalog.contains("| contract row |")
            && !architecture.contains("| contract row |")
            && !catalog.contains("reversibility / rollback path"),
        "catalog/architecture docs should orient without duplicating ticket or AI contracts"
    );
}
