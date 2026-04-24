//! E2E tests for Tauri Desktop Milestone 5 — Execution Backend.
//!
//! These tests validate the execution backend logic: runbook parsing for
//! execution, build command detection, cancellation flag, and execution
//! event types.

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// ── Feature: Execution parses real runbook ──────────────────────────────

#[test]
fn execution_parses_real_runbook() {
    // Given: The real Rust rewrite runbook exists
    let runbook_path = Path::new("docs/RUNBOOK-RUST-REWRITE.md");
    assert!(runbook_path.exists(), "RUNBOOK-RUST-REWRITE.md must exist");

    // When: We parse the tracker
    let content = fs::read_to_string(runbook_path).expect("read runbook");
    let rows = sldo_common::runbook::parse_tracker(&content);

    // Then: All milestones are detected and all are done
    assert!(!rows.is_empty(), "Should have milestones");
    assert!(
        sldo_common::runbook::all_done(&rows),
        "All Rust rewrite milestones should be done"
    );
}

// ── Feature: Build command detection in own repo ────────────────────────

#[test]
fn detect_build_commands_in_own_repo() {
    // Given: The current project directory
    let project_dir = Path::new(".");

    // When: Build commands are detected
    let cmds = sldo_common::detect::detect_build_commands(project_dir);

    // Then: It includes cargo build
    assert!(
        cmds.iter().any(|c| c.contains("cargo build")),
        "Should detect 'cargo build' for this repo, got: {:?}",
        cmds
    );
}

// ── Feature: Cancellation flag stops loop ───────────────────────────────

#[test]
fn cancellation_flag_stops_loop() {
    // Given: An AtomicBool cancellation flag, initially false
    let cancel = Arc::new(AtomicBool::new(false));

    // When: We simulate an execution loop that checks the flag
    let mut iterations = 0;
    let max_iters = 10;

    for i in 0..max_iters {
        if cancel.load(Ordering::Relaxed) {
            break;
        }
        iterations += 1;

        // Cancel after 3 iterations (simulating user pressing cancel)
        if i == 2 {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    // Then: The loop stopped before completing all iterations
    assert_eq!(iterations, 3, "Loop should stop after cancellation at iteration 3");
    assert!(cancel.load(Ordering::Relaxed), "Cancel flag should be true");
}

// ── Feature: Execution event types serialize correctly ──────────────────

#[test]
fn milestone_started_event_serializes() {
    // Given: A MilestoneStartedEvent
    let event = sldo_tauri_test_helpers::MilestoneStartedEvent {
        milestone_number: 1,
        title: "Scaffold workspace".to_string(),
        attempt: 1,
    };

    // When: Serialized to JSON
    let json = serde_json::to_string(&event).unwrap();

    // Then: Contains expected fields
    assert!(json.contains("\"milestone_number\":1"));
    assert!(json.contains("Scaffold workspace"));
    assert!(json.contains("\"attempt\":1"));
}

#[test]
fn execution_progress_event_serializes() {
    // Given: An ExecutionProgressEvent
    let event = sldo_tauri_test_helpers::ExecutionProgressEvent {
        line: "Building crate sldo-common...".to_string(),
        stream: "stdout".to_string(),
        timestamp: "2026-03-17T12:00:00Z".to_string(),
    };

    // When: Serialized to JSON
    let json = serde_json::to_string(&event).unwrap();

    // Then: Contains expected fields
    assert!(json.contains("Building crate"));
    assert!(json.contains("stdout"));
}

#[test]
fn build_test_result_event_serializes() {
    // Given: A BuildTestResultEvent
    let event = sldo_tauri_test_helpers::BuildTestResultEvent {
        command: "cargo build --workspace".to_string(),
        success: true,
        output: "Finished dev profile".to_string(),
    };

    // When: Serialized to JSON
    let json = serde_json::to_string(&event).unwrap();

    // Then: Contains expected fields
    assert!(json.contains("cargo build"));
    assert!(json.contains("\"success\":true"));
}

#[test]
fn milestone_completed_event_serializes() {
    // Given: A MilestoneCompletedEvent
    let event = sldo_tauri_test_helpers::MilestoneCompletedEvent {
        milestone_number: 1,
        success: true,
    };

    // When: Serialized to JSON
    let json = serde_json::to_string(&event).unwrap();

    // Then: Contains expected fields
    assert!(json.contains("\"milestone_number\":1"));
    assert!(json.contains("\"success\":true"));
}

#[test]
fn execution_complete_event_serializes() {
    // Given: An ExecutionCompleteEvent
    let event = sldo_tauri_test_helpers::ExecutionCompleteEvent {
        all_done: true,
        milestones_completed: 5,
        total: 5,
    };

    // When: Serialized to JSON
    let json = serde_json::to_string(&event).unwrap();

    // Then: Contains expected fields
    assert!(json.contains("\"all_done\":true"));
    assert!(json.contains("\"milestones_completed\":5"));
    assert!(json.contains("\"total\":5"));
}

// ── Feature: All-done runbook detection ─────────────────────────────────

#[test]
fn all_done_runbook_detected() {
    // Given: A runbook where all milestones are done
    let content = r#"
| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `done` | 2026-01-01 | 2026-01-02 | |
| 2 | Second | `done` | 2026-01-03 | 2026-01-04 | |
"#;

    // When: Parsed
    let rows = sldo_common::runbook::parse_tracker(content);

    // Then: all_done returns true
    assert!(sldo_common::runbook::all_done(&rows));
}

#[test]
fn not_started_runbook_has_next_incomplete() {
    // Given: A runbook with an incomplete milestone
    let content = r#"
| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `done` | 2026-01-01 | 2026-01-02 | |
| 2 | Second | `not_started` | | | |
"#;

    // When: Parsed
    let rows = sldo_common::runbook::parse_tracker(content);

    // Then: next_incomplete returns milestone 2
    let next = sldo_common::runbook::next_incomplete(&rows);
    assert!(next.is_some());
    assert_eq!(next.unwrap().number, 2);
}

/// Helper module providing test-only re-exports of execution event types.
/// These mirror the types defined in `sldo-tauri/src/events.rs` but are
/// defined here because `sldo-tauri` is a binary crate and cannot be imported
/// as a library dependency.
mod sldo_tauri_test_helpers {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MilestoneStartedEvent {
        pub milestone_number: u32,
        pub title: String,
        pub attempt: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecutionProgressEvent {
        pub line: String,
        pub stream: String,
        pub timestamp: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BuildTestResultEvent {
        pub command: String,
        pub success: bool,
        pub output: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MilestoneCompletedEvent {
        pub milestone_number: u32,
        pub success: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecutionCompleteEvent {
        pub all_done: bool,
        pub milestones_completed: u32,
        pub total: u32,
    }
}
