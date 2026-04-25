# Lessons Learned — biz-a Milestone 2 (`/slo-accounting` + 5 M2-tier shared references)

## What changed

- **NEW skill** `skills/slo-accounting/SKILL.md` — second advisor skill in the biz pack; replicates `/slo-legal`'s four-mode contract; defaults HMRC-routing to accountant (not lawyer).
- **NEW** `references/biz/artifact-schema.md` — frontmatter contract for biz artifacts; tier enum (`confidential` | `public`); per-skill / per-mode default-tier mapping.
- **NEW** `references/biz/jurisdiction-uk.md` — UK regulator index with per-skill routing override pattern; canonical "v1 supports UK only" error string; UK regulatory anchor URL list.
- **NEW** `references/biz/ico-duaa-index.md` — DUAA 2025 commencement timeline (2025-06-19 Royal Assent, 2026-02-05 Stage 3, 2026-06-19 complaints-procedure duty); £17.5M PECR ceiling; new 7th lawful basis; Article 22 narrowing; DSAR proportionality + stop-the-clock.
- **NEW** `references/biz/ico-enforcement-reality.md` — descriptive provenance for the broad GDPR hard-block (PECR-direct-marketing-dominated enforcement pattern for sub-£1M-turnover private companies, Apr 2024 – Apr 2026); non-normative disclaimer up front.
- **NEW** `references/biz/open-template-anchors.md` — oneNDA / oneSaaS / oneDPA / Kindrik / ai-legal-claude index with license obligations; CC BY-ND 4.0 verbatim rule for oneNDA documented as load-bearing.
- **NEW** `crates/sldo-install/tests/e2e_biz_a_m2.rs` — 11 structural-contract tests, all green.
- **NEW** `docs/verify/biz-a-m2-smoke.md` — manual smoke checklist with 6 fixtures (R&D claim, VAT triage, HMRC filing hard-block, GDPR cross-skill routing, cross-skill predicate consistency, non-UK jurisdiction).
- **UPDATED** `docs/ARCHITECTURE.md` — added `slo-accounting` row to the skill table.
- **UPDATED** `docs/RUNBOOK-BIZ-SKILL-PACK-A.md` Milestone Tracker — M2 status `done`.

## Design decisions and why

- **Per-skill routing override pattern documented in `jurisdiction-uk.md`.** The triage-gate.md `route_to: lawyer` field for `gate-1-regulated` was originally one-size-fits-all. M2 implementation surfaced a real case: HMRC matters fire gate-1, but the right professional is the accountant, not a lawyer. Two approaches: (a) refine triage-gate.md to support per-regulator routing tables (would change M1's stable interface); (b) document routing as a per-skill override in skill prose with the regulator index in `jurisdiction-uk.md`. Chose (b) — preserves M1's predicate-id immutability invariant; keeps per-skill domain expertise in the skill prose where it belongs.
- **`ico-enforcement-reality.md` non-normative disclaimer is load-bearing.** The doc documents that ICO enforcement against sub-£1M-turnover private companies is PECR-direct-marketing-dominated, not Article-13-dominated — which is a defensible counter-argument for a NARROWER GDPR posture than the locked broad block. To prevent future readers (or a future agent doing skill drift) from interpreting the descriptive content as authorization to weaken gate-4, the disclaimer is mandatory and the test asserts forbidden phrases are NOT in the doc. Test enforcement caught two self-trips during M2 implementation (see "Mistakes made" below) — proves the test is doing its job.
- **`artifact-schema.md` ships in M2 with the per-category default-tier mapping but the `/slo-verify` PII scan integration is deferred.** The schema is the forward contract; the runtime scan lands when there's a real PII-shaped artifact to scan against (Runbook B1 M1 `/slo-talk-to-users`).
- **`jurisdiction-uk.md` UK regulator table is evolving, not stable.** Adding a new regulator does not require `/slo-architect` re-pass; it's per-milestone discipline with a `/slo-critique` security-persona review of the new row. This balances adoption velocity (new advisor skills can add their own regulators as they ship) against the load-bearing four-predicate-id schema (which IS stable).

## Course corrections taken in flight

- **Removed forbidden-phrase example from `ico-enforcement-reality.md`'s disclaimer paragraph.** First test pass failed `ico_enforcement_reality_doc_does_not_contradict_gate_4` because the disclaimer paragraph quoted a forbidden phrase as an example ("e.g., 'narrow gate-4 to PECR direct-marketing only'"). Circular self-trip. Fixed by removing the explicit example and pointing readers to the test source as the canonical forbidden-phrase list.
- **Reworded "relax gate-4-gdpr-document" → "weaken `gate-4-gdpr-document`" in ico-enforcement-reality.md's "What this file is NOT" section.** The original phrasing was a legitimate negation ("This file is NOT a recommendation to relax gate-4...") but the test substring-match for "relax gate-4" caught it regardless. Two fix options: refine the test to recognize negation context (complex) or rephrase the doc (trivial). Chose the doc fix.
- **Added explicit `references/biz/templates/onenda-uk.md` path reference in `open-template-anchors.md`.** First test pass failed `open_template_anchors_documents_onenda_license` because the doc cited the test name and the URL but not the file path. Fixed by adding the path to the oneNDA row.

## Mistakes made

- **Wrote the test with substring-match forbidden phrases, then triggered them in my own provenance prose.** The substring-match approach is the simplest test design but is brittle when the doc itself needs to discuss the rule it's enforcing. Lesson: when authoring docs that reference a forbidden-phrase test, write the doc FIRST with the rules clear, then write the test to enforce; don't write the test first and let the test drive the prose.
- **Two failures both about my own circular-reference pattern.** Suggests a class issue: meta-discipline tests (test enforces "this doc must not say X"; doc explains "the test enforces that this doc must not say X") need careful authoring. Future option: split such tests into a fingerprint that excludes the disclaimer paragraph from the substring scan, OR keep the rule purely in test source and don't name it in the doc.

## Root causes

- **Test-and-doc co-authoring without intermediate review.** I wrote the doc with examples of the rule, then wrote the test that enforces the rule, didn't notice the circular trip until cargo test ran. One iteration cycle. Fix: when tests enforce content rules, run them mid-authoring not at end.

## What was harder than expected

- **The cross-skill citation test in M2 was easier than expected.** Anticipated complexity around walking SKILL.mds and asserting predicate-id presence; actual implementation is `for skill_name in ADVISOR_SKILLS { assert!(read(...).contains(pid), ...) }` — a few lines. The hard work was M1's predicate-id discipline; M2 just consumes the contract.
- **The artifact-schema's default-tier mapping required a domain-by-domain pass** through all 15 skills to decide which artifact categories carry confidential vs public tier. Generators are mostly public (`/slo-launch`, `/slo-pricing`, etc.), but `/slo-talk-to-users`, `/slo-cofounder`, `/slo-hire`, `/slo-founder-check` are confidential because they involve real persons. This taxonomy work doesn't have a precedent in the existing slo-security-embedding pack to copy from.

## What was easier than expected

- **Replicating the advisor pattern in `/slo-accounting/SKILL.md`.** The pattern from M1's `/slo-legal` SKILL.md transfers cleanly. The structural-contract test enforces predicate-id citation, not predicate-prose copying — leaves the skill author free to explain each gate in the skill's domain context (HMRC routing for accounting; lawyer routing for legal) without being forced into identical wording.
- **The `references/biz/` discovery-exclusion regression test passes for free.** `crates/sldo-install/src/install.rs:44-71` is unchanged and does what M1's design committed to.

## Recommendations for M3

- **Author M3's `/slo-equity/SKILL.md` second, not first.** Author `references/biz/hmrc-vcm-index.md` first (M3's reference-file dependency), then write the skill that cites it. M2's order (some references, then skill, then more references) was OK but a strict reference-first order would have caught one missed citation earlier.
- **Re-run M1 + M2 tests as part of M3 pre-flight.** The cross-skill citation test in M3 will extend to assert all THREE advisor SKILL.mds (slo-legal, slo-accounting, slo-equity) cite all four predicate IDs; running M1 + M2 first ensures no regression slipped in during M3 work.
- **Consider whether `/slo-equity` needs its own per-skill routing override.** SEIS / EIS work crosses HMRC (accountant) and FCA (lawyer for the regulated-activity question). M3's SKILL.md should clarify the routing in the same pattern M2 used.
- **The DUAA dates in `ico-duaa-index.md` are temporally fragile.** When 2026-06-19 passes (the complaints-procedure-duty date), the doc's "becomes effective" wording needs to flip to "is in effect". Add a `/loop @yearly` schedule (or a calendar-based `/schedule`) for review.

## Changes to ARCHITECTURE.md

- Added `slo-accounting` to the skill table.

## Changes to runbook tracker

- M2 status `not_started` → `done`. Started 2026-04-25, completed 2026-04-25.
