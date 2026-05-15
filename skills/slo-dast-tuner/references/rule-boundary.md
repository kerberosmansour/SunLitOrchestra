---
name: slo-dast-tuner-rule-boundary
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Rule Boundary

The load-bearing rule: shared repos may contain generic scanner policy, schemas, validators, and reusable examples. App-specific custom rules stay in the target repo or run artifacts.

## Generic Rule Candidate

A rule can be proposed for zaprun only when it:

- detects a vulnerability class across more than one application shape
- avoids private endpoint names, tenant ids, fixture ids, product strings, and bespoke response text
- has synthetic vulnerable and patched fixtures
- documents supported surface: API, web, SPA, or framework-scoped
- passes the future DAST verify gate

Framework-specific rules are allowed only when the framework behavior is the class under test and the scope is named.

## Target-Owned Rule Candidate

A rule must remain target-owned when it depends on:

- one private route or schema
- a tenant/account/fixture value
- app-specific auth/session shape
- proprietary response text
- a SARIF result that has not been generalized
- manual setup that cannot be reproduced in synthetic fixtures

Target-owned scripts may live under `.zaprun/scripts/` or an ignored run-artifact directory in the target repo.

## Promotion Flow

1. Confirm the issue with zaprun evidence or replay.
2. Write the narrow target-owned candidate if needed.
3. Extract the vulnerability class and remove app literals.
4. Add synthetic vulnerable and patched fixtures.
5. Run the generic-rule gate when M4 lands.
6. Only then propose a shared zaprun rule.

Skipping steps 3 to 5 is a false-negative fix for one app, not a generic scanner improvement.
