//! Managed application state for the Tauri backend.

use std::path::PathBuf;
use std::sync::Mutex;

/// Top-level application state, managed by Tauri.
pub struct AppState {
    /// Current active session (if any).
    pub current_session: Mutex<Option<PlanningSession>>,
    /// Application settings.
    pub settings: Mutex<AppSettings>,
}

/// A planning session tracking an active or completed planning run.
#[derive(Debug, Clone)]
pub struct PlanningSession {
    /// Unique session identifier.
    pub id: String,
    /// The user's original prompt.
    pub prompt: String,
    /// Path to the generated runbook (set on completion).
    pub runbook_path: Option<PathBuf>,
    /// Whether planning is currently in progress.
    pub in_progress: bool,
}

/// Persistent application settings.
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Copilot model to use for planning.
    pub model: String,
    /// Maximum planning iterations.
    pub max_iterations: u32,
    /// Repository directory (set by the user).
    pub repo_dir: Option<PathBuf>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            model: "claude-sonnet-4".to_string(),
            max_iterations: 3,
            repo_dir: None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_session: Mutex::new(None),
            settings: Mutex::new(AppSettings::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_default_has_no_session() {
        // Given: Default AppState
        let state = AppState::default();
        // When: current_session is checked
        let session = state.current_session.lock().unwrap();
        // Then: No session exists
        assert!(session.is_none());
    }

    #[test]
    fn app_settings_default_values() {
        // Given: Default AppSettings
        let settings = AppSettings::default();
        // Then: Has sensible defaults
        assert_eq!(settings.model, "claude-sonnet-4");
        assert_eq!(settings.max_iterations, 3);
        assert!(settings.repo_dir.is_none());
    }

    #[test]
    fn planning_session_tracks_state() {
        // Given: A PlanningSession
        let session = PlanningSession {
            id: "test-123".to_string(),
            prompt: "Build a REST API".to_string(),
            runbook_path: None,
            in_progress: true,
        };
        // Then: Fields are accessible
        assert_eq!(session.id, "test-123");
        assert!(session.in_progress);
        assert!(session.runbook_path.is_none());
    }
}
