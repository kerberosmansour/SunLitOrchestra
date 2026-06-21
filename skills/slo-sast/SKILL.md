---
name: slo-sast
description: >
  Use this skill to wire threat-model-driven SAST scanning into a target product
  repo. Reads docs/slo/design/<slug>-threat-model.md for CWE references, picks tuned
  Semgrep rule packs for the detected stack, emits a safe GitHub Actions workflow
  plus a baselined config plus an audit-defense manifest, and re-derives the
  ruleset on threat-model edit. Pure Markdown skill; no Rust binary dependency.
  Pair with /slo-rulegen for project-specific rules.
---

# /slo-sast — threat-model-driven Semgrep orchestration

You are a security engineer wiring SAST into a target product repo. Translate the repo threat model into tuned Semgrep configuration, a safe GitHub Actions workflow, an audit-defense manifest, and re-derivation PRs when the threat model changes.

## Shared discipline references

- Security-engineering claims follow the source hierarchy in [`../../references/templates/citation-discipline.md`](../../references/templates/citation-discipline.md).
- Subprocesses use argv-list and capture discipline from [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md); no `bash -c` shell-string interpolation.
- Pinned rule, action, and cache integrity claims follow [`../../references/templates/version-pinning-discipline.md`](../../references/templates/version-pinning-discipline.md).

## Inputs

- Target repo root as cwd, or `--target-dir`.
- `docs/slo/design/<slug>-threat-model.md`, where slug comes from the optional first positional arg or current `docs/slo/current/RUNBOOK-<SLUG>.md`.
- M2+ manifest files (`Cargo.toml`, `package.json`, `requirements.txt`, etc.) and `references/sast/scanner-orch-pinned-rules-sha.md`.

## Output Envelope

Print one JSON object with `cwes_extracted`, `detected_stack`, `selected_rules`, and `selection_strategy`; examples include `"CWE-77"`, `"CWE-89"`, `metadata.cwe`, `metadata.technology`, and `selection_strategy: "default-fallback"`. Output is deduplicated and sorted ascending; empty CWE prose returns `[]` semantics through `cwes_extracted` and exit 0.

Coverage-gap summaries may use [`../../references/security/security-assessment-summary-template.md`](../../references/security/security-assessment-summary-template.md) and findings may use [`../../references/security/security-finding-template.md`](../../references/security/security-finding-template.md). `CWE claimed vs covered` is the required mapping per [`../../references/security/standards-mapping.md`](../../references/security/standards-mapping.md); OWASP / ASVS rationale is optional.

## Pre-flight

1. Confirm cwd is a repo; otherwise exit non-zero with `/slo-sast must run inside a git repository (cwd=$PWD)`.
2. Resolve `docs/slo/design/<slug>-threat-model.md`; if unresolved, exit non-zero with `/slo-sast cannot determine slug; pass it as the first argument or run inside a directory with docs/slo/current/RUNBOOK-<slug>.md`.
3. If the threat model does not exist, exit non-zero with `threat-model not found: docs/slo/design/<slug>-threat-model.md`; do not print a partial list.
4. Optional Graphify Lens: for large/polyglot repos or noisy finding sets, `/slo-graphify` may map findings to code paths, GitHub Issues, and tests. It never replaces threat-model parsing, rule selection, or explicit finding disposition.

## Method Dispatch

| Stage | Contract |
|---|---|
| Method (M1 — parser scaffold) | [`references/methodology-m1-parser.md`](references/methodology-m1-parser.md): Threat-model parser scope rule; HTML comments, fenced code, and `~~~text` user-string fences are excluded; missing files do not emit partial output. |
| Method (M2 — stack detection + registry fetch + rule filter) | [`references/methodology-m2-stack-detect.md`](references/methodology-m2-stack-detect.md): Stack detection, Registry fetch, Rule filter, cache hit/cache miss behavior, `~/.cache/sldo/semgrep-rules/<SHA>/`, `XDG_CACHE_HOME`, `git rev-parse HEAD`, `serde_yaml_ng`, entity expansion / billion-laughs safety, argv-list subprocess discipline, no shell strings. |
| Method (M3 — emission) | [`references/methodology-m3-emission.md`](references/methodology-m3-emission.md): Emission flow, symlink traversal / `O_NOFOLLOW`, Workflow safety contract, `pull_request_target` ban, `permissions: {}`, `fetch-depth: 0`, `SEMGREP_RULES`, and byte-identical CWE-list independence. |
| Method (M4 — manifest + Preview-mode UX) | [`references/methodology-m4-manifest.md`](references/methodology-m4-manifest.md): Manifest schema v1.0, first install vs Re-derivation, mixed pre-existing state with workflow/config, rollback on decline, manifest symlink defense, defensive design not regulatory mandate. |
| Method (M5 — re-derivation loop) | [`references/methodology-m5-pr-creation.md`](references/methodology-m5-pr-creation.md): Re-derivation trigger evaluation, `scanner-orch-rederivation-triggers.md`, `no drift detected`, PR creation, argv-list `gh pr create`, no `--repo`, max 1 PR per invocation, TempDir dogfood with file-content copy, template-skeleton / manifest-derived PR body, Auto-merge forbidden. |

## Custom rule shape (when the registry under-covers the threat model)

When the threat model needs coverage the registry does not provide, project-specific rules are
authored. Empirically validated rule shape — taint mode + a destructuring propagator as the
default for request-data-flow classes, intrinsic-sink structural rules for flow-free classes,
driver-split DB sinks with the matching CWE, receiver-constrained heuristic sinks, default
doc/fixture excludes, and explicit intra-file-taint / no-SCA caveats — is specified in
[`references/custom-rule-shape.md`](references/custom-rule-shape.md). Apply it to any hand-authored
web pack; it roughly doubled recall in an adversarial NodeGoat + Juice Shop test with one net
false positive, and generalised unmodified across apps.

## Common Anti-patterns

- Treating CWE references inside HTML comments, code fences, or `~~~text` user-string fences as authoritative.
- Emitting any artifact before the stage allows it; M1 is parser-only, M3+ owns writes.
- Inferring stack or selecting rules while running only the M1 parser path.
- Falling back to a default rule pack on missing threat-model input.
- Running subprocesses outside the stage-specific contract.
- Authoring web rules that match a literal `req.*` at the sink (the dominant false-negative shape — use taint + propagator per `references/custom-rule-shape.md`).
- Implying dependency (CWE-1035/937) coverage from Semgrep alone; recommend a paired SCA step.

## See also

- [`../../references/sast/threat-model-parser-contract.md`](../../references/sast/threat-model-parser-contract.md)
- [`../../references/sast/stack-detection-contract.md`](../../references/sast/stack-detection-contract.md)
- [`../../references/sast/scanner-orch-pinned-rules-sha.md`](../../references/sast/scanner-orch-pinned-rules-sha.md)
- [`../../references/sast/scanner-orch-workflow-template.yml`](../../references/sast/scanner-orch-workflow-template.yml)
- [`../../references/sast/scanner-orch-action-shas.md`](../../references/sast/scanner-orch-action-shas.md)
- [`../../references/sast/scanner-orch-manifest-schema.md`](../../references/sast/scanner-orch-manifest-schema.md)
- [`../../references/sast/scanner-orch-rederivation-triggers.md`](../../references/sast/scanner-orch-rederivation-triggers.md)
- [`references/custom-rule-shape.md`](references/custom-rule-shape.md) — empirical custom-rule shape (taint+propagator, driver-split CWE, default excludes, coverage caveats)
- [`../../docs/slo/design/scanner-orchestration-threat-model.md`](../../docs/slo/design/scanner-orchestration-threat-model.md)
- [`../../docs/slo/design/scanner-orchestration-interfaces.md`](../../docs/slo/design/scanner-orchestration-interfaces.md)
- [`../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md`](../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md)

**Loops**: Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#security-tuning-loop](../../docs/LOOPS-ENGINEERING.md#security-tuning-loop).
