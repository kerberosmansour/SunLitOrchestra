//! Managed application state for the Tauri backend.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use sldo_common::toolflags;

/// Top-level application state, managed by Tauri.
pub struct AppState {
    /// Current active session (if any).
    pub current_session: Mutex<Option<PlanningSession>>,
    /// Application settings.
    pub settings: Mutex<AppSettings>,
    /// Cancellation flag for execution — set to `true` to stop the loop.
    pub cancel_execution: Arc<AtomicBool>,
    /// Whether an execution is currently running.
    pub execution_running: Arc<AtomicBool>,
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

/// Persistent application settings, saved as JSON in Tauri app data directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    /// Which agent provider to use (e.g., "claude").
    pub provider: String,
    /// Model to use for planning and execution.
    pub model: String,
    /// Tool permission allow flags.
    pub allow_flags: Vec<String>,
    /// Tool permission deny flags.
    pub deny_flags: Vec<String>,
    /// Maximum execution attempts before giving up.
    pub max_attempts: u32,
    /// Cooldown between execution attempts in seconds.
    pub cooldown_secs: u64,
    /// Maximum planning iterations.
    pub max_iterations: u32,
    /// Repository directory (set by the user).
    pub repo_dir: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            provider: "claude".to_string(),
            model: "claude-sonnet-4-6".to_string(),
            allow_flags: toolflags::plan_allow_flags(),
            deny_flags: toolflags::plan_deny_flags(),
            max_attempts: 150,
            cooldown_secs: 5,
            max_iterations: 3,
            repo_dir: None,
        }
    }
}

/// Settings file name within the Tauri app data directory.
const SETTINGS_FILENAME: &str = "settings.json";

/// Load settings from the given directory, falling back to defaults if
/// the file is missing or corrupted.
pub fn load_settings(app_data_dir: &std::path::Path) -> AppSettings {
    let path = app_data_dir.join(SETTINGS_FILENAME);
    if !path.exists() {
        return AppSettings::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: invalid settings.json, using defaults: {}", e);
            AppSettings::default()
        }),
        Err(e) => {
            eprintln!("Warning: could not read settings.json, using defaults: {}", e);
            AppSettings::default()
        }
    }
}

/// Save settings to the given directory.
pub fn save_settings(app_data_dir: &std::path::Path, settings: &AppSettings) -> Result<(), String> {
    std::fs::create_dir_all(app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    let path = app_data_dir.join(SETTINGS_FILENAME);
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write settings: {}", e))
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_session: Mutex::new(None),
            settings: Mutex::new(AppSettings::default()),
            cancel_execution: Arc::new(AtomicBool::new(false)),
            execution_running: Arc::new(AtomicBool::new(false)),
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
        assert_eq!(settings.provider, "claude");
        assert_eq!(settings.model, "claude-sonnet-4-6");
        assert_eq!(settings.max_iterations, 3);
        assert_eq!(settings.max_attempts, 150);
        assert_eq!(settings.cooldown_secs, 5);
        assert!(settings.repo_dir.is_none());
        assert!(!settings.allow_flags.is_empty());
        assert!(settings.deny_flags.is_empty());
    }

    #[test]
    fn app_settings_default_flags_match_toolflags() {
        // Given: Default AppSettings
        let settings = AppSettings::default();
        // Then: Flags match toolflags module
        assert_eq!(settings.allow_flags, toolflags::plan_allow_flags());
        assert_eq!(settings.deny_flags, toolflags::plan_deny_flags());
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

    // ── Execution state BDD tests (M5) ──────────────────────────────────

    #[test]
    fn app_state_has_cancel_flag() {
        // Given: Default AppState
        let state = AppState::default();
        // Then: Cancel flag is initially false
        assert!(
            !state.cancel_execution.load(std::sync::atomic::Ordering::Relaxed),
            "Cancel flag should start as false"
        );
    }

    #[test]
    fn app_state_has_execution_running_flag() {
        // Given: Default AppState
        let state = AppState::default();
        // Then: Execution running flag is initially false
        assert!(
            !state.execution_running.load(std::sync::atomic::Ordering::Relaxed),
            "Execution running should start as false"
        );
    }

    #[test]
    fn cancel_flag_can_be_set() {
        // Given: Default AppState
        let state = AppState::default();
        // When: Cancel flag is set to true
        state.cancel_execution.store(true, std::sync::atomic::Ordering::Relaxed);
        // Then: It reads as true
        assert!(state.cancel_execution.load(std::sync::atomic::Ordering::Relaxed));
    }

    // ── Feature: Settings persistence (M6) ──────────────────────────────

    #[test]
    fn settings_serializes_to_json() {
        // Given: Default AppSettings
        let settings = AppSettings::default();
        // When: Serialized to JSON
        let json = serde_json::to_string(&settings).unwrap();
        // Then: Contains expected fields
        assert!(json.contains("\"provider\":\"claude\""));
        assert!(json.contains("\"model\":\"claude-sonnet-4-6\""));
        assert!(json.contains("\"max_attempts\":150"));
    }

    #[test]
    fn settings_deserializes_from_json() {
        // Given: A JSON string
        let json = r#"{
            "provider": "claude",
            "model": "claude-opus-4-7",
            "allow_flags": ["--allowedTools=Read,Write,Bash"],
            "deny_flags": [],
            "max_attempts": 50,
            "cooldown_secs": 10,
            "max_iterations": 2,
            "repo_dir": "/tmp/repo"
        }"#;
        // When: Deserialized
        let settings: AppSettings = serde_json::from_str(json).unwrap();
        // Then: Fields match
        assert_eq!(settings.model, "claude-opus-4-7");
        assert_eq!(settings.max_attempts, 50);
        assert_eq!(settings.repo_dir, Some("/tmp/repo".to_string()));
    }

    #[test]
    fn load_settings_returns_defaults_when_no_file() {
        // Given: A non-existent directory
        let dir = std::path::Path::new("/tmp/sldo-test-nonexistent-dir-m6");
        // When: load_settings is called
        let settings = load_settings(dir);
        // Then: Returns defaults
        assert_eq!(settings.model, "claude-sonnet-4-6");
        assert_eq!(settings.provider, "claude");
    }

    #[test]
    fn save_and_load_settings_roundtrip() {
        // Given: Custom settings
        let dir = std::path::PathBuf::from("output/m6-state-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let settings = AppSettings {
            provider: "claude".to_string(),
            model: "claude-opus-4-7".to_string(),
            allow_flags: vec!["--allowedTools=Read,Write,Bash".to_string()],
            deny_flags: vec![],
            max_attempts: 42,
            cooldown_secs: 7,
            max_iterations: 1,
            repo_dir: Some("/tmp/repo".to_string()),
        };

        // When: Save then load
        save_settings(&dir, &settings).unwrap();
        let loaded = load_settings(&dir);

        // Then: Loaded matches saved
        assert_eq!(loaded, settings);

        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_settings_handles_corrupted_file() {
        // Given: A corrupted settings.json
        let dir = std::path::PathBuf::from("output/m6-corrupt-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("settings.json"), "not json!").unwrap();

        // When: load_settings is called
        let settings = load_settings(&dir);

        // Then: Returns defaults
        assert_eq!(settings.model, "claude-sonnet-4-6");
        assert_eq!(settings.provider, "claude");

        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }
}
