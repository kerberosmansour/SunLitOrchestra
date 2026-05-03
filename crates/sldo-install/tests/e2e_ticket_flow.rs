//! Structural-contract tests for the ticket-sized SLO workflow.
//!
//! These tests pin the new GitHub Issues-first, v4-derived ticket lane:
//! one issue becomes one compact SLO ticket contract, executed with the
//! same allow-list, BDD, evidence, static-analysis, assertion, and
//! resource-bound discipline as the full v4 runbook flow.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const TICKET_SKILLS: &[&str] = &[
    "slo-ticket-pick",
    "slo-ticket-plan",
    "slo-ticket-execute",
    "slo-ticket-verify",
    "slo-ticket-close",
];

#[test]
fn ticket_template_exists_and_preserves_v4_rigor_markers() {
    let root = repo_root();
    let template = read(root.join("docs/slo/templates/ticket-contract-template_v_1.md"));

    for marker in &[
        "## 2. Sizing Gate",
        "## 5. Contract Block",
        "Data classification",
        "Proactive controls in play",
        "Abuse acceptance scenarios",
        "Resource bounds introduced/changed",
        "Invariants/assertions required",
        "Debugger / inspection expectation",
        "Static analysis gates",
        "## 7. BDD Acceptance Scenarios",
        "## 8. Validation Plan",
        "## 10. Self-Review Gate",
    ] {
        assert!(
            template.contains(marker),
            "ticket contract template missing v4-derived marker: {marker}"
        );
    }
}

#[test]
fn skill_local_template_mirror_matches_docs_template() {
    let root = repo_root();
    let docs_template = read(root.join("docs/slo/templates/ticket-contract-template_v_1.md"));
    let skill_template =
        read(root.join("skills/slo-ticket-plan/references/ticket-contract-template_v_1.md"));
    assert_eq!(
        docs_template, skill_template,
        "skill-local ticket template must stay byte-identical to docs mirror"
    );
}

#[test]
fn ticket_skills_exist_with_frontmatter_and_loop_backlink() {
    let root = repo_root();
    for skill in TICKET_SKILLS {
        let body = read(root.join("skills").join(skill).join("SKILL.md"));
        assert!(
            body.starts_with("---\n"),
            "{skill} SKILL.md must start with YAML frontmatter"
        );
        assert!(
            body.contains(&format!("name: {skill}")),
            "{skill} SKILL.md frontmatter must declare its skill name"
        );
        assert!(
            body.contains("LOOPS-ENGINEERING.md"),
            "{skill} SKILL.md must cross-link the Ticket loop"
        );
    }
}

#[test]
fn ticket_workflow_doc_documents_github_issue_adapter_and_escalation() {
    let root = repo_root();
    let doc = read(root.join("docs/slo/design/ticket-sized-slo-workflow.md"));

    for marker in &[
        "GitHub Issues",
        "slo-ticket-workpad:v1",
        "slo:ready",
        "slo:in-progress",
        "slo:review",
        "per-issue",
        "bounded concurrency",
        "escalates to the normal `/slo-plan` v4 runbook path",
    ] {
        assert!(
            doc.contains(marker),
            "ticket workflow doc missing marker: {marker}"
        );
    }
}

#[test]
fn catalog_and_loops_list_ticket_flow() {
    let root = repo_root();
    let catalog = read(root.join("docs/skill-pack-catalog.md"));
    let loops = read(root.join("docs/LOOPS-ENGINEERING.md"));

    assert!(catalog.contains("Ticket-sized SLO flow"));
    assert!(loops.contains("## Ticket loop"));
    for skill in TICKET_SKILLS {
        let public_name = format!("/{skill}");
        assert!(
            catalog.contains(&public_name),
            "catalog missing {public_name}"
        );
        assert!(
            loops.contains(&public_name),
            "loops doc missing {public_name}"
        );
    }
}
