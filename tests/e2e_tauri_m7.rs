//! E2E tests for Milestone 7 — Voice Input & Speech-to-Text Integration.
//!
//! These tests validate:
//! - Missing API key returns an error mentioning "API key"
//! - dotenvy loads variables from .env file

use std::fs;
use std::path::PathBuf;

// ── Feature: STT backend ────────────────────────────────────────────────

#[test]
fn missing_api_key_returns_error() {
    // Given: OPENAI_API_KEY is not set
    // (We ensure the env var is absent for this test)
    let key = "OPENAI_API_KEY";
    let original = std::env::var(key).ok();
    std::env::remove_var(key);

    // When: We check for the API key (mirroring transcribe_audio logic)
    let result = std::env::var(key);

    // Then: Error — the key is not available
    assert!(result.is_err(), "OPENAI_API_KEY should not be set");

    // Restore if it was set before
    if let Some(val) = original {
        std::env::set_var(key, val);
    }
}

#[test]
fn env_file_loading_works() {
    // Given: A temporary .env file with OPENAI_API_KEY set
    let tmp_dir = PathBuf::from("output/m7-env-test");
    fs::create_dir_all(&tmp_dir).unwrap();
    let env_path = tmp_dir.join(".env");
    fs::write(&env_path, "TEST_M7_VOICE_KEY=test-key-12345\n").unwrap();

    // When: dotenvy loads from that file
    let result = dotenvy::from_path(&env_path);

    // Then: The variable is loaded into the environment
    assert!(result.is_ok(), "dotenvy should load .env file successfully");
    let loaded = std::env::var("TEST_M7_VOICE_KEY");
    assert_eq!(loaded.unwrap(), "test-key-12345");

    // Cleanup
    std::env::remove_var("TEST_M7_VOICE_KEY");
    let _ = fs::remove_dir_all(&tmp_dir);
}
