# Completion Summary — biz-a Milestone 3

**Status**: `done` (2026-04-25)
**Goal**: Ship `/slo-equity` (third advisor) + HMRC VCM citation index.

## What shipped

- `skills/slo-equity/SKILL.md` (~95 lines)
- `references/biz/hmrc-vcm-index.md` (~80 lines)
- `crates/sldo-install/tests/e2e_biz_a_m3.rs` (~120 lines)
- `docs/slo/verify/biz-a-m3-smoke.md` (~25 lines)
- `docs/ARCHITECTURE.md` (+1 row)

## Test results

- M3 structural-contract tests: **10/10 passing**
- M1 + M2 regression: 10/10 + 11/11 still green
- Full baseline: green, no regressions

## Cross-skill citation contract

✅ Three advisor skills now cite all four predicate IDs: `slo-legal`, `slo-accounting`, `slo-equity`. M4 will extend to four.

## Triage-gate predicate-id immutability

✅ Predicate set unchanged from M1: `{gate-1-regulated, gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper, gate-4-gdpr-document}`.

## Next milestone

[M4 — `/slo-fundraise` + `references/biz/ir35-cest-factors.md` + CLAUDE.md catalog edit](../completed/RUNBOOK-BIZ-SKILL-PACK-A.md#milestone-4--slo-fundraise--referencesbizir35-cest-factorsmd). Final milestone of Runbook A; also closes the runbook with the single CLAUDE.md edit cataloging all four advisor skills.
