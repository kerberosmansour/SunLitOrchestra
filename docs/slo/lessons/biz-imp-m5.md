# Lessons Learned — biz-imp Milestone 5

## What changed

- Extended `references/biz/artifact-schema.md` additively with 9 optional audit/provenance fields: `baseline_ref`, `intake_summary`, `gates_evaluation`, `restated_and_confirmed`, `restated_at`, `agent_version`, `agent_session_id`, `conversation_turn_count`, and `intake_duration_seconds`.
- Added `.sldo/refresh-loop.toml` as a PR-only annual refresh-loop configuration for M4 baseline files.
- Added `crates/sldo-install/tests/e2e_biz_imp_m5.rs` covering schema optionality, additive-only preservation, advisor/generator citations, predicate immutability, and no-auto-merge loop discipline.

## Design decisions and why

- The runbook referenced an existing `/schedule` skill, but no such skill body exists in this repo/session. Rather than inventing behavior, M5 records the refresh contract as data and tests the safety properties directly.
- The cross-skill citation test validates the already-shipped M2/M4 citations instead of requiring `references/templates/restate-and-confirm.md`, which is not present and not allowed for M5 edits.
- The schema extension uses all 9 audit fields described in the M5 file table, not only the 3 older summary fields. This keeps the "over-engineering for simplicity" discipline visible without breaking older artifacts.

## Recommendations after runbook

- Issue #34 remains the right lane for repo hygiene and branch discipline in `/slo-execute` and `/slo-ticket-execute`.
- A future schedule-runtime runbook should define how `.sldo/refresh-loop.toml` is executed, including manual dry-run behavior and PR body file creation.
- If `references/templates/restate-and-confirm.md` lands in another runbook, extend the M5 citation test to require that template link too.

## Tests run

- `cargo test -p sldo-install --test e2e_biz_imp_m5`: failed before M5 changes, then passed 5/5.
- `cargo test -p sldo-install`: passed.
- `cargo test --workspace`: passed.

## Changes to runbook tracker

- M5 status `not_started` to `done`. Started 2026-05-03, completed 2026-05-03.
