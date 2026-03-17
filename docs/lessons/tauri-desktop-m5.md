# Lessons Learned — Tauri Desktop Milestone 5: Execution Backend & Live Progress

## Design Decisions

- **Execution events as separate event types**: Rather than a single generic "execution-event" with a `type` discriminator, each event type (`MilestoneStartedEvent`, `ExecutionProgressEvent`, `BuildTestResultEvent`, `MilestoneCompletedEvent`, `ExecutionCompleteEvent`) is its own struct emitted on its own Tauri event channel. This makes frontend `listen()` calls type-safe and avoids runtime type-switching.

- **`Arc<AtomicBool>` cancellation pattern**: The cancellation flag is stored in `AppState` as `Arc<AtomicBool>`. The spawned execution loop clones the `Arc` and checks it between iterations with `Relaxed` ordering, which is sufficient since we only need eventual consistency (a few ms delay in cancellation is fine). No `Mutex` needed for a simple boolean flag.

- **`execution_running` guard**: A separate `AtomicBool` prevents starting a second execution while one is already running. The `start_execution` command checks this before proceeding and `cancel_execution` checks it before setting the cancel flag.

- **Prompt reuse from sldo-run**: The `build_execution_prompt()` function in `commands/run.rs` mirrors the one in `sldo-run/src/main.rs`. The prompt structure is intentionally duplicated rather than shared through `sldo-common`, because the Tauri version may diverge (e.g., shorter prompts, different retry context) and modifying `sldo-run` is out of scope.

- **Test helpers in E2E test file**: Since `sldo-tauri` is a binary crate and can't be imported as a library dependency, the E2E test file (`tests/e2e_tauri_m5.rs`) defines its own `sldo_tauri_test_helpers` module with mirror structs for the event types. These are tested for serialization parity but are not the canonical types.

- **MilestoneTracker `activeMilestone` prop**: Instead of creating a new component, the existing `MilestoneTracker` was extended with an optional `activeMilestone?: number` prop. When provided, the corresponding row gets a `milestone-tracker__row--active` CSS class. This keeps the component reusable across reviewing and executing phases.

## What Was Harder Than Expected

- **Nothing was significantly harder than expected for M5**. The patterns from M3 (event streaming) and M4 (runbook parsing) established clear conventions. The main work was implementing the execution loop and wiring it to events, which mapped directly from `sldo-run/src/main.rs`.

- **Keeping the E2E tests portable**: The `sldo-tauri` binary crate can't be imported by workspace-level integration tests. The solution of mirroring event types in a test-local module works but adds maintenance burden. A future improvement would be to extract event types into `sldo-common` or a shared types crate.

## Naming Conventions Established

- **Execution command module**: `commands/run.rs` — matches the CLI binary name `sldo-run`
- **Execution events**: Prefixed with their scope: `MilestoneStarted`, `ExecutionProgress`, `BuildTestResult`, `MilestoneCompleted`, `ExecutionComplete`
- **Tauri event names**: kebab-case matching the struct purpose: `milestone-started`, `execution-progress`, `build-test-result`, `milestone-completed`, `execution-complete`
- **Component**: `ExecutionView.tsx` — follows PascalCase convention from M2/M4
- **Hook**: `useExecution.ts` — follows `use<Feature>.ts` pattern from M3
- **E2E test files**: `tests/e2e_tauri_m5.rs` and `e2e/execution.e2e.test.tsx` — consistent with M3/M4

## Test Patterns That Worked Well

- **Mirror struct pattern for E2E tests**: Defining test-local structs that match the production types (with `Serialize`/`Deserialize`) allows testing serialization format without importing the binary crate.
- **56 frontend tests total**: 9 ExecutionView BDD, 3 Execution E2E, plus all 44 pre-existing tests pass unchanged.
- **10 Rust E2E tests**: Covers runbook parsing, build detection, cancellation, and all 5 event type serialization.
- **7 Rust unit tests in commands/run.rs**: Prompt construction and verification command execution.
- **8 Rust unit tests in events.rs and state.rs**: New event and state BDD tests.

## What the Next Milestone Should Do Differently

- **M6 (Settings Panel)**: The `model` field in the execution loop is currently hardcoded to `"claude-opus-4.6"`. When M6 adds `AppSettings` persistence, the execution loop should read the model from managed state. The `build_execution_prompt` and `start_execution` should accept the model as a parameter rather than hardcoding it.

- **Provider trait integration**: The `start_execution` command directly calls `CopilotInvocation`. When M6 introduces the `Provider` trait, the execution loop should use the provider abstraction instead of direct invocation.

- **Shared event types crate**: Consider extracting event types into a shared types module (either in `sldo-common` or a new `sldo-types` crate) so integration tests can import them directly instead of mirroring.

- **CSS for active milestone**: The `milestone-tracker__row--active` class is added but no CSS styles exist for it yet. M6 or M8 should add a visual highlight (e.g., gold border or background).

## BDD Scenarios to Retroactively Add

- None needed for M1, M2, M3, or M4. All existing tests remain valid and pass.
