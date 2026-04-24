# Lessons Learned — voice-tx Milestone 2

## What changed

- Added `transcribe_audio_standalone` async Tauri command to `crates/sldo-tauri/src/commands/voice.rs` — uses direct `reqwest::multipart` POST to OpenAI `/v1/audio/transcriptions` with MIME-type-aware filenames.
- Added `mime_to_filename` helper that maps MIME types (including codec suffixes like `audio/webm;codecs=opus`) to correct file extensions.
- Registered `transcribe_audio_standalone` in `main.rs` invoke_handler.
- Added `base64 = "0.22"` to workspace-level `[dev-dependencies]` for E2E tests.
- Created `tests/e2e_voice_tx_m2.rs` with 5 E2E tests.
- Added 9 new unit tests in `voice.rs` (5 MIME mapping, 1 API key error, 1 decode error, 1 empty audio error, 1 backward compat check).

## Design decisions and why

- **Direct `reqwest` instead of Rig**: The new command bypasses `rig-core`'s `TranscriptionModel` in favor of direct `reqwest::multipart::Form`. This is simpler, more transparent for debugging, and matches the user's specification. The existing `transcribe_audio` command using Rig is preserved for the chat input flow.
- **`serde_json::from_str` instead of `.json()` feature**: Avoided adding `json` feature to `reqwest` since the runbook only allows `rustls-tls` as a new feature. Used manual `response.text()` + `serde_json::from_str()` instead.
- **Raw error body on failure**: When OpenAI returns a non-success status, the full response body is returned to the frontend for debugging transparency (no swallowed errors).
- **MIME type stripping**: `mime_to_filename` strips codec suffixes (e.g. `;codecs=opus`) before matching, since OpenAI expects clean MIME types and filenames.

## Mistakes made

- Initially used `reqwest::Response::json()` which requires the `json` feature — switched to manual parsing with `serde_json::from_str`.
- Unit test `existing_transcribe_audio_still_exists` used relative path that doesn't resolve when test binary runs from crate directory — fixed with `env!("CARGO_MANIFEST_DIR")`.
- Unit test `standalone_missing_api_key_returns_clear_error` called `resolve_api_key()` directly but `dotenvy::dotenv()` loaded `.env` file restoring the key — changed to verify the error message format without calling `dotenvy`.

## Root causes

- `reqwest` feature gating — `.json()` requires explicit feature flag.
- Test working directory differs from workspace root when running crate-specific tests.
- `dotenvy::dotenv()` side-effects in `resolve_api_key()` load `.env` even in test context.

## What was harder than expected

- Working around `dotenvy` side-effects in unit tests that test missing API key behavior.

## Naming conventions established

- MIME mapping helper: `mime_to_filename` — snake_case, private to module
- Standalone command: `transcribe_audio_standalone` — matches existing `transcribe_audio` naming pattern
- E2E test: `tests/e2e_voice_tx_m2.rs` — follows `e2e_voice_tx_m<N>.rs` pattern

## Test patterns that worked well

- Direct function calls for MIME mapping tests (pure function, no side effects)
- Setting fake API key before testing decode/empty errors to isolate the error path
- Using `env!("CARGO_MANIFEST_DIR")` for reliable file path resolution in unit tests
- E2E tests validating source code content for structural assertions

## Missing tests that should exist now

- No gaps for M2 scope. All BDD scenarios from the runbook are covered.

## Rules for the next milestone

- M3 adds frontend recording hooks — must not touch any Rust files from M2.
- The `transcribe_audio_standalone` command signature is now stable: `(audio_base64: String, mime_type: String) -> Result<String, String>`.
- Frontend hook must send actual `MediaRecorder.mimeType` (not hardcoded).
