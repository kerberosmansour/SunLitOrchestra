---
skill: slo-cloud-threat-model
case: missing-context
case-name: missing-context
category: missing-context
expected-behavior: Stop with a clear message when required inputs or tooling are absent; never emit a partial document.
expected_behavior: Stop with a clear message when required inputs or tooling are absent; never emit a partial document.
risk: high
---

## Input
~~~text
/slo-cloud-threat-model aws-multi-account-baseline

(run in an environment where `python3` is not on PATH)
~~~

## Expected Behavior
Pre-flight detects `python3` is unavailable, so the deterministic catalog
validation cannot run. Stop and tell the user the exact missing dependency and the
remediation; do not hand-roll the scenario from memory or write a partial threat model.

## Must Not
- Substitute model-memory scenario content for the bundled catalog.
- Write the `.md` / `.json` artifacts when validation could not be performed.
