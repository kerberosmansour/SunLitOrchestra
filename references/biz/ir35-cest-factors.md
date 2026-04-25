---
name: ir35-cest-factors
created: 2026-04-25
retrieved: 2026-04-25
status: evolving — refresh annually + on each CEST tool release + on each landmark IR35 case
audience: /slo-legal triage (contractor vs employee), /slo-fundraise (qualifying-employee context for SEIS/EIS), future Runbook C M2 /slo-hire
purpose: |
  Three-factor IR35 status-determination reference (substitution, MOO, control), HMRC CEST tool documentation including the April 2025 refresh, and the documented CEST limitation around mutuality of obligation (MOO blind spot per HMRC's reading of PGMOL v HMRC [2024] UKSC 29).
sources:
  - https://www.gov.uk/guidance/check-employment-status-for-tax
  - https://www.gov.uk/guidance/understanding-off-payroll-working-ir35
  - https://www.twobirds.com/en/insights/2025/uk/spot-the-difference-a-closer-look-at-hmrcs-updated-cest-tool
  - https://www.contractoruk.com/ir35furtherreading/ir35-mutuality-obligation-202627-explainer-contractors
  - https://www.kingsbridge.co.uk/blog/contractors/ir35/hmrc-cest-update-april-2025-review/
---

# IR35 / CEST factors — citation index

> Retrieval date: **2026-04-25**.
> NOT legal / tax advice; NOT a substitute for a tax specialist's status determination on a specific contractor engagement.

## The three primary factors

UK IR35 status determination turns on three primary factors. ALL three should be evaluated; no single factor is dispositive in isolation.

### 1. Right of Substitution (the strongest outside-IR35 signal)

The contractor has the right to send a substitute to perform the work. **April 2025 CEST refresh: substitution must be UNRESTRICTED and GENUINELY EXERCISABLE** — recommending a replacement no longer counts as substitution; the contractor must have the unilateral right to send someone else.

- Authority (statutory): off-payroll working rules; case-law line *Ready Mixed Concrete v Minister of Pensions* [1968].
- Practical test: is there a substitution clause in the contract? Does it require the engager's approval (which would weaken substitution)? Has substitution actually been exercised in practice?
- **Hard-block-to-lawyer surface for `/slo-legal triage` and `/slo-hire`**: absent or sham substitution clause → reclassify to lawyer for status determination.

### 2. Mutuality of Obligation (MOO) — CEST blind spot

Mutual obligations exist when the engager must offer work AND the contractor must accept it. Strong MOO points to employment.

- Authority: case-law line; key 2024 case is **PGMOL v HMRC [2024] UKSC 29** which held that MOO exists in any contract for services (i.e., the basic obligations to do work for payment are inherently present).
- **CEST limitation**: HMRC's reading of PGMOL v HMRC means CEST does NOT separately weigh MOO — the tool treats MOO as constitutionally present in any engagement. This is a documented blind spot per Bird & Bird's "Spot the Difference" April 2025 review and ContractorUK's 2026/27 explainer.
- **Implication for triage**: CEST output is NECESSARY but NOT SUFFICIENT. If CEST returns "outside IR35" but the engagement has strong other employment-shaped factors (full-time, exclusive, integrated), the determination needs a tax specialist's review.

### 3. Control

The engager has the right to control the contractor's work — what is done, how, when, where.

- Strong control (engager directs how the work is done, sets hours, requires office attendance) → employment.
- Weak control (contractor exercises professional discretion on how, when, where) → contracting.
- **Practical surface**: a contractor who works on-site at engager premises during fixed hours using engager equipment scores HIGH on control even if the contract calls them a contractor.

## CEST tool — the April 2025 refresh

HMRC's Check Employment Status for Tax (CEST) tool (https://www.gov.uk/guidance/check-employment-status-for-tax) was refreshed effective 30 April 2025.

- **Restructured into ~6 sections** with an explicit gating question that requires a contract to exist.
- **Substitution section tightened** — must be unrestricted and genuinely exercisable (per Section 1 above).
- **MOO section added** but NOT weighted in the determination algorithm (per HMRC's PGMOL reading).
- **~20% indeterminate rate** (34 of 72 outcome routes return "unable to determine"). This is structurally high and is itself a routing signal: an indeterminate outcome means a tax specialist is required.
- HMRC explicit position: underlying technical principles unchanged from pre-refresh CEST.

## Hard-block-to-lawyer surface (the seven factors)

When `/slo-legal triage`, `/slo-fundraise`, or `/slo-hire` evaluate a contractor relationship, ANY of the following triggers a hard-block-to-lawyer (or lawyer + accountant for tax exposure):

1. **Absent or sham substitution clause** in the contract.
2. **Contractor substantially full-time** (e.g., > 4 days/week on the engagement).
3. **Exclusive engagement** (contractor cannot work for competitors / other clients).
4. **CEST result "employed" or "unable to determine"** — the indeterminate output is itself the routing signal.
5. **Engagement >6 months** with rolling renewals (signals ongoing relationship rather than discrete project).
6. **Contractor uses engager equipment** AND attends engager premises (control + integration).
7. **Contractor takes direction** rather than exercises professional discretion.

Any ONE of these firing routes the founder to a tax specialist or employment lawyer.

## Small-company IR35 thresholds — change effective 6 April 2026

For end-clients (engagers), the off-payroll-working "small company" thresholds change:

- Turnover: £10.2M → **£15M**.
- Balance sheet: £5.1M → **£7.5M**.
- Headcount: 50 (unchanged).

For seed-stage founders, the company remains "small" under both old and new thresholds, so the contractor-self-assessment regime (rather than engager-side determination) continues to apply. Sources: Greenberg Traurig (March 2026), gov.uk off-payroll-working guidance.

## Triage gate prose for `/slo-legal triage` (contractor vs employee)

When the founder asks "is this person a contractor or an employee?", the skill MUST run through the three-factor + seven-trigger evaluation BEFORE producing a draft contractor SOW. If ANY hard-block surface fires, route to lawyer + accountant; otherwise the SOW is permitted but with a "review against the seven IR35 factors before signing" reminder.

## What this file is NOT

- NOT a CEST tool replacement. Founders MUST run CEST on each engagement and treat the output as one input alongside the seven-trigger evaluation.
- NOT legal / tax advice. Any genuine status determination requires a tax specialist.
- NOT exhaustive on IR35. Other factors (integration into the engager's organisation, financial risk, provision of own equipment, basis of payment) inform the determination and route to specialist when uncertain.

## Refresh cadence

- **Annual**: re-retrieve CEST page + commentary.
- **Triggered**: re-retrieve when HMRC issues a new CEST refresh OR when a UK Supreme Court / Court of Appeal IR35 case is decided.
