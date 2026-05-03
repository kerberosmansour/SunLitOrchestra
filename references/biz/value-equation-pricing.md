---
name: value-equation-pricing
created: 2026-05-03
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified-with-opinion-caveat
audience: /slo-pricing
source_url: https://shop.acquisition.com/products/100m-offers-hardcover
last_checked: 2026-05-03
confidence: medium
methodology_note: Acquisition.com verifies the $100M Offers source and value-equation framing; the 25-33% capture band is SLO operating guidance, not a public statistical benchmark.
applicability_caveat: Value pricing is opinionated sales strategy. It must be tested against buyer behavior, not treated as market law.
---

# Value-equation pricing

Use this file for [`skills/slo-pricing/SKILL.md`](../../skills/slo-pricing/SKILL.md).

```yaml
baseline_ref: references/biz/value-equation-pricing.md@2026-05-03
```

## Refresh discipline

- Emit a **stale warning** when a consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.

## Rows

### `pricing-value-equation`

- `claim`: Price should be anchored to economic value delivered, with the skill calculating a 25-33% capture band.
- `source_url`: https://shop.acquisition.com/products/100m-offers-hardcover
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Acquisition.com describes $100M Offers as teaching the Value Equation and charging more by making the offer worth more. The SLO 25-33% band is an operational heuristic layered on that value-pricing source.
- `sample_size`: book/source framework; no survey sample.
- `vintage`: source checked 2026-05-03.
- `applicability_caveat`: Do not use when value cannot be estimated within an order of magnitude; run `/slo-talk-to-users` first.

### `pricing-increase-by-50-experiment`

- `claim`: Test a higher price on new prospects and decide based on conversion-rate delta.
- `source_url`: https://shop.acquisition.com/products/100m-offers-hardcover
- `last_checked`: 2026-05-03
- `confidence`: low
- `methodology_note`: This is SLO's founder-undercharge correction experiment inspired by value-pricing practice, not a cited market benchmark.
- `sample_size`: not applicable.
- `vintage`: source checked 2026-05-03.
- `applicability_caveat`: Only test on new prospects with clear consent to the quoted price. Do not surprise existing customers.

## Opinion labeling

The old "30-50% below market" line is not sourceable as a universal benchmark.
When used, label it as founder-undercharge pattern recognition and verify with
the 30-day price test.
