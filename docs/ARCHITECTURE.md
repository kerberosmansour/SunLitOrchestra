# Architecture — SunLitOrchestrate

> This document describes the architecture of SunLitOrchestrate, including both the CLI tools and the Tauri desktop application.

---

## Overview

SunLitOrchestrate is an AI-driven software development toolkit with two interfaces:

1. **CLI tools** — `sldo-plan` (runbook generation) and `sldo-run` (milestone execution)
2. **Tauri desktop app** — A graphical interface wrapping the same backend logic

Both interfaces share `sldo-common`, a library of reusable modules.

## Workspace Structure

```
SunLitOrchestrate/
├── crates/
│   ├── sldo-common/          # Shared library (copilot, runbook, preflight, etc.)
│   ├── sldo-plan/            # CLI: runbook generation
│   ├── sldo-run/             # CLI: milestone execution
│   └── sldo-tauri/           # Tauri v2 desktop app
│       ├── src/
│       │   ├── main.rs       # Tauri entry point, command registration
│       │   ├── commands/     # Tauri command handlers
│       │   │   └── plan.rs   # Planning commands (wraps sldo-common)
│       │   ├── state.rs      # Managed app state (AppState, AppSettings)
│       │   └── events.rs     # Event payload types for streaming
│       └── ui/               # React + TypeScript frontend
│           └── src/
│               ├── hooks/    # useStreamingEvents, usePlan
│               ├── components/
│               └── types/
├── tests/                    # Workspace-level E2E tests
└── docs/
```

## Shared Library: sldo-common

| Module | Purpose |
|---|---|
| `copilot.rs` | `CopilotInvocation` — builds and runs Copilot CLI commands |
| `runbook.rs` | Markdown milestone tracker parsing |
| `toolflags.rs` | Allow/deny permission flags for Copilot invocations |
| `preflight.rs` | Pre-flight validation (copilot installed, git safety) |
| `git.rs` | Repository and branch detection |
| `detect.rs` | Build/test command auto-detection |
| `logging.rs` | Timestamped log file writing |
| `color.rs` | Colored terminal output |

## Tauri Desktop App

### Command Registration

Tauri commands are registered in `main.rs` via `invoke_handler`:

```rust
tauri::Builder::default()
    .setup(|app| {
        // Load persisted settings on startup
        let settings = state::load_settings(&app.path().app_data_dir()?);
        *app.state::<AppState>().settings.lock().unwrap() = settings;
        Ok(())
    })
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
        commands::plan::start_planning,
        commands::plan::read_runbook,
        commands::plan::save_runbook,
        commands::run::start_execution,
        commands::run::cancel_execution,
        commands::settings::get_settings,
        commands::settings::update_settings,
        commands::settings::get_available_providers,
        commands::settings::get_available_models,
    ])
    .run(tauri::generate_context!())
```

### Event Streaming Pattern (M3)

The Tauri backend communicates with the React frontend via **events**. This pattern enables real-time streaming of Copilot output without blocking the UI.

**Flow:**

1. Frontend calls `invoke("start_planning", { prompt, repoDir })` — returns immediately
2. Backend spawns an async task via `tokio::spawn`
3. Task runs `CopilotInvocation::run_with_callback()`, which calls a closure for each output line
4. Closure calls `app.emit("plan-progress", PlanProgressEvent { ... })` for each line
5. On completion: emits `plan-complete` with runbook path and validation issues
6. On error: emits `plan-error` with error description

**Event Types** (defined in `events.rs`):

| Event | Payload | When |
|---|---|---|
| `plan-progress` | `PlanProgressEvent { line, stream, timestamp }` | Each line of Copilot stdout/stderr |
| `plan-complete` | `PlanCompleteEvent { runbook_path, validation_issues }` | Planning finishes |
| `plan-error` | `PlanErrorEvent { error }` | Planning fails |

**Frontend Hooks:**

- `useStreamingEvents<T>(eventName)` — Generic hook wrapping Tauri `listen()`, accumulates event payloads into React state
- `usePlan()` — Orchestrates `start_planning` invocation and listens to all three event types

### `run_with_callback` Extension (M3)

The `CopilotInvocation` struct in `sldo-common/copilot.rs` was extended with a callback-based variant:

```rust
pub fn run_with_callback<F>(&self, log_file: &LogFile, mut on_line: F) -> Result<i32>
where
    F: FnMut(&str, &str),  // (line_content, stream_name)
```

- **`on_line`** receives each output line and the stream name (`"stdout"` or `"stderr"`)
- The original `run()` method now delegates to `run_with_callback` with a print closure, preserving backward compatibility
- The Tauri backend uses `run_with_callback` to emit events instead of printing

### Managed State

`AppState` (in `state.rs`) holds:

- `current_session: Mutex<Option<PlanningSession>>` — tracks active planning session
- `settings: Mutex<AppSettings>` — provider, model, allow/deny flags, execution params, repo directory
- `cancel_execution: Arc<AtomicBool>` — cancellation flag for execution loop
- `execution_running: Arc<AtomicBool>` — whether an execution is currently running

### ConversationView Streaming

`ConversationView` accepts an optional `streamingLines` prop:

```tsx
interface ConversationViewProps {
  messages: Message[];
  onSubmit: (text: string) => void;
  streamingLines?: PlanProgressEvent[];
}
```

When `streamingLines` is non-empty, a streaming output block renders below the conversation messages, showing each line with its stream class (`streamLine--stdout` or `streamLine--stderr`).

### Runbook Persistence & Editor (M4)

After planning completes, the runbook is loaded into a Markdown editor for review and editing.

**Backend Commands:**

| Command | Input | Output | Purpose |
|---|---|---|---|
| `read_runbook` | `path: String` | `RunbookData { content, milestones, path }` | Read file, parse tracker, return content + milestones |
| `save_runbook` | `path: String, content: String` | `Vec<String>` (warnings) | Write to disk, re-validate, return any issues |

**Data Flow:**

1. `plan-complete` event fires with `runbook_path`
2. Frontend calls `read_runbook(path)` → gets content + parsed milestones
3. MarkdownEditor displays content in edit/preview toggle
4. MilestoneTracker renders milestone rows with color-coded status
5. User edits and saves → `save_runbook(path, content)` validates and writes
6. Warnings (missing sections, empty tracker) displayed below editor

**Frontend Components (M4):**

- `MarkdownEditor` — Toggle between raw textarea (edit) and rendered preview. Supports Ctrl+S, auto-save on blur, validation warnings display
- `MilestoneTracker` — Renders milestone rows with status icons (✅ done, 🔄 in_progress, ⬜ not_started) and progress bar

**App Phase: "reviewing":**

After planning, the app transitions to the `"reviewing"` phase which shows:
- MarkdownEditor (main area) — editable runbook content
- MilestoneTracker (sidebar) — milestone progress overview
- "Execute Plan" button — transitions to `"executing"` phase (wired in M5)

### Execution Backend & Live Progress (M5)

The execution backend drives Copilot through runbook milestones, streaming live progress to the frontend.

**Backend Commands:**

| Command | Input | Output | Purpose |
|---|---|---|---|
| `start_execution` | `runbook_path: String, repo_dir: String` | `String` (status message) | Starts async execution loop on `tokio::spawn` |
| `cancel_execution` | _(none)_ | `String` (status message) | Sets `AtomicBool` cancellation flag to stop the loop |

**Execution Flow:**

1. Frontend calls `start_execution(runbookPath, repoDir)`
2. Backend validates inputs, parses tracker, detects build/test commands
3. If all milestones done, emits `execution-complete` immediately
4. Otherwise, spawns async loop via `tokio::spawn`:
   - Reads runbook, finds next incomplete milestone
   - Emits `milestone-started` event
   - Builds execution prompt (matching `sldo-run` pattern)
   - Invokes Copilot via `run_with_callback`, emitting `execution-progress` for each line
   - Runs build/test verification commands, emitting `build-test-result` for each
   - Emits `milestone-completed`
   - Checks cancellation flag before next iteration
   - Sleeps cooldown between attempts
5. On completion (or cancellation), emits `execution-complete` with summary

**Cancellation Pattern:**

- `AppState` contains `cancel_execution: Arc<AtomicBool>` and `execution_running: Arc<AtomicBool>`
- `cancel_execution` command sets the flag to `true`
- The execution loop checks `cancel_flag.load(Ordering::Relaxed)` between iterations
- On cancel, the loop breaks and emits `execution-complete`

**Execution Event Types** (defined in `events.rs`):

| Event | Payload | When |
|---|---|---|
| `milestone-started` | `MilestoneStartedEvent { milestone_number, title, attempt }` | Each milestone attempt begins |
| `execution-progress` | `ExecutionProgressEvent { line, stream, timestamp }` | Each line of Copilot output |
| `build-test-result` | `BuildTestResultEvent { command, success, output }` | After each build/test command |
| `milestone-completed` | `MilestoneCompletedEvent { milestone_number, success }` | Milestone attempt finishes |
| `execution-complete` | `ExecutionCompleteEvent { all_done, milestones_completed, total }` | Entire execution run ends |

**Frontend Components (M5):**

- `ExecutionView` — Main execution display with log panel, build/test results, cancel button, and sidebar milestone tracker
- `useExecution` hook — Wraps `start_execution`/`cancel_execution` commands and listens to all execution events
- `MilestoneTracker` — Updated with optional `activeMilestone` prop to highlight the currently executing milestone

**App Phase: "executing":**

After clicking "Execute Plan", the app transitions to the `"executing"` phase which shows:
- Log panel (main area) — streaming agent output with stdout/stderr distinction
- Build/test results — pass/fail indicators for verification commands
- Cancel button — stops execution mid-run
- MilestoneTracker (sidebar) — live progress with active milestone highlight

### Provider Trait & Settings (M6)

The `Provider` trait (in `provider.rs`) abstracts agent invocation so the system can support multiple coding agents:

```rust
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn available_models(&self) -> Vec<String>;
    fn invoke(&self, prompt: &str, model: &str, allow_flags: &[String],
              deny_flags: &[String], working_dir: &Path, log_file: &LogFile,
              on_line: Box<dyn FnMut(&str, &str) + Send>) -> Result<i32>;
}
```

- `CopilotProvider` wraps `CopilotInvocation::run_with_callback()`
- `get_provider(name)` factory function returns a boxed provider by name
- `available_providers()` lists all registered provider names

**Settings Persistence:**

`AppSettings` (in `state.rs`) is serialized as JSON to `<tauri_app_data_dir>/settings.json`:

| Field | Type | Default | Purpose |
|---|---|---|---|
| `provider` | `String` | `"copilot"` | Active agent provider |
| `model` | `String` | `"claude-opus-4.6"` | Model for planning/execution |
| `allow_flags` | `Vec<String>` | `toolflags::plan_allow_flags()` | Tool permission allow flags |
| `deny_flags` | `Vec<String>` | `toolflags::plan_deny_flags()` | Tool permission deny flags |
| `max_attempts` | `u32` | `150` | Max execution attempts |
| `cooldown_secs` | `u64` | `5` | Cooldown between attempts |
| `max_iterations` | `u32` | `3` | Max planning iterations |
| `repo_dir` | `Option<String>` | `None` | Repository directory |

- Settings are loaded on app startup via `setup()` hook
- Invalid/missing settings.json falls back to defaults with a warning
- Planning and execution commands read from managed `AppSettings` instead of hardcoded values

**Settings Commands:**

| Command | Input | Output | Purpose |
|---|---|---|---|
| `get_settings` | _(none)_ | `AppSettings` | Read current in-memory settings |
| `update_settings` | `AppSettings` | `()` | Persist to disk and update state |
| `get_available_providers` | _(none)_ | `Vec<String>` | List provider names |
| `get_available_models` | `provider_name: String` | `Vec<String>` | List models for a provider |

**Frontend Components (M6):**

- `SettingsPanel` — Form with provider selector, model input, tool flags editor, execution params, save button
- `AppSettings` TypeScript type mirrors the Rust struct

**App Phase: "settings":**

Clicking "Settings" in the sidebar transitions to the `"settings"` phase, rendering the `SettingsPanel` component with current settings values.

### Voice Input & Speech-to-Text (M7)

**Architecture:**

The voice feature uses a three-layer design:

1. **Frontend** — `VoiceButton` component + `useVoice` hook manage `MediaRecorder` API lifecycle
2. **Tauri Command** — `transcribe_audio` receives base64-encoded audio, proxies to STT provider
3. **STT Provider** — Initially OpenAI Whisper API; the backend handles the API key securely

**Security:** The `OPENAI_API_KEY` is never sent to the frontend. The Tauri backend reads it from the environment (via `dotenvy` loading `.env` if present) and proxies the STT request.

**Voice Command:**

| Command | Input | Output | Purpose |
|---|---|---|---|
| `transcribe_audio` | `audio_base64: String` | `String` | Send audio to STT, return transcribed text |

**Frontend Components (M7):**

- `VoiceButton` — Three-state button: idle (🎙), recording (⏹ pulsing), transcribing (⏳ spinner)
- `useVoice` hook — Manages `MediaRecorder`, converts audio to base64, invokes Tauri command
- `ChatInput` — Integrates `VoiceButton` next to the Send button; transcription populates textarea

**Data Flow:**

```
User clicks 🎙 → MediaRecorder.start() → User clicks ⏹ → MediaRecorder.stop()
→ Blob → base64 → invoke("transcribe_audio") → Rust backend
→ dotenvy loads .env → reqwest POST to OpenAI Whisper API
→ transcribed text → frontend → textarea populated
```

**Error Handling:**

- Missing API key → descriptive error mentioning "API key"
- Invalid base64 → error mentioning "base64"
- Network failure → error mentioning "connection"
- Empty transcription → error mentioning "empty"

### Polish, Error Boundaries & Keyboard Shortcuts (M8)

**Error Boundary:**

The app uses a React `ErrorBoundary` class component (in `components/ErrorBoundary.tsx`) that wraps the entire application in `main.tsx`. If any child component throws during rendering:

1. `getDerivedStateFromError` captures the error and switches to fallback UI
2. Fallback shows "Something went wrong" with the error message and a "Try Again" button
3. "Try Again" resets the boundary state, re-rendering children

### Voice Transcriber — Standalone Page (voice-tx M1)

The app includes a dedicated standalone voice transcription page, separate from the chat-embedded `VoiceButton`. This page is accessible via the "Transcriber" button in the sidebar.

**Routing:**

- `AppPhase` type includes `"transcriber"` as a valid phase
- `App.tsx` renders `VoiceTranscriber` when the phase is `"transcriber"`
- `Sidebar.tsx` has a "Transcriber" button that sets the app phase

**Component: `VoiceTranscriber.tsx`**

A placeholder page with:
- Heading: "Tauri Voice Transcriber"
- Description text
- Start/stop recording buttons (disabled until M3 wires recording hooks)
- Transcript textarea (read-only, empty)
- Error display area

**Design rule:** `VoiceTranscriber` is fully separate from `VoiceButton` — it does not reuse `useVoice`. It will use its own `useStandaloneVoice` hook (added in M3).

**Keyboard Shortcuts:**

Global keyboard shortcuts are registered via a `useEffect` in `App.tsx`:

| Shortcut | Action |
|---|---|
| `Cmd/Ctrl+Enter` | Submit prompt (handled in `ChatInput.tsx` via `onKeyDown`) |
| `Cmd/Ctrl+N` | New session — resets all state to home screen |
| `Cmd/Ctrl+,` | Open settings panel |
| `Escape` | Close settings panel |

**Concurrency Safety:**

- `AppState.execution_running: Arc<AtomicBool>` prevents concurrent execution runs
- `PlanningSession.in_progress` prevents concurrent planning sessions
- Both use `compare_exchange` with `SeqCst` ordering for thread-safe checks

## Test Architecture

### Backend Tests

| Suite | Location | Tests | Purpose |
|---|---|---|---|
| sldo-common unit | `crates/sldo-common/src/*.rs` | 48 | Core library validation |
| sldo-plan unit | `crates/sldo-plan/src/main.rs` | 21 | Planning CLI tests |
| sldo-run unit | `crates/sldo-run/src/main.rs` | 13 | Execution CLI tests |
| sldo-tauri unit | `crates/sldo-tauri/src/**/*.rs` | 56 | Tauri backend tests |
| E2E scaffold | `tests/e2e_scaffold_m1.rs` | 4 | Framework validation |
| E2E common | `tests/e2e_common_m2.rs` | 7 | Shared library E2E |
| E2E plan | `tests/e2e_plan_m3.rs` | 4 | Planning CLI E2E |
| E2E run | `tests/e2e_run_m4.rs` | 4 | Execution CLI E2E |
| E2E integration | `tests/e2e_integration_m5.rs` | 4 | Cross-crate integration |
| E2E tauri M1 | `tests/e2e_tauri_m1.rs` | 7 | Tauri scaffold E2E |
| E2E tauri M3 | `tests/e2e_tauri_m3.rs` | 5 | Planning backend E2E |
| E2E tauri M4 | `tests/e2e_tauri_m4.rs` | 3 | Editor backend E2E |
| E2E tauri M5 | `tests/e2e_tauri_m5.rs` | 10 | Execution backend E2E |
| E2E tauri M6 | `tests/e2e_tauri_m6.rs` | 6 | Settings/provider E2E |
| E2E tauri M7 | `tests/e2e_tauri_m7.rs` | 2 | Voice backend E2E |
| E2E tauri M8 | `tests/e2e_tauri_m8.rs` | 6 | Integration & polish E2E |
| E2E voice-tx M1 | `tests/e2e_voice_tx_m1.rs` | 2 | Voice transcriber route E2E |

**Total backend tests: 202**

### Frontend Tests

| Suite | Location | Tests | Purpose |
|---|---|---|---|
| BDD component tests | `ui/src/components/*.test.tsx` | 55+ | Component behavior validation |
| E2E chatui | `ui/src/e2e/chatui.e2e.test.tsx` | 4 | Chat UI runtime validation |
| E2E planning | `ui/src/e2e/planning.e2e.test.tsx` | 2 | Planning flow validation |
| E2E editor | `ui/src/e2e/editor.e2e.test.tsx` | 3+ | Editor runtime validation |
| E2E execution | `ui/src/e2e/execution.e2e.test.tsx` | 3 | Execution flow validation |
| E2E settings | `ui/src/e2e/settings.e2e.test.tsx` | 3 | Settings panel validation |
| E2E voice | `ui/src/e2e/voice.e2e.test.tsx` | 3 | Voice input validation |
| E2E integration | `ui/src/e2e/integration.e2e.test.tsx` | 6 | Full workflow integration |

**Total frontend tests: 98**
