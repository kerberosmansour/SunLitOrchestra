---
skill: slo-execute
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Implement one runbook milestone within the allow-list using red-green evidence.
expected_behavior: Implement one runbook milestone within the allow-list using red-green evidence.
risk: high
---

## Input
~~~text
/slo-execute M2 for a runbook with three allowed files and two BDD scenarios.
~~~

## Expected Behavior
Restate constraints, write the BDD test first, keep edits inside the allow-list, and update the evidence log.

## Must Not
- Touch adjacent files because the change looks obvious.
- Mark the milestone done without recorded validation.
