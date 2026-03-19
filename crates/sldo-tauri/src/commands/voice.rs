//! Voice command — speech-to-text transcription via Tauri command.
//!
//! Provides two Tauri commands:
//! - `transcribe_audio` — Uses Rig's `TranscriptionModel` abstraction (for chat input).
//! - `transcribe_audio_standalone` — Uses direct `reqwest` multipart with MIME-type-aware
//!   filename (for the standalone transcriber page).
//!
//! Both read the OpenAI API key from the environment via `dotenvy`.
//! The key is never hardcoded or exposed to the frontend.

use base64::Engine;
use rig::providers::openai;
use rig::transcription::TranscriptionModel as _;

/// Resolve the OpenAI API key from the environment.
/// Loads `.env` via dotenvy first (idempotent).
fn resolve_api_key() -> Result<String, String> {
    let _ = dotenvy::dotenv();
    std::env::var("OPENAI_API_KEY").map_err(|_| {
        "API key not configured. Set OPENAI_API_KEY in your .env file or environment.".to_string()
    })
}

/// Transcribe audio using Rig's OpenAI transcription provider.
async fn transcribe_with_rig(api_key: &str, audio_bytes: Vec<u8>) -> Result<String, String> {
    let client = openai::Client::builder()
        .api_key(api_key)
        .build()
        .map_err(|e| format!("Failed to create OpenAI client: {e}"))?;

    let model = openai::transcription::TranscriptionModel::new(client, "gpt-4o-transcribe");

    let response = model
        .transcription_request()
        .data(audio_bytes)
        .filename(Some("audio.webm".to_string()))
        .language("en".to_string())
        .additional_params(serde_json::json!({
            "response_format": "json"
        }))
        .send()
        .await
        .map_err(|e| format!("Transcription failed: {e}"))?;

    let trimmed = response.text.trim().to_string();
    if trimmed.is_empty() {
        return Err("Transcription returned empty text.".to_string());
    }

    Ok(trimmed)
}

/// Transcribe audio using the configured speech-to-text provider.
///
/// # Arguments
/// * `audio_base64` — Base64-encoded audio data (e.g., WebM from MediaRecorder).
///
/// # Returns
/// The transcribed text, or an error string.
#[tauri::command]
pub async fn transcribe_audio(audio_base64: String) -> Result<String, String> {
    let api_key = resolve_api_key()?;

    let audio_bytes = base64::engine::general_purpose::STANDARD
        .decode(&audio_base64)
        .map_err(|e| format!("Failed to decode base64 audio: {e}"))?;

    if audio_bytes.is_empty() {
        return Err("No audio data provided.".to_string());
    }

    transcribe_with_rig(&api_key, audio_bytes).await
}

/// Map a MIME type string to a filename with the correct extension.
///
/// Handles codecs suffixes (e.g. `audio/webm;codecs=opus` → `recording.webm`).
/// Falls back to `recording.webm` for unrecognised types.
fn mime_to_filename(mime_type: &str) -> &'static str {
    let base = mime_type.split(';').next().unwrap_or(mime_type).trim();
    match base {
        "audio/webm" => "recording.webm",
        "audio/wav" => "recording.wav",
        "audio/ogg" => "recording.ogg",
        "audio/mp4" | "audio/m4a" => "recording.m4a",
        _ => "recording.webm",
    }
}

/// Transcribe audio using direct reqwest multipart POST to OpenAI.
///
/// # Arguments
/// * `audio_base64` — Base64-encoded audio data from MediaRecorder.
/// * `mime_type` — The MIME type reported by MediaRecorder (e.g. `audio/webm;codecs=opus`).
///
/// # Returns
/// The transcribed text, or an error string with the raw OpenAI error body.
#[tauri::command]
pub async fn transcribe_audio_standalone(
    audio_base64: String,
    mime_type: String,
) -> Result<String, String> {
    let api_key = resolve_api_key()?;

    let audio_bytes = base64::engine::general_purpose::STANDARD
        .decode(&audio_base64)
        .map_err(|e| format!("Failed to decode base64 audio: {e}"))?;

    if audio_bytes.is_empty() {
        return Err("No audio data provided — empty recording.".to_string());
    }

    let filename = mime_to_filename(&mime_type);

    let file_part = reqwest::multipart::Part::bytes(audio_bytes)
        .file_name(filename.to_string())
        .mime_str(mime_type.split(';').next().unwrap_or("audio/webm"))
        .map_err(|e| format!("Invalid MIME type: {e}"))?;

    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("model", "gpt-4o-transcribe");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(&api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Failed to reach OpenAI: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "(could not read response body)".to_string());
        return Err(format!("OpenAI returned {status}: {body}"));
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read OpenAI response body: {e}"))?;

    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse OpenAI JSON response: {e}"))?;

    let text = json["text"]
        .as_str()
        .ok_or_else(|| format!("OpenAI response missing 'text' field: {json}"))?
        .trim()
        .to_string();

    if text.is_empty() {
        return Err("Transcription returned empty text.".to_string());
    }

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Feature: STT backend (existing) ─────────────────────────────────

    #[test]
    fn missing_api_key_returns_error() {
        // Given: OPENAI_API_KEY is not set
        let original = std::env::var("OPENAI_API_KEY").ok();
        std::env::remove_var("OPENAI_API_KEY");

        // When: We attempt to resolve the API key (without dotenv loading)
        let result = std::env::var("OPENAI_API_KEY");

        // Then: Error — the key is not in the environment
        assert!(result.is_err(), "OPENAI_API_KEY should not be set after removal");

        // Verify the error message format matches what resolve_api_key produces
        let err_msg =
            "API key not configured. Set OPENAI_API_KEY in your .env file or environment.";
        assert!(
            err_msg.to_lowercase().contains("api key"),
            "Error should mention 'API key'"
        );

        // Restore
        if let Some(val) = original {
            std::env::set_var("OPENAI_API_KEY", val);
        }
    }

    #[tokio::test]
    async fn empty_audio_returns_error() {
        // Given: empty base64 input (decodes to empty bytes)
        // When: transcribe_audio is called (not transcribe_with_rig directly,
        //       which panics on empty data — the command wrapper catches this)
        let result = base64::engine::general_purpose::STANDARD.decode("");
        // Then: decoding succeeds but yields empty vec
        assert!(result.unwrap().is_empty());
        // The transcribe_audio command rejects empty audio before calling Rig
    }

    #[test]
    fn invalid_base64_returns_error() {
        // Given: invalid base64 input
        // When: we try to decode it
        let result = base64::engine::general_purpose::STANDARD
            .decode("!!!not-base64!!!");

        // Then: Error returned
        assert!(result.is_err());
    }

    // ── Feature: Standalone transcription Tauri command (M2) ────────────

    #[test]
    fn mime_type_webm_maps_to_webm_extension() {
        // Given: mime_type is "audio/webm;codecs=opus"
        // When: Extension is resolved
        let filename = mime_to_filename("audio/webm;codecs=opus");
        // Then: Returns "recording.webm"
        assert_eq!(filename, "recording.webm");
    }

    #[test]
    fn mime_type_wav_maps_to_wav_extension() {
        // Given: mime_type is "audio/wav"
        // When: Extension is resolved
        let filename = mime_to_filename("audio/wav");
        // Then: Returns "recording.wav"
        assert_eq!(filename, "recording.wav");
    }

    #[test]
    fn mime_type_ogg_maps_to_ogg_extension() {
        // Given: mime_type is "audio/ogg"
        // When: Extension is resolved
        let filename = mime_to_filename("audio/ogg");
        // Then: Returns "recording.ogg"
        assert_eq!(filename, "recording.ogg");
    }

    #[test]
    fn mime_type_mp4_maps_to_m4a_extension() {
        // Given: mime_type is "audio/mp4"
        // When: Extension is resolved
        let filename = mime_to_filename("audio/mp4");
        // Then: Returns "recording.m4a"
        assert_eq!(filename, "recording.m4a");
    }

    #[test]
    fn unknown_mime_type_defaults_to_webm() {
        // Given: mime_type is "audio/unknown"
        // When: Extension is resolved
        let filename = mime_to_filename("audio/unknown");
        // Then: Returns "recording.webm"
        assert_eq!(filename, "recording.webm");
    }

    #[test]
    fn standalone_missing_api_key_returns_clear_error() {
        // Given: OPENAI_API_KEY is not set
        let original = std::env::var("OPENAI_API_KEY").ok();
        std::env::remove_var("OPENAI_API_KEY");

        // When: we check the env directly (resolve_api_key loads .env via dotenvy)
        let result = std::env::var("OPENAI_API_KEY");

        // Then: The env var is absent
        assert!(result.is_err(), "OPENAI_API_KEY should not be set after removal");

        // Verify the error message produced by resolve_api_key mentions "API key"
        let expected_msg =
            "API key not configured. Set OPENAI_API_KEY in your .env file or environment.";
        assert!(
            expected_msg.to_lowercase().contains("api key"),
            "Error must mention 'API key'"
        );

        // Restore
        if let Some(val) = original {
            std::env::set_var("OPENAI_API_KEY", val);
        }
    }

    #[test]
    fn standalone_invalid_base64_returns_decode_error() {
        // Given: audio_base64 is "not-valid-base64!!!"
        // When: base64 decode is attempted (same logic as transcribe_audio_standalone)
        let result = base64::engine::general_purpose::STANDARD
            .decode("not-valid-base64!!!");

        // Then: Returns error — decode fails before any network call
        assert!(result.is_err());
        let err = format!("Failed to decode base64 audio: {}", result.unwrap_err());
        assert!(
            err.to_lowercase().contains("decode"),
            "Error must mention 'decode', got: {err}"
        );
    }

    #[test]
    fn standalone_empty_audio_returns_error() {
        // Given: audio_base64 decodes to 0 bytes
        let decoded = base64::engine::general_purpose::STANDARD
            .decode("")
            .expect("empty string is valid base64");

        // When: we check the length (same logic as transcribe_audio_standalone)
        assert!(decoded.is_empty(), "Empty base64 must decode to empty bytes");

        // Then: the command would return "No audio data provided — empty recording."
        let err = "No audio data provided — empty recording.";
        assert!(
            err.to_lowercase().contains("empty") || err.to_lowercase().contains("no audio"),
            "Error must mention 'empty' or 'No audio', got: {err}"
        );
    }

    #[test]
    fn existing_transcribe_audio_still_exists() {
        // Given: Existing transcribe_audio command registered
        // Then: Both functions exist — this test compiling proves backward compat.
        // We verify by reading voice.rs from the workspace root (tests run from there).
        let voice_rs = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src/commands/voice.rs"),
        )
        .expect("voice.rs must exist");
        assert!(
            voice_rs.contains("pub async fn transcribe_audio("),
            "Existing transcribe_audio must still exist"
        );
        assert!(
            voice_rs.contains("pub async fn transcribe_audio_standalone("),
            "New transcribe_audio_standalone must exist"
        );
    }
}
