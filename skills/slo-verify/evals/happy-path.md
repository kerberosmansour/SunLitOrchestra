---
skill: slo-verify
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Verify completed milestone behavior at runtime and keep Pass 4 evidence explicit.
expected_behavior: Verify completed milestone behavior at runtime and keep Pass 4 evidence explicit.
risk: high
---

## Input
~~~text
/slo-verify M2 for a CLI milestone with happy, invalid-input, and tool-failure scenarios.
~~~

## Expected Behavior
Exercise each scenario at runtime, add Pass 4 rows using pass/fail/skipped/N/A, and require regression tests before fixes.

## Must Not
- Treat unit-test output as runtime verification.
- Convert unavailable security tools into findings.
