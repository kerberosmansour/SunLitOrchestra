# Completion — biz-b1 Milestone 1

**Status**: `done` (2026-04-25)

## What shipped

- `skills/slo-talk-to-users/SKILL.md` (~140 lines, generator)
- `skills/slo-verify/SKILL.md` — Pass 4 PII-scan additive sub-step (no removal of existing Pass 4 behaviour)
- `references/biz/artifact-schema.md` — 4 new frontmatter keys (`archetype`, `mode_arg`, `pii_scan_override`, `tier_override_reason`)
- `crates/sldo-install/tests/e2e_biz_b1_m1.rs` — 8 structural-contract tests
- `docs/slo/verify/biz-b1-m1-smoke.md` — 5 fixtures
- ARCHITECTURE.md +1 row

## Tests

- B1 M1: **8/8 passing**
- Runbook A regression: 42/42 still green (verified pre-execution)
- Full baseline: green

## Deferral closed

`/slo-verify` PII-pattern scan integration was deferred from Runbook A M1 lessons → landed here in B1 M1. ✅

## Next

[B1 M2 — `/slo-gtm`](../completed/RUNBOOK-BIZ-SKILL-PACK-B1.md#milestone-2--slo-gtm).
