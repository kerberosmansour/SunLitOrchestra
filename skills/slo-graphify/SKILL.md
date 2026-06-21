---
name: slo-graphify
description: >
  Use this skill when a product repo needs graph-backed codebase understanding,
  fast troubleshooting, or security triage. It combines Graphify knowledge
  graphs with semantic-provider facts from rust-analyzer / TypeScript language
  service and analysis-provider findings from OpenGrep or Semgrep-compatible
  rule packs. It is designed for private repos: raw evidence stays out of git;
  only anonymized summaries may be committed.
---

# /slo-graphify

You are a graph-backed investigation lead. Your job is to turn a large repo into
useful, queryable evidence for an engineer and for an AI coding agent, without
leaking private implementation details.

Use this for three lanes:

- Knowledge: find the right files, concepts, owners, dependency paths, and
  domain seams faster than grep-first exploration.
- Troubleshooting: connect GitHub issues, stack traces, retries, error handling,
  route handlers, workers, and cross-language boundaries.
- Security: combine semantic facts and OpenGrep findings into a ledger of
  risks, controls, tests to add, and dispositions.

## Required Reading

- [`../../references/graphify/provider-evidence-loop.md`](../../references/graphify/provider-evidence-loop.md)
- [`../../docs/LOOPS-ENGINEERING.md`](../../docs/LOOPS-ENGINEERING.md)
- [`../../docs/SECURE-VALUE-LOOP.md`](../../docs/SECURE-VALUE-LOOP.md)
- [`../slo-sast/SKILL.md`](../slo-sast/SKILL.md) when security scanning is in scope.
- [`../slo-rulegen/SKILL.md`](../slo-rulegen/SKILL.md) when extending Rust rules.

## Inputs

- Target repo root as cwd, or an explicit target path from the user.
- Optional GitHub Issues list or issue numbers to use as the value lens.
- Optional rule packs: OpenGrep/Semgrep YAML, SunLit Orchestra `.semgrep/rust`,
  or approved project-specific rules.
- Optional prior SLO artifacts: runbook, threat model, incident notes, or
  troubleshooting notes.

## Confidentiality Gate

Before any scan or graph build:

1. Confirm whether the target repo is private.
2. Put raw outputs under an ignored directory such as
   `experiments/<slug>/private-target/<run-id>/raw/`.
3. Do not commit target repo names, absolute paths, source snippets, secrets,
   customer data, issue bodies, or proprietary identifiers.
4. Commit only anonymized summaries: counts, classes, generic path shapes,
   query categories, and recommendations.
5. If you cannot prove raw evidence is ignored, stop and add/update `.gitignore`
   before scanning.

## Preflight

Run these checks using argv-list subprocess discipline:

```text
sldo-install graphify --install-plan
sldo-install graphify
graphify --help
opengrep --version
rust-analyzer --version
node --version
```

If TypeScript semantic facts are required, also prove the target repo can load a
TypeScript project:

```text
npm exec -- tsc --version
```

Missing OpenGrep blocks the security lane. Missing rust-analyzer or TypeScript
language service blocks any claim that real semantic providers were tested.

## Method

1. Read GitHub Issues first when available. Bucket each issue as `knowledge`,
   `troubleshooting`, `security`, `qa-risk`, or `unclear`.
2. Build or refresh the Graphify graph for the target repo.
3. Add provider facts:
   - `syntax` facts from Graphify or local extractors.
   - `semantic` facts from rust-analyzer and TypeScript language service.
   - `analysis` facts from OpenGrep/Semgrep-compatible rules.
4. Normalize facts with provider name, provider kind, confidence, source range,
   run id, and timestamp.
5. Query for evidence that helps a person and an agent:
   - "Which files should I open first for this issue?"
   - "What code path crosses TypeScript and Rust?"
   - "Which findings map to user-visible bugs?"
   - "Which security findings have tests and ledger dispositions?"
   - "Which QA failures are waiting to happen?"
6. Write a plain-English readout with a table of capabilities, not a dump of
   raw findings.

## Readout Shape

Use this table shape in the final answer or artifact:

| Lane | What the graph found | Why useful | Next action |
|---|---|---|---|
| Knowledge | `<anonymized signal>` | `<how it reduces file/tool churn>` | `<query or doc follow-up>` |
| Troubleshooting | `<anonymized signal>` | `<how it points to likely root cause>` | `<test or issue follow-up>` |
| Security | `<anonymized signal>` | `<risk/control/test impact>` | `<fix, issue, accepted risk, or human review>` |
| QA risk | `<anonymized signal>` | `<bug waiting to happen>` | `<regression test or refactor>` |

## Stop Conditions

- The target is private and raw outputs are not ignored.
- The run depends on stub semantic providers but the readout claims real
  rust-analyzer or TypeScript language-service validation.
- Findings cannot be mapped to files, issues, controls, or explicit "not enough
  evidence" buckets.
- The readout would disclose target repo identity or proprietary snippets.
