# Tauri Desktop App — SunLitOrchestrate

> **Purpose**: Build a Tauri v2 desktop app with a React + TypeScript frontend that wraps the existing `sldo-plan` and `sldo-run` CLI workflow in a modern chatbot-style interface with live streaming, Markdown editing, voice input, and multi-provider support.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section **and** the Pre-Milestone Protocol. After completing it, follow the Post-Milestone Protocol. Never skip ahead.  
> **Prerequisite reading**: [README.md](../README.md), [tauri-desktop-requirements.md](tauri-desktop-requirements.md), [RUNBOOK-RUST-REWRITE.md](RUNBOOK-RUST-REWRITE.md), [docs/lessons/rust-rewrite-m5.md](lessons/rust-rewrite-m5.md)

---

## Milestone Tracker

Update this table as each milestone is completed. This is the **single source of truth** for progress.

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Tauri workspace scaffolding & shell app | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m1.md` |
| 2 | Chatbot UI — prompt screen & conversation layout | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m2.md` |
| 3 | Planning backend — Tauri commands & streaming | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m3.md` |
| 4 | Markdown plan editor & runbook persistence | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m4.md` |
| 5 | Execution backend — milestone runner & live progress | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m5.md` |
| 6 | Settings panel & provider architecture | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m6.md` |
| 7 | Voice input & speech-to-text integration | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m7.md` |
| 8 | Polish, integration tests & documentation | `done` | 2026-03-17 | 2026-03-17 | `docs/lessons/tauri-desktop-m8.md` |

<!-- Status values: not_started | in_progress | done -->
<!-- Lessons files go in docs/lessons/tauri-desktop-m<N>.md -->

---

## Pre-Milestone Protocol

**Do this before every milestone — no exceptions.**

1. **Read the lessons file from the previous milestone** (if one exists). Its path is in the Milestone Tracker table. Apply any design corrections, naming changes, or test strategy improvements it calls for before writing new code.
2. **Read the current milestone section fully** — goal, context, change set, BDD scenarios, regression tests, and smoke tests — before writing any code.
3. **Run the full existing test suite** and confirm it passes. Record the baseline:
   ```
   cargo test --workspace 2>&1 | tail -5
   ```
   If any tests fail before you start, **stop and fix them first**. Do not begin a milestone on a red baseline.
4. **Read the files listed in "Files Most Likely Touched"** for the current milestone. Understand their current shape before changing them.
5. **Update the Milestone Tracker** in this file: set the current milestone's Status to `in_progress` and record the Started date.
6. **Create BDD test files first** — write the scenario tests from the acceptance table **before** writing production code. Tests declare the contract, then implementation satisfies it.
7. **Create E2E test stubs** — write the end-to-end runtime validation tests from the milestone's "E2E Runtime Validation" section as stubs before writing production code.

---

## Post-Milestone Protocol

**Do this after every milestone — no exceptions.**

1. **Run the full test suite** (backend and frontend). Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   cd crates/sldo-tauri/ui && npx vitest run 2>&1 | tail -10
   ```
2. **Run the E2E runtime validation tests** for this milestone:
   ```
   cargo test --workspace --test 'e2e_*'
   cd crates/sldo-tauri/ui && npx vitest run --include 'src/e2e/**' 2>&1 | tail -10
   ```
3. **Verify the app builds and boots** — frontend compiles, backend compiles, and the app launches to a usable state:
   ```
   cd crates/sldo-tauri/ui && npm run build
   cargo build --workspace
   cargo tauri build --debug  # or: cargo tauri dev (for interactive check)
   ```
4. **Run the smoke tests** listed in the milestone. Check off each item in this runbook file.
5. **Verify backward compatibility**: The existing CLI binaries (`sldo-plan`, `sldo-run`) and all tests in `tests/` must still function unchanged.
6. **Update ARCHITECTURE.md** following the Documentation Update table at the bottom of this runbook.
7. **Update README.md** if user-facing capabilities changed, following the Documentation Update table.
8. **Write a lessons-learned file** at `docs/lessons/tauri-desktop-m<N>.md` containing:
   - What design decisions were made and why
   - What was harder than expected
   - What naming conventions were established (type names, file names, test patterns)
   - What test patterns worked well or didn't
   - What the next milestone should do differently based on what was learned
   - Any BDD scenarios that should be retroactively added to earlier milestones
9. **Update the Milestone Tracker** in this file: set Status to `done`, record the Completed date, and fill in the Lessons File path.
10. **Re-read the next milestone's section** with fresh eyes, and note in the lessons file whether any of its assumptions need to change.

---

## Background Context

### Current State

SunLitOrchestrate is a fully functional Rust CLI toolkit for AI-driven software development. The Rust rewrite (5 milestones, completed 2026-03-16) produced:

- **`sldo-common`** (`crates/sldo-common/`) — Shared library with 8 modules: `copilot.rs` (Copilot CLI invocation with streaming), `runbook.rs` (Markdown milestone tracker parsing), `toolflags.rs` (allow/deny permission flags), `detect.rs` (build/test command auto-detection), `git.rs` (repo & branch safety), `preflight.rs` (pre-flight validation), `logging.rs` (timestamped log files), `color.rs` (colored terminal output).
- **`sldo-plan`** (`crates/sldo-plan/src/main.rs`) — CLI binary that takes a prompt file and repo directory, invokes GitHub Copilot CLI to generate a milestone-based Markdown runbook. Supports iterative refinement (up to `--max-iterations`), runbook validation, and template fallback.
- **`sldo-run`** (`crates/sldo-run/src/main.rs`) — CLI binary that drives Copilot CLI through runbook milestones one at a time, verifying build and test commands after each. Supports auto-detection of build/test commands, retry with cooldown, and tracker state display.
- **40+ unit tests** across modules, **28+ E2E integration tests** across 5 test suites in `tests/`.
- **Design CSS** at `docs/App.css` — dark-themed with gold/amber accents, chatbot-ready layout tokens.
- **Logo** at `docs/sunlit.jpeg`.

### Problem

The CLI tools work well but require terminal usage and command-line knowledge. There is no graphical interface for:

1. **No GUI for prompt entry**: Users must create a prompt file and run `sldo-plan` from the terminal. There's no interactive way to type or speak a prompt.
2. **No plan visualization**: The generated runbook is a Markdown file on disk. There's no way to review, edit, or approve the plan in a structured UI before execution.
3. **No live execution monitoring**: `sldo-run` streams to the terminal. There's no rich UI showing milestone progress, build/test results, or agent activity in real time.
4. **No settings management**: Model selection, tool flags, and execution parameters are CLI flags. There's no persistent settings panel.
5. **No voice input**: Users cannot speak instructions — they must type and save prompt files.
6. **Single provider**: The system is tightly coupled to GitHub Copilot CLI. There's no abstraction for swapping to other coding agents.

### Target Architecture

```
SunLitOrchestrate/
├── Cargo.toml                    # Workspace root (adds sldo-tauri member)
├── crates/
│   ├── sldo-common/              # Shared library (UNCHANGED)
│   ├── sldo-plan/                # CLI binary (UNCHANGED)
│   ├── sldo-run/                 # CLI binary (UNCHANGED)
│   └── sldo-tauri/               # NEW: Tauri v2 app
│       ├── Cargo.toml            # Tauri backend, depends on sldo-common
│       ├── tauri.conf.json       # Tauri configuration
│       ├── build.rs              # Tauri build script
│       ├── icons/                # App icons
│       ├── src/
│       │   ├── main.rs           # Tauri app entry point
│       │   ├── commands/         # Tauri command handlers
│       │   │   ├── mod.rs
│       │   │   ├── plan.rs       # Planning commands (wraps sldo-common)
│       │   │   ├── run.rs        # Execution commands (wraps sldo-common)
│       │   │   ├── settings.rs   # Settings CRUD
│       │   │   └── voice.rs      # Speech-to-text proxy
│       │   ├── state.rs          # Managed app state
│       │   ├── events.rs         # Event types for streaming
│       │   └── provider.rs       # Agent provider abstraction
│       └── ui/                   # React + TypeScript frontend
│           ├── package.json
│           ├── tsconfig.json
│           ├── vite.config.ts
│           ├── index.html
│           ├── public/
│           │   └── sunlit.jpeg
│           └── src/
│               ├── main.tsx
│               ├── App.tsx
│               ├── App.css       # Design tokens (from docs/App.css)
│               ├── components/
│               │   ├── ChatInput.tsx
│               │   ├── ConversationView.tsx
│               │   ├── MarkdownEditor.tsx
│               │   ├── MilestoneTracker.tsx
│               │   ├── Sidebar.tsx
│               │   ├── SettingsPanel.tsx
│               │   └── VoiceButton.tsx
│               ├── hooks/
│               │   ├── useStreamingEvents.ts
│               │   ├── usePlan.ts
│               │   ├── useExecution.ts
│               │   └── useVoice.ts
│               ├── types/
│               │   └── index.ts
│               └── e2e/
│                   └── *.e2e.test.tsx
├── tests/                        # Existing E2E tests (UNCHANGED)
└── docs/
    └── lessons/tauri-desktop-m*.md
```

### Key Design Principles

1. **Reuse `sldo-common`, never duplicate**: The Tauri backend imports `sldo-common` as a library dependency. Planning logic reuses `CopilotInvocation`, `runbook::parse_tracker`, `toolflags`, `detect`, `git`, `preflight`, and `logging` directly. No re-implementation.
2. **Stream everything via Tauri events**: The Rust backend emits events (`emit`) for every line of Copilot output, milestone status change, and build/test result. The React frontend listens and renders in real time.
3. **Provider-agnostic design**: Introduce a `Provider` trait that abstracts agent invocation. Start with a `CopilotProvider` that wraps the existing `CopilotInvocation`. Design so `ClaudeCodeProvider` and others can be added later.
4. **Existing CLIs are untouched**: `sldo-plan` and `sldo-run` binaries remain fully functional CLI tools. The Tauri app is an **additional** interface, not a replacement.
5. **Frontend uses existing CSS**: The `docs/App.css` design tokens and component styles are the design foundation. Adapt, don't redesign.
6. **BDD test-first**: Every milestone writes tests before production code, following the established `Given/When/Then` pattern.
7. **Modular commands**: Each Tauri command module (`plan.rs`, `run.rs`, `settings.rs`, `voice.rs`) is self-contained and testable in isolation.

### What to Keep

- All files in `crates/sldo-common/` — no modifications
- All files in `crates/sldo-plan/` — no modifications
- All files in `crates/sldo-run/` — no modifications
- All test files in `tests/` — no modifications
- `src/plan-milestones.sh` and `src/run-milestones.sh` — legacy scripts preserved
- `docs/App.css` — used as design source (copied into the Tauri frontend)
- `docs/sunlit.jpeg` — used as app logo

### What to Change

- **`Cargo.toml` (root)** — Add `crates/sldo-tauri` to workspace members
- **`crates/sldo-tauri/`** — NEW: Entire Tauri v2 application (Rust backend + React frontend)
- **`README.md`** — Add desktop app section with build/run instructions
- **`docs/ARCHITECTURE.md`** — Create or update with Tauri architecture details

---

## BDD Practices

Every milestone follows these rules. Apply them consistently.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first — backend `#[test]` modules for backend scenarios, frontend `describe`/`it` blocks for frontend scenarios.
3. Confirm the tests fail (they reference types/functions that don't exist yet).
4. Write the production code to make the tests pass.
5. Refactor if needed, re-run tests to confirm green.

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
| Backend integration/BDD tests | `tests/e2e_tauri_m<N>.rs` | `tests/` |
| Frontend unit tests | `<module>.test.ts` or `<module>.test.tsx` | Co-located with source file |
| Frontend page tests | `<Component>.test.tsx` | Co-located with component |
| E2E runtime validation (backend) | `tests/e2e_tauri_m<N>.rs` | `tests/` |
| E2E runtime validation (frontend) | `src/e2e/<feature>.e2e.test.tsx` | `crates/sldo-tauri/ui/src/e2e/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go **beyond compilation** and verify that the system works correctly **at runtime**. These tests prove:

1. **The app boots without errors** — Tauri backend initializes, managed state is wired, and the frontend loads without console errors or white screens.
2. **Runtime contracts are met** — types serialize/deserialize correctly over IPC boundaries, commands return expected shapes, and events fire with correct payloads.
3. **BDD scenarios work at runtime, not just in isolation** — integration between Rust backend and React frontend is tested across the Tauri IPC boundary.
4. **No runtime panics, unhandled rejections, or silent failures** — the app survives real-world usage patterns.

#### E2E Test Layers

| Layer | What It Proves | How to Run |
|---|---|---|
| **Backend integration E2E** | Tauri commands wire together, managed state initializes, calls execute through full pipeline, no panics | `cargo test --workspace --test 'e2e_tauri_*'` |
| **Frontend rendering E2E** | Pages render without errors, components mount/unmount cleanly, user interactions produce correct state transitions | `cd crates/sldo-tauri/ui && npx vitest run --include 'src/e2e/**'` |
| **Build-and-boot E2E** | Frontend builds without errors, Tauri backend compiles, app launches | `cd crates/sldo-tauri/ui && npm run build && cargo build -p sldo-tauri` |

---

## Milestone Plan

---

### Milestone 1 — Tauri Workspace Scaffolding & Shell App

**Goal**: Create the `sldo-tauri` crate with a working Tauri v2 app that opens a window, loads a React frontend that renders "Hello SunLitOrchestrate", and confirms the workspace builds end-to-end including all existing crates.

**Context**: The workspace currently has three crates (`sldo-common`, `sldo-plan`, `sldo-run`). This milestone adds a fourth crate `sldo-tauri` as a Tauri v2 application. The Tauri backend depends on `sldo-common` as a library. The React frontend is initialized with Vite + TypeScript. The existing `docs/App.css` is copied into the frontend as the CSS foundation. No functional features yet — this is purely scaffolding.

**Important design rule**: The Tauri crate must not break any existing `cargo test --workspace` or `cargo build --workspace` commands. All 28+ existing E2E tests must continue to pass.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. Read these files before making changes:
   - `Cargo.toml` (root) — understand workspace member list
   - `crates/sldo-common/Cargo.toml` — understand shared dependency versions
   - `docs/App.css` — understand design tokens to copy into frontend

#### Files Most Likely Touched

| File | Change |
|---|---|
| `Cargo.toml` | Add `crates/sldo-tauri` to workspace members |
| `crates/sldo-tauri/Cargo.toml` | NEW: Tauri app package with dependencies on `sldo-common`, `tauri`, `serde`, `serde_json`, `tokio` |
| `crates/sldo-tauri/tauri.conf.json` | NEW: Tauri v2 configuration (window size, title, CSP, dev server URL) |
| `crates/sldo-tauri/build.rs` | NEW: Tauri build script (`tauri_build::build()`) |
| `crates/sldo-tauri/src/main.rs` | NEW: Tauri entry point — creates app with empty command set, opens main window |
| `crates/sldo-tauri/ui/package.json` | NEW: React + TypeScript + Vite project with `@tauri-apps/api` dependency |
| `crates/sldo-tauri/ui/vite.config.ts` | NEW: Vite config for Tauri (dev server port, HMR) |
| `crates/sldo-tauri/ui/tsconfig.json` | NEW: TypeScript config |
| `crates/sldo-tauri/ui/index.html` | NEW: HTML entry point |
| `crates/sldo-tauri/ui/src/main.tsx` | NEW: React root mount |
| `crates/sldo-tauri/ui/src/App.tsx` | NEW: Shell app component rendering "Hello SunLitOrchestrate" with logo |
| `crates/sldo-tauri/ui/src/App.css` | NEW: Copy of `docs/App.css` design tokens |
| `crates/sldo-tauri/ui/public/sunlit.jpeg` | NEW: Copy of logo for static serving |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below. They should compile but fail.
2. **Initialize the Tauri crate**:
   - Create `crates/sldo-tauri/Cargo.toml` with Tauri v2 dependencies and `sldo-common` dependency.
   - Create `crates/sldo-tauri/build.rs` with `tauri_build::build()`.
   - Create `crates/sldo-tauri/tauri.conf.json` with app identifier `com.sunlit.orchestrate`, window title "SunLitOrchestrate", default size 1200×800, dev URL `http://localhost:5173`.
   - Create `crates/sldo-tauri/src/main.rs` with minimal Tauri app setup.
3. **Initialize the React frontend**:
   - Create `crates/sldo-tauri/ui/package.json` with React 18, TypeScript, Vite, `@tauri-apps/api` v2.
   - Create `crates/sldo-tauri/ui/vite.config.ts`, `tsconfig.json`, `index.html`.
   - Create `crates/sldo-tauri/ui/src/main.tsx` (React root) and `App.tsx` (shell component).
   - Copy `docs/App.css` to `crates/sldo-tauri/ui/src/App.css`.
   - Copy `docs/sunlit.jpeg` to `crates/sldo-tauri/ui/public/sunlit.jpeg`.
4. **Update workspace root**:
   - Add `"crates/sldo-tauri"` to the `[workspace] members` list in root `Cargo.toml`.
5. **Install frontend dependencies**: Run `npm install` in `crates/sldo-tauri/ui/`.
6. **Verify all existing tests still pass**: `cargo test --workspace`.
7. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Tauri workspace integration**

| Scenario | Given | When | Then |
|---|---|---|---|
| Workspace builds with Tauri crate | sldo-tauri added to workspace members | `cargo build --workspace` runs | All 4 crates compile without errors |
| Existing tests unaffected | sldo-tauri crate exists in workspace | `cargo test --workspace --test 'e2e_*'` runs | All 28+ existing E2E tests pass |
| Tauri crate depends on sldo-common | sldo-tauri Cargo.toml lists sldo-common | `cargo check -p sldo-tauri` runs | Compiles and can reference `sldo_common::version()` |
| Frontend builds | React frontend initialized with Vite | `cd crates/sldo-tauri/ui && npm run build` | Produces dist/ folder with index.html and JS bundles |

**Feature: Shell app renders**

| Scenario | Given | When | Then |
|---|---|---|---|
| App CSS copied | `docs/App.css` content exists | `crates/sldo-tauri/ui/src/App.css` is read | Contains `--color-primary` and `--bg-base` token definitions |
| Logo available | `sunlit.jpeg` copied to public/ | Frontend references `/sunlit.jpeg` | Image file exists and is valid JPEG |

#### Regression Tests

- `cargo test --workspace --test e2e_scaffold_m1` — workspace builds, binaries run
- `cargo test --workspace --test e2e_common_m2` — shared library functions work
- `cargo test --workspace --test e2e_plan_m3` — planning binary works
- `cargo test --workspace --test e2e_run_m4` — execution binary works
- `cargo test --workspace --test e2e_integration_m5` — full integration tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `tauri_crate_compiles` | sldo-tauri compiles as part of workspace | `cargo check -p sldo-tauri` exits 0 |
| `tauri_references_common` | sldo-tauri can use sldo-common types | Test calls `sldo_common::version()` and gets a non-empty string |
| `frontend_dist_exists_after_build` | Frontend produces build output | `crates/sldo-tauri/ui/dist/index.html` exists after `npm run build` |
| `existing_tests_still_pass` | No regression from adding Tauri crate | All existing test suites pass |

#### Smoke Tests

- [x] `cargo build --workspace` completes without errors
- [x] `cargo test --workspace` — all existing tests pass, no new failures
- [x] `cd crates/sldo-tauri/ui && npm run build` produces `dist/` folder
- [ ] `cargo tauri dev` opens a window showing "Hello SunLitOrchestrate" (manual check)
- [ ] App window displays the SunLit logo
- [ ] App uses dark theme from `App.css` tokens

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Add "Desktop App" section with `cargo tauri dev` instructions.

---

### Milestone 2 — Chatbot UI: Prompt Screen & Conversation Layout

**Goal**: Build the chatbot-style home screen with a centered prompt input, and implement the conversation layout that appears after the user submits a prompt. The UI transitions from a "welcome" state to a "workspace" state, matching the feel of ChatGPT/Claude but tailored for long-running planning workflows.

**Context**: The Tauri shell app (M1) renders a static page. This milestone adds the interactive React UI: a sidebar for navigation, a home screen with the prompt centered, and a conversation view for streaming responses. No backend integration yet — the UI uses mock data to demonstrate layout and transitions. The design uses `App.css` tokens established in M1.

**Important design rule**: The UI must feel like a modern chatbot home screen (centered prompt, clean hero) that transitions smoothly into a conversation workspace (prompt at bottom, messages above). All layout and styling must use the existing CSS design tokens — no new design systems.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m1.md`** — apply any corrections from M1.
3. Read these files before making changes:
   - `crates/sldo-tauri/ui/src/App.tsx` — current shell component
   - `crates/sldo-tauri/ui/src/App.css` — design tokens available

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/ui/src/App.tsx` | Replace shell content with router/state-driven layout |
| `crates/sldo-tauri/ui/src/App.css` | Add component-specific styles using existing tokens |
| `crates/sldo-tauri/ui/src/components/Sidebar.tsx` | NEW: Navigation sidebar with logo, session list, settings link |
| `crates/sldo-tauri/ui/src/components/ChatInput.tsx` | NEW: Prompt input textarea with submit button, adapts position (centered vs bottom) |
| `crates/sldo-tauri/ui/src/components/ConversationView.tsx` | NEW: Message list renderer — user prompts, agent responses, status updates |
| `crates/sldo-tauri/ui/src/components/HomeScreen.tsx` | NEW: Welcome/hero screen with centered prompt, sample prompt chips |
| `crates/sldo-tauri/ui/src/types/index.ts` | NEW: TypeScript types for Message, Session, AppPhase |

#### Step-by-Step

1. **Write BDD test stubs first** for all frontend scenarios below.
2. **Define core TypeScript types**:
   - `AppPhase`: `"home" | "planning" | "reviewing" | "executing"`
   - `Message`: `{ id, role: "user" | "assistant" | "system", content, timestamp }`
   - `Session`: `{ id, title, messages, phase, runbookPath? }`
3. **Build the Sidebar**:
   - Logo at top (`sunlit.jpeg`)
   - Session list (placeholder for now)
   - "New Session" button
   - Settings link at bottom
4. **Build the HomeScreen**:
   - Hero section with app title and subtitle
   - Centered `ChatInput` component
   - Sample prompt chips below the input (`promptChip` class from App.css)
5. **Build the ChatInput**:
   - Textarea with auto-resize
   - Submit button (gold gradient from `button` class)
   - Handles Enter to submit (Shift+Enter for newline)
   - Renders centered on home screen, pinned to bottom in conversation
6. **Build the ConversationView**:
   - Scrollable message list
   - Renders user messages and assistant messages with distinct styling
   - Auto-scrolls to bottom on new messages
   - `ChatInput` pinned at bottom
7. **Wire App.tsx state transitions**:
   - Default phase: `"home"` → shows HomeScreen
   - On prompt submit: phase becomes `"planning"` → shows ConversationView with user message and mock streaming response
8. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Home screen layout**

| Scenario | Given | When | Then |
|---|---|---|---|
| Centered prompt on load | App launches | Home screen renders | Prompt input is vertically and horizontally centered with hero text above |
| Sample prompts shown | App on home screen | User views below input | At least 3 prompt chips are visible |
| Clicking prompt chip fills input | User views home screen | User clicks a prompt chip | ChatInput textarea is populated with the chip text |

**Feature: Prompt submission and transition**

| Scenario | Given | When | Then |
|---|---|---|---|
| Submit transitions to conversation | User on home screen | User types prompt and presses Enter | View transitions to ConversationView with prompt at bottom |
| User message appears in conversation | User submits "Build a REST API" | Conversation view loads | First message shows "Build a REST API" as user message |
| Empty prompt rejected | User on home screen | User presses Enter with empty input | Nothing happens, view stays on home screen |

**Feature: Conversation view layout**

| Scenario | Given | When | Then |
|---|---|---|---|
| Messages scroll | Conversation has many messages | New message is added | View auto-scrolls to show the latest message |
| Input stays at bottom | User is in conversation view | User resizes window | ChatInput remains pinned to the bottom of the viewport |

**Feature: Sidebar navigation**

| Scenario | Given | When | Then |
|---|---|---|---|
| Sidebar shows logo | App renders | User views sidebar | SunLit logo is displayed at top of sidebar |
| New session resets to home | User is in conversation | User clicks "New Session" | App returns to home screen with centered prompt |

#### Regression Tests

- All M1 scenarios still pass
- `cargo build --workspace` still succeeds
- Frontend builds without errors: `npm run build`

#### E2E Runtime Validation

**File**: `crates/sldo-tauri/ui/src/e2e/chatui.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `home_screen_renders_without_errors` | Home screen mounts cleanly | No unhandled exceptions, hero text visible |
| `prompt_submission_transitions_view` | State transition works at runtime | ConversationView mounts after submit, user message visible |
| `sidebar_renders_with_logo` | Sidebar component mounts | Logo element present, sidebar visible |
| `empty_prompt_does_not_transition` | Input validation works | Phase remains "home" after empty submit |

#### Smoke Tests

- [x] `npm run build` in `crates/sldo-tauri/ui` succeeds
- [ ] `cargo tauri dev` shows home screen with centered prompt
- [ ] Clicking a prompt chip populates the input
- [ ] Submitting a prompt transitions to conversation view
- [ ] Sidebar displays logo and "New Session" button
- [ ] "New Session" returns to home screen
- [ ] Dark theme with gold accents visible throughout

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Update desktop app section with UI preview description.

---

### Milestone 3 — Planning Backend: Tauri Commands & Streaming

**Goal**: Wire the Tauri Rust backend to invoke the planning workflow via `sldo-common` and stream real-time output to the React frontend via Tauri events, so the user can submit a prompt and watch the runbook being generated live in the conversation view.

**Context**: The React UI (M2) has a prompt input and conversation view but uses mock data. The existing `sldo-plan` binary shows how planning works: it builds a `CopilotInvocation` with the prompt, invokes Copilot CLI, and streams stdout/stderr. This milestone creates Tauri commands that reuse `sldo-common` modules (`CopilotInvocation`, `preflight`, `toolflags`, `logging`) and emit progress events to the frontend. The frontend listens for these events and renders them as streaming messages in the conversation view.

**Important design rule**: The Tauri commands must reuse `sldo-common` modules directly — do not re-implement Copilot invocation, tool flags, or validation. The `CopilotInvocation` struct's `run()` method pipes output to stdout/stderr; introduce an event-emitting variant that sends lines to the Tauri window instead.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m2.md`** — apply any corrections from M2.
3. Read these files before making changes:
   - `crates/sldo-common/src/copilot.rs` — understand `CopilotInvocation::run()` and how it streams output
   - `crates/sldo-common/src/toolflags.rs` — understand `plan_allow_flags()` and `plan_deny_flags()`
   - `crates/sldo-common/src/preflight.rs` — understand `check_copilot_installed()`, `check_file_exists()`, `check_git_safety()`
   - `crates/sldo-plan/src/main.rs` — understand `build_planning_prompt()`, `validate_runbook()`, the iteration loop

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/src/main.rs` | Register planning commands and managed state |
| `crates/sldo-tauri/src/commands/mod.rs` | NEW: Module declarations |
| `crates/sldo-tauri/src/commands/plan.rs` | NEW: `start_planning` Tauri command — validates inputs, builds prompt, invokes Copilot, emits streaming events |
| `crates/sldo-tauri/src/state.rs` | NEW: Managed state — current session, active plan path, planning status |
| `crates/sldo-tauri/src/events.rs` | NEW: Event payload types (`PlanProgress`, `PlanComplete`, `PlanError`) |
| `crates/sldo-common/src/copilot.rs` | Add `run_with_callback()` method that accepts a closure for each output line (instead of println!) |
| `crates/sldo-tauri/ui/src/hooks/useStreamingEvents.ts` | NEW: React hook that listens for Tauri events and accumulates messages |
| `crates/sldo-tauri/ui/src/hooks/usePlan.ts` | NEW: React hook wrapping the `start_planning` command invocation |
| `crates/sldo-tauri/ui/src/components/ConversationView.tsx` | Update to render streaming plan output as it arrives |
| `crates/sldo-tauri/ui/src/types/index.ts` | Add event payload types mirroring Rust structs |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Extend `CopilotInvocation` in `sldo-common`**:
   - Add a `run_with_callback<F>(&self, log_file: &LogFile, on_line: F) -> Result<i32>` method where `F: FnMut(&str)`.
   - This method works like `run()` but calls `on_line` for each stdout/stderr line instead of `println!`.
   - The existing `run()` method continues to work unchanged (calls `run_with_callback` with a print closure internally).
3. **Create Rust event types** in `events.rs`:
   - `PlanProgressEvent { line: String, stream: "stdout" | "stderr", timestamp: String }`
   - `PlanCompleteEvent { runbook_path: String, validation_issues: Vec<String> }`
   - `PlanErrorEvent { error: String }`
4. **Create managed state** in `state.rs`:
   - `AppState { current_session: Mutex<Option<Session>>, settings: Mutex<AppSettings> }`
   - `AppSettings { model: String, max_iterations: u32, repo_dir: Option<PathBuf> }`
5. **Create the `start_planning` command** in `commands/plan.rs`:
   - Accepts `prompt: String`, `repo_dir: String`, optional `output_path: String`
   - Runs preflight checks (copilot installed, git safety)
   - Builds planning prompt using logic from `sldo-plan`
   - Invokes `CopilotInvocation::run_with_callback()`, emitting `plan-progress` events for each line
   - On completion, validates runbook and emits `plan-complete` or `plan-error`
   - Runs in a `tokio::spawn` so the command returns immediately while planning streams
6. **Create frontend hooks**:
   - `useStreamingEvents(eventName)` — Tauri `listen()` wrapper that accumulates event payloads into state
   - `usePlan()` — Calls `invoke("start_planning", ...)` and returns `{ status, messages, runbookPath }`
7. **Update ConversationView** to render streaming output from `useStreamingEvents`.
8. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Planning command invocation**

| Scenario | Given | When | Then |
|---|---|---|---|
| Copilot not installed | Copilot CLI not on PATH | `start_planning` is called | Returns error event with message mentioning "copilot" |
| Protected branch rejected | Repo is on `main` branch | `start_planning` is called | Returns error event mentioning "protected" or "main" |
| Planning starts successfully | Copilot installed, feature branch, valid repo | `start_planning` is called | Progress events begin streaming |
| Planning produces runbook | Valid planning invocation completes | Copilot finishes | `plan-complete` event emitted with runbook path |

**Feature: Streaming output to frontend**

| Scenario | Given | When | Then |
|---|---|---|---|
| Progress events arrive in order | Planning is in progress | Copilot emits lines | Frontend receives `plan-progress` events in order |
| Streaming hook accumulates messages | Frontend listening for events | Multiple events arrive | `useStreamingEvents` state contains all received lines |

**Feature: run_with_callback extends CopilotInvocation**

| Scenario | Given | When | Then |
|---|---|---|---|
| Callback receives all lines | CopilotInvocation configured | `run_with_callback` is called | Callback is invoked for every stdout/stderr line |
| Existing run() still works | CopilotInvocation configured | `run()` is called | Output prints to terminal as before (backward compatible) |

#### Regression Tests

- `cargo test -p sldo-common` — all existing unit tests pass (copilot.rs backward compatible)
- `cargo test --workspace --test 'e2e_*'` — all 28+ E2E tests pass
- Frontend builds: `cd crates/sldo-tauri/ui && npm run build`
- M2 UI scenarios still work (home screen, transitions)

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `copilot_run_with_callback_captures_lines` | `run_with_callback` collects output via closure | Mock copilot produces lines, callback receives them all |
| `plan_command_rejects_missing_copilot` | Preflight validation works in Tauri context | Error returned when copilot binary not found |
| `plan_allow_flags_used` | Planning uses correct tool flags | `plan_allow_flags()` returns non-empty vec |

**File**: `crates/sldo-tauri/ui/src/e2e/planning.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `streaming_events_render_in_conversation` | Events appear as messages | Mock events produce visible message elements |
| `plan_error_shows_in_ui` | Errors surface to user | Error event renders an error message in conversation |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` — all existing + new tests pass
- [ ] `cargo tauri dev` — enter a prompt → streaming output appears in conversation (requires copilot on PATH)
- [ ] Error message shown if copilot not installed
- [ ] Progress events stream in real time (not all at once)

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Document Tauri command registration, event streaming pattern, and `run_with_callback` addition.

---

### Milestone 4 — Markdown Plan Editor & Runbook Persistence

**Goal**: After a plan is generated, render it as editable Markdown in the UI. The user can review, edit, and save the runbook before proceeding to execution. The runbook is persisted to disk and re-parseable by `sldo-common/runbook.rs`.

**Context**: After M3, the planning flow generates a runbook and streams output. But the user cannot see or edit the resulting Markdown plan. This milestone adds a Markdown editor component that loads the generated runbook, lets the user modify it, saves changes back to disk, and shows a milestone tracker summary. The editor must produce valid Markdown that `runbook::parse_tracker()` can parse.

**Important design rule**: The Markdown editor must produce output that passes `runbook::parse_tracker()` without errors. The editor should not introduce formatting that breaks the existing parser. Test round-trip: parse → edit → save → re-parse.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m3.md`** — apply any corrections from M3.
3. Read these files before making changes:
   - `crates/sldo-common/src/runbook.rs` — understand `parse_tracker()`, `MilestoneRow`, `MilestoneStatus`
   - `crates/sldo-tauri/src/commands/plan.rs` — understand where runbook path is emitted
   - `crates/sldo-tauri/ui/src/components/ConversationView.tsx` — understand current conversation rendering

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/ui/package.json` | Add Markdown editor dependency (e.g., `@uiw/react-md-editor` or `react-markdown` + `react-textarea-autosize`) |
| `crates/sldo-tauri/ui/src/components/MarkdownEditor.tsx` | NEW: Markdown editor component with preview/edit toggle |
| `crates/sldo-tauri/ui/src/components/MilestoneTracker.tsx` | NEW: Visual milestone tracker rendered from parsed runbook data |
| `crates/sldo-tauri/src/commands/plan.rs` | Add `read_runbook` and `save_runbook` Tauri commands |
| `crates/sldo-tauri/src/main.rs` | Register new commands |
| `crates/sldo-tauri/ui/src/hooks/usePlan.ts` | Add `loadRunbook()` and `saveRunbook()` functions |
| `crates/sldo-tauri/ui/src/App.tsx` | Add "reviewing" phase that shows MarkdownEditor |
| `crates/sldo-tauri/ui/src/types/index.ts` | Add `MilestoneRow` and `MilestoneStatus` TypeScript types |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Create Tauri backend commands**:
   - `read_runbook(path: String) -> Result<RunbookData>` — reads file, parses tracker, returns content + parsed milestones
   - `save_runbook(path: String, content: String) -> Result<Vec<String>>` — writes content to disk, re-parses to validate, returns any validation issues
   - `RunbookData { content: String, milestones: Vec<MilestoneRow>, path: String }`
3. **Create MarkdownEditor component**:
   - Toggle between edit mode (raw Markdown textarea) and preview mode (rendered Markdown)
   - Syntax highlighting for code blocks
   - Auto-save on blur or Ctrl+S
   - Save button that calls `save_runbook` command
   - Shows validation warnings if save produces issues
4. **Create MilestoneTracker component**:
   - Renders milestone table from parsed `MilestoneRow[]` data
   - Color-coded status: green for done, yellow for in_progress, gray for not_started
   - Visual progress bar showing completion percentage
5. **Wire the "reviewing" phase**:
   - After planning completes (`plan-complete` event), phase transitions to `"reviewing"`
   - MarkdownEditor loads the runbook from the emitted path
   - MilestoneTracker shows alongside the editor
   - "Execute Plan" button transitions to `"executing"` phase (wired in M5)
6. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Runbook loading and display**

| Scenario | Given | When | Then |
|---|---|---|---|
| Runbook loads after plan completion | Plan completed with valid runbook | Phase transitions to "reviewing" | MarkdownEditor displays the runbook content |
| Milestone tracker rendered | Runbook contains 5 milestones | Editor loads | MilestoneTracker shows 5 rows with correct statuses |
| Invalid runbook path | Path does not exist | `read_runbook` called | Error returned with descriptive message |

**Feature: Runbook editing**

| Scenario | Given | When | Then |
|---|---|---|---|
| Edit and save round-trip | Runbook loaded in editor | User edits text and saves | File on disk matches editor content |
| Parse-after-save validation | User edits milestone table formatting | User saves | `parse_tracker()` still succeeds on saved content |
| Validation warnings shown | User breaks milestone table formatting | User saves | Warning message shown listing validation issues |

**Feature: Editor modes**

| Scenario | Given | When | Then |
|---|---|---|---|
| Toggle edit/preview | Editor in edit mode | User clicks "Preview" | Rendered Markdown shown (not raw text) |
| Toggle preview/edit | Editor in preview mode | User clicks "Edit" | Raw Markdown textarea shown |

#### Regression Tests

- `cargo test -p sldo-common` — `runbook::parse_tracker` tests pass
- All existing E2E tests pass
- M2 and M3 UI scenarios still work
- Planning flow still generates and streams correctly

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `read_runbook_parses_real_file` | `read_runbook` works on actual runbook | Reads `docs/RUNBOOK-RUST-REWRITE.md`, returns 5 milestones |
| `save_and_reparse_roundtrip` | Save produces parseable output | Write content to temp file, re-read, milestones match |
| `save_invalid_content_returns_warnings` | Validation catches broken formatting | Save content missing milestone table, get warnings |

**File**: `crates/sldo-tauri/ui/src/e2e/editor.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `markdown_editor_renders_content` | Editor mounts with content | Content text visible in the editor |
| `milestone_tracker_shows_rows` | Tracker renders milestone data | Correct number of milestone rows rendered |
| `edit_preview_toggle_works` | Mode toggle functions | Switching modes changes the visible component |

#### Smoke Tests

- [x] `cargo build --workspace` succeeds
- [x] `cargo test --workspace` — all tests pass
- [ ] After plan generation, runbook appears in Markdown editor
- [x] Can switch between edit and preview modes
- [ ] Editing and saving produces a valid file on disk
- [x] MilestoneTracker shows colored status indicators
- [x] "Execute Plan" button visible in reviewing phase

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Document runbook persistence flow and editor architecture.

---

### Milestone 5 — Execution Backend: Milestone Runner & Live Progress

**Goal**: Wire the Tauri backend to execute a runbook through its milestones using `sldo-common`, stream live progress (agent output, build/test results, milestone status changes) to the frontend, and render a structured execution view showing real-time milestone progress.

**Context**: The existing `sldo-run` binary shows the execution pattern: loop through milestones, invoke Copilot for each, verify build/test commands, retry on failure. This milestone creates Tauri commands that replicate this logic using `sldo-common` modules and emit granular events for each phase. The frontend renders these as a structured execution view with milestone-by-milestone progress.

**Important design rule**: Execution must be cancellable. The user should be able to stop execution mid-milestone. Use a cancellation token pattern (e.g., `AtomicBool` in managed state) that the execution loop checks between iterations.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m4.md`** — apply any corrections from M4.
3. Read these files before making changes:
   - `crates/sldo-run/src/main.rs` — understand `build_execution_prompt()`, `verify_commands()`, the main execution loop
   - `crates/sldo-common/src/detect.rs` — understand `detect_build_commands()`, `detect_test_commands()`
   - `crates/sldo-tauri/src/commands/plan.rs` — understand existing command pattern
   - `crates/sldo-tauri/src/events.rs` — understand existing event types

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/src/commands/run.rs` | NEW: `start_execution`, `cancel_execution` Tauri commands |
| `crates/sldo-tauri/src/events.rs` | Add execution event types: `ExecutionProgress`, `MilestoneStarted`, `MilestoneCompleted`, `BuildTestResult`, `ExecutionComplete` |
| `crates/sldo-tauri/src/state.rs` | Add execution state: cancellation flag, current milestone, attempt count |
| `crates/sldo-tauri/src/main.rs` | Register execution commands |
| `crates/sldo-tauri/ui/src/hooks/useExecution.ts` | NEW: React hook wrapping execution commands and events |
| `crates/sldo-tauri/ui/src/components/ExecutionView.tsx` | NEW: Structured execution progress view |
| `crates/sldo-tauri/ui/src/components/MilestoneTracker.tsx` | Update to show real-time status changes during execution |
| `crates/sldo-tauri/ui/src/App.tsx` | Add "executing" phase rendering |
| `crates/sldo-tauri/ui/src/types/index.ts` | Add execution event types |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Create execution event types** in `events.rs`:
   - `MilestoneStartedEvent { milestone_number: u32, title: String, attempt: u32 }`
   - `ExecutionProgressEvent { line: String, stream: String, timestamp: String }`
   - `BuildTestResultEvent { command: String, success: bool, output: String }`
   - `MilestoneCompletedEvent { milestone_number: u32, success: bool }`
   - `ExecutionCompleteEvent { all_done: bool, milestones_completed: u32, total: u32 }`
3. **Add cancellation state** — `Arc<AtomicBool>` stored in managed Tauri state.
4. **Create `start_execution` command** in `commands/run.rs`:
   - Accepts `runbook_path: String`, `repo_dir: String`
   - Runs preflight checks
   - Detects or uses configured build/test commands
   - Spawns execution loop on `tokio::spawn`:
     - Parse tracker, find next incomplete milestone
     - Emit `milestone-started` event
     - Build execution prompt (reuse logic from `sldo-run`)
     - Invoke Copilot via `run_with_callback`, emitting `execution-progress` events
     - Run build/test verification, emitting `build-test-result` events
     - Emit `milestone-completed`
     - Check cancellation flag before next iteration
     - Sleep cooldown between attempts
   - On completion, emit `execution-complete`
5. **Create `cancel_execution` command** — sets `AtomicBool` to true, causing the loop to break.
6. **Create `ExecutionView` component**:
   - Shows current milestone being worked on (highlighted in tracker)
   - Streams agent output in a log-style panel
   - Shows build/test results with pass/fail indicators
   - "Cancel Execution" button
   - Progress summary (milestones done / total)
7. **Update MilestoneTracker** to accept live status updates from execution events.
8. **Wire the "executing" phase** in App.tsx.
9. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Execution command**

| Scenario | Given | When | Then |
|---|---|---|---|
| Execution starts from runbook | Valid runbook with not_started milestones | `start_execution` called | `milestone-started` event emitted for first milestone |
| All-done runbook skips execution | Runbook with all milestones done | `start_execution` called | `execution-complete` event emitted with `all_done: true` |
| Cancellation stops execution | Execution in progress | `cancel_execution` called | Execution loop stops after current attempt, `execution-complete` emitted |
| Build/test verification runs | Copilot invocation completes | Build and test commands run | `build-test-result` events emitted for each command |

**Feature: Execution streaming to frontend**

| Scenario | Given | When | Then |
|---|---|---|---|
| Agent output streams live | Execution in progress | Copilot emits lines | `execution-progress` events appear in ExecutionView |
| Milestone transitions shown | Milestone 1 completes, milestone 2 starts | Events fire in sequence | MilestoneTracker updates from not_started → in_progress → done |
| Build failure shown clearly | Build command fails | `build-test-result` event fires | UI shows red failure indicator with command output |

**Feature: Execution UI**

| Scenario | Given | When | Then |
|---|---|---|---|
| Cancel button visible during execution | Phase is "executing" | ExecutionView renders | "Cancel Execution" button is visible and enabled |
| Execution complete shows summary | All milestones done | `execution-complete` fires | Summary shows "N/N milestones completed" |

#### Regression Tests

- All existing E2E tests pass
- Planning flow still works (M3 scenarios)
- Editor still works (M4 scenarios)
- `sldo-run --help` still works (existing CLI unaffected)

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `execution_parses_real_runbook` | Execution reads actual runbook milestones | Parses `docs/RUNBOOK-RUST-REWRITE.md`, detects all milestones done |
| `detect_build_commands_in_own_repo` | Build detection works in Tauri context | Returns `cargo build --workspace` for this repo |
| `cancellation_flag_stops_loop` | Cancellation mechanism works | Set flag, verify loop exits |

**File**: `crates/sldo-tauri/ui/src/e2e/execution.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `execution_view_renders` | ExecutionView mounts cleanly | No errors, milestone tracker visible |
| `cancel_button_present` | Cancel UI exists | Button element found and clickable |
| `progress_events_render_as_log_lines` | Streaming events appear in UI | Mock events produce visible log entries |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` — all tests pass
- [ ] From "reviewing" phase, click "Execute Plan" → execution starts
- [ ] Agent output streams in real time in the execution view
- [ ] Milestone tracker updates as milestones complete
- [ ] Build/test results show pass/fail indicators
- [ ] "Cancel Execution" button stops the process
- [ ] After all milestones complete, summary is shown

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Document execution flow, event types, and cancellation pattern.

---

### Milestone 6 — Settings Panel & Provider Architecture

**Goal**: Build a settings panel where users can configure the agent provider, model, tool permissions, and execution parameters. Introduce a `Provider` trait to abstract agent invocation so the system can support multiple coding agents beyond GitHub Copilot.

**Context**: Currently all invocations use `CopilotInvocation` directly. Model and flags are hardcoded or use single defaults. This milestone adds a settings persistence layer (JSON config file in Tauri app data directory) and a `Provider` trait so the planning and execution commands read from settings instead of hardcoded values. The UI gets a settings panel accessible from the sidebar.

**Important design rule**: The `Provider` trait must be minimal — just enough to abstract invocation. Do not over-engineer for providers that don't exist yet. Start with `CopilotProvider` implementing the trait, and design the trait interface so adding `ClaudeCodeProvider` later requires only a new struct + impl, not changes to existing code.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m5.md`** — apply any corrections from M5.
3. Read these files before making changes:
   - `crates/sldo-common/src/copilot.rs` — understand `CopilotInvocation` struct and methods
   - `crates/sldo-common/src/toolflags.rs` — understand flag functions
   - `crates/sldo-tauri/src/state.rs` — understand current managed state
   - `crates/sldo-tauri/src/commands/plan.rs` — understand how invocation is currently hardcoded
   - `crates/sldo-tauri/src/commands/run.rs` — same

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/src/provider.rs` | NEW: `Provider` trait, `CopilotProvider` implementation |
| `crates/sldo-tauri/src/commands/settings.rs` | NEW: `get_settings`, `update_settings`, `get_available_providers`, `get_available_models` Tauri commands |
| `crates/sldo-tauri/src/state.rs` | Add `AppSettings` struct with provider, model, tool flags, execution params; persistence via JSON |
| `crates/sldo-tauri/src/commands/plan.rs` | Read provider/model/flags from settings instead of hardcoded values |
| `crates/sldo-tauri/src/commands/run.rs` | Same — read from settings |
| `crates/sldo-tauri/src/main.rs` | Register settings commands, load settings on startup |
| `crates/sldo-tauri/ui/src/components/SettingsPanel.tsx` | NEW: Settings form with provider, model, tool flags, execution params |
| `crates/sldo-tauri/ui/src/App.tsx` | Add settings route/view accessible from sidebar |
| `crates/sldo-tauri/ui/src/types/index.ts` | Add `AppSettings`, `Provider` TypeScript types |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Define the `Provider` trait**:
   ```rust
   pub trait Provider: Send + Sync {
       fn name(&self) -> &str;
       fn available_models(&self) -> Vec<String>;
       fn invoke(&self, prompt: &str, model: &str, allow_flags: &[String],
                 deny_flags: &[String], working_dir: &Path, log_file: &LogFile,
                 on_line: Box<dyn FnMut(&str) + Send>) -> Result<i32>;
   }
   ```
3. **Implement `CopilotProvider`** — wraps `CopilotInvocation::run_with_callback()`.
4. **Create `AppSettings` struct**:
   - `provider: String` (default: `"copilot"`)
   - `model: String` (default: `"claude-opus-4.6"`)
   - `allow_flags: Vec<String>` (default: from `toolflags::plan_allow_flags()`)
   - `deny_flags: Vec<String>` (default: from `toolflags::plan_deny_flags()`)
   - `max_attempts: u32` (default: 150)
   - `cooldown_secs: u64` (default: 5)
   - `max_iterations: u32` (default: 3)
   - `repo_dir: Option<String>`
5. **Implement settings persistence**:
   - Save to `<tauri_app_data_dir>/settings.json` via `serde_json`
   - Load on app startup; create defaults if file missing
6. **Create settings Tauri commands**: `get_settings`, `update_settings`.
7. **Update plan and run commands** to read provider/model/flags from managed `AppSettings`.
8. **Create SettingsPanel component**:
   - Provider selector dropdown
   - Model text input
   - Tool flags editor (expandable list with add/remove)
   - Execution params (max attempts, cooldown, max iterations)
   - Save button
   - Repo directory picker
9. **Wire settings into sidebar navigation**.
10. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Provider trait abstraction**

| Scenario | Given | When | Then |
|---|---|---|---|
| CopilotProvider implements Provider | CopilotProvider created | `name()` called | Returns "copilot" |
| CopilotProvider invokes Copilot CLI | Valid configuration | `invoke()` called | Delegates to `CopilotInvocation::run_with_callback()` |
| Provider selection from settings | Settings specify "copilot" | Planning command runs | CopilotProvider is used for invocation |

**Feature: Settings persistence**

| Scenario | Given | When | Then |
|---|---|---|---|
| Default settings on first launch | No settings file exists | App starts | Default settings created with model "claude-opus-4.6" |
| Settings saved to disk | User updates model to "gpt-4o" | Save clicked | settings.json contains `"model": "gpt-4o"` |
| Settings loaded on startup | settings.json exists with custom values | App starts | Managed state reflects saved values |
| Invalid settings file | settings.json is corrupted | App starts | Defaults used, warning logged |

**Feature: Settings UI**

| Scenario | Given | When | Then |
|---|---|---|---|
| Settings panel opens | User in any phase | User clicks Settings in sidebar | SettingsPanel renders with current values |
| Model change persists | User changes model field | User saves and reopens settings | New model value shown |
| Tool flags editable | Settings panel open | User adds an allow flag | Flag appears in the allow list |

#### Regression Tests

- Planning still works with default settings (no regression from settings layer)
- Execution still works with default settings
- All existing E2E tests pass
- CLI binaries unaffected

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m6.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `default_settings_created` | Settings initialization works | Default AppSettings has model "claude-opus-4.6" and non-empty flags |
| `settings_roundtrip_json` | Persistence works | Serialize → write → read → deserialize matches original |
| `copilot_provider_has_correct_name` | Provider trait implemented | `CopilotProvider::name()` returns "copilot" |

**File**: `crates/sldo-tauri/ui/src/e2e/settings.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `settings_panel_renders` | SettingsPanel mounts | Form fields for model, provider visible |
| `settings_loads_values` | Current settings populate form | Mock settings data reflected in input values |
| `settings_save_triggers_command` | Save calls backend | Mock invoke called with updated settings |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` — all tests pass
- [ ] Settings panel opens from sidebar
- [ ] Can change model and save
- [ ] Settings persist across app restart
- [ ] Planning uses saved model setting
- [ ] Default tool flags match `toolflags::plan_allow_flags()`
- [ ] Invalid settings.json → graceful fallback to defaults

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Document Provider trait, settings persistence, and configuration flow.
- **README.md**: Add settings configuration section.

---

### Milestone 7 — Voice Input & Speech-to-Text Integration

**Goal**: Add voice input so the user can speak instructions instead of typing. The voice button appears in the ChatInput component and uses a speech-to-text service (initially OpenAI Whisper API) to transcribe audio. The architecture uses an abstraction layer so the STT provider can be swapped.

**Context**: The ChatInput (M2) accepts typed text. This milestone adds a microphone button that records audio, sends it to a speech-to-text API, and populates the input with the transcription. A `.env` file with `OPENAI_API_KEY` already exists in the workspace root. The Tauri backend handles the API call (to avoid exposing the API key in the frontend).

**Important design rule**: The API key must never be sent to or stored in the frontend. The Tauri backend reads the key from environment or `.env` file and proxies the STT request. The frontend only sends audio data to the Tauri backend.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m6.md`** — apply any corrections from M6.
3. Read these files before making changes:
   - `crates/sldo-tauri/ui/src/components/ChatInput.tsx` — understand current input component
   - `crates/sldo-tauri/src/provider.rs` — understand existing provider abstraction pattern

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/Cargo.toml` | Add `reqwest` (HTTP client), `dotenv` or `dotenvy` (env loading), `base64` |
| `crates/sldo-tauri/src/commands/voice.rs` | NEW: `transcribe_audio` Tauri command — receives audio bytes, calls STT API, returns text |
| `crates/sldo-tauri/src/commands/mod.rs` | Add voice module |
| `crates/sldo-tauri/src/main.rs` | Register voice command |
| `crates/sldo-tauri/ui/src/components/VoiceButton.tsx` | NEW: Microphone button with recording state (idle/recording/transcribing) |
| `crates/sldo-tauri/ui/src/components/ChatInput.tsx` | Integrate VoiceButton, populate textarea with transcription result |
| `crates/sldo-tauri/ui/src/hooks/useVoice.ts` | NEW: React hook for recording audio via MediaRecorder API and invoking transcribe command |
| `crates/sldo-tauri/ui/src/types/index.ts` | Add voice-related types |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Create STT abstraction** in `commands/voice.rs`:
   - Define a trait or enum for STT providers (start simple — just OpenAI Whisper)
   - Read `OPENAI_API_KEY` from environment (using `dotenvy` to load `.env` if present)
   - `transcribe_audio` command: receives base64-encoded audio bytes, calls OpenAI Whisper API via `reqwest`, returns transcribed text
   - Error handling: missing API key, network failure, API error, empty response
3. **Create VoiceButton component**:
   - Three states: idle (microphone icon), recording (pulsing red), transcribing (spinner)
   - Click to start recording, click again to stop
   - Uses `MediaRecorder` API to capture audio from microphone
   - Sends recorded audio to `transcribe_audio` command
4. **Create useVoice hook**:
   - Manages `MediaRecorder` lifecycle
   - Handles browser microphone permission request
   - Converts audio blob to base64 for Tauri command
   - Returns `{ isRecording, isTranscribing, transcript, startRecording, stopRecording }`
5. **Integrate VoiceButton into ChatInput**:
   - Button appears to the right of the submit button
   - After transcription, text populates the textarea
   - User can edit the transcription before submitting
6. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Voice recording**

| Scenario | Given | When | Then |
|---|---|---|---|
| Voice button visible | ChatInput renders | User views input area | Microphone button is visible |
| Recording starts on click | Voice button idle | User clicks microphone | Button shows recording state, audio capture begins |
| Recording stops on second click | Voice button recording | User clicks microphone | Recording stops, audio sent for transcription |
| Transcription populates input | Audio recorded and transcribed | Transcription returns "Build a REST API" | Textarea contains "Build a REST API" |

**Feature: STT backend**

| Scenario | Given | When | Then |
|---|---|---|---|
| Missing API key returns error | `OPENAI_API_KEY` not set | `transcribe_audio` called | Error returned mentioning "API key" |
| Valid audio transcribed | API key set, valid audio | `transcribe_audio` called | Transcribed text returned |
| Network failure handled | API key set, network unavailable | `transcribe_audio` called | Error returned mentioning network/connection |

**Feature: Voice UX**

| Scenario | Given | When | Then |
|---|---|---|---|
| Transcription editable before submit | Text transcribed into textarea | User modifies the text | Modified text is what gets submitted |
| Voice works on home screen | User on home screen | User records and transcribes | Transcribed text appears in centered input |
| Voice works in conversation | User in conversation view | User records and transcribes | Transcribed text appears in bottom input |

#### Regression Tests

- All M1–M6 scenarios still pass
- ChatInput still works for typed input
- Planning and execution flows unaffected
- No API key exposure in frontend

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m7.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `missing_api_key_returns_error` | Error handling for missing key | `transcribe_audio` without key returns error mentioning "API key" |
| `env_file_loading_works` | dotenvy reads .env file | Environment variable loaded from .env if present |

**File**: `crates/sldo-tauri/ui/src/e2e/voice.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `voice_button_renders` | VoiceButton mounts | Microphone button element present |
| `voice_button_toggles_recording_state` | State management works | Clicking toggles between idle and recording state |
| `transcription_populates_input` | End-to-end voice flow (mocked) | Mock transcription text appears in textarea |

#### Smoke Tests

- [x] `cargo build --workspace` succeeds
- [x] `cargo test --workspace` — all tests pass
- [x] Voice button visible next to submit button
- [x] Clicking microphone requests browser permission (first time)
- [x] Recording indicator shown while speaking
- [x] Transcription appears in textarea after recording stops
- [x] Can edit transcription before submitting
- [x] Error message shown if API key missing
- [x] API key NOT visible in frontend/DevTools

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Document voice architecture and STT provider abstraction.
- **README.md**: Add voice input section with setup instructions (API key in `.env`).

---

### Milestone 8 — Polish, Integration Tests & Documentation

**Goal**: Harden the app with comprehensive integration tests, fix edge cases, improve error handling UX, add keyboard shortcuts, finalize documentation, and ensure the app is production-ready for daily use.

**Context**: Milestones 1–7 built all core features. This milestone focuses on polish: ensuring all features work together seamlessly, handling edge cases (network failures, missing tools, concurrent operations), adding quality-of-life features (keyboard shortcuts, loading states, error recovery), and completing all documentation.

**Important design rule**: No new features. This milestone is exclusively about reliability, testability, and documentation. Every change must be justified by a specific test failure, edge case, or documentation gap.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/tauri-desktop-m7.md`** — apply any corrections from M7.
3. Read these files before making changes:
   - All source files in `crates/sldo-tauri/src/` — full backend review
   - All source files in `crates/sldo-tauri/ui/src/` — full frontend review
   - `README.md` — current documentation state

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-tauri/ui/src/App.tsx` | Add keyboard shortcuts, loading states, error boundaries |
| `crates/sldo-tauri/ui/src/components/*.tsx` | Edge case handling, loading/error states, accessibility |
| `crates/sldo-tauri/src/commands/*.rs` | Error handling improvements, edge case fixes |
| `tests/e2e_tauri_m8.rs` | NEW: Comprehensive integration tests |
| `crates/sldo-tauri/ui/src/e2e/integration.e2e.test.tsx` | NEW: Full-flow frontend integration tests |
| `README.md` | Complete desktop app documentation |
| `docs/ARCHITECTURE.md` | NEW or updated: Full architecture document |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below.
2. **Add error boundaries**:
   - React error boundary component wrapping the main app
   - Graceful fallback UI when components crash
   - Error state resets on navigation
3. **Add keyboard shortcuts**:
   - `Cmd/Ctrl+Enter` to submit prompt
   - `Cmd/Ctrl+N` for new session
   - `Cmd/Ctrl+,` to open settings
   - `Escape` to cancel recording or close panels
4. **Add loading states**:
   - Planning: skeleton loader while initializing
   - Execution: spinner while verifying build/test
   - Settings: save confirmation feedback
   - Voice: transcription progress indicator
5. **Handle edge cases**:
   - Concurrent operations: prevent starting a new plan while one is running
   - Network failures: retry suggestions for STT
   - Empty/missing repo directory: clear error message
   - Large runbook files: performance check on editor
6. **Write comprehensive integration tests**:
   - Full flow: prompt → plan → review → execute
   - Error recovery: failed plan → retry → success
   - Settings change mid-workflow
   - Cancellation at every phase
7. **Finalize documentation**:
   - Complete ARCHITECTURE.md with all system components
   - Update README.md with full desktop app usage guide
   - Ensure all lesson files are complete and cross-referenced
8. **Make all BDD tests pass.**

#### BDD Acceptance Scenarios

**Feature: Error handling**

| Scenario | Given | When | Then |
|---|---|---|---|
| Error boundary catches crash | Component throws error | React error boundary triggers | Fallback UI shown with retry option |
| Concurrent plan rejected | Plan currently running | User tries to start another plan | Error message: "Planning already in progress" |
| Missing repo dir shows error | No repo directory configured | User starts planning | Clear error message prompting to set repo directory |

**Feature: Keyboard shortcuts**

| Scenario | Given | When | Then |
|---|---|---|---|
| Cmd+Enter submits prompt | User typing in ChatInput | User presses Cmd+Enter | Prompt submitted (same as clicking submit button) |
| Cmd+N creates new session | User in any phase | User presses Cmd+N | App resets to home screen |
| Escape cancels recording | Voice recording active | User presses Escape | Recording cancelled, input unchanged |

**Feature: Full workflow integration**

| Scenario | Given | When | Then |
|---|---|---|---|
| Prompt to execution flow | App on home screen | User submits prompt, plan generates, user clicks Execute | Execution starts and streams progress |
| Settings affect planning | User changes model to "gpt-4o" in settings | User starts planning | Planning uses "gpt-4o" model |
| Cancel returns to reviewing | Execution in progress | User cancels | Phase returns to "reviewing" with runbook intact |

#### Regression Tests

- Every BDD scenario from M1–M7 still passes
- `cargo test --workspace` — all tests green
- `cd crates/sldo-tauri/ui && npx vitest run` — all frontend tests green
- CLI binaries still fully functional

#### E2E Runtime Validation

**File**: `tests/e2e_tauri_m8.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `full_workspace_builds_clean` | No compile warnings or errors | `cargo build --workspace` exits 0 |
| `all_tauri_e2e_tests_pass` | All milestone E2E tests pass | `cargo test --workspace --test 'e2e_tauri_*'` exits 0 |
| `settings_defaults_valid` | Default config is coherent | Default settings produce valid CopilotInvocation parameters |
| `concurrent_plan_prevention` | Locking works | Cannot start two plans simultaneously |

**File**: `crates/sldo-tauri/ui/src/e2e/integration.e2e.test.tsx`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `full_flow_home_to_execution` | Complete workflow renders | Home → planning → reviewing → executing phases all render |
| `error_boundary_recovers` | App survives component errors | Error boundary catches, fallback shows, recovery works |
| `keyboard_shortcuts_work` | Shortcuts registered | Cmd+Enter triggers submit, Cmd+N resets |

#### Smoke Tests

- [x] `cargo build --workspace` — zero warnings
- [x] `cargo test --workspace` — all tests pass
- [x] `cd crates/sldo-tauri/ui && npx vitest run` — all tests pass
- [ ] `cargo tauri dev` — app launches cleanly
- [ ] Full workflow: type prompt → plan generates → edit plan → execute → milestones complete
- [ ] Settings changes persist and take effect
- [ ] Voice input works end to end
- [x] Keyboard shortcuts work
- [x] Error states show graceful messages
- [x] `sldo-plan --help` and `sldo-run --help` still work (CLI unaffected)
- [x] All 28+ original E2E tests still pass

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: Complete architecture document covering all components.
- **README.md**: Full desktop app section with screenshots description, build instructions, configuration guide, and troubleshooting.

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | ARCHITECTURE.md Update | README.md Update |
|---|---|---|
| 1 | — | Add "Desktop App" section with build instructions |
| 2 | — | Update desktop app section with UI description |
| 3 | Document event streaming pattern, `run_with_callback` | — |
| 4 | Document runbook persistence and editor architecture | — |
| 5 | Document execution flow, events, cancellation | — |
| 6 | Document Provider trait, settings persistence | Add settings configuration section |
| 7 | Document voice architecture, STT abstraction | Add voice input setup instructions |
| 8 | Complete full architecture document | Complete desktop app usage guide |
