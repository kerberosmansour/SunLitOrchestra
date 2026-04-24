# Lessons Learned — Tauri Desktop Milestone 7: Voice Input & Speech-to-Text Integration

## Design Decisions

- **Backend-proxied STT**: The `transcribe_audio` Tauri command receives base64-encoded audio from the frontend and makes the OpenAI Whisper API call on the backend. This keeps the API key secure — it never reaches the frontend code or browser DevTools.

- **dotenvy for .env loading**: Uses `dotenvy::dotenv()` (idempotent) at the start of `transcribe_audio` to load `.env` if present. This is called per-request rather than at startup, keeping the command self-contained and testable.

- **reqwest with multipart**: The Whisper API requires multipart form upload. Using `reqwest::multipart::Form` with a `Part::bytes()` for the audio data and text fields for model and response_format.

- **Three-state VoiceButton**: The button uses `data-state` attribute (`idle`, `recording`, `transcribing`) for both visual styling and test assertions. Icons change per state: 🎙 → ⏹ → ⏳.

- **useVoice hook encapsulates MediaRecorder**: The hook manages the full lifecycle — requesting microphone permission, starting/stopping recording, converting the audio blob to base64, and invoking the Tauri command. The component stays simple.

- **Transcription appends to textarea**: When voice transcription returns, it appends to existing text (with a space separator if text exists). This lets users combine typed and spoken input.

## What Was Harder Than Expected

- **E2E test with dotenvy**: The root workspace E2E tests couldn't use `dotenvy` because it wasn't a dev-dependency of the root package. Added `dotenvy = "0.15"` to root `[dev-dependencies]`.

- **MediaRecorder mocking in jsdom**: jsdom doesn't provide `MediaRecorder` or `navigator.mediaDevices.getUserMedia`. Both needed to be mocked globally in test files. The MockMediaRecorder class simulates the recording lifecycle including `ondataavailable` and `onstop` callbacks.

- **M6 test registration**: The `e2e_tauri_m6.rs` test file existed but wasn't registered as a `[[test]]` entry in root `Cargo.toml`. Added it alongside M7 registration.

## Naming Conventions Established

- **Voice command module**: `commands/voice.rs` — follows existing `commands/<feature>.rs` pattern
- **Tauri command**: `transcribe_audio` — verb_noun pattern matching existing commands
- **Component**: `VoiceButton.tsx` — PascalCase, follows existing naming
- **Hook**: `useVoice.ts` — camelCase with `use` prefix, follows React convention
- **Types**: `VoiceState`, `VoiceButtonProps` in `types/index.ts` — mirrors component name
- **E2E test files**: `tests/e2e_tauri_m7.rs` and `e2e/voice.e2e.test.tsx`

## Test Patterns That Worked Well

- **data-state attribute on VoiceButton**: Using `data-state` for test assertions is clean and decoupled from visual styling. Tests query `getAttribute("data-state")` rather than checking class names.
- **MockMediaRecorder class**: A lightweight mock that simulates the MediaRecorder API lifecycle. Reused across both BDD and E2E test files.
- **Environment variable save/restore pattern**: Tests that modify env vars save the original, modify, assert, then restore. This prevents test pollution.
- **75 frontend tests total**: 5 VoiceButton BDD + 3 Voice E2E + 67 pre-existing tests all pass.

## What the Next Milestone Should Do Differently

- **M8 (Polish & Integration)**: Consider extracting the MockMediaRecorder into a shared test utility since it's duplicated between `VoiceButton.test.tsx` and `voice.e2e.test.tsx`.
- **STT settings in SettingsPanel**: The voice feature uses a hardcoded "whisper-1" model. M8 could add STT provider/model fields to `AppSettings` and `SettingsPanel`.
- **Voice permission error UX**: Currently, if the user denies microphone permission, the error appears as a small text. M8 could add a more prominent dialog or guidance.

## BDD Scenarios to Retroactively Add

- None needed for M1–M6. All existing tests remain valid and pass.
