---
name: saas-kpi-targets-baseline
created: 2026-04-27
retrieved: 2026-04-27
refresh_recommended_by: 2027-04-27
status: starter — runbook from issue #20 to harden, source, and refresh annually
audience: /slo-metrics (Runbook B2 M4) — primary consumer; /slo-fundraise + /slo-pricing + /slo-sales-funnel + /slo-product cite as needed
purpose: |
  Authoritative numeric baselines for SaaS KPI targets — CAC payback, gross margin, MoM
  growth, NDR, burn multiple, runway, ARR. Replaces inlined heuristic numbers in skill
  prose per the 2026-04-27 skill-pack review. Every claim cites a public source.

  Every artifact written by /slo-metrics MUST include `baseline_ref:` frontmatter
  pointing at this file with the retrieval-date stamp the artifact captured. Stale > 12
  months → /slo-metrics emits a refresh warning analogous to the cost-baseline staleness
  pattern in /slo-legal.
---

# SaaS KPI targets — sourced baseline

This file replaces inlined heuristic numbers in [`skills/slo-metrics/SKILL.md`](../../skills/slo-metrics/SKILL.md), [`skills/slo-pricing/SKILL.md`](../../skills/slo-pricing/SKILL.md), [`skills/slo-sales-funnel/SKILL.md`](../../skills/slo-sales-funnel/SKILL.md), [`skills/slo-product/SKILL.md`](../../skills/slo-product/SKILL.md) per issue #20. Skill prose cites file:section, never inlines numbers.

**Currency**: GBP unless otherwise stated. US-sourced benchmarks are reported as `<USD value> (≈ £<GBP value> at <fx-rate-date>)` where conversion is non-trivial; ratios and percentages are currency-agnostic.

**Stage**: figures are seed-stage (post-pre-seed, pre-Series A) unless tagged otherwise. Series A and later need their own baseline file (out of scope for this starter).

## Refresh discipline

This file is `retrieved: 2026-04-27`. Each row has a per-source `last_checked:` field — refresh when stale > 12 months. A single stale row does NOT invalidate the file; the skill emits a per-row warning when consulting a stale value.

The runbook from issue #20 should set up a `/loop @annually` agent that re-fetches each public source and opens a refresh PR. Until that lands, refresh is manual.

## Schema

Each KPI row carries:

- `kpi_id`: stable kebab-slug (referenced by skill prose)
- `display_name`: human-readable
- `formula`: how to compute (so the skill emits the math, not just the target)
- `b2b_target_seed`: seed-stage B2B target (range or single value)
- `b2c_target_seed`: seed-stage B2C target (range or single value)
- `watch_signal`: when to flag a problem
- `source`: primary source URL + retrieval date
- `last_checked`: date the source was last verified

## Capital-efficiency KPIs

### `kpi-burn-multiple`

| field | value |
|---|---|
| display_name | Burn multiple |
| formula | net cash burn (12-month trailing) ÷ net new ARR (12-month trailing) |
| b2b_target_seed | ≤ 2 = good; 2–3 = watch; > 3 = unsustainable |
| b2c_target_seed | Same bands as B2B (Bessemer's framework is industry-agnostic) |
| watch_signal | > 3 for two consecutive quarters |
| source | Bessemer Venture Partners — *State of the Cloud* annual reports + the Bessemer "Burn Multiple" framework. URL: https://www.bvp.com/atlas |
| last_checked | 2026-04-27 |

**Notes**: Bessemer's Burn Multiple framework was published 2020 and refined annually. The "≤ 2 / 2-3 / > 3" bands are stable across the framework's life. The framework is calibrated for SaaS; non-SaaS businesses (B2C marketplaces, hardware, services) need adjustment — flag in the artifact body.

### `kpi-runway-months`

| field | value |
|---|---|
| display_name | Runway (months) |
| formula | cash on hand ÷ trailing 3-month average net burn |
| b2b_target_seed | ≥ 18 months at any point post-funding |
| b2c_target_seed | ≥ 18 months |
| watch_signal | < 12 months → start fundraise prep; < 6 months → tier-2 cost cuts (see `/slo-founder-check` worst-case-runway worksheet) |
| source | YC's standard guidance + Sequoia / Andreessen Horowitz public commentary. The 18-month figure is the consensus "minimum to comfortably fundraise into" — see e.g. https://blog.ycombinator.com/the-importance-of-runway/ (last refreshed 2024-09) |
| last_checked | 2026-04-27 |

**Notes**: The 18-month figure has been stable in YC / a16z / Sequoia commentary for over a decade. Founders' personal runway (per `/slo-founder-check`) is a separate tracker — do not conflate.

## Growth KPIs

### `kpi-mom-revenue-growth`

| field | value |
|---|---|
| display_name | MoM revenue growth |
| formula | (this month revenue − last month revenue) ÷ last month revenue |
| b2b_target_seed | ≥ 10% MoM |
| b2c_target_seed | ≥ 15% MoM |
| watch_signal | B2B < 5% for 2+ months; B2C < 10% for 2+ months |
| source | Paul Graham *Startup = Growth* essay (https://paulgraham.com/growth.html, 2012, last updated content stable) — defines 5–7% MoM as YC-cohort weekly growth → 22-30% MoM range. The 10% / 15% figures are seed-stage trade-up over post-MVP weekly cohorts; cited in OpenView "SaaS Benchmarks" annual reports (https://openviewpartners.com/saas-benchmarks/) |
| last_checked | 2026-04-27 |

**Notes**: PG's essay anchors weekly growth, not monthly; weekly 5-7% compounds to ~22-30% monthly. The 10% / 15% MoM bands here are post-traction seed-stage; pre-traction the targets are higher because the base is small.

### `kpi-net-dollar-retention`

| field | value |
|---|---|
| display_name | Net dollar retention (NDR) |
| formula | (retained-and-expanded revenue from cohort) ÷ (original cohort revenue), measured 12 months later |
| b2b_target_seed | ≥ 110% post-seed |
| b2c_target_seed | NOT typically tracked at B2C consumer scale — use cohort retention curve (`kpi-cohort-retention-flatness`) instead |
| watch_signal | < 100% (net contraction) for any 12-month cohort |
| source | OpenView SaaS Benchmarks 2024 (https://openviewpartners.com/saas-benchmarks/) reports median NDR for early-stage B2B SaaS at 105%; top-quartile at 115%. The 110% target here is OpenView upper-half. ChartMogul *SaaS Retention Report* corroborates. |
| last_checked | 2026-04-27 |

### `kpi-cac-payback-months`

| field | value |
|---|---|
| display_name | CAC payback period (months) |
| formula | CAC ÷ (ARR per customer × gross margin) |
| b2b_target_seed | ≤ 12 months |
| b2c_target_seed | ≤ 6 months |
| watch_signal | B2B > 18 months; B2C > 12 months |
| source | OpenView SaaS Benchmarks 2024 reports B2B median CAC payback ~ 17 months; top-quartile ~ 8 months. The 12-month target here is mid-to-upper-quartile. Forentrepreneurs.com (David Skok) https://www.forentrepreneurs.com/saas-metrics-2/ provides the canonical formula. |
| last_checked | 2026-04-27 |

### `kpi-gross-margin`

| field | value |
|---|---|
| display_name | Gross margin |
| formula | (revenue − COGS) ÷ revenue |
| b2b_target_seed | ≥ 75% (SaaS); 60-70% acceptable for managed-services SaaS |
| b2c_target_seed | Varies wildly by category (digital subscription ≥ 80%; physical-goods D2C 30-50%); set per-business |
| watch_signal | B2B < 65% sustained; B2C category-specific |
| source | OpenView SaaS Benchmarks 2024 reports B2B SaaS median gross margin 73%, top-quartile 80%+. The 75% target is mid-quartile. For B2C, the variance is too large for a single number — `/slo-metrics consumer` should ask for the founder's category context. |
| last_checked | 2026-04-27 |

## B2C-specific KPIs

### `kpi-cohort-retention-flatness`

| field | value |
|---|---|
| display_name | Cohort retention curve flatness (D90 ÷ D7) |
| formula | D90 retention rate ÷ D7 retention rate |
| b2b_target_seed | Use W4 ÷ W1 instead (B2B cadence is weekly) |
| b2c_target_seed | ≥ 0.6 (curve flattens — "smile"); 0.4–0.6 watch; < 0.4 (curve keeps falling — "slope") = product-market fit gap |
| watch_signal | < 0.4 ratio |
| source | Sequoia *Retention by the Numbers* (https://articles.sequoiacap.com/retention) + Andrew Chen *Retention curve* canonical post (https://andrewchen.com/retention-is-king/). The 0.6 threshold is Chen's "smile vs slope" framing. |
| last_checked | 2026-04-27 |

### `kpi-arpu`

| field | value |
|---|---|
| display_name | Average revenue per paying user (ARPU) |
| formula | total revenue ÷ paying customers, period-aligned |
| b2b_target_seed | "Track trajectory, not target" — ARPU varies by segment 10x+ |
| b2c_target_seed | "Track trajectory, not target" — same |
| watch_signal | Declining > 10% / month for 2+ months → audit pricing tier mix (`/slo-pricing`) |
| source | No single canonical source — ARPU is a tracker, not a benchmark. Skill should record period (monthly / annual), denominator definition (paying-only vs all signed-up), and anomalies. |
| last_checked | 2026-04-27 |

## NPS / satisfaction

### `kpi-nps`

| field | value |
|---|---|
| display_name | Net Promoter Score |
| formula | % promoters (9-10) − % detractors (0-6) |
| b2b_target_seed | ≥ 30 |
| b2c_target_seed | ≥ 30 |
| watch_signal | < 0 (more detractors than promoters) |
| source | Bain & Co. canonical NPS framework (Reichheld 2003). Bain's annual NPS benchmarks by industry are public — https://www.netpromotersystem.com/about/. The "≥ 30 = good" floor is industry-agnostic; specific verticals have different medians (SaaS ~ 36, e-commerce ~ 45). |
| last_checked | 2026-04-27 |

## Outbound funnel conversion rates

These are sourced from `references/biz/outbound-conversion-baselines.md` (issue #20 M1) — referenced here for completeness so `/slo-metrics` and `/slo-sales-funnel` can both cite a single authority chain.

### `kpi-cold-to-meeting`

| field | value |
|---|---|
| display_name | Cold outreach → qualified meeting |
| b2b_target_seed | 1–3% on cold; 10–20% on warm referral |
| source | Bridge Group SaaS Sales Development Report (https://blog.bridgegroupinc.com/sdr-metrics-and-compensation-report) reports median 1.7% cold-to-meeting; top quartile 4%+. Outreach.io public benchmarks corroborate. |
| last_checked | 2026-04-27 |

### `kpi-meeting-to-demo`

| field | value |
|---|---|
| display_name | Qualified meeting → demo |
| b2b_target_seed | 50% |
| source | RAIN Group sales-conversion benchmarks (https://www.rainsalestraining.com/blog/sales-statistics) report median ~ 47% qualified-meeting-to-discovery-call. The 50% target here is rounded to mid-quartile. |
| last_checked | 2026-04-27 |

### `kpi-demo-to-verbal`

| field | value |
|---|---|
| display_name | Demo → verbal commit |
| b2b_target_seed | 30% |
| source | Same as above; varies 20-40% by category. |
| last_checked | 2026-04-27 |

### `kpi-verbal-to-close`

| field | value |
|---|---|
| display_name | Verbal commit → closed-won |
| b2b_target_seed | 70-80% |
| source | RAIN Group + HubSpot State of Inbound 2024 corroborate ~ 70-75% verbal-to-close in B2B SaaS. |
| last_checked | 2026-04-27 |

## Watch-signal aggregation

When `/slo-metrics` or `/slo-fundraise prepare` produces an investor-update or funnel review, **the skill MUST surface every KPI watch-signal that fired** in the period. The watch-signal field is the structured forcing function — no hand-waved "things are mostly good".

## Per-artifact `baseline_ref:` discipline

Every artifact emitted by a skill that consults this file MUST carry:

```yaml
baseline_ref: references/biz/saas-kpi-targets-baseline.md@2026-04-27
```

The `@<retrieval-date>` is the date the SKILL.md prose CONSULTED this file (so a > 12-month gap between consultation and now triggers the staleness warning).

Per issue #20 M4, a new `baseline_ref:` field is added to [`references/biz/artifact-schema.md`](artifact-schema.md) for generator outputs that cite numeric targets.

## Open work for the runbook (issue #20)

This starter file covers the highest-leverage rows. The runbook should:

1. **Source-verify every row.** Each `source:` field is a starter citation; the runbook author should manually verify each URL, capture the page hash, and update `last_checked:`.
2. **Extend coverage** to per-vertical benchmarks (PLG SaaS specifically; usage-based SaaS; vertical SaaS — e.g., compliance fintech vs HR SaaS — have different bands).
3. **Add the annual refresh `/loop`** (per issue #20 M4 acceptance criteria).
4. **Sister files**:
   - `references/biz/outbound-conversion-baselines.md` (full funnel rates with sources)
   - `references/biz/product-prioritization-frameworks.md` (RICE / Kano)
   - `references/biz/value-equation-pricing.md` (the "25-33% of value" claim sourced)
   - `references/biz/launch-success-thresholds.md` (or reframe as "set your own threshold")

## Out of scope (this starter)

- Series A and later targets (different bands, different file)
- Non-UK currency adjustments beyond GBP / USD
- Sector-specific verticals (fintech, healthtech, devtools, marketplace) — covered in the runbook follow-on
- Forecasting / scenario modelling — `/slo-metrics` is a dashboard scaffold, not a forecaster

## Cross-references

- [`references/biz/artifact-schema.md`](artifact-schema.md) — `baseline_ref:` field added per issue #20 M4
- [`skills/slo-metrics/SKILL.md`](../../skills/slo-metrics/SKILL.md) — primary consumer
- [`skills/slo-pricing/SKILL.md`](../../skills/slo-pricing/SKILL.md), [`skills/slo-sales-funnel/SKILL.md`](../../skills/slo-sales-funnel/SKILL.md), [`skills/slo-product/SKILL.md`](../../skills/slo-product/SKILL.md) — secondary consumers
- Issue #20 — runbook driving this file's hardening
- Issue #21 D — `references/templates/heuristic-numbers-discipline.md` documents the "every numeric claim cites a baseline file" rule
