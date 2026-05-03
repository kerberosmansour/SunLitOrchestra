# Lessons Learned — biz-imp Milestone 3

## What changed

- Added `crates/sldo-install/tests/e2e_biz_imp_m3.rs`.
- Updated `/slo-fundraise` with SAFE worksheet verification: runnable stdlib-only Python snippet, expected-results table, two-pass verification, tolerances, and refusal-on-mismatch.
- Updated `/slo-equity` with cap-table snapshot verification: sum-down and weighted-product checks for every Total row.
- Updated `/slo-pricing` with value-equation verification: runnable Python snippet and reciprocal checks for the 25-33% band.

## Design decisions and why

- Verification lives in SKILL.md prose rather than a runtime dependency. The founder-visible Python snippets make the math auditable without changing the skill runtime.
- Tolerances are explicit: ±£1 for currency, ±0.01% for percentages, and ±1 for share counts.
- Mismatch behavior says "refuse to write" rather than silently correcting, because silent correction hides the failure mode this milestone exists to catch.

## Recommendations for M4

- Keep numeric heuristics out of generator skill prose wherever possible; point to baseline files instead.
- For unsourceable launch or sales heuristics, reframe as founder-set thresholds with provenance rather than pretending precision.
- Use source URLs and `last_checked:` dates per row; avoid vendor blogs as primary authority.

## Tests run

- `cargo test --workspace`: passed.
- `cargo test -p sldo-install --test e2e_biz_imp_m3`: failed before M3 changes, then passed 5/5.
- `cargo test -p sldo-install`: passed.

## Changes to runbook tracker

- M3 status `not_started` to `done`. Started 2026-05-03, completed 2026-05-03.
