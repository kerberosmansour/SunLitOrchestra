# Lessons Learned — biz-a Milestone 4 (`/slo-fundraise` + IR35/CEST + CLAUDE.md catalog)

## What changed

- **NEW skill** `skills/slo-fundraise/SKILL.md` — fourth and final advisor skill in Runbook A; runs SEIS/EIS Advance Assurance pre-check on every interaction; refuses term-sheet drafting without AA ≥ 6 weeks ahead of signature.
- **NEW** `references/biz/ir35-cest-factors.md` — three-factor IR35 reference (substitution, MOO, control), CEST April 2025 refresh, PGMOL v HMRC [2024] UKSC 29 commentary, seven hard-block-to-lawyer triggers.
- **NEW** `crates/sldo-install/tests/e2e_biz_a_m4.rs` — 11 structural-contract tests including the FINAL four-skill cross-citation test, AA pre-check enforcement, IR35 three-factor / CEST refresh / seven-trigger documentation, and the CLAUDE.md catalog completeness check.
- **NEW** `docs/slo/verify/biz-a-m4-smoke.md` — manual smoke checklist with 7 fixtures + cross-skill consistency + final pack-level verification.
- **UPDATED** `CLAUDE.md` — added "Biz skill pack — first-party advisor + generator skills" section cataloging all four advisor skills with their domains and cited reference files. Single bundled edit per the runbook plan.
- **UPDATED** `docs/ARCHITECTURE.md` — added `slo-fundraise` row, completing the four-row biz advisor table.
- **UPDATED** `docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md` Milestone Tracker — M4 status `done`.

## Design decisions and why

- **AA pre-check is mandatory on EVERY `/slo-fundraise` interaction**, not just SEIS-tagged ones. Reason: founders sometimes ask "is this term sheet OK to sign" thinking the SEIS angle is decided, when in fact AA was never applied. Surfacing the question on every interaction catches this trap. This is similar to `/slo-equity`'s on-every-draft SEIS pre-check pattern (M3) — both skills enforce the same discipline because the same trap exists across cofounder-split work and term-sheet work.
- **Term-sheet redline brief is technically `prepare`-shaped but lives under `draft <doc-type>`** because founders intuitively reach for "draft a brief" rather than "prepare for a meeting". Filing it under `draft` with explicit "ALWAYS-prepare-shape" note in the SKILL.md prose maintains discoverability without breaking the mode contract.
- **CLAUDE.md catalog is a single edit at M4** — bundled at end-of-runbook because that's when all four advisor skills exist to be cataloged together. Smaller per-milestone CLAUDE.md edits would have made the table churn.
- **The four predicate-id immutability test continues to pass** through M4. The cross-skill citation test is now at its final extent (4 advisors × 4 predicates = 16 citations) and serves as the runbook's headline drift-prevention surface for any future biz-pack work.

## Course corrections taken in flight

- **`slo-fundraise` SKILL.md initially failed the AA-pre-check structural test** because I cited "VCM34080 / VCM3000 / VCM31000" as a parenthetical group in the description, but the body referenced only VCM34080 and VCM3000. Added explicit per-trigger VCM31000 citations to satisfy the test. One iteration cycle.
- **No course corrections to the runbook plan** itself. The 4-milestone structure of Runbook A held; pattern from M1 transferred cleanly to M2 / M3 / M4 with monotonically decreasing per-milestone effort.

## Mistakes made

- **Initial test wording for VCM citations was overly strict** — required all three VCM paragraphs by literal substring. The skill prose initially used "VCM3000 series" as a group reference covering both VCM3000 and VCM31000, which the strict test missed. Lesson: when a reference document has a numerical hierarchy (VCM3000, VCM31000, VCM34080), tests should accept either the exact paragraph or a documented group-name AND the prose should be specific where the test demands specificity.

## Recommendations for /slo-critique on Runbook A

When `/slo-critique` runs against this runbook, the following questions are worth surfacing per persona:

- **CEO**: Is the £5,000 deal-value threshold (gate-2) the right number, or does it under-trigger for low-value matters where templates would actually be safe? This was the original locked decision (2026-04-25); reversal would relax draft availability for routine micro-engagements but reduce the safety margin.
- **Eng**: Is the placeholder-for-oneNDA pattern correct, or should `/slo-legal draft nda` SHIP with a substantive UK NDA authored from skill prose (no oneNDA dependency)? The placeholder is conservative; the alternative is faster but loses the lawyer-reviewed-text benefit.
- **Security**: The cross-skill citation test is the load-bearing drift-prevention surface. Are there other invariants that should also have structural-contract tests? (Examples: artifact-frontmatter schema validation per skill output; jurisdiction-uk regulator-list completeness; cost-baseline retrieval-date staleness check.)
- **Design** (skipped per `/slo-critique` rules — biz pack has no UI surface): N/A.

## Recommendations for /slo-ship

- The biz pack ships ~15 new files across `skills/slo-{legal,accounting,equity,fundraise}/`, `references/biz/`, `crates/sldo-install/tests/`, `docs/slo/verify/`, `docs/slo/lessons/`, `docs/slo/completion/`, plus CLAUDE.md + ARCHITECTURE.md updates.
- The PR description should call out: GDPR broad hard-block locked decision; JPP Law cost baseline locked decision; oneNDA SHA-256 hash check deferred (with placeholder); `/slo-verify` PII-pattern scan deferred to Runbook B1 M1.
- Runbook B1, B2, C remain unstarted; the pack is at 4/15 skills shipped with the load-bearing scaffolding (`references/biz/`) in place to make B1+B2+C lighter-weight per-skill.

## Changes to ARCHITECTURE.md

- Added `slo-fundraise` row.

## Changes to CLAUDE.md

- Added "Biz skill pack" section with all four advisor-skill rows and the references/biz/ scaffolding location.

## Changes to runbook tracker

- M4 status `not_started` → `done`. Started 2026-04-25, completed 2026-04-25.
- All four milestones now `done`. Runbook A is ready for `/slo-critique` and `/slo-ship`.
