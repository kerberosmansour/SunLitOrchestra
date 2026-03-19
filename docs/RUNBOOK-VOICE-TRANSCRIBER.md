# Voice Transcriber Standalone App — SunLitOrchestrate (AI-First Runbook v2)

> **Purpose**: Build a standalone Tauri 2 + React + TypeScript desktop app that records microphone audio, sends it to Rust via Tauri `invoke`, and Rust calls OpenAI's `/v1/audio/transcriptions` endpoint — separate from the existing SunLitOrchestrate desktop app, reusing the existing Tauri + React patterns.  
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.  
> **Prerequisite reading**: [ARCHITECTURE.md](ARCHITECTURE.md), [README.md](../README.md), [docs/lessons/tauri-desktop-m7.md](lessons/tauri-desktop-m7.md), [docs/lessons/tauri-desktop-m8.md](lessons/tauri-desktop-m8.md)

---

## Runbook Metadata

- **Runbook ID**: `voice-tx`
- **Prefix for test files and lessons files**: `voice-tx`
- **Primary stack**: `Rust + Tauri 2 + React 18 + TypeScript`
- **Primary package/app names**: `sldo-tauri` (existing Tauri crate), frontend in `crates/sldo-tauri/ui/`
- **Default test commands**:
  - Backend: `cargo test --workspace`
  - Frontend: `cd crates/sldo-tauri/ui && npx vitest run`
  - E2E backend: `cargo test --workspace --test 'e2e_*'`
  - E2E frontend: `cd crates/sldo-tauri/ui && npx vitest run --include 'src/e2e/**'`
  - Build/boot: `cargo tauri dev`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - All existing Tauri commands: `start_planning`, `read_runbook`, `save_runbook`, `start_execution`, `cancel_execution`, `get_settings`, `update_settings`, `get_available_providers`, `get_available_models`, `transcribe_audio`
  - All existing frontend hooks: `usePlan`, `useExecution`, `useStreamingEvents`, `useVoice`
  - All existing event types: `plan-progress`, `plan-complete`, `plan-error`, `milestone-started`, `execution-progress`, `build-test-result`, `milestone-completed`, `execution-complete`
  - All existing TypeScript types in `types/index.ts`
  - Existing `AppSettings` serialization shape (persisted as JSON)

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Standalone voice transcriber page & route | `done` | 2026-03-19 | 2026-03-19 | `docs/lessons/voice-tx-m1.md` | Added transcriber phase, VoiceTranscriber component, sidebar button, routing. 8 new frontend + 2 E2E tests. |
| 2 | Rust transcription backend with direct reqwest | `in_progress` | 2026-03-19 | | | |
| 3 | React recording UI with MediaRecorder | `not_started` | | | | |
| 4 | End-to-end wiring, error handling & macOS permission | `not_started` | | | | |
| 5 | Polish, production shape guidance & documentation | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/voice-tx-m<N>.md -->
<!-- Completion summaries go in docs/completion/voice-tx-m<N>.md -->

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

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, test strategy improvements, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   cargo test --workspace
   cd crates/sldo-tauri/ui && npx vitest run
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create BDD test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   cd crates/sldo-tauri/ui && npx vitest run
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   cargo test --workspace --test 'e2e_*'
   cd crates/sldo-tauri/ui && npx vitest run --include 'src/e2e/**'
   ```
3. Verify the app builds and boots to a usable state.
   ```
   cargo tauri dev
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. Update ARCHITECTURE.md following the Documentation Update Table.
8. Update README.md if user-facing capabilities changed.
9. Write a lessons-learned file at `docs/lessons/voice-tx-m<N>.md`.
10. Write a completion summary at `docs/completion/voice-tx-m<N>.md`.
11. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
12. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

SunLitOrchestrate is an AI-driven software development toolkit with two interfaces:

1. **CLI tools** — `sldo-plan` (runbook generation) and `sldo-run` (milestone execution)
2. **Tauri v2 desktop app** — A graphical interface wrapping the same backend logic

The Tauri app already has a voice input feature (added in M7 of the Tauri Desktop runbook):
- **Backend**: `crates/sldo-tauri/src/commands/voice.rs` — `transcribe_audio` Tauri command that uses the `rig-core` crate's `TranscriptionModel` abstraction to call OpenAI's `gpt-4o-transcribe` model. API key read from `.env` via `dotenvy`.
- **Frontend**: `crates/sldo-tauri/ui/src/hooks/useVoice.ts` — React hook managing MediaRecorder lifecycle, base64 encoding, and Tauri invoke. `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` — three-state button (idle/recording/transcribing).
- **Integration**: `ChatInput.tsx` imports `VoiceButton` and appends transcriptions to the text area.

The existing Cargo dependencies already include `reqwest = { version = "0.12", features = ["multipart"] }`, `base64 = "0.22"`, `serde`/`serde_json`, and `rig-core = { version = "0.33", features = ["audio"] }` in `crates/sldo-tauri/Cargo.toml`.

### Problem

The user wants a **standalone voice transcriber page** that is a dedicated, focused recording-and-transcribing interface — not the existing chat-input-embedded voice button. The key changes are:

1. **Standalone UI**: A dedicated recording page with start/stop buttons, a status display, and a textarea for the transcript — matching the design described in the user's specification (a "Tauri Voice Transcriber" page separate from the chatbot flow).
2. **Direct reqwest instead of rig-core (optional)**: The user's specification shows a direct `reqwest` + multipart approach calling OpenAI's `/v1/audio/transcriptions` endpoint, which is a simpler and more transparent implementation. The existing `rig-core` approach works but adds a dependency layer. This runbook will add a second Tauri command using direct `reqwest` for the standalone page while preserving the existing `transcribe_audio` command.
3. **MIME type awareness**: The user's specification passes `mime_type` from the frontend so the Rust backend picks the correct filename extension for the OpenAI upload. The existing implementation hardcodes `audio.webm`.
4. **macOS microphone permission**: Need to add `NSMicrophoneUsageDescription` to an `Info.plist` for bundled macOS distribution.
5. **Production security guidance**: Documented guidance about not shipping shared API keys in distributed apps.

### Target Architecture

```
crates/sldo-tauri/
├── src/
│   ├── main.rs              # Adds route to standalone transcriber page
│   ├── commands/
│   │   ├── voice.rs          # EXTENDED: add transcribe_audio_standalone (direct reqwest)
│   │   └── (existing modules unchanged)
│   └── (existing modules unchanged)
├── Info.plist                # NEW: macOS microphone permission
└── ui/
    └── src/
        ├── App.tsx           # EXTENDED: add "transcriber" phase/route
        ├── types/index.ts    # EXTENDED: add TranscriberPhase, standalone types
        ├── components/
        │   ├── VoiceTranscriber.tsx     # NEW: standalone recording + transcript UI
        │   ├── VoiceTranscriber.test.tsx # NEW: BDD tests
        │   └── (existing components unchanged)
        ├── hooks/
        │   ├── useStandaloneVoice.ts    # NEW: hook with MIME-type-aware recording
        │   └── (existing hooks unchanged)
        └── e2e/
            └── transcriber.e2e.test.tsx # NEW: E2E tests

tests/
└── e2e_voice_tx_m<N>.rs     # NEW: backend E2E tests per milestone
```

### Key Design Principles

1. **API key stays in Rust**: The OpenAI API key is read from the environment or `.env` file by the Rust backend only. It is never sent to or accessible from the frontend. This follows OpenAI's guidance that API keys should not be exposed in client-side code.
2. **Direct reqwest for transparency**: The standalone transcriber uses direct `reqwest::multipart` to call OpenAI, making the HTTP interaction clear and debuggable without abstraction layers.
3. **MIME type forwarded from frontend**: The frontend reports the `MediaRecorder.mimeType` to the backend so the correct filename extension is used in the multipart upload to OpenAI.
4. **Existing voice feature preserved**: The current `transcribe_audio` command and `VoiceButton`/`useVoice` remain unchanged. The standalone feature is additive.
5. **Local development only**: This is a local dev tool. For distribution, the direct OpenAI call should be replaced with a backend proxy to avoid shipping shared API keys.

### What to Keep

- `crates/sldo-tauri/src/commands/voice.rs` — existing `transcribe_audio` command (rig-core based)
- `crates/sldo-tauri/ui/src/hooks/useVoice.ts` — existing voice hook
- `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` — existing voice button
- `crates/sldo-tauri/ui/src/components/ChatInput.tsx` — existing chat input integration
- All existing Tauri commands, events, state, settings, and types
- All existing tests (90 frontend, 200+ backend)

### What to Change

- **`crates/sldo-tauri/src/commands/voice.rs`** — Add `transcribe_audio_standalone` command using direct reqwest with MIME type parameter
- **`crates/sldo-tauri/src/main.rs`** — Register the new command
- **`crates/sldo-tauri/ui/src/App.tsx`** — Add "transcriber" phase and route
- **`crates/sldo-tauri/ui/src/types/index.ts`** — Add standalone transcriber types
- **`crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx`** — NEW: standalone recording page
- **`crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts`** — NEW: MIME-type-aware recording hook
- **`crates/sldo-tauri/Info.plist`** — NEW: macOS microphone permission entry

### Global Red Lines

These are forbidden unless explicitly overridden inside a milestone.

- No unrelated refactors
- No new dependencies (reqwest, base64, serde already present in Cargo.toml)
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control
- No breaking changes to existing `transcribe_audio` command or `VoiceButton`

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
- retry or rollback behavior if relevant
- concurrency or race behavior if relevant
- persistence / restore behavior if relevant
- backward compatibility behavior

If a category does not apply, state why.

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition]
    // When: [action]
    // Then: [expected outcome]
}
```

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
| Backend E2E tests | `tests/e2e_voice_tx_m<N>.rs` | `tests/` |
| Frontend unit/BDD tests | `<module>.test.tsx` | Co-located with source file |
| Frontend E2E tests | `e2e/transcriber.e2e.test.tsx` | `crates/sldo-tauri/ui/src/e2e/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go beyond compilation and verify that the system works correctly at runtime. These tests prove:

1. the app boots without errors
2. runtime contracts are met across IPC/API boundaries
3. BDD scenarios work at runtime, not just in isolation
4. there are no runtime panics, unhandled rejections, or silent failures
5. degraded states behave safely and visibly

### E2E Test Design Rules

1. Test runtime behavior, not just types.
2. Test the full stack where possible.
3. Test degraded and failure states, not just the happy path.
4. Assert against observable behavior.
5. Prefer at least one test that crosses the backend/frontend boundary when both layers changed.

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

No new crate or npm dependencies are needed for this runbook. All required Rust crates (`reqwest` with `multipart`, `base64`, `serde`, `serde_json`, `dotenvy`) are already in `crates/sldo-tauri/Cargo.toml`. The `reqwest` crate needs the `rustls-tls` feature added (it currently only has `multipart`).

If the `rustls-tls` feature is needed, Milestone 2 explicitly allows adding it to the existing `reqwest` dependency.

### Migration policy

No migrations are needed. No persisted state shapes change.

### Refactor budget

Each milestone states its own refactor budget. The default is: `No refactor permitted beyond direct implementation`.

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| Baseline frontend | `cd crates/sldo-tauri/ui && npx vitest run` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| Full frontend | `cd crates/sldo-tauri/ui && npx vitest run` | green | | | |
| E2E runtime | `cargo test --workspace --test 'e2e_*'` | green | | | |
| Build/boot | `cargo tauri dev` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
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
- Is every assumption either verified or explicitly documented as unresolved?
- Is the milestone truly done according to its Definition of Done?

If any answer is "no", the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/lessons/voice-tx-m<N>.md`

```md
# Lessons Learned — voice-tx Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Naming conventions established
- [types, files, tests, events, commands]

## Test patterns that worked well
- [pattern]

## Missing tests that should exist now
- [test]

## Rules for the next milestone
- [rule]

## Template improvements suggested
- [improvement]
```

---

## Completion Summary Template

Path: `docs/completion/voice-tx-m<N>.md`

```md
# Completion Summary — voice-tx Milestone <N>

## Goal completed
- [what capability now exists]

## Files changed
- [file]
- [file]

## Tests added
- [test file]
- [test file]

## Runtime validations added
- [e2e file]

## Compatibility checks performed
- [check]

## Documentation updated
- [doc and section]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## Milestone Plan

### Milestone 1 — Standalone Voice Transcriber Page & Route

**Goal**: Add a new "transcriber" phase/route to the Tauri app that renders a dedicated `VoiceTranscriber` page with placeholder UI. The page is accessible from the sidebar. No actual recording or transcription yet — just the React component skeleton, types, and routing.

**Context**: The app currently has phases: `home`, `planning`, `reviewing`, `executing`, `settings`. This milestone adds a `transcriber` phase and a new `VoiceTranscriber` component that will become the standalone recording interface. The existing `VoiceButton` on `ChatInput` remains unchanged. The sidebar gets a new "Transcriber" button to navigate to this page.

**Important design rule**: The `VoiceTranscriber` component must be fully separate from `VoiceButton` — it does not reuse `useVoice` but will later use its own `useStandaloneVoice` hook. This keeps the existing chat-embedded voice feature completely isolated.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | User clicks "Transcriber" in sidebar |
| Outputs | VoiceTranscriber page renders with start/stop buttons (disabled), transcript textarea (empty), status area |
| Interfaces touched | `AppPhase` type, `App.tsx` routing, `Sidebar.tsx` navigation |
| Files allowed to change | `crates/sldo-tauri/ui/src/types/index.ts`, `crates/sldo-tauri/ui/src/App.tsx`, `crates/sldo-tauri/ui/src/components/Sidebar.tsx` |
| Files to read before changing anything | `crates/sldo-tauri/ui/src/App.tsx`, `crates/sldo-tauri/ui/src/types/index.ts`, `crates/sldo-tauri/ui/src/components/Sidebar.tsx`, `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` |
| New files allowed | `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx`, `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx`, `tests/e2e_voice_tx_m1.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing phases, commands, events, and components continue to work. Existing 90 frontend + 200 backend tests still pass. |
| Forbidden shortcuts | No mocks in production, no TODOs, no silent fallbacks |

#### Out of Scope / Must Not Do

- Do not add actual recording logic (that's M3)
- Do not add Rust transcription commands (that's M2)
- Do not modify `VoiceButton`, `useVoice`, or `ChatInput`
- Do not add new npm or Rust dependencies
- Do not change `AppSettings` shape

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/tauri-desktop-m8.md` and apply relevant conventions (component naming, test patterns, keyboard shortcuts).
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-tauri/ui/src/types/index.ts` | Add `"transcriber"` to `AppPhase` union type |
| `crates/sldo-tauri/ui/src/App.tsx` | Add `transcriber` phase rendering and navigation handler |
| `crates/sldo-tauri/ui/src/components/Sidebar.tsx` | Add "Transcriber" navigation button |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` | NEW: standalone transcriber page component |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx` | NEW: BDD tests for the component |
| `tests/e2e_voice_tx_m1.rs` | NEW: backend E2E — verify app still compiles with new route |

#### Step-by-Step

1. Write BDD test stubs for `VoiceTranscriber.test.tsx` — test that the component renders with expected elements.
2. Write E2E stub `tests/e2e_voice_tx_m1.rs` — verify workspace compiles.
3. Add `"transcriber"` to the `AppPhase` type union in `types/index.ts`.
4. Create `VoiceTranscriber.tsx` with the UI skeleton: heading "Tauri Voice Transcriber", description text, start/stop buttons (disabled), transcript textarea, error display area.
5. Update `App.tsx` to render `VoiceTranscriber` when phase is `"transcriber"`, add `handleSelectTranscriber` callback.
6. Update `Sidebar.tsx` to include a "Transcriber" button that calls `onSelectTranscriber`.
7. Make all BDD tests pass.
8. Run the full test suite.
9. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Standalone voice transcriber page**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Transcriber page renders heading | happy path | VoiceTranscriber mounted | Component renders | Heading "Tauri Voice Transcriber" is visible |
| Transcriber page renders description | happy path | VoiceTranscriber mounted | Component renders | Description text about recording is visible |
| Start button renders disabled initially | empty state | VoiceTranscriber mounted, no hook connected | Component renders | "Start recording" button is present and disabled |
| Stop button renders disabled initially | empty state | VoiceTranscriber mounted, no recording active | Component renders | "Stop recording" button is present and disabled |
| Transcript textarea renders empty | empty state | VoiceTranscriber mounted | Component renders | Textarea with placeholder text is visible and empty |
| Sidebar shows transcriber button | happy path | Sidebar rendered | User sees sidebar | "Transcriber" button/link is visible |
| Existing phases still work | backward compat | App rendered in "home" phase | User navigates to home | HomeScreen renders correctly |

#### Regression Tests

- All existing frontend tests (90) must still pass
- All existing backend tests (200+) must still pass
- `App.test.tsx` existing tests remain green
- `Sidebar.test.tsx` existing tests remain green

#### Compatibility Checklist

- [ ] `AppPhase` type still includes all existing values (`home`, `planning`, `reviewing`, `executing`, `settings`)
- [ ] All existing sidebar buttons still work
- [ ] All existing app phases still render their correct components
- [ ] Existing keyboard shortcuts still work (Cmd+N, Cmd+,, Escape)
- [ ] Existing tests for App, Sidebar pass unchanged

#### E2E Runtime Validation

**File**: `tests/e2e_voice_tx_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `workspace_compiles_with_transcriber_route` | Adding the transcriber route doesn't break the build | `cargo test --workspace` passes |
| `app_phase_type_includes_transcriber` | The new phase is valid in the type system | TypeScript compiles without errors |

**File**: `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| All BDD scenarios above | Component renders expected elements | All assertions pass |

#### Smoke Tests

- [ ] `cargo test --workspace` passes
- [ ] `cd crates/sldo-tauri/ui && npx vitest run` passes
- [ ] App launches with `cargo tauri dev`
- [ ] Clicking "Transcriber" in sidebar shows the transcriber page
- [ ] Clicking back to "Home" or other sidebar items returns to previous views

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Baseline frontend | `cd crates/sldo-tauri/ui && npx vitest run` | all green | | | |
| BDD tests created | `VoiceTranscriber.test.tsx` | fail (component missing) | | | |
| Implementation | Add types, component, routing, sidebar | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| Full frontend | `cd crates/sldo-tauri/ui && npx vitest run` | green | | | |
| Smoke tests | Navigate to transcriber page | renders correctly | | | |
| Compatibility checks | Existing phases work | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- all listed BDD scenarios pass
- full existing test suite remains green (90 frontend + 200 backend)
- `VoiceTranscriber` component renders heading, description, start/stop buttons, and textarea
- sidebar includes "Transcriber" navigation button
- `AppPhase` type includes `"transcriber"`
- `App.tsx` routes to `VoiceTranscriber` for the `"transcriber"` phase
- no forbidden shortcuts remain in production code
- lessons file is written
- completion summary is written
- Milestone Tracker is updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Add "Voice Transcriber" section under Tauri Desktop App describing the new standalone page
- **README.md**: No update needed yet (page is not user-visible until M3)

#### Notes

- Retry/rollback, concurrency, and persistence categories do not apply — this milestone only adds a static UI component with no state management.
- The start/stop buttons are intentionally disabled in this milestone; they will be wired in M3.

---

### Milestone 2 — Rust Transcription Backend with Direct reqwest

**Goal**: Add a new Tauri command `transcribe_audio_standalone` that takes `audio_base64` and `mime_type` from the frontend, decodes the audio, determines the correct file extension from the MIME type, and calls OpenAI's `/v1/audio/transcriptions` endpoint with `reqwest::multipart`. Returns the transcribed text.

**Context**: The existing `transcribe_audio` command in `voice.rs` uses `rig-core`'s `TranscriptionModel` abstraction and hardcodes the filename as `audio.webm`. The new command uses direct `reqwest` with a MIME-type-aware filename, matching the user's specification for transparency and debuggability. Both commands coexist — the existing one for the chat input, the new one for the standalone page.

**Important design rule**: The OpenAI API key must be read from the environment via `dotenvy`, never hardcoded. The command returns OpenAI's raw error body on failure for easy debugging.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `audio_base64: String` (base64-encoded audio), `mime_type: String` (MIME type from MediaRecorder) |
| Outputs | `Result<String, String>` — transcript text on success, descriptive error on failure |
| Interfaces touched | `commands/voice.rs`, `main.rs` (command registration) |
| Files allowed to change | `crates/sldo-tauri/src/commands/voice.rs`, `crates/sldo-tauri/src/main.rs`, `crates/sldo-tauri/Cargo.toml` |
| Files to read before changing anything | `crates/sldo-tauri/src/commands/voice.rs`, `crates/sldo-tauri/src/main.rs`, `crates/sldo-tauri/Cargo.toml` |
| New files allowed | `tests/e2e_voice_tx_m2.rs` |
| New dependencies allowed | `rustls-tls` feature added to existing `reqwest` dependency (if not already present) |
| Migration allowed | `no` |
| Compatibility commitments | Existing `transcribe_audio` command unchanged. All existing tests pass. |
| Forbidden shortcuts | No hardcoded API keys, no TODOs, no swallowed errors, no placeholders |

#### Out of Scope / Must Not Do

- Do not modify the existing `transcribe_audio` function
- Do not modify any frontend code (that's M3)
- Do not change `AppSettings` or state
- Do not add new event types
- Do not modify provider architecture

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/voice-tx-m1.md` and apply relevant corrections.
3. Read `crates/sldo-tauri/src/commands/voice.rs` and `crates/sldo-tauri/Cargo.toml` before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-tauri/src/commands/voice.rs` | Add `transcribe_audio_standalone` function with direct reqwest + MIME type logic |
| `crates/sldo-tauri/src/main.rs` | Register `commands::voice::transcribe_audio_standalone` in `invoke_handler` |
| `crates/sldo-tauri/Cargo.toml` | Add `rustls-tls` feature to `reqwest` if not present |
| `tests/e2e_voice_tx_m2.rs` | NEW: E2E test verifying command registration and error paths |

#### Step-by-Step

1. Write unit test stubs in `voice.rs` `#[cfg(test)]` module for the new function's logic (MIME-to-extension mapping, base64 decode error, empty audio error, missing API key).
2. Write E2E test stub `tests/e2e_voice_tx_m2.rs`.
3. Add `rustls-tls` feature to `reqwest` in `Cargo.toml` if needed.
4. Implement `transcribe_audio_standalone` in `voice.rs`:
   - Take `audio_base64: String` and `mime_type: String`
   - Load API key from environment via `dotenvy`
   - Decode base64 audio
   - Map MIME type to file extension (`audio/webm` → `recording.webm`, `audio/wav` → `recording.wav`, `audio/ogg` → `recording.ogg`, `audio/mp4`/`audio/m4a` → `recording.m4a`, default → `recording.webm`)
   - Build `reqwest::multipart::Form` with `file` part and `model` = `gpt-4o-transcribe`
   - POST to `https://api.openai.com/v1/audio/transcriptions` with bearer auth
   - Parse JSON response for `text` field
   - Return raw error body on non-success status
5. Register the command in `main.rs`.
6. Make all unit tests pass.
7. Run the full test suite.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Standalone transcription Tauri command**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| MIME type webm maps to .webm extension | happy path | mime_type is "audio/webm;codecs=opus" | Extension is resolved | Returns "recording.webm" |
| MIME type wav maps to .wav extension | happy path | mime_type is "audio/wav" | Extension is resolved | Returns "recording.wav" |
| MIME type ogg maps to .ogg extension | happy path | mime_type is "audio/ogg" | Extension is resolved | Returns "recording.ogg" |
| MIME type mp4 maps to .m4a extension | happy path | mime_type is "audio/mp4" | Extension is resolved | Returns "recording.m4a" |
| Unknown MIME type defaults to .webm | empty state / fallback | mime_type is "audio/unknown" | Extension is resolved | Returns "recording.webm" |
| Missing API key returns clear error | dependency failure | OPENAI_API_KEY not set | transcribe_audio_standalone called | Returns error mentioning "API key" |
| Invalid base64 returns decode error | invalid input | audio_base64 is "not-valid-base64!!!" | transcribe_audio_standalone called | Returns error mentioning "decode" |
| Empty audio bytes returns error | invalid input | audio_base64 decodes to 0 bytes | transcribe_audio_standalone called | Returns error mentioning "No audio" or "empty" |
| Existing transcribe_audio still works | backward compat | Existing command registered | Command invoked | Works as before |

#### Regression Tests

- All existing `voice.rs` unit tests must still pass
- All existing backend tests (200+) must still pass
- Existing `transcribe_audio` command remains registered and functional
- `e2e_tauri_m7.rs` tests still pass

#### Compatibility Checklist

- [ ] Existing `transcribe_audio` command still registered in `main.rs`
- [ ] Existing `transcribe_audio` function signature unchanged
- [ ] All existing voice.rs unit tests pass
- [ ] All existing E2E tests pass
- [ ] `cargo build -p sldo-tauri` succeeds

#### E2E Runtime Validation

**File**: `tests/e2e_voice_tx_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `standalone_command_registered` | New command is listed in invoke_handler | Build succeeds, command handler compiles |
| `mime_type_resolution_unit` | MIME→extension mapping is correct | All 5 MIME types resolve to expected extensions |
| `missing_api_key_error` | Missing env var produces user-friendly error | Error string contains "API key" |
| `invalid_base64_error` | Malformed base64 is caught before network call | Error string contains "decode" or "base64" |
| `empty_audio_rejected` | Zero-length audio caught before network call | Error string contains "empty" or "No audio" |

#### Smoke Tests

- [ ] `cargo test --workspace` passes
- [ ] `cargo build -p sldo-tauri` succeeds
- [ ] Existing `transcribe_audio` tests in `e2e_tauri_m7.rs` pass

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | `voice.rs` unit tests | fail (function missing) | | | |
| E2E stubs created | `tests/e2e_voice_tx_m2.rs` | fail (function missing) | | | |
| Implementation | Add `transcribe_audio_standalone` | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| Smoke tests | Build succeeds | compiles cleanly | | | |
| Compatibility checks | Existing voice tests | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- `transcribe_audio_standalone` command exists in `voice.rs`
- Command takes `audio_base64: String` and `mime_type: String`, returns `Result<String, String>`
- MIME-to-extension mapping handles webm, wav, ogg, mp4/m4a with webm default
- API key read from environment via dotenvy (never hardcoded)
- Direct reqwest multipart POST to OpenAI `/v1/audio/transcriptions`
- Raw error body returned on non-success for debuggability
- Command registered in `main.rs` invoke_handler
- All new unit tests pass
- All existing tests pass (90 frontend + 200 backend)
- lessons file written
- completion summary written
- Milestone Tracker updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Document the new `transcribe_audio_standalone` command in the Tauri Commands table
- **README.md**: No update needed yet

#### Notes

- The actual runtime integration with OpenAI cannot be validated in automated tests without a real API key. The E2E tests validate error handling paths and the MIME mapping logic. Manual smoke testing with a real API key is expected during M4.
- Concurrency and persistence categories do not apply to this milestone.

---

### Milestone 3 — React Recording UI with MediaRecorder

**Goal**: Implement the `useStandaloneVoice` hook and wire the `VoiceTranscriber` component to record audio using `MediaRecorder`, convert it to base64, and call the `transcribe_audio_standalone` Tauri command. The start/stop buttons become functional.

**Context**: M1 created the page skeleton. M2 created the backend command. This milestone connects them: the frontend hook manages the MediaRecorder lifecycle (requesting microphone permission, recording chunks, combining them on stop, base64-encoding, invoking the Tauri command) and the component displays recording state, transcribing state, errors, and the resulting transcript in the textarea.

**Important design rule**: The hook must report the actual `MediaRecorder.mimeType` to the backend (not hardcode `audio/webm`). It must prefer `audio/webm;codecs=opus` but fall back to whatever the browser supports. The transcript textarea must be editable by the user after transcription.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | User clicks "Start recording" → speaks → clicks "Stop recording" |
| Outputs | Transcript text appears in editable textarea; error messages display on failure; microphone is released on stop |
| Interfaces touched | `VoiceTranscriber.tsx`, new `useStandaloneVoice.ts` hook, Tauri `invoke("transcribe_audio_standalone", ...)` |
| Files allowed to change | `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx`, `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx` |
| Files to read before changing anything | `crates/sldo-tauri/ui/src/hooks/useVoice.ts`, `crates/sldo-tauri/ui/src/components/VoiceButton.tsx`, `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` |
| New files allowed | `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts`, `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.test.ts`, `crates/sldo-tauri/ui/src/e2e/transcriber.e2e.test.tsx` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing `useVoice` hook unchanged. Existing `VoiceButton` unchanged. All existing tests pass. |
| Forbidden shortcuts | No mocks in production, no TODOs, no silent fallbacks, no hardcoded MIME types |

#### Out of Scope / Must Not Do

- Do not modify `useVoice.ts` or `VoiceButton.tsx`
- Do not modify any Rust code
- Do not add new Tauri commands
- Do not change the App routing logic (already done in M1)
- Do not add new npm dependencies

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/voice-tx-m2.md` and apply relevant corrections.
3. Read `useVoice.ts` to understand the existing recording pattern.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts` | NEW: hook for standalone recording with MIME-type awareness |
| `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.test.ts` | NEW: BDD tests for the hook |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` | Wire hook: start/stop buttons functional, status display, error display, transcript textarea |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx` | Update tests for recording/transcribing/error states |
| `crates/sldo-tauri/ui/src/e2e/transcriber.e2e.test.tsx` | NEW: E2E tests for the full component lifecycle |

#### Step-by-Step

1. Write BDD test stubs for `useStandaloneVoice.test.ts`.
2. Write updated BDD test stubs for `VoiceTranscriber.test.tsx` covering interactive states.
3. Write E2E stub `transcriber.e2e.test.tsx`.
4. Implement `useStandaloneVoice` hook:
   - `startRecording()`: request microphone with `getUserMedia({ audio: true })`, create `MediaRecorder` with preferred MIME types (`audio/webm;codecs=opus`, `audio/webm`, `audio/mp4`, `audio/ogg;codecs=opus`), collect chunks via `ondataavailable`, store recorder/stream refs.
   - `stopRecording()`: call `recorder.stop()`, in `onstop` callback: combine chunks into Blob, convert to base64 via `FileReader.readAsDataURL`, call `invoke("transcribe_audio_standalone", { audioBase64, mimeType })`.
   - State: `isRecording`, `isTranscribing`, `transcript`, `error`.
   - Cleanup: release microphone tracks on stop and on unmount.
5. Update `VoiceTranscriber.tsx`:
   - Import and use `useStandaloneVoice`.
   - Start button: enabled when idle, calls `startRecording()`, shows "Recording…" when active.
   - Stop button: enabled when recording, calls `stopRecording()`.
   - Status display: "Listening to your microphone…" when recording, "Transcribing with OpenAI…" when transcribing.
   - Error display: red-bordered box when error is set.
   - Transcript textarea: shows transcript text, editable by user.
6. Make all BDD tests pass.
7. Run the full test suite.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Standalone voice recording UI**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Start button begins recording | happy path | VoiceTranscriber idle | User clicks "Start recording" | Button shows "Recording…", stop button enabled |
| Stop button triggers transcription | happy path | Recording in progress | User clicks "Stop recording" | Recording stops, status shows "Transcribing with OpenAI…" |
| Transcript appears after success | happy path | Recording stopped, backend returns text | Transcription completes | Textarea contains transcript text |
| Transcript textarea is editable | happy path | Transcript displayed | User types in textarea | New text appears in textarea |
| Error displayed on failure | dependency failure | Backend returns error | Transcription fails | Red error box visible with error message |
| Error cleared on new recording | happy path | Previous error displayed | User clicks "Start recording" | Error clears |
| Microphone released on stop | happy path | Recording in progress | User clicks stop | All media tracks stopped |
| Microphone released on unmount | cleanup | Recording in progress | Component unmounts | All media tracks stopped |
| Status shows listening during recording | happy path | Recording in progress | User observes UI | "Listening to your microphone…" visible |
| Status shows transcribing after stop | happy path | Recording stopped | Backend called | "Transcribing with OpenAI…" visible |
| Permission denied shows error | dependency failure | User denies microphone | startRecording called | Error message about microphone permissions |
| MIME type preference order | happy path | webm;codecs=opus supported | Recording starts | MediaRecorder uses audio/webm;codecs=opus |

#### Regression Tests

- All existing frontend tests (90) must still pass
- All existing backend tests (200+) must still pass
- `VoiceButton.test.tsx` tests unchanged and passing
- `useVoice` hook tests unchanged and passing

#### Compatibility Checklist

- [ ] Existing `VoiceButton` component works unchanged
- [ ] Existing `useVoice` hook works unchanged
- [ ] ChatInput voice integration still works
- [ ] All existing frontend tests pass
- [ ] All existing backend tests pass

#### E2E Runtime Validation

**File**: `crates/sldo-tauri/ui/src/e2e/transcriber.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `start_recording_updates_ui_state` | MediaRecorder lifecycle works | UI transitions to recording state |
| `stop_recording_invokes_backend` | Full recording→transcription flow | Tauri invoke called with base64 + mimeType |
| `error_state_renders_correctly` | Error handling works end-to-end | Error message visible after backend failure |
| `component_cleanup_releases_mic` | Microphone released on unmount | Media tracks stopped |

#### Smoke Tests

- [ ] `cd crates/sldo-tauri/ui && npx vitest run` passes (all tests including new ones)
- [ ] `cargo test --workspace` passes
- [ ] App launches, navigate to Transcriber page
- [ ] Start recording button becomes active
- [ ] Click Start → "Recording…" state visible
- [ ] Click Stop → "Transcribing with OpenAI…" shows
- [ ] (With API key) Transcript appears in textarea

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline frontend | `cd crates/sldo-tauri/ui && npx vitest run` | all green | | | |
| BDD tests created | `useStandaloneVoice.test.ts`, `VoiceTranscriber.test.tsx` | fail (not implemented) | | | |
| E2E stubs created | `transcriber.e2e.test.tsx` | fail (not implemented) | | | |
| Implementation | Hook + component wiring | contract satisfied | | | |
| Full frontend tests | `cd crates/sldo-tauri/ui && npx vitest run` | green | | | |
| Full backend tests | `cargo test --workspace` | green | | | |
| Smoke tests | Recording flow in app | works end-to-end | | | |
| Compatibility checks | Existing VoiceButton tests | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- `useStandaloneVoice` hook exists with `startRecording`, `stopRecording`, `isRecording`, `isTranscribing`, `transcript`, `error`
- Hook prefers `audio/webm;codecs=opus` MIME type with fallbacks
- Hook sends actual `mimeType` from MediaRecorder to backend (not hardcoded)
- Hook converts audio to base64 and calls `transcribe_audio_standalone` Tauri command
- Hook releases microphone on stop and unmount
- `VoiceTranscriber` component wired with functional start/stop buttons
- Status display shows recording/transcribing messages
- Error display shows backend errors in red box
- Transcript textarea is editable
- All new BDD and E2E tests pass
- All existing tests pass (90+ frontend + 200 backend)
- lessons file written
- completion summary written
- Milestone Tracker updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Document the `useStandaloneVoice` hook and `VoiceTranscriber` component
- **README.md**: No update needed yet

#### Notes

- Persistence and concurrency categories do not apply.
- The retry category does not apply — recording is a single-shot operation.
- Mock `MediaRecorder` and `navigator.mediaDevices.getUserMedia` in jsdom tests, following the pattern from `VoiceButton.test.tsx` and `voice.e2e.test.tsx`.

---

### Milestone 4 — End-to-End Wiring, Error Handling & macOS Permission

**Goal**: Complete the full end-to-end integration: verify the React→Rust→OpenAI flow works at runtime, add `Info.plist` for macOS microphone permission, handle all edge cases (empty recordings, network errors, API errors), and add the `blobToBase64` helper using `FileReader` as specified.

**Context**: M2 built the backend command. M3 built the frontend recording UI. This milestone focuses on integration quality: verifying the full stack works together, handling edge cases robustly, adding the macOS permission plist, and ensuring the `blobToBase64` helper in the hook uses `FileReader.readAsDataURL` (as the user specification shows) rather than the `ArrayBuffer` approach used in the existing `useVoice` hook.

**Important design rule**: Error messages from OpenAI must be surfaced to the user for debuggability. The `Info.plist` must include `NSMicrophoneUsageDescription` for macOS microphone access.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — the `blobToBase64` conversion in `useStandaloneVoice` may be adjusted to use `FileReader.readAsDataURL` pattern.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | All edge case inputs: empty recording, short recording, no API key, invalid API key, network failure, API error response |
| Outputs | Clear error messages for every failure mode; macOS microphone prompt on first use |
| Interfaces touched | `useStandaloneVoice.ts`, `VoiceTranscriber.tsx`, `Info.plist` |
| Files allowed to change | `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts`, `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx`, `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx` |
| Files to read before changing anything | All files from M2 and M3 |
| New files allowed | `crates/sldo-tauri/Info.plist`, `tests/e2e_voice_tx_m4.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing functionality unchanged |
| Forbidden shortcuts | No silent error swallowing, no mocks in production, no TODOs |

#### Out of Scope / Must Not Do

- Do not add live partial transcription (streaming while recording)
- Do not add settings UI for STT model/provider
- Do not add OS keychain storage for API key
- Do not modify existing `transcribe_audio` or `VoiceButton`

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/voice-tx-m3.md` and apply relevant corrections.
3. Read the allowed files before editing.
4. Copy the Evidence Log template.
5. Re-state constraints.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-tauri/ui/src/hooks/useStandaloneVoice.ts` | Add blobToBase64 using FileReader.readAsDataURL, improve error messages |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` | Enhanced error display, empty-recording guard, status refinements |
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.test.tsx` | Add edge case tests |
| `crates/sldo-tauri/Info.plist` | NEW: macOS microphone permission entry |
| `tests/e2e_voice_tx_m4.rs` | NEW: E2E integration tests |

#### Step-by-Step

1. Write BDD test stubs for edge case scenarios.
2. Write E2E stub `tests/e2e_voice_tx_m4.rs`.
3. Create `crates/sldo-tauri/Info.plist` with `NSMicrophoneUsageDescription`.
4. Update `useStandaloneVoice.ts`:
   - Add `blobToBase64` helper using `FileReader.readAsDataURL` pattern (split on comma to extract base64).
   - Guard against empty recordings (`audioBlob.size === 0` → error).
   - Improve error messages to be user-friendly.
5. Update `VoiceTranscriber.tsx`:
   - Display different error styles for different failure types if applicable.
   - Ensure buttons are disabled during transcription (prevent double-invocation).
6. Make all tests pass.
7. Run the full test suite.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Integration edge cases and macOS permission**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Empty recording caught before backend call | invalid input | Recording is 0 bytes | Stop pressed | Error "No audio was captured" displayed |
| API key missing shows clear error | dependency failure | OPENAI_API_KEY not set | Transcription attempted | Error mentions "API key" |
| OpenAI error body surfaced to user | partial failure | OpenAI returns 401 | Response received | Error shows status code and body |
| Network error shows clear message | dependency failure | No internet / DNS fails | Transcription attempted | Error mentions "Network error" |
| Info.plist has microphone description | happy path | File exists | Parsed as plist | Contains NSMicrophoneUsageDescription string |
| Buttons disabled during transcription | concurrency/race | Transcription in progress | User clicks start | Start button is disabled |
| Short recording (< 1 second) still works | happy path | User records briefly | Stop pressed | Audio sent to backend (even if short) |
| blobToBase64 extracts correct base64 | happy path | Blob with audio data | blobToBase64 called | Returns pure base64 string (no data URI prefix) |

#### Regression Tests

- All existing frontend tests pass
- All existing backend tests pass
- M1, M2, M3 tests all pass
- Existing voice feature tests pass

#### Compatibility Checklist

- [ ] Existing `transcribe_audio` command works
- [ ] Existing `VoiceButton` works
- [ ] All existing tests pass
- [ ] `AppSettings` shape unchanged
- [ ] All existing Tauri commands still registered

#### E2E Runtime Validation

**File**: `tests/e2e_voice_tx_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `info_plist_exists_and_valid` | macOS permission entry present | File exists with NSMicrophoneUsageDescription |
| `standalone_command_rejects_empty_audio` | Empty audio caught at Rust layer | Error returned before any network call |
| `standalone_command_rejects_missing_key` | Missing API key caught cleanly | Error mentions "API key" |
| `standalone_command_rejects_invalid_base64` | Malformed input caught | Error mentions "decode" or "base64" |

#### Smoke Tests

- [ ] `cargo test --workspace` passes
- [ ] `cd crates/sldo-tauri/ui && npx vitest run` passes
- [ ] `crates/sldo-tauri/Info.plist` exists and contains `NSMicrophoneUsageDescription`
- [ ] App launches with `cargo tauri dev`
- [ ] Record 3–5 seconds of speech, stop → transcript appears
- [ ] With no API key set → error message mentions API key
- [ ] Very short recording (< 1 second) → either transcribes or shows clear error

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Baseline frontend | `cd crates/sldo-tauri/ui && npx vitest run` | all green | | | |
| BDD tests created | Edge case tests | fail for expected reason | | | |
| Info.plist created | File with NSMicrophoneUsageDescription | file exists | | | |
| Implementation | Error handling, blobToBase64, guards | contract satisfied | | | |
| Full tests | Both test suites | green | | | |
| E2E runtime | `cargo test --workspace --test 'e2e_*'` | green | | | |
| Smoke test manual | Record + transcribe with real API key | transcript appears | | | |
| Compatibility checks | Existing voice feature | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- `blobToBase64` uses `FileReader.readAsDataURL` pattern
- Empty recording guard prevents backend call with user-friendly error
- All error paths produce clear, actionable error messages
- `Info.plist` exists with `NSMicrophoneUsageDescription`
- Buttons properly disabled during transcription (no double-invoke)
- All new tests pass
- All existing tests pass
- lessons file written
- completion summary written
- Milestone Tracker updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Document Info.plist for macOS mic permission
- **README.md**: Add note about macOS microphone permission for bundled app

#### Notes

- Persistence category does not apply.
- Retry/rollback: not applicable — one-shot operation, user can try again manually.
- The manual smoke test with a real API key is critical for this milestone. Automated tests validate error handling paths.

---

### Milestone 5 — Polish, Production Shape Guidance & Documentation

**Goal**: Final polish of the standalone voice transcriber: consistent styling consistent with the existing app, production-security guidance in README, keyboard shortcuts, comprehensive documentation updates, and final integration-test sweep.

**Context**: Milestones 1–4 delivered the working feature. This milestone ensures it is polished, documented, and production-ready from a guidance perspective. The inline styling from the user's specification should match or complement the existing `App.css` styles. Add guidance about not shipping shared API keys in distributed apps.

**Important design rule**: No new features. Only polish, documentation, and tests.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — CSS/style adjustments in the VoiceTranscriber component.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | None — polish and docs only |
| Outputs | Updated ARCHITECTURE.md, README.md, polished UI, final test sweep |
| Interfaces touched | Documentation files, VoiceTranscriber styling |
| Files allowed to change | `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx`, `crates/sldo-tauri/ui/src/App.css`, `docs/ARCHITECTURE.md`, `README.md` |
| Files to read before changing anything | All milestone lesson files, ARCHITECTURE.md, README.md, App.css |
| New files allowed | `tests/e2e_voice_tx_m5.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing functionality unchanged |
| Forbidden shortcuts | No new features dressed as polish, no TODOs |

#### Out of Scope / Must Not Do

- Do not add live partial transcription
- Do not add settings for STT model/provider
- Do not add OS keychain integration
- Do not modify existing voice feature
- Do not add new Tauri commands
- Do not add new React routes/phases

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read all previous lessons files (`voice-tx-m1.md` through `voice-tx-m4.md`).
3. Read the allowed files before editing.
4. Copy the Evidence Log template.
5. Re-state constraints.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-tauri/ui/src/components/VoiceTranscriber.tsx` | Styling polish, consistent with App.css patterns |
| `crates/sldo-tauri/ui/src/App.css` | Add voiceTranscriber CSS classes if needed |
| `docs/ARCHITECTURE.md` | Add Voice Transcriber architecture section |
| `README.md` | Add Voice Transcriber usage section, production security guidance |
| `tests/e2e_voice_tx_m5.rs` | NEW: final integration sweep E2E |

#### Step-by-Step

1. Write E2E stub `tests/e2e_voice_tx_m5.rs`.
2. Polish `VoiceTranscriber.tsx` styling:
   - Use font-family matching existing app
   - Error box styling matching existing patterns
   - Button styling consistent with existing buttons
   - MaxWidth, margin, padding matching existing pages
3. Add CSS classes to `App.css` for the transcriber page (`.voiceTranscriber`, `.voiceTranscriber-error`, etc.).
4. Update `docs/ARCHITECTURE.md`:
   - Add "Voice Transcriber" section documenting the standalone page
   - Document the `transcribe_audio_standalone` command
   - Document the `useStandaloneVoice` hook
   - Document the `VoiceTranscriber` component
5. Update `README.md`:
   - Add "Voice Transcriber" section under Desktop App
   - Document how to use the standalone page
   - Add production security guidance about not shipping shared API keys
   - Document common problems and troubleshooting
6. Run the full test suite.
7. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Polish and documentation**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| ARCHITECTURE.md documents voice transcriber | happy path | Doc file read | Search for "Voice Transcriber" | Section exists with command/hook/component docs |
| README.md documents standalone page | happy path | Doc file read | Search for "Voice Transcriber" | Usage section exists |
| README.md has production security guidance | happy path | Doc file read | Search for "API key" / "production" | Guidance about not shipping shared keys |
| All previous tests still pass | backward compat | Full test suite | Run all tests | 90+ frontend + 200+ backend green |
| VoiceTranscriber has consistent styling | happy path | Component rendered | Visual inspection | Matches app's design language |

#### Regression Tests

- All existing tests pass
- All M1–M4 tests pass
- No visual regressions in existing pages

#### Compatibility Checklist

- [ ] All existing tests pass
- [ ] All existing pages render correctly
- [ ] All existing commands work
- [ ] Settings are unchanged
- [ ] Sidebar buttons all work

#### E2E Runtime Validation

**File**: `tests/e2e_voice_tx_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `all_workspace_tests_pass` | Final integration sweep | `cargo test --workspace` green |
| `architecture_doc_mentions_voice_transcriber` | Documentation updated | File contains "Voice Transcriber" or "transcribe_audio_standalone" |
| `readme_mentions_voice_transcriber` | User-facing docs updated | File contains voice transcriber usage info |

#### Smoke Tests

- [ ] `cargo test --workspace` passes
- [ ] `cd crates/sldo-tauri/ui && npx vitest run` passes
- [ ] App launches with `cargo tauri dev`
- [ ] Transcriber page looks polished and consistent with rest of app
- [ ] Full recording→transcription flow works (with API key)
- [ ] README has clear setup instructions for voice feature
- [ ] ARCHITECTURE.md is up to date

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | Both test suites | all green | | | |
| Styling polish | VoiceTranscriber.tsx | consistent with app | | | |
| ARCHITECTURE.md | Updated with voice transcriber section | section exists | | | |
| README.md | Updated with usage and security guidance | section exists | | | |
| Full tests | Both test suites | green | | | |
| E2E runtime | `cargo test --workspace --test 'e2e_*'` | green | | | |
| Smoke tests | Full app walkthrough | all checked | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- VoiceTranscriber styling is polished and consistent with existing app
- ARCHITECTURE.md has a Voice Transcriber section
- README.md has Voice Transcriber usage docs and production security guidance
- All tests pass (full suite)
- No forbidden shortcuts remain
- Lessons file written
- Completion summary written
- Milestone Tracker updated
- This runbook's Milestone Tracker shows all 5 milestones as `done`

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Final review of Voice Transcriber section
- **README.md**: Final review of Voice Transcriber usage and troubleshooting

#### Notes

- This is the final milestone. No future milestones are planned.
- Future enhancements (live partial transcription, STT model settings, OS keychain) are documented as deferred follow-ups in the completion summary.

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | ARCHITECTURE.md Update | README.md Update | Other Docs |
|---|---|---|---|
| 1 | Voice Transcriber page stub mention | None | None |
| 2 | `transcribe_audio_standalone` command docs | None | None |
| 3 | `useStandaloneVoice` hook + `VoiceTranscriber` wiring | None | None |
| 4 | Info.plist / macOS mic permission | macOS mic permission note | None |
| 5 | Full Voice Transcriber architecture section | Full voice transcriber usage, production security, troubleshooting | None |

---

## Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.
