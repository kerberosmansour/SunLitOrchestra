# Lessons Learned — research Milestone 1

## What changed
- New `sldo-research` crate added to workspace with CLI skeleton
- `research_allow_flags()` and `research_deny_flags()` added to `sldo-common/toolflags.rs`
- E2E test infrastructure established at `tests/e2e_research_m1.rs`
- `.gitignore` updated with `.sldo-logs/` and `output/`

## Design decisions and why
- `prompt_file` is `Option<PathBuf>` (not required positional) — both file and `--prompt` are alternatives; clap handles parsing, run() validates mutual exclusion
- `--output` defaults to `output/research-dossier.md` relative to CWD — repo-dir is optional so we can't always anchor to it
- Default model set to `claude-opus-4-7` — current latest Opus; matches runbook step-by-step guidance
- `research_allow_flags()` includes WebFetch and WebSearch from day one — M5 will use them; no cost to include early

## Mistakes made
- E2E tests were run before `cargo build --workspace`, causing binary-not-found panics. `cargo test --workspace` builds test binaries but not bin targets.

## Root causes
- `cargo test` does not build `[[bin]]` targets as a prerequisite for running integration tests that reference the binary by path. Always run `cargo build --workspace` before E2E tests.

## What was harder than expected
- Nothing significant in M1 — it's scaffolding.

## Naming conventions established
- Crate: `sldo-research`, binary: `sldo-research`
- E2E test file: `tests/e2e_research_m1.rs`, test names prefixed with `test_`
- Log dir: `.sldo-logs/` (via `ensure_log_dir` from sldo-common)
- Lessons files: `docs/lessons/research-m<N>.md`
- Completion files: `docs/completion/research-m<N>.md`

## Test patterns that worked well
- Splitting unit tests (CLI parse correctness) from E2E tests (binary invocation)
- E2E tests that accept either exit 0 or a clear error (for tests dependent on `claude` being installed)
- Temp dir cleanup with `let _ = std::fs::remove_dir_all(&tmp)`

## Missing tests that should exist now
- None for M1 scope.

## Rules for the next milestone
- Always `cargo build --workspace` before running E2E tests that invoke the binary
- Keep `prompt.rs` functions pure (no I/O) to make unit testing straightforward
- Write BDD tests in the source file (`#[cfg(test)] mod tests`) before production code

## Template improvements suggested
- The runbook BDD table says default model is `claude-opus-4.6` but step-by-step says `claude-opus-4-7`. The step-by-step is authoritative. Update BDD tables when model names change.
