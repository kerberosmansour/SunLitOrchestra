# Lessons Learned — Tauri Desktop Milestone 1: Workspace Scaffolding & Shell App

## Design Decisions

- **Tauri v2 with React 18 + Vite**: Chose Tauri v2 (`tauri = "2"`) with React 18 and Vite 6. This matches the runbook's target architecture and provides a modern, fast dev experience with HMR.
- **Minimal main.rs**: The Tauri entry point uses `tauri::Builder::default()` with no commands registered yet. Commands will be added in subsequent milestones.
- **Copy design tokens, don't modify**: `docs/App.css` was copied verbatim to `crates/sldo-tauri/ui/src/App.css`. The CSS has a minor lint warning (extra closing brace) but works correctly. Future milestones should fix it when extending styles.
- **RGBA icons from JPEG**: The JPEG logo was converted to RGBA PNG format at multiple sizes using Python PIL. Tauri v2's `generate_context!` macro requires RGBA PNGs — RGB-only PNGs cause a compile-time panic. The `.icns` file is actually a PNG (Tauri accepts this).
- **React JSX transform**: With `tsconfig.json` set to `"jsx": "react-jsx"` and `noUnusedLocals: true`, explicit `import React from "react"` is not needed and would cause a TS error. Use named imports (`{ StrictMode }`, `{ createRoot }`) instead.

## What Was Harder Than Expected

- **Tauri icon requirements**: The `generate_context!()` macro validates icons at compile time. It requires RGBA PNGs — JPEG-sourced RGB PNGs fail with a cryptic "icon is not RGBA" panic. The `cargo tauri icon` command wouldn't run without interactive permission, so we generated icons manually with Python PIL.
- **Permission issues with sips/cargo-tauri**: macOS security prevented both `sips` and `cargo tauri icon` from running in the automated context. Python PIL was the reliable fallback for image processing.

## Naming Conventions Established

- **Tauri crate**: `sldo-tauri` at `crates/sldo-tauri/`
- **Frontend directory**: `crates/sldo-tauri/ui/` (not `frontend/` or `web/`)
- **E2E test file**: `tests/e2e_tauri_m1.rs` following the existing `e2e_<prefix>_m<N>.rs` pattern
- **Frontend package name**: `sldo-tauri-ui` in package.json
- **App identifier**: `com.sunlit.orchestrate`
- **Window title**: "SunLitOrchestrate"
- **Default window size**: 1200×800

## Test Patterns That Worked Well

- **Process-based E2E tests for build verification**: Running `cargo build --workspace`, `cargo check -p sldo-tauri`, and `npm run build` as subprocess commands in tests ensures the full toolchain works. These are slow but high-fidelity.
- **File existence tests for scaffolding**: Checking that `App.css` contains specific tokens (`--gold`, `--bg`) and that `sunlit.jpeg` starts with JPEG magic bytes provides fast, deterministic validation of the copy steps.
- **Regression test loop**: Iterating over all pre-existing E2E test names and running each one confirms no regressions without hardcoding expected counts.

## What the Next Milestone Should Do Differently

- **M2 (Chatbot UI)** will add React components. Consider installing a test runner (Vitest) as a devDependency now if not already planned, since M2 BDD scenarios will need frontend tests.
- **vitest** should be added to `package.json` devDependencies in M2 for component testing.
- The CSS warning about an extra `}` in App.css should be investigated and fixed when M2 extends the styles.
- Consider adding `@testing-library/react` for component-level BDD tests in M2.

## BDD Scenarios to Retroactively Add

- None needed for earlier milestones. The Rust CLI milestones (M1-M5) remain complete and unaffected.
