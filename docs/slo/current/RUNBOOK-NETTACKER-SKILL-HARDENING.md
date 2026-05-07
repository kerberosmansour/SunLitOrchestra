# Nettacker Skill Hardening - SunLitOrchestra (AI-First Runbook v4)

> **Purpose**: Harden `/slo-nettacker` from the Juice Shop and NodeGoat lab run recommendations: baseline probes, noisy-module triage, confidentiality checks, Docker runner metadata, and teardown handoff.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Execute the single milestone below without widening scope. This is a docs/skill-pack milestone, not a live security assessment.
> **Prerequisite reading**: [skills/slo-nettacker/SKILL.md](../../../skills/slo-nettacker/SKILL.md), [assessment workflow](../../../skills/slo-nettacker/references/assessment-workflow.md), [Nettacker location reference](../../../skills/slo-nettacker/references/nettacker-location.md), and the source recommendations at `/Users/sherifmansour/Dev/GitHub/Dast.Spike/.sldo/nettacker/2026-05-07-juiceshop-nodegoat/recommendations-skill.md`.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `nettacker-skill-hardening` |
| Project name | `SunLitOrchestra` |
| Primary stack | Markdown skill docs + Rust structural-contract tests |
| Primary package/app names | `skills/slo-nettacker`, `crates/sldo-install` |
| Prefix for tests and lesson files | `nettacker-hardening` |
| Default unit test command | `cargo test -p sldo-install --test e2e_slo_nettacker` |
| Default integration/BDD test command | `cargo test -p sldo-install --test e2e_slo_nettacker` |
| Default E2E/runtime validation command | `cargo test -p sldo-install --test e2e_slo_nettacker` |
| Default build/boot command | `cargo test -p sldo-install --test e2e_slo_nettacker` |
| Default formatter command | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_slo_nettacker.rs` |
| Default static analysis / lint command | `cargo clippy -p sldo-install --test e2e_slo_nettacker -- -D warnings` |
| Default dependency / security audit command | `N/A - no dependency changes` |
| Default debugger or state-inspection tool | `direct Markdown and Rust test inspection` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public Interfaces That Must Remain Stable

- `/slo-nettacker` skill name and frontmatter shape.
- Existing assessment artifact paths under `.sldo/nettacker/<date>-<slug>/`.
- Existing Nettacker runner-resolution options: checkout, PATH CLI, Poetry, Docker, API.
- Existing authorization and credential-testing hard gates.

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Encode lab-derived Nettacker safety and triage hardening | `done` | 2026-05-07 | 2026-05-07 | [docs/slo/lessons/nettacker-hardening-m1.md](../lessons/nettacker-hardening-m1.md) | [docs/slo/completion/nettacker-hardening-m1.md](../completion/nettacker-hardening-m1.md) |

<!-- Status values: not_started | in_progress | blocked | done -->

## 3. Architecture Diagram

```text
User request
  |
  v
/slo-nettacker SKILL.md
  |-- authorization + auto-mode gates
  |-- output confidentiality precondition
  |
  v
references/assessment-workflow.md
  |-- pre-flight
  |-- baseline / SPA / wildcard probes
  |-- recon + active checks
  |-- noisy-module validation notes
  |-- response-header cross-check
  |-- teardown handoff
  |
  v
references/nettacker-location.md
  |-- runner resolution
  |-- Docker image / host architecture record
```

## 4. High-Level Design for State Modeling / Formal Verification

`tla_required: false`

This milestone changes Markdown instructions and one structural-contract test. It introduces no concurrent actors, persistent state machine, resource ownership protocol, queue, retry loop, or distributed ordering guarantee.

## 5. Background Context

### Current State

The `/slo-nettacker` skill already gates authorization, resolves the Nettacker runner before commands, records confidential assessment artifacts, and separates assessment, triage, monitoring, and custom module authoring.

### Problem

The Juice Shop and NodeGoat lab run exposed repeatable operator friction:

1. URL-probe modules generated noisy status-code-only matches against SPA/wildcard routes.
2. Header-oriented modules missed manually visible header posture gaps.
3. `-d/--skip-service-discovery` was useful when service discovery disagreed with manual HTTP liveness, but it needs diagnostic-fallback framing.
4. Confidential `.sldo/nettacker` artifacts need a write-time gitignore/tracking precondition.
5. Docker runner records should capture architecture mismatch because emulation can dominate wall-clock.
6. Handoff should list teardown commands for resources spun up during a lab assessment.

### Target Architecture

The skill remains Markdown-only. The assessment workflow gains a passive baseline phase and validation notes before expensive or noisy modules. Runner metadata gains Docker platform evidence. Output handling explicitly checks whether `.sldo/nettacker` artifacts are ignored and untracked before writing.

### Global Red Lines

- No live Nettacker scan.
- No commands against public targets.
- No new dependencies.
- No edits outside the allow-list.
- No weakening of authorization, credential-testing, or no-evasion gates.
- No one-lab-run claim promoted to universal truth without version/evidence scoping.

## 6. Milestone 1 - Encode Lab-Derived Nettacker Safety And Triage Hardening

### Goal

Update `/slo-nettacker` so future assessments explicitly baseline HTTP targets, detect SPA/wildcard false-positive modes, cross-check response headers, record Docker runner architecture, guard confidential report paths, and surface teardown commands.

### Contract Block

| Field | Value |
|---|---|
| Data classification | `Public` for skill docs and tests; assessment artifacts remain `Confidential` |
| Proactive controls in play | Authorization gate, output confidentiality precondition, bounded request-volume warning, lowest-impact validation, no-evasion rule |
| Abuse acceptance scenarios | N/A - no new live surface introduced; the skill continues to refuse unscoped public scanning and brute/default credential testing |
| Files allowed to change | `docs/slo/current/RUNBOOK-NETTACKER-SKILL-HARDENING.md`; `docs/slo/lessons/nettacker-hardening-m1.md`; `docs/slo/completion/nettacker-hardening-m1.md`; `docs/slo/verify/nettacker-hardening-m1.md`; `skills/slo-nettacker/**`; `crates/sldo-install/tests/e2e_slo_nettacker.rs`; `crates/sldo-install/tests/e2e_eng_imp_m5.rs`; `docs/skill-pack-catalog.md`; `docs/slo/design/agent-host-capabilities.md` |
| Files to read before changing | `skills/slo-nettacker/SKILL.md`; `skills/slo-nettacker/references/assessment-workflow.md`; `skills/slo-nettacker/references/nettacker-location.md`; source recommendations file in `Dast.Spike` |
| Forbidden shortcuts | Do not add runnable live-scan commands without the authorization gate; do not mark `-d` as a production default; do not auto-run teardown; do not report missing legacy `X-XSS-Protection` as a modern vulnerability |
| Resource bounds | No runtime resources introduced. URL-probe modules must warn when request volume is estimated above bounded thresholds |
| Invariants/assertions | Structural test asserts baseline, noisy-module, confidentiality, teardown, and Docker-architecture instructions exist |
| Static-analysis gates | Targeted `rustfmt --check`; targeted `cargo clippy -p sldo-install --test e2e_slo_nettacker -- -D warnings`; full `cargo test --workspace` |
| Compatibility | Existing `/slo-nettacker` invocation, runner-resolution paths, output paths, and module-authoring paths remain stable |

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Baseline before URL probes | happy path | A user plans `dir_scan`, `admin_scan`, or `pma_scan` | The assessment workflow is read | It requires passive baseline headers, OPTIONS, and wildcard/body fingerprint probes first |
| Noisy hits classified | false-positive triage | A SPA or catch-all returns identical unknown-path responses | Results are triaged | URL-probe hits matching baseline fingerprint are candidates or false positives, not reportable findings |
| Header no-hit is not proof | dependency gap | Nettacker header modules do not fire | Findings are written | The workflow requires `curl -I` style header cross-check and records missing headers as manual posture evidence |
| Confidential output guarded | abuse case | `.sldo/nettacker` would be written | The skill prepares report paths | It checks ignore/tracked state and refuses or warns before writing leak-prone artifacts |
| Docker runner records platform | empty/platform state | Docker is selected as runner | The runner record is written | It records image architecture, host architecture, and emulation risk if they differ |
| Teardown is explicit | cleanup | The assessment spins up Docker/lab resources | Handoff is written | It lists teardown commands and does not auto-run them without operator confirmation |

### Regression Tests

- Extend `crates/sldo-install/tests/e2e_slo_nettacker.rs` with structural assertions for each BDD scenario.

### Runtime Validation

- Run `cargo test -p sldo-install --test e2e_slo_nettacker`.
- Run `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_slo_nettacker.rs`.
- Run `cargo clippy -p sldo-install --test e2e_slo_nettacker -- -D warnings`.
- Run `cargo test --workspace`.

### Definition Of Done

- Structural tests fail before the docs update and pass after it.
- Skill docs encode the accepted R1/R4/R5/R6/R8 changes and the modified R2/R3/R7 framing.
- No live scan or external target is touched.
- Evidence log below is complete.
- Lessons and completion artifacts are written.

### Evidence Log

| Check | Command / File | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene | `git status --short --branch`; `git rev-parse --abbrev-ref HEAD`; `git symbolic-ref --short refs/remotes/origin/HEAD` | Branch is non-default and existing user changes are preserved | Branch `slo-nettacker-skill`; default `origin/main`; unrelated Fowler drafts and `.gitignore` change left unstaged | pass | No branch switch needed |
| Baseline test | `cargo test -p sldo-install --test e2e_slo_nettacker` | Current scaffold passes before new hardening assertions | 6 passed before adding hardening assertions | pass | Baseline captured |
| BDD red | `cargo test -p sldo-install --test e2e_slo_nettacker` after test update | New hardening assertions fail before docs update | 5 passed, 4 failed for expected missing hardening strings | pass | Red for expected reason |
| BDD green | `cargo test -p sldo-install --test e2e_slo_nettacker` after docs update | All tests pass | 9 passed | pass | New structural contract satisfied |
| Formatter | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_slo_nettacker.rs` | Pass | Passed | pass | Full `cargo fmt --check` is pre-existing red outside allow-list |
| Static analysis | `cargo clippy -p sldo-install --test e2e_slo_nettacker -- -D warnings` | Pass | Passed | pass | Full package clippy is pre-existing red outside allow-list |
| Full test suite | `cargo test --workspace` | Pass | Passed | pass | Workspace tests green |
| Runtime QA | `docs/slo/verify/nettacker-hardening-m1.md` | Verification report covers all BDD scenarios | Report written with all scenarios pass | pass | Markdown-only milestone |
| Gitignore review | `git check-ignore -v .sldo/nettacker/marker`; `git ls-files .sldo/nettacker`; `git status --porcelain -- .sldo/nettacker` | `.sldo/nettacker` artifacts are protected and no test artifacts are left | `.sldo/nettacker/marker` ignored by `.gitignore`; no tracked or pending `.sldo/nettacker` artifacts | pass | Existing `.sldo/refresh-loop.toml` is outside Nettacker artifact path |
