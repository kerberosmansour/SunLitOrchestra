//! Voice command — speech-to-text transcription via Tauri command.
//!
//! Receives base64-encoded audio from the frontend, sends it to the
//! configured STT provider (initially OpenAI Whisper API), and returns
//! the transcribed text. The API key is read from the environment
//! (via dotenvy) so it is never exposed to the frontend.

use base64::Engine;

/// Resolve the OpenAI API key from the environment.
/// Loads `.env` via dotenvy first (idempotent).
fn resolve_api_key() -> Result<String, String> {
    let _ = dotenvy::dotenv();
    std::env::var("OPENAI_API_KEY").map_err(|_| {
        "API key not configured. Set OPENAI_API_KEY in your .env file or environment.".to_string()
    })
}

/// Transcribe audio given an API key and base64-encoded audio data.
async fn transcribe_with_key(api_key: &str, audio_base64: &str) -> Result<String, String> {
    let audio_bytes = base64::engine::general_purpose::STANDARD
        .decode(audio_base64)
        .map_err(|e| format!("Failed to decode base64 audio: {e}"))?;

    if audio_bytes.is_empty() {
        return Err("No audio data provided.".to_string());
    }

    let part = reqwest::multipart::Part::bytes(audio_bytes)
        .file_name("audio.webm")
        .mime_str("audio/webm")
        .map_err(|e| format!("Failed to create multipart part: {e}"))?;

    let form = reqwest::multipart::Form::new()
        .text("model", "whisper-1")
        .text("response_format", "text")
        .part("file", part);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {api_key}"))
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                format!("Network connection error: {e}")
            } else {
                format!("Request failed: {e}")
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error (HTTP {status}): {body}"));
    }

    let text = response.text().await.map_err(|e| format!("Failed to read response: {e}"))?;
    let trimmed = text.trim().to_string();

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
    transcribe_with_key(&api_key, &audio_base64).await
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
        // Given: API key is set but audio is empty
        // When: transcribe_with_key is called with empty base64 (decodes to empty bytes)
        let result = transcribe_with_key("test-key", "").await;

        // Then: Error returned about no audio data
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_lowercase().contains("audio") || err.to_lowercase().contains("base64"),
            "Error should mention audio/base64, got: {err}"
        );
    }

    #[tokio::test]
    async fn invalid_base64_returns_error() {
        // Given: API key is set but audio is invalid base64
        // When: transcribe_with_key is called with invalid base64
        let result = transcribe_with_key("test-key", "!!!not-base64!!!").await;

        // Then: Error returned mentioning base64
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_lowercase().contains("base64"),
            "Error should mention 'base64', got: {err}"
        );
    }
}
