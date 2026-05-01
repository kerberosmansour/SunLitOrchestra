# Lessons Learned — biz-a Milestone 3 (`/slo-equity` + `references/biz/hmrc-vcm-index.md`)

## What changed

- **NEW skill** `skills/slo-equity/SKILL.md` — third advisor skill; same four-mode contract; runs SEIS / EIS pre-check on every equity draft via the new HMRC VCM index.
- **NEW** `references/biz/hmrc-vcm-index.md` — citation index for VCM34080 (control / disqualifying arrangements), VCM3000 (excluded activities), VCM31000 (SEIS income tax relief), plus Advance Assurance lead-time guidance (≥ 6 weeks before term-sheet signature) and the Abingdon Health Limited v HMRC [2016] preferential-rights case-law citation.
- **NEW** `crates/sldo-install/tests/e2e_biz_a_m3.rs` — 10 structural-contract tests including the cross-skill three-advisor citation test, the AA-lead-time and Abingdon-Health citation checks, and the SEIS / EIS pre-check enforcement on `/slo-equity` SKILL.md.
- **NEW** `docs/slo/verify/biz-a-m3-smoke.md` — manual smoke checklist with 5 fixtures.
- **UPDATED** `docs/ARCHITECTURE.md` — added `slo-equity` row.
- **UPDATED** `docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md` Milestone Tracker — M3 status `done`.

## Design decisions and why

- **`/slo-equity` requires "LAWYER + ACCOUNTANT REVIEW RECOMMENDED"** (dual review), not the single-professional review of M1 (lawyer) or M2 (accountant). Reason: equity work is the first real cross-discipline domain — share-class drafting is solicitor work, tax/qualifying-trade is accountant work, and most equity decisions touch both. The dual header makes the contract explicit.
- **Preferential-rights ambiguity hard-blocks the cofounder-split draft.** Per the Abingdon Health line, ordinary shares granted preferential rights (even via SHA drafting) lose SEIS / EIS qualification retroactively. The skill defaults to ordinary-only-for-founders unless the founder explicitly confirms — and even confirmation routes to lawyer for solicitor-drafted articles / SHA review.
- **HMRC VCM index pre-check runs on EVERY equity draft, not just SEIS-tagged ones.** Reason: founders don't always realise that what looks like a non-SEIS equity question (e.g., cofounder split with founder loans) can break SEIS later. Running the four-question check unconditionally surfaces the trap early.

## Course corrections taken in flight

- None — pattern is now well established. M3 wrote, tested, and shipped without test-design self-trips. Time-per-milestone is dropping (M1 ~ longer for the new pattern; M2 ~ pattern + minor self-trips; M3 ~ smooth).

## Mistakes made

- None material. The SEIS / EIS pre-check question wording in `hmrc-vcm-index.md` had to be carefully designed to be pre-emptive ("are you a 51%-owned subsidiary?") rather than reactive ("if you become a 51%-owned subsidiary, you'll lose SEIS"); reactive framing is less useful at the point of the founder asking the question.

## Recommendations for M4

- **Author `references/biz/ir35-cest-factors.md` first**, then `/slo-fundraise/SKILL.md`. Same reference-first discipline.
- **The cross-skill citation test extends to all FOUR advisor skills** in M4 — write it before the SKILL.md so the test fails for the right reason.
- **The single CLAUDE.md edit** is M4 only. Plan it as the LAST edit in M4 (after all four advisor skills exist) so the catalog is complete on first commit.
- **Plan the IR35 reference's three-factor structure deliberately**: the test should assert substitution / MOO / control are all named AND that CEST April 2025 refresh is documented AND that the PGMOL v HMRC [2024] UKSC 29 commentary is present.

## Changes to ARCHITECTURE.md

- Added `slo-equity` row.

## Changes to runbook tracker

- M3 status `not_started` → `done`. Started 2026-04-25, completed 2026-04-25.
