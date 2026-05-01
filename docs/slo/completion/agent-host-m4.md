# Completion Summary — agent-host Milestone 4

## Goal completed
- Claude-only Rust automation surfaces are now named honestly. The shared CLI helper lives in `crates/sldo-common/src/claude_cli.rs` and the live runtime harness lives in `crates/sldo-install/tests/common/claude_runtime.rs`. Behavior is unchanged; the source tree itself carries the Claude-only boundary.

## Files changed
- `crates/sldo-common/src/claude_cli.rs` (NEW; supersedes `copilot.rs`).
- `crates/sldo-common/src/copilot.rs` (DELETED).
- `crates/sldo-common/src/lib.rs` (export `claude_cli` instead of `copilot`).
- `crates/sldo-common/src/preflight.rs` (clarified docs that `check_claude_installed` is explicitly Claude-specific).
- `crates/sldo-research/src/research.rs` (import `sldo_common::claude_cli::ClaudeInvocation`).
- `crates/sldo-install/tests/common/claude_runtime.rs` (NEW; supersedes `judgment_runtime.rs`).
- `crates/sldo-install/tests/common/judgment_runtime.rs` (DELETED).
- `crates/sldo-install/tests/common/mod.rs` (export `claude_runtime` instead of `judgment_runtime`).
- `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs`, `…_m2.rs` (import from `common::claude_runtime`).
- `references/biz/judgment-fixtures/README.md` (point at the renamed module; mark the runtime as Claude-only).
- `docs/ARCHITECTURE.md` (sldo-common module table + current host boundaries note the renamed Claude-only modules).
- `docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` (forward-looking note recording the rename; body untouched).

## Tests added
- None. The existing `ClaudeInvocation` unit tests in the renamed `claude_cli` module and the `claude_runtime`-helper unit tests act as compile-failing rename guards — adding a third file purely to assert filename-shaped facts would have widened scope past the milestone's allow-list.

## Runtime validations added
- None new. `cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1 --test e2e_biz_judgment_runtime_m2` continues to compile and skip live tests via `skip_if_not_live()` when `BIZ_JUDGMENT_RUNTIME_LIVE=1` is not set.

## Compatibility checks performed
- `cargo build -p sldo-common -p sldo-research -p sldo-install` builds cleanly.
- `cargo test -p sldo-common -p sldo-install -p sldo-research` is fully green; per-crate test counts match the pre-rename baseline.
- Existing live judgment runtime tests still compile under the renamed import path.
- Env vars `BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN`, `BIZ_JUDGMENT_RUNTIME_LIVE`, `BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD`, and `BIZ_JUDGMENT_RUNTIME_RETRIES` keep their existing names — no aliasing required because the rename is at the file/module level, not the env-var level.
- No new host-neutral runtime promise was introduced; the runbook explicitly forbids inventing one without a second real implementation.

## Documentation updated
- `docs/ARCHITECTURE.md` — sldo-common module table renamed `copilot` → `claude_cli`; preflight description records the Claude-CLI scope; "Current host boundaries" notes the Claude-named helper module.
- `docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` — single forward-looking note at the top recording the M4 rename. The body is unchanged on purpose (Global Red Lines forbid mass-editing historical runbooks).
- `references/biz/judgment-fixtures/README.md` — Runtime harness section now points at `claude_runtime.rs` and labels the harness explicitly Claude-only.
- `crates/sldo-common/src/claude_cli.rs` — module docstring records the rename and the no-host-neutral-abstraction stance.
- `crates/sldo-install/tests/common/claude_runtime.rs` — module docstring records the rename, the env-var compatibility decision, and the Claude-only auth/HOME caveat.

## .gitignore changes
- None. M4 introduces no generated files or build outputs; existing patterns remain correct.

## Test artifact cleanup verified
- `git status` is clean of untracked test artifacts. The only working-tree changes are the renamed source modules, the doc updates, and the M3 closeout files (lessons + completion).

## Deferred follow-ups
- Optional drift guard: a one-shot structural assertion that no `copilot.rs` remains under `crates/sldo-common/src/` and no `judgment_runtime.rs` remains under `crates/sldo-install/tests/common/`. Today the absence is enforced implicitly by the build. Skipped here because adding it would land outside the M4 allow-list.

## Known non-blocking limitations
- The forward-looking note in `docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` does not rewrite every internal reference to `judgment_runtime.rs`. That is deliberate — the runbook is a historical record and the rename is documented at the top so a reader hitting any older internal reference understands it now lives under `claude_runtime.rs`.
