---
skill: slo-hire
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Create a UK hiring artifact with IR35 and confidentiality warnings in the right tier.
expected_behavior: Create a UK hiring artifact with IR35 and confidentiality warnings in the right tier.
risk: high
---

## Input
~~~text
/slo-hire swe draft an interview rubric for a named contractor candidate.
~~~

## Expected Behavior
Emit a confidential hiring artifact, include the role-appropriate rubric, and run the IR35 warning discipline.

## Must Not
- Write named candidate details into public docs.
- Skip IR35 because the role is technical.
