---
name: mom-test-canonical-questions
created: 2026-05-03
retrieved: 2026-05-03
refresh_recommended_by: 2027-05-03
status: source-verified-attribution-only
audience: /slo-talk-to-users
source_url: https://www.momtestbook.com/
last_checked: 2026-05-03
confidence: high
methodology_note: Source verifies Rob Fitzpatrick's Mom Test book and customer-conversation discipline; question schema is paraphrased to avoid copyright over-quotation.
applicability_caveat: The questions are a discovery scaffold, not a survey instrument with statistical validity.
---

# Mom Test canonical question schema

Use this file for
[`skills/slo-talk-to-users/SKILL.md`](../../skills/slo-talk-to-users/SKILL.md).

```yaml
baseline_ref: references/biz/mom-test-canonical-questions.md@2026-05-03
```

## Refresh discipline

- Emit a **stale warning** when a consulted row is more than 12 months old.
- Refuse at +24 months until the row is refreshed.

## Source identity

### `mom-test-source`

- `book`: Rob Fitzpatrick, *The Mom Test: How to talk to customers & learn if your business is a good idea when everyone is lying to you*.
- `isbn`: 9781492180746.
- `source_url`: https://www.momtestbook.com/
- `last_checked`: 2026-05-03
- `confidence`: high
- `methodology_note`: The official site frames the book around avoiding biased feedback and learning from customer conversations. Bookshop.org corroborates publication details and ISBN.
- `sample_size`: book/framework source; no survey sample.
- `vintage`: 2013 book, official site checked 2026-05-03.
- `applicability_caveat`: Use for interview question discipline; do not quote long passages into generated artifacts.

## Question schema

The skill asks about:

1. A specific recent occasion when the user tried to achieve the outcome.
2. What made that occasion difficult.
3. Why the difficulty mattered.
4. What workaround the user actually used.
5. What the workaround cost in time, money, risk, or social effort.
6. What alternatives they searched for or bought.
7. What switching would require.
8. Who else has the same pain and could be introduced.

## Anti-patterns

- Asking whether the user would use the founder's idea.
- Asking whether the idea is good.
- Asking hypothetical price questions before pain and current spend are established.
