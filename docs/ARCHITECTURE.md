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
