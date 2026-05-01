# Lessons Learned — Tauri Desktop Milestone 3: Planning Backend — Tauri Commands & Streaming

## Design Decisions

- **`run_with_callback` takes `(line, stream)` tuple**: The callback receives both the line content and the stream name (`"stdout"` or `"stderr"`). This is more informative than just the line, and allows the frontend to style stderr differently. The original `run()` method delegates to `run_with_callback` with a print closure that maps `"stdout"` to `println!` and `"stderr"` to `eprintln!`.

- **Planning runs in `tokio::spawn`**: The `start_planning` Tauri command returns immediately with the output path. The actual Copilot invocation runs in a spawned task that emits events. This prevents the UI from blocking during planning. `std::panic::catch_unwind` wraps the sync code to prevent panics from crashing the app.

- **Event types use `Serialize + Deserialize`**: All event payloads derive Serde traits for Tauri emission and potential persistence. `PlanProgressEvent` includes a timestamp for ordering.

- **Managed state via `Mutex`**: `AppState` uses `Mutex<Option<PlanningSession>>` for the session and `Mutex<AppSettings>` for settings. Simple and sufficient for single-user desktop use.

- **Planning prompt simplified in Tauri command**: The `build_planning_prompt` in `commands/plan.rs` is a simplified version of the one in `sldo-plan/src/main.rs`. This avoids duplicating the full 100+ line prompt template while keeping the essential structure.

- **Validation function duplicated (intentionally)**: `validate_runbook` is re-implemented in `commands/plan.rs` rather than importing from `sldo-plan` because `sldo-plan` is a binary crate. Consider extracting common validation to `sldo-common` in a future milestone.

## What Was Harder Than Expected

- **Tauri event emission from sync context**: `CopilotInvocation::run_with_callback` is synchronous (spawns a `std::process::Command`), but Tauri's `emit` works in both async and sync contexts. The key was using `AppHandle.clone()` inside the callback closure.

- **E2E test design without Copilot**: Since the CI environment may not have `copilot` installed, E2E tests must handle both the installed and not-installed cases. Tests use `match result { Ok(...) => ..., Err(e) => assert!(e.contains("copilot")) }` pattern.

- **Root workspace `dev-dependencies`**: The E2E test file `tests/e2e_tauri_m3.rs` needed `serde_json` in `[dev-dependencies]` at the workspace root `Cargo.toml`, not in any crate's `Cargo.toml`.

## Naming Conventions Established

- **Tauri command files**: `commands/<domain>.rs` — `commands/plan.rs`, future `commands/run.rs`, `commands/settings.rs`
- **Event types**: `<Feature><EventType>Event` — `PlanProgressEvent`, `PlanCompleteEvent`, `PlanErrorEvent`
- **Event names**: kebab-case — `plan-progress`, `plan-complete`, `plan-error`
- **Frontend hooks**: `use<Feature>.ts` — `useStreamingEvents.ts`, `usePlan.ts`
- **BDD test files**: `<Feature>.test.tsx` in `components/` — `StreamingPlan.test.tsx`
- **E2E test files**: `<feature>.e2e.test.tsx` in `e2e/` — `planning.e2e.test.tsx`
- **Rust E2E**: `tests/e2e_tauri_m3.rs` — follows existing `e2e_tauri_m<N>.rs` pattern

## Test Patterns That Worked Well

- **Optional prop pattern**: Making `streamingLines` optional in `ConversationView` ensured all M2 tests passed unchanged. New streaming tests pass the prop explicitly.
- **Tauri API mocking**: `vi.mock("@tauri-apps/api/event")` and `vi.mock("@tauri-apps/api/core")` allow testing hooks and components without the Tauri runtime.
- **29 frontend tests**: 4 ChatInput, 3 HomeScreen, 4 ConversationView, 4 Sidebar, 3 App, 5 StreamingPlan, 2 Planning E2E, 4 Chatui E2E.
- **Dual-outcome E2E tests**: Each Rust E2E test handles both "copilot installed" and "copilot not installed" outcomes, making them pass in any environment.

## What the Next Milestone Should Do Differently

- **M4 (Markdown Editor)**: The `validate_runbook` function exists in both `sldo-plan` and `commands/plan.rs`. Consider extracting it to `sldo-common` so both the CLI and Tauri can share it.
- **State management**: The `usePlan` hook processes events during render (checking `completeEvents.length > 0`). If this causes re-render loops, switch to `useEffect` for event processing.
- **The `App.css` extra `}` warning**: Still present during vite build (noted in M2 lessons). Should be cleaned up when CSS is next modified.

## BDD Scenarios to Retroactively Add

- None needed for M1 or M2. All existing tests remain valid and pass.
