### Milestone X — dogfood PROVEN case (remediated)

This fixture represents the same milestone after remediation: the Outcome
Scenario asserts a real front-to-end user outcome, and every Regression Matrix
row is resolved. The Outcome Validation gate MUST pass it.

#### Outcome Scenarios

| ID | Type | Scenario (Given / When / Then + And…) |
|---|---|---|
| oc-dogfood-1 | user value | Given an endpoint contains a plaintext AWS credential When the scan runs Then the AWS key is detected And the finding severity is High And the finding appears in findings history And the finding survives application restart |

#### Critical User Journeys

| ID | Journey |
|---|---|
| cuj-dogfood-1 | secret exists -> scan -> finding -> risk visible -> remediate |

#### Core Capability Regression Matrix

| Capability | Must still pass | Evidence path | Resolution |
|---|---|---|---|
| Login / auth | yes | tests/outcome/login.rs | pass |
| Findings | yes | tests/outcome/findings.rs | pass |
