# Completion Summary — biz-a Milestone 2

**Status**: `done` (2026-04-25)
**Goal**: Ship `/slo-accounting` (second advisor skill, proves replication) + 5 M2-tier shared references.
**Runbook**: [docs/RUNBOOK-BIZ-SKILL-PACK-A.md](../RUNBOOK-BIZ-SKILL-PACK-A.md) Milestone 2.

## What shipped

| Path | Purpose | Lines |
|---|---|---|
| `skills/slo-accounting/SKILL.md` | Second advisor skill; same four-mode contract; HMRC default routing → accountant | ~100 |
| `references/biz/artifact-schema.md` | Frontmatter contract + per-skill / per-mode default-tier mapping for all 15 biz skills | ~70 |
| `references/biz/jurisdiction-uk.md` | UK regulator index with per-skill routing override pattern; canonical "v1 supports UK only" error string | ~70 |
| `references/biz/ico-duaa-index.md` | DUAA 2025 dates + key changes (lawful basis, Article 22, PECR ceiling, complaints duty) | ~70 |
| `references/biz/ico-enforcement-reality.md` | Descriptive provenance for the broad GDPR hard-block; non-normative disclaimer-led | ~80 |
| `references/biz/open-template-anchors.md` | Open-licensed template index + license obligations (oneNDA CC BY-ND 4.0 verbatim) | ~70 |
| `crates/sldo-install/tests/e2e_biz_a_m2.rs` | 11 structural-contract tests including cross-skill citation + predicate-id immutability | ~270 |
| `docs/verify/biz-a-m2-smoke.md` | Manual smoke checklist with 6 fixtures | ~80 |
| `docs/ARCHITECTURE.md` | One-row addition to skill table | +1 |

## Test results

- **M2 structural-contract tests** (`cargo test -p sldo-install --test e2e_biz_a_m2`): **11/11 passing**
  - `slo_accounting_skill_md_has_required_frontmatter` ✅
  - `slo_accounting_skill_md_documents_four_modes` ✅
  - `slo_accounting_skill_md_cites_all_four_predicate_ids` ✅
  - `cross_skill_advisor_pattern_replicated` ✅ (every advisor SKILL.md cites every predicate ID — M2's load-bearing test)
  - `triage_gate_predicate_set_unchanged_from_m1` ✅ (predicate-id immutability)
  - `artifact_schema_tier_value_constrained_to_enum` ✅
  - `jurisdiction_uk_md_has_canonical_error_string` ✅
  - `ico_duaa_index_carries_2026_dates` ✅
  - `ico_enforcement_reality_doc_does_not_contradict_gate_4` ✅
  - `open_template_anchors_documents_onenda_license` ✅
  - `references_biz_dir_still_not_discovered_as_skill` ✅

- **M1 regression** (`cargo test -p sldo-install --test e2e_biz_a_m1`): **10/10 still green**.
- **Full baseline**: all green, 330+ tests, no regressions.

## Pre-ship decisions implemented

- **Per-skill routing override pattern** documented in `jurisdiction-uk.md` UK regulator index. `gate-1-regulated`'s default `route_to: lawyer` is a default; HMRC matters route to accountant via skill prose. Predicate-id immutability preserved.
- **Cross-skill citation contract** locked: every advisor SKILL.md cites all four predicate IDs. Test enforces.

## What broke / what got fixed during execution

- Two test failures during first M2 test run, both self-inflicted:
  - `ico_enforcement_reality_doc_does_not_contradict_gate_4` failed because the doc quoted forbidden phrases as examples in the disclaimer (circular). Fixed by removing examples and pointing readers to the test source as canonical list.
  - `open_template_anchors_documents_onenda_license` failed because the doc didn't reference the placeholder file path. Fixed by adding path to the oneNDA row.
- After fixes: `relax gate-4` substring-match in a legitimate negation ("NOT a recommendation to relax gate-4...") tripped the test. Reworded to "weaken `gate-4-gdpr-document`".

Three iteration cycles in total to reach 11/11 green.

## Compatibility checked

- ✅ M1 outputs (`skills/slo-legal/SKILL.md`, `references/biz/triage-gate.md`, `cost-baseline-jpp-law-2026.md`, `templates/onenda-uk.md`) byte-identical to M1's commit.
- ✅ All 14 pre-runbook skills + `/slo-legal` + `/slo-accounting` = 16 skills loaded by `sldo-install --dry-run`.
- ✅ `references/biz/` correctly NOT discovered as a skill (regression on M1 invariant).
- ✅ All existing runbooks parse against `runbook-template_v_3_template.md`.
- ✅ `crates/sldo-install/src/install.rs`, `docs/runbook-template_v_3_template.md`, `crates/sldo-tauri/`, `SECURITY.md` (root) all unchanged.

## Deferrals (carried forward from M1)

- **oneNDA SHA-256 hash check** still deferred to a follow-up runbook. M2 did not change M1's placeholder mechanism.
- **`/slo-verify` PII-pattern scan** deferred to Runbook B1 M1 (`/slo-talk-to-users`) — first generator with PII-shaped artifacts.

## Next milestone

[M3 — `/slo-equity` + `references/biz/hmrc-vcm-index.md`](../RUNBOOK-BIZ-SKILL-PACK-A.md#milestone-3--slo-equity--referencesbizhmrc-vcm-indexmd). M3 follows M2's pattern (third advisor + new shared reference); cross-skill citation test extends to assert three advisor SKILL.mds cite all four predicates.
