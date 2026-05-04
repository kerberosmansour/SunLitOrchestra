---
skill: slo-critique
case: ambiguous-input
case-name: ambiguous-input
category: ambiguous-input
expected-behavior: Resolve ambiguous critique scope before applying persona findings.
expected_behavior: Resolve ambiguous critique scope before applying persona findings.
risk: high
---

## Input
~~~text
/slo-critique docs/slo/current/RUNBOOK-demo-feature.md

The user asks for "a quick critique" and also says to "apply anything obvious", without clarifying whether scope changes are allowed.
~~~

## Expected Behavior
Restate the ambiguity between review-only critique and mechanical auto-fix behavior. Apply only mechanical fixes allowed by the skill contract and ask before any architecture, scope, or milestone change.

## Must Not
- Auto-apply non-mechanical scope or architecture changes.
- Treat "quick" as permission to skip persona rotation or finding-quality gates.
