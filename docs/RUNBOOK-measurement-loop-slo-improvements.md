# Measurement Loop — Bringing Post-Release Product Learning Into The SLO Loop (AI-First Runbook v4)

> **Purpose**: Add a first-class **feature-performance / value-realisation loop** across SunLit Orchestra's existing skills so every non-trivial feature leaves planning with a written **success thesis** and a **measurement contract** (value hypothesis, leading + lagging metrics, guardrails, telemetry deliverables, diagnosis + experiment plan, privacy controls, scheduled readout), and prove the **failure bar** end-to-end: a runbook whose telemetry is missing / not firing / leaking PII is *caught* by `/slo-verify`'s new measurement pass, *remediated*, and *re-verified green*.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Prefer contracted measurement over best-effort afterthought. Prefer "smallest *complete* value slice" over "too-thin-to-teach-us-anything". Prefer pseudonymised, masked, minimised telemetry by default. A shipped feature is "done" only when we can answer *how do we know we did a good job, and what should we change next?*
> **Prerequisite reading**: [docs/slo/design/measurement-loop-slo-improvements-overview.md](slo/design/measurement-loop-slo-improvements-overview.md), [-interfaces.md](slo/design/measurement-loop-slo-improvements-interfaces.md), [-threat-model.md](slo/design/measurement-loop-slo-improvements-threat-model.md), [-reversibility.md](slo/design/measurement-loop-slo-improvements-reversibility.md), [-code-map.md](slo/design/measurement-loop-slo-improvements-code-map.md), [docs/slo/templates/runbook-template_v_4_template.md](slo/templates/runbook-template_v_4_template.md), [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md), [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md), [references/biz/artifact-schema.md](../references/biz/artifact-schema.md), `~/Downloads/deep-research-report(5).md` (research input).

> **What's new in v4 vs v3**: explicit Carmack-style reliability rules; extended Contract Block with resource bounds + invariants + static-analysis gates. This runbook is a Markdown-contract + structural-test enhancement to the skill pack itself (the same shape as `docs/RUNBOOK-kani-verification.md`), so "tests" are Rust structural-contract tests in `xtasks/sast-verify/tests/` that assert the new contract sentinels exist and that every other skill's baseline stays green.

---

## 0. How To Use This Template

Follow the v4 Global Entry Protocol (§7), Carmack practices (§4), and Global Exit Protocol (§8) from [docs/slo/templates/runbook-template_v_4_template.md](slo/templates/runbook-template_v_4_template.md). Those sections are not duplicated here; this runbook carries the project-specific Metadata, Architecture, §5, Background, and Milestone Plan.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `measurement-loop-slo-improvements` |
| Project name | SunLit Orchestra |
| Primary stack | Markdown skill pack (5 SKILL.md edits + v4 template + loop docs + 1 frontmatter key) + Rust structural-contract tests (`sast-verify`) |
| Primary package/app names | `skills/slo-{ideate,product,plan,verify,retro}`, `docs/slo/templates/`, `references/biz/artifact-schema.md`, `docs/LOOPS-*.md`, `sast-verify` (tests) |
| Prefix for tests and lesson files | `mloop` |
| Default unit test command | `cargo test -p sast-verify` |
| Default integration/BDD test command | `cargo test -p sast-verify` |
| Default E2E/runtime validation command | `./target/release/sldo-install --dry-run` (skill discovery unaffected); `cargo test -p sast-verify` (full structural gate) |
| Default build/boot command | `cargo build -p sast-verify -p sldo-install` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy --workspace --all-targets -- -D warnings` |
| Default dependency / security audit command | `cargo audit` |
| Default debugger or state-inspection tool | `cargo test -p sast-verify -- --nocapture` (assertion output); manual diff review of generated artifacts |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- `/slo-product` and `/slo-metrics` command verbs + `mode_arg` values + output paths (`docs/biz-public/product/`, `docs/biz-public/metrics.md`) — declared `stable` in [biz-skill-pack-interfaces.md](slo/design/biz-skill-pack-interfaces.md).
- The `/slo-product` (PM) vs `/slo-metrics` (financial) responsibility split — recorded prior critique decision.
- Existing biz artifact-schema frontmatter keys (`name`, `created`, `tier`, `archetype`, `skill`, `mode`, `mode_arg`, `pii_scan_override`, …) — not renamed or removed.
- Existing v4 template sections §1–§20 — not renumbered or removed (Measurement Contract is an insertion).
- `/slo-ideate` idea-doc section names (`## The pain`, `## Top risks`, `## Recommendation`, `## Open questions for /slo-research`) and forcing-question slot count.
- `/slo-verify` Pass 1–5 numbering (the measurement checks are an additive sub-pass, not a renumber).
- `/slo-retro` lessons + completion file paths and existing section names.
- Existing structural-contract test baselines in `xtasks/sast-verify/tests/` (esp. `sap_imp_m5_agents.rs` SHA baseline) — a new test is a sibling file; baseline edits, when a SKILL.md changes, are explicit and recorded.
- `discover_skills()` in `crates/sldo-install/src/install.rs` — no installer change; `references/biz/` stays ignored by discovery.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-ideate` philosophy shift (wedge → complete value slice) + Success thesis section | `done` | 2026-05-22 | 2026-05-22 | [mloop-m1](slo/lessons/mloop-m1.md) | [mloop-m1](slo/completion/mloop-m1.md) |
| 2 | `/slo-product metrics` feature measurement spec + `feature_measurement_spec` schema key | `done` | 2026-05-22 | 2026-05-22 | [mloop-m2](slo/lessons/mloop-m2.md) | [mloop-m2](slo/completion/mloop-m2.md) |
| 3 | v4 template Measurement Contract section + Contract Block row + `/slo-plan` requirement | `done` | 2026-05-22 | 2026-05-22 | [mloop-m3](slo/lessons/mloop-m3.md) | [mloop-m3](slo/completion/mloop-m3.md) |
| 4 | `/slo-verify` measurement pass + `/slo-retro` Results-vs-thesis + **failure-bar demo** | `not_started` | | | | |
| 5 | Document the Feature-performance loop in `LOOPS-ENGINEERING.md` + cross-ref in `LOOPS-BUSINESS.md` | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/mloop-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/mloop-m<N>.md -->

---

## 3. End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│             SunLit Orchestra — Feature-Performance Loop overlay (end state)             │
│                                                                                          │
│   IDEATE (M1)           PLAN (M3)             EXECUTE / VERIFY (M4)       RETRO (M4)      │
│  ┌─────────┐          ┌─────────┐           ┌──────────────┐          ┌─────────┐        │
│  │/slo-    │  success │/slo-plan│ measure-  │/slo-execute  │ telemetry│/slo-    │        │
│  │ ideate  │─ thesis ▶│ + v4    │─ ment    ▶│   ↓          │  evidence│ retro   │        │
│  │ §slice  │          │ template│ contract  │/slo-verify   │─────────▶│ results │        │
│  └────┬────┘          └────┬────┘           │ MEAS. PASS   │          │ vs      │        │
│       │                    │                │ (catch→fix→  │          │ thesis  │        │
│       │ feeds              │ cites          │  green)      │          └────┬────┘         │
│       ▼                    ▼                └──────┬───────┘               │ next runbook │
│  ┌──────────────────────────────┐                 │                       │  scope       │
│  │ /slo-product metrics (M2)     │◀────────────────┴───────────────────────┘             │
│  │  · feature measurement spec   │                                                        │
│  │  · feature_measurement_spec ▢ │   - - -▶  /slo-metrics (financial KPIs — UNCHANGED)    │
│  └──────────────────────────────┘                                                        │
│                                                                                          │
│  Feature-performance LOOP documented in LOOPS-ENGINEERING.md (M5) ◀── cross-ref ── LOOPS-BUSINESS.md │
│                                                                                          │
│  Legend:  ─── existing skill surface   - - - cross-skill reference   ▶ artifact/data flow │
│           NEW/changed surface introduced by this runbook is annotated (M1..M5)           │
│  Every SKILL.md/template/key change is gated by a sibling structural-contract test       │
│  in xtasks/sast-verify/tests/mloop_m<N>_*.rs (pre-fix fails → post-fix green).            │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `skills/slo-ideate/SKILL.md` | Reframe Q3 (wedge→complete value slice); add `## Success thesis` to idea-doc output | changed | M1 | idea-doc section contract |
| `xtasks/sast-verify/tests/mloop_m1_ideate.rs` | Assert M1 sentinels present | new | M1 | structural gate |
| `skills/slo-product/SKILL.md` | Add feature measurement spec to `mode_arg: metrics`; set `feature_measurement_spec: true` when present | changed | M2 | `mode_arg: metrics` (path unchanged) |
| `references/biz/artifact-schema.md` | Register one optional key `feature_measurement_spec: bool` | changed | M2 | biz frontmatter contract |
| `xtasks/sast-verify/tests/mloop_m2_product.rs` | Assert M2 sentinels + key registration | new | M2 | structural gate |
| `docs/slo/templates/runbook-template_v_4_template.md` | New optional Measurement Contract section + Contract Block row | changed | M3 | v4 runbook contract |
| `skills/slo-plan/SKILL.md` | Require Measurement Contract for value-bearing features | changed | M3 | runbook authoring |
| `xtasks/sast-verify/tests/mloop_m3_plan.rs` | Assert template section + plan requirement | new | M3 | structural gate |
| `skills/slo-verify/SKILL.md` | New measurement pass (event presence, telemetry PII/masking, failure-path emission, replay tagging) | changed | M4 | verify pass list |
| `skills/slo-retro/SKILL.md` | Add Results-vs-thesis section to lessons template | changed | M4 | lessons template |
| `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs` + fixtures | Assert pass sentinels + drive catch→remediate→green on a fixture | new | M4 | structural gate + failure-bar |
| `docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md` | Document feature-performance loop + cross-ref | changed | M5 | loop catalog |
| `xtasks/sast-verify/tests/mloop_m5_loops.rs` | Assert loop entry + cross-ref present | new | M5 | structural gate |

### Data Flow Summary

| Flow | From | To | Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Success thesis | `/slo-ideate` idea doc | `/slo-product`, `/slo-plan` | Markdown section | yes — 3 metrics + guardrails | absent → `/slo-plan` flags missing contract | M1 |
| Feature measurement spec | `/slo-product metrics` artifact | `/slo-plan` Measurement Contract | Markdown section + `feature_measurement_spec` flag | yes — 1 spec per feature | flag absent → treated as no spec (false) | M2 |
| Measurement Contract | runbook (v4) | `/slo-execute`, `/slo-verify` | runbook section + Contract Block row | yes — per milestone | missing for value-bearing feature → plan incomplete | M3 |
| Telemetry evidence | `/slo-execute` milestone | `/slo-verify` measurement pass | Evidence Log rows + emitted events | yes — per critical event | event not firing / PII unmasked → pass fails (caught) | M4 |
| Results vs thesis | `/slo-verify` + post-ship metrics | `/slo-retro` lessons | Markdown section | yes — per thesis metric | blank actuals → retro refuses (existing rule) | M4 |
| Loop documentation | this runbook | `docs/LOOPS-*` | Markdown loop entry | n/a | n/a | M5 |

---

## 4. Carmack-Style Development Best Practices

Apply §4 of [the v4 template](slo/templates/runbook-template_v_4_template.md) verbatim. Project-specific bindings:

- **Inspect state, do not guess** — when a structural test fails, read the assertion message and the actual file content (`cargo test -p sast-verify -- --nocapture`) before editing; do not guess sentinel wording.
- **Static analysis mandatory** — `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify` every milestone.
- **Assertions are executable comments** — each structural test asserts EXACT contiguous sentinel substrings (kani-m1 lesson: `contains()` is satisfied by interleaved prose, so assert the full contiguous phrase).
- **Bounded resources** — additions are bounded: one new optional key (not a cluster), one new section per artifact, ≤2 SKILL.md edits per milestone.
- **Make invalid states unrepresentable** — `feature_measurement_spec` is a `bool` (absent==false), never a tier value; the Measurement Contract section is optional so legacy runbooks stay valid.

---

## 5. High-Level Design for State Modeling / Formal Verification

`N/A — no concurrency, shared mutable state, ordering guarantees across processes, leases/locks, or failure-recovery protocol.` This runbook edits Markdown contracts + adds Rust structural-contract tests. Correctness is enforced by those tests (pre-fix-fails → post-fix-green, anti-vacuity) and by the existing `xtasks/sast-verify` gate, which are the property/contract-test substitutes the v4 template explicitly permits for non-concurrent systems.

### 5.8 Kani proof obligations

`N/A — no Rust kernels introduced.` The only Rust touched is structural-contract tests (SKILL.md/template sentinel + frontmatter-key assertions) — no `unsafe`, arithmetic, parser, or representation-invariant kernel worth a bounded proof. (`kani_required: false` in the overview.)

---

## 6–8. Global Rules / Entry / Exit Protocols

Apply §6, §7, §8 of [the v4 template](slo/templates/runbook-template_v_4_template.md) verbatim. The repo-specific binding for "run the full existing test suite" is: `cargo test -p sldo-common -p sldo-install -p sldo-research` (the documented baseline) **plus** `cargo test -p sast-verify` (the structural gate this runbook extends). A milestone never begins on a red baseline.

---

## 9. Background Context

### Current State

SunLit Orchestra has strong pre-release discipline: `/slo-ideate` (smallest-wedge forcing questions), `/slo-research`, `/slo-architect`, `/slo-plan` (v4 milestone contracts + evidence logs), `/slo-critique`, `/slo-execute`, `/slo-verify` (Pass 1–5 incl. Pass 4 PII scan over `docs/biz-public/`), `/slo-retro`. PM metrics live in `/slo-product metrics` (north-star, activation funnel, retention, feature adoption); financial KPIs live in `/slo-metrics`. Loops are catalogued in `docs/LOOPS-ENGINEERING.md` (sprint, secure-construction, ticket, security-tuning, lessons, library-feedback) and `docs/LOOPS-BUSINESS.md` (user-interview, GTM, pricing, founder-check).

### Problem

1. **No post-release measurement loop.** No catalogued "ship → measure → diagnose → experiment → feed-back-into-plan" loop. Build completion is tracked; value realisation is not.
2. **Ideation under-shoots learnability.** `/slo-ideate` Q3 pushes the "smallest wedge / one-week ship", which can produce a slice too thin to teach the team why it worked or failed.
3. **Measurement is not contracted.** `/slo-plan` and the v4 template have no mandatory measurement contract; telemetry is best-effort, so issues are discovered too late (analysis latency).
4. **No telemetry verification.** `/slo-verify` cannot confirm events fire, telemetry fields are masked/pseudonymised, or failure paths emit signals.
5. **Retros don't ask "did it work?"** `/slo-retro` records code changes, not whether the planned leading/lagging indicators moved.

### Target Architecture

See §3. Five additive disciplines threaded across existing skills; one optional frontmatter key; new loop documented. No new skill, no merged skill, no vendored SDK.

### Key Design Principles

1. **Enrichment-only / inline-first.** Additive Markdown sections + one optional key; defer the machine-readable telemetry `.slo.json` schema to a future architect pass (no fixtures yet).
2. **Stable interfaces preserved.** No command-verb or output-path changes; `/slo-product` vs `/slo-metrics` split kept.
3. **Backward compatible by absence.** Legacy idea docs / runbooks / artifacts without the new sections/key remain valid; `/slo-plan` flags gaps, never invalidates the past.
4. **Heuristic verification, like Pass 4.** The measurement pass is presence/pattern based, not schema-parsing.
5. **Privacy stricter by default.** Pseudonymise + mask + minimise + consent + DPIA trigger are part of the contract (GDPR/PECR-aware).
6. **Every contract change is test-gated.** A sibling structural-contract test fails pre-fix and passes post-fix; other skills' baselines stay green.

### What to Keep

- `/slo-product` and `/slo-metrics` command surfaces, output paths, and split.
- All existing v4 template sections, idea-doc sections, verify passes, retro sections.
- All existing `xtasks/sast-verify` baselines (esp. `sap_imp_m5_agents.rs` SHA baseline).
- `discover_skills()` behaviour and `references/biz/` exclusion from discovery.

### What to Change

- **`skills/slo-ideate/SKILL.md`** — Q3 reframing + `## Success thesis` output section (M1).
- **`skills/slo-product/SKILL.md`** — feature measurement spec in `mode_arg: metrics`; set the new flag (M2).
- **`references/biz/artifact-schema.md`** — register `feature_measurement_spec: bool` (M2).
- **`docs/slo/templates/runbook-template_v_4_template.md`** — Measurement Contract section + Contract Block row (M3).
- **`skills/slo-plan/SKILL.md`** — require the Measurement Contract for value-bearing features (M3).
- **`skills/slo-verify/SKILL.md`** — measurement pass (M4).
- **`skills/slo-retro/SKILL.md`** — Results-vs-thesis lessons section (M4).
- **`docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md`** — feature-performance loop + cross-ref (M5).
- **`xtasks/sast-verify/tests/mloop_m*.rs`** — one structural test per milestone; SHA-baseline updates where a SKILL.md changes (each M).

### Global Red Lines

The v4 template's Global Red Lines (§9) apply, plus:
- No new skill; no merge of `/slo-product` and `/slo-metrics`; no vendored analytics SDK.
- No machine-readable telemetry schema / `.slo.json` companion in this runbook (deferred).
- No more than one new frontmatter key (`feature_measurement_spec`); no extension of the `tier` enum.
- No renumber of v4 template sections or `/slo-verify` passes; additions are insertions/sub-passes.
- No structural-test baseline bypass; baseline edits are explicit, minimal, and recorded in the Evidence Log.

---

## 10. Carry-forward from prior retros

> Optional section. No `mloop`-prefixed retro-derived issues exist yet (this is the first runbook for this prefix). `/slo-execute` Step 1.5 falls back to `gh issue list --label retro-derived`.

| Issue | Title | Suggested lane | Suggested milestone | Status |
|---|---|---|---|---|
| (none yet) | | | | |

---

## 11–16. BDD Rules / Dependency Policy / Evidence Log / Self-Review / Lessons / Completion Templates

Apply §11–§16 of [the v4 template](slo/templates/runbook-template_v_4_template.md). Project bindings: "tests" are Rust structural-contract tests in `xtasks/sast-verify/tests/`; the only test-artifact discipline needed is that fixtures under `xtasks/sast-verify/tests/fixtures/` are committed intentionally and temp scratch is cleaned. Lessons and completion files go in `docs/slo/lessons/mloop-m<N>.md` and `docs/slo/completion/mloop-m<N>.md`.

---

## 17. Milestone Plan

### Milestone 1 — `/slo-ideate` philosophy shift + Success thesis section

**Goal**: `/slo-ideate` stops forcing only the "smallest wedge" and asks for the **smallest *complete* value slice**, and every idea doc gains a `## Success thesis` section (leading metric, lagging metric, top guardrails, review window, and the "is the problem technical / pricing / UX / demand" diagnostic question) — so a measurement intent exists from the very first artifact.

**Context**: `skills/slo-ideate/SKILL.md` Q3 currently reads "What is the smallest wedge that would be obviously better within one week? … keep cutting." The idea-doc output template (the `---` frontmatter + `## The pain` … `## Recommendation` … `## Open questions for /slo-research` block) has no measurement section. We reframe Q3 and insert one section; we do **not** change the question-slot count or rename existing sections.

**Carmack-style reliability goal**: Make invalid states unrepresentable — a value-bearing idea doc that carries no success thesis is the invalid state we eliminate at the source of the loop.

**Important design rule**: Preserve the one-week-shippability discipline's *spirit* (a slice, not a fat MVP) while requiring the slice be "complete enough for a user to experience the core value AND for the team to learn why it worked or failed." The success thesis is three metrics + guardrails, not a metrics essay.

**Refactor budget**: `Minimal local refactor permitted in listed files only` (reword Q3; insert one output section; the structural test is new).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A founder's rough idea (existing `/slo-ideate` invocation) |
| Outputs | Idea doc with reframed Q3 prompt + new `## Success thesis` section |
| Interfaces touched | `/slo-ideate` SKILL.md prose + idea-doc output section contract |
| Files allowed to change | `skills/slo-ideate/SKILL.md`; `xtasks/sast-verify/tests/mloop_m1_ideate.rs` (NEW) |
| Files to read before changing anything | `skills/slo-ideate/SKILL.md`; `docs/slo/design/measurement-loop-slo-improvements-interfaces.md`; `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` (test pattern exemplar) |
| New files allowed | `xtasks/sast-verify/tests/mloop_m1_ideate.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Question-slot count unchanged; existing idea-doc section names (`## The pain`, `## Top risks`, approach blocks, `## Recommendation`, `## Open questions for /slo-research`) unchanged; legacy idea docs without a success thesis remain valid |
| Resource bounds introduced/changed | Success thesis bounded to exactly 3 metric lines (1 leading, 1 lagging) + a short guardrails list + 1 review window; no unbounded metric list |
| Invariants/assertions required | Structural test asserts the reframed Q3 phrase and the `## Success thesis` section sentinel are present as exact contiguous substrings; asserts existing section names still present (no rename) |
| Debugger / inspection expectation | On test failure, run `cargo test -p sast-verify -- --nocapture` and read the actual SKILL.md slice before re-editing |
| Static analysis gates | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p sast-verify` |
| Exemplar code to copy | `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` (frontmatter + exact-substring sentinel assertions); idea-doc section style in current `skills/slo-ideate/SKILL.md` |
| Anti-exemplar code not to copy | A `contains()` assertion satisfied by interleaved prose (kani-m1 lesson) — assert the full contiguous phrase. Do not copy v3-era one-week-only wedge wording into the new section. |
| Refactoring discipline | Cite `skills/slo-plan/references/refactoring-discipline.md`: reword Q3 as a behavior-preserving microstep (question slot stays), prove the existing-section sentinels still pass after the edit. |
| AI tolerance contract | `N/A — no AI component` (editing a static Markdown contract; no new model invocation or eval) |
| Data classification | `Public` (the skill pack is public; idea-doc *content* may be Confidential but that is governed downstream, not by this edit) |
| Proactive controls in play | OWASP C8 (data protection) — the success-thesis template names *behaviour* metrics, not raw PII, and a comment routes real-user-quote risk to the `/slo-verify` PII scan |
| Abuse acceptance scenarios | `N/A — no new runtime surface introduced (Markdown contract edit). The PII-paste risk for idea-doc content is tm-measurement-loop-abuse-2, addressed by the M4 measurement pass + existing Pass 4 scan; the success-thesis template carries a one-line "name behaviour, not PII" note.` |
| Forbidden shortcuts | No renaming/removing existing idea-doc sections; no extra forcing questions beyond the reframe; no TODO/placeholder in the new section; no metrics-essay (3-line bound) |

#### Out of Scope / Must Not Do

- Do not touch `/slo-product`, `/slo-plan`, `/slo-verify`, `/slo-retro` (later milestones).
- Do not add the `feature_measurement_spec` key here (M2).
- Do not change `/slo-ideate` frontmatter keys.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-ideate/SKILL.md` | Reframe Q3 to "smallest *complete* value slice"; add diagnostic question; insert `## Success thesis` output section (leading metric, lagging metric, guardrails, review window, "technical/pricing/UX/demand?" question) |
| `xtasks/sast-verify/tests/mloop_m1_ideate.rs` | NEW: assert reframed Q3 + `## Success thesis` sentinels present; assert existing section names unchanged |

#### Step-by-Step

1. Write `mloop_m1_ideate.rs` first; run it; confirm it fails for the right reason (sentinels absent).
2. Read current `skills/slo-ideate/SKILL.md` Q3 and the idea-doc output block.
3. Reword Q3 to the "smallest complete value slice" framing (preserve slot, add the diagnostic prompt).
4. Insert the `## Success thesis` section into the idea-doc output template with the 3-metric + guardrail + window + diagnostic shape and the "name behaviour, not PII" note.
5. Re-run `mloop_m1_ideate.rs`; confirm green.
6. Run `cargo test -p sast-verify` (all baselines green), `cargo fmt --check`, `cargo clippy -D warnings`.
7. If any existing baseline trips (it should not — no other SKILL.md changed), stop and inspect.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: success-thesis-bearing idea docs**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Reframed Q3 present | happy path | Edited SKILL.md | `mloop_m1_ideate.rs` runs | Asserts the exact "smallest complete value slice" phrase present |
| Success thesis section present | happy path | Edited SKILL.md | test runs | Asserts `## Success thesis` + leading/lagging/guardrail sentinels present |
| Existing sections preserved | backward compat | Edited SKILL.md | test runs | Asserts `## The pain`, `## Top risks`, `## Recommendation`, `## Open questions for /slo-research` still present |
| Pre-fix fails | assertion violation | Unedited SKILL.md | test runs | Test FAILS (anti-vacuity: the guardrail proves it can catch absence) |
| Question slot count unchanged | invalid input | Edited SKILL.md | test counts forcing questions | Slot count equals the pre-edit count (no silent extra question) |

#### Regression Tests

- `cargo test -p sast-verify` — every existing structural baseline (kani_*, sap_imp_*, slo_tm_*, slo_tmp_*) stays green.
- `./target/release/sldo-install --dry-run` — `/slo-ideate` still discovered.

#### Compatibility Checklist

- [ ] `/slo-ideate` forcing-question slot count unchanged
- [ ] Existing idea-doc section names unchanged
- [ ] Legacy idea docs (no success thesis) still valid (section is additive)
- [ ] All other skills' structural baselines pass

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/mloop_m1_ideate.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_ideate_complete_value_slice_present` | Q3 reframed | exact phrase asserted |
| `slo_ideate_success_thesis_section_present` | output section added | `## Success thesis` + 3-metric sentinels asserted |
| `slo_ideate_existing_sections_preserved` | no rename/removal | existing section sentinels asserted |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes
- [ ] `./target/release/sldo-install --dry-run` lists `/slo-ideate`
- [ ] Static analysis passes
- [ ] `git status` shows no untracked test artifacts

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 4 passed (slo_tmp_m2) + full suite green | Pass | clean baseline before edits |
| BDD test created | `mloop_m1_ideate.rs` | fails for expected reason | 3 new-content tests FAILED (sentinels absent), 2 invariant tests passed | Pass | correct anti-vacuity: failures were "smallest complete value slice"/`## Success thesis`/behaviour-not-PII absent |
| Implementation | reframe Q3 + add `## Success thesis` + stop-condition | contract satisfied | Q3 reframed; section added after `## Recommendation`; stop bullet added | Pass | slot count kept at 7 (diagnostics folded into Q3) |
| Formatter | `cargo fmt --all -- --check` | clean | clean after `cargo fmt --all` | Pass | rustfmt reflowed the test's array literal |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | **pre-existing red** in `tests/sap_imp_m3_standards.rs` (regex-in-loop), `src/tier_detect.rs` + `src/yaml_schema.rs` (dead-code) — all OUT of M1 allow-list and unmodified by me | Documented exception | `mloop_m1_ideate.rs` is clippy-clean; pre-existing failures filed as follow-up (see Notes / lessons) |
| Full tests | `cargo test -p sast-verify` | green | full suite green incl. `mloop_m1_ideate` 5/5 | Pass | no baseline regressions |
| Skill discovery | `cargo run -q -p sldo-install -- --dry-run` | `/slo-ideate` listed | `= slo-ideate` listed (frontmatter untouched) | Pass | debug dry-run substituted for release build |
| Compatibility checks | existing section sentinels | no regressions | `## The pain`/`## Top risks`/`## Recommendation`/`## Open questions` all asserted present | Pass | `slo_ideate_existing_sections_preserved` green |
| Test artifact cleanup | `git status` | clean | only `M skills/slo-ideate/SKILL.md` + `?? mloop_m1_ideate.rs` | Pass | no scratch artifacts |

#### Definition of Done

- reframed Q3 + `## Success thesis` section present and asserted
- existing idea-doc sections unchanged and asserted
- `mloop_m1_ideate.rs` fails pre-fix, passes post-fix
- all existing baselines + fmt + clippy green
- lessons + completion files written; tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: none (design doc already captures it)
- **README.md**: none
- **Other docs**: note the new idea-doc section in `skills/slo-ideate/SKILL.md` only

---

### Milestone 2 — `/slo-product metrics` feature measurement spec + `feature_measurement_spec` key

**Goal**: `/slo-product metrics` gains an optional `## Feature measurement specification` section (north-star link, primary leading + lagging metric, guardrails, activation/completion funnel, adoption thresholds, diagnostic questions, segmentation, experiment backlog, telemetry requirements), and sets the single new optional frontmatter key `feature_measurement_spec: true` when the section is present — while the `/slo-product` vs `/slo-metrics` split stays intact.

**Context**: `skills/slo-product/SKILL.md` `mode_arg: metrics` outputs `docs/biz-public/product/metrics.md` (north-star, activation funnel, retention curves, feature-adoption rubric, cross-ref to `/slo-metrics`). `references/biz/artifact-schema.md` is the stable frontmatter contract whose own rule says "adding new keys is a `/slo-architect` decision" — the [interfaces design doc](slo/design/measurement-loop-slo-improvements-interfaces.md) is that decision and authorizes exactly one key.

**Carmack-style reliability goal**: Bounded resources + make-invalid-states-unrepresentable — one spec per feature, one bool key (absent==false), no key cluster, no financial KPIs absorbed.

**Important design rule**: Keep the split: this section carries PM-side feature value only; financial mix (CAC/LTV/NDR/burn) stays in `/slo-metrics`. The cross-reference must be strengthened, not weakened.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A founder running `/slo-product metrics`, optionally for a specific feature |
| Outputs | `docs/biz-public/product/metrics.md` with optional feature measurement spec section + `feature_measurement_spec` frontmatter key |
| Interfaces touched | `/slo-product` SKILL.md `mode_arg: metrics` body + idea-doc→product handoff; `references/biz/artifact-schema.md` frontmatter contract |
| Files allowed to change | `skills/slo-product/SKILL.md`; `references/biz/artifact-schema.md`; `xtasks/sast-verify/tests/mloop_m2_product.rs` (NEW) |
| Files to read before changing anything | `skills/slo-product/SKILL.md`; `references/biz/artifact-schema.md`; `docs/slo/design/biz-skill-pack-interfaces.md`; `docs/slo/design/measurement-loop-slo-improvements-interfaces.md` |
| New files allowed | `xtasks/sast-verify/tests/mloop_m2_product.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` (key is additive, absent==false; no rewrite of existing artifacts) |
| Compatibility commitments | `/slo-product` and `/slo-metrics` verbs/paths/`mode_arg` unchanged; PM/financial split preserved and cross-ref strengthened; existing artifact-schema keys unchanged; legacy `metrics.md` without the key/section remains valid |
| Resource bounds introduced/changed | Exactly one new optional key; one feature-spec section template; no cluster; no enum extension |
| Invariants/assertions required | Structural test asserts: the `## Feature measurement specification` sentinel present in SKILL.md; `feature_measurement_spec` registered in artifact-schema.md with type `bool` + "default-absent = false" + "optional"; the financial-KPI cross-ref sentinel still present; the `tier` enum still exactly `confidential\|public` |
| Debugger / inspection expectation | `cargo test -p sast-verify -- --nocapture` on failure |
| Static analysis gates | `cargo fmt --check`; `cargo clippy -D warnings`; `cargo test -p sast-verify` |
| Exemplar code to copy | `pii_scan_override` / `tier_override_reason` rows in `references/biz/artifact-schema.md` (optional, paired, read-by-/slo-verify style); `kani_m1_skill_contract.rs` for frontmatter-key assertion |
| Anti-exemplar code not to copy | Adding a cluster of telemetry keys; extending the `tier` enum; absorbing financial KPIs into `/slo-product` |
| Refactoring discipline | Cite `skills/slo-plan/references/refactoring-discipline.md`: schema-row addition + section insertion as microsteps; prove existing key rows + cross-ref sentinel unchanged after. |
| AI tolerance contract | `N/A — no AI component` |
| Data classification | `Public` (`/slo-product metrics` output is `tier: public`) |
| Proactive controls in play | OWASP C8 (data protection) — the telemetry-requirements sub-block mandates pseudonymised identifiers + masking by default; C5 (input handling) — feature names rendered as plain text |
| Abuse acceptance scenarios | `N/A — no new runtime surface introduced. The "set the flag without a spec" gaming risk is tm-measurement-loop-abuse-3, caught by the M4 measurement pass (flag↔section cross-check); the PII-in-public-artifact risk is tm-measurement-loop-abuse-2, caught by Pass 4 + M4.` |
| Forbidden shortcuts | No second new key; no `tier` enum change; no command/path rename; no financial KPI absorption; no TODO/placeholder |

#### Out of Scope / Must Not Do

- Do not edit the v4 template, `/slo-plan`, `/slo-verify`, `/slo-retro`.
- Do not implement the flag↔section cross-check enforcement (that is M4's measurement pass).
- Do not add a `.slo.json` companion (deferred).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-product/SKILL.md` | Add optional `## Feature measurement specification` section to `mode_arg: metrics`; set `feature_measurement_spec: true` when present; strengthen `/slo-metrics` cross-ref |
| `references/biz/artifact-schema.md` | Register one optional key `feature_measurement_spec: bool` (default-absent=false; read by `/slo-verify` measurement pass) per the "adding a new key is an `/slo-architect` decision" rule |
| `xtasks/sast-verify/tests/mloop_m2_product.rs` | NEW: assert section sentinel + key registration + split preserved + tier enum intact |

#### Step-by-Step

1. Write `mloop_m2_product.rs` first; run; confirm fail.
2. Read `slo-product/SKILL.md` `mode_arg: metrics` body + `artifact-schema.md` key table.
3. Add the feature-spec section template and the conditional `feature_measurement_spec: true` rule to SKILL.md; strengthen the cross-ref line.
4. Add the `feature_measurement_spec` row to `artifact-schema.md` (type bool, optional, default-absent=false, read-by note).
5. Re-run `mloop_m2_product.rs`; confirm green.
6. Run full `cargo test -p sast-verify`, fmt, clippy.
7. Verify the `tier` enum row is byte-unchanged.
8. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: feature measurement spec + schema key**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Feature-spec section present | happy path | Edited SKILL.md | test runs | `## Feature measurement specification` + sub-field sentinels asserted |
| Key registered, optional, bool | happy path | Edited artifact-schema | test runs | `feature_measurement_spec` row asserted with `bool` + optional + default-false |
| Split preserved | backward compat | Edited SKILL.md | test runs | financial-KPI cross-ref sentinel still present; no CAC/LTV definitions added to `/slo-product` |
| Tier enum intact | invalid input | Edited artifact-schema | test runs | `tier` enum still exactly `confidential \| public` |
| Pre-fix fails | assertion violation | Unedited files | test runs | test FAILS (anti-vacuity) |
| Single-key bound | resource bound | Edited artifact-schema | test runs | asserts `feature_measurement_spec` present AND a forbidden-telemetry-key denylist (`telemetry_schema`, `event_names`, `measurement_spec_json`) is absent — proves no key cluster crept in |

#### Regression Tests

- `cargo test -p sast-verify` — all baselines green (esp. any biz-pack schema test).
- `./target/release/sldo-install --dry-run` — `/slo-product` discovered.

#### Compatibility Checklist

- [ ] `/slo-product` / `/slo-metrics` verbs, `mode_arg`, output paths unchanged
- [ ] PM/financial split preserved; cross-ref present
- [ ] Existing artifact-schema keys unchanged; `tier` enum unchanged
- [ ] Legacy `metrics.md` without key/section still valid

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/mloop_m2_product.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_product_feature_spec_section_present` | section added | sentinel asserted |
| `slo_product_measurement_key_registered` | one optional bool key | row asserted with type+optional+default |
| `slo_product_split_preserved` | split intact | cross-ref sentinel asserted; no financial KPI absorbed |
| `artifact_schema_tier_enum_unchanged` | no enum drift | `confidential \| public` asserted |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes
- [ ] `./target/release/sldo-install --dry-run` lists `/slo-product`
- [ ] Static analysis passes
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | full suite green pre-edit | Pass | |
| BDD test created | `mloop_m2_product.rs` | fails for expected reason | 4 new-content tests FAILED (section/key/pseudonym/single-key absent), 2 (split/tier) passed | Pass | correct anti-vacuity |
| Implementation | add §6 spec + flag + schema key | contract satisfied | §6 added; `feature_measurement_spec: true` flag rule + frontmatter line; one schema row added | Pass | cross-ref + tier enum untouched |
| Formatter | `cargo fmt --all -- --check` | clean | clean | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | same pre-existing red as M1 (3 out-of-scope files); `mloop_m2_product.rs` clean | Documented exception | see mloop-m1 lessons |
| Full tests | `cargo test -p sast-verify` | green | 17 test binaries green incl. `mloop_m2_product` 6/6 | Pass | |
| Split / enum check | sentinels | unchanged | CAC/LTV/NDR→/slo-metrics cross-ref present; `tier` enum still `confidential \| public` | Pass | `slo_product_split_preserved` + `artifact_schema_tier_enum_unchanged` green |
| Skill discovery | `cargo run -q -p sldo-install -- --dry-run` | `/slo-product` listed | `= slo-product` listed | Pass | frontmatter additive only |
| Test artifact cleanup | `git status` | clean | only `M artifact-schema.md`, `M slo-product/SKILL.md`, `?? mloop_m2_product.rs` | Pass | |

#### Definition of Done

- feature-spec section + one optional bool key present and asserted
- split preserved; tier enum unchanged; both asserted
- `mloop_m2_product.rs` fails pre-fix, passes post-fix
- baselines + fmt + clippy green
- lessons + completion written; tracker updated

#### Post-Flight

- **Other docs**: optionally note the additive feature-spec section in `docs/slo/design/biz-skill-pack-interfaces.md` (additive note only; do not change its stable rows).

---

### Milestone 3 — v4 template Measurement Contract section + Contract Block row + `/slo-plan` requirement

**Goal**: The v4 runbook template gains an optional, additive **Measurement Contract** section (value hypothesis, review windows 24h/7d/28d, primary leading metric, primary lagging metric, guardrails, telemetry deliverables, rollout plan, diagnosis plan, experiment plan, privacy controls) plus a per-milestone Contract Block row **Measurement deliverables**, and `/slo-plan` requires both to be filled for any value-bearing (user-facing-value) feature — flagging the gap rather than invalidating legacy runbooks.

**Context**: `docs/slo/templates/runbook-template_v_4_template.md` is the output contract of `/slo-plan`. §10 "Carry-forward from prior retros" is the precedent for an optional, additive, fallback-on-absence section. `skills/slo-plan/SKILL.md` lists Contract Block Sentinels; the Measurement Contract requirement attaches there. We must not renumber existing template sections.

**Carmack-style reliability goal**: Compatibility + assertions-as-contracts — telemetry becomes a contracted deliverable; the structural test asserts the section/row exist and that no existing section was renumbered.

**Important design rule**: The Measurement Contract is **optional by template shape but required by `/slo-plan` for value-bearing features** — exactly the "Carry-forward" pattern (legacy runbooks valid; the driver flags absence). Insert as a new section (e.g., §5A "Measurement Contract") without renumbering §6–§20, mirroring how §10 carry-forward was inserted; the structural test pins both the new section and the unchanged neighbours.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A founder running `/slo-plan` on a feature with a success thesis (M1) and optionally a feature spec (M2) |
| Outputs | A runbook with a Measurement Contract section + per-milestone Measurement-deliverables row; `/slo-plan` flags the gap when missing for a value-bearing feature |
| Interfaces touched | v4 template section list + Contract Block row set; `/slo-plan` authoring contract |
| Files allowed to change | `docs/slo/templates/runbook-template_v_4_template.md`; **`skills/slo-plan/references/runbook-template_v_4_template.md`** (allow-list EXTENDED 2026-05-22, user-approved: this is the skill-PRIMARY copy that `/slo-plan` reads first; the repo path is its byte-identical mirror — both must carry the identical edit or `/slo-plan` authors from a stale template); `skills/slo-plan/SKILL.md`; `xtasks/sast-verify/tests/mloop_m3_plan.rs` (NEW) |
| Files to read before changing anything | the v4 template (esp. §5, §10, §17 Contract Block); `skills/slo-plan/SKILL.md`; `docs/slo/design/measurement-loop-slo-improvements-interfaces.md` |
| New files allowed | `xtasks/sast-verify/tests/mloop_m3_plan.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` (section + row additive; legacy runbooks valid by absence) |
| Compatibility commitments | Existing template sections §1–§20 not renumbered/removed; legacy runbooks without the section/row remain valid; existing Contract Block rows unchanged; `/slo-plan` 5-milestone cap + existing gates unchanged |
| Resource bounds introduced/changed | One new template section (10 documented fields) + one Contract Block row; review windows bounded to a small set (24h/7d/28d or equivalent); no unbounded metric list |
| Invariants/assertions required | Structural test asserts: the Measurement Contract section heading + its 10 field sentinels present in the template; the `Measurement deliverables` Contract Block row present; `/slo-plan` SKILL.md carries the "required for value-bearing features" sentinel + the **value-bearing definition** sentinel (ENG-1) + the "flag (don't invalidate legacy)" sentinel; existing §6/§10/§17 headings still present (no renumber) |
| Debugger / inspection expectation | `cargo test -p sast-verify -- --nocapture` on failure; diff the template to confirm no renumber |
| Static analysis gates | `cargo fmt --check`; `cargo clippy -D warnings`; `cargo test -p sast-verify` |
| Exemplar code to copy | v4 template §10 "Carry-forward" optional-section framing; the report's 10-field Measurement Contract table; `kani_m3_integration.rs` (template-section assertion pattern) |
| Anti-exemplar code not to copy | Renumbering existing sections; making the section mandatory at the template level (breaks legacy); a `contains()` that interleaved prose can satisfy |
| Refactoring discipline | Cite `skills/slo-plan/references/refactoring-discipline.md`: section insertion + row addition as microsteps; prove neighbouring section headings unchanged after. |
| AI tolerance contract | `N/A — no AI component` |
| Data classification | `Public` (template + skill are public; the *contract content* a runbook carries may be Confidential, governed downstream) |
| Proactive controls in play | OWASP C8 (data protection) — the Privacy controls field mandates pseudonymise/mask/minimise/consent/DPIA-trigger; C10 (error/logging) — telemetry-deliverables includes failure-path emission |
| Abuse acceptance scenarios | `N/A — no new runtime surface introduced. The "thin/dishonest contract" residual is R1 + tm-measurement-loop-abuse-3, mitigated by the M4 verify pass + /slo-retro refusal on blank actuals; the template's Privacy controls field is the class-elimination of tm-measurement-loop-abuse-2 at plan time.` |
| Forbidden shortcuts | No renumber of existing sections; no mandatory-at-template-level section; no removal/rename of existing Contract Block rows; no TODO/placeholder; no skipping the "flag, don't invalidate" sentinel |

#### Out of Scope / Must Not Do

- Do not edit `/slo-verify` or `/slo-retro` (M4).
- Do not edit the v3 template.
- Do not implement runtime enforcement of telemetry firing (that is M4's pass).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/slo/templates/runbook-template_v_4_template.md` | Insert optional **Measurement Contract** section (10 fields) without renumbering §6–§20; add `Measurement deliverables` row to the §17 Contract Block template |
| `skills/slo-plan/references/runbook-template_v_4_template.md` | Apply the IDENTICAL edit (skill-primary mirror; allow-list extension above) so the two copies stay byte-identical |
| `skills/slo-plan/SKILL.md` | Add Contract Block Sentinel: Measurement Contract required for value-bearing features; **define "value-bearing"** crisply (= introduces or changes user-facing capability; EXCLUDES internal refactor, docs-only, test-only) so the trigger is deterministic; flag the gap (don't invalidate legacy); cite the success thesis (M1) + feature spec (M2) as inputs |
| `xtasks/sast-verify/tests/mloop_m3_plan.rs` | NEW: assert template section + 10 fields + Contract Block row + plan sentinels + **value-bearing definition sentinel** + no-renumber |

#### Step-by-Step

1. Write `mloop_m3_plan.rs` first; run; confirm fail.
2. Read v4 template §5/§10/§17 and `slo-plan/SKILL.md` sentinels.
3. Insert the Measurement Contract section (mirror §10's optional framing) + the Contract Block row, without renumbering.
4. Add the `/slo-plan` requirement sentence (required for value-bearing — with the deterministic definition: user-facing capability, excludes refactor/docs-only/test-only; flag-don't-invalidate; cite M1/M2 inputs).
5. Re-run `mloop_m3_plan.rs`; confirm green.
6. Full `cargo test -p sast-verify`, fmt, clippy.
7. Diff the template to confirm §6–§20 headings unchanged.
8. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: contracted measurement in planning**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Section present with 10 fields | happy path | Edited template | test runs | Measurement Contract heading + 10 field sentinels asserted |
| Contract Block row present | happy path | Edited template | test runs | `Measurement deliverables` row sentinel asserted |
| Plan requirement present | happy path | Edited slo-plan | test runs | "required for value-bearing" + "flag … legacy" sentinels asserted |
| Value-bearing defined | invalid input | Edited slo-plan | test runs | the deterministic definition (user-facing capability; excludes refactor/docs-only/test-only) asserted as a contiguous sentinel — ENG-1 |
| No renumber | backward compat | Edited template | test runs | §6/§10/§17 headings still present and in order |
| Optional by shape | backward compat | Edited template | test runs | "optional … legacy runbooks remain valid" framing sentinel present |
| Pre-fix fails | assertion violation | Unedited files | test runs | test FAILS (anti-vacuity) |

#### Regression Tests

- `cargo test -p sast-verify` — all baselines green (esp. `kani_m3_integration.rs` which also asserts template §5 shape; confirm no collision).
- `./target/release/sldo-install --dry-run` — `/slo-plan` discovered.

#### Compatibility Checklist

- [ ] v4 template §1–§20 not renumbered/removed
- [ ] `kani_m3_integration.rs` (asserts template §5 / §5.8 shape) still green after the new section is inserted near §5 — hard check, not just "confirm no collision"
- [ ] Existing Contract Block rows unchanged
- [ ] Legacy runbooks without the section/row still valid
- [ ] `/slo-plan` 5-milestone cap + existing gates unchanged

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/mloop_m3_plan.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `v4_measurement_contract_section_present` | section + 10 fields added | sentinels asserted |
| `v4_measurement_deliverables_row_present` | Contract Block row added | sentinel asserted |
| `slo_plan_requires_measurement_contract` | plan requirement wired | "required for value-bearing" + "flag legacy" asserted |
| `slo_plan_defines_value_bearing` | deterministic trigger | value-bearing definition sentinel asserted (ENG-1) |
| `v4_existing_sections_not_renumbered` | no renumber | §6/§10/§17 headings asserted |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes
- [ ] `./target/release/sldo-install --dry-run` lists `/slo-plan`
- [ ] Static analysis passes
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | full suite green pre-edit | Pass | |
| BDD test created | `mloop_m3_plan.rs` | fails for expected reason | 3 new-content tests FAILED (section/plan-requirement/value-bearing absent); byte-identity + no-renumber + kani-subblock passed | Pass | anti-vacuity correct |
| Implementation | §5A section + §17 row + plan sentinel + value-bearing def | contract satisfied | §5A added (10 fields, optional/legacy framing); `Measurement deliverables` row; `/slo-plan` Measurement Contract requirement + deterministic value-bearing def | Pass | inserted between §5 and §6, no renumber |
| Mirror sync | `diff` primary vs repo template | byte-identical | edited repo mirror then `cp` → skill-primary; `diff` IDENTICAL | Pass | allow-list extended (user-approved) to skill-primary copy |
| Formatter | `cargo fmt --all -- --check` | clean | clean | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | same pre-existing out-of-scope red; `mloop_m3_plan.rs` clean | Documented exception | see mloop-m1 lessons |
| Full tests | `cargo test -p sast-verify` | green | 18 test binaries green incl. `mloop_m3_plan` 6/6 | Pass | |
| No-renumber check | §6/§10/§17 headings | unchanged | all three headings asserted present | Pass | `template_existing_sections_not_renumbered` green |
| kani_m3 regression | `cargo test -p sast-verify --test kani_m3_integration` | stays green | 6/6 green; §5 Kani sub-block survived (`template_section5_kani_subblock_preserved`) | Pass | the M3 hard compatibility check |
| Skill discovery | `cargo run -q -p sldo-install -- --dry-run` | `/slo-plan` listed | listed | Pass | |
| Test artifact cleanup | `git status` | clean | only the 4 intended edits + new test | Pass | |

#### Definition of Done

- Measurement Contract section (10 fields) + Contract Block row present and asserted
- `/slo-plan` requirement + value-bearing definition (ENG-1) + flag-don't-invalidate sentinels present and asserted
- existing template sections not renumbered (asserted)
- `mloop_m3_plan.rs` fails pre-fix, passes post-fix
- baselines + fmt + clippy green
- lessons + completion written; tracker updated

#### Post-Flight

- **Other docs**: none beyond the template + `/slo-plan` SKILL.md.

---

### Milestone 4 — `/slo-verify` measurement pass + `/slo-retro` Results-vs-thesis + **failure-bar demonstration**

**Goal**: `/slo-verify` gains a measurement pass — (a) event/schema **presence** smoke check, (b) telemetry **PII/masking** check, (c) **failure-path emission** check, (d) replay-tagging check where enabled, (e) `feature_measurement_spec` **flag↔section cross-check**, and (f) **unfenced user-string / template-injection** check over the new generated sections — added as an additive sub-pass without renumbering Pass 1–5; `/slo-retro` gains a `## Results vs thesis` lessons section; and the **failure bar is proven end-to-end**: a fixture runbook/artifact with a missing event, an unmasked-PII telemetry field, a gamed flag, AND an injection payload in a thesis/feature-name field is **caught** by the pass, **remediated**, and **re-verified green**.

**Context**: `skills/slo-verify/SKILL.md` runs Pass 1–5; Pass 4 already scans `docs/biz-public/` for PII patterns (email/UK-NI/sort-code/capitalised-bigram) — the measurement pass reuses that heuristic style (presence/pattern, not schema-parsing). `skills/slo-retro/SKILL.md` lessons template has no results section. The failure-bar demo mirrors `docs/RUNBOOK-kani-verification.md` M4 (catch→remediate→green), but mechanically gated via test-local checks over committed fixtures.

**Carmack-style reliability goal**: No silent failure + assertions — the pass makes "telemetry not firing / PII unmasked / flag gamed" a *visible, caught* failure rather than a post-launch surprise; the fixture pair proves the guardrail is non-vacuous (the bad fixture must fail, the good one must pass).

**Important design rule**: The pass is heuristic and tool-optional like Pass 4 (missing context → `skipped` row, not a hard fail), EXCEPT the flag↔section cross-check and the synthetic-PII-in-telemetry-field check are hard signals demonstrated by the fixture pair. The failure-bar test helpers live in the test file (test-only); no new production binary or dependency.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A completed milestone with a Measurement Contract (M3) + emitted telemetry evidence; for the demo, a committed fixture pair (bad / remediated) |
| Outputs | `/slo-verify` measurement sub-pass; `/slo-retro` Results-vs-thesis section; a passing failure-bar demonstration over fixtures |
| Interfaces touched | `/slo-verify` pass list (additive sub-pass); `/slo-retro` lessons template; `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/` |
| Files allowed to change | `skills/slo-verify/SKILL.md`; `skills/slo-retro/SKILL.md`; `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs` (NEW); `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/` (NEW fixtures) |
| Files to read before changing anything | `skills/slo-verify/SKILL.md` (Pass 4 PII-scan section); `skills/slo-retro/SKILL.md` (lessons template); `docs/slo/design/measurement-loop-slo-improvements-threat-model.md`; `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` |
| New files allowed | `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs`; `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/{bad,remediated}.md` |
| New dependencies allowed | `none` (reuse existing test-crate deps; pattern checks via std/existing regex if already a dep, else simple substring/byte checks) |
| Migration allowed | `no` |
| Compatibility commitments | Pass 1–5 numbering unchanged (measurement is a sub-pass); Pass 4 PII scan behaviour unchanged; `/slo-retro` existing sections + refusal-on-blank-actuals rule unchanged; legacy runbooks without telemetry → pass emits `skipped`, not fail |
| Resource bounds introduced/changed | Pass scans a bounded file set (runbook + named artifacts); five named checks, no open-ended scan; fixtures are small (<100 lines each) |
| Invariants/assertions required | SKILL.md sentinels for all **six** checks (a–f incl. the injection/fence check) + tool-optional `skipped`-row rule; **prose↔mechanical lockstep (ENG-2)**: the SKILL.md prose must *name* the same checks the test mechanizes, asserted as contiguous substrings; `/slo-retro` `## Results vs thesis` sentinel; **synthetic-PII marker (SEC-2)**: both fixtures carry a `SYNTHETIC PII` header comment, asserted; **failure-bar invariant**: the bad fixture FAILS the mechanical checks (catch), the remediated fixture PASSES (green) — both asserted in one test |
| Debugger / inspection expectation | `cargo test -p sast-verify -- --nocapture` shows which check fired on which fixture line |
| Static analysis gates | `cargo fmt --check`; `cargo clippy -D warnings`; `cargo test -p sast-verify` |
| Exemplar code to copy | Pass 4 PII-pattern scan section in `skills/slo-verify/SKILL.md` (heuristic, tool-optional, finding-vs-skip); `kani_m4` failure-bar demo shape; `tests/fixtures/clean_subset/` for fixture layout |
| Anti-exemplar code not to copy | Renumbering Pass 1–5; making the pass a hard fail when telemetry context is absent (use `skipped`); a vacuous failure-bar test where the bad fixture also passes; **real PII in fixtures** (use synthetic) |
| Refactoring discipline | Cite `skills/slo-plan/references/refactoring-discipline.md`: sub-pass insertion + section addition as microsteps; prove Pass 1–5 headings + retro existing sections unchanged after. |
| AI tolerance contract | `N/A — no AI component` (the pass is agent-run prose; the *test* mechanizes the checkable subset; no model invocation or eval harness is added) |
| Data classification | `Internal` (fixtures model PII-bearing telemetry — **synthetic PII only**; cite C8 below; includes abuse cases per the Confidential/Internal rule) |
| Proactive controls in play | OWASP C8 (data protection — pseudonymise/mask check); C10 (error handling/logging — failure-path emission check); C5 (input handling — fence-rule note for user strings in generated artifacts) |
| Abuse acceptance scenarios | New control surface (the measurement pass). BDD includes all three threat-model abuse cases, each demonstrated by the failure-bar fixture: `tm-measurement-loop-abuse-1` (**injection** payload in a thesis/feature-name field → injection/fence check CATCHES the unfenced user-string — SEC-1, now TESTED not just cited; CWE-1336/CWE-94), `tm-measurement-loop-abuse-2` (privacy-careless team ships unmasked-PII telemetry field → PII/masking check CATCHES it), `tm-measurement-loop-abuse-3` (author sets `feature_measurement_spec: true` with no spec → flag↔section cross-check CATCHES it). Class-elimination of abuse-1 remains the existing `~~~text` fence rule; this milestone makes the control verifiable. |
| Forbidden shortcuts | No Pass renumber; no hard-fail on absent telemetry context; no vacuous demo (bad fixture must fail); no real PII in fixtures; no TODO/placeholder; no swallowing a caught finding |

#### Out of Scope / Must Not Do

- Do not implement a production telemetry collector or a machine-readable `.slo.json` schema (deferred).
- Do not change Pass 1–5 semantics or the Pass 4 PII-scan behaviour.
- Do not edit the loop docs (M5).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-verify/SKILL.md` | Add measurement sub-pass: **six** checks (a–f incl. injection/fence) + tool-optional `skipped`-row rule + finding format; reuse Pass 4 heuristic style; prose names the same checks the test mechanizes (ENG-2) |
| `skills/slo-retro/SKILL.md` | Add `## Results vs thesis` to the lessons template (did leading move? lagging? implication for next milestone/runbook?) |
| `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs` | NEW: assert pass + retro sentinels; assert prose↔mechanical lockstep (ENG-2); mechanize the checkable subset; assert catch→remediate→green over the fixture pair; assert synthetic-PII markers (SEC-2) |
| `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/bad.md` | NEW: `SYNTHETIC PII` header (SEC-2); four seeded defects — missing event, unmasked synthetic-PII telemetry field, gamed flag, AND an injection payload in a thesis/feature-name field (SEC-1 / abuse-1) |
| `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/remediated.md` | NEW: `SYNTHETIC PII` header; the corrected fixture — event present, PII masked/pseudonymised, flag matches a real spec, user-string `~~~text`-fenced/neutralized |

#### Step-by-Step

1. Write `mloop_m4_verify_retro.rs` first, including the failure-bar + lockstep assertions; create the `bad.md` fixture with the `SYNTHETIC PII` header and all four seeded defects (missing event, unmasked synthetic-PII field, gamed flag, injection payload); run; confirm the test fails for the right reason.
2. Read Pass 4 in `slo-verify/SKILL.md` and the retro lessons template.
3. Add the measurement sub-pass prose — six checks (a–f incl. injection/fence) + skipped-row rule + finding format — naming the same checks the test mechanizes (ENG-2 lockstep).
4. Add `## Results vs thesis` to `slo-retro/SKILL.md`.
5. Author the `remediated.md` fixture so the mechanical checks pass on it and still fail on `bad.md`.
6. Re-run `mloop_m4_verify_retro.rs`; confirm green (catch on bad, green on remediated, sentinels present).
7. Full `cargo test -p sast-verify`, fmt, clippy.
8. Confirm Pass 1–5 headings + retro existing sections unchanged (diff).
9. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: telemetry verification + durable learning + failure bar**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Six checks documented | happy path | Edited slo-verify | test runs | sentinels for event-presence, PII/masking, failure-path, replay-tagging, flag↔section, injection/fence asserted |
| Prose↔mechanical lockstep | invalid input | Edited slo-verify | test runs | the SKILL.md prose names the SAME checks the test mechanizes — asserted as contiguous substrings (ENG-2) |
| Tool-optional skip | dependency failure | Telemetry context absent | pass logic on a no-telemetry fixture | emits `skipped` row, not a hard fail |
| Results-vs-thesis present | happy path | Edited slo-retro | test runs | `## Results vs thesis` sentinel asserted |
| Catch injection (abuse) | abuse case | `bad.md` has an injection payload in a thesis/feature-name field | injection/fence check runs | check FAILS on `bad.md` — `tm-measurement-loop-abuse-1` caught (SEC-1; CWE-1336/CWE-94) |
| Catch unmasked PII (abuse) | abuse case | `bad.md` has a raw synthetic email in a telemetry field | mechanical PII check runs | check FAILS on `bad.md` — `tm-measurement-loop-abuse-2` caught |
| Catch gamed flag (abuse) | abuse case | `bad.md` sets `feature_measurement_spec: true` with no spec section | flag↔section cross-check runs | check FAILS on `bad.md` — `tm-measurement-loop-abuse-3` caught |
| Remediate → green | retry/rollback | `remediated.md` fixes all four defects | same checks run | all checks PASS on `remediated.md` |
| Non-vacuous demo | assertion violation | both fixtures | test runs | bad FAILS and remediated PASSES in the same test (vacuity guard) |
| Synthetic-PII marker | empty state | both fixtures | test runs | both carry the `SYNTHETIC PII` header comment (SEC-2) |
| Pass numbering preserved | backward compat | Edited slo-verify | test runs | Pass 1–5 headings present and unchanged |
| Retro refusal preserved | backward compat | blank actuals | existing retro rule | refusal-on-blank-actuals sentinel still present |

#### Regression Tests

- `cargo test -p sast-verify` — all baselines green; Pass 4 PII-scan sentinels (if asserted elsewhere) unchanged.
- `./target/release/sldo-install --dry-run` — `/slo-verify` + `/slo-retro` discovered.

#### Compatibility Checklist

- [ ] `/slo-verify` Pass 1–5 numbering + Pass 4 behaviour unchanged
- [ ] `/slo-retro` existing sections + refusal-on-blank-actuals unchanged
- [ ] Legacy runbooks without telemetry → `skipped`, not fail
- [ ] Fixtures contain synthetic PII only; `git status` clean

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_verify_measurement_pass_six_checks_present` | pass documented | six-check sentinels asserted |
| `slo_verify_prose_mechanical_lockstep` | drift guard | prose names the mechanized checks (ENG-2) |
| `slo_verify_measurement_pass_tool_optional` | graceful skip | `skipped`-row rule sentinel asserted |
| `slo_retro_results_vs_thesis_present` | retro learning | `## Results vs thesis` asserted |
| `failure_bar_bad_fixture_is_caught` | catch | mechanical checks FAIL on `bad.md` (abuse-1 + abuse-2 + abuse-3) |
| `failure_bar_remediated_fixture_is_green` | remediate→green | checks PASS on `remediated.md` |
| `failure_bar_is_non_vacuous` | guardrail real | bad fails AND remediated passes asserted together |
| `failure_bar_fixtures_marked_synthetic` | hygiene | both fixtures carry `SYNTHETIC PII` header (SEC-2) |
| `slo_verify_passes_not_renumbered` | compat | Pass 1–5 headings asserted |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes (incl. failure-bar tests)
- [ ] `./target/release/sldo-install --dry-run` lists `/slo-verify` + `/slo-retro`
- [ ] Static analysis passes
- [ ] Fixtures use synthetic PII only (manual read)
- [ ] `git status` shows only the intended new fixtures

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | | | |
| BDD test + bad fixture created | `mloop_m4_verify_retro.rs` + `bad.md` | fails for expected reason | | | |
| Implementation | pass + retro section | contract satisfied | | | |
| Remediated fixture | `remediated.md` | bad fails, remediated passes | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Static analyzer | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Full tests | `cargo test -p sast-verify` | green | | | |
| Failure-bar (catch→remediate→green) | failure-bar tests | non-vacuous demo passes | | | |
| Pass/section no-renumber | headings | unchanged | | | |
| Fixture PII review | read fixtures | synthetic only | | | |
| Test artifact cleanup | `git status` | clean | | | |

#### Definition of Done

- six-check measurement pass (incl. injection/fence) + tool-optional rule + prose↔mechanical lockstep (ENG-2) present and asserted
- `/slo-retro` `## Results vs thesis` present and asserted
- failure-bar demo non-vacuous: `bad.md` caught (abuse-1 + abuse-2 + abuse-3), `remediated.md` green
- Pass 1–5 + retro existing sections unchanged (asserted)
- fixtures synthetic-PII only, marked with `SYNTHETIC PII` header (SEC-2)
- baselines + fmt + clippy green
- lessons + completion written; tracker updated

#### Post-Flight

- **Other docs**: none beyond the two SKILL.md files (loop docs are M5).

---

### Milestone 5 — Document the Feature-performance loop in `LOOPS-ENGINEERING.md` + cross-ref in `LOOPS-BUSINESS.md`

**Goal**: The new loop is catalogued as a normal operating mode: a **Feature-performance loop** entry in `docs/LOOPS-ENGINEERING.md` (standard loop-entry format — User-visible outcome, Trigger, Steps, Exit condition, Artifacts, Skills involved, ASCII diagram) and a cross-reference from `docs/LOOPS-BUSINESS.md` (where the post-ship `/slo-metrics` cohort-tracking touchpoint lives), closing the documentation gap the report identified.

**Context**: `docs/LOOPS-ENGINEERING.md` documents sprint, secure-construction, ticket, security-tuning, lessons, library-feedback loops; `docs/LOOPS-BUSINESS.md` documents user-interview, GTM, pricing, founder-check loops. Both use an identical loop-entry format. The v4 template and CLAUDE.md link these docs by anchor, so anchors must stay stable.

**Carmack-style reliability goal**: Compatibility — additive loop entry; existing loop anchors unchanged so prerequisite-reading links don't break.

**Important design rule**: Place the new loop in `LOOPS-ENGINEERING.md` (it threads ideate→plan→execute→verify→retro, all engineering skills) and add a short cross-reference in `LOOPS-BUSINESS.md` near the GTM/pricing loops (where the post-ship `/slo-metrics` cohort touchpoint sits). One canonical home, one cross-ref — no duplication.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | The completed M1–M4 surfaces (success thesis, feature spec, measurement contract, verify pass, retro section) |
| Outputs | Feature-performance loop entry in `LOOPS-ENGINEERING.md`; cross-ref paragraph in `LOOPS-BUSINESS.md` |
| Interfaces touched | Loop catalog (additive entry + cross-ref); existing loop anchors |
| Files allowed to change | `docs/LOOPS-ENGINEERING.md`; `docs/LOOPS-BUSINESS.md`; `xtasks/sast-verify/tests/mloop_m5_loops.rs` (NEW) |
| Files to read before changing anything | `docs/LOOPS-ENGINEERING.md`; `docs/LOOPS-BUSINESS.md`; `docs/slo/design/measurement-loop-slo-improvements-overview.md` (the loop diagram) |
| New files allowed | `xtasks/sast-verify/tests/mloop_m5_loops.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing loop entries + anchors unchanged; the v4 template / CLAUDE.md links to these docs still resolve; loop-entry format reused exactly |
| Resource bounds introduced/changed | Exactly one new loop entry + one cross-ref paragraph; no restructure of existing entries |
| Invariants/assertions required | Structural test asserts: `Feature-performance loop` heading + the standard sub-headings (User-visible outcome / Trigger / Steps / Exit condition / Artifacts / Skills involved) present in `LOOPS-ENGINEERING.md`; a cross-ref to the loop present in `LOOPS-BUSINESS.md`; existing loop headings (Sprint loop, Lessons loop, GTM loop, Pricing loop) still present |
| Debugger / inspection expectation | `cargo test -p sast-verify -- --nocapture` on failure |
| Static analysis gates | `cargo fmt --check`; `cargo clippy -D warnings`; `cargo test -p sast-verify` |
| Exemplar code to copy | Any existing loop entry in `LOOPS-ENGINEERING.md` (Sprint/Lessons loop format); the loop diagram in `measurement-loop-slo-improvements-overview.md` |
| Anti-exemplar code not to copy | Restructuring/renaming existing loop entries; duplicating the full loop in both docs (one home + one cross-ref) |
| Refactoring discipline | Cite `skills/slo-plan/references/refactoring-discipline.md`: append loop entry + cross-ref as microsteps; prove existing loop headings unchanged after. |
| AI tolerance contract | `N/A — no AI component` |
| Data classification | `Public` |
| Proactive controls in play | `N/A — pure documentation, no data-handling surface (the loop it documents carries the controls, enforced in M3/M4)` |
| Abuse acceptance scenarios | `N/A — no new runtime surface introduced (documentation only).` |
| Forbidden shortcuts | No restructure of existing loops; no anchor changes; no full duplication across both docs; no TODO/placeholder |

#### Out of Scope / Must Not Do

- Do not edit any SKILL.md or the template (done in M1–M4).
- Do not rename existing loop anchors.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/LOOPS-ENGINEERING.md` | Add **Feature-performance loop** entry in the standard format (after Lessons loop, before Library-feedback loop) with the overview's ASCII diagram |
| `docs/LOOPS-BUSINESS.md` | Add a short cross-reference paragraph near the GTM/pricing loops pointing to the engineering-side Feature-performance loop and the post-ship `/slo-metrics` cohort touchpoint |
| `xtasks/sast-verify/tests/mloop_m5_loops.rs` | NEW: assert loop entry + sub-headings + cross-ref present; existing loop headings unchanged |

#### Step-by-Step

1. Write `mloop_m5_loops.rs` first; run; confirm fail.
2. Read both loop docs and the overview loop diagram.
3. Add the Feature-performance loop entry to `LOOPS-ENGINEERING.md` in the standard format.
4. Add the cross-ref paragraph to `LOOPS-BUSINESS.md`.
5. Re-run `mloop_m5_loops.rs`; confirm green.
6. Full `cargo test -p sast-verify`, fmt, clippy.
7. Confirm existing loop headings + anchors unchanged (diff).
8. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: catalogued feature-performance loop**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Loop entry present | happy path | Edited LOOPS-ENGINEERING | test runs | `Feature-performance loop` + standard sub-headings asserted |
| Cross-ref present | happy path | Edited LOOPS-BUSINESS | test runs | cross-ref-to-feature-performance-loop sentinel asserted |
| Existing loops preserved | backward compat | Edited docs | test runs | Sprint/Lessons/GTM/Pricing loop headings still present |
| No full duplication | invalid input | Edited docs | test runs | the full step-list appears once (engineering doc); business doc carries a cross-ref, not the full loop |
| Pre-fix fails | assertion violation | Unedited docs | test runs | test FAILS (anti-vacuity) |

#### Regression Tests

- `cargo test -p sast-verify` — all baselines green.
- Anchor check: links to `LOOPS-ENGINEERING.md#sprint-loop` (used by skills/templates) still resolve.

#### Compatibility Checklist

- [ ] Existing loop entries + anchors unchanged
- [ ] v4 template / CLAUDE.md links to loop docs still resolve
- [ ] Loop-entry format reused exactly
- [ ] No full duplication across the two docs

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/mloop_m5_loops.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `feature_performance_loop_entry_present` | loop catalogued | heading + sub-headings asserted |
| `feature_performance_loop_cross_ref_present` | business-side discoverability | cross-ref sentinel asserted |
| `existing_loops_preserved` | no restructure | existing loop headings asserted |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes
- [ ] Existing loop anchors resolve
- [ ] Static analysis passes
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | | | |
| BDD test created | `mloop_m5_loops.rs` | fails for expected reason | | | |
| Implementation | loop entry + cross-ref | contract satisfied | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Static analyzer | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Full tests | `cargo test -p sast-verify` | green | | | |
| Anchor / no-restructure check | existing loop headings | unchanged | | | |
| Test artifact cleanup | `git status` | clean | | | |

#### Definition of Done

- Feature-performance loop entry (standard format) present and asserted
- cross-ref in `LOOPS-BUSINESS.md` present and asserted
- existing loops + anchors unchanged (asserted)
- `mloop_m5_loops.rs` fails pre-fix, passes post-fix
- baselines + fmt + clippy green
- lessons + completion written; tracker updated; **runbook complete**

#### Post-Flight

- **Other docs**: optionally add the loop to the CLAUDE.md / catalog overview if the user wants it surfaced there (additive; confirm first).

---

## 18. Documentation Update Table

| Milestone | ARCHITECTURE.md | README.md | .gitignore | Other Docs |
|---|---|---|---|---|
| 1 | none | none | none | `skills/slo-ideate/SKILL.md` (self-documenting) |
| 2 | none | none | none | `references/biz/artifact-schema.md`; optional additive note in `biz-skill-pack-interfaces.md` |
| 3 | none | none | none | `docs/slo/templates/runbook-template_v_4_template.md`; `skills/slo-plan/SKILL.md` |
| 4 | none | none | confirm fixtures committed intentionally (no scratch) | `skills/slo-verify/SKILL.md`; `skills/slo-retro/SKILL.md` |
| 5 | none | none | none | `docs/LOOPS-ENGINEERING.md`; `docs/LOOPS-BUSINESS.md`; optional CLAUDE.md surface |

---

## 19. Optional Fast-Fail Review Prompt for Agents

Use §19 of [the v4 template](slo/templates/runbook-template_v_4_template.md) before writing production (here: contract + test) code for each milestone.

---

## 19A. Deferred to a follow-up runbook (post-critique decision)

- **CEO-1 — financial-loop wiring.** The loop's post-ship financial touchpoint (`/slo-metrics consumer|b2b` cohort tracking against the success-thesis window) is *documented* in M5 but **not wired** into a `/slo-metrics` contract change. Wiring it would require a 6th milestone, breaking the `/slo-plan` 5-cap. **Decision: defer to a separate follow-up runbook** (`/slo-metrics` cohort-vs-thesis touchpoint), preserving the cap and the PM/financial split. Tracked here so `/slo-retro` can file it as `fresh-runbook` lane at close-out.
- **CEO-2 — M4 size.** M4 is kept whole (the failure bar is the point). If the non-vacuous demo cannot land in one `/slo-execute` pass, split the fixture/demo portion to a follow-up rather than shipping a "demo later" shortcut.
- **Machine-readable telemetry `.slo.json` schema** — deferred by the architecture decision (no fixtures yet); promote in a future `/slo-architect` pass.

## 20. Source Basis

Authored by `/slo-plan` against the locked architecture in `docs/slo/design/measurement-loop-slo-improvements-*.md`, which were produced by `/slo-architect` from `~/Downloads/deep-research-report(5).md`. Structure mirrors `docs/RUNBOOK-kani-verification.md` (the closest precedent: a Markdown-contract + structural-test enhancement to the skill pack with a catch→remediate→green failure-bar demonstration). Next step: `/slo-critique` before any `/slo-execute`.


