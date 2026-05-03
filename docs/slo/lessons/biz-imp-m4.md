# Lessons Learned — biz-imp Milestone 4

## What changed

- Refreshed `references/biz/saas-kpi-targets-baseline.md` with `retrieved: 2026-05-03`, per-row `source_url:`, `last_checked:`, `confidence:`, `methodology_note:`, `sample_size:`, `vintage:`, and `applicability_caveat:`.
- Added five sister baselines: outbound conversion, product prioritization frameworks, value-equation pricing, Mom Test question schema, and launch success thresholds.
- Updated seven generator skills to emit `baseline_ref:`, cite the relevant baseline, warn when consulted rows are stale after 12 months, and refuse at +24 months.
- Added `crates/sldo-install/tests/e2e_biz_imp_m4.rs`.

## Design decisions and why

- Outbound conversion rates are intentionally low-confidence. Bridge Group and RAIN Group are useful public anchors, but the exact stage rates depend heavily on ICP, ACV, and qualification definitions.
- Launch thresholds are founder-owned rather than universal. The baseline says "set your own threshold" and records `threshold_owner: founder`.
- Mom Test content is attribution-only. The question schema is paraphrased and avoids long verbatim excerpts from Fitzpatrick's book.
- Existing B2 compatibility tests require visible strings such as `110%`, `15%`, and `≤ 2` in `/slo-metrics`; those remain as readability mirrors, with the baseline file as authority.

## Recommendations for M5

- Add `baseline_ref:`, `intake_summary:`, and `gates_evaluation:` as optional schema fields only; do not tighten existing artifact parsing.
- The refresh loop should open a PR and never auto-merge.
- Cross-skill citation tests should assert the new M4 baseline links without touching generator skill prose again.

## Tests run

- `cargo test -p sldo-install --test e2e_biz_imp_m4`: failed before M4 changes, then passed 5/5.
- `cargo test -p sldo-install`: passed.
- `cargo test --workspace`: passed.

## Changes to runbook tracker

- M4 status `not_started` to `done`. Started 2026-05-03, completed 2026-05-03.
