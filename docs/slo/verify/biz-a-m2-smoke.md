# Manual smoke-test checklist — Runbook A M2 (`/slo-accounting` + M2-tier shared references)

> Created: 2026-04-25
> Runbook: [docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md](../completed/RUNBOOK-BIZ-SKILL-PACK-A.md) Milestone 2
> Skill under test: [skills/slo-accounting/SKILL.md](../../skills/slo-accounting/SKILL.md)
> M2-tier references: artifact-schema.md, jurisdiction-uk.md, ico-duaa-index.md, ico-enforcement-reality.md, open-template-anchors.md.

## Prerequisites

- [ ] M1 smoke tests all passed and recorded in `docs/slo/verify/biz-a-m1-smoke.md`.
- [ ] Baseline tests green: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`.
- [ ] Structural tests green: `cargo test -p sldo-install --test e2e_biz_a_m1 --test e2e_biz_a_m2`.
- [ ] `sldo-install --dry-run` shows `slo-legal` and `slo-accounting` discovered (16 skills total); no `references/biz/` entry.

## Smoke test 1 — Happy path: `/slo-accounting draft brief-the-accountant` for an R&D claim

Fixture: 6 months of FTE engineering spend (£120k total across 4 engineers), plausible R&D narrative ("novel ML training pipeline for X domain"), claim period 2026-04 → 2026-09, expected refund £30k+.

Invocation: `/slo-accounting draft brief-the-accountant` with the fixture context.

Verify:

- [ ] Skill REFUSES the brief-the-accountant draft because gate-2 fires (the £30k+ refund crosses the £5k deal-value threshold).
- [ ] OR — if the founder reframes as "I want a brief for my accountant about whether to claim, not a claim narrative" — the skill produces the memo at `docs/biz/accounting/brief-the-accountant-rd-claim-2026-04-25.md`.
- [ ] Frontmatter (when memo is produced) carries:
  - [ ] `tier: confidential`.
  - [ ] `skill: slo-accounting`.
  - [ ] `mode: draft`.
  - [ ] `jurisdiction: uk`.
  - [ ] `triage_gate_passed:` (correct based on actual evaluation).
  - [ ] `lawyer_review_recommended: false` (this is accountant territory) BUT body header reads "**ACCOUNTANT REVIEW RECOMMENDED**".
- [ ] Body structured for accountant call: numbers, qualifying-arguments, open questions.

## Smoke test 2 — Routing: `/slo-accounting triage "registering for VAT"`

Fixture: founder asks if/when to register for VAT.

Verify:

- [ ] Output landed at `docs/biz-public/accounting/triage-vat-registration-2026-04-25.md`.
- [ ] `tier: public`.
- [ ] `mode: triage`.
- [ ] Routes to ACCOUNTANT (not lawyer — verify the body explicitly says "your accountant", not "your solicitor").
- [ ] Cites the £90k VAT registration threshold (or current threshold from gov.uk; verify retrieval-date stamped if quoted).
- [ ] Body cites `gate-1-regulated` because HMRC is the regulator (per-skill override to accountant routing per `references/biz/jurisdiction-uk.md` UK regulator index).

## Smoke test 3 — HMRC filing hard-block: `/slo-accounting draft tax-return`

Fixture: any context (e.g., end-of-tax-year corp tax return).

Verify:

- [ ] Skill REFUSES to draft.
- [ ] Routes to triage citing `gate-1-regulated` (HMRC is the regulator; tax returns are regulated filings).
- [ ] Output recommends accountant with a "what to bring to your accountant" briefing checklist.
- [ ] Body explicitly states "drafting actual HMRC filings is accountant-territory; this skill produces only briefs and triage memos".

## Smoke test 4 — Cross-skill GDPR routing: `/slo-accounting draft privacy-notice`

Fixture: founder wants a privacy notice covering accounting data (customer financial records).

Verify:

- [ ] Skill REFUSES to draft (gate-4-gdpr-document fires).
- [ ] Output landed at `docs/biz-public/accounting/triage-privacy-notice-2026-04-25.md`.
- [ ] Routes to DPO + ACCOUNTANT (dual discipline — DPO for the GDPR question, accountant for the data-classification question).
- [ ] Body cites `gate-4-gdpr-document` AND references `references/biz/ico-duaa-index.md` for DUAA dates.
- [ ] Body explicitly states GDPR documents are unconditionally hard-blocked — locked decision 2026-04-25.

## Smoke test 5 — Cross-skill predicate consistency

Verify: `/slo-legal triage` and `/slo-accounting triage` for the SAME fact pattern (e.g., "deal worth £20k where the other side has a lawyer") fire the SAME predicates (`gate-2-deal-value-over-5k` and `gate-3-counterparty-has-lawyer-or-their-paper`) — but route differently:

- [ ] `/slo-legal triage` routes to LAWYER for the contractual position.
- [ ] `/slo-accounting triage` routes to LAWYER + ACCOUNTANT (lawyer for the deal, accountant for any tax implications).
- [ ] Both cite the same two predicate IDs.
- [ ] No drift in predicate-id wording between the two outputs.

## Smoke test 6 — Non-UK jurisdiction: `/slo-accounting draft brief-the-accountant --jurisdiction us`

Verify:

- [ ] Skill emits the canonical "v1 supports UK only; US/EU is a v2 architectural pivot" error from `references/biz/jurisdiction-uk.md`.
- [ ] No artifact written.
- [ ] Skill suggests engaging a US-jurisdiction CPA / tax advisor.

## Notes captured during the smoke test

(Record any deviation, ambiguity in SKILL.md prose, or improvements that should land in a follow-up.)

- _<empty until smoke run>_
