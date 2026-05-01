# Completion Summary — slo-sp Milestone 1

## Goal completed
- `sldo-install` Rust binary can install, uninstall, show status for, and verify a skill pack on disk. Works against `~/.claude/skills/` (global) or `./.claude/skills/` (local). Idempotent. Safe re: user modifications. Dry-run mode. `--force` to replace existing symlinks.

## Files changed
- `Cargo.toml` — added `crates/sldo-install` to workspace members.
- `.gitignore` — added `.claude/`, `.sldo/`.
- `docs/slo/completed/RUNBOOK-SKILL-PACK.md` — updated baseline command (per-crate, not `--workspace`).

## Files added
- `crates/sldo-install/Cargo.toml`
- `crates/sldo-install/src/main.rs`
- `crates/sldo-install/src/install.rs`
- `crates/sldo-install/src/manifest.rs`
- `crates/sldo-install/src/paths.rs`
- `crates/sldo-install/tests/install_e2e.rs`
- `skills/README.md`

## Tests added
- 8 unit tests (manifest roundtrip, upsert/remove semantics, path resolution, skill discovery).
- 10 E2E tests (install/uninstall cycle, idempotency, --force behavior, --dry-run, --local, missing dir error, empty dir, non-skill dirs skipped, --help, user-modified symlink preservation).

## Runtime validations added
- `tests/install_e2e.rs::test_full_install_uninstall_cycle` — boots the binary against a tempdir HOME, confirms symlinks and manifest appear, then disappear.
- `tests/install_e2e.rs::test_local_install_into_project` — confirms `--local` targets `./.claude/skills/` and leaves `~/.sldo/install.toml` absent.

## Compatibility checks performed
- `sldo-plan --help` exits 0 ✓
- `sldo-run --help` exits 0 ✓
- `sldo-research --help` exits 0 ✓
- `sldo-common` public API unchanged (verified by all library tests still passing)
- `crates/sldo-tauri/` untouched (no files in that path in the diff)

## Documentation updated
- `skills/README.md` — NEW.
- `docs/slo/completed/RUNBOOK-SKILL-PACK.md` — baseline command corrected.
- Root `README.md` — NOT updated yet; deferred to a single update near M9 to avoid churn.

## .gitignore changes
- Added: `.claude/`, `.sldo/`
- Nothing removed.

## Test artifact cleanup verified
- `git status` shows no untracked test artifacts after the full `cargo test` run.
- All 10 E2E tests use `tempfile::TempDir`, so nothing leaks out of the test process.

## Deferred follow-ups
- Windows CI for the symlink path.
- Manifest-lock tests for concurrent installer runs.
- Root `README.md` update (deferred to M9 bulk update).

## Known non-blocking limitations
- `--host` flag exists but only accepts `claude-code`. Other hosts are a future milestone.
- No auto-update logic; users re-run `sldo-install` manually after pulling new skills.
- Baseline is per-crate, not `--workspace`, because the parked Tauri tests leave the workspace red. When Tauri is un-parked (if ever), restore the `--workspace` baseline.
