# Completion — biz-b1 M4 (final B1 milestone)

**Status**: `done` (2026-04-25). Goal: ship `/slo-marketing` + CLAUDE.md catalog edit. Closes Runbook B1.

## Shipped

- `skills/slo-marketing/SKILL.md` (~150 lines, generator with mode arg `b2b | b2c`)
- `crates/sldo-install/tests/e2e_biz_b1_m4.rs` (9 tests including CLAUDE.md catalog)
- `docs/verify/biz-b1-m4-smoke.md` (6 fixtures)
- `CLAUDE.md` — bundled catalog edit (4 B1 generator rows + PII discipline paragraph)
- `docs/ARCHITECTURE.md` +1 row

## Tests

- B1 M4: **9/9 passing**
- B1 M1+M2+M3+M4: 8 + 7 + 8 + 9 = **32/32 passing**
- Runbook A regression: 42/42 still green
- Full baseline: green

## Pack-level state at end of Runbook B1

### Generators added (4)

- `/slo-talk-to-users` (M1, mode_arg `pre-interview | post-interview`)
- `/slo-gtm` (M2, single-mode)
- `/slo-product` (M3, mode_arg `roadmap | metrics | okrs`)
- `/slo-marketing` (M4, mode_arg `b2b | b2c`)

### Total skills in pack: 8 (4 advisors from Runbook A + 4 generators from B1)

### Sub-pack scope queued

- **Runbook B2** (4 milestones): `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`
- **Runbook C** (3 milestones): `/slo-cofounder`, `/slo-hire`, `/slo-founder-check`

### Critique findings status (from Runbook A critique)

- f1 confirmed (one PR per runbook).
- f2 deferred — `biz-pack-cost-baseline-refresh` still pending.
- f3, f4, f5 deferred — `biz-pack-test-hardening` + `biz-pack-onenda-canonical` still pending.
- f6 deferred — `biz-pack-judgment-tests` still pending (post-B2 + C).

## Next: `/slo-critique` on Runbook B1, then `/slo-ship`, then Runbook B2.
