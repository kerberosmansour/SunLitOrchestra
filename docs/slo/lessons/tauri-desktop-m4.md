# Lessons Learned — Tauri Desktop Milestone 4: Markdown Plan Editor & Runbook Persistence

## Design Decisions

- **`read_runbook` and `save_runbook` as separate Tauri commands**: Rather than bundling read/save into a single command, they are separate for flexibility. `read_runbook` returns both raw content and parsed milestones; `save_runbook` writes to disk and re-validates, returning warnings.

- **`MilestoneRowDto` as a serializable DTO**: The `sldo_common::runbook::MilestoneRow` struct doesn't derive `Serialize`/`Deserialize`. Rather than modifying `sldo-common` (which would touch M1-M5 code), a `MilestoneRowDto` in `commands/plan.rs` implements `From<&MilestoneRow>` for clean conversion. The DTO uses `String` for status instead of the enum, matching the frontend's `MilestoneStatus` type.

- **Simple built-in Markdown preview**: Instead of adding a heavy dependency like `@uiw/react-md-editor` or `react-markdown`, a lightweight `simpleMarkdownToHtml()` function handles basic Markdown rendering (headers, tables, code blocks, horizontal rules). This avoids dependency bloat while providing adequate preview functionality for runbook content.

- **MarkdownEditor is a controlled component**: Content is passed as a prop and managed internally via `editContent` state. The `onSave` callback returns the current editor content. This makes testing straightforward — tests control what content is displayed.

- **MilestoneTracker uses CSS class conventions**: Status indicators use `milestone-status--done`, `milestone-status--in_progress`, `milestone-status--not_started` classes, following the BEM-like pattern established in M2/M3 (`streamLine--stdout`, etc.).

- **`transitionToReviewing` function in App.tsx**: The function that transitions from planning to reviewing phase is wired to a "Review Plan" button visible during the planning phase. In a real integration, this would be triggered by the `plan-complete` event via `usePlan()`.

- **`save_runbook` validates after write**: After writing content to disk, the command checks for required sections and parseable milestone table, returning warnings. This ensures the user sees validation feedback immediately after saving.

## What Was Harder Than Expected

- **TypeScript strict mode with unused variables**: `noUnusedLocals: true` in tsconfig caught `_runbookPath` (prefixed with underscore) but accepted it. The `handleReviewRunbook` function initially failed because it wasn't referenced in JSX. Solution was renaming to `transitionToReviewing` and wiring it to a "Review Plan" button.

- **Validation warning text matching in tests**: The `⚠️` emoji prefix in warning messages caused `getByText("exact string")` to fail because the rendered DOM splits the text across elements. Switched to regex matchers (`/Missing section: Milestone Tracker/`) for resilience.

- **Not modifying `sldo-common`**: The milestone spec suggested extracting `validate_runbook` to `sldo-common`, but this would modify M1-M5 code. Instead, `save_runbook` reimplements validation logic (checking required sections, milestone table, file size) without touching `sldo-common`.

## Naming Conventions Established

- **Component files**: `MarkdownEditor.tsx`, `MilestoneTracker.tsx` — PascalCase, matching existing convention
- **Component test files**: `MarkdownEditor.test.tsx`, `MilestoneTracker.test.tsx` — same directory as component
- **E2E test files**: `editor.e2e.test.tsx` in `e2e/` — matches existing `planning.e2e.test.tsx` pattern
- **Rust E2E**: `tests/e2e_tauri_m4.rs` — follows `e2e_tauri_m<N>.rs` pattern
- **DTO types**: `<Name>Dto` suffix for serializable transfer objects (`MilestoneRowDto`)
- **TypeScript types**: `MilestoneRow`, `MilestoneStatus`, `RunbookData` in `types/index.ts`
- **CSS classes**: `markdown-editor__*` for editor, `milestone-tracker__*` and `milestone-status--*` for tracker

## Test Patterns That Worked Well

- **Regex text matchers**: Using `/pattern/` instead of exact strings for text that includes emoji or spans multiple elements
- **`data-testid` for structural assertions**: `markdown-preview` and `validation-warnings` testids make mode-toggle and warning-presence tests reliable
- **CSS class-based status assertions**: Querying `.milestone-status--done` etc. to verify color-coded indicators without depending on text content
- **44 frontend tests total**: 7 MarkdownEditor, 5 MilestoneTracker, 3 Editor E2E, plus all 29 pre-existing tests pass unchanged
- **3 Rust E2E tests**: `read_runbook_parses_real_file`, `save_and_reparse_roundtrip`, `save_invalid_content_returns_warnings` — all work without Tauri runtime

## What the Next Milestone Should Do Differently

- **M5 (Execution Backend)**: The `transitionToReviewing` function in App.tsx is currently wired to a mock "Review Plan" button. When integrating with the real `usePlan()` hook, the transition should be triggered by the `plan-complete` event with actual runbook data loaded via `loadRunbook()`.

- **Progress bar CSS**: The `milestone-tracker__progress-bar` and `milestone-tracker__progress-fill` classes are rendered but don't have CSS styles yet. M5 or M8 should add visual styles for the progress bar.

- **Markdown preview enhancement**: The built-in `simpleMarkdownToHtml` is minimal. If richer preview is needed (syntax highlighting, proper table rendering), consider adding `react-markdown` as a dependency in a later milestone.

- **Auto-save behavior**: The editor calls `onSave` on blur, which may be surprising. Consider debounced auto-save or explicit save-only behavior based on user feedback.

## BDD Scenarios to Retroactively Add

- None needed for M1, M2, or M3. All existing tests remain valid and pass.
