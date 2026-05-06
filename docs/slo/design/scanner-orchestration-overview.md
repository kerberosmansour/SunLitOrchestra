---
name: scanner-orchestration
created: 2026-04-26
status: design-locked
tla_required: false
tla_reason: >
  Markdown skill emits config + workflow files into a target repo. No concurrent
  actors sharing state, no distributed consensus, no leader election, no
  cross-process ordering guarantees, no resource leases. Every step is sequential
  file I/O plus subprocess invocation (git clone of semgrep-rules at a pinned SHA).
security_libs_required: false
security_libs_reason: >
  This skill produces SAST scan artifacts (workflow YAML, rule selection,
  manifest). Hulumi and SunLitSecurityLibraries are runtime libraries downstream
  user apps consume; they are orthogonal to scanner orchestration. No capability
  manifest match is required for /slo-sast itself.
ai_component: true
ai_component_reason: >
  /slo-sast is interpreted by Claude Code reading SKILL.md plus a user-authored
  threat-model file. The MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad
  applies to two surfaces: prompt injection via the threat-model file content,
  and tool poisoning via tampered semgrep-rules at the upstream fetch step.
compliance: [soc2, asvs]
compliance_reason: >
  Defaults — SLO is the orchestrator, not the regulated app. The skill's emitted
  artifacts (manifest.json, SARIF) feed the user's downstream PCI DSS 6.2.3 (v4.0.1)
  and SOC 2 CC7.1 evidence trail, but those frameworks apply to the user's project,
  not SLO. Downstream users opt into other frameworks via their own /slo-architect run.
---

# Design — Scanner Orchestration (`/slo-sast` v1)

## System goal

Ship `/slo-sast` as a pure-Markdown Claude Code skill that reads a project's `docs/slo/design/<slug>-threat-model.md`, extracts the named CWE list, picks tuned Semgrep rule packs covering the CWE × stack intersection, emits a safe `.github/workflows/sast.yml` (`on: pull_request`, never `pull_request_target`) plus a baselined `.semgrep.yml` plus the selected rule files committed under `.semgrep/rules/`, writes a `.semgrep/manifest.json` audit-defense artifact, and re-derives the ruleset whenever the threat model changes — surfacing the diff for human review before the redeploy lands.

The wedge is the **threat-model → ruleset selection mapping** plus the **auto-tuning loop**, not the scanner itself. Semgrep does the scanning; Claude does the orchestration.

## Planned architecture (this feature)

Solid lines exist today. Dashed lines are added by the v1 runbook (M1–M5).

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                              USER (target product repo)                          │
└──────────────────────────────────┬───────────────────────────────────────────────┘
                                   │ /slo-sast
                                   ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│                          Claude Code (skill loader)                              │
└──────────────────────────────────┬───────────────────────────────────────────────┘
                                   │ reads SKILL.md (Markdown only)
                                   ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│                     /slo-sast skill   (skills/slo-sast/SKILL.md)         dashed  │
│                                                                                  │
│  ┌────────────────────────┐    ┌────────────────────────────────────┐            │
│  │ docs/slo/design/<slug>-    │ ──►│ Parse CWE-\d+ refs (regex)         │            │
│  │   threat-model.md      │    └────────────────┬───────────────────┘            │
│  └────────────────────────┘                     │                                │
│                                                 ▼                                │
│  ┌────────────────────────┐    ┌────────────────────────────────────┐            │
│  │ Cargo.toml /           │ ──►│ Detect stack (lang × framework)    │            │
│  │ package.json / etc.    │    └────────────────┬───────────────────┘            │
│  └────────────────────────┘                     │                                │
│                                                 ▼                                │
│  ┌────────────────────────┐    ┌────────────────────────────────────┐            │
│  │ ~/.cache/sldo/         │ ◄──│ Fetch semgrep-rules @ pinned SHA   │◄────────── github.com/
│  │   semgrep-rules/<SHA>/ │    └────────────────┬───────────────────┘    │       semgrep/
│  └────────────────────────┘                     │                        │       semgrep-rules
│                                                 ▼                        │       (trust boundary:
│                              ┌────────────────────────────────────┐      │        external network)
│                              │ Filter: metadata.cwe ∋ CWE-NN ∧    │      │
│                              │   metadata.technology ∋ <stack>    │      │
│                              └────────────────┬───────────────────┘      │
│                                               │                          │
│         ┌────────────────────┬────────────────┼─────────────────┐        │
│         ▼                    ▼                ▼                 ▼        │
│  .semgrep/rules/       .semgrep.yml    .github/workflows/   .semgrep/    │
│  <selected rule        (config:        sast.yml             manifest.json│
│   files committed,     ./.semgrep/     (safe template:      (cwes_claimed│
│   from registry        rules/)         on: pull_request,     vs covered, │
│   pin)                                 fetch-depth: 0,       threat-model│
│                                        permissions: {},      sha,        │
│                                        SHA-pinned actions)   rules sha,  │
│                                                              version,    │
│                                                              timestamp)  │
└────────────────────────────┬─────────────────────────────────────────────┘
                             │ git add + commit + PR
                             ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Target repo (PR opened against user's repo)                                     │
│                                                                                  │
│  On PR / scheduled — GitHub Actions runs (out of SLO scope; emitted artifact):   │
│    actions/checkout@<SHA> (fetch-depth: 0)                                       │
│    semgrep ci  →  SARIF  →  github/codeql-action/upload-sarif@<SHA>              │
│                                          │                                       │
│                                          ▼                                       │
│                               GitHub Code Scanning UI                            │
│                               (PR review comments,                               │
│                                Security tab findings)                            │
│                                                                                  │
│  Trust boundary: PR contributor (potentially fork) ↔ workflow secrets.           │
│  Hard ban: pull_request_target. Even after 2025-12-08 default-branch sourcing    │
│  mitigation, pull_request_target confers full secret access.                     │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### Legend

- **Solid lines / boxes**: exist today (Claude Code, the skill-loader pattern, Markdown SKILL.md convention, target repo filesystem).
- **Dashed lines / boxes**: added by the v1 runbook (the `/slo-sast` skill, all four emitted artifacts).
- **External trust boundary** (network arrow to upstream): semgrep-rules clone at pinned SHA — integrity via SHA, not chain-of-trust.
- **Per-PR trust boundary** (in target repo box): the emitted workflow's `pull_request` event isolates fork-PR contributors from upstream secrets.

## Auto-tuning loop

```
threat-model.md edit → /slo-sast diff → review PR → merge → CI tightened
        ▲                                                          │
        │                                                          │
        │      (next CVE / threat-model refinement)                │
        └──────────────────────────────────────────────────────────┘
```

This loop is the wedge. Without it, the skill is "Semgrep on a timer." With it, scanner config tracks the project's actual fear surface as it evolves.

## What's deferred to v2

- DAST orchestration (`/slo-dast`) — separate runbook.
- Two-workflow split (`pull_request` analysis + `workflow_run` commenter) for fork-PR comments — v1 relies on Code Scanning UI for PR feedback.
- `/slo-rulegen` integration (project-specific custom rules deployed by `/slo-sast`) — second product per idea doc Q4.
- CycloneDX TM-BOM input format — re-evaluate 2026-10 if it ships consumers.
- Coverage-doc skill that reads the manifest and produces audit-ready PDF/SARIF for PCI 6.2.3 / SOC 2 CC7.1 — out of v1 scope; v1 manifest schema must support it.

## Constraints carried forward from research

- **Severity gating doesn't exist standalone** — rule selection is the only gate. The skill's promise is "we picked tight rules so all findings block."
- **`fetch-depth: 0` is mandatory** in the emitted workflow — `actions/checkout`'s default 1 breaks `semgrep ci` diff-aware scans.
- **Registry `metadata.cwe` is canonical** — query at scan time, no hand-curated table; tolerate legacy schema gaps.
- **PCI DSS 6.2.3 v4.0.1** (NOT 6.3.2 v3.2.1) is the relevant code-review clause; v4.0.1's 6.3.2 is the SBOM-inventory mandate (different scope).
- **The wedge is genuinely unoccupied** — no published OTM-or-similar → Semgrep config converter exists.
