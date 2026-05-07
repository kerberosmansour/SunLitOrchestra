---
skill: slo-critique
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Run the four critique personas against a runbook and record only concrete, actionable findings.
expected_behavior: Run the four critique personas against a runbook and record only concrete, actionable findings.
risk: high
---

## Input
~~~text
/slo-critique docs/slo/current/RUNBOOK-demo-feature.md

The runbook has backend work, a small UI surface, abuse-case BDD rows, and a threat model with named trust boundaries. Its code-map has a four-object summary and exemplar rows; one milestone has an exemplar mismatch where the runbook cites an exemplar absent from the code-map.
~~~

## Expected Behavior
Run CEO, engineering, security, and design personas in order. The engineering persona runs the architecture coherence check against the four-object summary, reversibility rows, exemplar / anti-exemplar rows, and AI tolerance rows. Record findings in the shared critique table only when each finding has a concrete actor, action, bad outcome, and actionable recommendation.

## Must Not
- Blend all personas into one generic review voice.
- Emit vague findings that do not include a concrete scenario.
- Accept an exemplar mismatch as a vague style concern; it must recommend updating either the Contract Block row or the code-map.
