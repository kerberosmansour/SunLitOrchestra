---
name: slo-dast-tuner-sarif-guided-scans
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# SARIF-Guided DAST Tuning

This workflow borrows the useful pattern from ZAP's "Guided ZAP Scans: Faster CI/CD Feedback Using Static Analysis" article (2026-03-27): SAST is most useful to DAST when it points to endpoint, HTTP method, and CWE, not just file and line.

## Required Classification

For each SARIF result, classify:

- `dast-detectable`: externally observable through HTTP with route/request evidence.
- `dast-partial`: some HTTP symptom is plausible, but custom replay, fixtures, or a target-owned script may be needed.
- `dast-not-applicable`: not externally observable by DAST.
- `needs-human-input`: route, auth, fixture, exploitability, or live request is unclear.

Never classify as `dast-detectable` from CWE alone.

## Preferred SARIF Facts

Look for:

- CWE tags, rule id, severity, stable fingerprint.
- endpoint path and HTTP method from result metadata.
- source file/line plus taint/dataflow trace.
- route/controller mapping to the source location.
- whether the route requires authentication or a role.
- whether an OpenAPI operation confirms the path and method.

If SARIF lacks endpoint/method, infer only from typed route/OpenAPI evidence. If inference is ambiguous, mark `needs-human-input`.

## Guided Scan Map

When M3 lands, `zaprun triage-sarif` should produce an endpoint x CWE map consumed by zaprun. Until that command exists, the skill may write a human-readable assessment, but must not claim an automated guided scan exists.

The guided PR lane is a fast feedback lane, not the whole DAST program. Keep broader scheduled scans for coverage SAST did not expose.

## False Positive And False Negative Handling

False positive candidates need replay evidence, baseline rationale, expiry, or threshold tuning. False negative candidates need a route-confirmed request, relevant ZAP rule selection, auth/fixture setup, or a target-owned custom rule candidate. Generic-rule promotion requires the rule-boundary reference.
