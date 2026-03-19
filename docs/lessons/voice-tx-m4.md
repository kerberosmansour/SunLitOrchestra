# Lessons Learned — voice-tx Milestone 4

## What changed

- Created `crates/sldo-tauri/Info.plist` with `NSMicrophoneUsageDescription` for macOS microphone permission.
- Updated `useStandaloneVoice.ts` — added empty recording guard (`audioBlob.size === 0`) that rejects before calling backend with "No audio was captured" error.
- Added 8 new BDD tests in `VoiceTranscriber.test.tsx` covering: empty recording rejection, API key missing error, OpenAI error body surfacing, network error display, buttons disabled during transcription, short recording acceptance, and blobToBase64 correctness.
- Created `tests/e2e_voice_tx_m4.rs` with 4 E2E tests: Info.plist validation, empty audio rejection, missing API key, invalid base64.
- Updated `docs/ARCHITECTURE.md` with Info.plist / macOS microphone permission documentation.
- Updated `README.md` with macOS microphone permission note.

## Design decisions and why

- **Empty recording guard in hook, not component**: The `useStandaloneVoice` hook checks `audioBlob.size === 0` before calling `blobToBase64` or `invoke`. This keeps the guard at the data layer, preventing any unnecessary base64 conversion of empty data.
- **`blobToBase64` already correct from M3**: The existing `FileReader.readAsDataURL` implementation was already correct — it strips the `data:` URI prefix at the comma. No changes needed here, only tests verifying it.
- **Start button disabled during both recording AND transcribing**: The component already had `disabled={isRecording || isTranscribing}` from M3, which satisfies the "buttons disabled during transcription" requirement.
- **Error messages passed through as-is**: Rust backend error strings (API key, network, OpenAI status) are surfaced directly to the user. No frontend error message transformation needed — the Rust messages are already user-friendly.

## Mistakes made

- The "Buttons disabled during transcription" test was initially flaky when run in the full suite due to race conditions with `mockImplementation` returning a pending promise. Fixed by adding proper microtask flushing with `setTimeout`.

## Root causes

- Promise-based mock implementations in vitest can interact unpredictably with React's batched state updates when many test files share the same jsdom environment.

## What was harder than expected

- Nothing significant. M3 laid good groundwork — the hook API was stable and the error propagation paths were already clean.

## Naming conventions established

- E2E Rust test file: `tests/e2e_voice_tx_m4.rs` — consistent with M1 and M2 patterns.
- BDD test describe block: `Feature: Integration edge cases and macOS permission (M4)` — includes milestone tag for traceability.

## Test patterns that worked well

- Subclassing `MockMediaRecorder` to `EmptyMockMediaRecorder` for the empty recording test, then restoring the original — clean isolation without affecting other tests.
- Using `mockImplementation` with a manually-resolved promise to test the "buttons disabled during transcription" scenario.

## Missing tests that should exist now

- No gaps for M4 scope. All 8 BDD scenarios and 4 E2E scenarios from the runbook are covered.

## Rules for the next milestone

- M5 is polish and documentation only — no new features.
- Frontend test count is now 129 (was 122). Backend tests unchanged.
- `Info.plist` is in place — do not modify it in M5.
- `useStandaloneVoice` hook API is stable: `{ isRecording, isTranscribing, transcript, error, startRecording, stopRecording, setTranscript }`.
