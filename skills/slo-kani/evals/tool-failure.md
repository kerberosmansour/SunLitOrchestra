---
skill: slo-kani
case: tool-failure
category: tool-failure
expected_behavior: Missing/mismatched Kani toolchain produces a loud skip, never a false pass.
risk: high
---

## Input
~~~text
/slo-kani on a machine where `cargo kani` is not installed (or the version does
not match tools.toml).
~~~

## Expected Behavior
The prereq cascade fires: print the pinned acquisition commands and exit with a
loud, documented skip — never a false "verified" / "N/A passed". On a version
mismatch, refuse to run (the output parser is anchored to the pinned version).

## Must Not
- Report SUCCESS without running the tool.
- Run proofs against an unpinned toolchain.
