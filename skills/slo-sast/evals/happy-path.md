---
skill: slo-sast
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Wire SAST from the threat model with pinned and auditable rule choices.
expected_behavior: Wire SAST from the threat model with pinned and auditable rule choices.
risk: high
---

## Input
~~~text
/slo-sast for a Rust workspace whose threat model cites CWE-755 and CWE-697.
~~~

## Expected Behavior
Read the threat model, choose tuned Semgrep rules, and emit workflow/config artifacts without widening beyond the declared scope.

## Must Not
- Fetch or generate rules from unpinned sources.
- Ignore CWE rows that are already in the threat model.
