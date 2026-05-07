# Fowler AI Architecture SLO Improvements — SunLitOrchestra (AI-First Runbook v4)

> **Purpose**: Add Fowler-informed architecture and AI-era engineering disciplines to the SLO skill pack: reversibility, brownfield code maps, exemplar code, true refactoring discipline, AI tolerance contracts, architecture-coherence critique, and ticket-flow parity.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Prefer automated guardrails over developer intention. Prefer direct inspection over guessing. Prefer executable assumptions over comments. Prefer bounded design over silent growth. Prefer evidence over claims.
> **Prerequisite reading**: [ARCHITECTURE.md](../../ARCHITECTURE.md), [docs/LOOPS-ENGINEERING.md](../../LOOPS-ENGINEERING.md), [docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md](../../PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md), [research synthesis](../research/fowler-ai-architecture-slo-improvements/synthesis.md), [design overview](../design/fowler-ai-architecture-slo-improvements-overview.md).

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `fowler-ai-architecture-slo-improvements` |
| Project name | `SunLitOrchestra` |
| Primary stack | Markdown skill pack + Rust structural-contract tests |
| Primary package/app names | `skills/slo-architect`, `skills/slo-plan`, `skills/slo-verify`, `skills/slo-critique`, `skills/slo-ticket-*`, `crates/sldo-install` |
| Prefix for tests and lesson files | `fowler-ai-arch` |
| Default unit test command | `cargo test -p sldo-install` |
| Default integration/BDD test command | `cargo test --workspace` |
| Default E2E/runtime validation command | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m<N>` |
| Default build/boot command | `cargo build --workspace` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy --workspace --all-targets -- -D warnings` |
| Default dependency / security audit command | `N/A — no dependency graph changes planned` |
| Default debugger or state-inspection tool | `rg`, `git diff`, `cargo test -- --nocapture` for Markdown structural assertions |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public Interfaces That Must Remain Stable Unless Explicitly Listed Otherwise

- Existing skill invocation paths: `skills/<name>/SKILL.md`.
- Existing installer discovery rule: `skills/<name>/SKILL.md`.
- Existing v4 runbook rows and ticket contract rows.
- Existing Codex/GitHub Copilot/Claude Code host-boundary statements.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-architect` reversibility matrix + brownfield code map | `done` | 2026-05-07 | 2026-05-07 | `docs/slo/lessons/fowler-ai-arch-m1.md` | `docs/slo/completion/fowler-ai-arch-m1.md` |
| 2 | `/slo-plan` exemplar-code rows + true refactoring discipline | `not_started` | | | | |
| 3 | AI nondeterminism tolerance contract across architect/plan/verify | `not_started` | | | | |
| 4 | `/slo-critique` architecture-coherence review pass | `not_started` | | | | |
| 5 | Ticket-flow parity + catalog/docs structural checks | `not_started` | | | | |

---

## 3. End-to-End Architecture Diagram

```text
┌────────────────────────────────────────────────────────────────────┐
│             Fowler-informed SLO planning contracts                 │
│                                                                    │
│  research + design docs                                            │
│     │                                                              │
│     ▼                                                              │
│  M1 /slo-architect                                                 │
│     ├── <slug>-reversibility.md                                    │
│     └── <slug>-code-map.md                                         │
│              │                                                     │
│              ▼                                                     │
│  M2 /slo-plan + v4 template                                        │
│     ├── exemplar code rows                                         │
│     ├── anti-exemplar rows                                         │
│     └── refactoring-discipline.md                                  │
│              │                                                     │
│              ▼                                                     │
│  M3 AI tolerance contract                                          │
│     ├── ai-tolerance-contract.md                                   │
│     └── /slo-verify AI pass                                        │
│              │                                                     │
│              ▼                                                     │
│  M4 /slo-critique architecture coherence                           │
│     └── four-object / exemplar / reversibility checks              │
│              │                                                     │
│              ▼                                                     │
│  M5 ticket-sized SLO parity                                        │
│     └── ticket template + ticket plan/execute/verify updates       │
│                                                                    │
│  Deterministic guardrail: crates/sldo-install/tests/e2e_*.rs       │
└────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `skills/slo-architect/SKILL.md` | Emits design artifacts before planning | Changed | M1, M3 | Output artifact list, Step 3/4 handoff |
| `skills/slo-plan/SKILL.md` | Orchestrates v4 runbook authoring | Changed | M2, M3 | Contract Block sentinels |
| `skills/slo-plan/references/refactoring-discipline.md` | Defines true refactor microstep/evidence discipline | New | M2 | Cited from `/slo-plan`, template, ticket plan |
| `skills/slo-plan/references/ai-tolerance-contract.md` | Defines AI nondeterminism tolerance contract | New | M3 | Cited from `/slo-plan`, `/slo-verify`, template |
| `skills/slo-verify/SKILL.md` | Runtime QA | Changed | M3 | AI tolerance pass after normal runtime passes |
| `skills/slo-critique/SKILL.md` and `personas/eng.md` | Adversarial plan review | Changed | M4 | Architecture-coherence checks |
| `skills/slo-ticket-*` and ticket templates | Issue-sized SLO flow | Changed | M5 | Contract rows aligned with sprint flow |
| Rust structural tests | Contract enforcement | New | M1-M5 | `cargo test -p sldo-install` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Hard-to-change decisions | `/slo-architect` | `/slo-plan` | `*-reversibility.md` citation | yes | irreversible decision locked silently | M1-M2 |
| Brownfield exemplars | `/slo-architect` | `/slo-plan`, tickets | `*-code-map.md` citation | yes | AI copies bad legacy shape | M1-M2, M5 |
| Refactoring intent | Plan/ticket | Execute/verify | refactor budget + microsteps | yes | behavior change hidden as refactor | M2, M5 |
| AI tolerance | Architecture/plan | Verify | tolerance table + eval evidence | yes | nondeterministic behavior passes by anecdote | M3 |
| Coherence review | Critique | Runbook revision | findings table | yes | architecture docs and milestones diverge | M4 |

---

## 4. High-Level Design for Formal Verification

`tla_required: false`

This runbook edits Markdown contracts and Rust structural tests. No concurrent actors, distributed state, resource ownership, ordering guarantees, or failure recovery protocol are introduced. The right correctness tools are structural-contract tests, BDD/eval cases, and `/slo-critique`.

---

## 5. Background Context

### Current State

SLO already has a strong feedback loop: ideate, research, architect, plan, critique, execute, verify, retro, ship. The v4 template requires BDD-first tests, runtime validation, bounded resources, invariants, static analysis, evidence logs, and a refactor budget. `/slo-critique` already runs CEO, engineering, security, and design passes before execution.

The missing pieces are the Fowler-specific disciplines identified by research:

1. Architecture should identify hard-to-change decisions and reduce irreversibility.
2. Architecture is communicated through shared understanding and exemplar code, not just diagrams.
3. Brownfield systems require comprehension before modification.
4. Refactoring means behavior-preserving microsteps with testing.
5. AI/LLM behavior requires explicit tolerance boundaries because outputs are nondeterministic.

### Problem

1. **Reversibility is implicit**: `/slo-architect` locks stack and interfaces but does not require a reversibility matrix for hard-to-change decisions.
2. **Brownfield comprehension is too thin**: existing stack detection reads manifests but does not produce a four-object system map or exemplar/anti-exemplar list.
3. **Exemplar code is absent from Contract Blocks**: agents know which files to read but not which code shape to copy.
4. **Refactor budget is underspecified**: `Targeted refactor permitted` can still hide non-refactoring behavior changes.
5. **AI nondeterminism is security-modeled but not behavior-modeled**: `ai_component: true` gates threat-model content, but not tolerance/eval proof.
6. **Ticket flow would drift if only sprint flow is updated**: issue-sized contracts need the same discipline in compact form.

### Target Architecture

See [design overview](../design/fowler-ai-architecture-slo-improvements-overview.md), [interfaces](../design/fowler-ai-architecture-slo-improvements-interfaces.md), [reversibility matrix](../design/fowler-ai-architecture-slo-improvements-reversibility.md), and [code map](../design/fowler-ai-architecture-slo-improvements-code-map.md).

### Key Design Principles

1. **Additive only**: add new rows/artifacts; do not rename existing skill paths or Contract Block rows.
2. **N/A with reason**: every new discipline must have an honest N/A path for docs-only, refactor-only, or non-AI work.
3. **Skill-local first**: behavior changes land in SKILL.md and skill-local references, then repo mirror templates.
4. **Structural tests guard prose**: every new required output/row gets a deterministic test.
5. **Ticket parity**: sprint and issue-sized flows must converge on the same planning vocabulary.

### What To Keep

- `docs/LOOPS-ENGINEERING.md` loop shape.
- Existing `skills/slo-*` invocation paths.
- Existing host-boundary docs.
- Existing v4 template rows.
- Existing security/threat-model gates.

### What To Change

- `/slo-architect` output list and method.
- `/slo-plan` sentinels and milestone-authoring reference.
- v4 runbook template.
- `/slo-verify` runtime QA passes.
- `/slo-critique` engineering persona.
- Ticket contract template and ticket plan/execute/verify prose.
- Rust structural tests.

### Global Red Lines

- No unrelated Nettacker edits.
- No new dependencies.
- No host-runtime abstraction.
- No one-off template row names that appear in docs but not skills.
- No broad SKILL.md growth without reference extraction.
- No relying on the YouTube/Gemini notes for source-backed claims unless the claim is also supported by cited sources.

---

## 6. Carry-Forward From Prior Retros

No prior `fowler-ai-arch` retros exist. First execution of this runbook should output `no carry-forward from prior retros (this is M1)`.

---

## 7. Milestone Plan

### Milestone 1 — `/slo-architect` Reversibility Matrix + Brownfield Code Map

**Goal**: `/slo-architect` emits two new additive design artifacts: `docs/slo/design/<slug>-reversibility.md` and `docs/slo/design/<slug>-code-map.md`.

**Context**: Fowler/Dishman architecture guidance emphasizes hard-to-change decisions, reversibility, and exemplar code. SLO currently emits architecture, stack decision, interfaces, SECURITY.md, and a threat model, but not a reversibility matrix or brownfield code map.

**Carmack-style reliability goal**: strengthen compatibility and direct inspection before planning.

**Important design rule**: Brownfield code maps are required only when the target repo is non-empty. Greenfield projects get `N/A — greenfield; no existing codebase to map`.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Research synthesis, design overview, existing `/slo-architect` skill, prior engineering-skill-improvements lessons |
| Outputs | Updated `/slo-architect`; two new documented output artifacts; structural test |
| Interfaces touched | `/slo-architect` output artifact list and Method steps |
| Files allowed to change | `skills/slo-architect/SKILL.md`; `skills/slo-architect/evals/happy-path.md`; `skills/slo-architect/evals/high-risk-case.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m1.rs` (NEW) |
| Files to read before changing | `skills/slo-architect/SKILL.md`; `docs/slo/design/fowler-ai-architecture-slo-improvements-{overview,reversibility,code-map}.md`; `docs/slo/lessons/eng-imp-m1.md`; `docs/slo/lessons/slo-sec-m1.md` |
| New files allowed | `crates/sldo-install/tests/e2e_fowler_ai_arch_m1.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing `/slo-architect` outputs remain documented and unchanged; new outputs are additive |
| Resource bounds introduced/changed | N/A — Markdown contract change only |
| Invariants/assertions required | Structural test asserts `*-reversibility.md` and `*-code-map.md` are named; existing five architect outputs still present; stale "Five files" wording is updated to the new count |
| Debugger / inspection expectation | Use `rg` and `git diff` to verify no existing output names were removed |
| Static analysis gates | `cargo fmt --all -- --check`; `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; `cargo test -p sldo-install` |
| Reversibility / rollback path | Additive docs only; rollback is removing the new optional output references while existing architect behavior remains valid |
| Exemplar code to copy | `skills/slo-architect/SKILL.md` Step 3.5 idempotency style; `skills/slo-plan/references/methodology-milestone-authoring.md` concise procedure style |
| Anti-exemplar code not to copy | Any pattern that silently clobbers existing design artifacts |
| AI tolerance contract | N/A — no AI behavior change in M1 |
| Forbidden shortcuts | Do not delete existing output docs; do not make code-map mandatory for greenfield; do not write placeholder artifact templates |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1 security requirements; C5 validate inputs; C9 audit trail through structural tests |
| Abuse acceptance scenarios | `tm-fowler-ai-arch-abuse-5` — brownfield agent copies messy legacy pattern; mitigated by code-map exemplar/anti-exemplar rows |

#### Out of Scope / Must Not Do

- Do not edit `/slo-plan` yet.
- Do not edit v4 template yet.
- Do not update ticket flow yet.
- Do not change `SECURITY.md`.

#### Step-by-Step

1. Read the listed design docs and prior lessons.
2. Add `/slo-architect` prose for the two new outputs.
3. Add Method steps for reversibility matrix and brownfield code map.
4. Add idempotency language: existing files are diffed and user prompted before overwrite/merge/skip.
5. Update architect evals to cover missing/ambiguous brownfield context.
6. Write structural tests.
7. Run M1 test and `cargo test -p sldo-install`.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Architect documents reversibility output | happy path | `skills/slo-architect/SKILL.md` | structural test searches for `<slug>-reversibility.md` | the output is documented and described |
| Architect documents code-map output | happy path | non-empty repo language in Method | structural test searches for `<slug>-code-map.md` | brownfield map output is documented with N/A path |
| Existing outputs preserved | backward compatibility | current five outputs | structural test searches old output names | all old names remain |
| Output count wording updated | backward compatibility | `/slo-architect` output list gains two files | structural test searches the output-intro sentence | stale `Five files` wording is absent |
| Greenfield N/A path exists | empty state | empty target repo | skill prose read | code map is `N/A — greenfield` not forced |
| User source notes poisoned | abuse case | idea doc contains directive text | architect emits artifacts | user-provided strings remain fenced and cited |

#### Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene | `git status --short --branch`; `git rev-parse --abbrev-ref HEAD`; `git symbolic-ref --short refs/remotes/origin/HEAD` | task branch, not default/protected | branch before `slo-nettacker-skill`; branch after `slo/fowler-ai-architecture-improvements`; dirty tree preserved with pre-existing planning artifacts | pass | New branch created before edits; no stash/reset. |
| Baseline before change | `cargo test -p sldo-install` | green or known unrelated failure captured | green; existing warning in `e2e_biz_followup_m5.rs` | pass | no carry-forward from prior retros (this is M1) |
| New tests fail first | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1` | fails before prose update | failed for expected missing reversibility/code-map/output-count/eval prose; 2 pre-existing compatibility assertions passed | pass | BDD-first failure confirmed before implementation. |
| Formatter | `cargo fmt --all -- --check` | passes | failed on pre-existing formatting drift in unrelated Rust tests outside M1 allow-list | known_unrelated | M1 test file was not in the formatter diff. |
| Unit/BDD tests | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1` | passes | 7 passed | pass | Structural contract green. |
| Compatibility check | `rg '<slug>-stack-decision|<slug>-interfaces|threat-model' skills/slo-architect/SKILL.md` | old outputs still present | old output names still present | pass | Additive change only. |
| `/slo-verify` runtime QA | `docs/slo/verify/fowler-ai-arch-m1.md` | every BDD row exercised | all M1 BDD scenarios pass; no bugs found | pass | Pass 4: `cargo audit` pass; Semgrep pass; `ast-grep` skipped; DAST/PII N/A; `cargo deny` skipped due missing project config/default-policy noise. |
| `.gitignore` / artifact cleanup | `git status --short` | no stray artifacts from tests | no Pass 4 artifacts written into repo; expected source/docs edits only | pass | Semgrep JSON written to `/tmp`. |

#### Definition of Done

- [x] `/slo-architect` documents both new outputs.
- [x] Existing outputs remain documented.
- [x] Greenfield and brownfield behavior both have explicit paths.
- [x] M1 structural tests pass.
- [x] Evidence Log rows filled.

---

### Milestone 2 — `/slo-plan` Exemplar-Code Rows + True Refactoring Discipline

**Goal**: New v4 runbooks include exemplar/anti-exemplar rows and true refactoring discipline in `/slo-plan`, skill-local references, and the v4 template.

**Context**: Fowler/Dishman emphasize code examples as architecture drivers, and Fowler defines refactoring as small behavior-preserving transformations. Current SLO contracts have file read-lists and refactor budgets, but not exemplar rows or microstep proof.

**Carmack-style reliability goal**: strengthen direct inspection and compatibility.

**Important design rule**: Refactoring discipline applies only when the milestone's refactor budget is not `No refactor permitted beyond direct implementation`.

**Refactor budget**: `Targeted refactor permitted for extracting refactoring-discipline prose into a skill-local reference`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1 architect outputs, research synthesis, existing `/slo-plan` methodology |
| Outputs | Updated `/slo-plan`; new `refactoring-discipline.md`; v4 template rows; structural tests |
| Interfaces touched | Runbook Contract Block row names and `/slo-plan` sentinels |
| Files allowed to change | `skills/slo-plan/SKILL.md`; `skills/slo-plan/references/methodology-milestone-authoring.md`; `skills/slo-plan/references/refactoring-discipline.md` (NEW); `skills/slo-plan/references/runbook-template_v_4_template.md`; `docs/slo/templates/runbook-template_v_4_template.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m2.rs` (NEW) |
| Files to read before changing | M1 lessons; `skills/slo-plan/SKILL.md`; `skills/slo-plan/references/methodology-milestone-authoring.md`; `skills/slo-plan/references/runbook-template_v_4_template.md`; `docs/slo/templates/runbook-template_v_4_template.md`; `docs/slo/design/fowler-ai-architecture-slo-improvements-code-map.md` |
| New files allowed | `skills/slo-plan/references/refactoring-discipline.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m2.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing v4 Contract Block rows remain; new rows are additive |
| Resource bounds introduced/changed | N/A — Markdown contract change only |
| Invariants/assertions required | Structural test asserts new rows and no deletion of `Refactor budget` row |
| Debugger / inspection expectation | Use `git diff --word-diff` to inspect template row additions |
| Static analysis gates | formatter + `cargo test -p sldo-install --test e2e_fowler_ai_arch_m2` + `cargo test -p sldo-install` |
| Reversibility / rollback path | If rows prove noisy, allow `N/A — no brownfield exemplar needed` while keeping references additive |
| Exemplar code to copy | `skills/slo-plan/references/methodology-milestone-authoring.md`; v4 existing Contract Block shape |
| Anti-exemplar code not to copy | Broad prose added directly to `SKILL.md` when it belongs in references |
| AI tolerance contract | N/A — M3 owns AI-specific row |
| Forbidden shortcuts | Do not redefine refactoring as generic cleanup; do not allow free-form row names; do not remove v4 rows |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1; C9 audit trail |
| Abuse acceptance scenarios | `tm-fowler-ai-arch-abuse-4` — behavior change hidden as refactor; mitigated by refactoring microstep proof |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Plan emits exemplar row | happy path | v4 Contract Block | structural test parses template | `Exemplar code to copy` exists |
| Plan emits anti-exemplar row | happy path | v4 Contract Block | structural test parses template | `Anti-exemplar code not to copy` exists |
| Refactor discipline reference exists | happy path | skill-local references | structural test opens file | file defines behavior-preserving, pre-test, microstep, post-test proof |
| No refactor budget preserved | backward compatibility | current v4 template | structural test searches | `Refactor budget` remains |
| Docs-only milestone | empty state | no code change | plan row | exemplar rows can be `N/A — docs-only` |

#### Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install` | green or known unrelated failure captured | | pending | |
| New tests fail first | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m2` | fails before rows/reference exist | | pending | |
| Formatter | `cargo fmt --all -- --check` | passes | | pending | |
| Unit/BDD tests | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m2` | passes | | pending | |
| Full tests | `cargo test --workspace` | green or documented unrelated failure | | pending | |
| `.gitignore` / artifact cleanup | `git status --short` | no stray test artifacts | | pending | |

#### Definition of Done

- [ ] `/slo-plan` cites exemplar and refactoring discipline.
- [ ] v4 template includes new additive rows.
- [ ] `refactoring-discipline.md` exists and is cited.
- [ ] M2 structural tests pass.

---

### Milestone 3 — AI Nondeterminism Tolerance Contract Across Architect / Plan / Verify

**Goal**: Systems with `ai_component: true` get an explicit AI tolerance contract that `/slo-plan` emits and `/slo-verify` exercises.

**Context**: `/slo-architect` already gates AI-specific security content through `ai_component: true`, but SLO has no behavior-tolerance contract for nondeterministic AI outputs.

**Carmack-style reliability goal**: make AI assumptions executable.

**Important design rule**: AI tolerance is required only when a milestone introduces, modifies, or verifies AI/LLM behavior.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | User-provided Fowler AI notes; Thoughtworks AI cognitive-debt source; current `/slo-verify` Pass 4 |
| Outputs | AI tolerance reference; updated architect/plan/verify prose; skill-local and docs-mirror v4 template AI row; structural tests |
| Interfaces touched | `ai_component: true` downstream behavior; `/slo-verify` pass list |
| Files allowed to change | `skills/slo-architect/SKILL.md`; `skills/slo-plan/SKILL.md`; `skills/slo-plan/references/methodology-milestone-authoring.md`; `skills/slo-plan/references/ai-tolerance-contract.md` (NEW); `skills/slo-plan/references/runbook-template_v_4_template.md`; `skills/slo-verify/SKILL.md`; `docs/slo/templates/runbook-template_v_4_template.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m3.rs` (NEW) |
| Files to read before changing | M1-M2 lessons; current files above; `docs/slo/research/fowler-ai-architecture-slo-improvements/synthesis.md`; `docs/slo/design/fowler-ai-architecture-slo-improvements-threat-model.md` |
| New files allowed | `skills/slo-plan/references/ai-tolerance-contract.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m3.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Non-AI milestones can write `N/A — no AI component`; existing Pass 1-4 verification remains |
| Resource bounds introduced/changed | AI sample counts/eval budgets must be bounded in the contract |
| Invariants/assertions required | Structural test asserts AI row includes accepted variance, deterministic boundary, eval evidence, and must-never outcomes |
| Debugger / inspection expectation | Use real artifact examples or eval case files; do not infer pass from a single sample |
| Static analysis gates | formatter + M3 structural test + `cargo test -p sldo-install` |
| Reversibility / rollback path | AI pass is gated; rollback can keep reference while marking non-AI paths N/A |
| Exemplar code to copy | `/slo-verify` Pass 4 tool-error vs finding discipline; explicit skipped/N/A vocabulary |
| Anti-exemplar code not to copy | One-off "LLM looks right" smoke checks without tolerance thresholds |
| AI tolerance contract | Required: accepted variance, deterministic boundary, eval/golden set, retry/fallback, must-never outcomes, sample budget |
| Forbidden shortcuts | No claiming deterministic AI output; no unbounded retry/sample loops; no accepting screenshots or one sample as proof |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1/C5; AI-specific threat rows from M1 design |
| Abuse acceptance scenarios | `tm-fowler-ai-arch-abuse-3` — nondeterministic behavior marked pass by anecdote; mitigated by AI tolerance evidence |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| AI tolerance reference exists | happy path | skill-local references | structural test opens file | required tolerance fields present |
| Verify has AI pass | happy path | `skills/slo-verify/SKILL.md` | structural test searches | AI tolerance pass documented after normal runtime passes |
| Non-AI milestone | empty state | no AI component | plan row | `N/A — no AI component` accepted |
| Unbounded eval sample count | resource bound | AI contract omits sample budget | structural test runs | fails with missing sample budget |
| Must-never outcome missing | abuse case | AI milestone lacks banned outcome | critique/structural test runs | fails or emits finding |

#### Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install` | green or known unrelated failure captured | | pending | |
| New tests fail first | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m3` | fails before AI reference/pass exist | | pending | |
| Formatter | `cargo fmt --all -- --check` | passes | | pending | |
| Unit/BDD tests | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m3` | passes | | pending | |
| Full tests | `cargo test --workspace` | green or documented unrelated failure | | pending | |

#### Definition of Done

- [ ] `ai-tolerance-contract.md` exists.
- [ ] `/slo-plan` emits AI tolerance row when applicable.
- [ ] `/slo-verify` documents AI tolerance pass.
- [ ] Non-AI N/A path is explicit.
- [ ] M3 structural tests pass.

---

### Milestone 4 — `/slo-critique` Architecture-Coherence Review Pass

**Goal**: `/slo-critique` engineering persona checks architecture coherence using four-object summary, reversibility rows, exemplar rows, and AI tolerance rows.

**Context**: Fowler/Dishman emphasize shared architecture understanding, hands-on architecture, and exemplars. `/slo-critique` already catches hidden assumptions and missing failure modes, but it does not explicitly compare architecture docs, code map, and milestone contracts for coherence.

**Carmack-style reliability goal**: catch design drift before execution.

**Important design rule**: This stays inside the engineering persona; do not add a fifth persona or new command.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1-M3 outputs, current critique skill/personas |
| Outputs | Updated critique skill/persona/evals; structural test |
| Interfaces touched | `/slo-critique` findings expectations |
| Files allowed to change | `skills/slo-critique/SKILL.md`; `skills/slo-critique/personas/eng.md`; `skills/slo-critique/evals/happy-path.md`; `skills/slo-critique/evals/high-risk-case.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m4.rs` (NEW) |
| Files to read before changing | M1-M3 lessons; `skills/slo-critique/SKILL.md`; `skills/slo-critique/personas/eng.md`; `docs/slo/design/fowler-ai-architecture-slo-improvements-code-map.md` |
| New files allowed | `crates/sldo-install/tests/e2e_fowler_ai_arch_m4.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing CEO/security/design personas and finding categories remain |
| Resource bounds introduced/changed | N/A — Markdown contract change only |
| Invariants/assertions required | Test asserts eng persona names four-object summary, reversibility, exemplar rows, and AI tolerance rows |
| Debugger / inspection expectation | Use `rg` to verify no persona headings removed |
| Static analysis gates | formatter + M4 structural test + `cargo test -p sldo-install` |
| Reversibility / rollback path | If too noisy, findings can be `defer` unless concrete scenario exists |
| Exemplar code to copy | Current `skills/slo-critique/personas/security.md` acceptance-gate specificity style |
| Anti-exemplar code not to copy | Generic "architecture could be better" critique |
| AI tolerance contract | Critique must flag missing tolerance rows on AI milestones |
| Forbidden shortcuts | Do not add vague findings; every coherence finding needs actor/action/bad outcome |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1/C9 |
| Abuse acceptance scenarios | `tm-fowler-ai-arch-abuse-2` — sprint/ticket standards drift; caught by coherence + M5 parity |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Four-object check documented | happy path | eng persona file | structural test searches | four-object architecture check present |
| Exemplar mismatch finding | happy path | runbook cites exemplar absent from code map | critique eval runs manually | finding recommends updating row or code map |
| Missing reversibility row | invalid input | milestone touches hard-to-change interface | critique pass | finding asks for reversibility/rollback path |
| No UI runbook | empty state | no UI surface | critique | design persona still skipped; eng pass runs |
| Vague architecture concern | abuse case | reviewer writes "architecture feels messy" | finding gate | rejected for lacking concrete scenario |

#### Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install` | green or known unrelated failure captured | | pending | |
| New tests fail first | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m4` | fails before critique prose exists | | pending | |
| Formatter | `cargo fmt --all -- --check` | passes | | pending | |
| Unit/BDD tests | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m4` | passes | | pending | |
| Full tests | `cargo test --workspace` | green or documented unrelated failure | | pending | |

#### Definition of Done

- [ ] Eng persona has architecture-coherence checks.
- [ ] `/slo-critique` mentions the coherence pass.
- [ ] Critique evals cover missing/rejected coherence findings.
- [ ] M4 structural tests pass.

---

### Milestone 5 — Ticket-Flow Parity + Catalog / Docs Structural Checks

**Goal**: The issue-sized SLO ticket flow mirrors the new sprint-flow rows in compact form, and catalog/docs mention the new discipline without becoming competing catalogs.

**Context**: The ticket loop exists so small GitHub issues can keep v4 rigor without a full runbook. If only `/slo-plan` changes, ticket contracts become a lower-discipline path.

**Carmack-style reliability goal**: preserve cross-flow compatibility and prevent contract drift.

**Important design rule**: Ticket rows stay compact; do not copy the full v4 runbook template into ticket contracts.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1-M4 outputs, ticket flow skills/templates |
| Outputs | Ticket template/skill updates; catalog/docs note; structural test |
| Interfaces touched | Ticket Contract Block rows and validation plan |
| Files allowed to change | `skills/slo-ticket-plan/SKILL.md`; `skills/slo-ticket-execute/SKILL.md`; `skills/slo-ticket-verify/SKILL.md`; `skills/slo-ticket-plan/references/ticket-contract-template_v_1.md`; `docs/slo/templates/ticket-contract-template_v_1.md`; `docs/skill-pack-catalog.md`; `docs/ARCHITECTURE.md`; `crates/sldo-install/tests/e2e_fowler_ai_arch_m5.rs` (NEW) |
| Files to read before changing | M1-M4 lessons; all ticket files above; `docs/LOOPS-ENGINEERING.md`; `docs/slo/design/agent-host-capabilities.md` |
| New files allowed | `crates/sldo-install/tests/e2e_fowler_ai_arch_m5.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing ticket fields remain; new rows are additive and N/A-capable |
| Resource bounds introduced/changed | Ticket AI tolerance rows require bounded eval/sample budget when applicable |
| Invariants/assertions required | Test asserts ticket template includes exemplar, anti-exemplar, reversibility, refactoring, and AI tolerance rows |
| Debugger / inspection expectation | Use `git diff --word-diff` to verify compact row additions only |
| Static analysis gates | formatter + M5 structural test + `cargo test -p sldo-install` |
| Reversibility / rollback path | Additive ticket rows can be N/A for simple docs/test issues |
| Exemplar code to copy | `docs/slo/templates/ticket-contract-template_v_1.md` compact style |
| Anti-exemplar code not to copy | Copying entire v4 template into ticket flow |
| AI tolerance contract | Compact ticket row mirrors sprint-flow row |
| Forbidden shortcuts | Do not make ticket flow second-class; do not expand ticket sizing beyond one issue-sized change |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1/C9 |
| Abuse acceptance scenarios | `tm-fowler-ai-arch-abuse-2` — ticket flow drifts from sprint flow; mitigated by parity test |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Ticket template has parity rows | happy path | ticket template | structural test parses | selected rows present |
| Ticket plan asks for exemplars | happy path | ticket-plan skill | structural test searches | exemplar/anti-exemplar instructions present |
| Ticket execute restates new constraints | happy path | ticket-execute skill | structural test searches | constraints include exemplar/refactor/AI tolerance when present |
| Simple docs ticket | empty state | docs-only issue | ticket contract | rows can be N/A with reason |
| Oversized ticket | boundary | issue needs multi-surface architecture work | ticket-plan sizing gate | escalates to `/slo-plan` |

#### Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install` | green or known unrelated failure captured | | pending | |
| New tests fail first | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m5` | fails before ticket parity exists | | pending | |
| Formatter | `cargo fmt --all -- --check` | passes | | pending | |
| Unit/BDD tests | `cargo test -p sldo-install --test e2e_fowler_ai_arch_m5` | passes | | pending | |
| Full tests | `cargo test --workspace` | green or documented unrelated failure | | pending | |
| Catalog/doc check | `rg 'reversibility|exemplar|AI tolerance' docs/skill-pack-catalog.md docs/ARCHITECTURE.md` | concise orientation only | | pending | |

#### Definition of Done

- [ ] Ticket template has compact parity rows.
- [ ] Ticket plan/execute/verify consume the new rows.
- [ ] Catalog/architecture docs mention the discipline without duplicating skill contracts.
- [ ] M5 structural tests pass.
- [ ] All runbook milestones ready for `/slo-execute`.

---

## 8. Cross-Milestone Regression Tests

- `cargo test -p sldo-install`
- `cargo test --workspace`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `rg 'host-neutral|interactive hosts|headless runtime' docs/slo/design/agent-host-capabilities.md docs/ARCHITECTURE.md`
- `rg 'Exemplar code to copy|Anti-exemplar code not to copy|AI tolerance contract|Reversibility / rollback path' docs/slo/templates/runbook-template_v_4_template.md docs/slo/templates/ticket-contract-template_v_1.md`

## 9. Self-Review Gate

- [ ] Did every milestone stay inside its allow-list?
- [ ] Did every new Contract Block row have an N/A-with-reason path?
- [ ] Did skill-local references change before or alongside repo mirror templates?
- [ ] Did ticket flow reach parity with sprint flow?
- [ ] Did every new behavior get a structural-contract test?
- [ ] Did no host-runtime promise expand beyond current host-capability docs?
- [ ] Did all user/source-note content remain treated as untrusted unless source-backed?

## 10. Handoff

Run `/slo-critique fowler-ai-architecture-slo-improvements` before executing M1. After critique findings are accepted and folded in, execute milestones sequentially with `/slo-execute M1` through `/slo-execute M5`, verifying and retroing each milestone before starting the next.
