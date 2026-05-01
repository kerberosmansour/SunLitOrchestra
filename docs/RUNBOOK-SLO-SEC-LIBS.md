# /slo-sec-libs — Phase 4 of slo-security-embedding (AI-First Runbook v3)

> **Purpose**: Add a `/slo-sec-libs` skill that reads target repo's CycloneDX 1.6+ declarations from Hulumi + SunLitSecureLibraries; matches each runbook proactive-control requirement against advertised library capabilities; recommends a specific component or files a structured capability-gap issue.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. Pre-requisites (one-time) must complete BEFORE M1.
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [docs/design/slo-sec-libs-overview.md](design/slo-sec-libs-overview.md), [docs/idea/slo-sec-libs.md](idea/slo-sec-libs.md), [docs/research/slo-sec-libs/synthesis.md](research/slo-sec-libs/synthesis.md), [docs/research/slo-security-embedding/synthesis.md](research/slo-security-embedding/synthesis.md), [Issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)

---

## Runbook Metadata

- **Runbook ID**: `slo-sec-libs`
- **Prefix for test files and lessons files**: `sec-libs`
- **Primary stack**: Markdown + Rust (`crates/sldo-install`, `crates/sldo-common::toolflags`) + Python 3.10+ subprocess (jsonschema validator) + `gh` CLI + `git`
- **Primary package/app names**: `skills/slo-sec-libs`, `crates/sldo-common`, `crates/sldo-install`
- **Default test commands**:
  - Workspace tests: `cargo test --workspace`
  - Specific install tests: `cargo test -p sldo-install`
  - Build: `cargo build --workspace`
  - Python subprocess test (M1): manual run with fixture declarations file
- **Allowed new dependencies by default**: `none` (Python jsonschema is system-installed; no new Rust crates)
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable**:
  - Phase 1 security-aware skills (`/slo-ideate`, `/slo-architect`, `/slo-plan`, `/slo-critique`, `/slo-verify`) unchanged.
  - Existing `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify` argv-list discipline preserved.
  - `crates/sldo-common::toolflags` deny-list pattern unchanged.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | CycloneDX 1.6 declarations reader (Python jsonschema subprocess) | `not_started` | | | | |
| 2 | Capability matcher (proactive-controls → advertised capabilities) | `not_started` | | | | |
| 3 | SLO-intake filer (default channel; `kerberosmansour/slo-security-intake`) | `not_started` | | | | |
| 4 | Third-party filing gate + per-session 40-issues/hr cap | `not_started` | | | | |
| 5 | Dogfood: re-critique an SLO milestone using `/slo-sec-libs` | `not_started` | | | | |

### Pre-requisites (one-time, BEFORE M1)

- [ ] Create `kerberosmansour/slo-security-intake` repo (issue-tracker-only).
- [ ] Populate `ISSUE_TEMPLATE/capability-gap-record.md` per the M3 schema.
- [ ] Add CycloneDX 1.6 `declarations` JSON to `kerberosmansour/hulumi`. Each crate / component advertises controls (use `cdx:sunlit:crypto:*` namespace for parametric crypto claims).
- [ ] Add CycloneDX 1.6 `declarations` JSON to `SunLitSecureLibraries`. Same shape.
- [ ] Confirm `gh` CLI scopes (`repo` or `public_repo` for same-owner; `repo` for cross-repo fork+PR fallback) on contributor machines.

---

## End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                  /slo-sec-libs skill                                     │
│                                                                          │
│  Inputs:                                                                 │
│  - target repo's ARCHITECTURE.md + stack-decision.md                    │
│  - target repo's RUNBOOK-<slug>.md proactive-controls rows              │
│  - pinned CycloneDX schema SHA                                          │
│  - pinned Hulumi + SunLitSecureLibraries declaration SHA per source     │
│                                                                          │
│  ┌───────────────────────────────────────────────────────────────┐     │
│  │ Pre-flight (every invocation):                                 │     │
│  │ 1. Python 3.10+ + jsonschema available (`python3 -c "import   │     │
│  │    jsonschema"`)                                               │     │
│  │ 2. target repo has ARCHITECTURE.md + stack-decision.md         │     │
│  │ 3. runbook declares security_libs_required: true               │     │
│  │ 4. pinned-SHA values present (regex 40-char hex)              │     │
│  │ 5. cwd is a git repo                                           │     │
│  └─────────────────────────┬─────────────────────────────────────┘     │
│                            │                                              │
│                            ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ M1: declarations reader (Python jsonschema subprocess)          │   │
│  │   - fetch CycloneDX 1.6 declarations from cache or git clone    │   │
│  │     (~/.cache/sldo/declarations/<sha>/)                         │   │
│  │   - validate with jsonschema (strict; reject billion-laughs)    │   │
│  │   - extract structured capability catalog (stdout JSON)         │   │
│  │   - argv-list subprocess invocation                             │   │
│  │   - file size cap 10 MiB before parse                           │   │
│  └─────────────────────────┬───────────────────────────────────────┘   │
│                            │                                              │
│                            ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ M2: capability matcher                                          │   │
│  │   - read target runbook proactive-controls rows                 │   │
│  │   - match each row against capability catalog                   │   │
│  │   - tiebreaker: more parametric claims = more specific          │   │
│  │   - emit JSON: { matched: [...], unmatched: [...] }             │   │
│  └─────────────────────────┬───────────────────────────────────────┘   │
│                            │                                              │
│                            ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ M3: SLO-intake filer (default)                                  │   │
│  │   - for each unmatched: produce capability-gap record           │   │
│  │     (regex-validated schema; NO free-text from target prose)    │   │
│  │   - user confirms each filing                                   │   │
│  │   - gh issue create (argv-list, NO --repo)                      │   │
│  │     → kerberosmansour/slo-security-intake                       │   │
│  └─────────────────────────┬───────────────────────────────────────┘   │
│                            │                                              │
│                            ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ M4: third-party filing gate (--file-upstream flag)              │   │
│  │   - default: file to slo-security-intake                        │   │
│  │   - if --file-upstream: file to library-owner repo              │   │
│  │   - per-session 40-issues/hr cap (defensive)                    │   │
│  │   - exceeded → spillover to LESSONS-BACKLOG.md                  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  M5: Dogfood — re-critique one SLO milestone using /slo-sec-libs        │
│                                                                          │
│  Out-of-scope this runbook:                                             │
│  - Rust CycloneDX emitter (cyclonedx-bom 0.8.1 is 1.5 only — separate)  │
│  - Phase 2 /slo-threat-model (parked; reader-side only here)            │
│  - Phase 3 /slo-security-test (separate program)                        │
│                                                                          │
│  Legend:  ─── existing    NEW (this runbook)    ▶ data flow             │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone | Key Interfaces |
|---|---|---|---|
| `skills/slo-sec-libs/SKILL.md` | Skill orchestrator; mode dispatch; pre-flight cascade | M1-M5 | Cited from runbook proactive-controls rows |
| `skills/slo-sec-libs/scripts/read-declarations.py` | Python jsonschema validator + capability extractor | M1 | Subprocess input: CycloneDX JSON; output: stdout JSON catalog |
| `skills/slo-sec-libs/references/methodology-m1-reader.md` | Declarations reader spec | M1 | Cited from SKILL.md |
| `skills/slo-sec-libs/references/methodology-m2-matcher.md` | Capability matching algorithm + tiebreaker | M2 | Cited from SKILL.md |
| `skills/slo-sec-libs/references/capability-gap-schema.md` | Regex-validated schema for gap records | M3 | Cited from SKILL.md and consumed by intake-template |
| `skills/slo-sec-libs/references/upstream-filing-discipline.md` | argv-list + no-`--repo` + rate-limit | M3-M4 | Cited from SKILL.md |
| `crates/sldo-common::toolflags::sec_libs_deny_flags()` | Skill-flag denial: `WebFetch`, `WebSearch` denied | M1 | SLO-CLI invocation layer |
| `kerberosmansour/slo-security-intake` (repo) | Default capability-gap filing destination | Pre-req | `gh issue create` target |
| Hulumi + SunLitSecureLibraries CycloneDX 1.6 declarations | Capability advertisement | Pre-req | JSON in each repo's release artifacts |
| `~/.cache/sldo/declarations/<sha>/` | Pinned-SHA cache | M1 | Local FS |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Declarations fetch | Hulumi / SunLitSecureLibraries repo | Local cache | `git clone` (argv-list) → cache | M1 |
| Declarations parse | Python subprocess | stdout JSON | `python3 scripts/read-declarations.py` (argv-list) | M1 |
| Capability match | runbook proactive-controls | catalog | LLM judgment + matcher rules | M2 |
| Capability-gap filing | unmatched record | GitHub | `gh issue create` (argv-list, no `--repo`) | M3-M4 |
| Rate-limit enforcement | session counter | spillover file | local file write | M4 |

---

## High-Level Design for Formal Verification (TLA+ Section)

`tla_required: false`

No concurrent actors, no distributed state. Single-session, single-actor, sequential (read declarations → match → file). Rate-limit is per-session client-side, not a distributed state machine. Per `/slo-tla`'s suitability gate, this is the wrong tool here.

---

## Global Execution Rules

See [docs/templates/runbook-template_v_3_template.md §"Global Execution Rules"](templates/runbook-template_v_3_template.md). Project-specific overrides:

- **Argv-list discipline non-negotiable**. Every `git`, `gh`, `python3` invocation uses argv-list form. Never shell-string interpolation. Inherited from `/slo-sast` M5 + `/slo-rulegen`.
- **NO `--repo` flag on `gh pr create` / `gh issue create`** (SEC-8 from `/slo-sast` M5; confused-deputy defense).
- **NO autofix / merge flags** anywhere. Reader-side + filer only.
- Research-validation discipline (R2's `references/templates/citation-discipline.md`) applies to every claim about CycloneDX schema, GitHub API, Octokit rate-limit point cost, jsonschema billion-laughs defenses.

## Global Entry / Exit Rules

See template. Specifics:

- Baseline: `cargo test --workspace`. Confirm green before each milestone.
- Pre-requisites listed at top of Milestone Tracker MUST be confirmed complete before M1.

---

## Background Context

### Current State

[PR #3 (slo-security-embedding M1-M4)](https://github.com/kerberosmansour/SunLitOrchestrate/pull/3) merged. Phase 1 of the security-embedding program complete: `/slo-ideate`, `/slo-architect`, `/slo-plan`, `/slo-critique`, `/slo-verify` are security-aware. Every milestone's Contract Block now declares Data classification + Proactive controls + Abuse acceptance scenarios.

What works:

- Phase 1 makes runbooks security-aware at the planning + critique + verify level.
- Hulumi (Pulumi components for AWS) and SunLitSecureLibraries (Rust security crates) advertise capabilities via CycloneDX (existing repos).
- `/slo-sast` ships the SAST orchestration (separate program).

What does not work:

- When a runbook milestone declares "this surface needs an Argon2id password hasher", the agent has no source-of-truth for which library covers that requirement at the runbook's pinned version. Defaults to whatever the model recalls.
- Hulumi + SunLitSecureLibraries' CycloneDX advertising is dead weight — no skill reads the declarations.
- When the recommender finds NO library that covers a requirement, the gap dies in lessons file commentary. No upstream filing.

### Problem

1. **No declarations reader**: Phase 1 produces security-aware runbooks; Phase 4 needs to consume the per-library capability advertising that Hulumi + SunLitSecureLibraries already publish.
2. **No capability matcher**: declarations alone don't pick the library; the matcher reads runbook proactive-controls rows and matches.
3. **No capability-gap filing**: gaps die in markdown.
4. **Rust ecosystem lacks 1.6+ declarations support** ([Phase 1 research Q2](research/slo-security-embedding/synthesis.md)): `cyclonedx-bom 0.8.1` is spec-1.5 only. Python jsonschema subprocess is the path; matches Phase 2 SecOpsTM precedent.
5. **GitHub secondary rate-limit point costs are undocumented** ([Phase 1 research Q5](research/slo-security-embedding/synthesis.md)): defensive cap (40 issues/hr per session) is the answer.

### Target Architecture

See "End-to-End Architecture Diagram" above.

### Key Design Principles

0. **Over-engineering for simplicity**: per [`docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`](PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md), `/slo-sec-libs` exemplifies the paradigm at the security-library scale. A human-driven library-feedback loop ("file capability gaps upstream") routinely fails because filing 10 issues per quarter is 5 hours of cumulative effort, and developers shortcut to "good enough libraries". The LLM-driven `/slo-sec-libs` files capability gaps as structured records with regex-validated schema, per-session 40-issues/hr cap, argv-list discipline, no `--repo` flag, multi-layer Unicode-trick + billion-laughs + symlink-traversal defenses — all costing the user nothing visible. The user sees: "your runbook needed Argon2id; here's the SunLitSecureLibraries crate that advertises it with parametrics; we filed gaps upstream for the 2 unmatched controls."

1. **Reader-side only this phase**: no Rust CycloneDX emitter work; declarations come from Hulumi + SunLitSecureLibraries; SLO reads them.
2. **Python jsonschema subprocess**: matches Phase 2 precedent; Rust ecosystem will catch up later.
3. **SLO-owned intake repo as default**: `kerberosmansour/slo-security-intake`; same-owner authentication; user controls issue-template shape.
4. **Third-party filing is gated**: `--file-upstream` flag + per-session 40-issues/hr cap.
5. **Argv-list discipline + no `--repo` flag**: inherited from `/slo-sast` M5; defends against confused-deputy via tampered `.git/config`.
6. **Capability-gap records are regex-validated**: only structured fields flow into issue body; no free-text from target repo prose.
7. **Crypto-primitive parametric claims in `cdx:sunlit:crypto:*` namespace**: vendored until upstream Property Taxonomy contribution lands (deferred to follow-up).
8. **Pre-requisites are out-of-band**: 3 one-time setups before M1; runbook does NOT bootstrap the intake repo or the declarations files.
9. **Strict jsonschema validate**: reject malformed; reject > 10 MiB before parse (billion-laughs / SEC-2 defense).
10. **Source hierarchy applies**: every claim about CycloneDX schema / GitHub API / Octokit costs source-verified per R2's `citation-discipline.md`.

### What to Keep

- Phase 1 security-aware skills unchanged.
- Existing `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify` argv-list discipline.
- `crates/sldo-common::toolflags` deny-list pattern.
- Existing `/slo-architect` `~~~text` user-string fence rule.
- Existing `crates/sldo-install` install-symlink pattern.

### What to Change

- `skills/slo-sec-libs/` (NEW skill: SKILL.md + scripts/ + references/) — M1-M5.
- `crates/sldo-common::toolflags::sec_libs_deny_flags()` (NEW function) — M1.
- `crates/sldo-install/tests/e2e_sec_libs_m<N>.rs` (NEW per milestone).

### Global Red Lines

Standard set; in addition:

- **Argv-list discipline non-negotiable**.
- **NO `--repo` flag on `gh` commands**.
- **NO `--autofix`, `--auto`, `--squash`, `--rebase`, `--admin`, `--merge`, `gh pr merge`, `gh auth login` from this skill**.
- **NO Vendor SaaS API fallbacks** (Semgrep AppSec, Snyk, GitHub Advanced Security — explicitly rejected per Phase 1 stack-decision).
- **NO state persistence across invocations** (rate-limit is per-session; cross-invocation rate is external).
- **NO `WebFetch` / `WebSearch`** — denied at the SLO-CLI invocation layer.
- Research-validation discipline applies; unverifiable claims removed.

---

## BDD and Runtime Validation Rules

See template. Project specifics:

### Required Test Coverage Categories

For each milestone:

- happy path
- empty state (no proactive-controls rows; no declarations file)
- dependency failure (Python missing, gh missing, declarations file > 10 MiB, malformed JSON)
- backward compatibility (Phase 1 skills unchanged)
- abuse case (per-milestone, see `tm-slo-sec-libs-abuse-1..5` from design overview)

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Skill structural-contract tests | `tests/e2e_sec_libs_m<N>.rs` | `crates/sldo-install/tests/` |
| Python script tests | inline doctests + integration test in `tests/python/` (M1) | same |

---

## Dependency, Migration, and Refactor Policy

See template. **No new Rust dependencies**. Python 3.10+ + jsonschema is system-installed; pre-flight check.

Refactor budget per-milestone in milestone sections.

---

## Evidence Log Template + Self-Review Gate + Lessons + Completion Templates

See template (`docs/lessons/sec-libs-m<N>.md`, `docs/completion/sec-libs-m<N>.md`).

---

## Milestone Plan

### Milestone 1 — CycloneDX 1.6 declarations reader

**Goal**: `skills/slo-sec-libs/SKILL.md` (skeleton) + `scripts/read-declarations.py` (Python jsonschema validator) reads a CycloneDX 1.6 declarations file from `~/.cache/sldo/declarations/<sha>/`, validates with strict jsonschema, extracts a structured capability catalog as stdout JSON.

**Context**: Phase 1 research Q1-Q2 confirmed CycloneDX 1.6 `declarations` as the canonical input; Rust ecosystem has no 1.6+ support → Python subprocess. M1 lands the reader scaffold + the Python script + the toolflag denial.

**Important design rule**: Strict jsonschema validate. Reject any input > 10 MiB before parse (billion-laughs / SEC-2 defense). Argv-list invocation throughout.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Pinned CycloneDX 1.6 schema URL + SHA; pinned Hulumi + SunLitSecureLibraries declaration source SHAs; Python 3.10+ + jsonschema availability |
| Outputs | `skills/slo-sec-libs/SKILL.md` (skeleton); `scripts/read-declarations.py`; `references/methodology-m1-reader.md`; `crates/sldo-common::toolflags::sec_libs_deny_flags()`; structured-contract test |
| Interfaces touched | NEW skill subtree; toolflag deny-list extension |
| Files allowed to change | `skills/slo-sec-libs/SKILL.md` (NEW), `skills/slo-sec-libs/scripts/read-declarations.py` (NEW), `skills/slo-sec-libs/references/methodology-m1-reader.md` (NEW), `crates/sldo-common/src/toolflags.rs` (extend), `crates/sldo-install/tests/e2e_sec_libs_m1.rs` (NEW) |
| Files to read before changing anything | `docs/research/slo-security-embedding/dossier.md` Q1-Q2; `crates/sldo-common/src/toolflags.rs`; existing Phase 1 skills' SKILL.md prose for shape |
| New files allowed | The 4 new files above |
| New dependencies allowed | `none` (Python is system-installed; pre-flight check enforces availability) |
| Migration allowed | `no` |
| Compatibility commitments | Phase 1 security-aware skills unchanged; existing toolflag deny-list patterns preserved |
| Forbidden shortcuts | shell-string subprocess; missing argv-list; missing 10 MiB cap; missing strict jsonschema; non-stdlib Python imports (only `jsonschema` allowed; everything else rejected); **per critique S-4: forbidden vendor-SaaS API fallbacks** — Semgrep AppSec, Snyk, GitHub Advanced Security, Veracode, Checkmarx all explicitly banned (Phase 1 stack-decision); **per paradigm — comprehensive defense layers**: missing entity-expansion guard; missing anchor-recursion guard; reading the cache without `git rev-parse HEAD` integrity check; SHA hex non-lowercase or non-40-char (regex `^[0-9a-f]{40}$` strict); using `--depth=1` clone without then verifying SHA matches pin (depth-1 can mask SHA tampering if not verified post-clone); cache layout outside `~/.cache/sldo/declarations/<sha>/` (e.g., `/tmp` is rejected — survives across reboots wrong, attacker-influenceable); silently retrying on `git clone` failure (must surface error + diagnose, not retry blind); reading declarations from a directory that contains a symlink anywhere in the path (O_NOFOLLOW-style) |
| **Data classification** | `Public` (CycloneDX schemas + capability declarations are public artifacts) |
| **Proactive controls in play** | C1 (security requirements); C5 (validate inputs — strict jsonschema); C9 (audit trail via cache-SHA) |
| **Abuse acceptance scenarios** | `tm-slo-sec-libs-abuse-5: malicious CycloneDX file triggers Python jsonschema infinite recursion` — class eliminated by strict validate (no entity expansion / anchor recursion) + 10 MiB cap; `tm-slo-sec-libs-abuse-1: tampered declarations file inflates capability claims` — class eliminated by SHA pinning + `git rev-parse HEAD` cache integrity check; **per paradigm — additional abuse cases enumerated**: `tm-slo-sec-libs-abuse-9: schema-version mismatch attack` (declarations file claims `specVersion: "1.6"` but uses 1.5-only fields) — eliminated by strict validate against pinned 1.6 schema rejects unknown / missing fields; `tm-slo-sec-libs-abuse-10: declarations file with unicode-trick component name` (e.g., a component named `secure-jsonʿ` with homoglyph) — eliminated by NFKC normalization on component names + reject if normalization changes name; `tm-slo-sec-libs-abuse-11: cache poisoning via in-flight tag-rewrite` — eliminated by post-clone `git rev-parse HEAD` verification; `tm-slo-sec-libs-abuse-12: declarations claim a control with no parametric details` (e.g., "supports Argon2id" with no iterations / memory / parallelism) — surfaced as low-confidence match, never as preferred recommendation; `tm-slo-sec-libs-abuse-13: declarations file references an external schema URL not on the pinned hierarchy` — strict validate rejects (offline mode required) |

#### Out of Scope / Must Not Do

- M2's matcher.
- M3's filer.
- Rust CycloneDX emitter work.
- SecOpsTM integration.
- Vendor SaaS API fallbacks.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Visit https://cyclonedx.org/docs/1.6/json/ at runbook-author time. Capture schema URL + SHA.
3. Confirm pre-requisites complete (intake repo + Hulumi/SunLitSecureLibraries declarations exist).
4. Read `crates/sldo-common/src/toolflags.rs` for the existing deny-list pattern.
5. Read R2 M1's `references/templates/citation-discipline.md` for source hierarchy.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sec-libs/SKILL.md` | NEW: skeleton with pre-flight cascade (Python + jsonschema check, target repo check, runbook check, pinned SHA check, cwd-is-git check), mode dispatch (M1: declarations-reader-only), anti-patterns (shell-string, autofix, vendor SaaS, etc.) |
| `skills/slo-sec-libs/scripts/read-declarations.py` | NEW: argv `<declarations.json>`; reject > 10 MiB; strict jsonschema validate against pinned 1.6 schema; extract structured capability catalog; emit JSON to stdout; stdlib-only |
| `skills/slo-sec-libs/references/methodology-m1-reader.md` | NEW: declarations reader spec, cache layout (`~/.cache/sldo/declarations/<sha>/`), SHA pinning + `git rev-parse HEAD` cache integrity check, fault-handling. **Per critique C-4**: cache eviction policy specified — size cap **1 GiB**, age cap **90 days**, eviction strategy **LRU** (least-recently-used). Document in this methodology file; pre-flight cache check enforces. |
| `crates/sldo-common/src/toolflags.rs` | Extend with `sec_libs_deny_flags()` returning `WebFetch`, `WebSearch` denials |
| `crates/sldo-install/tests/e2e_sec_libs_m1.rs` | NEW: structural-contract test asserting (a) skill files exist + frontmatter, (b) `sec_libs_deny_flags()` returns expected set, (c) Python script is stdlib-only (no `import requests` etc.), (d) SKILL.md cites argv-list discipline + 10 MiB cap |

#### Step-by-Step

1. Test stub first.
2. Author SKILL.md skeleton.
3. Author Python script with strict validate + 10 MiB cap.
4. Author `methodology-m1-reader.md`.
5. Extend toolflags.
6. Run structural-contract test.
7. Manual smoke: feed a fixture declarations file; observe stdout catalog.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: declarations reader works under discipline**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Valid CycloneDX 1.6 declarations | happy path | fixture file | Python script runs | stdout emits structured capability catalog |
| Malformed JSON | dependency failure | invalid JSON file | script runs | refuses with clear stderr; exit non-zero |
| File > 10 MiB | abuse case (`tm-slo-sec-libs-abuse-5` related) | 11 MiB file | script runs | refuses before parse |
| Strict validate rejects entity expansion | abuse case (`tm-slo-sec-libs-abuse-5`) | file with anchor recursion | script runs | strict mode rejects |
| Tampered declarations (SHA mismatch) | abuse case (`tm-slo-sec-libs-abuse-1`) | cache content's `git rev-parse HEAD` differs from pinned SHA | reader pre-flight | refuses to use cache; wipes cache |
| Python missing | dependency failure | `which python3` returns nothing | skill pre-flight | clear install hint; exit non-zero |
| jsonschema library missing | dependency failure | `python3 -c "import jsonschema"` fails | skill pre-flight | install hint (`pip install jsonschema`); exit non-zero |
| Non-stdlib import in script | abuse case (script provenance) | someone adds `import requests` to the script | structural-contract test | test FAILS |
| `WebFetch` attempt within script | abuse case (deny-list bypass) | someone adds a WebFetch wrapper | toolflag deny-list | denied at SLO-CLI invocation layer |
| Backward compat: Phase 1 skills unchanged | backward compatibility | full suite | runs | passes |

#### Regression Tests

- `cargo test --workspace`.
- Phase 1 skills' install symlink tests.
- All existing biz-pack and SAST tests.

#### Compatibility Checklist

- [ ] Phase 1 skills unchanged.
- [ ] `crates/sldo-common::toolflags` extension is additive.
- [ ] No new Rust dependencies.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_sec_libs_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_sec_libs_skill_exists` | Skill subtree present | `SKILL.md`, `scripts/read-declarations.py`, `references/methodology-m1-reader.md` exist |
| `sec_libs_deny_flags_returns_webfetch_websearch` | Deny-list extended | function call returns expected set |
| `python_script_stdlib_only` | Script provenance | grep imports against stdlib allow-list |
| `argv_list_discipline_documented` | Discipline cited | grep SKILL.md for argv-list rule |
| `ten_mib_cap_documented` | Size cap cited | grep methodology + script |
| `strict_jsonschema_documented` | Strict mode cited | grep methodology |
| `phase_1_skills_unchanged` | Backward compat | git diff against Phase 1 skill SKILL.md files |

#### Smoke Tests

- [ ] Run Python script against a valid Hulumi declarations fixture; observe catalog.
- [ ] Run against a malformed fixture; observe refusal.
- [ ] Run against an 11 MiB file; observe refusal before parse.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Python script stdlib-only + strict + size-capped.
- Toolflag deny-list extended.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: add `/slo-sec-libs` to skill inventory.
- README.md: optional bullet.

#### Notes

- The Python script is invoked by Claude Code via `Bash` tool; the SKILL.md prose specifies the exact argv form.

---

### Milestone 2 — Capability matcher

**Goal**: Read target runbook's proactive-controls rows; match each against the capability catalog from M1; emit JSON with `matched: [...]` and `unmatched: [...]`. Tiebreaker rule: more parametric claims = more specific advertising = preferred.

**Context**: M1 produced the catalog. M2 produces the recommendations + gap list. Pure-read; no GitHub side effects.

**Important design rule**: Tiebreaker rule documented; ambiguity-resolution explicit. When two libraries claim the same capability, prefer the one with more parametric claims (more specific advertising). When claims are equally specific, surface BOTH to the user with a "tie" disposition.

**Refactor budget**: `Targeted refactor permitted for matcher logic`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1's catalog stdout; target runbook's proactive-controls rows; M1's pre-flight cascade |
| Outputs | JSON `{ matched: [...], unmatched: [...] }`; methodology-m2 file |
| Interfaces touched | SKILL.md mode dispatch extended; methodology files |
| Files allowed to change | `skills/slo-sec-libs/SKILL.md` (extend), `skills/slo-sec-libs/references/methodology-m2-matcher.md` (NEW), `crates/sldo-install/tests/e2e_sec_libs_m2.rs` (NEW) |
| Files to read before changing anything | M1 lessons + outputs |
| New files allowed | methodology + test |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | M1 reader unchanged; toolflag deny-list unchanged |
| Forbidden shortcuts | LLM-only matching without structured comparison; missing tiebreaker; missing tie disposition |
| **Data classification** | `Public` |
| **Proactive controls in play** | C1; C5 (validate inputs against catalog) |
| **Abuse acceptance scenarios** | `tm-slo-sec-libs-abuse-7: matcher fabricates a capability claim from training memory rather than reading catalog` — class eliminated by structural-contract test asserting matcher output's matched-rows reference catalog entries by ID; no free-text recommendations |

#### Out of Scope / Must Not Do

- Filing (M3 job).
- Rate-limiting (M4 job).
- Modifying the M1 catalog format.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1 lessons + the methodology file.
3. Plan tiebreaker rule.

#### Files Allowed To Change

(See contract block.)

#### Step-by-Step

1. Test stub first.
2. Author `methodology-m2-matcher.md` with the matching algorithm + tiebreaker.
3. Update SKILL.md mode dispatch with M2 mode.
4. Verify structural-contract test.
5. Smoke test: feed fixture catalog + fixture runbook; observe matched/unmatched output.
6. Self-review.

#### BDD Acceptance Scenarios

**Feature: capability matcher resolves runbook → catalog**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All proactive-controls match | happy path | runbook rows match catalog entries | matcher runs | JSON `matched:[...]` populated; `unmatched:[]` |
| One row unmatched | happy path | runbook needs Argon2id but catalog only has Bcrypt | matcher runs | one entry in `unmatched:[...]` with reason |
| Tie (two libs claim same capability equally) | happy path | two libs each claim Argon2id with same parametrics | matcher runs | both surfaced in `matched:[...]` with `disposition: tie` |
| Tiebreaker by specificity | happy path | one lib claims "Argon2id", another "Argon2id with iterations≥3, memory≥64MiB" | matcher runs | second wins; disposition `preferred-by-specificity` |
| Empty runbook | empty state | no proactive-controls rows | matcher runs | `matched:[]`, `unmatched:[]`; "no controls to match" stderr |
| Empty catalog | empty state | M1 returned empty catalog | matcher runs | every runbook row → `unmatched`; "no candidate libraries" stderr |
| Fabricated match | abuse case (`tm-sec-libs-abuse-7`) | matcher emits a `matched:` entry with an ID not in catalog | structural-contract test | test FAILS |
| Backward compat | backward compatibility | M1 catalog format unchanged | full suite | passes |

#### Regression Tests

- `cargo test --workspace`.
- M1 structural-contract test.

#### Compatibility Checklist

- [ ] M1 reader unchanged.
- [ ] Toolflag deny-list unchanged.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_sec_libs_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `methodology_m2_exists` | Methodology file present | path + frontmatter |
| `tiebreaker_rule_documented` | Specificity rule cited | grep methodology |
| `tie_disposition_documented` | Tie handling | grep |
| `every_matched_entry_references_catalog_id` | No fabrication | structural rule on output JSON |

#### Smoke Tests

- [ ] Feed fixture runbook + catalog; observe matcher output.
- [ ] Feed runbook with unmatched control; observe `unmatched:` populated.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Tiebreaker rule documented + tested.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md: not required.
- README.md: not required.

#### Notes

- M2 is reader-side only; no GitHub side effects yet. M3 is when the filing surface comes in.

---

### Milestone 3 — SLO-intake filer (default channel)

**Goal**: For each `unmatched` capability gap from M2, produce a regex-validated capability-gap record; user confirms each filing; `gh issue create` (argv-list, no `--repo`) to default destination `kerberosmansour/slo-security-intake`.

**Context**: Default destination is SLO-owned. Pre-requisite repo (`slo-security-intake` with `ISSUE_TEMPLATE/capability-gap-record.md`) must exist before this milestone runs.

**Important design rule**: **Capability-gap record is regex-validated.** Only structured fields flow into issue body. No free-text from target repo prose. argv-list throughout. No `--repo` flag.

**Refactor budget**: `Targeted refactor permitted for filer logic`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M2's `unmatched:` records; user `gh` auth; pre-requisite intake repo + ISSUE_TEMPLATE |
| Outputs | filed issues in `kerberosmansour/slo-security-intake`; capability-gap-schema.md (NEW); upstream-filing-discipline.md (NEW) |
| Interfaces touched | SKILL.md mode dispatch extended; new methodology files; `gh issue create` invocation |
| Files allowed to change | `skills/slo-sec-libs/SKILL.md` (extend), `skills/slo-sec-libs/references/capability-gap-schema.md` (NEW), `skills/slo-sec-libs/references/upstream-filing-discipline.md` (NEW), `crates/sldo-install/tests/e2e_sec_libs_m3.rs` (NEW) |
| Files to read before changing anything | M2 lessons; `/slo-sast/SKILL.md` M5 (argv-list + no `--repo`); R1 M3's `issue-filing-discipline.md` (rate-limit cap shape) |
| New files allowed | 2 references + test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | M1 + M2 unchanged; toolflag deny-list unchanged; existing `/slo-sast` argv-list discipline preserved |
| Forbidden shortcuts | shell-string interpolation; `--repo` flag; auto-filing without confirmation; merge flags; free-text prose in issue body |
| **Data classification** | `Public` (capability-gap records describe library deltas, not user content) |
| **Proactive controls in play** | C1; C5 (validate inputs — regex-validated record schema); C7 (access controls — user `gh` auth + per-issue confirmation); C9 (audit trail — every filing carries `gh` author + timestamp) |
| **Abuse acceptance scenarios** | `tm-slo-sec-libs-abuse-2: capability-gap record body splices attacker-supplied prose from target repo content` — class eliminated by regex-validated schema; `tm-slo-sec-libs-abuse-4: confused-deputy via tampered .git/config redirecting gh` — class eliminated by NO `--repo` flag (inherits SEC-8 from `/slo-sast` M5) |

#### Out of Scope / Must Not Do

- Third-party filing (M4 job).
- Rate-limit cap (M4 job).
- Auto-merging anything.
- Vendor SaaS API fallbacks.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1, M2 lessons.
3. Confirm `slo-security-intake` repo exists and has `ISSUE_TEMPLATE/capability-gap-record.md`.
4. Read `/slo-sast/SKILL.md` M5 (argv-list + no `--repo` discipline).
5. Read R1 M3's `references/issue-filing-discipline.md` if R1 has shipped (otherwise inline the rate-limit pattern).

#### Files Allowed To Change

(See contract block.)

#### Step-by-Step

1. Test stub first.
2. Author `capability-gap-schema.md` with regex-validated fields.
3. Author `upstream-filing-discipline.md` with argv-list + no `--repo` rules.
4. Update SKILL.md with M3 mode dispatch.
5. Verify structural-contract test.
6. Manual smoke: feed M2 unmatched record; confirm filing; observe issue created in intake repo.
7. Self-review.

#### BDD Acceptance Scenarios

**Feature: capability-gap filing under discipline**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Single gap filed | happy path | M2 produces 1 unmatched | M3 runs with confirmation | issue created in `slo-security-intake` |
| Multiple gaps filed | happy path | 3 unmatched | M3 runs | 3 issues filed (one per gap, with confirmation each) |
| User declines a filing | invalid input / consent | user types `n` | M3 prompts | that gap not filed; others proceed |
| Free-text prose in record | abuse case (`tm-sec-libs-abuse-2`) | adversary plants `<script>` in target repo's RUNBOOK | M3 constructs issue body | regex-validated schema rejects; no script in body |
| Tampered `.git/config` | abuse case (`tm-sec-libs-abuse-4`) | adversary modifies `.git/config` remote.origin.url | M3 files | NO `--repo` flag = uses local origin; same trust class as user's local config |
| Shell injection attempt | abuse case (argv-list) | record content contains `;` `|` `&` | argv-list invocation | passed as single argument; no shell interpretation |
| `gh` not authenticated | dependency failure | `gh auth status` unauth | M3 pre-flight | clear `gh auth login` install hint; exit non-zero (do NOT auth from skill) |
| Issue template missing in intake repo | dependency failure | `slo-security-intake` lacks ISSUE_TEMPLATE | M3 attempts file | warns; falls back to plain issue body using regex-validated schema |
| Backward compat | backward compatibility | M1 + M2 unchanged | full suite | passes |

#### Regression Tests

- `cargo test --workspace`.
- M1, M2 structural-contract tests.
- `/slo-sast` argv-list discipline tests.

#### Compatibility Checklist

- [ ] M1 + M2 unchanged.
- [ ] `/slo-sast` discipline preserved.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_sec_libs_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `capability_gap_schema_regex_validated` | Schema is regex-validated | parse + verify all fields have regex constraints |
| `upstream_filing_discipline_argv_list` | argv-list cited | grep |
| `no_repo_flag_documented` | Confused-deputy defense | grep |
| `no_merge_flags` | No auto-merge | grep refuses any merge flag |
| `no_gh_auth_login_from_skill` | Auth discipline | grep |

#### Smoke Tests

- [ ] Manually run M3 against a fixture M2 output; observe confirmation prompt.
- [ ] Decline at confirmation; verify no issue created.
- [ ] `gh issue list --repo kerberosmansour/slo-security-intake --label capability-gap` after manual filing — observe filed issue.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md: not required (M5 dogfood will exercise it).
- README.md: optional bullet about capability-gap filing.

#### Notes

- The default destination (`slo-security-intake`) means even contributors without third-party access can file gaps. M4 adds the `--file-upstream` gate.

---

### Milestone 4 — Third-party filing gate + rate-limit cap

**Goal**: `--file-upstream` flag enables filing to library-owner repos (Hulumi, SunLitSecureLibraries, etc.); per-session 40-issues/hr cap as defensive cap against GitHub secondary rate limits; spillover to `LESSONS-BACKLOG.md` on cap-exceeded.

**Context**: GitHub secondary rate-limit point costs are undocumented (Phase 1 research Q5). 40-issues/hr is a defensive cap derived from public Octokit benchmarks.

**Important design rule**: **Per-session cap, not global**. State doesn't persist across invocations. Cross-invocation rate is the user's responsibility.

**Refactor budget**: `Targeted refactor permitted for upstream-filing-discipline.md extension`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M3's filer; `--file-upstream` flag; per-session counter |
| Outputs | Filings to library-owner repos when `--file-upstream` is passed; spillover to `LESSONS-BACKLOG.md` on cap |
| Interfaces touched | SKILL.md mode dispatch; `upstream-filing-discipline.md` extended |
| Files allowed to change | `skills/slo-sec-libs/SKILL.md` (extend), `skills/slo-sec-libs/references/upstream-filing-discipline.md` (extend), `crates/sldo-install/tests/e2e_sec_libs_m4.rs` (NEW) |
| Files to read before changing anything | M3 lessons; R1 M3's rate-limit pattern (if shipped) |
| New files allowed | test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | M1, M2, M3 unchanged; default destination remains `slo-security-intake` |
| Forbidden shortcuts | persisting cap counter across sessions; auto-`--file-upstream`; missing user-confirmation for upstream filing |
| **Data classification** | `Public` |
| **Proactive controls in play** | C1; C7 (access controls — `--file-upstream` is opt-in flag); C9 (audit — cap firing recorded) |
| **Abuse acceptance scenarios** | `tm-slo-sec-libs-abuse-3: cross-org filing storm via --file-upstream loop` — mitigated by per-session 40-issues/hr cap + user-confirmation gate per filing |

#### Out of Scope / Must Not Do

- Persisting cap state across sessions.
- Default-on `--file-upstream`.
- Modifying M1/M2/M3 deliverables beyond the named extension.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M3 lessons.
3. Verify R1 M3's rate-limit cap pattern (if R1 shipped).

#### Files Allowed To Change

(See contract block.)

#### Step-by-Step

1. Test stub.
2. Extend SKILL.md with `--file-upstream` flag dispatch.
3. Extend `upstream-filing-discipline.md` with cap + spillover discipline.
4. Verify structural-contract test.
5. Smoke test: simulate 41 fillings per session; observe cap fire on 41st.
6. Self-review.

#### BDD Acceptance Scenarios

**Feature: third-party filing + rate-limit**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Default destination unchanged without `--file-upstream` | happy path | filing without flag | M4 runs | files to `slo-security-intake` |
| `--file-upstream` enables third-party filing | happy path | flag passed; capability gap maps to Hulumi | M4 runs | files to `kerberosmansour/hulumi` (with confirmation) |
| Cap fires at 40 issues/session | abuse case (`tm-sec-libs-abuse-3`) | session has filed 40 | 41st filing attempted | cap fires; spillover to `LESSONS-BACKLOG.md` |
| Spillover record format | happy path | cap fired | spillover written | row appended to `LESSONS-BACKLOG.md` with capability-gap-schema fields |
| Cap counter resets across sessions | backward compatibility | session 1 fired 40, ended | session 2 starts | counter resets; new session can file 40 more |
| Library-owner mapping ambiguity | invalid input | capability gap doesn't map to a known library owner | M4 runs | refuses third-party filing; default to slo-security-intake with note |
| User declines `--file-upstream` confirmation | invalid input | user `n` at prompt | M4 | falls back to slo-security-intake |

#### Regression Tests

- `cargo test --workspace`.
- M1, M2, M3 structural-contract tests.

#### Compatibility Checklist

- [ ] M1, M2, M3 unchanged.
- [ ] Default destination still `slo-security-intake`.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_sec_libs_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `file_upstream_flag_documented` | Flag present | grep SKILL.md |
| `forty_per_hour_cap_documented` | Cap cited | grep |
| `cross_session_state_not_persisted` | Per-session discipline | grep refuses any "saved counter" pattern |
| `lessons_backlog_spillover` | Spillover documented | grep |

#### Smoke Tests

- [ ] Mock-invoke 40 fillings; observe all proceed; 41st triggers cap + spillover.
- [ ] End session, restart, attempt filing — observe counter reset.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md: not required.
- README.md: not required.

#### Notes

- 40-issues/hr is a conservative cap. If real-world telemetry from M5 dogfood + early adoption suggests this is too low or too high, refresh in a future runbook (not this one).

---

### Milestone 5 — Dogfood: re-critique an SLO milestone using `/slo-sec-libs`

**Goal**: Re-critique **multiple already-shipped SLO milestones** (per paradigm — comprehensive coverage, not single-target) using `/slo-sec-libs` against the Hulumi + SunLitSecureLibraries declarations. Produce: per-target dogfood reports listing `matched:` recommendations, `unmatched:` capability gaps, files filed. Targets: at least 3 candidates (recommend: `slo-security-embedding` M3, `slo-sast` M3, R3 M3 if shipped). Multiple targets validate the matcher across diverse proactive-controls surfaces.

**Context**: Dogfood is the integration test. Confirms the full pipeline (read → match → file) works end-to-end against real declarations + a real runbook. Proves the value-add: are we surfacing meaningful library recommendations + gaps?

**Important design rule**: **No new code in M5**. Pure exercise of M1-M4 against a real target. Outcomes documented in lessons.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | An already-shipped SLO milestone's RUNBOOK + ARCHITECTURE.md + stack-decision.md (likely `docs/RUNBOOK-SLO-SECURITY-EMBEDDING.md` or similar); Hulumi + SunLitSecureLibraries declarations |
| Outputs | Dogfood report at `docs/sec-libs-dogfood-<date>.md` capturing matched/unmatched/filed; lessons file; completion summary |
| Interfaces touched | None — exercise of M1-M4 only |
| Files allowed to change | `docs/sec-libs-dogfood-<YYYY-MM-DD>.md` (NEW), `crates/sldo-install/tests/e2e_sec_libs_m5.rs` (NEW) |
| Files to read before changing anything | All M1-M4 outputs; the chosen target milestone's runbook |
| New files allowed | dogfood report + test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All M1-M4 deliverables unchanged |
| Forbidden shortcuts | introducing new code in M5; auto-merging filed issues |
| **Data classification** | `Public` (dogfood report is public-tier) |
| **Proactive controls in play** | C9 (audit trail via dogfood report) |
| **Abuse acceptance scenarios** | none new (dogfood inherits M1-M4 abuse cases) |

#### Out of Scope / Must Not Do

- Adding new code beyond the dogfood report and test file.
- Cross-runbook dogfood (M5 picks ONE target milestone).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1-M4 lessons.
3. **Per critique C-1**: shortlist 2-3 candidate target milestones with the richest proactive-controls surface (recommend: `slo-security-embedding` M3, `slo-sast` M3, or M2 of this runbook's R2 if shipped). Pick the one with the most rows in its Contract Block "Proactive controls in play" entries — dogfood needs a real surface to exercise.
4. Confirm pre-requisites still hold (intake repo + declarations files).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/sec-libs-dogfood-<YYYY-MM-DD>.md` | NEW: dogfood report (matched/unmatched/filed sections) |
| `crates/sldo-install/tests/e2e_sec_libs_m5.rs` | NEW: structural-contract test asserting (a) dogfood report exists, (b) every section populated, (c) filed-issues list includes valid issue URLs |

#### Step-by-Step

1. Test stub.
2. Run M1-M4 pipeline against the target milestone.
3. Capture matched/unmatched/filed in the dogfood report.
4. Verify structural-contract test.
5. Self-review.

#### BDD Acceptance Scenarios

**Feature: dogfood validates the pipeline end-to-end**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Dogfood report has all sections | happy path | M5 closes | inspect report | matched / unmatched / filed sections all populated |
| At least one matched recommendation | happy path | M5 closes | inspect | ≥ 1 matched (proves the matcher works on real declarations) |
| Filed-issues list has valid URLs | happy path | M5 closes | inspect | each URL resolves; each issue is in `slo-security-intake` (default) |
| Empty dogfood (no proactive-controls in target) | empty state | target milestone has no security_libs_required: true | M5 runs | dogfood report says "target milestone declares no security-libs requirement; dogfood inconclusive" |
| Backward compat: M1-M4 unchanged | backward compatibility | M5 closes | git diff M1-M4 deliverables | empty |

#### Regression Tests

- `cargo test --workspace`.
- M1-M4 structural-contract tests.

#### Compatibility Checklist

- [ ] M1-M4 deliverables unchanged.
- [ ] No skill behavior change.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_sec_libs_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `dogfood_report_exists` | Report present | path + frontmatter |
| `dogfood_report_has_matched_section` | Matcher worked on real input | grep |
| `dogfood_report_has_filed_issues` | Filer worked | grep + URL pattern |

#### Smoke Tests

- [ ] Open dogfood report; verify renders.
- [ ] Click filed-issues URLs; verify resolve.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Dogfood report authored.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: confirm `/slo-sec-libs` skill listed in skill inventory.
- README.md update: link to dogfood report.

#### Notes

- The dogfood is M5's value: proves the pipeline produces meaningful recommendations + gaps on real input, not just contrived fixtures.

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| Pre-req | none | none | none | Out-of-band: create `slo-security-intake` repo + ISSUE_TEMPLATE; add CycloneDX declarations to Hulumi + SunLitSecureLibraries |
| 1 | Add `/slo-sec-libs` to skill inventory | optional | `~/.cache/sldo/declarations/` (already gitignored at user level) | `skills/slo-sec-libs/` (new subtree) |
| 2 | none | none | none | `methodology-m2-matcher.md` |
| 3 | none | optional bullet | none | `capability-gap-schema.md`, `upstream-filing-discipline.md` |
| 4 | none | none | `LESSONS-BACKLOG.md` (gitignore in target repos) | `upstream-filing-discipline.md` (extend) |
| 5 | confirm skill inventory | link to dogfood | none | `docs/sec-libs-dogfood-<date>.md` |

---

## Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## Carry-forward from prior retros

(Empty until R1 M3 ships and `/slo-retro` files lessons as issues. R4 may run in parallel with R1; if R1 ships first, R4 gets carry-forward from R1 lessons.)

| Issue | Title | Suggested milestone | Status |
|---|---|---|---|
| (none yet) | | | |

---

## Paradigm-driven enhancements (per `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`)

This runbook applies the over-engineering-for-simplicity paradigm at the security-library scale. The `/slo-sec-libs` skill absorbs more discipline than a human-driven library-feedback loop would sustain because the LLM does not pay the cognitive-load tax. Specific layers added because the LLM is the executor:

### Comprehensive abuse-case enumeration (M1)

Original M1 enumerated 5 abuse cases. Paradigm-driven extension: **13 abuse cases total** (5 original + 8 paradigm additions: schema-version mismatch, unicode-trick component name, cache-poisoning via in-flight tag-rewrite, low-confidence parametric claims, external-schema reference, plus pre-existing forbidden-fallback enumeration). Each adds a distinct failure mode the LLM filer will not absorb silently.

### Comprehensive forbidden-shortcut enumeration (M1 contract)

Original contract listed 5 forbidden shortcuts. Paradigm-driven extension: **13 forbidden shortcuts**, including post-clone-SHA-verification-skipped, `/tmp` cache layout, silent retry on `git clone` failure, symlink-anywhere-in-cache-path. A human implementer would catch 2-3; the LLM pipeline absorbs all 13.

### Multiple dogfood targets (M5)

Critique C-1 surfaced that single-target dogfood could be inconclusive. Paradigm-driven correction: **≥ 3 targets** validates the matcher across diverse proactive-controls surfaces. A human team would balk at 3× the dogfood effort; the LLM pipeline absorbs it and gets stronger validation in return.

### Capability-gap record schema — comprehensive structured fields

Original schema is regex-validated. Paradigm-driven enhancement (during M3 implementation): every record carries `impact_class:` (which OWASP / ASVS chapter the gap touches), `exploitability:` (low/med/high based on the threat model), `alternatives_tried:` (what the recommender attempted before declaring gap), `parametric_requirements:` (the specific Argon2id iterations / memory / parallelism the gap needs), `target_repo_context:` (one-line: which target proactive-controls row triggered this gap). Each field is regex-validated; the comprehensive coverage gives the upstream library maintainer enough context to act without back-and-forth.

### Defense-in-depth across milestones

| Concern | Layer 1 | Layer 2 | Layer 3 | Layer 4 |
|---|---|---|---|---|
| Tampered declarations | SHA pin per source (M1) | `git rev-parse HEAD` post-clone verify (M1) | Strict jsonschema validate against pinned 1.6 schema (M1) | `O_NOFOLLOW`-style cache path verification (M1) |
| Filing-storm | Per-session 40/hr cap (M4) | Adaptive backoff on `gh` rate-limit responses (M4) | Spillover to `LESSONS-BACKLOG.md` (M4) | User-confirmation gate per filing (M3) |
| Confused-deputy via `.git/config` | NO `--repo` flag (M3) | argv-list-only subprocess (M3+M4+M5) | Confirmation gate shows resolved origin (M3) | Cross-org filing gated by `--file-upstream` (M4) |
| Capability fabrication (LLM hallucinates a match) | Match must reference catalog entry by ID (M2) | Tiebreaker prefers parametric specificity (M2) | Tie-disposition surfaces both candidates (M2) | Structural-contract test asserts no fabricated IDs (M2) |
| Schema poisoning | Strict validate (no entity expansion / anchor recursion) (M1) | 10 MiB pre-parse cap (M1) | Schema URL pinned (M1) | NFKC normalization on component names (M1, paradigm extension) |
| Cache poisoning | SHA-pinned cache directory (M1) | Post-clone `git rev-parse HEAD` (M1) | Cache size cap 1 GiB + 90-day age cap + LRU (M1, paradigm extension) | Wipe + refuse on SHA mismatch (M1) |

### Bounded by context-window

The paradigm's discipline-vs-context-window balance: SKILL.md is the orchestrator (≤ ~150 lines projected), per-stage methodology files (M1 reader, M2 matcher, M3 schema, M3-M4 filing discipline) load on-demand. The Python script is small (~50 lines stdlib-only). The capability-gap record schema fits in one reference file. Cache layout under `~/.cache/sldo/declarations/<sha>/` is bounded by the 1 GiB cap.

### Paradigm balance — what we did NOT add

Per the paradigm's anti-patterns ("don't add infinite gates"), the following enhancements were considered and **declined**:

- **CycloneDX XML variant support**: 1.6 spec supports both JSON and XML. Adding XML reader would multiply attack surface (XXE attacks). The paradigm balances comprehensive vs context-window — JSON-only is the right cut for v1; XML support is a separate `/slo-architect` re-pass.
- **Auto-merge of refresh PRs**: never; matches `/slo-sast` M5 discipline.
- **Cross-org auto-filing without confirmation**: always user-gated; matches R1 M3 discipline.
- **Caching parsed-schema results across invocations**: re-parse per call (matches `/slo-sast` discipline) — context-window-cheap, prevents stale-cache bugs.

### Ask items still open from critique

- C-3: where does the canonical `gh issue create` rate-limit discipline live — R1 or R4? (Lean: R2 M1's `references/templates/rate-limiting-discipline.md` consolidates both, per paradigm)
- C-6: confirm conservative-by-default tiebreaker in matcher
- S-3: accept review-after-3-months follow-up for the 40/hr cap
