# Secure Execution Controls — SunLitOrchestra (planning draft)

> **Purpose**: Make `/slo-execute` write secure code and secure infrastructure from day one, not merely discover security mistakes later. The runbook adds a secure-construction gate, stack-aware library selection, capability-gap routing, and a threat-model-driven security-test matrix covering SAST, DAST, E2E, supply-chain, and IaC checks.
>
> **Audience**: AI coding agents first, humans second.
>
> **Status**: executed locally on branch `slo/secure-execution-controls`; awaiting review/ship.
>
> **Prerequisite reading**: [`docs/slo/research/secure-execution-controls/synthesis.md`](../research/secure-execution-controls/synthesis.md), [`skills/slo-execute/SKILL.md`](../../../skills/slo-execute/SKILL.md), [`skills/slo-plan/references/proactive-controls-vocabulary.md`](../../../skills/slo-plan/references/proactive-controls-vocabulary.md), [`skills/slo-sec-libs/SKILL.md`](../../../skills/slo-sec-libs/SKILL.md), [`skills/slo-sast/SKILL.md`](../../../skills/slo-sast/SKILL.md), [`skills/slo-dast-tuner/SKILL.md`](../../../skills/slo-dast-tuner/SKILL.md), [`skills/slo-cloud-threat-model/SKILL.md`](../../../skills/slo-cloud-threat-model/SKILL.md).

---

## Runbook Metadata

- **Runbook ID**: `secure-execution-controls`
- **Prefix for tests / lessons / completion**: `sec-exec`
- **Primary stack**: Markdown skill contracts + Rust structural-contract tests in `crates/sldo-install/tests/`
- **Primary surfaces**: `/slo-execute`, `/slo-plan`, `/slo-verify`, `/slo-sec-libs`, `/slo-sast`, `/slo-dast-tuner`, `/slo-cloud-threat-model`
- **Default test command**: `cargo test -p sldo-install`
- **Full regression command**: `cargo test --workspace`
- **Allowed new dependencies by default**: none
- **Schema/config migration allowed by default**: no
- **Stable public interfaces**: skill names, existing runbook template locations, `/slo-sec-libs` mode names, `/slo-dast-tuner` zaprun-only boundary

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-execute` secure-construction pre-flight | `done` | 2026-05-17 | 2026-05-17 | N/A - compact execution | N/A - compact execution |
| 2 | Secure-construction matrix + `/slo-plan` contract refinement | `done` | 2026-05-17 | 2026-05-17 | N/A - compact execution | N/A - compact execution |
| 3 | Security-test selector: SAST, DAST, E2E, supply-chain | `done` | 2026-05-17 | 2026-05-17 | N/A - compact execution | N/A - compact execution |
| 4 | Pulumi TypeScript / Hulumi secure-IaC lane | `done` | 2026-05-17 | 2026-05-17 | N/A - compact execution | N/A - compact execution |
| 5 | Dogfood + capability-gap route through `/slo-sec-libs` | `done` | 2026-05-17 | 2026-05-17 | N/A - compact execution | [`docs/slo/verify/sec-exec-dogfood.md`](../verify/sec-exec-dogfood.md) |

---

## Target Shape

During `/slo-execute`, before code is written:

1. Read the runbook Contract Block, `SECURITY.md`, and `docs/slo/design/<slug>-threat-model.md`.
2. Identify touched surfaces: boundary input, authn/authz, secrets, persistence, subprocess, SQL, HTTP route, UI/DOM, GitHub Actions, Pulumi/Hulumi cloud resource, or third-party API.
3. Map each surface to a secure-construction default:
   - Rust/axum + `security_libs_required: true`: prefer SunLitSecurityLibraries declarations via `/slo-sec-libs`.
   - Pulumi TypeScript + Hulumi explicit/detected: prefer Hulumi components and policy packs.
   - Generic Pulumi TypeScript: use Pulumi secure patterns, Policy as Code, unit tests, and a recorded decision on whether Hulumi is in scope.
   - Other TypeScript/Java stacks: fall back to OWASP Proactive Controls / ASVS plus current official framework docs; do not invent library capability claims.
4. If a secure library capability exists, write tests and implementation around it.
5. If no capability exists, route to `/slo-sec-libs` gap handling or record a time-boxed residual-risk workaround.
6. Select security tests from the threat model and touched surfaces; do not run noisy scanners just to have a row.

## Non-Negotiables

- Secure-library use is preferred over local hand-rolled security code when a declared capability exists.
- Raw crypto, auth, authz, path/URL/shell/SQL construction, output encoding, or secret-handling needs an explicit justification when a secure abstraction exists.
- No app-specific DAST rule lands in SunLitOrchestra or `zaprun`.
- DAST goes through `/slo-dast-tuner` / `zaprun` when the target has DAST scope. Do not add fresh direct ZAP commands to `/slo-verify`.
- An unauthenticated scan of an authenticated app is a coverage failure.
- Hulumi is the preferred lane only when the target or user makes Hulumi explicit, or when the repo is Sherif-owned Pulumi TypeScript with `security_libs_required: true`.
- TypeScript/Java library recommendations are control-first unless and until a declarations-backed catalog exists.

---

## Milestone 1 — `/slo-execute` Secure-Construction Pre-Flight

**Goal**: Insert a required secure-construction pre-flight in `/slo-execute` after file/context reads and before BDD tests are written.

### Planned Changes

| Path | Planned change |
|---|---|
| `skills/slo-execute/SKILL.md` | Add "Secure-construction pre-flight" after current file-read step. It builds a short implementation-security map from Contract Block rows, threat-model rows, and `SECURITY.md`. |
| `skills/slo-execute/references/secure-construction-preflight.md` | New reference with surface detection, secure-library preference rules, residual-risk branch, and what to avoid. |
| `crates/sldo-install/tests/e2e_sec_exec_m1.rs` | Structural-contract tests for the new pre-flight, gap routing, and refusal to hand-roll common security primitives silently. |

### Acceptance Scenarios

| Scenario | Then |
|---|---|
| Rust boundary input surface | `/slo-execute` must mention SunLitSecurityLibraries / `/slo-sec-libs` matching before code. |
| Secure capability missing | `/slo-execute` routes to capability-gap handling or residual-risk row, not model-memory recommendation. |
| Raw crypto/auth/path/SQL proposed | pre-flight requires explicit justification if a secure abstraction exists. |
| No new security surface | pre-flight emits `N/A — no new security-relevant surface` with reason. |

---

## Milestone 2 — Secure-Construction Matrix + `/slo-plan` Contract Refinement

**Goal**: Give `/slo-plan` a sharper contract so `/slo-execute` has enough information to act.

### Planned Changes

| Path | Planned change |
|---|---|
| `skills/slo-plan/SKILL.md` | Add one sentence: proactive controls must be actionable implementation constraints, not decorative labels. |
| `skills/slo-plan/references/secure-construction-matrix.md` | New matrix mapping touched surface → secure library/default → tests expected. |
| `skills/slo-plan/references/proactive-controls-vocabulary.md` | Clarify Pulumi TypeScript vs Hulumi rule, and add TypeScript/Java fallback language that forbids hallucinated library claims. |
| `crates/sldo-install/tests/e2e_sec_exec_m2.rs` | Tests for Hulumi distinction, TypeScript/Java fallback, and matrix citation from `/slo-plan`. |

### Acceptance Scenarios

| Scenario | Then |
|---|---|
| Pulumi TypeScript target with Hulumi explicit | Contract row points to Hulumi component/policy pack. |
| Pulumi TypeScript target without Hulumi explicit | Contract row uses Pulumi secure IaC controls and asks whether Hulumi is in scope. |
| Java or TypeScript app target | Contract row names OWASP control + framework-doc lookup, not invented library claims. |

---

## Milestone 3 — Security-Test Selector: SAST, DAST, E2E, Supply-Chain

**Goal**: Reconcile `/slo-verify` Pass 4 with `/slo-sast`, `/slo-dast-tuner`, and the new secure-construction matrix.

### Planned Changes

| Path | Planned change |
|---|---|
| `skills/slo-verify/SKILL.md` | Add "Security-test selector" before Pass 4 command dispatch. |
| `skills/slo-verify/references/security-pass-commands.md` | Replace direct generic ZAP/Dastardly guidance with a `zaprun` / `/slo-dast-tuner` handoff when DAST applies; keep N/A for no smoke service. |
| `skills/slo-dast-tuner/SKILL.md` | Cross-link the selector so DAST expectations are not duplicated. |
| `crates/sldo-install/tests/e2e_sec_exec_m3.rs` | Tests for SAST/DAST/E2E selection and no direct new ZAP command prose. |

### Acceptance Scenarios

| Scenario | Then |
|---|---|
| New HTTP route with OpenAPI/smoke service | selector requires SAST + `/slo-dast-tuner`/`zaprun`. |
| Authenticated route | unauthenticated DAST is reported as coverage failure. |
| Pure library code | DAST is `N/A`; SAST/unit/variant checks remain live. |
| SPA/DOM surface | selector points to the PTK/DOM-XSS lane only when image capability is validated. |

---

## Milestone 4 — Pulumi TypeScript / Hulumi Secure-IaC Lane

**Goal**: Make cloud/platform work secure-by-default for AWS, GitHub, and Cloudflare without forcing Hulumi onto unrelated users.

### Planned Changes

| Path | Planned change |
|---|---|
| `skills/slo-execute/references/cloud-iac-secure-construction.md` | New IaC lane: Pulumi TypeScript, Hulumi explicit/detected, secrets/state, policy packs, preview/drift evidence, unit tests. |
| `skills/slo-cloud-threat-model/SKILL.md` | Add handoff note: execute cloud resource milestones through the secure-IaC lane after threat model generation. |
| `skills/slo-plan/references/secure-construction-matrix.md` | Add cloud/platform rows for AWS, GitHub, Cloudflare. |
| `crates/sldo-install/tests/e2e_sec_exec_m4.rs` | Tests for Hulumi explicit/detected rule, Pulumi unit-test citation, policy-as-code citation, GitHub workflow hardening. |

### Acceptance Scenarios

| Scenario | Then |
|---|---|
| S3 bucket in Hulumi target | plan/execute require `SecureBucket` or gap handling. |
| GitHub Actions deployment identity | plan/execute require OIDC, scoped permissions, SHA-pinned actions, no plaintext cloud keys. |
| Cloudflare protected admin host | plan/execute require Hulumi/Cloudflare baseline if explicit, otherwise secure Pulumi pattern plus policy check. |

---

## Milestone 5 — Dogfood + Capability-Gap Route

**Goal**: Prove the full loop on a realistic fixture and document how upstream fixes flow back into downstream execution.

### Planned Changes

| Path | Planned change |
|---|---|
| `tests/fixtures/secure-execution-controls/` | New synthetic Rust + Pulumi TypeScript fixture with a route, a secret, and an IaC resource. |
| `docs/slo/verify/sec-exec-dogfood.md` | Dogfood report: secure-construction map, selected tests, matched library capabilities, gaps, residual risks. |
| `docs/LOOPS-ENGINEERING.md` | Add a short "secure-construction loop" or extend security-tuning/library-feedback loops if that is cleaner. |
| `crates/sldo-install/tests/e2e_sec_exec_m5.rs` | Fixture and docs tests for end-to-end shape. |

### Acceptance Scenarios

| Scenario | Then |
|---|---|
| Secure capability found | dogfood shows matched catalog `bom_ref` and implementation/test expectation. |
| Capability gap found | dogfood shows SLO-intake/upstream decision branch and no silent local hand-roll. |
| Scanner not applicable | dogfood records N/A with reason rather than blank or fake evidence. |
| Loop docs updated | newcomer can answer "what does `/slo-execute` do when a secure library gap blocks implementation?" |

---

## Open Questions Before Execution

- Should the secure-construction gate be mandatory for every `/slo-execute`, or only when `security_libs_required: true` or the milestone touches a listed surface?
- Should fixing upstream Hulumi / SunLitSecurityLibraries be a ticket-sized SLO flow by default, or a separate full runbook when the change spans multiple repos?
- Should TypeScript and Java capability catalogs be introduced in this runbook, or left as a follow-up after the control-first fallback lands?
- Should `/slo-verify` retain any generic DAST alternative beside `zaprun`, or fully converge on `/slo-dast-tuner` for SLO-owned DAST?

## Suggested Next Step

Run `/slo-critique` against this draft before execution. The highest-risk critique questions are scope size, whether `/slo-execute` becomes too heavy, and whether DAST convergence should be a separate runbook.
