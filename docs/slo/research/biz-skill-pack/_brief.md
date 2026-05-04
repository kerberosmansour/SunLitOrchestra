# Research brief — biz-skill-pack

## Wedge (one sentence)

Ship `/slo-legal` v1 — NDA + Contractor SOW + IP Assignment + Terms & Conditions templates plus an advisor-pattern triage gate that refuses to `draft` when the matter is regulated, deal value > £5,000, the counterparty has their own lawyer, or the document is GDPR-related — as Milestone 1 of a 12-skill business-side SunLitOrchestra (SLO) skill pack that mirrors the existing engineering pipeline (`/slo-ideate` → `/slo-research` → `/slo-architect` → `/slo-plan` → `/slo-critique` → `/slo-execute` → `/slo-verify` → `/slo-retro` → `/slo-ship`).

## Target user (one sentence)

UK-based seed-stage technical founder already running SunLitOrchestra for the engineering side of their startup who hits a legal / business-ops wall (e.g. needing an NDA plus contractor IP-assignment before a Tuesday hiring conversation, or wondering whether to incorporate now versus later) and currently has zero in-pack scaffolding to handle the company-around-the-product side.

## Pack scope context (so research stays grounded)

12 skills across 3 runbooks:

- Runbook A — advisor cluster (4): `/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`. Each operates with `draft` + `translate` + `triage` + `prepare` modes.
- Runbook B — customer-facing generators (5): `/slo-talk-to-users`, `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`. One-shot artifact each.
- Runbook C — team (3): `/slo-cofounder`, `/slo-hire`, `/slo-founder-check`. Generators.

UK first; US/EU jurisdictions deferred. Advisor skills hard-block `draft` for: FCA / MHRA / ICO / health / financial-services regulated matters, deal value > £5,000, counterparty has a lawyer, and ALL GDPR-related documents (privacy notice, ROPA, DPA, internal data-protection policies — translate/triage only, never draft).

Reference price baseline being used for ROI claims: Russell Cooke "Services for Start Ups – Price List (General) (2026-2027)" — NDA £750, Standard Contractor Agreement £1,450, IP Assignment £750, T&Cs for sale of goods/services £2,750, GDPR Package (basic) £1,850, GDPR Package (full) £4,950, Shareholders Agreement (cofounders) £2,950, Shareholders Agreement (SEIS investment) £5,950, Articles of Association £1,350.

## Five specific, answerable research questions

1. **Buy vs build for `/slo-legal`** — direct competitors in the UK legal-doc-as-a-service market: Lawpath UK, Genie AI, SeedLegals, Rocket Lawyer UK, Sparqa Legal, Farillio. Plus US-side reference points: Stripe Atlas, Clerky. For each: monthly/per-doc price, which of the v1 documents (NDA, contractor SOW, IP assignment, T&Cs) they cover for UK-incorporated companies, and whether they bundle a lawyer-review step. Goal: identify the price/feature point at which an in-Claude `/slo-legal` with triage gate beats the cheapest substitute.

2. **UK GDPR enforcement reality for sub-seed companies** — ICO (Information Commissioner's Office) published enforcement actions, monetary penalty notices, and reprimands from the last 24 months (2024-04 to 2026-04) where the subject was a private company with under £1M turnover or under 50 employees. Specifically: how many enforcement actions targeted privacy-notice / lawful-basis / Article 13 transparency failures (vs breach notification, vs marketing/PECR violations)? Is the proposed "hard-block draft for ALL GDPR documents" gate justified by realistic enforcement risk for early-stage startups, or is it over-conservative?

3. **SEIS / EIS qualification rules and founder responsibility** — HMRC Advance Assurance process for SEIS/EIS, founder/director obligations to maintain qualifying status post-investment, the top 5 ways a company inadvertently voids investor tax relief retroactively (e.g. wrong share class, value extraction, qualifying-trade drift, control changes), and what specific actions `/slo-fundraise` triage gate should force BEFORE any term-sheet drafting (e.g. "have you applied for advance assurance?", "what's your qualifying trade per HMRC ITA 2007 Pt 5A?"). Cite HMRC manuals VCM31000 / VCM34000 series if relevant.

4. **UK employment law minimums for first hire and IR35 status determination** — minimum statutory protections for an employer's first UK employee (notice periods per Employment Rights Act 1996 s86, statutory holiday per Working Time Regulations 1998, SSP, NI/PAYE registration, right-to-work checks, pension auto-enrolment per Pensions Act 2008). Plus the IR35 / off-payroll-working status-determination factors (HMRC's CEST tool criteria, mutuality of obligation, control, substitution) at the level needed for `/slo-hire` and `/slo-legal` to triage "is this person a contractor or an employee?" without misclassifying. What absolutely must hard-block to a lawyer + accountant?

5. **Jurisdiction expansion — marginal cost of US/EU after UK is shipped** — for a template-plus-triage skill pack, what fraction of UK-template prose, triage logic, and reference baseline can be reused for US (Delaware C-corp, common contractor IP defaults under US copyright law's "work made for hire" doctrine) and EU (multiple member states, GDPR shared but employment law per-state)? Is the right design "UK + US + EU as parallel `references/biz/jurisdiction-<code>.md` matrices" or "UK now, deal with the rest as a v2 architecture pivot"? Cite any prior art on multi-jurisdiction legal templating from Stripe Atlas, SeedLegals' international expansion, or Clerky.

## Out of scope (to keep the dossier focused)

- API / SDK reference for any specific tool — that's `chub` / `/get-api-docs` territory, not this dossier.
- Restating the idea doc — synthesis must be about external findings, not the pitch.
- US / EU detail beyond Q5's marginal-cost framing — full multi-jurisdiction research is deferred until UK ships.
