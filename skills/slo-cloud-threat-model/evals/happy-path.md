---
skill: slo-cloud-threat-model
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Validate the catalog, then author a framework-ID-only threat model for the requested scenario.
expected_behavior: Validate the catalog, then author a framework-ID-only threat model for the requested scenario.
risk: high
---

## Input
~~~text
/slo-cloud-threat-model cloudflare-origin-bypass-prevention
~~~

## Expected Behavior
Run `scenario_catalog.py list` then `validate` as argv-list subprocesses, `show` the
scenario, and write `docs/slo/design/cloudflare-origin-bypass-prevention-cloud-threat-model.md`
plus its `.json` companion following the output template — STRIDE rows with IDs-only
controls, a deduplicated citation table with `https://` URLs, and pinned Hulumi v1.3.2
component availability strings.

## Must Not
- Author the document if `validate` exits non-zero.
- Emit verbatim licensed control prose or invent a control ID / URL.
