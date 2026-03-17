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
}
