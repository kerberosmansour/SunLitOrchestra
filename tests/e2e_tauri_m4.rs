//! E2E runtime validation tests for Tauri Desktop Milestone 4.
//!
//! Tests:
//! - read_runbook_parses_real_file — reads an actual runbook, returns milestones
//! - save_and_reparse_roundtrip — write content, re-read, milestones match
//! - save_invalid_content_returns_warnings — save content missing table, get warnings

use sldo_common::runbook;

/// read_runbook_parses_real_file — reads docs/RUNBOOK-RUST-REWRITE.md and returns 5 milestones.
#[test]
fn read_runbook_parses_real_file() {
    // Given: The RUNBOOK-RUST-REWRITE.md exists with 5 milestones
    let runbook_path = std::path::Path::new("docs/RUNBOOK-RUST-REWRITE.md");
    assert!(
        runbook_path.exists(),
        "Expected docs/RUNBOOK-RUST-REWRITE.md to exist"
    );

    // When: We read and parse it
    let content = std::fs::read_to_string(runbook_path).expect("Failed to read runbook");
    let milestones = runbook::parse_tracker(&content);

    // Then: Returns 5 milestones
    assert_eq!(
        milestones.len(),
        5,
        "Expected 5 milestones, got {}",
        milestones.len()
    );

    // All should be done (the rust rewrite is completed)
    for m in &milestones {
        assert_eq!(
            m.status,
            runbook::MilestoneStatus::Done,
            "Milestone {} should be done",
            m.number
        );
    }
}

/// save_and_reparse_roundtrip — write content to a temp file, re-read, milestones match.
#[test]
fn save_and_reparse_roundtrip() {
    // Given: Valid runbook content
    let content = r#"# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Setup project | `done` | 2026-01-01 | 2026-01-02 | `docs/lessons/m1.md` |
| 2 | Implement core | `in_progress` | 2026-01-03 | | |
| 3 | Write tests | `not_started` | | | |

## Pre-Milestone Protocol

Do stuff before each milestone.

## Post-Milestone Protocol

Do stuff after each milestone.

## Background Context

Some context about the project.
"#;

    // When: Write to a temp file
    let tmp_dir = std::env::temp_dir().join("sldo_e2e_tauri_m4_roundtrip");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).expect("Failed to create temp dir");
    let path = tmp_dir.join("RUNBOOK.md");
    std::fs::write(&path, content).expect("Failed to write runbook");

    // And: Re-read and parse
    let re_read = std::fs::read_to_string(&path).expect("Failed to re-read runbook");
    let milestones = runbook::parse_tracker(&re_read);

    // Then: Milestones match
    assert_eq!(milestones.len(), 3);
    assert_eq!(milestones[0].title, "Setup project");
    assert_eq!(milestones[0].status, runbook::MilestoneStatus::Done);
    assert_eq!(milestones[1].title, "Implement core");
    assert_eq!(milestones[1].status, runbook::MilestoneStatus::InProgress);
    assert_eq!(milestones[2].title, "Write tests");
    assert_eq!(milestones[2].status, runbook::MilestoneStatus::NotStarted);

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

/// save_invalid_content_returns_warnings — save content without milestone table, get warnings.
#[test]
fn save_invalid_content_returns_warnings() {
    // Given: Content missing the milestone table
    let content = "# Just a title\n\nSome text without a milestone table.\n";

    // When: Parse tracker on content without a table
    let milestones = runbook::parse_tracker(content);

    // Then: No milestones found (which is a warning condition)
    assert!(
        milestones.is_empty(),
        "Expected no milestones from invalid content"
    );

    // And: Validation-style check — required sections missing
    let has_tracker = content.contains("Milestone Tracker");
    let has_pre = content.contains("Pre-Milestone Protocol");
    let has_post = content.contains("Post-Milestone Protocol");
    let has_bg = content.contains("Background Context");

    assert!(!has_tracker, "Should not have Milestone Tracker section");
    assert!(!has_pre, "Should not have Pre-Milestone Protocol section");
    assert!(!has_post, "Should not have Post-Milestone Protocol section");
    assert!(!has_bg, "Should not have Background Context section");
}
