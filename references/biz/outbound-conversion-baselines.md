---
name: outbound-conversion-baselines
created: 2026-05-03
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified-with-low-confidence-rates
audience: /slo-sales-funnel primary; /slo-metrics secondary
source_url: https://blog.bridgegroupinc.com/sales-development-metrics
last_checked: 2026-05-03
confidence: medium
methodology_note: Top-level source verifies Bridge Group's current SDR metrics report surface and 365-company sample; stage-rate rows below combine this with RAIN proposal-win data where public detail is available.
applicability_caveat: Outbound conversion varies by ICP, ACV, channel warmth, and qualification definition; founders must replace these with their own observed rates as soon as data exists.
---

# Outbound conversion baselines

Use this file for funnel math in [`skills/slo-sales-funnel/SKILL.md`](../../skills/slo-sales-funnel/SKILL.md).
Do not present the rates as universal truths. They are starting priors for a
founder with no funnel history.

Every artifact that uses this file carries:

```yaml
baseline_ref: references/biz/outbound-conversion-baselines.md@2026-05-03
```

## Refresh discipline

- Emit a **stale warning** when a consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.

## Rows

### `outbound-cold-to-qualified-meeting`

- `stage`: cold outreach to qualified meeting.
- `starter_rate`: 1-3% for cold; 10-20% for warm referral.
- `source_url`: https://blog.bridgegroupinc.com/sales-development-metrics
- `last_checked`: 2026-05-03
- `confidence`: low
- `methodology_note`: Bridge Group's public page verifies a 365-company SDR benchmark report and scope, but the detailed stage-rate table is gated; use this as a conservative planning prior.
- `sample_size`: 365 B2B companies for the Bridge Group report.
- `vintage`: 2023 report surface, checked 2026-05-03.
- `applicability_caveat`: Founder-led targeted outreach can outperform broad SDR benchmarks; high-ACV enterprise outreach can underperform on volume but still work economically.

### `outbound-qualified-meeting-to-demo`

- `stage`: qualified meeting to demo or discovery-confirmed next step.
- `starter_rate`: 50%.
- `source_url`: https://blog.bridgegroupinc.com/sales-development-metrics
- `last_checked`: 2026-05-03
- `confidence`: low
- `methodology_note`: Bridge Group source verifies SDR-motion scope; the skill records this as an operational default, not a primary published median.
- `sample_size`: 365 B2B companies for the Bridge Group report.
- `vintage`: 2023 report surface, checked 2026-05-03.
- `applicability_caveat`: If "qualified meeting" is defined strictly, demo conversion should be higher; if it means any first call, it will be lower.

### `outbound-demo-to-verbal-commit`

- `stage`: demo to verbal commit or proposal-stage intent.
- `starter_rate`: 30%.
- `source_url`: https://www.rainsalestraining.com/hubfs/PDFs/The_Top-Performing_Sales_Manager_Benchmark_Report.pdf
- `last_checked`: 2026-05-03
- `confidence`: low
- `methodology_note`: RAIN reports proposal win-rate differences from a 1,004-respondent sales-management study; this row maps that late-stage evidence back to an earlier demo-to-commit planning prior.
- `sample_size`: 1,004 respondents in the RAIN Group report.
- `vintage`: RAIN report checked 2026-05-03.
- `applicability_caveat`: Do not use for self-serve PLG. Use only when a human demo and explicit buying conversation exist.

### `outbound-verbal-to-close`

- `stage`: verbal commit or proposal to closed-won.
- `starter_rate`: 70-80% when the buyer has explicit authority and timeline; 45-55% when counted from proposal sent.
- `source_url`: https://www.rainsalestraining.com/hubfs/PDFs/The_Top-Performing_Sales_Manager_Benchmark_Report.pdf
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: RAIN reports average proposal win rates of 72% for top performers and 47% for the rest; the skill uses 70-80% only for founder-confirmed verbal commits, not all proposals.
- `sample_size`: 1,004 respondents in the RAIN Group report.
- `vintage`: RAIN report checked 2026-05-03.
- `applicability_caveat`: Procurement, security review, and legal paper can break verbal commitments; route contract blockers to `/slo-legal`.

## Founder override rule

Once the founder has 20+ opportunities in a stage, replace the starter prior with
their own observed rate and record `baseline_ref` plus `founder_data_window`.
