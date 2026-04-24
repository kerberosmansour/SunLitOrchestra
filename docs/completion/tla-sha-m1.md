# Completion Summary — tla-sha Milestone 1

## Goal completed
- `sldo-tla-sha` reads `skills/slo-tla/tools.toml`, identifies `UNSET` entries, streams each URL while enforcing an allow-list on the final host (critique f1) and a max-response-size cap (critique f2), and prints a TOML patch to stdout. Humans commit the patch.

## Files added
- `crates/sldo-tla-sha/Cargo.toml`
- `crates/sldo-tla-sha/src/lib.rs`
- `crates/sldo-tla-sha/src/main.rs`
- `crates/sldo-tla-sha/tests/e2e.rs`

## Files changed
- `Cargo.toml` — added `crates/sldo-tla-sha` to workspace members.

## Tests added
- 10 library unit tests (hash known-answers, oversize cap, allow-list correctness including lookalike-host rejections, TOML parsing, patch formatting, URL parsing).
- 7 integration tests (dry-run shows plan without network, missing file clean error, skips populated sections, nothing-to-do case, --verify refuses UNSET, --help shape, malformed TOML).

## Runtime validations added
- Dry-run path does NOT hit the network (verified by integration test running against a tempfile tools.toml — no network mocks needed).
- `--verify` refuses when any section is UNSET (verified by integration test).

## Compatibility checks performed
- `cargo test -p sldo-install` — still passes (no skill-pack regressions).
- `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sldo-tla-sha` — all green.
- `skills/slo-tla/tools.toml` schema unchanged.

## Documentation updated
- None in this milestone (CLAUDE.md reference comes in M2 when the binary gets its public story).

## .gitignore changes
- None.

## Test artifact cleanup verified
- All tests use `tempfile::TempDir`; `git status` is clean after the suite.

## Deferred follow-ups
- Live-network smoke test gated behind an env flag.
- 10-redirect-limit test.
- CLAUDE.md reference (bundled into M2).

## Known non-blocking limitations
- The size cap is a fixed 500 MB; not configurable per-section in M1. If Apalache ever ships a release near that size, we revisit.
- The binary depends on reqwest's blocking API, which doesn't support HTTP/3. Not relevant at today's scale.
