# Fix Home Page Voice Recording — SunLitOrchestrate (AI-First Runbook v3)

> **Purpose**: Replace the broken `useVoice` hook (used by the home page's `VoiceButton` / `ChatInput`) with the proven recording and transcription approach from `useStandaloneVoice`, so that voice input on the home page works identically to the standalone Transcriber page.  
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.  
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [README.md](../README.md), [RUNBOOK-VOICE-TRANSCRIBER.md](RUNBOOK-VOICE-TRANSCRIBER.md)

---

## Runbook Metadata

- **Runbook ID**: `voice-fix-home`
- **Prefix for test files and lessons files**: `voice-fix`
- **Primary stack**: `Rust + Tauri + React + TypeScript`
- **Primary package/app names**: `sldo-tauri`
- **Default test commands**:
  - Backend: `cargo test -p sldo-tauri`
  - Frontend: `cd crates/sldo-tauri/ui && npm test`
  - E2E backend: `cargo test --test 'e2e_voice_fix_*'`
  - E2E frontend: N/A
  - Build/boot: `cargo build -p sldo-tauri`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `transcribe_audio` Tauri command (signature may change, see M1)
  - `transcribe_audio_standalone` Tauri command (must remain stable)
  - `VoiceButton` component props (`onTranscription: (text: string) => void`)
  - `ChatInput` component props and behavior
  - `HomeScreen` component behavior

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Align `useVoice` hook with `useStandaloneVoice` approach | `not_started` | | | | |
| 2 | Update `VoiceButton` for new hook interface | `not_started` | | | | |
| 3 | End-to-end validation and cleanup | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/voice-fix-m<N>.md -->
<!-- Completion summaries go in docs/completion/voice-fix-m<N>.md -->

---

## End-to-End Architecture Diagram

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SunLitOrchestrate — Voice Flow                       │
│                                                                             │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │                         React Frontend (Tauri WebView)                │  │
│  │                                                                        │  │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────────┐    ┌──────────────┐  │  │
│  │  │HomeScreen│───▶│ChatInput │───▶│ VoiceButton  │───▶│  useVoice    │  │  │
│  │  └──────────┘    └──────────┘    └──────────────┘    └──────┬───────┘  │  │
│  │                                                             │          │  │
│  │  ┌─────────────────┐    ┌──────────────────┐                │          │  │
│  │  │VoiceTranscriber │───▶│useStandaloneVoice│                │          │  │
│  │  └─────────────────┘    └────────┬─────────┘                │          │  │
│  │                                  │                          │          │  │
│  │         ┌────────────────────────┴──────────────────────────┘          │  │
│  │         │  Both hooks will use same approach:                          │  │
│  │         │  - selectMimeType() for MediaRecorder                       │  │
│  │         │  - FileReader-based blobToBase64                            │  │
│  │         │  - invoke("transcribe_audio_standalone", {audioBase64,      │  │
│  │         │    mimeType})                                               │  │
│  │         ▼                                                             │  │
│  └─────── IPC boundary ─────────────────────────────────────────────────┘  │
│            │                                                               │
│  ┌─────── Tauri Rust Backend ───────────────────────────────────────────┐  │
│  │         ▼                                                             │  │
│  │  ┌────────────────────────────┐                                       │  │
│  │  │transcribe_audio_standalone │─── reqwest multipart POST            │  │
│  │  └────────────┬───────────────┘                                       │  │
│  │               │                                                       │  │
│  │  ┌────────────────────────────┐  (to be removed or kept as fallback) │  │
│  │  │  transcribe_audio (Rig)   │─── Rig TranscriptionModel            │  │
│  │  └────────────┬───────────────┘                                       │  │
│  └───────────────┼───────────────────────────────────────────────────────┘  │
│                  │                                                          │
│  ════════════════╪══════════════════════════════════════════════════════════ │
│                  ▼                                                          │
│         ┌────────────────┐                                                  │
│         │  OpenAI API    │  (external: /v1/audio/transcriptions)            │
│         └────────────────┘                                                  │
│                                                                             │
│  Legend:                                                                    │
│  ─── existing    - - - changed in this runbook    ═══ external boundary    │
│  ▶ data flow                                                                │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `useVoice` hook | Record audio from mic, send to backend for transcription (used by home page ChatInput) | M1 (changed) | `startRecording()`, `stopRecording()`, `voiceState`, `transcript`, `error` |
| `VoiceButton` | Toggle mic recording on/off, display state, relay transcription to parent | M2 (changed) | `onTranscription` callback prop |
| `useStandaloneVoice` hook | Working standalone voice recording & transcription | unchanged (reference) | N/A |
| `transcribe_audio_standalone` | Rust Tauri command: direct reqwest multipart POST to OpenAI | unchanged | `(audio_base64: String, mime_type: String) -> Result<String, String>` |
| `transcribe_audio` | Rust Tauri command: Rig-based transcription (broken path) | M1 (may be removed) | `(audio_base64: String) -> Result<String, String>` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| User clicks mic button | `VoiceButton` | `useVoice` | React hook call | M2 |
| Audio recording | `useVoice` | `MediaRecorder` | Web API | M1 |
| Base64 conversion | `useVoice` | `FileReader` | Web API (blobToBase64) | M1 |
| IPC transcription call | `useVoice` | `transcribe_audio_standalone` | Tauri IPC invoke | M1 |
| OpenAI API call | `transcribe_audio_standalone` | OpenAI | HTTPS reqwest multipart | unchanged |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — This is a small bug fix involving a single-user, single-session recording flow with no concurrency, distributed state, or ordering concerns. The voice hook is a linear state machine (idle → recording → transcribing → idle) with no concurrent actors.

---

## Global Execution Rules

These rules apply to every milestone without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone unless a listed step explicitly requires one additional file.
- Do not refactor unrelated code.
- Do not rename public APIs, commands, routes, events, persisted state shapes, or config keys unless the milestone explicitly says so.
- Do not introduce a new dependency unless the milestone explicitly allows it.
- Do not change database schema, file formats, or migration behavior unless the milestone explicitly includes migration work and migration tests.

### 2) Tests define the contract

- Write BDD tests before production code.
- Write E2E runtime validation stubs before production code.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when code compiles. It is done when the declared contract is satisfied and evidence is recorded.

### 3) No placeholders in production paths

The following are not allowed unless explicitly permitted in the milestone:

- TODO or placeholder logic in production code
- silent fallbacks that hide errors
- swallowed errors without structured logging or user-visible handling
- fake implementations left in place after tests pass
- commented-out dead code
- temporary mocks in production paths
- hard-coded secrets, test keys, or unsafe defaults

### 4) Preserve backwards compatibility

Every milestone must explicitly verify that previously working user flows, commands, routes, persisted state, and public interfaces still work unless the milestone explicitly replaces them.

### 5) Prefer smallest safe change

- Prefer narrow, local modifications over broad rewrites.
- Prefer extending existing patterns over inventing new abstractions.
- Prefer deleting complexity over adding new layers.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

All meaningful checks must be recorded in the milestone Evidence Log:

- command run
- relevant file or test
- expected result
- actual result
- pass/fail
- notes

### 7) Keep .gitignore current and clean up test artifacts

- If a milestone introduces new build outputs, generated files, test fixtures, scratch directories, or tool-specific caches, add matching patterns to `.gitignore` before committing.
- Review `.gitignore` at the end of every milestone for staleness — remove patterns that no longer apply.
- Never commit test output data, temporary fixtures, scratch files, or generated artifacts to source control.
- Every test that creates files on disk must clean up after itself (use `tempdir`, `tempfile`, `afterEach` cleanup, or equivalent). Tests must not leave residual data in the working tree.
- Record the `.gitignore` review in the Evidence Log.

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, test strategy improvements, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   cargo test -p sldo-tauri
   cd crates/sldo-tauri/ui && npm test
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create BDD test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - files allowed to change
   - out-of-scope items
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test -p sldo-tauri
   cd crates/sldo-tauri/ui && npm test
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   cargo test --test 'e2e_voice_fix_*'
   ```
3. Verify the app builds and boots to a usable state.
   ```
   cargo build -p sldo-tauri
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. **Clean up test artifacts**: Verify no test output files, temporary fixtures, or generated data remain in the working tree. Run `git status` and confirm no untracked test artifacts exist.
8. **Review .gitignore**: Ensure any new build outputs, generated files, or tool caches introduced in this milestone have matching `.gitignore` patterns. Remove stale patterns that no longer apply.
9. Update ARCHITECTURE.md following the Documentation Update Table.
10. Update README.md if user-facing capabilities changed.
11. Write a lessons-learned file at `docs/lessons/voice-fix-m<N>.md`.
12. Write a completion summary at `docs/completion/voice-fix-m<N>.md`.
13. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
14. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

The SunLitOrchestrate Tauri desktop app has two voice recording flows:

1. **Standalone Transcriber page** (`VoiceTranscriber.tsx` → `useStandaloneVoice.ts` → `transcribe_audio_standalone` Tauri command): This flow **works correctly**. It uses MIME-type-aware `MediaRecorder` options, `FileReader`-based `blobToBase64` conversion, and a direct `reqwest` multipart POST to OpenAI's `/v1/audio/transcriptions` endpoint.

2. **Home page ChatInput voice button** (`VoiceButton.tsx` → `useVoice.ts` → `transcribe_audio` Tauri command): This flow is **broken**. It uses a different code path with two critical issues.

### Problem

1. **Broken base64 conversion in `useVoice`**: The `useVoice` hook at `crates/sldo-tauri/ui/src/hooks/useVoice.ts` uses a manual `btoa(binary)` approach to convert audio blobs to base64. This iterates over `Uint8Array` bytes with `String.fromCharCode` and then calls `btoa()`. This is fragile and can produce corrupt base64 for binary audio data. The working `useStandaloneVoice` hook uses `FileReader.readAsDataURL()` which is the reliable, standard approach.

2. **Wrong Tauri command**: `useVoice` calls `transcribe_audio` which uses Rig's `TranscriptionModel` abstraction with a hardcoded filename `"audio.webm"`. The working `useStandaloneVoice` calls `transcribe_audio_standalone` which uses direct `reqwest` multipart POST to OpenAI and accepts a `mimeType` parameter for proper filename resolution. The Rig-based path appears to be the broken path.

3. **No MIME type detection**: `useVoice` does not use `MediaRecorder.isTypeSupported()` to select the best MIME type. It creates a `MediaRecorder` with default settings and hardcodes `"audio/webm"` as the blob type. The working `useStandaloneVoice` uses `selectMimeType()` to pick the best supported format and passes the actual MIME type through to the backend.

4. **No empty-recording guard**: `useVoice` doesn't check for empty audio blobs before calling the backend. The working `useStandaloneVoice` checks `audioBlob.size === 0` and shows a user-friendly error.

5. **No microphone cleanup on unmount**: `useVoice` does not release the microphone stream when the component unmounts. The working `useStandaloneVoice` has a cleanup `useEffect` that calls `releaseStream()`.

### Target Architecture

After this runbook is complete, the home page `VoiceButton` will use the same proven recording and transcription approach as `VoiceTranscriber`:

```
User clicks 🎙 on HomeScreen ChatInput
  → VoiceButton → useVoice (rewritten)
    → MediaRecorder with selectMimeType()
    → FileReader blobToBase64()
    → invoke("transcribe_audio_standalone", { audioBase64, mimeType })
      → Rust: reqwest multipart POST to OpenAI
    → transcript appended to ChatInput via onTranscription callback
```

### Key Design Principles

1. **Reuse the working code path**: Do not invent a third approach. Reuse the MIME type selection, base64 conversion, and backend command from `useStandaloneVoice`.
2. **Keep the hook interface stable**: `useVoice` must continue to expose the same interface that `VoiceButton` expects (`voiceState`, `transcript`, `error`, `startRecording`, `stopRecording`).
3. **Single backend command**: After the fix, both hooks should call `transcribe_audio_standalone`. The old `transcribe_audio` Rig-based command can be left in place (it's a registered Tauri command) but is no longer called from the frontend.

### What to Keep

- `VoiceTranscriber.tsx` — must not change
- `useStandaloneVoice.ts` — must not change (reference implementation)
- `transcribe_audio_standalone` Rust command — must not change
- `ChatInput.tsx` `handleTranscription` callback pattern — must not change
- `VoiceButton` component prop interface (`onTranscription`) — must not change
- `transcribe_audio` Rust command — leave registered, do not remove (backward compat)
- All existing tests

### What to Change

- **`crates/sldo-tauri/ui/src/hooks/useVoice.ts`** — Rewrite internals to use `selectMimeType()`, `blobToBase64()` via FileReader, `transcribe_audio_standalone` Tauri command, empty-recording guard, and unmount cleanup. Keep the same external interface.
- **`crates/sldo-tauri/ui/src/components/VoiceButton.tsx`** — Minor adjustments if useVoice interface changes slightly (should be minimal).

### Global Red Lines

- No unrelated refactors
- No new dependencies
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control
- No test output data committed to source control

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first.
3. Confirm the tests fail for the expected reason.
4. Write production code to make the tests pass.
5. Re-run tests after any refactor.

### Required Test Coverage Categories

Every milestone must explicitly cover the categories that apply:

- happy path
- invalid input
- empty state / first-run state
- dependency failure / partial failure
- backward compatibility behavior

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```typescript
it("descriptive test name", () => {
  // Given: [precondition]
  // When: [action]
  // Then: [expected outcome]
});
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Backend unit tests | `#[cfg(test)] mod tests` inside the source file | Same file as production code |
| Frontend unit tests | `<module>.test.ts` | Co-located with source file |
| E2E runtime validation (backend) | `tests/e2e_voice_fix_m<N>.rs` | `tests/` |

### Test Artifact Cleanup Rules

Every test that creates files, directories, or temporary data on disk must follow these rules:

1. **Use temporary directories**: Prefer `tempdir()`, `tempfile::TempDir`, `tmp` from the test framework, or OS-provided temp locations.
2. **Clean up on completion and failure**: Use RAII patterns or `afterEach`/`afterAll` hooks.
3. **No residual state**: After the full test suite runs, `git status` must show no untracked files from test execution.

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

No new dependencies are required for this runbook. Both `reqwest` and `base64` are already used by the backend. No new frontend packages are needed.

### Migration policy

No migrations required. No persisted state changes.

### Refactor budget

`Minimal local refactor permitted in listed files only` — limited to `useVoice.ts` and `VoiceButton.tsx`.

---

## Milestone 1 — Rewrite `useVoice` to Use the Standalone Approach

### Goal

Replace the broken internals of `useVoice` with the proven recording and transcription logic from `useStandaloneVoice`. After this milestone, the `useVoice` hook records audio with proper MIME type selection, converts to base64 via `FileReader`, calls `transcribe_audio_standalone` instead of `transcribe_audio`, guards against empty recordings, and cleans up the microphone on unmount. The external interface (`voiceState`, `transcript`, `error`, `startRecording`, `stopRecording`) remains the same.

### Context

The `useVoice` hook at `crates/sldo-tauri/ui/src/hooks/useVoice.ts` currently has these bugs:
1. Uses `btoa(String.fromCharCode(...))` for base64 — fragile for binary data.
2. Calls `transcribe_audio` (Rig-based) — broken transcription path.
3. Hardcodes `"audio/webm"` blob type without MIME type detection.
4. No empty-recording guard.
5. No microphone cleanup on unmount.

The working `useStandaloneVoice` hook at `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts` has a correct implementation of all five areas.

### Contract

```
GIVEN the useVoice hook is imported and called with an onTranscription callback
WHEN startRecording() is invoked
THEN the hook requests microphone access using navigator.mediaDevices.getUserMedia
AND creates a MediaRecorder with the best supported MIME type via selectMimeType()
AND sets voiceState to "recording"

GIVEN the hook is recording
WHEN stopRecording() is invoked
THEN the MediaRecorder stops
AND collected audio chunks are combined into a Blob with the actual MIME type
AND the microphone stream tracks are stopped (released)
AND if the blob is empty, error is set to a user-friendly message
AND if the blob is non-empty, voiceState transitions to "transcribing"
AND the blob is converted to base64 using FileReader.readAsDataURL
AND invoke("transcribe_audio_standalone", { audioBase64, mimeType }) is called
AND on success, transcript is set and onTranscription callback is called
AND on failure, error is set
AND voiceState returns to "idle"

GIVEN the component using useVoice unmounts
THEN any active microphone stream is released
```

### Out of Scope

- Changes to `useStandaloneVoice.ts`
- Changes to `VoiceTranscriber.tsx`
- Changes to Rust backend commands
- Changes to `ChatInput.tsx`
- Changes to `VoiceButton.tsx` (next milestone)
- Removing the `transcribe_audio` Rust command
- Adding new dependencies

### Files Allowed to Change

| File | Nature of Change |
|---|---|
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | Rewrite internals, keep external interface |

### Files to Read Before Changing Anything

| File | Why |
|---|---|
| `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts` | Reference implementation to copy patterns from |
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | Current broken implementation to understand interface |
| `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` | Consumer of useVoice — must understand expected interface |
| `crates/sldo-tauri/ui/src/components/ChatInput.tsx` | Integrates VoiceButton — understand how transcription flows |
| `crates/sldo-tauri/ui/src/types/index.ts` | VoiceState type definition |

### BDD Acceptance Scenarios

| # | Scenario | Given | When | Then |
|---|---|---|---|---|
| 1 | Happy path: record and transcribe | Hook is idle, mic available | startRecording → stopRecording | voiceState transitions idle→recording→transcribing→idle, transcript is set, onTranscription called |
| 2 | Empty recording guard | Hook is recording, no audio data captured | stopRecording | error is set to user-friendly message, voiceState returns to idle, no backend call made |
| 3 | Mic unavailable | navigator.mediaDevices.getUserMedia not available | startRecording | error is set, voiceState stays idle |
| 4 | Backend transcription error | Recording completes but backend returns error | stopRecording triggers transcription | error is set with backend message, voiceState returns to idle |
| 5 | Unmount cleanup | Hook is recording, component unmounts | unmount | microphone stream tracks are stopped |
| 6 | MIME type selection | MediaRecorder supports audio/webm;codecs=opus | startRecording | MediaRecorder is created with that MIME type |

### Implementation Steps

1. Read `useStandaloneVoice.ts` fully.
2. Read `useVoice.ts` fully.
3. Extract shared utilities from `useStandaloneVoice.ts` into `useVoice.ts`:
   - Copy the `PREFERRED_MIME_TYPES` array
   - Copy the `selectMimeType()` function
   - Copy the `blobToBase64()` function
4. Rewrite `useVoice` internals:
   - In `startRecording`: use `selectMimeType()` and pass to `MediaRecorder` options (same as `useStandaloneVoice`)
   - Replace `recorder.onstop` handler: use `blobToBase64()` instead of `btoa`, add empty blob guard, call `transcribe_audio_standalone` with `{ audioBase64, mimeType }` instead of `transcribe_audio` with `{ audioBase64 }`
   - Add `releaseStream()` helper and call it on stop and in a cleanup `useEffect`
5. Verify the external interface (`UseVoiceReturn`) is unchanged:
   - `voiceState: VoiceState` (idle | recording | transcribing)
   - `transcript: string | null`
   - `error: string | null`
   - `startRecording: () => Promise<void>`
   - `stopRecording: () => void`
6. Run tests and verify.

### Refactor Budget

`Minimal local refactor permitted in listed files only` — `useVoice.ts` only.

### Compatibility Checklist

- [ ] `VoiceButton` still renders and functions (voiceState, startRecording, stopRecording, onTranscription callback)
- [ ] `ChatInput` still integrates `VoiceButton` without changes
- [ ] `HomeScreen` still renders `ChatInput` without changes
- [ ] `VoiceTranscriber` standalone page still works (no changes to `useStandaloneVoice`)
- [ ] `transcribe_audio` Rust command still exists and is registered (not called, but not removed)
- [ ] `transcribe_audio_standalone` Rust command still works

### Smoke Tests

- [ ] App builds without errors: `cargo build -p sldo-tauri`
- [ ] Home page renders with mic button visible
- [ ] Clicking mic button requests microphone permission
- [ ] After granting permission, voiceState shows "recording" (button changes to ⏹)
- [ ] Clicking stop triggers transcription (button shows ⏳)
- [ ] Transcription result appears in ChatInput textarea
- [ ] Errors display in VoiceButton error span
- [ ] Standalone Transcriber page still works identically

### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-tauri` | all pre-existing tests green | | | |
| Read reference hook | `useStandaloneVoice.ts` | understand working patterns | | | |
| Read broken hook | `useVoice.ts` | understand current interface | | | |
| Implementation | Rewrite `useVoice.ts` internals | contract satisfied | | | |
| Full tests | `cargo test -p sldo-tauri` | green | | | |
| Build/boot | `cargo build -p sldo-tauri` | builds cleanly | | | |
| Smoke tests | Manual verification | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current, no stale entries | | | |
| Compatibility checks | Manual verification | no regressions | | | |

---

## Milestone 2 — Update `VoiceButton` for Robustness

### Goal

Ensure `VoiceButton` correctly handles all states from the updated `useVoice` hook, including the new empty-recording error case. Verify the integration between `VoiceButton` → `useVoice` → `transcribe_audio_standalone` works end-to-end from the home page.

### Context

After M1, `useVoice` has the same internal approach as `useStandaloneVoice`. `VoiceButton` should already work because the external interface is unchanged. This milestone verifies the integration and makes any minor adjustments if needed.

### Contract

```
GIVEN VoiceButton is rendered in ChatInput on the HomeScreen
WHEN the user clicks the mic button
THEN recording starts and the button shows the stop icon (⏹)
AND when the user clicks stop, transcription runs
AND the transcribed text is appended to the ChatInput textarea via onTranscription
AND errors are displayed in the voiceError span
AND the button returns to idle state (🎙) after transcription completes or fails
```

### Out of Scope

- Changes to `useStandaloneVoice.ts`
- Changes to `VoiceTranscriber.tsx`
- Changes to `ChatInput.tsx`
- Changes to `HomeScreen.tsx`
- Changes to Rust backend commands
- Adding new dependencies

### Files Allowed to Change

| File | Nature of Change |
|---|---|
| `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` | Minor adjustments if needed for error display or state handling |

### Files to Read Before Changing Anything

| File | Why |
|---|---|
| `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` | Current component to potentially adjust |
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | Updated hook from M1 — understand new behavior |
| `crates/sldo-tauri/ui/src/components/ChatInput.tsx` | Integration point — understand how VoiceButton is used |

### BDD Acceptance Scenarios

| # | Scenario | Given | When | Then |
|---|---|---|---|---|
| 1 | Idle → recording → idle | VoiceButton is idle | Click start → Click stop → transcription completes | Visual states match: 🎙 → ⏹ → ⏳ → 🎙 |
| 2 | Error display | useVoice returns an error | Any state transition | Error is visible in `.voiceError` span |
| 3 | Transcription callback fires | Recording completes successfully | Transcription returned | `onTranscription` prop is called with transcript text |
| 4 | Button disabled during transcription | voiceState is "transcribing" | User tries to click button | Button is disabled |

### Implementation Steps

1. Read `VoiceButton.tsx` and confirm it works with updated `useVoice` without changes.
2. If no changes needed, document that in Evidence Log and proceed to smoke tests.
3. If minor adjustments needed (e.g., error display for empty-recording case), make them.
4. Run full test suite.
5. Perform smoke tests.

### Refactor Budget

`No refactor permitted beyond direct implementation`

### Compatibility Checklist

- [ ] `VoiceButton` renders correctly in all three states (idle, recording, transcribing)
- [ ] `ChatInput` receives transcription via `handleTranscription` callback
- [ ] Standalone Transcriber page still works

### Smoke Tests

- [ ] Home page mic button works end-to-end: click → record → stop → transcript appears in input
- [ ] Error states display correctly
- [ ] Standalone Transcriber page works identically to before

### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Read VoiceButton | `VoiceButton.tsx` | understand current component | | | |
| Integration check | manual | VoiceButton works with updated useVoice | | | |
| Full tests | `cargo test -p sldo-tauri` | green | | | |
| Build/boot | `cargo build -p sldo-tauri` | builds cleanly | | | |
| Smoke tests | Manual verification | all checked | | | |
| Compatibility checks | Manual verification | no regressions | | | |

---

## Milestone 3 — End-to-End Validation and Cleanup

### Goal

Write E2E validation tests confirming both voice paths (home page and standalone transcriber) work correctly. Clean up any dead code. Verify the complete integration.

### Context

After M1 and M2, the home page voice flow uses the same proven approach as the standalone transcriber. This milestone adds E2E test coverage and performs final validation.

### Contract

```
GIVEN the application is built and running
WHEN the e2e test suite runs
THEN all voice-related E2E tests pass
AND the old transcribe_audio command still compiles (backward compat)
AND both voice paths (home page and standalone) use transcribe_audio_standalone
```

### Out of Scope

- Removing the `transcribe_audio` Rust command (keep for backward compat)
- UI redesign
- New features
- Changes to any file not listed below

### Files Allowed to Change

| File | Nature of Change |
|---|---|
| `tests/e2e_voice_fix_m3.rs` | New E2E test file |
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | Remove dead import of `transcribe_audio` if still present (cleanup only) |

### Files to Read Before Changing Anything

| File | Why |
|---|---|
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | Verify it no longer imports/calls `transcribe_audio` |
| `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts` | Verify unchanged |
| `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` | Verify integration |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` | Verify unchanged |
| `crates/sldo-tauri/src/commands/voice.rs` | Backend commands still compile |

### BDD Acceptance Scenarios

| # | Scenario | Given | When | Then |
|---|---|---|---|---|
| 1 | Both Rust voice commands compile | Tauri crate with both commands registered | `cargo test -p sldo-tauri` | all tests pass |
| 2 | useVoice calls transcribe_audio_standalone | useVoice.ts source code | grep for invoke call | calls "transcribe_audio_standalone" not "transcribe_audio" |
| 3 | useVoice uses FileReader base64 | useVoice.ts source code | grep for blobToBase64 | uses FileReader-based conversion, no btoa |
| 4 | useVoice has MIME type selection | useVoice.ts source code | grep for selectMimeType | uses selectMimeType() for MediaRecorder options |
| 5 | useVoice has unmount cleanup | useVoice.ts source code | grep for useEffect cleanup | has useEffect with releaseStream cleanup |
| 6 | useStandaloneVoice unchanged | diff check | compare to known working state | no changes |

### Implementation Steps

1. Create `tests/e2e_voice_fix_m3.rs` with compile-level validation tests for the Rust backend.
2. Verify `useVoice.ts` no longer has any reference to the old `transcribe_audio` command string.
3. Verify `useVoice.ts` uses `blobToBase64` (FileReader), `selectMimeType`, and cleanup `useEffect`.
4. Verify `useStandaloneVoice.ts` and `VoiceTranscriber.tsx` are unchanged.
5. Run all tests.
6. Update documentation.

### Refactor Budget

`No refactor permitted beyond direct implementation`

### Compatibility Checklist

- [ ] `transcribe_audio` Rust command still registered and compiles
- [ ] `transcribe_audio_standalone` Rust command still works
- [ ] All pre-existing E2E tests pass
- [ ] Home page voice flow works
- [ ] Standalone Transcriber page works

### Smoke Tests

- [ ] `cargo build -p sldo-tauri` succeeds
- [ ] `cargo test -p sldo-tauri` passes
- [ ] `cargo test --test e2e_voice_fix_m3` passes
- [ ] App launches, home page mic button works
- [ ] App launches, standalone Transcriber page works

### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-tauri` | all pre-existing tests green | | | |
| E2E test created | `tests/e2e_voice_fix_m3.rs` | compiles | | | |
| Code verification | grep useVoice.ts | uses standalone approach | | | |
| Full tests | `cargo test -p sldo-tauri` | green | | | |
| E2E tests | `cargo test --test e2e_voice_fix_m3` | green | | | |
| Build/boot | `cargo build -p sldo-tauri` | builds cleanly | | | |
| Smoke tests | Manual verification | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current | | | |
| Compatibility checks | Manual verification | no regressions | | | |

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-tauri` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `cargo test -p sldo-tauri` | green | | | |
| E2E runtime | `cargo test --test 'e2e_voice_fix_*'` | green | | | |
| Build/boot | `cargo build -p sldo-tauri` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current, no stale entries | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces and compatibility requirements?
- Did I add tests for failure modes, not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update documentation to match the implementation?

---

## Appendix A — Root Cause Analysis

### Why the Home Page Voice Is Broken

| Issue | Broken (`useVoice`) | Working (`useStandaloneVoice`) |
|---|---|---|
| Base64 conversion | `btoa(String.fromCharCode(...bytes))` — fragile for binary | `FileReader.readAsDataURL` — reliable standard API |
| Backend command | `transcribe_audio` (Rig `TranscriptionModel`) | `transcribe_audio_standalone` (direct `reqwest` multipart) |
| MIME type handling | Hardcoded `"audio/webm"` blob type, no detection | `selectMimeType()` picks best supported format, passes to backend |
| Empty recording guard | None — sends empty data to backend | Checks `audioBlob.size === 0`, shows user error |
| Unmount cleanup | None — microphone stays active | `useEffect` cleanup calls `releaseStream()` |

### Fix Strategy

Copy the five working patterns from `useStandaloneVoice` into `useVoice`, keeping the `useVoice` external interface (`voiceState`, `transcript`, `error`, `startRecording`, `stopRecording`) unchanged so that `VoiceButton` and `ChatInput` require no changes.
