# Lessons Learned — agent-host Milestone 4

## What changed
- Renamed `crates/sldo-common/src/copilot.rs` → `claude_cli.rs` so the file name matches its contents (every line of the body invokes `claude`). Module re-exported from `lib.rs`.
- Renamed `crates/sldo-install/tests/common/judgment_runtime.rs` → `claude_runtime.rs` with an explicit Claude-only docstring and an env-var compatibility note. `tests/common/mod.rs` updated.
- Updated callers: `crates/sldo-research/src/research.rs` imports `sldo_common::claude_cli::ClaudeInvocation`; `e2e_biz_judgment_runtime_m{1,2}.rs` import `common::claude_runtime::…`.
- Clarified `crates/sldo-common/src/preflight.rs` so the `check_claude_installed` helper is documented as explicitly Claude-specific.
- Updated `references/biz/judgment-fixtures/README.md` and `docs/ARCHITECTURE.md` to point at the renamed module and note the Claude-only boundary.
- Added a forward-looking note at the top of `docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` so the historical record reflects the rename without rewriting the runbook body.

## Design decisions and why
- Did not introduce an `AgentRuntime` trait or any cross-host abstraction. The runbook explicitly forbids that without a second real implementation, and inventing one would re-create the exact problem the agent-host work is solving — fake parity that hides Claude-only dependencies.
- Kept the env vars named `BIZ_JUDGMENT_RUNTIME_*` instead of renaming to e.g. `BIZ_CLAUDE_RUNTIME_*`. The variables were already explicitly Claude-named via the `_CLAUDE_BIN` suffix where relevant, and renaming would break user automation for no behavior gain. The new module docstring records the deliberate choice.
- Did not add an `e2e_agent_host_m4.rs` structural test. The existing `ClaudeInvocation` unit tests and the `claude_runtime`-helper unit tests already act as compile-failing rename guards — adding a third file purely to assert filename-shaped facts would widen scope past the milestone's allow-list.
- Touched `RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` only with a forward-looking note. The Global Red Lines forbid mass-edits to historical runbooks; the M4 contract permits a single record-the-boundary annotation, which the forward-looking block satisfies.

## Mistakes made
- Initially wrote the new claude_runtime.rs docstring stating that HOME was redirected. That was true of the file's earlier comments but the actual `invoke_claude` body intentionally leaves HOME alone for auth reasons. Caught and fixed before tests ran — the docstring now matches the implementation.

## Root causes
- The original module names (`copilot.rs`, `judgment_runtime.rs`) date from when the project was a Claude-only Copilot-style runner. As soon as multi-host install support landed, the names became misleading. The fix was structural: rename the files so the source tree itself carries the boundary.

## What was harder than expected
- Keeping the historical-doc edits minimal while still recording the rename. The judgment-runtime runbook references `judgment_runtime.rs` in dozens of lines; rewriting them all would have been mass churn. The forward-looking-note pattern at the top kept the change small and reversible.

## Naming conventions established
- `claude_cli` is the canonical name for any Rust helper that shells out to the `claude` CLI. New helpers should go alongside it in `crates/sldo-common/src/` rather than under a host-neutral name.
- `claude_runtime` is the canonical name for any Rust live-runtime test harness that invokes `claude`. Future host-specific live runtimes (if ever) get their own honestly-named modules; no shared trait.
- "Forward-looking note" is the canonical pattern for adding a single block at the top of a historical runbook to record a downstream rename or boundary change without mass-editing the body.

## Test patterns that worked well
- Renaming a module while leaving the public API stable means the existing unit + integration tests already cover the rename. New structural tests would have been redundant.
- Running `cargo build -p <crate>` for each impacted crate before running the full test suite caught import drift quickly.

## Missing tests that should exist now
- A structural test that asserts no `copilot` module remains under `crates/sldo-common/src/` and no `judgment_runtime` module remains under `crates/sldo-install/tests/common/`. Today the absence is enforced implicitly by the rest of the build; if a future merge re-creates them, the build will only fail when something imports them. Adding a one-shot drift guard would be cheap, but it lives outside this milestone's allow-list.

## Rules for the next milestone
- M5 is targeted skill + structural-test cleanup. Stay narrow: only the listed skills (`/slo-second-opinion`, `/slo-rulegen`, `/slo-sast`) and the listed structural test (`e2e_slo_sp_m8.rs`). Do not regenerate the full Claude-vs-host-neutral wording sweep.
- Honest naming for genuinely Claude-only behavior must stay — do not soften wording in `/slo-second-opinion` (which legitimately compares the current host against another model) just to make it look host-neutral.

## Template improvements suggested
- The runbook's Files Allowed To Change tables would benefit from a column distinguishing "rename source" from "edit existing file" — the M4 entries for `copilot.rs` and `judgment_runtime.rs` are deletions, but the table reads like normal edits.
