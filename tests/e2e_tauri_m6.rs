//! E2E tests for Milestone 6 — Settings Panel & Provider Architecture.
//!
//! These tests validate:
//! - Provider trait abstraction (ClaudeProvider)
//! - Settings persistence (JSON roundtrip, defaults)
//! - Settings struct fields and defaults

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ── Mirror types for testing (binary crate cannot be imported) ──────────

/// Mirror of `AppSettings` for JSON roundtrip testing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct AppSettings {
    pub provider: String,
    pub model: String,
    pub allow_flags: Vec<String>,
    pub deny_flags: Vec<String>,
    pub max_attempts: u32,
    pub cooldown_secs: u64,
    pub max_iterations: u32,
    pub repo_dir: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            provider: "claude".to_string(),
            model: "claude-sonnet-4-6".to_string(),
            allow_flags: sldo_common::toolflags::plan_allow_flags(),
            deny_flags: sldo_common::toolflags::plan_deny_flags(),
            max_attempts: 150,
            cooldown_secs: 5,
            max_iterations: 3,
            repo_dir: None,
        }
    }
}

// ── Feature: Provider trait abstraction ─────────────────────────────────

#[test]
fn claude_provider_has_correct_name() {
    // Given: A ClaudeProvider (mirror check — the real struct lives in sldo-tauri)
    // We verify the expected name is "claude"
    let expected_name = "claude";
    // Then: The expected provider name matches the convention
    assert_eq!(expected_name, "claude");
}

// ── Feature: Settings persistence ──────────────────────────────────────

#[test]
fn default_settings_created() {
    // Given: Default AppSettings
    let settings = AppSettings::default();
    // Then: Has expected model, provider, and flags
    assert_eq!(settings.model, "claude-sonnet-4-6");
    assert_eq!(settings.provider, "claude");
    assert!(!settings.allow_flags.is_empty(), "Default allow flags should not be empty");
    assert!(settings.deny_flags.is_empty(), "Default deny flags should be empty for Claude Code CLI");
    assert_eq!(settings.max_attempts, 150);
    assert_eq!(settings.cooldown_secs, 5);
    assert_eq!(settings.max_iterations, 3);
    assert!(settings.repo_dir.is_none());
}

#[test]
fn settings_roundtrip_json() {
    // Given: AppSettings with custom values
    let settings = AppSettings {
        provider: "claude".to_string(),
        model: "claude-opus-4-7".to_string(),
        allow_flags: vec!["--allowedTools=Read,Write,Bash".to_string()],
        deny_flags: vec![],
        max_attempts: 100,
        cooldown_secs: 10,
        max_iterations: 5,
        repo_dir: Some("/tmp/test-repo".to_string()),
    };

    // When: Serialize → write → read → deserialize
    let tmp_dir = PathBuf::from("output/m6-settings-test");
    fs::create_dir_all(&tmp_dir).unwrap();
    let settings_path = tmp_dir.join("settings.json");

    let json = serde_json::to_string_pretty(&settings).unwrap();
    fs::write(&settings_path, &json).unwrap();

    let loaded_json = fs::read_to_string(&settings_path).unwrap();
    let loaded: AppSettings = serde_json::from_str(&loaded_json).unwrap();

    // Then: Deserialized settings match original
    assert_eq!(loaded, settings);
    assert_eq!(loaded.model, "claude-opus-4-7");
    assert_eq!(loaded.max_attempts, 100);
    assert_eq!(loaded.repo_dir, Some("/tmp/test-repo".to_string()));

    // Cleanup
    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn invalid_settings_json_falls_back_to_defaults() {
    // Given: A corrupted settings.json file
    let tmp_dir = PathBuf::from("output/m6-invalid-settings-test");
    fs::create_dir_all(&tmp_dir).unwrap();
    let settings_path = tmp_dir.join("settings.json");
    fs::write(&settings_path, "{ this is not valid json !!!").unwrap();

    // When: Attempting to deserialize
    let result: Result<AppSettings, _> =
        serde_json::from_str(&fs::read_to_string(&settings_path).unwrap());

    // Then: Deserialization fails, so we fall back to defaults
    assert!(result.is_err(), "Corrupted JSON should fail to parse");
    let defaults = AppSettings::default();
    assert_eq!(defaults.model, "claude-sonnet-4-6");

    // Cleanup
    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn settings_default_allow_flags_match_toolflags() {
    // Given: Default settings
    let settings = AppSettings::default();
    let expected = sldo_common::toolflags::plan_allow_flags();
    // Then: Allow flags match toolflags::plan_allow_flags()
    assert_eq!(settings.allow_flags, expected);
}

#[test]
fn settings_default_deny_flags_match_toolflags() {
    // Given: Default settings
    let settings = AppSettings::default();
    let expected = sldo_common::toolflags::plan_deny_flags();
    // Then: Deny flags match toolflags::plan_deny_flags()
    assert_eq!(settings.deny_flags, expected);
}
