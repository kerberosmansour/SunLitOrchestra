# Completion Summary — biz-a Milestone 4

**Status**: `done` (2026-04-25)
**Goal**: Ship `/slo-fundraise` (fourth advisor) + IR35/CEST factors reference + bundled CLAUDE.md catalog. Closes Runbook A.

## What shipped

- `skills/slo-fundraise/SKILL.md` (~110 lines)
- `references/biz/ir35-cest-factors.md` (~75 lines)
- `crates/sldo-install/tests/e2e_biz_a_m4.rs` (~140 lines)
- `docs/slo/verify/biz-a-m4-smoke.md` (~30 lines)
- `CLAUDE.md` — added "Biz skill pack" section (5 advisor rows + scaffolding paragraph + locked-decisions notes)
- `docs/ARCHITECTURE.md` — added `slo-fundraise` row, completing the biz advisor table

## Test results

- M4 structural-contract tests: **11/11 passing**
- M1 + M2 + M3 regression: 10 + 11 + 10 = 31/31 still green
- Full baseline: green, no regressions
- `sldo-install --dry-run`: 18 skills (14 pre-runbook + 4 biz advisors); `references/biz/` correctly NOT discovered

## Final cross-skill citation contract

✅ **All four advisor skills** (`slo-legal`, `slo-accounting`, `slo-equity`, `slo-fundraise`) cite ALL FOUR predicate IDs from `references/biz/triage-gate.md`. Test `cross_skill_advisor_pattern_replicated_all_four` enforces.

## Final triage-gate predicate-id immutability

✅ Predicate set unchanged from M1: `{gate-1-regulated, gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper, gate-4-gdpr-document}`. No additions, no removals, no renames across all four milestones.

## Pack-level deliverable inventory at end of Runbook A

### Skills (4)

- `skills/slo-legal/SKILL.md`
- `skills/slo-accounting/SKILL.md`
- `skills/slo-equity/SKILL.md`
- `skills/slo-fundraise/SKILL.md`

### Shared scaffolding under `references/biz/` (10 files)

- `references/biz/triage-gate.md` (M1)
- `references/biz/cost-baseline-jpp-law-2026.md` (M1)
- `references/biz/templates/onenda-uk.md` (M1, placeholder)
- `references/biz/artifact-schema.md` (M2)
- `references/biz/jurisdiction-uk.md` (M2)
- `references/biz/ico-duaa-index.md` (M2)
- `references/biz/ico-enforcement-reality.md` (M2)
- `references/biz/open-template-anchors.md` (M2)
- `references/biz/hmrc-vcm-index.md` (M3)
- `references/biz/ir35-cest-factors.md` (M4)

### Structural-contract tests (4)

- `crates/sldo-install/tests/e2e_biz_a_m1.rs` (10 tests)
- `crates/sldo-install/tests/e2e_biz_a_m2.rs` (11 tests)
- `crates/sldo-install/tests/e2e_biz_a_m3.rs` (10 tests)
- `crates/sldo-install/tests/e2e_biz_a_m4.rs` (11 tests)

**Total**: 42 structural-contract tests, all green.

### Smoke checklists (4)

- `docs/slo/verify/biz-a-m1-smoke.md`
- `docs/slo/verify/biz-a-m2-smoke.md`
- `docs/slo/verify/biz-a-m3-smoke.md`
- `docs/slo/verify/biz-a-m4-smoke.md`

### Lessons + completion files (8)

- `docs/slo/lessons/biz-a-m{1,2,3,4}.md`
- `docs/slo/completion/biz-a-m{1,2,3,4}.md`

### Updated existing files

- `CLAUDE.md` — biz pack catalog section
- `docs/ARCHITECTURE.md` — 4 new advisor rows + invariants paragraph
- `SECURITY.md` — biz pack handling rules (added by `/slo-architect` at design time; preserved through all four milestones)

## Pre-ship decisions implemented (locked 2026-04-25)

- ✅ GDPR broad hard-block on `draft` for all GDPR documents
- ✅ JPP Law fixed-fee public pricing as cost baseline
- ✅ UK only in v1 (canonical "v1 supports UK only" error from jurisdiction-uk.md)
- ✅ oneNDA verbatim render under CC BY-ND 4.0 (placeholder pending canonical bytes)
- ✅ Two-tier output convention (`docs/biz/` confidential / `docs/biz-public/` placeholder)
- ✅ No WebFetch / WebSearch in biz skills
- ✅ Per-skill routing override pattern for HMRC matters (accountant default)
- ✅ Cross-skill citation contract enforced by structural test
- ✅ Predicate-id immutability enforced by structural test

## Deferrals (carried into future runbooks)

- **oneNDA SHA-256 hash check** — small follow-up runbook pinning canonical bytes once project owner has fetched and verified.
- **`/slo-verify` PII-pattern scan integration** — Runbook B1 M1 (`/slo-talk-to-users`) where the first PII-shaped artifacts will exist as fixtures.
- **JPP Law GBP figure replacement** — manual step at first use; placeholder ships in M1 with retrieval-date stamping convention.

## Next steps

- `/slo-critique` against Runbook A (CEO / eng / security personas; design persona skipped — no UI surface).
- `/slo-ship` once critique findings are resolved → first PR for the biz pack.
- Runbook B1 (`/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`) follows.
- Runbook B2 (`/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`) follows.
- Runbook C (`/slo-cofounder`, `/slo-hire`, `/slo-founder-check`) closes the pack.

The advisor cluster is done. The 11 generators in B1 / B2 / C will be lighter-weight per-skill because the shared scaffolding (`references/biz/`) is now in place.
