# Completion Summary — tla-sha Milestone 2

## Goal completed
- `sldo-tla-sha --verify` re-fetches every populated section in `tools.toml`, computes its SHA-256, and compares against the stored value. Exit 0 on all-match; non-zero on any mismatch. Refuses to run when any section is still `UNSET` (tested in M1). The core verification logic lives in the library (`verify_all`) and is unit-tested without network I/O via injected fetchers.

## Files added
- `docs/lessons/tla-sha-m2.md`
- `docs/completion/tla-sha-m2.md`

## Files changed
- `crates/sldo-tla-sha/src/lib.rs` — added `VerifyOutcome` struct and `verify_all<F>` function; 4 new unit tests.
- `crates/sldo-tla-sha/src/main.rs` — refactored `run_verify` to delegate to `verify_all`, passing the production `fetch_and_hash` through a closure that prints progress.

## Tests added
- 4 library unit tests: `verify_all_all_match_returns_pass_for_every_section`, `verify_all_one_mismatch_flags_only_the_bad_one`, `verify_all_propagates_fetch_errors`, `verify_all_empty_when_no_populated_sections`.

## Runtime validations added
- None beyond M1's integration tests — M2 is a pure refactor + library addition, exercised end-to-end by M1's `verify_refuses_when_any_unset` E2E test.

## Compatibility checks performed
- `cargo test -p sldo-tla-sha` — 14 lib + 7 E2E tests green (was 10 lib + 7 E2E in M1).
- `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sldo-tla-sha` — full green.
- M1's default (patch-printing) behavior unchanged.

## Documentation updated
- None outside the runbook tracker.

## .gitignore changes
- None.

## Test artifact cleanup verified
- `git status` clean after test run.

## Deferred follow-ups
- E2E test for `--verify` against a mismatch (requires a tools.toml with a known-bad SHA; skip for now).
- Upstream `.sha256` sibling-file cross-check, if GitHub ever starts publishing them.

## Known non-blocking limitations
- `--verify` is network-bound; it cannot run offline. Not a regression from the goal.
- Progress output in the injected closure is a cosmetic coupling — verify_all technically could be called from any context, but the closure printer assumes a TTY.
