# Completion Summary — research Milestone 2

**Date:** 2026-04-19
**Status:** done

## What was delivered

- `crates/sldo-research/src/prompt.rs` (NEW) — three pure prompt constructors,
  13 phase-specific section-name constants, plus a deepening truncation helper.
- `crates/sldo-research/src/main.rs` — added `mod prompt;`, captured the
  canonicalised repo dir into a local, built the exploration prompt after
  pre-flight, and surfaced its byte count + first line via `info(…)`.
- `tests/e2e_research_m2.rs` (NEW, 3 tests) — registered under `[[test]]` in
  workspace root `Cargo.toml`.
- `docs/ARCHITECTURE.md` — added a `## sldo-research CLI` section with an
  `### sldo-research — Research Prompt Builder (M2)` subsection listing the
  builder functions and section header contracts.

## Tests

- `cargo test -p sldo-research` → **25 passed** (was 13 at M1; +12 new prompt
  unit/BDD tests).
- `cargo test --test e2e_research_m2 -- --test-threads=1` → **3 passed**.
- `cargo test --test e2e_research_m1 -- --test-threads=1` → **6 passed**
  (regression).
- Full workspace `cargo test --workspace` is green except for the pre-existing
  `e2e_tauri_m1::frontend_dist_exists_after_build` esbuild failure documented
  in the runbook background.

## Build & lint

- `cargo build --workspace` → clean (only the pre-existing 2 Tauri dead-code
  warnings).
- `cargo fmt --check -p sldo-research` → clean.
- `cargo clippy -p sldo-research --all-targets -- -D warnings` → clean.
- `cargo clippy --test e2e_research_m2 -- -D warnings` → clean.
- Workspace-wide `cargo fmt --all --check` and `cargo clippy --workspace`
  failures are pre-existing baseline issues in non-M2 crates and are explicitly
  out of M2's scope per the "Do NOT touch code or tests belonging to other
  milestones" hard rule.

## Smoke test

```
$ target/debug/sldo-research --prompt "evaluate async runtimes"
…
[…] ℹ  Exploration prompt: 1786 bytes
[…] ℹ  Exploration prompt first line: You are an expert research agent. …
[…] ℹ  Research loop pending (milestone 3).
```

`git status` is clean after the smoke test (no scratch files written under
the CWD or anywhere else).

## Out of scope / handed off

- The Claude Code invocation and iterative research loop land in M3.
- Web-search phase prompts land in M5.
- Synthesis prompt lands in M6.
- Removal of the `#[allow(dead_code)]` markers on `build_deepening_prompt` and
  `build_repo_context_prompt` happens in M3 when they are first wired in.
