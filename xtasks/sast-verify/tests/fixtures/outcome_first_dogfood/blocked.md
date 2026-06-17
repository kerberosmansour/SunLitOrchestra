### Milestone X — dogfood BLOCKED case (deliberately unproven outcome)

This fixture represents a value-bearing milestone whose outcome is NOT proven:
an Outcome Scenario satisfied only by a mock, and a Core Capability Regression
Matrix with a blank resolution cell. The Outcome Validation gate (Pass 0 +
`/slo-retro` refusal) MUST block it.

#### Outcome Scenarios

| ID | Type | Scenario (Given / When / Then + And…) |
|---|---|---|
| oc-dogfood-1 | user value | Given a secret exists When scan runs Then assert(true) |

#### Critical User Journeys

| ID | Journey |
|---|---|
| cuj-dogfood-1 | secret exists -> scan -> finding -> risk visible -> remediate |

#### Core Capability Regression Matrix

| Capability | Must still pass | Evidence path | Resolution |
|---|---|---|---|
| Login / auth | yes | | |
| Findings | yes | tests/outcome/findings.rs | pass |
