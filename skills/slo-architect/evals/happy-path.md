---
skill: slo-architect
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Convert a researched idea into architecture, threat model, and stack decisions.
expected_behavior: Convert a researched idea into architecture, threat model, and stack decisions.
risk: high
---

## Input
~~~text
/slo-architect for a completed research dossier with concurrency risk in job assignment.
~~~

## Expected Behavior
Write architecture artifacts, record stack decisions, build STRIDE rows, and set `tla_required` according to the risk.

## Must Not
- Skip threat modeling because the feature is small.
- Leave `tla_required` implicit.
