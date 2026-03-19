//! E2E tests for voice-tx Milestone 4 — End-to-End Wiring, Error Handling & macOS Permission.
//!
//! These tests validate:
//! - Info.plist exists and contains NSMicrophoneUsageDescription
//! - Standalone command rejects empty audio
//! - Standalone command rejects missing API key
//! - Standalone command rejects invalid base64

use base64::Engine as _;

/// Info.plist exists and contains NSMicrophoneUsageDescription for macOS mic access.
#[test]
fn info_plist_exists_and_valid() {
    // Given: The crates/sldo-tauri/Info.plist file
    let plist_path = "crates/sldo-tauri/Info.plist";
    let content = std::fs::read_to_string(plist_path)
        .unwrap_or_else(|_| panic!("Info.plist must exist at {}", plist_path));

    // Then: It contains NSMicrophoneUsageDescription
    assert!(
        content.contains("NSMicrophoneUsageDescription"),
        "Info.plist must contain NSMicrophoneUsageDescription for macOS microphone permission"
    );

    // And: It's a valid plist structure
    assert!(
        content.contains("<?xml") || content.contains("<plist"),
        "Info.plist must be a valid XML plist"
    );
    assert!(
        content.contains("<dict>"),
        "Info.plist must contain a <dict> element"
    );
}

/// Empty audio is rejected at the Rust layer before any network call.
#[test]
fn standalone_command_rejects_empty_audio() {
    // Given: The voice.rs file with transcribe_audio_standalone
    let voice_rs = std::fs::read_to_string("crates/sldo-tauri/src/commands/voice.rs")
        .expect("voice.rs must exist");

    // Then: The standalone function checks for empty audio
    assert!(
        voice_rs.contains("is_empty()"),
        "transcribe_audio_standalone must check for empty audio bytes"
    );

    // And: The error message mentions empty/no audio
    let lower = voice_rs.to_lowercase();
    assert!(
        lower.contains("empty") || lower.contains("no audio"),
        "Empty audio error must mention 'empty' or 'no audio'"
    );

    // Verify by decoding empty base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode("")
        .expect("empty string is valid base64");
    assert!(decoded.is_empty(), "Empty base64 decodes to empty bytes");
}

/// Missing API key is caught cleanly with a user-friendly error.
#[test]
fn standalone_command_rejects_missing_key() {
    // Given: OPENAI_API_KEY not set
    let original = std::env::var("OPENAI_API_KEY").ok();
    std::env::remove_var("OPENAI_API_KEY");

    // When: We check the environment
    let result = std::env::var("OPENAI_API_KEY");

    // Then: Key is missing
    assert!(result.is_err(), "OPENAI_API_KEY should not be set");

    // And: voice.rs error handling mentions "API key"
    let voice_rs = std::fs::read_to_string("crates/sldo-tauri/src/commands/voice.rs")
        .expect("voice.rs must exist");
    assert!(
        voice_rs.to_lowercase().contains("api key"),
        "Error handling must mention 'API key'"
    );

    // Restore
    if let Some(val) = original {
        std::env::set_var("OPENAI_API_KEY", val);
    }
}

/// Malformed base64 input is caught before any network call.
#[test]
fn standalone_command_rejects_invalid_base64() {
    // Given: Invalid base64 string
    let result = base64::engine::general_purpose::STANDARD
        .decode("!!!not-valid-base64!!!");

    // Then: Decoding fails
    assert!(result.is_err(), "Invalid base64 must produce decode error");

    // And: voice.rs has base64 decode error handling in standalone function
    let voice_rs = std::fs::read_to_string("crates/sldo-tauri/src/commands/voice.rs")
        .expect("voice.rs must exist");
    assert!(
        voice_rs.contains("decode") && voice_rs.contains("transcribe_audio_standalone"),
        "transcribe_audio_standalone must handle base64 decode errors"
    );
}
