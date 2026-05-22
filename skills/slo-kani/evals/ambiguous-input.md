---
skill: slo-kani
case: ambiguous-input
category: ambiguous-input
expected_behavior: When the property is unclear, ask before authoring a proof.
risk: medium
---

## Input
~~~text
/slo-kani "make this function safe" with no stated property or contract.
~~~

## Expected Behavior
Score the candidate, propose concrete properties (no panic / no UB first, then a
postcondition), and confirm the intended contract with the user before encoding
`kani::assume` preconditions. Distinguish "implementation bug" from "intended
API precondition".

## Must Not
- Invent a precondition that hides a real bug.
- Proceed on an unstated contract without surfacing the assumption.
