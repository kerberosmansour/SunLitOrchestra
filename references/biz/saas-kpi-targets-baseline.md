---
name: saas-kpi-targets-baseline
created: 2026-04-27
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified
audience: /slo-metrics primary; /slo-fundraise, /slo-pricing, /slo-sales-funnel, and /slo-product may cite by row
source_url: https://www.highalpha.com/saas-benchmarks/2024
last_checked: 2026-05-03
confidence: medium
methodology_note: Top-level file metadata points at the highest-coverage SaaS benchmark source; each row below has its own source and method.
applicability_caveat: Seed-stage UK founders should treat these as diagnostic bands, not investor-grade promises.
---

# SaaS KPI targets - sourced baseline

This file replaces free-floating KPI heuristics in
[`skills/slo-metrics/SKILL.md`](../../skills/slo-metrics/SKILL.md). Skill prose may
show target values for readability, but the authority is this file plus the row
retrieval stamp.

Every artifact that uses these rows carries:

```yaml
baseline_ref: references/biz/saas-kpi-targets-baseline.md@2026-05-03
```

## Refresh discipline

- `retrieved: 2026-05-03`.
- Emit a **stale warning** when any consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.
- A stale row does not invalidate unrelated rows; warn per consulted row.

## Schema

Each row carries:

- `kpi_id`
- `formula`
- `seed_target`
- `watch_signal`
- `source_url`
- `last_checked`
- `confidence`
- `methodology_note`
- `sample_size`
- `vintage`
- `applicability_caveat`

## Capital efficiency

### `kpi-burn-multiple`

- `formula`: net cash burn for the period divided by net new ARR for the same period.
- `seed_target`: <= 2 is healthy enough to keep investing; 2-3 is watch; > 3 is a spending/pricing review trigger.
- `watch_signal`: > 3 for two consecutive quarters.
- `source_url`: https://www.bvp.com/atlas/state-of-the-cloud-2024
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Bessemer's 2024 cloud report uses the inverse capital-efficiency framing, BVP Efficiency Ratio = Net New CARR / Net Burn, and reports a ~1.1x ratio for scaled Vertical AI companies. The skill keeps the market's burn-multiple convention because founders commonly present it as net burn / net new ARR.
- `sample_size`: several dozen Bessemer portfolio / observed Vertical AI companies for the cited 2024 efficiency-ratio point; not a full-market burn-multiple survey.
- `vintage`: 2024
- `applicability_caveat`: Best for recurring-revenue SaaS. Usage-based, services-heavy, hardware, marketplace, or pre-revenue businesses should label this row as not yet comparable.

### `kpi-runway-months`

- `formula`: cash on hand divided by trailing 3-month average net burn.
- `seed_target`: >= 18 months after a funding event; < 12 months triggers fundraise or cost-cut planning.
- `watch_signal`: < 12 months; < 6 months is urgent.
- `source_url`: https://www.highalpha.com/saas-benchmarks/2024
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: High Alpha/OpenView reports founder concern with burn and fundraising environment; the 18-month target is a planning guardrail rather than a directly surveyed benchmark.
- `sample_size`: 800+ SaaS survey respondents for the 2024 report context.
- `vintage`: 2024
- `applicability_caveat`: Founder personal runway is separate from company runway and belongs in `/slo-founder-check`.

## Growth and retention

### `kpi-mom-revenue-growth`

- `formula`: (this month revenue - last month revenue) / last month revenue.
- `seed_target`: B2B >= 10% MoM; consumer >= 15% MoM, with smaller bases expected to grow faster.
- `watch_signal`: B2B < 5% for 2+ months; consumer < 10% for 2+ months.
- `source_url`: https://www.paulgraham.com/growth.html
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Paul Graham anchors startup growth as a rate and uses weekly growth framing. These monthly seed targets are derived operating bands, not directly quoted survey medians.
- `sample_size`: essay framework; no survey sample.
- `vintage`: 2012 essay, stable page checked 2026-05-03.
- `applicability_caveat`: Early revenue bases are noisy; quote both absolute revenue and growth rate.

### `kpi-net-dollar-retention`

- `formula`: retained and expanded ARR from a cohort / original cohort ARR, measured 12 months later.
- `seed_target`: B2B SaaS >= 110% once 12-month cohorts exist.
- `watch_signal`: < 100% for any mature 12-month cohort.
- `source_url`: https://www.highalpha.com/saas-benchmarks/2024
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: High Alpha/OpenView reports public SaaS NRR stabilization around 110% and uses NRR as a core health metric for SaaS retention.
- `sample_size`: 800+ SaaS survey respondents for the 2024 private-company report, plus public SaaS bellwether data discussed in the report.
- `vintage`: 2024
- `applicability_caveat`: Not meaningful before renewal cohorts mature; consumer products should use cohort-retention curves instead.

### `kpi-cohort-retention-flatness`

- `formula`: later-period retention divided by early-period retention for the relevant cadence, e.g. D90 / D7.
- `seed_target`: curve flattens; do not force a universal numeric threshold without category-specific comps.
- `watch_signal`: curve keeps falling toward zero.
- `source_url`: https://articles.sequoiacap.com/retention
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: Sequoia frames retention-curve shape as flattening, declining, or smiling and treats retention as a product-growth foundation.
- `sample_size`: framework article; no survey sample.
- `vintage`: stable article, checked 2026-05-03.
- `applicability_caveat`: Pick D/W/M cadence from actual product frequency; monthly retention can hide weekly-use failure.

### `kpi-nps`

- `formula`: percentage of promoters minus percentage of detractors.
- `seed_target`: track trend and qualitative follow-up; >= 30 can be a starter floor only when the founder lacks sector comps.
- `watch_signal`: < 0, or sharp decline after a launch or pricing change.
- `source_url`: https://www.netpromotersystem.com/about/
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: Bain owns the Net Promoter System framing; the skill uses it as a feedback instrument, not as a universal valuation benchmark.
- `sample_size`: methodology source; no single startup sample.
- `vintage`: stable methodology page, checked 2026-05-03.
- `applicability_caveat`: Industry medians vary widely; use relative movement and verbatim follow-up comments.

## Unit economics

### `kpi-cac-payback-months`

- `formula`: CAC / (ARR per customer * gross margin), period-aligned.
- `seed_target`: B2B <= 12 months; consumer <= 6 months when spend is paid-acquisition led.
- `watch_signal`: B2B > 18 months; consumer > 12 months.
- `source_url`: https://www.highalpha.com/saas-benchmarks/2024
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: High Alpha/OpenView treats GTM and efficiency as core benchmark dimensions; exact payback bands vary by ACV and motion, so the skill records assumptions with the formula.
- `sample_size`: 800+ SaaS survey respondents for the 2024 report context.
- `vintage`: 2024
- `applicability_caveat`: Founder-led sales can make paid CAC look artificially low; include founder time separately when relevant.

### `kpi-gross-margin`

- `formula`: (revenue - COGS) / revenue.
- `seed_target`: B2B SaaS >= 75%; managed-service SaaS may sit lower while delivery is still human-heavy.
- `watch_signal`: B2B SaaS < 65% sustained.
- `source_url`: https://www.bvp.com/atlas/state-of-the-cloud-2024
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Bessemer reports ~65% gross margin for scaled Vertical AI companies and notes model costs as a share of revenue/COGS; the older pure-SaaS 75% target remains a seed-stage quality target rather than an AI-native observed median.
- `sample_size`: several dozen Bessemer observed Vertical AI companies for the cited 2024 datapoint.
- `vintage`: 2024
- `applicability_caveat`: AI-native products with material model costs should show model-cost share and gross margin separately.

### `kpi-arpu`

- `formula`: period revenue / paying customers.
- `seed_target`: track trajectory, not a universal target.
- `watch_signal`: declining > 10% per month for 2+ months.
- `source_url`: https://www.highalpha.com/saas-benchmarks/2024
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: The source supports segmenting SaaS metrics by ARR band and ICP; ARPU itself is a business-model-specific tracker.
- `sample_size`: 800+ SaaS survey respondents for the 2024 report context.
- `vintage`: 2024
- `applicability_caveat`: Define denominator carefully: paying users, accounts, seats, or active customers.

## Outbound funnel cross-reference

Outbound conversion targets live in
[`references/biz/outbound-conversion-baselines.md`](outbound-conversion-baselines.md).
`/slo-metrics` may cite that file when CAC or pipeline conversion becomes a KPI.
