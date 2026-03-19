# Lessons Learned — voice-tx Milestone 5

## What changed

- Polished `VoiceTranscriber.tsx` — removed all inline styles in favor of CSS classes from `App.css` that use the app's design tokens (colors, spacing, radius, typography). Added `data-has-error` attribute for conditional error styling.
- Added `.voiceTranscriber*` CSS classes to `App.css` — page container, heading, description, controls, status, textarea, and error classes matching the app's dark theme, gold accents, and existing `.textarea`/`.button`/`.error` patterns.
- Updated `docs/ARCHITECTURE.md` — added Styling and Production Security Guidance subsections to the Voice Transcriber section. Added M4 and M5 E2E test entries to the test table. Updated total backend test count to 223.
- Updated `README.md` — added "Voice Transcriber (Standalone Page)" section with usage instructions, requirements, and production security guidance about not shipping shared API keys. Added troubleshooting entries for "No audio was captured" and missing Info.plist.
- Created `tests/e2e_voice_tx_m5.rs` with 3 E2E tests validating ARCHITECTURE.md, README.md, and workspace compilation.
- Fixed pre-existing flaky test "Buttons disabled during transcription" — guarded `resolveInvoke` cleanup against undefined when the async invoke hasn't been called yet.

## Design decisions and why

- **CSS classes over inline styles**: Inline styles bypass the app's design system (CSS custom properties, consistent radius/spacing tokens). Moving to CSS classes ensures the transcriber page looks native to the rest of the app and supports future theme changes.
- **`data-has-error` attribute for conditional styling**: Instead of JavaScript-computed inline style objects, the error div uses a data attribute that CSS selects on. This keeps styling in CSS and logic minimal in JSX.
- **Textarea matches `.textarea` pattern but uses its own class**: Used `.voiceTranscriberTextarea` with the same property values as `.textarea` rather than adding the `.textarea` class, since the transcriber textarea has different sizing (min-height) and the class name is more specific for future overrides.

## Mistakes made

- None significant. The milestone was straightforward polish and documentation.

## Root causes

- N/A

## What was harder than expected

- The pre-existing "Buttons disabled during transcription" test was flaky in full-suite runs due to the async `invoke` mock not being called before cleanup. This was documented in M4 lessons but manifested consistently during M5 validation.

## Naming conventions established

- CSS class prefix: `.voiceTranscriber*` — camelCase matching existing convention (`.conversationCard`, `.chatInput`, etc.)
- Data attribute: `data-has-error` — kebab-case for HTML data attributes
- E2E test file: `tests/e2e_voice_tx_m5.rs` — consistent with M1–M4 pattern

## Test patterns that worked well

- E2E tests reading documentation files and asserting on content (structural validation)
- Guarding cleanup functions with `if (resolveInvoke)` for tests using pending promise mocks

## Missing tests that should exist now

- No gaps for M5 scope. All BDD scenarios and E2E scenarios from the runbook are covered.

## Rules for the next milestone

- This is the final milestone. No subsequent milestones are planned.
- Future enhancements (live partial transcription, STT model/provider settings, OS keychain integration) are documented as deferred follow-ups in the completion summary.
