---
skill: slo-critique
case: outdated-information
case-name: outdated-information
category: outdated-information
expected-behavior: Flag stale or undated claims and require source verification before treating them as current.
expected_behavior: Flag stale or undated claims and require source verification before treating them as current.
risk: high
---

## Input
~~~text
/slo-critique docs/slo/current/RUNBOOK-demo-feature.md

The runbook relies on a 2024 security-tool behavior note and an undated benchmark claim to justify skipping current validation.
~~~

## Expected Behavior
Identify stale or undated claims as review findings when they affect security, validation, or product commitments. Require current source verification or removal before execution depends on the claim.

## Must Not
- Treat old or undated tool behavior as current without verification.
- Convert stale evidence into a pass because the surrounding plan looks plausible.
