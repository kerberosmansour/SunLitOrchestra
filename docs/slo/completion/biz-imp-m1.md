# Completion Summary — biz-imp Milestone 1

**Status**: `done` (2026-05-03)
**Goal**: Source-verify the UK regulator enum, add statute-anchor authority files, and refresh HMRC/ICO reference files with short verbatim quotes.
**Runbook**: [docs/slo/future/RUNBOOK-BUSINESS-SKILL-IMPROVEMENTS.md](../future/RUNBOOK-BUSINESS-SKILL-IMPROVEMENTS.md) Milestone 1.

## What shipped

| Path | Purpose |
|---|---|
| `references/biz/uk-regulator-enumeration.md` | Adds a source verification register with official URLs, `last_checked: 2026-05-03`, `next_review_due: 2027-05-03`, and confidence values. |
| `references/biz/uk-employment-statute-anchors.md` | New authority anchors for IANA 2006, ERA 1996, Pensions Act 2008, Equality Act 2010, and ITEPA 2003 Ch 10. |
| `references/biz/uk-consumer-statute-anchors.md` | New authority anchors for CRA 2015, Consumer Contracts Regulations 2013, and DMCC 2024. |
| `references/biz/uk-marketing-statute-anchors.md` | New authority anchors for ASA/CAP, PECR, DUAA Schedule 13, and DPA 2018 s157. |
| `references/biz/hmrc-vcm-index.md` | Refreshes VCM34080, VCM3000, VCM31000, Abingdon Health marker, and Advance Assurance floor with source URLs and `quoted_text:` fields. |
| `references/biz/ico-duaa-index.md` | Refreshes DUAA commencement, ICO Stage 3 date, PECR amendment, and penalty ceiling anchors. |
| `crates/sldo-install/tests/e2e_biz_imp_m1.rs` | New M1 structural-contract test. |
| `tests/e2e_research_m1.rs` through `tests/e2e_research_m7.rs` | Baseline fix: build `sldo-research` when missing; isolate M1 prompt tests from live provider execution. |

## Test results

- `cargo test --workspace`: passed after the baseline research-test fix.
- `cargo test -p sldo-install --test e2e_biz_imp_m1`: passed, 6/6.
- `cargo test -p sldo-install`: passed.

## Compatibility checked

- Four hard-block predicate IDs unchanged.
- `references/biz/cost-baseline-jpp-law-2026.md` unchanged.
- `references/biz/ir35-cest-factors.md` unchanged.
- `references/biz/jurisdiction-uk.md` unchanged.
- Existing DUAA date and HMRC Advance Assurance lead-time expectations remain satisfied.

## Deferred

- `docs/ARCHITECTURE.md` reference-subtree update was not touched in M1 because it is outside the M1 allow-list. M2 or a small docs follow-up should add that cross-reference once advisor SKILL.md prose starts consuming the new authority files.

## Next milestone

M2: conversational intake contracts and advisor SKILL.md updates.
