---
synthetic: true
non-normative: true
abbreviates: slo-gtm
tier: public
---

# Synthetic GTM Strategy — Acme Widget Platform (UK)

> **Synthetic, non-normative.** Names, ICPs, and channel choices are invented. Real GTM artifacts live in `docs/biz-public/gtm/strategy.md` (placeholder) or `docs/biz/` (confidential drafts).

## ICP (3-segment cap)

| Segment | Description | Sizing | Wedge |
|---|---|---|---|
| Solo developers (UK) | Indie devs shipping side projects to GitHub | est. 200k UK | Free tier with no signup; usage-based pricing kicks in at 1k events/mo |
| Mid-market SaaS engineering teams | 10–50 engineer SaaS companies on AWS, Postgres, Rust/Go | est. 4k UK companies | Compliance-shaped onboarding; SOC 2 starter pack at month 3 |
| FinTech tech leads (regulated UK) | FCA-authorized firms with FCA SYSC 8 outsourcing rules | est. 600 UK firms | UK-data-residency option; FCA reporting hooks; deferred to Q4 |

## Motion choice

**PLG primary; sales-led for FinTech segment.**

Rationale: solo + mid-market segments self-serve; FinTech requires named-rep + procurement + DPA. Hybrid is the answer; not three independent motions.

## Channel strategy

| Channel | Segment | Hypothesis |
|---|---|---|
| Hacker News + r/rust + r/programming | Solo devs | Demo posts on Saturdays; expect 200 signups per top-3 post |
| Conference + content (Rust Nation UK, FOSDEM) | Mid-market | Talk-driven inbound; 50 leads/conference |
| Inbound from FCA-shaped content (SYSC 8 outsourcing white paper) | FinTech | 3 qualified meetings per month; long-cycle |
| Direct cold outbound | _routed to /slo-legal triage for PECR considerations under DUAA Stage 3_ | Per UK PECR + DUAA 2025 — corporate B2B email may be lawful basis but content rules apply; legal triage required before sending |

## KPI alignment

PM-side metrics (DAU / activation / retention) live in `slo-product metrics`. Financial KPIs (CAC / LTV / NDR / burn multiple) live in `slo-metrics`. This GTM artifact tracks:

- Top-of-funnel signups by source.
- Activation rate (first 1k events sent within 7 days).
- Conversion to paid by segment.

## Out of scope for this artifact

- Pricing tier design (see `slo-pricing`).
- Sales playbook detail (see `slo-sales-funnel` for outbound funnel math).
- Pitch narrative (see `slo-fundraise` for AA-eligible storytelling).
