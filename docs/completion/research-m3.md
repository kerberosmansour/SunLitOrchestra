# Completion Summary — research Milestone 3

**Date:** 2026-04-19
**Status:** done

## What was delivered

- `crates/sldo-research/src/research.rs` (NEW) — `ResearchConfig` struct,
  `research_loop()` driver, internal `run_phase()` Claude invocation helper,
  `persist_scratch()` writer. Plus 5 unit tests covering struct field
  accessibility and the scratch-helper invariants.
- `crates/sldo-research/src/main.rs` — declared `mod research;`, added
  `const COOLDOWN_SECS: u64 = 5`, called `ensure_log_dir(&working_dir)?`,
  constructed a `ResearchConfig` from CLI args + canonicalised repo dir,
  invoked `research::research_loop(&cfg)?`, and surfaced
  `info("Research accumulated N bytes of findings")` after the loop.
- `crates/sldo-research/src/prompt.rs` — removed `#[allow(dead_code)]` from
  `build_deepening_prompt` and `build_repo_context_prompt` (both now wired).
- `tests/e2e_research_m3.rs` (NEW, 9 tests) — registered under `[[test]]`
  in workspace root `Cargo.toml`. Uses the PATH-shim pattern (drop a stub
  `claude` script in a tempdir, prepend to child PATH) to exercise the
  real research loop without invoking the live Claude API.
- `tests/e2e_research_m2.rs` — added `.env(...)` PATH-clear to all 3
  tests so they continue to pass with the M3 loop wired in. See
  `docs/lessons/research-m3.md` for the rule-conflict rationale.
- `docs/ARCHITECTURE.md` — added `### sldo-research — Research Loop (M3)`
  subsection and updated the test-counts table (`E2E research M1/M2/M3`).

## Tests

- `cargo test -p sldo-research` → **30 passed** (was 25 at M2; +5 new
  research-module unit tests).
- `cargo test --test e2e_research_m3 -- --test-threads=1` → **9 passed**
  in 11.47 s.
- `cargo test --test e2e_research_m2 -- --test-threads=1` → **3 passed**
  in 0.02 s (regression — was previously 400+ s when claude was on PATH).
- `cargo test --test e2e_research_m1 -- --test-threads=1` → **6 passed**
  (regression).
- Full workspace `cargo test --workspace` is green except for the
  pre-existing `e2e_tauri_m1::frontend_dist_exists_after_build` esbuild
  failure documented in the runbook background. That failure is unrelated
  to M3 and is out of scope per the milestone-isolation rule.

## Build & lint

- `cargo build --workspace` → clean.
- `cargo build -p sldo-research` → clean (used to rebuild before smoke
  tests).
- `cargo fmt -p sldo-research` + `rustfmt tests/e2e_research_m3.rs` +
  `rustfmt tests/e2e_research_m2.rs` → clean.
- `cargo clippy -p sldo-research --tests -- -D warnings` → clean.
- `cargo clippy --test e2e_research_m3 -- -D warnings` → clean.
- Workspace-wide `cargo fmt --all` and `cargo clippy --workspace` were
  deliberately NOT run — pre-existing issues in non-research crates are
  out of M3's scope per the M2 lessons file's explicit guidance.

## Smoke tests

- `target/debug/sldo-research --help` → exit 0; `--prompt`, `--repo-dir`,
  `--output`, `--model`, `--max-iterations`, `--max-searches` all listed.
- `target/debug/sldo-plan --help` → exit 0 (regression).
- `target/debug/sldo-run --help` → exit 0 (regression).
- `git status` is clean after the test runs (the test suite cleans up its
  own tempdirs; the `.sldo-logs/` directory in the repo root is unchanged).

## Out of scope / handed off

- Dossier extraction and validation land in M4. M4 should consume the
  `.research-scratch-iter-N.md` files written by M3.
- Web-search phase prompts and invocation land in M5. The
  `--max-searches` CLI flag is still parsed but unused; M5 wires it.
- Synthesis prompt and final dossier emission land in M6.
- Removal of any remaining `#[allow(dead_code)]` markers in `prompt.rs`
  is complete — both deepening and repo-context constructors are now
  wired and exercised by the E2E suite.
