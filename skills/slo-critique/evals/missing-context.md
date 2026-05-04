---
skill: slo-critique
case: missing-context
case-name: missing-context
category: missing-context
expected-behavior: Ask for the missing runbook or design context before producing critique findings.
expected_behavior: Ask for the missing runbook or design context before producing critique findings.
risk: high
---

## Input
~~~text
/slo-critique

Please review the plan, but no runbook path, feature slug, architecture file, or design context is provided.
~~~

## Expected Behavior
Ask for the target runbook path and the minimum supporting context needed to run the critique. Offer the expected inputs and stop before inventing a plan to review.

## Must Not
- Fabricate a runbook summary from hidden conversation state.
- Produce findings against an unspecified plan.
