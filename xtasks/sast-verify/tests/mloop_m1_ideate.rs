//! M1 structural-contract test (measurement-loop runbook).
//!
//! Asserts the `/slo-ideate` measurement-loop contract from M1:
//!
//! - Q3 is reframed from "smallest wedge" to the "smallest complete value
//!   slice" framing (philosophy shift; the phrase is asserted as an exact
//!   contiguous substring — mloop-m1 reuses the kani-m1 lesson that a
//!   `contains()` check is not satisfied by interleaved prose).
//! - The idea-doc output template gains a `## Success thesis` section with
//!   leading metric, lagging metric, guardrails, and review window.
//! - The success-thesis template names *behaviour*, not raw PII (C8 / data
//!   minimisation note routing real-user-quote risk to the /slo-verify scan).
//! - Existing idea-doc section names are preserved (no rename / removal).
//! - The forcing-question slot count is unchanged (no silent extra question):
//!   the diagnostic prompts the report asked for are folded INTO Q3, not added
//!   as new numbered questions.

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

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

fn ideate_md() -> String {
    read(&workspace_root().join("skills/slo-ideate/SKILL.md"))
}

/// Count forcing questions: lines whose trimmed start is `<digits>. **`
/// (the numbered-bold pattern the seven forcing questions use). Other numbered
/// prose in the file is not bold-led, so this isolates the question slots.
fn count_forcing_questions(content: &str) -> usize {
    content
        .lines()
        .filter(|line| {
            let t = line.trim_start();
            let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
            !digits.is_empty() && t[digits.len()..].starts_with(". **")
        })
        .count()
}

#[test]
fn slo_ideate_complete_value_slice_present() {
    let content = ideate_md();
    assert!(
        content.contains("smallest complete value slice"),
        "Q3 must be reframed to the `smallest complete value slice` framing (philosophy shift)"
    );
}

#[test]
fn slo_ideate_success_thesis_section_present() {
    let content = ideate_md();
    assert!(
        content.contains("## Success thesis"),
        "idea-doc output template must carry a `## Success thesis` section"
    );
    for sentinel in [
        "Leading metric",
        "Lagging metric",
        "Guardrails",
        "Review window",
    ] {
        assert!(
            content.contains(sentinel),
            "Success thesis section must include `{sentinel}`"
        );
    }
}

#[test]
fn slo_ideate_success_thesis_names_behaviour_not_pii() {
    let content = ideate_md().to_lowercase();
    assert!(
        content.contains("behaviour, not") && content.contains("pii"),
        "Success thesis must instruct naming behaviour, not PII (C8 / minimisation; routes real-user-quote risk to the /slo-verify scan)"
    );
}

#[test]
fn slo_ideate_existing_sections_preserved() {
    let content = ideate_md();
    for section in [
        "## The pain",
        "## Top risks",
        "## Recommendation",
        "## Open questions for /slo-research",
    ] {
        assert!(
            content.contains(section),
            "existing idea-doc section `{section}` must be preserved (no rename/removal)"
        );
    }
}

#[test]
fn slo_ideate_question_slot_count_unchanged() {
    // The seven forcing questions stay seven: the report's diagnostic prompts
    // are folded into Q3, not added as new numbered questions.
    let content = ideate_md();
    assert_eq!(
        count_forcing_questions(&content),
        7,
        "the forcing-question slot count must remain 7 (no silent extra question)"
    );
}
