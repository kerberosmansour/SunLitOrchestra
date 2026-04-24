//! Event payload types for Tauri frontend communication.

use serde::{Deserialize, Serialize};

/// Emitted for each line of Copilot output during planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanProgressEvent {
    /// The output line content.
    pub line: String,
    /// Which stream produced the line: `"stdout"` or `"stderr"`.
    pub stream: String,
    /// ISO 8601 timestamp of when the line was received.
    pub timestamp: String,
}

/// Emitted when planning completes successfully.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCompleteEvent {
    /// Path to the generated runbook file.
    pub runbook_path: String,
    /// Any validation issues found in the generated runbook.
    pub validation_issues: Vec<String>,
}

/// Emitted when planning fails.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanErrorEvent {
    /// Description of the error.
    pub error: String,
}

// ── Execution events (M5) ───────────────────────────────────────────────

/// Emitted when a milestone execution attempt begins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneStartedEvent {
    /// Which milestone number is starting.
    pub milestone_number: u32,
    /// Human-readable milestone title.
    pub title: String,
    /// Which attempt this is (1-based).
    pub attempt: u32,
}

/// Emitted for each line of agent output during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProgressEvent {
    /// The output line content.
    pub line: String,
    /// Which stream produced the line: `"stdout"` or `"stderr"`.
    pub stream: String,
    /// ISO 8601 timestamp of when the line was received.
    pub timestamp: String,
}

/// Emitted after a build or test verification command completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTestResultEvent {
    /// The command that was run.
    pub command: String,
    /// Whether the command succeeded.
    pub success: bool,
    /// Captured output (stdout + stderr).
    pub output: String,
}

/// Emitted when a single milestone attempt completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneCompletedEvent {
    /// Which milestone number completed.
    pub milestone_number: u32,
    /// Whether the milestone succeeded.
    pub success: bool,
}

/// Emitted when the entire execution run finishes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCompleteEvent {
    /// Whether all milestones are now done.
    pub all_done: bool,
    /// How many milestones were completed in this run.
    pub milestones_completed: u32,
    /// Total milestones in the runbook.
    pub total: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_progress_event_serializes() {
        // Given: A PlanProgressEvent
        let event = PlanProgressEvent {
            line: "Analyzing repository...".to_string(),
            stream: "stdout".to_string(),
            timestamp: "2026-03-17T12:00:00Z".to_string(),
        };
        // When: Serialized to JSON
        let json = serde_json::to_string(&event).unwrap();
        // Then: Contains expected fields
        assert!(json.contains("Analyzing repository"));
        assert!(json.contains("stdout"));
    }

    #[test]
    fn plan_complete_event_serializes() {
        // Given: A PlanCompleteEvent
        let event = PlanCompleteEvent {
            runbook_path: "/tmp/RUNBOOK.md".to_string(),
            validation_issues: vec!["Missing section: Smoke Tests".to_string()],
        };
        // When: Serialized to JSON
        let json = serde_json::to_string(&event).unwrap();
        // Then: Contains expected fields
        assert!(json.contains("RUNBOOK.md"));
        assert!(json.contains("Missing section"));
    }

    #[test]
    fn plan_error_event_serializes() {
        // Given: A PlanErrorEvent
        let event = PlanErrorEvent {
            error: "copilot not found".to_string(),
        };
        // When: Serialized to JSON
        let json = serde_json::to_string(&event).unwrap();
        // Then: Contains the error message
        assert!(json.contains("copilot not found"));
    }

    #[test]
    fn plan_progress_event_deserializes() {
        // Given: A JSON string for PlanProgressEvent
        let json = r#"{"line":"hello","stream":"stderr","timestamp":"2026-03-17T00:00:00Z"}"#;
        // When: Deserialized
        let event: PlanProgressEvent = serde_json::from_str(json).unwrap();
        // Then: Fields match
        assert_eq!(event.line, "hello");
        assert_eq!(event.stream, "stderr");
    }

    // ── Execution event BDD tests (M5) ──────────────────────────────────

    #[test]
    fn milestone_started_event_serializes() {
        // Given: A MilestoneStartedEvent
        let event = MilestoneStartedEvent {
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
        let event = ExecutionProgressEvent {
            line: "Building crate...".to_string(),
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
        let event = BuildTestResultEvent {
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
        let event = MilestoneCompletedEvent {
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
        let event = ExecutionCompleteEvent {
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

    #[test]
    fn execution_complete_event_deserializes() {
        // Given: A JSON string for ExecutionCompleteEvent
        let json = r#"{"all_done":false,"milestones_completed":2,"total":5}"#;
        // When: Deserialized
        let event: ExecutionCompleteEvent = serde_json::from_str(json).unwrap();
        // Then: Fields match
        assert!(!event.all_done);
        assert_eq!(event.milestones_completed, 2);
        assert_eq!(event.total, 5);
    }
}
