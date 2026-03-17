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
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![commands::plan::start_planning])
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
- `settings: Mutex<AppSettings>` — model, max iterations, repo directory

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
