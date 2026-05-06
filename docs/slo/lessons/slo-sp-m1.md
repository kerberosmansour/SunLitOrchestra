# Lessons Learned — slo-sp Milestone 1

## What changed
- New `crates/sldo-install` binary with `install / uninstall / status / verify` subcommands.
- New `skills/` directory at repo root as the source of truth for all skills.
- `skills/README.md` documents layout and usage.
- `.gitignore` updated to exclude `.claude/` (user-local Claude Code state) and `.sldo/`.
- Baseline test command redefined to be per-crate (not `--workspace`), because the parked Tauri work leaves the full workspace suite red on macOS arm64.

## Design decisions and why
- **Symlinks, not copies.** Skills live in the repo and symlink into `~/.claude/skills/`. Rationale: zero sync cost, edits propagate immediately, uninstall is trivial. Copy-based installers need version tracking or invalidation logic; symlinks don't.
- **Manifest at `~/.sldo/install.toml`.** A separate file, not inside `~/.claude/`. Rationale: Claude Code owns `~/.claude/`; we don't want to collide with or pollute its state.
- **Uninstall preserves user-modified symlinks.** If someone replaces our symlink with their own, we leave it alone and warn. This mirrors how package managers handle `/etc` drop-ins.
- **Library crates baseline only.** The integration tests crate (`sunlit-orchestra-tests`) has Tauri and voice-tx e2e tests that depend on parked work. Rather than fix those or delete them, we baseline per-crate. Documented in the runbook so the next milestone knows the rule.

## Mistakes made
- First baseline attempt used `cargo test --workspace --exclude sldo-tauri -- --skip e2e_tauri`. The `--skip` flag filters by test-function name inside a binary, not by binary name. The tauri integration tests have function names like `frontend_dist_exists_after_build`, so `--skip e2e_tauri` didn't touch them. Moved to per-crate `-p` filtering.

## Root causes
- The `sunlit-orchestra-tests` root crate has `[[test]]` entries for every milestone of every prior runbook, including parked Tauri work. The failing test is purely environmental (esbuild arm64 binary missing in the Tauri UI's `node_modules/`), not a regression from this branch. But a red baseline is a red baseline.

## What was harder than expected
- Getting `cargo test` to skip specific test *binaries* (not test functions) cleanly. Cargo has `--exclude` for crates but no direct "exclude this binary in a keep-other-binaries crate" flag.

## Naming conventions established
- Skill directory name == installed skill name. `skills/slo-ideate/` → `~/.claude/skills/slo-ideate/`. No re-prefixing.
- First-party SLO skills use `slo-` prefix; third-party vendored skills do not (e.g., `get-api-docs` comes later in M10).
- Test file naming in this crate: `tests/install_e2e.rs` (integration), inline `#[cfg(test)] mod tests` (unit). Matches the rest of SLO.

## Test patterns that worked well
- `tempfile::TempDir` as a fake `$HOME`, with `Command::env("HOME", ...)`. Lets every test run hermetically, even in parallel, against a real binary invoked through the filesystem.
- `CARGO_BIN_EXE_sldo-install` env var to locate the binary from integration tests — cargo sets it automatically; no manual path logic.
- Plan → mutate structure: `plan()` returns a plan that `install()` then applies. Lets `--dry-run` reuse planning without duplicating logic.

## Missing tests that should exist now
- Windows symlink path (we currently `#[cfg(windows)]` the symlink call but don't CI-test Windows). Deferred until someone actually runs on Windows.
- Concurrent installer invocations. Unlikely in practice, but two sessions running `sldo-install` simultaneously could race on the manifest.
- A test that verifies the manifest remains stable across install/uninstall/install (schema_version persistence).

## Rules for the next milestone (M2 — /slo-ideate + /slo-retro)
- Do NOT touch `crates/sldo-install/` in M2. If an installer bug surfaces while testing skills, surface it separately as a deferred follow-up.
- The `skills/README.md` in this milestone already documents how to add new skills. Follow it literally: new skills go in `skills/<name>/SKILL.md`, no code changes needed in `sldo-install`.
- Idempotency assumption: re-running `sldo-install` after adding a skill must pick it up. That's working today; M2 should not re-verify it unless a change makes it doubtful.

## Template improvements suggested
- The v3 template's Evidence Log table is useful, but for M1 we skipped copying it inline because the tests, build, and smoke steps are all captured by `cargo test` + `cargo build` + one `--dry-run`. Consider making the evidence log optional for milestones where every row would just be "`cargo test` passed".
