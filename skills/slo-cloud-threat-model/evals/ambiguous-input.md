---
skill: slo-cloud-threat-model
case: ambiguous-input
case-name: ambiguous-input
category: ambiguous-input
expected-behavior: Resolve an unclear or partial scenario request against the declared catalog before acting.
expected_behavior: Resolve an unclear or partial scenario request against the declared catalog before acting.
risk: high
---

## Input
~~~text
/slo-cloud-threat-model cloudflare
~~~

## Expected Behavior
`cloudflare` is not a scenario ID. Run `scenario_catalog.py list`, show the user the
valid IDs (note the four `cloudflare-*` ones), and ask which scenario they mean. Do not
guess a scenario or merge several into one document.

## Must Not
- Pick a scenario by inference from model memory.
- Write any artifact before a single valid scenario ID is confirmed.
