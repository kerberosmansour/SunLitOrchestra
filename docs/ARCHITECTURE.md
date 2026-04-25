# Architecture — SunLitOrchestrate

> This document describes the architecture of SunLitOrchestrate, including both the CLI tools and the Tauri desktop application.

---

## Overview

SunLitOrchestrate is an AI-driven software development toolkit with two interfaces:

1. **CLI tools** — `sldo-plan` (runbook generation) and `sldo-run` (milestone execution)
2. **Tauri desktop app** — A graphical interface wrapping the same backend logic

Both interfaces share `sldo-common`, a library of reusable modules.

A third, first-class surface ships alongside the CLIs and desktop app: the `/slo-*` **skill pack** (see [Skill Pack](#skill-pack) below). Skills are Markdown `SKILL.md` files consumed by Claude Code; they are not compiled Rust. The CLI tools (`sldo-plan`, `sldo-run`) were the original interface; the skill pack is the active direction (the `sldo-tauri` desktop app is parked).

## Skill Pack

The skill pack lives at `skills/slo-*/` and is installed to `~/.claude/skills/` by `sldo-install`. Each skill is a directory with a single `SKILL.md` plus optional support files (`personas/`, `examples/`, `references/`). Skills are invoked by the user as `/<skill-name>` in Claude Code; the loader reads `SKILL.md` frontmatter (`name:`, `description:`) and exposes the skill.

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `slo-ideate` | YC-style interrogation before code |
| Research | `slo-research` | Wraps the `sldo-research` Rust binary for sourced dossiers |
| Architect | `slo-architect` | Emits `ARCHITECTURE.md` / `docs/design/*.md`, decides `tla_required` |
| Verify design | `slo-tla` | TLC model-check when the architect flags concurrency risk |
| Plan | `slo-plan` | Interactive v3 runbook authoring, one milestone at a time |
| Critique | `slo-critique` | Four-persona adversarial review (CEO, eng, security, design) |
| Execute | `slo-execute` | Per-milestone driver with allow-list enforcement |
| Verify | `slo-verify` | Runtime QA in four passes (happy / degraded / partial failure / **security — supply-chain + variant-analysis + conditional DAST**); Playwright for UI surfaces |
| Close | `slo-retro` | Lessons + completion + tracker update |
| Ship | `slo-ship` | Open PR with runbook-aware description |
| Power tools | `slo-freeze`, `slo-resume`, `slo-second-opinion`, `get-api-docs` | Auxiliary disciplines |
| SAST rule-gen (M1 of sast pack Runbook A — DESIGN, not yet implemented) | `slo-rulegen` | Bootstrap mode emits a Semgrep rule pack covering the top-10 Rust CWE classes from a 2-hop RustSec → GHSA → OSV `cwe_ids` join (CWE-755 panic-DoS, CWE-416 UAF, CWE-697 incorrect-comparison, CWE-125 OOB-read, CWE-787 OOB-write, CWE-190 integer-overflow, CWE-295 cert-validation, CWE-672 expired-resource, CWE-20 input-validation, CWE-79 webapp-XSS) into `.semgrep/rust/`. Extend mode (M2) takes `(bug_summary, fix_diff, file_paths)` from a Claude-found bug and outputs 3-5 variation rules with auto-derived corpus, appending to the existing pack. `rulegen_*` toolflags DENY WebFetch / WebSearch — CWE map is pre-baked in `references/sast/`. See [docs/design/sast-rulegen-skill-pack-overview.md](design/sast-rulegen-skill-pack-overview.md). |
| SAST rule-gen verify (M1 of sast pack Runbook A — DESIGN, not yet implemented) | `slo-ruleverify` | Read-only skill that invokes `cargo xtask sast-verify gate <rule-path>`; composes `validate` (Semgrep YAML/syntax via exit codes 5/7/4), `test` (fire-on-bad / silent-on-good per `// ruleid:` / `// ok:` annotations — Semgrep upstream convention), `check-coverage` (≥ N `pattern-either` arms per CWE template), `check-clean` (zero false positives on the host crate's `src/` after fix is applied). `ruleverify_*` toolflags DENY Write/Edit/WebFetch/WebSearch. |

### Skill pack invariants (reality at HEAD)

- **Markdown-only skill contract.** No compiled code in `skills/slo-*/`. Shell-outs are the extension mechanism — skills invoke `sldo-research`, `gh`, `cargo test`, or any CLI the host has available.
- **Canonical planning artifact.** Every feature runbook is `docs/RUNBOOK-<feature>.md` and follows `docs/runbook-template_v_3_template.md`. The template is the output contract of `/slo-plan`.
- **Reality-first ARCHITECTURE.md.** This file describes implemented code at HEAD. Planned work for a feature lives in `docs/design/<slug>-overview.md` and in that feature's runbook Target Architecture section.
- **Baseline test command.** `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (not `--workspace`; the parked `sldo-tauri` crate breaks that on macOS arm64).
- **Shared scaffolding for the SAST rule-gen pack lives at `references/sast/`** (planned, lands in M1 of Runbook A). Same installer-bypass pattern as `references/biz/` — sibling of `skills/`, never walked by `discover_skills()`. Holds `cwe-map-rust.md` (top-10 Rust CWEs from a 2-hop RustSec → GHSA → OSV join), `variations/cwe-<NNN>.md` (one per CWE, declares minimum-N for `pattern-either` enumeration coverage), `semgrep-rust-syntax.md` (Semgrep primitives confirmed working for Rust in 2026 + smoke-test results for `pattern-inside: unsafe { ... }`), `manifest-schema.md` (rule YAML metadata schema including `cwe`, `sldo-rulegen-version`, `sldo-variation-template` fields), `MIN-SEMGREP-VERSION.md` (pinned minimum semgrep CLI), `AUTHORING.md` (Trail of Bits AGPL clean-room re-authoring policy), and `prompts/{bootstrap,extend}.md` (the prompt bodies the skills consume). See [docs/design/sast-rulegen-skill-pack-overview.md](design/sast-rulegen-skill-pack-overview.md) for the full design.
- **The SAST verifier xtask lives at `xtasks/sast-verify/`** (planned, lands in M1 of Runbook A — DESIGN, not yet implemented). New Cargo workspace member registered in repo-root `Cargo.toml`. Cargo alias `xtask = "run --package sast-verify --"` declared in a new repo-root `.cargo/config.toml` (matklad/cargo-xtask convention; rust-analyzer / Tokio / wasmtime / OpenVMM precedent). Single binary with subcommands `validate` / `test` / `check-coverage` / `check-clean` / `gate` — `gate` composes the four into a single deterministic exit code that `/slo-rulegen` and `/slo-ruleverify` both shell out to. Body wraps `semgrep --validate` and `semgrep --test`; never reaches the network. Adds `-p sast-verify` to [CLAUDE.md](../CLAUDE.md)'s baseline test list (still NOT `--workspace` — `sldo-tauri` remains parked).

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
| `model` | `String` | `"claude-opus-4-7"` | Model for planning/execution |
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
| `transcribe_audio` | `audio_base64: String` | `String` | Send audio to STT via Rig abstraction (chat input) |
| `transcribe_audio_standalone` | `audio_base64: String, mime_type: String` | `String` | Direct reqwest multipart POST to OpenAI with MIME-aware filename (standalone page) |

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

A functional recording page with:
- Heading: "Tauri Voice Transcriber"
- Description text
- Start recording button (enabled when idle, disabled when recording/transcribing)
- Stop recording button (enabled only when recording)
- Status display: "Listening to your microphone…" during recording, "Transcribing with OpenAI…" during transcription
- Transcript textarea (editable, populated after transcription)
- Error display area (red-bordered box on error)

**Hook: `useStandaloneVoice.ts`**

Manages the full MediaRecorder lifecycle for the standalone transcriber:
- `startRecording()`: requests microphone via `getUserMedia`, creates `MediaRecorder` with MIME type preference (`audio/webm;codecs=opus` → `audio/webm` → `audio/mp4` → `audio/ogg;codecs=opus`), collects chunks via `ondataavailable`
- `stopRecording()`: stops recorder, in `onstop` combines chunks, converts to base64 via `FileReader.readAsDataURL`, invokes `transcribe_audio_standalone` Tauri command with actual `mimeType`
- State: `isRecording`, `isTranscribing`, `transcript`, `error`, `setTranscript`
- Cleanup: releases microphone tracks on stop and on unmount
- Empty recording guard: rejects 0-byte recordings with "No audio was captured" before calling the backend

**macOS Microphone Permission:**

The file `crates/sldo-tauri/Info.plist` declares `NSMicrophoneUsageDescription` so that macOS prompts the user for microphone access on first use. This plist entry is required for any macOS app that accesses the microphone via `getUserMedia`.

**Styling:**

`VoiceTranscriber` uses CSS classes defined in `App.css` (prefixed with `voiceTranscriber`) that follow the app's design system — dark background, gold accent tokens, consistent border-radius, and the same font/spacing scale used throughout. Inline styles have been removed in favor of these shared classes.

**Production Security Guidance:**

The `OPENAI_API_KEY` is loaded server-side by the Tauri backend — it is never exposed to the frontend process. For local development, place the key in a `.env` file at the project root. **Do not ship a shared API key in a distributed binary.** In production, each user should provide their own key (via environment variable, `.env` file, or a future settings UI backed by the OS keychain).

**Design rule:** `VoiceTranscriber` is fully separate from `VoiceButton` — it does not reuse `useVoice`. It uses its own `useStandaloneVoice` hook.

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

## sldo-research CLI

A third CLI, `sldo-research`, generates a structured research dossier that can be
fed as the `prompt_file` argument to `sldo-plan`. The full pipeline lands across
milestones; the sections below grow as each milestone ships.

### sldo-research — Research Prompt Builder (M2)

`crates/sldo-research/src/prompt.rs` holds three pure prompt constructors that
return `String` values. They take plain inputs (slices and optional paths) and
perform no I/O, no network, and no environment access — the caller is
responsible for canonicalising paths and reading prompt files.

| Function | Purpose |
|---|---|
| `build_exploration_prompt(prompt_content, repo_dir)` | Asks Claude Code to decompose the topic, identify key concepts, gather initial findings, and optionally observe a repo. |
| `build_deepening_prompt(prompt_content, previous_findings, iteration, repo_dir)` | Re-feeds prior findings (truncated at ~32 KiB) and asks for deepened findings, library evaluations, architecture options, and unanswered questions. Iteration ≥ 3 invites consolidation for the M6 synthesis pass. |
| `build_repo_context_prompt(repo_dir)` | Asks Claude Code to summarise tech stack, project structure, build/test commands, existing patterns, and constraints from a given repository. |

Each prompt embeds fixed `## …` section headers as a contract that later
milestones rely on:

- **Exploration phase:** `## Topic Decomposition`, `## Key Questions`, `## Initial Findings`, and (when a repo is supplied) `## Repo Context`.
- **Deepening phase:** `## Deepened Findings`, `## Library Evaluations`, `## Architecture Options`, `## Unanswered Questions`.
- **Repo-context phase:** `## Tech Stack`, `## Project Structure`, `## Build & Test`, `## Existing Patterns`, `## Constraints`.

At M2 the binary constructs the exploration prompt after pre-flight and prints
its byte length + first line. The Claude Code invocation itself lands in M3.

### sldo-research — Research Loop (M3)

`crates/sldo-research/src/research.rs` drives Claude Code through the
exploration → deepening pipeline. The module is two layers:

- **`ResearchConfig`** — owned input bundle (prompt, optional canonicalised
  repo dir, output path, model, max iterations, cooldown seconds, log
  directory). Constructed once in `main.rs::run()` and passed by reference
  into `research_loop`.
- **`research_loop(&ResearchConfig) -> Result<String>`** — drives three
  phases and returns the accumulated findings string. Per-phase failures are
  logged via `warn(...)` but never abort the loop; the function only fails
  fast on filesystem errors (e.g., the output parent cannot be created).

**Phases:**

| Phase | Log file | Triggered when |
|---|---|---|
| Repo context | `.sldo-logs/research-repo-context.log` | `repo_dir` is `Some` |
| Exploration | `.sldo-logs/research-exploration.log` | always (iteration 1) |
| Deepening   | `.sldo-logs/research-deepen-N.log` (N=2..max) | `max_iterations >= 2` |

A `cooldown_secs` sleep is inserted **before** each deepening iteration
(matching `sldo-plan`'s pacing). A scratch file named
`.research-scratch-iter-N.md` is written next to the dossier output path
after each phase that produced findings — these are the raw per-phase
captures used by M5 (web-search) and M6 (synthesis).

Claude Code is invoked through `sldo_common::copilot::ClaudeInvocation`
with `toolflags::research_allow_flags()` / `research_deny_flags()`. The
working directory passed to Claude is the canonical `repo_dir` if provided,
otherwise the process CWD.

The binary surfaces an `info("Research accumulated N bytes of findings")`
line after the loop — the M3 E2E test asserts on this string to prove the
loop ran end-to-end without invoking the real Claude API (the test suite
prepends a temp-dir containing a stub `claude` shell script to `PATH`).

### Dossier format (M4)

`crates/sldo-research/src/dossier.rs` holds the dossier markdown schema and
the writer/validator pair that sit between the research loop and the rest of
the pipeline. The writer is pure `std + chrono` — no I/O beyond the target
file — and the validator follows `sldo-plan::validate_runbook`'s shape
(returns `Vec<String>` issues, never panics).

**Required sections (order-preserving):**

| Section | Purpose |
|---|---|
| `## Executive Summary` | One-paragraph synthesis (stubbed at M4, filled at M6). |
| `## Topic Decomposition` | The sub-questions from the exploration phase. |
| `## Key Findings` | Raw findings captured from the research loop. |
| `## Library & Tool Evaluations` | Libraries considered and pros/cons. |
| `## Architecture Options` | Candidate architectures with trade-offs. |
| `## API & SDK Documentation` | Notes from web search / vendor docs. |
| `## Design Recommendations` | Ranked recommendations (confidence at M6). |
| `## Risks & Open Questions` | Anything left unanswered. |
| `## References` | URLs and citations. |

The optional `## Repository Context` section is inserted between the
frontmatter and the required sections when `--repo-dir` was supplied and the
repo-context phase produced output.

**Frontmatter fields:** `topic` (single-line excerpt of the user prompt,
≤200 chars), `generated_on` (local timestamp), `source_prompt_bytes`,
`generator: sldo-research`.

**Key constants:**

- `REQUIRED_SECTIONS` — the nine always-present section headers above.
- `PLACEHOLDER_PATTERNS` — `[TBD]`, `[description]`, `[findings]`,
  `[to be filled]`, `TODO:`. These cause `validate_dossier` to report an
  issue.
- `M4_STUB_SENTINEL` — the literal string `"To be synthesised in M6"` that
  the writer emits inside every non-`Key Findings` required section. M4's
  validator tolerates the sentinel; M6's synthesis pass replaces it; M7's
  `check_plan_readiness` asserts its absence.

**`write_dossier`** creates any missing parent directories, writes the
frontmatter + section skeleton, and fails only if the target path cannot
be written. **`validate_dossier`** checks file existence, size ≥ 500 bytes,
every `REQUIRED_SECTIONS` header, and any placeholder pattern. M3's
`research_loop` return type became `ResearchFindings { raw, repo_context }`
in M4 so the writer can emit the repo-context section separately from the
raw findings.

### Synthesis pass (M6)

`build_synthesis_prompt(prompt, all_findings, repo_context)` in
`crates/sldo-research/src/prompt.rs` produces a single Claude Code prompt
that consumes the concatenated raw findings (exploration + web-search +
deepening) and asks for a coherent dossier body conforming exactly to
`dossier::REQUIRED_SECTIONS`. The prompt embeds the section list verbatim
(so Claude cannot rename or omit headers), requires `(confidence: high|
medium|low)` tags on every entry under `## Design Recommendations`, asks
for explicit `## Risks & Open Questions` flagging, and instructs URL
extraction into `## References` as `- [Title](URL)` bullets. Raw findings
are truncated at 100 KiB (tail-preserving) to keep the prompt under
Claude's context window.

`research_loop` runs synthesis as one final invocation after the
deepening loop. The captured stdout becomes
`ResearchFindings.synthesised: Option<String>`. Synthesis output is gated
by `synth_output_well_formed` — every entry in `REQUIRED_SECTIONS` must
appear in the response, otherwise the result is treated as malformed and
`synthesised` stays `None`. This shields the M4 fallback path from
truncated, off-spec, or test-shim responses. Spawn errors, non-zero exit
codes, and empty output also resolve to `None`. The phase log file is
`.sldo-logs/research-synthesis.log`.

`dossier::write_dossier` accepts an additional `synthesised: Option<&str>`
parameter. When `Some(text)` and `text` is non-empty, the synthesised
body is embedded verbatim in place of the M4 stub skeleton. When `None`,
the writer falls back to the M4 layout (raw findings under
`## Key Findings`, `M4_STUB_SENTINEL` everywhere else). The dossier is
always written.

`dossier::check_synthesis_complete(path)` is a stricter post-M6 readiness
helper that returns issues if the dossier still contains the
`M4_STUB_SENTINEL`. Designed to be called by M7's plan-readiness gate;
intentionally separate from `validate_dossier`, which still tolerates the
sentinel for M4 compatibility.

The phase is **prompt-driven** — no new Rust crates were added, no parsing
of section headers happens on the Rust side. Claude Code does the
synthesis; the Rust pipeline structurally validates the result and falls
back when necessary.

### Web search phase (M5)

`build_websearch_prompt(topic, questions, search_index)` in
`crates/sldo-research/src/prompt.rs` produces a Claude Code prompt that
asks the model to use its built-in `WebFetch` and `WebSearch` tools to
find current documentation, library versions, and community articles for
the supplied topic. The prompt requires output under three fixed headers
— `## Web Search Results`, `## Documentation Found`, and
`## Library Versions` — and explicitly asks for URL + title pairs so
M6 can extract them into the dossier's `## References` section.

`research_loop` runs `cfg.max_searches` web-search invocations between
exploration (iteration 1) and deepening (iterations 2..=max). Per-search
log files are named `.sldo-logs/research-websearch-<N>.log`. The phase
partitions the exploration's `## Key Questions` body across invocations
via `extract_key_questions`; when the section is absent the prompt falls
back to broad topic research. `--max-searches 0` skips the phase entirely
(no log files, no invocations). Per-search Claude failures (spawn errors,
non-zero exits) log a warning and the loop continues — they never halt
the pipeline. No `cooldown_secs` sleep is inserted between web-search
invocations: web phases are lighter than deepening passes and don't need
the inter-call pause.

The phase is **prompt-driven**: no new Rust HTTP client or `reqwest`
dependency was added. Tool access is gated by
`toolflags::research_allow_flags()` (which already shipped `WebFetch` and
`WebSearch` at M1). `plan_allow_flags()` does **not** include `WebSearch`
— planning runs offline by design — and a regression test pins this.

### Plan-readiness gate (M7)

`dossier::check_plan_readiness(path) -> Vec<String>` is the strict
end-of-pipeline gate that confirms the dossier is suitable as input to
`sldo-plan`. It composes `validate_dossier` + `check_synthesis_complete`
with three additional constraints:

1. The file is valid UTF-8 (`std::fs::read_to_string` succeeds) — the
   contract `sldo-plan` requires of its `prompt_file`.
2. Total size > `MIN_PLAN_READY_SIZE` (1 KiB) — stricter than the M4
   500-byte threshold.
3. `## Design Recommendations` has > `MIN_SECTION_BODY_BYTES` (100 bytes)
   of content after its header, AND at least one of `## Library & Tool
   Evaluations` / `## Architecture Options` has > 100 bytes of content.

Section-body extraction uses a private `section_body(content, header)`
helper that returns the text between a header and the next top-level
`## ` marker. The helper is line-aware (skips the header's own newline
before slicing) so a header followed immediately by another header
returns an empty body.

`sldo-plan` is **not modified**. The gate lives entirely inside
`sldo-research` and is layered on top of the existing M4/M6 validators —
none of those are weakened.

`sldo-research::main` calls the gate after `validate_dossier` and prints
either:
- a "Research dossier is ready for planning" success block followed by a
  suggested `sldo-plan <dossier> <repo-dir>` command, when the gate
  returns no issues, or
- a warning block listing the issues, with the next-step suggestion
  suppressed.

Either way the binary exits 0 and the dossier is always written —
plan-readiness is observability, not a hard gate. A "Summary" block
(dossier path, byte count, iteration/search counts, total wall time) is
printed unconditionally before the readiness verdict, mirroring
`sldo-plan`'s end-of-run summary style.

### sldo-research pipeline overview

End-to-end, a single `sldo-research` invocation runs:

```
prompt + (optional repo-dir)
   │
   ▼
preflight ──── claude-on-PATH check, optional repo + git safety check
   │
   ▼
[optional] repo-context phase ─── one Claude call per --repo-dir
   │
   ▼
exploration phase ─── one Claude call (always)
   │                   ↓ raw findings appended
   ▼
web-search phase ─── 0..=N Claude calls (N = --max-searches)
   │                   ↓ each appended to raw
   ▼
deepening phase ─── 0..=M Claude calls (M = --max-iterations - 1)
   │                   ↓ each appended to raw, cooldown_secs between calls
   ▼
synthesis phase ─── one Claude call (skipped only when raw is empty)
   │                   ↓ Option<String>; well-formedness gate rejects
   │                     responses missing required dossier headers
   ▼
write_dossier ─── synth body if Some; M4 fallback layout otherwise
   │
   ▼
validate_dossier (M4 ruleset) ─── informational
   │
   ▼
check_plan_readiness (M7 strict ruleset) ─── prints Next-step or warnings
```

Per-phase log files: `.sldo-logs/research-repo-context.log`,
`research-exploration.log`, `research-websearch-<N>.log`,
`research-deepen-<N>.log`, `research-synthesis.log`. Per-phase scratch
files (raw stdout captures): `.research-scratch-iter-<N>.md` next to the
dossier output. The full pipeline is **prompt-driven** — every Claude
invocation goes through `sldo_common::copilot::ClaudeInvocation` with the
`research_allow_flags()` / `research_deny_flags()` toolset.

### Pipeline composition

The three CLIs compose into a single user workflow:

```
$ sldo-research --prompt "add feature flags" --repo-dir ./my-repo
  → output/research-dossier.md  (plan-ready)

$ sldo-plan output/research-dossier.md ./my-repo -o docs/RUNBOOK.md
  → docs/RUNBOOK.md             (milestone tracker)

$ sldo-run docs/RUNBOOK.md ./my-repo
  → drives Claude Code through each milestone
```

`sldo-research` is the only CLI that runs web search; `sldo-plan` and
`sldo-run` operate offline (their `*_allow_flags()` deliberately exclude
`WebFetch`/`WebSearch`).

## Test Architecture

### Backend Tests

| Suite | Location | Tests | Purpose |
|---|---|---|---|
| sldo-common unit | `crates/sldo-common/src/*.rs` | 48 | Core library validation |
| sldo-plan unit | `crates/sldo-plan/src/main.rs` | 21 | Planning CLI tests |
| sldo-run unit | `crates/sldo-run/src/main.rs` | 13 | Execution CLI tests |
| sldo-tauri unit | `crates/sldo-tauri/src/**/*.rs` | 65 | Tauri backend tests |
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
| E2E voice-tx M2 | `tests/e2e_voice_tx_m2.rs` | 5 | Standalone transcription backend E2E |
| E2E voice-tx M4 | `tests/e2e_voice_tx_m4.rs` | 4 | Edge cases & macOS permission E2E |
| E2E voice-tx M5 | `tests/e2e_voice_tx_m5.rs` | 3 | Polish & documentation E2E |
| E2E research M1 | `tests/e2e_research_m1.rs` | 6 | Research scaffold E2E |
| E2E research M2 | `tests/e2e_research_m2.rs` | 3 | Prompt builder E2E |
| E2E research M3 | `tests/e2e_research_m3.rs` | 9 | Research loop E2E |
| E2E research M4 | `tests/e2e_research_m4.rs` | 6 | Dossier writer & validator E2E |
| E2E research M5 | `tests/e2e_research_m5.rs` | 4 | Web-search phase integration E2E |
| E2E research M6 | `tests/e2e_research_m6.rs` | 4 | Multi-source synthesis pass E2E |
| E2E research M7 | `tests/e2e_research_m7.rs` | 7 | Plan-readiness gate + sldo-plan integration E2E |

**Total backend tests: 241**

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
| E2E transcriber | `ui/src/e2e/transcriber.e2e.test.tsx` | 4 | Standalone transcriber E2E |
| E2E integration | `ui/src/e2e/integration.e2e.test.tsx` | 6 | Full workflow integration |

**Total frontend tests: 122**
