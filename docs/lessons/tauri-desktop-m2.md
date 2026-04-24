# Lessons Learned — Tauri Desktop Milestone 2: Chatbot UI — Prompt Screen & Conversation Layout

## Design Decisions

- **Vitest + Testing Library for frontend BDD tests**: Added `vitest`, `@testing-library/react`, `@testing-library/jest-dom`, `@testing-library/user-event`, and `jsdom` as devDependencies. Vitest integrates seamlessly with the existing Vite config via the `/// <reference types="vitest" />` directive.
- **Test files excluded from tsc build**: Added `exclude` array to `tsconfig.json` to prevent `tsc` (used in the `build` script) from type-checking test files. Test files use vitest globals which aren't part of the production build. Vitest handles its own type resolution.
- **CSS reuses existing design tokens**: All new components use classes already defined in `App.css` from the `docs/App.css` design source — `agentPage--empty`, `agentWelcome`, `conversationPanel`, `sidebar*`, `promptChip`, `textarea`, `button`, etc. Only one new CSS block was added for `.chatInputWrapper` layout.
- **Mock assistant responses**: The App component generates a mock assistant message immediately after user submission. This provides visual feedback in the conversation view without any backend integration (deferred to M3).
- **Component prop design**: Components receive callbacks (`onSubmit`, `onNewSession`, `onSelectSettings`) rather than managing global state. This keeps components testable in isolation and ready for state management if needed later.
- **ChatInput `initialValue` + `key` pattern**: HomeScreen uses `key={inputValue}` to force ChatInput to re-mount when a prompt chip is clicked, ensuring the textarea value updates. This is cleaner than managing controlled state across parent/child.

## What Was Harder Than Expected

- **jsdom lacks `scrollIntoView`**: The `Element.scrollIntoView()` method is not implemented in jsdom. ConversationView needed a guard (`typeof messagesEndRef.current.scrollIntoView === "function"`) to avoid runtime errors in tests while still working in the real browser.
- **tsc checking test files**: The `npm run build` script runs `tsc && vite build`. By default, tsc includes all files in `src/`, including tests. Tests import `vitest` globals which aren't available in the production tsconfig context, causing build failures. The fix was adding `exclude` patterns to `tsconfig.json`.

## Naming Conventions Established

- **Component files**: PascalCase in `src/components/` — `Sidebar.tsx`, `ChatInput.tsx`, `ConversationView.tsx`, `HomeScreen.tsx`
- **Component test files**: Co-located as `<Component>.test.tsx` — `Sidebar.test.tsx`, `ChatInput.test.tsx`, etc.
- **E2E test files**: `src/e2e/<feature>.e2e.test.tsx` — `chatui.e2e.test.tsx`
- **Type definitions**: `src/types/index.ts` — exports `AppPhase`, `Message`, `Session`, `MessageRole`
- **Test setup**: `src/test-setup.ts` — imports `@testing-library/jest-dom/vitest`
- **CSS class names**: Follow existing camelCase convention from `App.css` (`chatInputWrapper`, `chatInputSubmit`)

## Test Patterns That Worked Well

- **Testing Library with user-event**: `userEvent.setup()` + `user.type()` / `user.click()` / `user.keyboard()` provided realistic user interaction simulation. The `{enter}` and `{Shift>}{Enter}{/Shift}` key sequences work naturally.
- **BDD Given/When/Then comments**: Structuring each test with GWT comments made the intent clear and matched the runbook's acceptance scenarios directly.
- **Component isolation tests + App integration tests**: Testing each component with mock callbacks verified contracts, while App.test.tsx verified the full state machine works together.
- **22 total tests**: 4 ChatInput, 3 HomeScreen, 4 ConversationView, 4 Sidebar, 3 App integration, 4 E2E runtime validation.

## What the Next Milestone Should Do Differently

- **M3 (Planning Backend)** will add Tauri commands. The frontend will need to call `invoke()` from `@tauri-apps/api/core`. Consider mocking `@tauri-apps/api` in tests.
- **Streaming events**: M3 introduces Tauri event streaming. The `useStreamingEvents` hook should be testable by mocking `listen()` from `@tauri-apps/api/event`.
- **State management**: If M3 adds significant state complexity, consider whether React Context or a lightweight state library is needed, or if prop drilling remains sufficient.
- **The extra `}` in App.css** still produces an esbuild warning during `vite build`. It's cosmetic and doesn't affect functionality, but should be cleaned up when CSS is next modified.

## BDD Scenarios to Retroactively Add

- None needed for M1. The scaffolding milestone's tests remain valid and all pass.
