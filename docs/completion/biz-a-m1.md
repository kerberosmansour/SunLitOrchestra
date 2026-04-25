# Completion Summary — biz-a Milestone 1

**Status**: `done` (2026-04-25)
**Goal**: Ship `/slo-legal` v1 with the four-mode advisor contract, hard-block gate wiring, and the M1-tier shared `references/biz/` scaffolding.
**Runbook**: [docs/RUNBOOK-BIZ-SKILL-PACK-A.md](../RUNBOOK-BIZ-SKILL-PACK-A.md) Milestone 1.

## What shipped

| Path | Purpose | Lines |
|---|---|---|
| `skills/slo-legal/SKILL.md` | Advisor skill: 4 modes (`draft`/`translate`/`triage`/`prepare`), 4 hard-block gate citations, oneNDA verbatim render rule, JPP Law cost-baseline ROI block, UK-only jurisdiction error, two-tier output convention | ~120 |
| `references/biz/triage-gate.md` | Single source of truth for the four hard-block predicates with the six-column predicate-id schema | ~60 |
| `references/biz/cost-baseline-jpp-law-2026.md` | UK fixed-fee solicitor pricing baseline (JPP Law, retrieved 2026-04-25); placeholder GBP figures pending live-page replacement at first use | ~70 |
| `references/biz/templates/onenda-uk.md` | oneNDA UK Country Schedule placeholder (CC BY-ND 4.0 verbatim render obligation; canonical bytes deferred) | ~50 |
| `crates/sldo-install/tests/e2e_biz_a_m1.rs` | 10 structural-contract tests | ~280 |
| `docs/verify/biz-a-m1-smoke.md` | Manual smoke-test checklist for runtime verification (5 fixtures + edge cases) | ~80 |
| `docs/ARCHITECTURE.md` | One-row addition to skill table; one paragraph documenting `references/biz/` invariant | +2 |

## Test results

- **Structural-contract tests** (`cargo test -p sldo-install --test e2e_biz_a_m1`): **10/10 passing**
  - `slo_legal_skill_md_has_required_frontmatter` ✅
  - `slo_legal_skill_md_documents_four_modes` ✅
  - `triage_gate_md_defines_four_predicate_ids` ✅
  - `slo_legal_skill_md_cites_all_four_predicate_ids` ✅
  - `cost_baseline_md_carries_retrieval_date` ✅
  - `onenda_template_placeholder_or_pinned_hash` ✅
  - `references_biz_dir_not_discovered_as_skill` ✅
  - `gdpr_doc_draft_routes_to_triage` ✅
  - `confidential_draft_to_public_tier_rejected` ✅
  - `non_uk_jurisdiction_arg_rejected` ✅

- **Full baseline** (`cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`): all green, 330+ tests, no regressions.

- **Installer dry-run** (`./target/release/sldo-install --dry-run`): `slo-legal` would be created (NEW); 14 existing skills unchanged; `references/biz/` correctly NOT discovered.

## Pre-ship decisions implemented (locked 2026-04-25)

- **GDPR rule calibration**: Broad hard-block on `draft` for ALL GDPR documents. `gate-4-gdpr-document` is unconditional refusal in the `triage-gate.md` source of truth and in `/slo-legal` SKILL.md prose.
- **Cost baseline**: JPP Law fixed-fee public pricing (https://www.jpplaw.co.uk/sectors/fixed-fee-startup/) — auditable, reversible to a different firm via migration.

## Deferrals (deliberate, documented)

- **oneNDA SHA-256 hash check**: deferred from M1 to a small follow-up runbook. Reason: oneNDA is published as PDF; reliable verbatim Markdown rendering requires the project owner to fetch and verify canonical bytes manually. M1 ships a placeholder with marker `ONENDA-UK-PLACEHOLDER`; structural test accepts placeholder OR future-pinned hash; `/slo-legal draft nda` refuses to draft until placeholder replaced.
- **`/slo-verify` PII-pattern scan**: deferred from M2 (per M1's runbook spec) to **Runbook B1 M1** (`/slo-talk-to-users`). Reason: cleaner to land the scan when the first PII-shaped generator artifacts exist as test fixtures.

## What broke / what got fixed during execution

- First test pass failed `slo_legal_skill_md_has_required_frontmatter` because YAML `description: >` block-scalar handling was strict for inline-only. Fixed by extending the test to accept block-scalar followed by indented content. One iteration cycle. See lessons file for root cause.

## Compatibility checked

- ✅ All 14 pre-runbook skills load via `sldo-install --dry-run`
- ✅ All 10 existing runbooks (`docs/RUNBOOK-*.md`) parse against `runbook-template_v_3_template.md` (asserted by full baseline test set)
- ✅ `crates/sldo-install/src/install.rs` unchanged
- ✅ `docs/runbook-template_v_3_template.md` unchanged
- ✅ `crates/sldo-tauri/` unchanged
- ✅ `SECURITY.md` (root) — biz-pack section already merged by `/slo-architect`; no further M1 edits

## Next milestone

[M2 — `/slo-accounting` + 5 M2-tier shared references](../RUNBOOK-BIZ-SKILL-PACK-A.md#milestone-2--slo-accounting--m2-tier-shared-references). M2 proves the advisor pattern replicates by reusing M1's `triage-gate.md` predicates without modification (cross-skill citation structural test).
