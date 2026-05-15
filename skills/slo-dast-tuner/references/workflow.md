---
name: slo-dast-tuner-workflow
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# /slo-dast-tuner Workflow

## First Install

Run `zaprun init` against the target repo:

```text
zaprun init --target-dir <target-repo> --deployment-target <https-url>
```

If no staging URL is available, `zaprun` may emit a local default, but the skill must tell the user that live coverage remains unproven until a reachable target is supplied.

Expected target-owned outputs:

- `.zaprun/policy-pr.yml`
- `.zaprun/policy-nightly.yml`
- `.zaprun/rules.tsv`
- `.zaprun/baseline.json`
- `.zaprun/manifest.json`
- `.github/workflows/dast.yml`

The generated workflow must invoke the latest approved digest-pinned zaprun image with first arg `zaprun`. It must not call legacy ZAP scripts.

## Drift Check

Run:

```text
zaprun rederive --target-dir <target-repo>
```

If no drift is detected, record the stderr/stdout note and stop. If drift exists, `zaprun` owns any re-derivation PR creation. The skill must not hand-roll PR bodies from threat-model prose.

## Choosing Inputs

Supply a staging URL when the target is externally reachable or CI can start it. Supply an OpenAPI spec when an API has one; otherwise let `zaprun` fall back to web scan mode. Supply route/controller context when SARIF lacks endpoint/method metadata.

## Evidence Discipline

Record the selected image digest from `.zaprun/manifest.json`. Treat a generated config as planned coverage until a zaprun run produces artifacts such as `summary.json`, `coverage.json`, `observations.json`, and `zap.sarif`.
