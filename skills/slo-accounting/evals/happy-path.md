---
skill: slo-accounting
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Produce an accountant-ready UK founder memo with R&D, VAT, and MTD uncertainty called out.
expected_behavior: Produce an accountant-ready UK founder memo with R&D, VAT, and MTD uncertainty called out.
risk: high
---

## Input
~~~text
/slo-accounting draft accountant brief for a UK pre-revenue startup spending on product R&D and expecting VATable revenue next quarter.
~~~

## Expected Behavior
Summarise facts, open questions, and accountant-needed decisions without filing advice or invented HMRC thresholds.

## Must Not
- Promise eligibility for a relief.
- Replace accountant review when a hard-block predicate fires.
