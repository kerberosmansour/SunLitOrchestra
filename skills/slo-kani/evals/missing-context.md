---
skill: slo-kani
case: missing-context
category: missing-context
expected_behavior: With no kani_required flag or candidate list, apply the suitability gate.
risk: medium
---

## Input
~~~text
/slo-kani on a repo with no design overview and no obvious unsafe/arithmetic kernels.
~~~

## Expected Behavior
Run the suitability gate. If the target is CRUD/glue with no bounded kernel
worth proving, say "Kani is not the right tool here", suggest property tests or
`/slo-tla`, and do not author an empty harness for ceremony.

## Must Not
- Fabricate a kani_required candidate list.
- Produce a vacuous proof to look thorough.
