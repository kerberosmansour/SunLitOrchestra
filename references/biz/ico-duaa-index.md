---
name: ico-duaa-index
created: 2026-04-25
retrieved: 2026-04-25
status: evolving — refresh annually or when ICO publishes new commencement notices
source_primary: https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/the-data-use-and-access-act-2025-duaa-summary-of-the-changes/
source_statute: https://www.legislation.gov.uk/ukpga/2025/18
purpose: |
  Snapshot of the UK Data (Use and Access) Act 2025 commencement timeline, scope changes, and PECR ceiling — the moving baseline that `/slo-legal triage` (gate-4-gdpr-document) cites in GDPR-routing prose. NOT a substitute for ICO guidance; this file is a citation index, not legal advice.
---

# DUAA 2025 — commencement timeline + key changes (snapshot)

> Retrieval date: **2026-04-25**.
> Sources: ICO summary page + legislation.gov.uk + commentary from Clifford Chance (Feb 2026) and Hogan Lovells (2025).

## Commencement timeline

| Date | Event | Source |
|---|---|---|
| **2025-06-19** | DUAA 2025 receives Royal Assent (statute enacted) | https://www.legislation.gov.uk/ukpga/2025/18 |
| **2026-02-05** | DUAA Stage 3 commencement — most data-protection provisions take effect | https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/the-data-use-and-access-act-2025-duaa-summary-of-the-changes/ |
| **2026-06-19** | Complaints-procedure duty for all controllers becomes effective | ICO summary page (above) |

## Key changes (UK GDPR + PECR)

### New 7th lawful basis: "Recognised Legitimate Interests"

A new Article 6(1)(eaa) added to UK GDPR, codifying specific examples that no longer require a balancing test:

- **Direct marketing** (with PECR consent rules still applying)
- **Intra-group administrative purposes**
- **Network and information security**
- **Detecting / preventing fraud or unlawful conduct**
- **Safeguarding vulnerable individuals**

Implication for `/slo-legal triage`: This narrows but does NOT eliminate the legitimate-interest analysis. Founders who tried to claim broad "legitimate interest" pre-DUAA may now have a clearer basis for these specific use-cases — but ALL OTHER legitimate-interest uses still require the three-part balancing test. The broad GDPR hard-block on `draft` (locked 2026-04-25) stands: the DUAA's enumeration is precisely WHY drafting a privacy notice without solicitor / DPO review is dangerous — getting the lawful basis wrong is now more concrete and audit-able.

### Article 22 narrowed to special-category data

Pre-DUAA Article 22 prohibited solely-automated decisions producing legal / similarly significant effects on data subjects. DUAA narrows the prohibition to special-category-data automated decisions only. Other automated decisions are now permitted with safeguards (transparency + meaningful human review on request).

Implication for `/slo-legal triage` and `/slo-product` (Runbook B1 M3): Founders building automated decision systems (scoring, eligibility, ranking) on non-special-category data now have more legal headroom — but the safeguards are still load-bearing and any uncertainty should route to a DPO.

### PECR fine ceiling raised £500k → £17.5M / 4% global turnover

Direct marketing / electronic communications enforcement (PECR) was capped at £500k pre-DUAA. The ceiling is now aligned with UK GDPR at £17.5M or 4% of global turnover, whichever is greater.

Implication for `/slo-legal triage` (gate-4) and `/slo-marketing` (Runbook B1 M4): The PECR enforcement risk for B2C startups doing email / SMS / push-notification marketing is now MATERIALLY higher. PECR-direct-marketing is the ICO's most active enforcement surface for sub-£1M-turnover private companies (per `references/biz/ico-enforcement-reality.md`); the ceiling raise puts a credible tail on the risk. `/slo-marketing b2c` MUST route any direct-marketing implementation question to triage with DPO routing.

### DSAR ("data subject access request") proportionality + "stop the clock" codified

Pre-DUAA, DSAR responses were due in one month with limited grounds for refusal / extension. DUAA codifies:

- **Proportionality**: controllers may take "reasonable and proportionate" search effort (statutory test, not previously codified).
- **"Stop the clock"**: when a controller is awaiting clarification from the requester, the one-month timer pauses.
- These reduce the operational burden on small controllers facing wide DSARs.

Implication for `/slo-legal triage`: DSAR-handling questions still route to triage, but the new codification means a DPO consultation is more likely to produce a defensible "narrow the request" response than pre-DUAA.

### Complaints-procedure duty (effective 2026-06-19)

All controllers must have a documented complaints procedure that data subjects can use. This is a NEW operational obligation; small controllers without one are non-compliant once the date passes.

Implication for `/slo-legal triage`: From **2026-06-19**, the gate-4 routing for any GDPR matter MUST cite this duty as a baseline — every controller, regardless of size, needs a complaints procedure. The triage memo points the founder to ICO guidance + DPO consultation for procedure design.

## What this file is NOT

- This file is NOT legal advice. It is a citation index summarising publicly-published ICO and legislation.gov.uk content as of the retrieval date.
- This file is NOT authorization to relax the broad GDPR hard-block on `draft` mode. The broad block (locked 2026-04-25) is reversible only via fresh `/slo-architect` pass with new evidence — see `references/biz/ico-enforcement-reality.md` for the descriptive evidence, and `docs/slo/design/biz-skill-pack-overview.md` for the locked-decision provenance.
- This file is NOT exhaustive on DUAA. Many other provisions exist (smart data schemes, digital identity verification trust framework, online safety amendments) that are out of scope for the biz skill pack.

## Refresh cadence

- **Annual**: re-retrieve and update `retrieved:` frontmatter.
- **Triggered**: re-retrieve immediately when ICO publishes a new DUAA-related enforcement guidance update or commencement notice.
- **Recommended `/loop` schedule**: see the `cost-baseline-jpp-law-2026.md` annual-refresh schedule for the pattern; this file's refresh runs alongside.
