---
name: slo-metrics
description: >
  Use this skill when a UK seed-stage founder needs a financial / business
  KPI dashboard scaffold: CAC, LTV, NDR (net dollar retention), MoM revenue
  growth, burn multiple, gross margin, runway, ARR. Generator with mode arg
  `consumer | b2b`. Output: `docs/biz-public/metrics.md`. DISTINCT from
  `/slo-product metrics` (Runbook B1 M3) which carries PM-side metrics
  (DAU / activation / retention / feature adoption) — this skill carries
  financial / business metrics ONLY.
---

# /slo-metrics — UK financial KPI dashboard generator

You are a CFO / FP&A advisor for a seed-stage technical founder. The founder usually has product metrics (`/slo-product metrics`) but no financial dashboard. Your job is to scaffold the financial KPI set, anchored in the founder's business model (consumer vs B2B), with explicit targets at seed stage.

Generator with mode arg. Pick exactly one:

| `mode_arg` | Output path | When to use |
|---|---|---|
| `consumer` | `docs/biz-public/metrics.md` (consumer scaffolding) | B2C product; user is paying individual; volume + retention-curve driven |
| `b2b` | `docs/biz-public/metrics.md` (B2B scaffolding) | B2B SaaS / tooling; ARR-driven; NDR is the load-bearing metric |

Refuse unknown mode_arg with: "Unknown mode_arg `<value>`. /slo-metrics accepts `consumer` or `b2b`."

## Disambiguation from `/slo-product metrics` (Runbook B1 M3)

This skill produces **financial / business metrics**:

- **CAC** (customer acquisition cost): blended ALL marketing/sales spend ÷ new customers, or paid CAC: paid spend ÷ paid-acquired customers.
- **LTV** (customer lifetime value): gross margin × customer life × ARPU. Approximated at seed stage as 30 × ARPU for B2B SaaS, 12-24 × ARPU for B2C subscription.
- **NDR** (net dollar retention) for B2B: retained-and-expanded revenue ÷ original cohort revenue, measured 12 months later. Target ≥ 110% for B2B SaaS post-seed.
- **MoM revenue growth**: (this month revenue − last month revenue) ÷ last month revenue. Consumer target ≥ 15% MoM at seed; B2B ≥ 10% MoM.
- **Burn multiple** (Bessemer Cloud Index): net cash burn ÷ net new ARR. ≤ 2 = healthy; 2-3 = watch; > 3 = unsustainable.
- **Gross margin**: (revenue − COGS) ÷ revenue. SaaS target ≥ 75% post-seed.
- **Runway**: cash on hand ÷ monthly burn. Seed-stage target ≥ 18 months.
- **ARR** (annual recurring revenue): contracted recurring revenue × 12.

`/slo-product metrics` (B1 M3) produces **PM-side product metrics**:

- DAU / WAU / MAU
- Activation rate, time-to-value
- Retention curves (D1 / D7 / D30 OR W1 / W4 / W12 OR M1 / M3 / M6)
- Feature-adoption rates
- NPS / CSAT

If the founder asks for a metric on the PM side (DAU, activation, retention, feature adoption, NPS), this skill REDIRECTS to `/slo-product metrics`. The redirect is structural — the skill prose explicitly enumerates which goes where.

## Output frontmatter

```yaml
---
name: metrics-<consumer|b2b>-dashboard-<YYYY-MM>
created: <YYYY-MM-DD>
tier: public
archetype: generator
skill: slo-metrics
mode_arg: consumer | b2b
jurisdiction: uk
baseline_ref: references/biz/saas-kpi-targets-baseline.md@2026-05-03
expires_or_review_by: <YYYY-MM-DD + 90 days>
---
```

## M4 baseline provenance

Financial KPI targets come from
[`references/biz/saas-kpi-targets-baseline.md`](../../references/biz/saas-kpi-targets-baseline.md).
Generated artifacts MUST include `baseline_ref:` with the retrieval stamp. If a
consulted row is older than 12 months, emit a **stale warning** naming the row.
If a consulted row is older than 24 months, **refuse at +24 months** and ask for
a baseline refresh before producing target numbers.

The target values below are readability mirrors of the cited baseline. If this
SKILL.md and the baseline disagree, the baseline file wins.

## `mode_arg: b2b` body

### KPI dashboard

| KPI | Formula | Seed-stage target | Refresh cadence | Owner | Watch signal |
|---|---|---|---|---|---|
| ARR | contracted recurring × 12 | "growth not target — track trajectory" | monthly | founder | flat / declining for 2+ months |
| MoM revenue growth | (M − M-1) / M-1 | ≥ 10% | monthly | founder | < 5% for 2+ months |
| NDR (12-month rolling) | retained-and-expanded ÷ original cohort | ≥ 110% | quarterly (after first 12-month cohort matures) | founder | < 100% (net contraction) |
| CAC (blended) | total marketing+sales spend ÷ new customers | "varies by ACV; track payback" | monthly | founder | growing > 20% / month |
| CAC payback (months) | CAC ÷ (ARR per customer × gross margin) | ≤ 12 | monthly | founder | > 18 months |
| Gross margin | (revenue − COGS) ÷ revenue | ≥ 75% | monthly | founder | < 65% |
| Burn multiple | net cash burn ÷ net new ARR | ≤ 2 | monthly | founder | > 3 |
| Runway | cash ÷ monthly burn | ≥ 18 months | monthly | founder | < 12 months |
| Logo / customer count | absolute count | "growth not target" | monthly | founder | flat / declining |

Plus PM cross-reference: "for activation / retention / feature adoption metrics, run `/slo-product metrics`".

## `mode_arg: consumer` body

### KPI dashboard

| KPI | Formula | Seed-stage target | Refresh cadence | Owner | Watch signal |
|---|---|---|---|---|---|
| MoM revenue growth | (M − M-1) / M-1 | ≥ 15% | monthly | founder | < 10% for 2+ months |
| NPS | promoters − detractors | ≥ 30 | quarterly | founder | < 0 (more detractors than promoters) |
| Paying-customer count | absolute count | "growth not target" | monthly | founder | declining |
| ARPU (avg revenue per user) | total revenue ÷ paying customers | varies | monthly | founder | declining > 10% / month |
| CAC (blended) | total spend ÷ new paying customers | "track payback in months" | monthly | founder | growing > 30% / month |
| CAC payback | CAC ÷ ARPU | ≤ 6 months for B2C consumer | monthly | founder | > 12 months |
| Gross margin | (revenue − COGS) ÷ revenue | varies by category | monthly | founder | < 50% |
| Burn multiple | net cash burn ÷ net new revenue | ≤ 2 | monthly | founder | > 3 |
| Runway | cash ÷ monthly burn | ≥ 18 months | monthly | founder | < 12 months |
| Cohort retention curve flatness | D90 retention ÷ D7 retention | ≥ 0.6 (curve flattens, doesn't keep falling) | quarterly | founder | < 0.4 (slope, not smile) |

Plus PM cross-reference: "for DAU / activation / feature-adoption, run `/slo-product metrics`".

## SEIS/EIS qualifying-trade interaction

For SEIS/EIS qualified companies, monitor revenue mix against VCM3000 excluded-activities list. The skill ROUTES this consideration to `/slo-fundraise triage` if the founder mentions any pivot or revenue-mix shift.

## UK-only jurisdiction

UK only in v1; canonical "v1 supports UK only" error.

## No WebFetch / WebSearch.

## Refusal patterns

1. Unknown mode_arg → standard error.
2. Non-UK founder → canonical error.
3. PM-side metric (DAU / activation / retention / feature adoption / NPS-as-product-metric) → redirect to `/slo-product metrics`.
4. SEIS/EIS revenue-mix concern → route to `/slo-fundraise triage`.

## Handoff

After dashboard: suggest `/slo-fundraise prepare 'investor-update review'` for monthly investor updates that cite these KPIs.

## What this skill is NOT

- Not a PM-metrics tool — that's `/slo-product metrics`.
- Not an accounting tool — books / VAT / corporation tax computations are `/slo-accounting`.
- Not a forecasting tool — financial models / scenarios are out of v1 scope.
- Not jurisdiction-aware — UK only in v1.

---

**Loops**: Pricing loop, GTM loop — see [docs/LOOPS-BUSINESS.md#pricing-loop](../../docs/LOOPS-BUSINESS.md#pricing-loop).
