# Tauri Desktop App — Requirements

## Summary

Design and build a Tauri desktop app that sits on top of the existing `sldo-plan` and `sldo-run` CLI workflow. The app wraps the two Rust CLI binaries with a modern chatbot-style interface.

## Existing CLI Tools

The workspace already contains two fully functional Rust CLI binaries:

1. **`sldo-plan`** — Takes a prompt file and repo directory, uses GitHub Copilot CLI to generate a milestone-based Markdown runbook.
2. **`sldo-run`** — Takes a runbook and repo directory, drives Copilot CLI through each milestone, verifying builds and tests after each.

These binaries live in `crates/sldo-plan/` and `crates/sldo-run/`. There is also a shared library in `crates/sldo-common/`.

## What to Build

A Tauri desktop app with a TypeScript + React frontend that provides a modern GUI around the existing planning and execution CLI workflow.

## UX & Design

### Chatbot-Style Interface
- Take inspiration from OpenAI ChatGPT, Claude, and other modern chatbot interfaces — but do not just copy them.
- The initial screen should feel like a chatbot home screen with the prompt input centered in the middle.
- Once the user submits a prompt, the interface should transition into a conversational workspace where the input moves to the bottom and conversation, updates, plan, and execution logs appear above it.
- The app should feel clean, focused, responsive, and built for long-running planning/execution workflows.

### Existing CSS & Assets
- An existing CSS file is provided at `docs/App.css` — use it as the design foundation.
- A logo image is available at `docs/sunlit.jpeg` — integrate it into the app branding (sidebar, header, etc.).

### Navigation
The interface should make it easy to move between:
- Entering a prompt
- Reviewing/editing a plan
- Executing a plan
- Monitoring progress and updates

## Core Workflow

1. The user enters a prompt describing what they want to build.
2. The app generates a proposed plan using the `sldo-plan` CLI (or equivalent Tauri command).
3. That plan is rendered as editable Markdown.
4. The user can edit the Markdown plan directly.
5. Once ready, the user can ask the app to execute the plan.
6. The `sldo-run` CLI (or equivalent Tauri command) then runs the plan and builds the code.
7. During both planning and execution, the user receives constant live streaming updates from the agent.

## Planning Requirements

- The generated plan must be shown as editable Markdown.
- The plan should be easy to revise before execution.
- The app should stream progress updates while the plan is being created.
- After the first plan is complete, the user should have a chance to review it, edit it again, and then either finalize or execute it.

## Execution Requirements

- Once execution starts, progress should be shown step by step, milestone by milestone, and phase by phase.
- The UI should show continuous updates from the agent while code is being written and tasks are being completed.
- The user should be able to follow the execution in a clear, structured way.
- The execution experience should feel like watching an autonomous coding agent work in real time.
- Show milestone tracker status, build/test verification results, and retry context.

## Agent/Provider Architecture

- The system should be agent-provider-independent where possible.
- Start with GitHub Copilot support first (the existing `sldo-common/src/copilot.rs` already wraps Copilot CLI).
- Design it so it can later support GitHub Copilot, Claude Code, and other coding agents/providers.
- Settings for:
  - Selected provider/agent
  - Selected model (default: claude-opus-4.6)
  - Allowed tools (maps to the existing `toolflags.rs` allow/deny flags)
  - Execution behavior (max attempts, cooldown, etc.)
  - Other agent-level configuration

## Voice Support

- The app should support voice input so the user can speak instructions instead of typing them.
- Voice input should work naturally from the main chatbot prompt area.
- Use an abstraction layer so the system is not tightly coupled to OpenAI.
- OpenAI speech-to-text can be used initially, but the architecture should allow swapping in other providers later.
- A `.env` file with the `OPENAI_API_KEY` already exists in the workspace root.
- Use a Rust-friendly approach where appropriate (Tauri commands for the backend, TypeScript for the UI).

## Technical Architecture

### Tauri Setup
- Use Tauri v2 with a TypeScript + React frontend.
- The Tauri backend (Rust) should expose commands that wrap the existing Rust library (`sldo-common`) and CLI logic.
- Reuse the existing shared library modules (copilot invocation, runbook parsing, tool flags, detection, git safety, logging, etc.).
- The frontend communicates with the Rust backend via Tauri's IPC invoke/event system.

### Streaming & Live Updates
- Use Tauri events (emit from Rust → listen in React) to stream planning and execution progress to the frontend.
- The Rust backend should pipe stdout/stderr from `copilot` CLI child processes and emit lines as events.
- The frontend renders these events as a live-updating conversation/log stream.

### Markdown Editing
- Use a React Markdown editor component for editing the generated runbook/plan.
- The runbook should be saveable back to disk and re-parseable by the existing `runbook.rs` module.

### Settings Persistence
- Store settings (provider, model, tools, etc.) in Tauri's app data directory or a config file in the project.
- Load settings on startup and make them editable from a settings panel.

## File Organization

The Tauri app should be added as a new member of the existing Cargo workspace:
- `crates/sldo-tauri/` — Tauri Rust backend (src-tauri equivalent)
- `crates/sldo-tauri/ui/` — React + TypeScript frontend
- The existing crates (`sldo-common`, `sldo-plan`, `sldo-run`) remain unchanged.

## Constraints

- Do NOT break the existing CLI binaries or their tests.
- All existing `cargo test` and E2E tests must continue to pass.
- The Tauri app should depend on `sldo-common` as a library, reusing its modules.
- Keep the architecture modular so the app can evolve beyond a single provider.
