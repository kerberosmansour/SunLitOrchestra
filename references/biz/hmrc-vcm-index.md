---
name: hmrc-vcm-index
created: 2026-04-25
retrieved: 2026-04-25
status: evolving — refresh when HMRC publishes Venture Capital Schemes Manual updates (https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/updates)
audience: /slo-equity (M3) + /slo-fundraise (M4)
purpose: |
  Citation index for HMRC Venture Capital Schemes Manual — VCM34080 (control / disqualifying arrangements), VCM3000 (excluded activities), VCM31000 (SEIS income tax relief), plus the SEIS / EIS Advance Assurance process anchor. Every cited paragraph carries a source URL + retrieval date so downstream advisor-skill outputs can be audited.
sources:
  - https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual
  - https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm34080
  - https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm3000
  - https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm31000
  - https://www.gov.uk/guidance/venture-capital-schemes-apply-for-advance-assurance
  - https://www.gov.uk/guidance/venture-capital-schemes-apply-for-the-enterprise-investment-scheme
---

# HMRC Venture Capital Schemes Manual — citation index

> Retrieval date: **2026-04-25**.
> NOT legal / tax advice; NOT a substitute for HMRC guidance or accountant review.
> This file is a citation index summarising the published HMRC manual paragraphs that `/slo-equity` (M3) and `/slo-fundraise` (M4) cite when triaging SEIS / EIS qualification questions.

## Top retroactive-disqualification triggers (cited by `/slo-fundraise` triage gate)

When a founder asks about SEIS / EIS qualification, the advisor skill checks the founder's situation against the following triggers BEFORE any Advance Assurance application or term-sheet drafting. Each trigger fires `gate-1-regulated` (HMRC) and routes to accountant + (where indicated) lawyer.

### Trigger 1 — Breach of control / independence (VCM34080)

The qualifying company must NOT be a 51%-owned subsidiary of another company AND must NOT be under the control of another company or connected persons.

- Authority: **VCM34080** ("Disqualifying arrangements; control and connected persons; companies in administration / insolvency").
- URL: https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm34080
- Triage routing: lawyer (corporate structure) + accountant (tax position).
- Retrieved: 2026-04-25.

### Trigger 2 — Disqualifying arrangements (ITA07 s257HJ(1))

Any scheme or agreement under which the company would breach independence at any point during "Period A" (typically share-issue date to 3 years after).

- Authority: **VCM34080** (same paragraph).
- Statutory citation: ITA07 s257HJ(1).
- Triage routing: lawyer + accountant.

### Trigger 3 — Preferential rights on share class

Ordinary shares granted preferential rights (via articles of association or shareholders agreement drafting) — even WITHOUT formal reissue — lose SEIS / EIS qualification.

- Authority case law: **Abingdon Health Limited v HMRC [2016] TC 05525**; **Flix Innovations** line.
- URL: https://www.rossmartin.co.uk/companies/seis-eis/2501-eis-preferential-rights-acquired
- Triage routing: lawyer (drafting review) + accountant (tax impact).
- Triage prose MUST flag this trigger explicitly when reviewing any cofounder split, vesting schedule, or articles drafting that touches share rights.

### Trigger 4 — Qualifying-trade drift into excluded activities (VCM3000 series)

The company's trade must not include "excluded activities" (e.g., dealing in land / commodities, banking, insurance, legal / accountancy services, property development, hotels). The list is statutory and detailed; subtle drift (e.g., a SaaS company that becomes >20% revenue from a property-management feature) can disqualify.

- Authority: **VCM3000** ("Excluded activities: contents") with sub-paragraphs VCM3010 through VCM3070+.
- URL: https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm3000
- Triage routing: accountant (with specialist VCT advisor for borderline cases).

### Trigger 5 — Value extraction / non-independent transactions

Loans, benefits, related-party transactions to investors AFTER the share issue can disqualify by breaching the no-value-received rule for the investor.

- Authority: **VCM31000** ("SEIS income tax relief: contents") sub-paragraphs.
- URL: https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm31000
- Triage routing: accountant (founders typically).

## Advance Assurance — process and lead time

- **Process**: Apply via gov.uk SEIS / EIS Advance Assurance form. https://www.gov.uk/guidance/venture-capital-schemes-apply-for-advance-assurance
- **HMRC internal target**: ~15 working days.
- **Realistic end-to-end**: 4–6 weeks (HMRC follow-up questions are common).
- **2022-23 baseline**: ~26% of applications exceeded 30 days.
- **SeedLegals-prepared applications**: typically clear in 2–3 weeks after a 1–2 week pre-submission review, per their published help docs.
- **Practical floor for `/slo-fundraise` triage gate**: apply for Advance Assurance **at least 6 weeks before term-sheet signature**. Earlier-than-needed AA is preferred to a tight-timeline scramble.

## Triage gate prose for `/slo-fundraise` (M4 will cite this section)

When `/slo-fundraise` is invoked for any term-sheet drafting, SAFE math, or pitch narrative, the skill MUST surface these questions BEFORE drafting:

1. **"Have you applied for Advance Assurance, and if so, when?"** — if not applied, hard-block term-sheet drafting; route to accountant immediately. AA must precede term-sheet by ≥ 6 weeks.
2. **"Are you a 51%-owned subsidiary or controlled by another company / connected persons (VCM34080)?"** — if yes, hard-block; route to lawyer + accountant for restructuring.
3. **"Have you audited your qualifying-trade status against VCM3000 in the last 12 months?"** — if not, warn; recommend pre-AA review with accountant.
4. **"Are any share rights preferential vs ordinary (Abingdon Health line)?"** — if yes (or unsure), hard-block; route to lawyer for drafting review.

## What this file is NOT

- NOT legal / tax advice.
- NOT exhaustive on SEIS / EIS qualification — many other detailed rules exist (gross asset test, employee-count test, age-of-trade limits, prior funding limits) that are out of scope for the M3 / M4 advisor surface and route directly to accountant + specialist VCT advisor when relevant.
- NOT a substitute for HMRC's manual or for accountant / specialist advice. The triggers above are the highest-frequency disqualification surfaces, not the complete set.

## Refresh cadence

- **Quarterly check** of https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/updates for material changes.
- **Annual** retrieval-date refresh.
- **Triggered**: re-retrieve immediately when a new SEIS / EIS budget announcement (Spring / Autumn statements) shifts qualifying parameters — e.g., the 2023 SEIS limits raise (annual investment £150k → £250k, age limit 2 → 3 years).
