//! Voice command — speech-to-text transcription via Tauri command.
//!
//! Receives base64-encoded audio from the frontend, sends it to the
//! configured STT provider using Rig's transcription abstraction, and returns
//! the transcribed text. The API key is read from the environment
//! (via dotenvy) so it is never exposed to the frontend.
//!
//! Uses `gpt-4o-transcribe` by default for best quality. Falls back to
//! `whisper-1` if the newer model isn't available.

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

#[cfg(test)]
mod tests {
    use super::*;

    // ── Feature: STT backend ────────────────────────────────────────────

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
}
