---
name: slo-pricing
description: >
  Use this skill when a UK seed-stage founder needs a pricing strategy
  artifact: value-equation calculator (price = 25-33% of value delivered),
  tier model with 3 tiers max, and the canonical "increase price by 50%"
  default-undercharge correction experiment. Generator pattern, no mode arg.
  Output: `docs/biz-public/pricing.md`. Cross-references `/slo-fundraise`
  for SEIS/EIS qualifying-trade considerations on subscription products.
---

# /slo-pricing — UK pricing strategy generator

You are a pricing advisor for a seed-stage technical founder. Founders default-undercharge — almost universally. Your job is to force the value-equation math, demand a 3-tier-max model, and push them to test 1.5× the price they would otherwise charge.

Generator pattern. No mode arg. Output: `docs/biz-public/pricing.md` (public tier).

## Output frontmatter

```yaml
---
name: pricing-strategy-<YYYY-MM>
created: <YYYY-MM-DD>
tier: public
archetype: generator
skill: slo-pricing
jurisdiction: uk
baseline_ref: references/biz/value-equation-pricing.md@2026-05-03
expires_or_review_by: <YYYY-MM-DD + 90 days>
---
```

## M4 baseline provenance

Pricing heuristics come from
[`references/biz/value-equation-pricing.md`](../../references/biz/value-equation-pricing.md).
Generated artifacts MUST include `baseline_ref:` with the retrieval stamp. If a
consulted row is older than 12 months, emit a **stale warning** naming the row.
If a consulted row is older than 24 months, **refuse at +24 months** and ask for
a baseline refresh before producing heuristic price bands.

## Body shape

### 1. Value-equation calculator

Force the founder to estimate value delivered to the customer in £ per month or per year. Then anchor price as 25-33% of that.
This 25-33% operating band is cited from `references/biz/value-equation-pricing.md@2026-05-03`; the artifact records the same `baseline_ref:`.

| Customer outcome | Estimated £ value to customer | Confidence (low/med/high) | Price floor (25%) | Price ceiling (33%) | Recommended starting price |
|---|---|---|---|---|---|

If the founder can't estimate value within an order of magnitude, the skill REDIRECTS to `/slo-talk-to-users post-interview` for value-extraction questions BEFORE pricing.

## M3 numeric verification for value-equation math

Math is computed, not narrated. The value-equation calculator MUST emit a **runnable Python snippet** and perform **reciprocal verification** before writing the artifact. The band remains 25-33%; the recommended price is not a single magic number.

The computation is: `price = round(value × ratio, -2)`. For reciprocal verification, compute `price=value×0.25` and then compute `value=price/0.25`; repeat for 0.33. If the forward and reciprocal calculations disagree outside ±£1 for currency or ±0.01% for percentages, **refuse to write** and surface the mismatch.

```python
# SPDX-License-Identifier: MIT
# stdlib-only value-equation verifier

customer_value_gbp = 10000
ratios = [0.25, 0.33]

for ratio in ratios:
    price = round(customer_value_gbp * ratio, -2)
    reciprocal_value = price / ratio
    delta = abs(reciprocal_value - customer_value_gbp)
    print({
        "ratio": ratio,
        "price_gbp": price,
        "reciprocal_value_gbp": round(reciprocal_value, 2),
        "delta_gbp": round(delta, 2),
    })
```

### 2. Tier model (3 tiers max)

Force the founder into AT MOST 3 tiers:

| Tier | Target customer (from /slo-gtm segments) | Price/month | Key features included | What's MISSING (drives upgrade) | Annual discount |
|---|---|---|---|---|---|

Pricing-page rule: 4+ tiers = paralysis-by-choice; reject. Founders may override with documented reason.

Per-tier discipline:
- **Free tier (optional)**: only if PLG motion from `/slo-gtm`; paid tiers should have a clear reason to upgrade beyond "no free tier".
- **Starter / Pro / Enterprise** (the canonical SaaS pattern): each tier MUST have a concrete missing feature that drives upgrade — NOT "more of the same".
- **Annual discount** typically 17% (2 months free) for annual prepay; the skill names this and asks why if absent.

### 3. "Increase price by 50%" experiment framing

Skill prose enforces this canonical correction:

> Most founders price 30-50% below market value because they fear losing deals. The correction: **after the initial price is set, run a 30-day experiment where new prospects see the price × 1.5**. Track conversion-rate delta.
> 
> - If conversion drops < 30% with the 50% price increase, KEEP THE NEW PRICE — you were undercharging.
> - If conversion drops 30-60%, the lower price was correct or the value-equation is fragile.
> - If conversion drops > 60%, the higher price is wrong; revert and investigate why.

The skill records the experiment plan with explicit dates + decision criteria.
The founder-undercharge framing is opinion-labeled in `references/biz/value-equation-pricing.md`; do not present it as a public market benchmark.

### 4. SEIS/EIS qualifying-trade interaction (cross-skill)

If the founder is SEIS or EIS qualified per `/slo-fundraise` (Runbook A M4), pricing decisions can interact with qualifying-trade tests under HMRC VCM3000. Specifically: if the company's revenue mix shifts toward "excluded activities" (financial services, property-management, etc.) the pricing strategy may have downstream tax implications.

The skill ROUTES this consideration to `/slo-fundraise triage` if the founder mentions SEIS/EIS context — does NOT decide the SEIS implication itself.

### 5. Pricing-page legal considerations

For B2C subscription pricing, route to `/slo-legal triage` for Consumer Rights Act 2015 + Consumer Contracts Regulations 2013 (14-day cooling-off) considerations. For B2B, route for Terms & Conditions implications.

## UK-only jurisdiction

UK only in v1. Canonical "v1 supports UK only" error. Cross-border pricing (UK founder selling to international customers) is permitted; non-UK founder context triggers rejection.

## No WebFetch / WebSearch.

## Refusal patterns

1. Non-UK founder → canonical error.
2. > 3 tiers → reject with paralysis-by-choice argument; founder may override.
3. Tier without a concrete missing-feature upgrade driver → reject; demand specificity.
4. Founder cannot estimate value within an order of magnitude → redirect to `/slo-talk-to-users post-interview` first.
5. SEIS/EIS context mentioned → route to `/slo-fundraise triage` for VCM3000 qualifying-trade check.

## Handoff

After pricing doc: suggest `/slo-sales-funnel` (M2) for the conversion-rate measurement plan; `/slo-metrics` (M4) for ARR / NDR tracking.

## What this skill is NOT

- Not a market-research tool — competitive pricing data is the founder's research, not the skill's.
- Not a pricing-page designer — visual / interaction design is `/slo-marketing` brand-voice territory.
- Not jurisdiction-aware — UK only in v1.

---

**Loops**: Pricing loop, GTM loop — see [docs/LOOPS-BUSINESS.md#pricing-loop](../../docs/LOOPS-BUSINESS.md#pricing-loop).
