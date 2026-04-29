---
name: slo-sec-libs
created: 2026-04-27
status: design lock-in
tla_required: false
security_libs_required: true
ai_component: false
compliance: [soc2, asvs]
---

# Design overview — /slo-sec-libs

## System goal

Add a `/slo-sec-libs` skill that reads a target repo's `ARCHITECTURE.md` + `stack-decision.md` + the runbook's per-milestone proactive-control requirements, matches each requirement against capabilities advertised by Hulumi + SunLitSecureLibraries via CycloneDX 1.6+ `declarations`, and either (a) recommends a specific library component covering the requirement or (b) files a structured capability-gap issue against the library owner's intake repo (default: `kerberosmansour/slo-security-intake`; third-party gated by `--file-upstream` flag + per-session 40-issues/hr cap).

## Stack decision

Same shape as the rest of the SLO toolchain, with one new external surface:

- Markdown for SKILL.md
- Rust (`crates/sldo-common::toolflags`) for skill-flag enforcement
- **Python subprocess** for CycloneDX 1.6 `declarations` jsonschema validation (Rust ecosystem has no 1.6+ declarations support — `cyclonedx-bom 0.8.1` is spec-1.5 only per Phase 1 research Q2)
- `git` and `gh` CLI for upstream filing (argv-list discipline; no `--repo` flag)

New runtime dependency: Python 3.10+ with `jsonschema` library available locally. The skill checks `which python3` + `python3 -c "import jsonschema"` in pre-flight; fails with install hint if missing. No SLO-vendored Python package — uses system Python.

## Components

| Component | Responsibility | Milestone | Key interfaces |
|---|---|---|---|
| `skills/slo-sec-libs/SKILL.md` | Skill orchestrator; mode dispatch; pre-flight | M1-M5 | Reads target repo's `ARCHITECTURE.md`, `docs/design/<slug>-stack-decision.md`, `docs/RUNBOOK-<slug>.md` |
| `skills/slo-sec-libs/scripts/read-declarations.py` | Python jsonschema validator + capability extractor | M1 | Input: CycloneDX 1.6 JSON; output: structured capability catalog (stdout JSON) |
| `skills/slo-sec-libs/references/methodology-m1-reader.md` | Declarations reader spec | M1 | Cited from SKILL.md |
| `skills/slo-sec-libs/references/methodology-m2-matcher.md` | Capability matching algorithm | M2 | Cited from SKILL.md |
| `skills/slo-sec-libs/references/capability-gap-schema.md` | Regex-validated schema for gap records (issue body shape) | M3 | Cited by M3 + M4 |
| `skills/slo-sec-libs/references/upstream-filing-discipline.md` | argv-list rules + no-`--repo` rule + rate-limit cap | M3-M4 | Cited from SKILL.md |
| `crates/sldo-common::toolflags::sec_libs_deny_flags()` | Skill-flag denial: `WebFetch`, `WebSearch` denied | M1 | Enforced at SLO-CLI invocation layer |
| `kerberosmansour/slo-security-intake` repo | Default capability-gap filing destination | One-time prereq (out-of-band) | Issue template: `capability-gap-record` per the schema |
| Hulumi + SunLitSecureLibraries CycloneDX 1.6 declarations | Capability advertisement | One-time prereq | Published as JSON in each repo's release artifacts |

## Data flow

```
Target repo with ARCHITECTURE.md + stack-decision.md + RUNBOOK
       │
       ▼
┌──────────────────────────────────────────────────┐
│ /slo-sec-libs invoked                              │
│ Pre-flight:                                        │
│   - Python 3.10+ + jsonschema available           │
│   - target repo has ARCHITECTURE.md               │
│   - runbook declares security_libs_required: true │
└────────────────────┬─────────────────────────────┘
                     │
                     ▼
        ┌────────────────────────────┐
        │ Fetch CycloneDX declarations │ ← cached at ~/.cache/sldo/declarations/<sha>
        │ from Hulumi + SunLitSecureLibraries
        │ (pinned SHA per stack-decision)
        └────────────┬───────────────┘
                     │
                     ▼
        ┌────────────────────────────┐
        │ scripts/read-declarations.py │ ← Python jsonschema validate
        │ extract capability catalog   │   reject malformed
        └────────────┬───────────────┘
                     │
                     ▼
        ┌────────────────────────────┐
        │ Match runbook proactive-    │
        │ controls rows against the   │
        │ catalog                     │
        └────────────┬───────────────┘
                     │
            ┌────────┴────────────────┐
            ▼                         ▼
   ┌────────────────┐         ┌──────────────────┐
   │ MATCH found:   │         │ NO MATCH:         │
   │ recommend lib  │         │ produce capability-│
   │ component      │         │ gap record         │
   │ + parametrics  │         │ (regex-validated)  │
   └────────────────┘         └────────┬─────────┘
                                       │
                                       ▼
                            ┌──────────────────┐
                            │ User confirms    │
                            │ filing           │
                            └────────┬─────────┘
                                     │
                            ┌────────┴────────┐
                            ▼                 ▼
                   ┌──────────────┐   ┌──────────────┐
                   │ Default:     │   │ --file-upstream:│
                   │ slo-security-│   │ third-party    │
                   │ intake repo  │   │ owner repo     │
                   │              │   │ (rate-limited) │
                   └──────────────┘   └──────────────┘
                            │                 │
                            └────────┬────────┘
                                     ▼
                           gh issue create
                           (argv-list, NO --repo)
```

## Trust boundaries

- `~/.cache/sldo/declarations/<sha>/` — local cache; SHA-pinned per declaration source. Compromise of the local user account compromises both the cache and the target repo.
- `gh` CLI auth — user-controlled GitHub auth; same trust posture as `/slo-ship`.
- Upstream intake repo — public (when published); recipients of capability-gap records are the intake repo's maintainers.
- Target repo's `ARCHITECTURE.md` / `stack-decision.md` content — author-controlled; not user-input. Low injection risk; argv-list discipline is the standard defense.

## Interfaces locked

| Interface | Stability | Notes |
|---|---|---|
| CycloneDX 1.6 `declarations` schema URL + pinned SHA | `stable` per runbook | Bumping is `/slo-architect` re-pass discipline |
| Capability-gap record schema (regex-validated fields) | `stable` | Cross-org consumers (Hulumi, SunLitSecureLibraries) parse this |
| `cdx:sunlit:crypto:*` namespace | `evolving` | Migrates to upstream Property Taxonomy post-MVP |
| `--file-upstream` flag semantics | `stable` | Third-party filing requires this flag; no implicit upstream |
| Per-session 40-issues/hr rate-limit cap | `stable` | Defensive; reuses pattern in [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) (R1) |
| `gh pr create` discipline (argv-list, no `--repo`, no merge flags) | `stable-interface` | Inherited from `/slo-sast` M5 |

## TLA+ section

Not required (`tla_required: false`). No concurrent actors. The skill is single-session, single-actor, sequential (read declarations → match → emit OR file). Rate-limit cap is per-session client-side, not a distributed state machine.

## STRIDE sweep (per Step 3.5)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | EoP |
|---|---|---|---|---|---|---|
| CycloneDX declarations fetch | mitigated — pinned SHA per source; `git rev-parse HEAD` cache integrity check | mitigated — SHA verification rejects in-flight tampering | N/A | N/A — public capability content | mitigated — rate-limit cap on filing endpoint; declarations fetch is one-shot per session | N/A |
| `read-declarations.py` (Python subprocess) | N/A | mitigated — strict jsonschema validate; reject malformed | N/A | N/A — output is capability catalog (no PII) | mitigated — bounded JSON file size (refuse > 10 MiB) | N/A |
| Capability matching | N/A | N/A | N/A | N/A | N/A | N/A |
| Capability-gap filing (gh issue create) | mitigated — uses user's `gh` auth; argv-list only | mitigated — issue body content from regex-validated record schema; no free-text from user-input | mitigated — every filing carries `gh` author + timestamp | residual — gap content goes to public intake repo (default) or third-party repo (gated); mitigated by user-confirmation gate | mitigated — 40 issues/hr cap | N/A |

New abuse cases (extends `docs/design/slo-security-embedding-threat-model.md`):

- `tm-slo-sec-libs-abuse-1: tampered declarations file inflates capability claims to bypass real review` — class eliminated by SHA pinning + `git rev-parse HEAD` cache integrity check.
- `tm-slo-sec-libs-abuse-2: capability-gap record body splices attacker-supplied prose from target repo content` — class eliminated by regex-validated schema; only structured fields flow into issue body.
- `tm-slo-sec-libs-abuse-3: cross-org filing storm via --file-upstream loop` — mitigated by client-side 40-issues/hr cap + user-confirmation gate per filing.
- `tm-slo-sec-libs-abuse-4: confused-deputy via tampered .git/config redirecting gh` — mitigated by NO `--repo` flag (inherits SEC-8 from `/slo-sast` M5).
- `tm-slo-sec-libs-abuse-5: malicious CycloneDX file triggers Python jsonschema infinite recursion (billion-laughs analog)` — mitigated by strict jsonschema validate (no entity expansion / anchor recursion); 10 MiB file size cap before parse.

## Compatibility commitments

- Phase 1 security-aware skills (`/slo-ideate`, `/slo-architect`, `/slo-plan`, `/slo-critique`, `/slo-verify`) unchanged; `/slo-sec-libs` is a new skill, additive.
- Existing runbooks without `security_libs_required: true` see `/slo-sec-libs` exit cleanly with "no security-libs required for this runbook".
- Runbooks with `security_libs_required: true` but no `ARCHITECTURE.md` see a clear error pointing at `/slo-architect`.
- The `update-config` skill is the canonical mutation surface for any settings.json work (parallels R2's `/slo-freeze` hook).

## Out-of-scope

- Rust CycloneDX 1.6+ emitter work (Phase 1 research Q1 found `cyclonedx-bom 0.8.1` is 1.5 only; emitter work is a separate program).
- Phase 2 `/slo-threat-model` (parked; `/slo-sec-libs` is reader-side only — no SecOpsTM dependency).
- Phase 3 `/slo-security-test` (separate program; wraps the existing Pass 4 command catalog).
- Vendor SaaS API fallbacks (Semgrep AppSec, Snyk, GitHub Advanced Security) — explicitly rejected per Phase 1 stack-decision.

## Pre-requisites (one-time, before runbook M1)

These are out-of-band of the runbook (per [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)):

- [ ] Create `kerberosmansour/slo-security-intake` repo (issue-tracker-only); populate `ISSUE_TEMPLATE/capability-gap-record.md` per the M3 schema.
- [ ] Add CycloneDX 1.6 `declarations` JSON to [`kerberosmansour/hulumi`](https://github.com/kerberosmansour/hulumi) and [`SunLitSecureLibraries`](https://github.com/SunLitSecureLibraries/SunLitSecureLibraries). Each crate / component advertises the controls it implements; crypto-primitive parametric claims ride in `properties` under the vendored `cdx:sunlit:crypto:*` namespace.
- [ ] Confirm `gh` CLI scopes (`repo` or `public_repo` for same-owner filing; `repo` for cross-repo fork+PR fallback) on contributor machines.

The runbook's Background Context section flags these as required pre-flight before M1.

## Research-validation discipline (load-bearing)

Every claim in this runbook touching CycloneDX schema, GitHub API behavior, or Octokit rate-limit point cost must be source-verified against:

| Claim category | Primary source | Acceptance |
|---|---|---|
| CycloneDX 1.6 schema | https://cyclonedx.org/docs/1.6/json/ + pinned schema file SHA | quote + URL + retrieval date |
| GitHub API behavior (issue create / search / list) | https://docs.github.com/en/rest at API version pin | quote + version + retrieval date |
| GitHub Actions / `pull_request_target` ban (inherited from `/slo-sast`) | GitHub Docs + Trail of Bits' `audit-action` rationale | quote + retrieval date |
| Secondary rate-limit point costs | Public Octokit benchmarks + GitHub support commentary | quote + URL + retrieval date |
| Argon2id parameters (in `cdx:sunlit:crypto:*` namespace) | OWASP Password Storage Cheat Sheet + RFC 9106 | quote + version + retrieval date |
| jsonschema billion-laughs / entity expansion defenses | Python `jsonschema` library docs at pinned version | quote + version |

**Bright-line**: unverifiable claims removed, not weakened. The bar is set in `references/templates/citation-discipline.md` (R2 M1) and applies to every milestone of this runbook that emits a regex-validated field or a schema constraint.
