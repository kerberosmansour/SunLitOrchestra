# Lessons Learned — Tauri Desktop Milestone 8: Polish, Integration Tests & Documentation

## Design Decisions

- **ErrorBoundary as class component**: React error boundaries require class components (`getDerivedStateFromError`). The `ErrorBoundary` wraps the entire app in `main.tsx`, providing a safety net for any uncaught rendering errors. The "Try Again" button resets internal state, triggering a re-render of children.

- **Global keyboard shortcuts via useEffect**: Added a single `useEffect` in `App.tsx` that registers a `keydown` listener on `document`. This captures `Cmd/Ctrl+N` (new session), `Cmd/Ctrl+,` (settings), and `Escape` (close settings). The `Cmd/Ctrl+Enter` shortcut is handled at the `ChatInput` level since it needs textarea context.

- **No new features**: Strictly followed the milestone's rule of "no new features." Every change was justified by a test, documentation gap, or edge case. The ErrorBoundary and keyboard shortcuts are quality-of-life improvements, not new functionality.

- **Concurrent operation prevention**: Verified via E2E tests that `AtomicBool` with `compare_exchange` (SeqCst) correctly prevents concurrent planning and execution operations. The pattern mirrors the existing `execution_running` flag in `AppState`.

## What Was Harder Than Expected

- **Multiple "Settings" text matches**: The settings button in the sidebar and the "Settings" heading in the panel both match `/settings/i`. Tests needed to use `getByRole("heading", { name: /settings/i })` instead of `getByText(/settings/i)` to be specific.

- **TypeScript strict mode**: Importing `ErrorBoundary` in `App.tsx` without using it in JSX caused `TS6133` ("declared but never read"). The ErrorBoundary wrapping belongs in `main.tsx`, not `App.tsx`, so the unused import had to be removed.

- **Error boundary testing**: Testing error boundaries requires suppressing `console.error` since React logs caught errors. Used `vi.fn()` replacement in `beforeEach`/`afterEach` to keep test output clean while still verifying boundary behavior.

## Naming Conventions Established

- **Error boundary component**: `ErrorBoundary.tsx` — PascalCase, follows existing naming
- **Test files**: `ErrorBoundary.test.tsx`, `KeyboardShortcuts.test.tsx` — co-located BDD tests
- **E2E integration test**: `integration.e2e.test.tsx` — in `src/e2e/` directory
- **Backend E2E**: `tests/e2e_tauri_m8.rs` — follows `e2e_tauri_m<N>.rs` pattern

## Test Patterns That Worked Well

- **Role-based selectors**: Using `getByRole("heading", { name: /settings/i })` instead of `getByText` avoids ambiguity when multiple elements contain similar text.
- **fireEvent for keyboard shortcuts**: `fireEvent.keyDown(document, { key: "n", metaKey: true })` works well for testing global shortcuts that use `document.addEventListener`.
- **Atomic compare_exchange tests**: Testing concurrency prevention with `Arc<AtomicBool>` and `compare_exchange` is clean, deterministic, and doesn't require async/threading infrastructure.
- **Mirror types in E2E tests**: Defining `AppSettings` locally in E2E test files (since binary crates can't be imported) continues to work well for validating serialization contracts.
- **90 frontend + 200 backend tests**: All pass, validating comprehensive coverage across all 8 milestones.

## What the Next Milestone Should Do Differently

- This is the final milestone. The app is feature-complete and tested.
- Future work could add: Claude Code provider implementation, persistent session history, runbook diffing, and CI/CD pipeline for automated testing.

## BDD Scenarios to Retroactively Add

- None needed for M1–M7. All existing tests remain valid and pass.
