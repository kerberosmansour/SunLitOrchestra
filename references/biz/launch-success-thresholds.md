---
name: launch-success-thresholds
created: 2026-05-03
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified-threshold-discipline
audience: /slo-launch
source_url: https://articles.sequoiacap.com/retention
last_checked: 2026-05-03
confidence: medium
methodology_note: Source verifies retention and product-health discipline; launch-stage thresholds are founder-owned guardrails, not public universal benchmarks.
applicability_caveat: Launch quality depends on audience, category, and goal. The founder must set thresholds before each stage.
threshold_owner: founder
---

# Launch success thresholds

Use this file for [`skills/slo-launch/SKILL.md`](../../skills/slo-launch/SKILL.md).
The rule is: **set your own threshold** before each stage, record the owner, and
use the threshold to decide continue / delay / rework.

```yaml
baseline_ref: references/biz/launch-success-thresholds.md@2026-05-03
threshold_owner: founder
```

## Refresh discipline

- Emit a **stale warning** when a consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.

## Rows

### `launch-stage-thresholds`

- `claim`: Launch thresholds are founder-set gates, not borrowed vanity metrics.
- `source_url`: https://articles.sequoiacap.com/retention
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Sequoia frames retention and product health through product-specific event definitions and cohort behavior. `/slo-launch` applies the same discipline to staged launch signals.
- `sample_size`: framework article; no launch survey sample.
- `vintage`: stable article, checked 2026-05-03.
- `applicability_caveat`: Product Hunt rank, Hacker News position, or signup counts are only meaningful if the founder names the intended audience and activation event first.

### `launch-stage-guardrails`

- `claim`: Silent, friends-and-family, niche-community, and broader-press stages each need explicit success and kill signals.
- `source_url`: https://www.paulgraham.com/growth.html
- `last_checked`: 2026-05-03
- `confidence`: medium
- `methodology_note`: Paul Graham's growth essay anchors the need to measure real startup growth rather than surface attention. The launch skill translates this into stage gates.
- `sample_size`: essay framework; no survey sample.
- `vintage`: 2012 essay, checked 2026-05-03.
- `applicability_caveat`: If the founder cannot define the activation event, delay broader launch and run `/slo-talk-to-users`.

## Founder threshold template

Each stage records:

- `threshold_owner: founder`
- target audience
- success threshold
- kill / delay threshold
- measurement window
- next action if threshold misses

This replaces generic launch numbers with explicit founder-owned bets.
