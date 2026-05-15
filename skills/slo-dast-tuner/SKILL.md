---
name: slo-dast-tuner
description: >
  Use this skill to wire and tune DAST for an authorized web app or web service
  through zaprun only. Operates OWASP ZAP via the latest approved digest-pinned
  ghcr.io/kerberosmansour/zaprun image, reads threat models, OpenAPI/routes/auth
  context, and SAST SARIF, then tunes generic DAST policy while keeping
  app-specific custom rules in the target repo or run artifacts.
---

# /slo-dast-tuner - zaprun-backed DAST tuning

You are a security engineer wiring DAST into a target repo. Treat ZAP as an implementation detail owned by `zaprun`; this skill orchestrates `zaprun` commands and records evidence.

## Non-negotiable Boundaries

- Run ZAP only through `zaprun` and the latest approved digest-pinned `ghcr.io/kerberosmansour/zaprun@sha256:<digest>` image.
- Never call `zap-baseline.py`, `zap-full-scan.py`, `zap-api-scan.py`, or handwritten user-derived Automation Framework YAML.
- Treat SAST SARIF as evidence, not proof of DAST coverage.
- Do not commit app-specific custom rules to SunLitOrchestra or zaprun. Keep them under the target repo's `.zaprun/scripts/`, an ignored scratch directory, or run artifacts.
- Before touching a live target, confirm authorization, in-scope URL(s), auth permission, and any rate/concurrency limits.

## Inputs

- Target repo root, either cwd or explicit `--target-dir`.
- `docs/slo/design/<slug>-threat-model.md` when present.
- Optional OpenAPI spec, route/controller source, framework manifests, existing `.zaprun/manifest.json`.
- Optional SAST SARIF file(s).
- Optional staging URL and auth material. Credentials must never be written to committed files.

## Resolve the Runner

Prefer an installed `zaprun`. If unavailable and a sibling zaprun checkout exists, run `cargo run -p zaprun --` from that checkout. If neither is available, ask for the zaprun checkout path or install instructions; do not invent ZAP commands.

Use argv-list subprocess discipline from [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md). Never interpolate target URLs, SARIF paths, headers, or credentials into shell strings.

## Method Dispatch

| Intent | Action |
|---|---|
| First install | Read [`references/workflow.md`](references/workflow.md), then run `zaprun init --target-dir <repo> [--deployment-target <url>]`. |
| Drift check | Read [`references/workflow.md`](references/workflow.md), then run `zaprun rederive --target-dir <repo>`. |
| SARIF-guided tuning | Read [`references/sarif-guided-scans.md`](references/sarif-guided-scans.md). When M3 is available, run `zaprun triage-sarif`; until then, classify the evidence and do not claim automated tuning. |
| Authenticated coverage | Read [`references/authentication-coverage.md`](references/authentication-coverage.md). Auth failure is a coverage failure, not a clean scan. |
| Rule authoring or promotion | Read [`references/rule-boundary.md`](references/rule-boundary.md) before proposing any custom script or generic rule. |

## Expected Output

Report:

- commands run, cwd, exit codes, and artifact paths
- selected profile and image digest
- CWEs claimed versus covered, plus coverage gaps
- SARIF findings classified as `dast-detectable`, `dast-partial`, `dast-not-applicable`, or `needs-human-input`
- auth coverage status and diagnostics path when applicable
- any target-owned custom rule candidates and why they are not generic yet

## Anti-patterns

- Saying "DAST covered" because SARIF mentioned a CWE.
- Promoting a one-app script into a shared rule because it found one real bug.
- Lowering thresholds to hide findings without expiry and rationale.
- Reporting "no vulnerabilities" when crawling, auth, fixtures, or endpoint reachability failed.
- Committing credentials, session cookies, auth diagnostics, private SARIF, or app-specific scripts to a public/shared repo.

**Loops**: Security-tuning loop - see [docs/LOOPS-ENGINEERING.md#security-tuning-loop](../../docs/LOOPS-ENGINEERING.md#security-tuning-loop).
