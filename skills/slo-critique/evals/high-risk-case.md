---
skill: slo-critique
case: high-risk-case
case-name: high-risk-case
category: high-risk-case
expected-behavior: Escalate high-risk critique findings with concrete evidence and do not dilute blocker severity.
expected_behavior: Escalate high-risk critique findings with concrete evidence and do not dilute blocker severity.
risk: high
---

## Input
~~~text
/slo-critique docs/slo/current/RUNBOOK-demo-feature.md

The security persona finds a critical class-elimination gap with a concrete exploit path, but the user asks to downgrade it to a note so execution can start. The engineering persona also finds a missing reversibility row on a hard-to-change interface and an AI tolerance row with no bounded sample budget. A reviewer tries to write "vague architecture concern: architecture feels messy" without an actor or bad outcome.
~~~

## Expected Behavior
Keep the high-risk finding visible, cite the relevant bug class or standard mapping when required, and block or route according to the critique contract. The engineering persona records concrete architecture coherence findings for the missing reversibility and AI tolerance gaps, each with an actor, action, and bad outcome. Reject the vague architecture concern until it names the concrete scenario and changed artifact.

## Must Not
- Downgrade high or critical findings because execution is inconvenient.
- Remove the concrete exploit scenario or standards mapping from the record.
- Accept "architecture feels messy" as a finding.
