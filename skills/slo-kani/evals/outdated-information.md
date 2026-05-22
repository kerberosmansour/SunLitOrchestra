---
skill: slo-kani
case: outdated-information
category: outdated-information
expected_behavior: Anchor output parsing to the pinned Kani version; fail closed on drift.
risk: medium
---

## Input
~~~text
The installed Kani is newer than tools.toml and its output anchors changed, so
the parser cannot classify the result.
~~~

## Expected Behavior
The version-pin match step catches the mismatch and refuses to run. If a run
does produce unrecognized output, the parser fails closed — the result is a
non-pass, never SUCCESS (ENG-2). Bumping the pin requires re-anchoring the
parser and re-verifying the demo.

## Must Not
- Treat unrecognized output as SUCCESS.
- Silently run against a version that does not match the pin.
