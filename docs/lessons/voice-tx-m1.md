# Lessons Learned — voice-tx Milestone 1

## What changed

- Added `"transcriber"` to the `AppPhase` union type in `types/index.ts`.
- Created `VoiceTranscriber.tsx` — a standalone voice transcription page skeleton with heading, description, disabled start/stop buttons, empty transcript textarea, and error display area.
- Updated `App.tsx` to route the `"transcriber"` phase to `VoiceTranscriber` and pass `onSelectTranscriber` to Sidebar.
- Updated `Sidebar.tsx` to accept an optional `onSelectTranscriber` prop and render a "Transcriber" navigation button.
- Created `VoiceTranscriber.test.tsx` with 6 BDD tests covering all UI elements.
- Added 2 sidebar BDD tests for transcriber button visibility and click handling.
- Created `tests/e2e_voice_tx_m1.rs` with 2 E2E tests (workspace compilation, AppPhase type validation).

## Design decisions and why

- **Optional `onSelectTranscriber` prop**: Made the Sidebar prop optional (`onSelectTranscriber?: () => void`) to preserve backward compatibility with all existing Sidebar tests that don't pass this prop. This prevents breaking existing tests while adding new functionality.
- **Disabled buttons**: Start/stop buttons are rendered with `disabled` attribute and no click handlers since recording logic is deferred to M3.
- **`data-testid` for error area**: Used `data-testid="transcriber-error"` for the empty error div since it has no text content to match by role or text.
- **Separate from VoiceButton**: VoiceTranscriber does not import or reference `useVoice` or `VoiceButton`, maintaining full isolation as required.

## Mistakes made

- None significant. The implementation was straightforward since it's a static UI component.

## Root causes

- N/A

## What was harder than expected

- Nothing. The milestone was well-scoped and the existing patterns were easy to follow.

## Naming conventions established

- Component: `VoiceTranscriber.tsx` — PascalCase, matches existing convention
- Test: `VoiceTranscriber.test.tsx` — co-located BDD tests
- E2E: `tests/e2e_voice_tx_m1.rs` — follows `e2e_voice_tx_m<N>.rs` pattern from runbook
- CSS class prefix: `voiceTranscriber` — camelCase matching existing component patterns

## Test patterns that worked well

- `getByRole("heading", { name: /.../ })` for heading assertions (per M8 lessons)
- `getByRole("button", { name: /.../ })` for button assertions
- `getByRole("textbox")` for textarea assertions
- `data-testid` for elements with no visible text content
- Optional prop pattern preserves backward compatibility of existing tests

## Missing tests that should exist now

- No gaps identified for M1 scope. All BDD scenarios from the runbook are covered.

## Rules for the next milestone

- M2 adds Rust backend command — must not touch any frontend files from M1.
- The `VoiceTranscriber` component should remain static until M3 wires recording hooks.
- Keep the `onSelectTranscriber` prop optional on Sidebar to avoid breaking tests.

## Template improvements suggested

- None.
