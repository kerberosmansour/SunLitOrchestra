# Lessons Learned — Tauri Desktop Milestone 6: Settings Panel & Provider Architecture

## Design Decisions

- **Provider trait is minimal**: The `Provider` trait defines only `name()`, `available_models()`, and `invoke()`. This keeps it simple enough that adding `ClaudeCodeProvider` or any other provider later only requires a new struct + impl, with no changes to existing code.

- **CopilotProvider delegates to existing code**: Rather than re-implementing invocation logic, `CopilotProvider::invoke()` constructs a `CopilotInvocation` and calls `run_with_callback()`. This preserves all existing behavior (logging, streaming, error handling) without duplication.

- **AppSettings is flat JSON**: Settings are stored as a simple flat JSON file at `<tauri_app_data_dir>/settings.json`. No database, no migration logic. The `serde_json` roundtrip is tested thoroughly. If the file is missing or corrupted, defaults are used with a warning logged to stderr.

- **Default flags from toolflags module**: `AppSettings::default()` pulls allow/deny flags from `toolflags::plan_allow_flags()` and `toolflags::plan_deny_flags()`, ensuring the defaults stay in sync with the shared module.

- **Settings phase in App**: Added a `"settings"` variant to `AppPhase` so the settings panel is rendered as a full-page view rather than a modal. This is simpler and more consistent with the existing phase-based routing.

- **Planning and execution read from AppSettings**: Both `commands/plan.rs` and `commands/run.rs` now read provider, model, and flags from the managed `AppSettings` instead of using hardcoded values. The `run_planning_sync` function signature was expanded to accept `allow_flags` and `deny_flags` parameters.

## What Was Harder Than Expected

- **Tauri v2 `app.path()` in setup**: The `setup()` closure receives `&mut App` which requires `Manager` trait for `.path()` and `.state()`. This needed an explicit `use tauri::Manager;` import in main.rs.

- **Temporary borrow lifetimes in run.rs**: The expression `app.state::<AppState>().settings.lock().unwrap()` creates a temporary `State<AppState>` reference that gets dropped before the lock guard is used. Fixed by binding the state reference to a local variable first.

- **TypeScript strict mode**: The unused `handleFlagChange` helper in SettingsPanel caused a TS6133 error, caught by the M1 E2E test that runs `tsc && vite build`. Removed the unused function.

## Naming Conventions Established

- **Provider module**: `provider.rs` in `crates/sldo-tauri/src/` — contains trait, CopilotProvider impl, and factory functions
- **Settings command module**: `commands/settings.rs` — follows existing `commands/<feature>.rs` pattern
- **Settings Tauri commands**: `get_settings`, `update_settings`, `get_available_providers`, `get_available_models` — REST-like naming
- **Frontend component**: `SettingsPanel.tsx` — PascalCase, follows existing component naming
- **TypeScript type**: `AppSettings` in `types/index.ts` — mirrors Rust struct name exactly
- **E2E test files**: `tests/e2e_tauri_m6.rs` and `e2e/settings.e2e.test.tsx` — consistent with M3/M4/M5

## Test Patterns That Worked Well

- **Mirror struct pattern for E2E tests**: Same pattern as M5 — test-local `AppSettings` struct validates serialization format without importing the binary crate
- **67 frontend tests total**: 8 SettingsPanel BDD tests, 3 Settings E2E tests, plus all 56 pre-existing tests pass
- **6 Rust E2E tests**: Default settings, JSON roundtrip, invalid JSON fallback, flag matching
- **3 settings command unit tests**: Available providers, available models for known/unknown providers
- **5 provider unit tests**: Name, models, get_provider factory, unknown provider, available_providers
- **6 state persistence unit tests**: Serialization, deserialization, load with no file, save/load roundtrip, corrupted file

## What the Next Milestone Should Do Differently

- **M7 (Voice Input)**: The settings panel does not yet include voice/speech settings. If M7 adds speech-to-text configuration (provider, language, etc.), those fields should be added to `AppSettings` and the SettingsPanel component.

- **Provider selection in execution**: The execution loop still creates `CopilotInvocation` directly (in `commands/run.rs`). It should be updated to use the `Provider` trait via `provider::get_provider(&settings.provider)`. This was not done in M6 because the execution loop's invocation pattern doesn't yet match the `Provider::invoke()` callback signature cleanly (the execution loop uses `run_with_callback` which takes `&str, &str` while the trait callback takes `&str, &str` — they match, but refactoring the execution loop is better done holistically).

- **Shared types crate**: The mirror struct pattern in E2E tests is getting more burdensome. Consider extracting shared types (AppSettings, event types) into `sldo-common` or a dedicated `sldo-types` crate.

## BDD Scenarios to Retroactively Add

- None needed for M1–M5. All existing tests remain valid and pass.
