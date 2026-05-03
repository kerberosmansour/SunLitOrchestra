---
name: product-prioritization-frameworks
created: 2026-05-03
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified
audience: /slo-product
source_url: https://www.intercom.com/blog/rice-simple-prioritization-for-product-managers/
last_checked: 2026-05-03
confidence: high
methodology_note: Top-level source verifies the RICE framework; Kano has its own row-level academic source below.
applicability_caveat: Prioritization frameworks create conversation discipline; they do not prove strategy quality.
---

# Product prioritization frameworks

Use this file for [`skills/slo-product/SKILL.md`](../../skills/slo-product/SKILL.md)
roadmap outputs.

```yaml
baseline_ref: references/biz/product-prioritization-frameworks.md@2026-05-03
```

## Refresh discipline

- Emit a **stale warning** when a consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.

## Rows

### `framework-rice`

- `definition`: Reach x Impact x Confidence / Effort.
- `source_url`: https://www.intercom.com/blog/rice-simple-prioritization-for-product-managers/
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: Intercom's original RICE article defines the four factors, example scales, and the formula.
- `sample_size`: framework article; no survey sample.
- `vintage`: 2018 article, checked 2026-05-03.
- `applicability_caveat`: RICE can create false precision when reach, impact, or effort are guessed. The artifact must record confidence and the evidence behind it.

### `framework-kano`

- `definition`: classify product attributes by satisfaction response, including attractive, must-be, one-dimensional, indifferent, and reverse categories.
- `source_url`: https://www.jstage.jst.go.jp/article/quality/14/2/14_KJ00002952366/_article
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: Kano, Seraku, Takahashi, and Tsuji's 1984 paper proposes two-dimensional quality recognition and quality-element categories.
- `sample_size`: original paper includes consumer questionnaire examples; not a SaaS benchmark sample.
- `vintage`: 1984
- `applicability_caveat`: Kano categories are user-perception labels. Do not label a feature a delighter without user evidence.

## Skill use

`/slo-product roadmap` may use RICE, Kano, or both. If both disagree, the skill
surfaces the disagreement rather than averaging the frameworks.
