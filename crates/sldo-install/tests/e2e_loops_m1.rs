//! M1 structural-contract tests for the loops-and-lessons-closure runbook.
//!
//! These tests assert that `docs/LOOPS-ENGINEERING.md` exists, has the four
//! documented loop sections (sprint, security-tuning, lessons,
//! library-feedback), opens with a "Start here" outcome-first orienter,
//! and is cross-linked from `docs/ARCHITECTURE.md` plus from each
//! engineering SKILL.md cited under a loop.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` Milestone 1.

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
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const LOOP_SECTIONS: &[&str] = &[
    "Sprint loop",
    "Security-tuning loop",
    "Lessons loop",
    "Library-feedback loop",
];

// Engineering skills cited under at least one loop in LOOPS-ENGINEERING.md.
// Every skill listed here must carry a back-link to LOOPS-ENGINEERING.md
// in its SKILL.md so the cross-reference invariant holds bidirectionally.
const CITED_ENGINEERING_SKILLS: &[&str] = &[
    "slo-ideate",
    "slo-research",
    "slo-architect",
    "slo-tla",
    "slo-plan",
    "slo-critique",
    "slo-execute",
    "slo-verify",
    "slo-retro",
    "slo-ship",
    "slo-sast",
    "slo-rulegen",
    "slo-ruleverify",
];

#[test]
fn loops_engineering_doc_exists_and_has_required_sections() {
    let doc = read(&repo_root().join("docs/LOOPS-ENGINEERING.md"));

    for section in LOOP_SECTIONS {
        assert!(
            doc.contains(section),
            "LOOPS-ENGINEERING.md missing required section: {section}"
        );
    }

    // Per-loop schema: name + trigger + steps + exit + skills + diagram.
    for marker in &[
        "Trigger",
        "Exit condition",
        "Skills involved",
        "User-visible outcome",
    ] {
        assert!(
            doc.contains(marker),
            "LOOPS-ENGINEERING.md missing per-loop schema marker: {marker}"
        );
    }
}

#[test]
fn loops_engineering_doc_has_start_here_orienter() {
    let doc = read(&repo_root().join("docs/LOOPS-ENGINEERING.md"));

    assert!(
        doc.contains("Start here"),
        "LOOPS-ENGINEERING.md missing 'Start here' outcome-first orienter"
    );

    // The orienter must map a question to a loop and a first skill so a
    // newcomer can reach an actionable suggestion in one screen.
    assert!(
        doc.contains("question") || doc.contains("Question"),
        "Start here section should map a question to a loop and skill"
    );
    assert!(
        doc.contains("Lessons loop"),
        "Start here orienter must reference the Lessons loop entrypoint"
    );
}

#[test]
fn architecture_md_cross_links_loops_engineering() {
    let arch = read(&repo_root().join("docs/ARCHITECTURE.md"));

    assert!(
        arch.contains("LOOPS-ENGINEERING.md"),
        "ARCHITECTURE.md must cross-link to LOOPS-ENGINEERING.md in a Feedback loops section"
    );
    assert!(
        arch.contains("Feedback loops") || arch.contains("Feedback Loops"),
        "ARCHITECTURE.md must declare a 'Feedback loops' section that hosts the cross-link"
    );
}

#[test]
fn every_cited_engineering_skill_has_cross_reference() {
    let root = repo_root();
    for skill_name in CITED_ENGINEERING_SKILLS {
        let skill_path = root.join(format!("skills/{skill_name}/SKILL.md"));
        let body = read(&skill_path);
        assert!(
            body.contains("LOOPS-ENGINEERING.md"),
            "skills/{skill_name}/SKILL.md missing cross-reference to LOOPS-ENGINEERING.md \
             (skill is cited in a loop section, so the back-link is required)"
        );
    }
}

#[test]
fn library_feedback_loop_has_unshipped_footnote() {
    // Per runbook M1 BDD: the Library-feedback loop section must explicitly
    // mark itself as "ships in Runbook 4" rather than be removed silently
    // while /slo-sec-libs is unshipped.
    let doc = read(&repo_root().join("docs/LOOPS-ENGINEERING.md"));
    let lower = doc.to_lowercase();
    assert!(
        lower.contains("runbook 4")
            || lower.contains("r4")
            || lower.contains("slo-sec-libs"),
        "Library-feedback loop section must footnote that /slo-sec-libs ships in Runbook 4"
    );
}
