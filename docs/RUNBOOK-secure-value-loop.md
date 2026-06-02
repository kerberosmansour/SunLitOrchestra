# Secure Value Loop — SunLit Orchestra (AI-First Runbook v4)

> **Purpose**: Wrap the SLO sprint loop in a Secure Value Envelope by adding three typed, unavoidable runbook disciplines — Operator Readiness Gate, Detected Work Ledger, honest exit states — plus a canonical `docs/SECURE-VALUE-LOOP.md`, a v4-template "Secure Value & Security Contract" section, and LOOPS-doc updates, all additive and reusing the security machinery the pack already ships.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Prefer automated guardrails over intention. Reuse shipped security capability; add only contract discipline. Every change additive and backward compatible.
> **How to use**: Work milestones sequentially. Complete the Global Entry Protocol before each, the Global Exit Protocol after each. Never widen scope silently.
> **Prerequisite reading**: [docs/slo/design/secure-value-loop-overview.md](slo/design/secure-value-loop-overview.md), [stack-decision](slo/design/secure-value-loop-stack-decision.md), [interfaces](slo/design/secure-value-loop-interfaces.md), [threat-model](slo/design/secure-value-loop-threat-model.md), [reversibility](slo/design/secure-value-loop-reversibility.md), [code-map](slo/design/secure-value-loop-code-map.md), [synthesis](slo/research/secure-value-loop/synthesis.md), [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md).

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `secure-value-loop` |
| Project name | SunLit Orchestra (the skill pack itself) |
| Primary stack | Markdown skill contracts + Rust structural tests (`xtasks/sast-verify`) |
| Primary package/app names | `skills/slo-*`, `docs/slo/templates`, `docs/LOOPS-*`, `xtasks/sast-verify` |
| Prefix for tests and lesson files | `svl` |
| Default unit test command | `cargo test -p sast-verify` (M3 also: `cargo test -p sldo-common`) |
| Default integration/BDD test command | `cargo test -p sast-verify` (structural-contract tests) |
| Default E2E/runtime validation command | `cargo test -p sast-verify -- --include-ignored` |
| Default build/boot command | `cargo build -p sast-verify` (M3 also: `cargo build -p sldo-common`) |
| Default formatter command | `cargo fmt --all` |
| Default static analysis / lint command | `cargo clippy --all-targets` |
| Default dependency / security audit command | `cargo deny check` (M3: crate version bumped, run it) |
| Default debugger or state-inspection tool | `cargo test ... -- --nocapture`; `diff` for dual-template byte-identity |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- Existing v4-template section numbers §6 / §10 / §17 (no renumbering).
- Milestone-status enum `not_started | in_progress | blocked | done` (extend additively only) — in BOTH the template comment AND `sldo-common::runbook::MilestoneStatus` (M3 makes the Rust enum total over the documented set; published-crate semver bump required).
- `sldo-common::runbook` public API (`MilestoneStatus`, `parse_tracker`, `all_done`, `next_incomplete`) — `evolving`: extended additively in M3, never reshaped.
- `/slo-retro` lane verbs (`product | upstream-OSS | slo-process`) and carry-forward lanes (`micro | milestone | fresh-runbook`) — reuse, do not fork.
- `/slo-product` / `/slo-metrics` / `/slo-verify` Pass 4/5 internals; `/slo-architect` threat-model schema `slo_schema_version: 0.1.0`.
- Repo-root `SECURITY.md`; the §5A Measurement Contract and §10 Carry-forward sections.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Canonical doc + v4-template Secure Value & Security Contract section | `done` | 2026-06-02 | 2026-06-02 | docs/slo/lessons/svl-m1.md | docs/slo/completion/svl-m1.md |
| 2 | `/slo-plan` requires the Secure Value Contract | `done` | 2026-06-02 | 2026-06-02 | docs/slo/lessons/svl-m2.md | docs/slo/completion/svl-m2.md |
| 3 | Additive status enum + Operator Readiness Gate (`/slo-execute`, `/slo-resume`) | `done` | 2026-06-02 | 2026-06-02 | docs/slo/lessons/svl-m3.md | docs/slo/completion/svl-m3.md |
| 4 | Detected Work Ledger (`/slo-execute` ↔ `/slo-retro`) + Bundle A–F evidence (`/slo-verify`) | `done` | 2026-06-02 | 2026-06-02 | docs/slo/lessons/svl-m4.md | docs/slo/completion/svl-m4.md |
| 5 | LOOPS docs + `/slo-ship` secure-release checklist + dogfood | `done` | 2026-06-02 | 2026-06-02 | docs/slo/lessons/svl-m5.md | docs/slo/completion/svl-m5.md |

<!-- Status values: not_started | in_progress | blocked | done (extended additively in M3 with: human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk). Unknown values → treat as `blocked` (never silent `done`/`not_started`), enforced in both skill prose and sldo-common::runbook::MilestoneStatus from the M3 release onward. -->
<!-- Lessons files go in docs/slo/lessons/svl-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/svl-m<N>.md -->

---

## 3. End-to-End Architecture Diagram

See [overview](slo/design/secure-value-loop-overview.md) for the full envelope-overlay diagram. Condensed end state:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│              SunLit Orchestra — Secure Value Envelope (end state)               │
│                                                                                │
│  docs/SECURE-VALUE-LOOP.md (NEW, canonical) ── cites ──▶ SSDF + OWASP PC 2024  │
│        │ referenced by                                                          │
│        ▼                                                                        │
│  v4 template §5B "Secure Value & Security Contract"  (NEW, optional/additive)   │
│   ├─ Value Wedge                                                                │
│   ├─ Security Definition of Ready (Operator Readiness)──▶ /slo-execute Global   │
│   ├─ Threat Model Summary (from /slo-architect, cited)        Entry gate (M3)   │
│   ├─ Security Test Plan (Bundles A–F)───────────────────▶ /slo-verify Pass4/5   │
│   └─ Detected Work Ledger ──────────────────────────────▶ /slo-retro lanes (M4) │
│        │ authored by /slo-plan (M2)                                             │
│        ▼                                                                        │
│  Milestone Tracker status enum (additive, M3): + human_review_required /        │
│   blocked_by_operator / blocked_by_upstream / issue_filed / accepted_risk       │
│        │ read by /slo-resume + /slo-execute (unknown → blocked fallback)        │
│        ▼                                                                        │
│  /slo-ship secure-release checklist + ship_state (M5);  LOOPS docs name each    │
│   stage's security output (M5)                                                  │
│                                                                                │
│  Gate (all milestones): xtasks/sast-verify structural-contract tests           │
│  Legend: ─── existing  ▶ data/ref flow  NEW = additive surface                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `docs/SECURE-VALUE-LOOP.md` | Canonical envelope definition + agent one-page prompt | NEW | M1 | Project doc |
| v4 template (dual copy) | Add §5B Secure Value & Security Contract; tighten proactive-controls row | changed | M1 | v4 runbook contract |
| `skills/slo-plan/SKILL.md` | Require §5B for value-bearing/security-relevant milestones | changed | M2 | Runbook authoring |
| v4 template status comment + `skills/slo-execute` + `skills/slo-resume` | Additive status enum + Operator Readiness gate | changed | M3 | Status vocabulary |
| `crates/sldo-common/src/runbook.rs` (+Cargo.toml) | Extend published `MilestoneStatus` parser total over additive set; fix `all_done` silent-completion (F-ENG-1); crates.io 0.1.3 | changed | M3 | `sldo-common::runbook` public API |
| `skills/slo-execute` + `skills/slo-retro` + `skills/slo-verify` | Detected Work Ledger + lane reconciliation + Bundle A–F evidence rows | changed | M4 | Ledger / lanes / verify passes |
| `docs/LOOPS-*` + `skills/slo-ship` | Stage security outputs + secure-release checklist + `ship_state` | changed | M5 | Loop catalog / ship |
| `xtasks/sast-verify/tests/svl_m*.rs` | Structural-contract gates per milestone | NEW | each M | Test gate |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Operator prerequisites | `/slo-plan` §5B | `/slo-execute` Global Entry | runbook section | yes — per milestone | `blocked`+`safe_to_continue:false` → milestone must not start | M2→M3 |
| Detected work | `/slo-execute` | Ledger → `/slo-retro` | runbook table + disposition | yes — per finding | undisposed row → not `done` | M4 |
| Milestone status | `/slo-execute`/`/slo-retro` | Tracker, `/slo-resume` | Status cell (extended enum) | yes — closed enum | unknown value → `blocked` fallback | M3 |
| Bundle selection | §5B Security Test Plan | `/slo-verify` Pass 4/5 | bundle ref, surface-resolved | yes — A–F | wrong bundle → over/under-test; waiver needs reason | M4 |

---

## 4. Carmack-Style Development Best Practices

All §4.1–§4.8 rules from the v4 template apply. The high-leverage ones for this Markdown-discipline runbook:

- **§4.2 Static analysis is mandatory** — here that means the `cargo test -p sast-verify` structural-contract gate is the analysis; treat a failing assertion as design feedback.
- **§4.5 Make invalid states unrepresentable** — the additive status enum is a *closed* set with a documented fallback; the Detected Work Ledger forbids the "observed-with-no-disposition" state by construction.
- **§4.6 Preserve compatibility until explicitly broken** — the binding constraint of this entire runbook. Every change is additive; legacy runbooks and the existing parsers must keep working.
- **§4.7 Prefer small, local, reviewable changes** — edit only the listed skill/template/test files; mirror dual-template edits in the same change.
- **§4.8 No silent failure** — unknown status value must surface as `blocked`, never silently `done`; an undisposed ledger row must block `done`.

---

## 5. High-Level Design for State Modeling / Formal Verification

`N/A — no concurrency, distributed state, resource ownership, ordering guarantees, retries, queues, idempotency, or irreversible runtime actions.` This is Markdown skill-contract + template + loop-doc work plus an additive enum and Rust structural tests. The only "state machine" is the milestone-status enum, which is a small closed set verified by a structural-contract test, not a model-checked protocol. `tla_required: false`, `kani_required: false` (see [overview](slo/design/secure-value-loop-overview.md) frontmatter for justifications).

### 5.8 Kani proof obligations

`N/A — no Rust kernels introduced. The only Rust touched is xtasks/sast-verify structural tests (string/section/byte-identity assertions); no unsafe, arithmetic, parser, or representation-invariant kernel worth a bounded proof.`

---

## 5A. Measurement Contract

`N/A — not a value-bearing feature. This runbook changes the SLO pack's own skill-contract discipline (tooling/process); it ships no user-facing telemetry runtime and no analytics surface. Adoption is measured against the proposal §11 acceptance criteria, tracked qualitatively in each milestone's /slo-retro and the §10 carry-forward, not via runtime events. This mirrors the measurement-loop runbook's posture for SLO-internal tooling work.`

---

## 5B. Secure Value and Security Contract

> **Dogfood (M5):** this runbook fills its own §5B to prove the contract is usable on a real runbook, not merely declared. The work is security-relevant (it edits the SLO security skills + a published crate), so §5B applies even though it is not value-bearing.

### Value Wedge

| Field | Value |
|---|---|
| Value hypothesis | Threading three typed disciplines (Operator Readiness Gate, Detected Work Ledger, honest exit states) into the SLO loop makes security/operability unavoidable and machine-checkable, eliminating mid-milestone operator stalls and "observed-but-undisposed" findings. |
| Smallest valuable wedge | The five milestones here — canonical doc + §5B template + `/slo-plan` requirement + the three disciplines + LOOPS/ship wiring — reusing all shipped security machinery. |
| User-visible proof of value | An agent knows before a milestone starts what human action is required; every finding ends with a disposition; a milestone can end honestly (`blocked_by_operator`, etc.) instead of a false `done`. |
| Security-visible proof of safety | The published `sldo-common` parser can no longer report a blocked runbook complete (F-ENG-1 regression test); the Operator Readiness Gate fails closed; injection surfaces are `~~~text`-fenced. |
| What would make this wedge too small to matter? | Shipping the doc/template without the enforcement consumers — declared-but-inert (the reason M3/M4 land the consumers). |

### Security Definition of Ready (Operator Readiness)

| Prerequisite | Owner | Needed by | Validation (executable proof) | Status |
|---|---|---|---|---|
| Green baseline `cargo test -p sldo-common -p sast-verify` | agent | M1 | suite runs green (recorded each milestone Evidence Log) | ready |
| GitHub labels `operator-action-required` + `security-review-required` | human | ship | `gh label list` shows both (commands in SECURE-VALUE-LOOP.md §10) | partially_ready |

`safe_to_continue_without_blockers: true` — the labels are a post-merge/ship operator action; they do not block development. The degraded path: development proceeds; label creation is tracked as DW-002 and surfaced at ship.

### Threat Model Summary

| Area | Summary |
|---|---|
| Assets | integrity of the runbook contract; the disposition guarantee; trust in the gate; user strings flowing into generated artifacts |
| Actors | author/agent; content-injector; gate-gamer; legacy-runbook consumer (`sldo-common`, `/slo-resume`) |
| Trust boundaries | author text → generated docs; status cell → enum parsers; ledger row → `/slo-retro` filing → GitHub |
| Entry points | §5B sections; additive status enum; ledger→retro bridge |
| Abuse cases | `tm-secure-value-loop-abuse-1` (contract string injection), `-abuse-2` (readiness-gate bypass), `-abuse-3` (disposition laundering), `-abuse-4` (additive-enum break) |
| Required controls | `~~~text` fence at named generation surfaces; gate fails closed; `fix_now` reserved for safe/local; unknown-status→`Blocked` in prose + `sldo-common` |
| Residual risks | thin/dishonest contract (owner Sherif, review 2026-09-02); guidance-not-runtime; bundle-edition drift — all `accepted_residual: true` in the `.slo.json` |

### Security Test Plan (Bundle A — docs/skill-contract + Rust tests)

| Test | Required? | Command/tool | Evidence path | Waiver if not applicable |
|---|---|---|---|---|
| SAST | yes (Rust) | `cargo clippy` | per-milestone verify reports | — |
| SCA/dependency audit | conditional | `cargo deny check` | DW-001 (pre-existing licenses failure) | — |
| Secrets scan | yes | heuristic grep / gitleaks (skipped — not installed) | per-milestone verify reports | — |
| IaC / Container / DAST | not_applicable | — | — | no IaC/images/services |
| Abuse-case tests | yes | `svl_m1..m5` + `sldo-common` regression tests | `docs/slo/verify/svl-m*.md` | — |
| Fuzz/property/formal | not_applicable | — | — | no parser/protocol kernel beyond the tested enum |
| SBOM/provenance | not_applicable | — | — | no released artifact built in this runbook (0.1.3 publish deferred) |

### Detected Work Ledger

| ID | Finding | Severity | Disposition | Owner | Evidence/link | Due |
|---|---|---:|---|---|---|---|
| DW-001 | Pre-existing `cargo deny check` licenses-policy failure (not introduced by this runbook; dependency graph unchanged) | Low | file_github_issue | agent→human | `docs/slo/verify/svl-m4.md`; `cargo deny check` output | ship / next runbook |
| DW-002 | GitHub labels `operator-action-required` + `security-review-required` not yet created on the live repo | Low | operator_action | human | `gh label create …` in SECURE-VALUE-LOOP.md §10 | ship |
| DW-003 | crates.io publish of `sldo-common` 0.1.3 (version bumped in M3) | Low | accepted_risk | Sherif | bump in `Cargo.toml`; publish is a deliberate release, not this runbook | review at next release |
| DW-004 | M3 workspace version bump was incomplete — `publish_prep` lockstep test required the two internal dep version strings + `PUBLISH_READY_VERSION` to also move to 0.1.3 (caught by full-workspace run in M5 verify) | Medium | fix_now | agent | `crates/sldo-install/Cargo.toml`, `crates/sldo-research/Cargo.toml`, `crates/sldo-install/tests/e2e_crates_io_followup.rs`; allow-list extended with rationale | M5 (done) |
| DW-005 | Skill-prose additions tripped two structural caps: `/slo-plan` SKILL.md > 80-line hard cap (extracted detail to `references/secure-value-contract.md`); `/slo-verify` SKILL.md > 200-line soft cap (added `# soft-cap-exception`) | Low | fix_now | agent | `skills/slo-plan/SKILL.md`, `skills/slo-plan/references/secure-value-contract.md`, `skills/slo-verify/SKILL.md` | M5 (done) |

---

## 6. Global Execution Rules

All §6.1–§6.11 rules from the v4 template apply. Runbook-specific emphases:

- **§6.1 Stay inside scope** — change only the files in each milestone's allow-list. Editing a `SKILL.md` that has a pinned SHA baseline (search `xtasks/sast-verify/tests/` for the skill name) requires updating that baseline in the **same** milestone.
- **§6.8 Preserve backward compatibility** — every milestone must verify legacy runbooks (e.g. an existing `docs/RUNBOOK-*.md` without §5B), `/slo-resume`, and `/slo-execute` Step 1.5 still work unchanged.

### Global Red Lines (additions for this runbook)

- No NEW crate, no new dependency, no new runtime. (M3 extends the EXISTING `sldo-common` crate — additive enum variants + a recorded crates.io semver bump; this is the only Rust touched.)
- No third disposition taxonomy — the five ledger dispositions resolve to existing `/slo-retro` lanes per the [interfaces mapping](slo/design/secure-value-loop-interfaces.md#3-detected-work-ledger-disposition-vocabulary-reconciled--no-third-taxonomy).
- No section renumbering of §6 / §10 / §17.
- No dual-template drift — both v4-template copies stay byte-identical.
- No OWASP control cited by bare number — name + edition (2024) only.
- No clobbering repo-root `SECURITY.md`.

---

## 7. Global Entry Rules (Pre-Milestone Protocol)

Follow the v4 template §7 protocol verbatim. Key points for this runbook:

1. Read the previous milestone's `docs/slo/lessons/svl-m<N-1>.md`.
2. `/slo-execute` Step 1.5 carry-forward (§10) applies.
3. Run the baseline: `cargo test -p sast-verify` must be green before starting.
4. Before editing a `SKILL.md`, grep `xtasks/sast-verify/tests/` for a pinned SHA baseline of that skill.
5. Write the milestone's new `xtasks/sast-verify/tests/svl_m<N>.rs` structural test FIRST and confirm it fails for the right reason.

---

## 8. Global Exit Rules (Post-Milestone Protocol)

Follow the v4 template §8 protocol verbatim. Mandatory for every milestone here:

1. `cargo fmt --all` + `cargo clippy -p sast-verify --all-targets`.
2. `cargo test -p sast-verify` green (incl. the new `svl_m<N>` test).
3. **Dual-template byte-identity check** when the template changed: `diff docs/slo/templates/runbook-template_v_4_template.md skills/slo-plan/references/runbook-template_v_4_template.md` returns empty.
4. **Backward-compat check**: a legacy `docs/RUNBOOK-*.md` without §5B still resolves under `/slo-resume` (manual trace).
5. Update any pinned SKILL.md SHA baseline touched this milestone.
6. Write `docs/slo/lessons/svl-m<N>.md` and `docs/slo/completion/svl-m<N>.md`; update the Tracker.

---

## 9. Background Context

### Current State

The SLO pack already ships ~80% of the proposal's security machinery: threat model in `/slo-architect` Step 3.5, class-elimination security persona in `/slo-critique`, Pass 4/5 security+LLM matrix in `/slo-verify`, lane-classified issue filing in `/slo-retro`, nine dedicated security skills, and v4 Contract Block rows for Data classification / Proactive controls / Abuse scenarios / AI tolerance / Measurement deliverables. The v4 template is dual-copied (`docs/slo/templates/...` mirror + `skills/slo-plan/references/...` primary) and byte-identity is asserted by `xtasks/sast-verify`. (Full evidence: [research synthesis](slo/research/secure-value-loop/synthesis.md).)

### Problem

1. **No Operator Readiness Gate** — milestones can start then stall on a missing account/credential/approval; readiness is implicit in the Definition of Done, never a first-class pre-execution state. (`grep` for "operator readiness" returns nothing.)
2. **No Detected Work Ledger** — work discovered mid-sprint lands as unstructured lessons-file prose; findings can end as merely "observed" with no enforced disposition.
3. **No honest exit states** — milestone status is only `not_started | in_progress | blocked | done`; a run that genuinely needs a human review, an operator action, or an accepted-risk decision is forced into `done` or a bare `blocked`.
4. **No canonical envelope doc / template contract** — the value-first rule and per-stage security outputs live only in the external proposal, not in the repo; `docs/LOOPS-BUSINESS.md` has no security stages.
5. **OWASP control-citation drift** — the Contract Block row says "e.g., C1, C5, C9" (bare numbers); OWASP renumbered C1–C10 between 2018 and 2024, so bare numbers silently change meaning.

### Key Design Principles

1. **Additive-only.** Every section/row/status value is optional; legacy runbooks and existing parsers keep working. (Precedent: §5A, §10.)
2. **Reuse, don't rebuild.** Cite shipped security skills and the `/slo-retro` lane vocabulary; introduce no new capability or taxonomy.
3. **Fail closed, surface honestly.** Unknown status → `blocked`; undisposed finding → not `done`; unprovisioned prerequisite → milestone does not start.
4. **One gate.** Enforcement is the `xtasks/sast-verify` structural-contract test, not a runtime validator.

### What to Keep

- The dual-template byte-identity discipline and all existing v4 sections/numbers.
- `/slo-retro` lane vocabulary and filing discipline (dedupe, `~~~text` fence, 40/hr cap).
- `/slo-verify` Pass 4/5 internals; `/slo-architect` threat-model schema; repo-root `SECURITY.md`.

### What to Change

- **v4 template (both copies)** — add §5B; tighten proactive-controls row.
- **`skills/slo-plan|slo-execute|slo-verify|slo-retro|slo-resume|slo-ship/SKILL.md`** — wire the three disciplines.
- **`docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md`** — name per-stage security outputs.
- **`docs/SECURE-VALUE-LOOP.md`** — NEW canonical doc.
- **`xtasks/sast-verify/tests/svl_m*.rs`** — NEW structural tests.

---

## 10. Carry-forward from prior retros

No prior-retro issues exist for the `svl` prefix yet. `/slo-execute` Step 1.5 falls back to the live `gh issue list` query. Populate this table once `/slo-retro` files the first `svl`-prefixed issue.

| Issue | Title | Suggested lane | Suggested milestone | Status |
|---|---|---|---|---|
| (none yet) | | | | |

---

## 11. BDD and Runtime Validation Rules

Standard v4 §11 rules. For this runbook, "BDD tests" are **structural-contract assertions** in `xtasks/sast-verify/tests/svl_m<N>.rs` (read repo files, assert content/shape), and "runtime validation" is a **manual trace** of an agent reading the edited contract plus a legacy-runbook backward-compat trace. Each milestone covers: happy path (new contract present + parsed), invalid input (malformed/injected field), empty/first-run (legacy runbook without the new section), compatibility (old parsers + old four status values), and abuse case (per the threat model).

---

## 12. Dependency, Migration, and Refactor Policy

- **Dependencies**: none permitted (no new crate; structural tests use std only, matching existing `svl`-peer tests like `mloop_m3_plan.rs`).
- **Migration**: none — all changes additive; no schema/config migration.
- **Refactor budget**: per-milestone, stated in each Contract Block (default `No refactor permitted beyond direct implementation`).

---

## 13. Evidence Log Template

(Standard v4 §13 table — copied into each milestone below.)

---

## 14. Self-Review Gate

Standard v4 §14 questions, plus runbook-specific:

- Did I keep both v4-template copies byte-identical?
- Did I avoid renumbering §6/§10/§17?
- Did I keep the change additive (legacy runbook + `/slo-resume` + `/slo-execute` Step 1.5 still work)?
- Did I reuse `/slo-retro` lanes instead of inventing a taxonomy?
- Did I update any pinned SKILL.md SHA baseline I touched?
- Did I cite OWASP controls by name + edition, never bare number?

---

## 15. Lessons-Learned File Template

Path: `docs/slo/lessons/svl-m<N>.md` — use the v4 §15 template.

## 16. Completion Summary Template

Path: `docs/slo/completion/svl-m<N>.md` — use the v4 §16 template.

---

## 17. Milestone Plan

### Milestone 1 — `Canonical doc + v4-template Secure Value & Security Contract section`

**Goal**: Add `docs/SECURE-VALUE-LOOP.md` (canonical envelope definition + agent one-page prompt) and insert an optional, additive **§5B "Secure Value & Security Contract"** section into both v4-template copies, plus tighten the proactive-controls Contract Block row to OWASP 2024 by-name. After this milestone the contract *shape* exists and is byte-identical across copies; nothing yet *requires* it.

**Context**: The v4 template lives at `docs/slo/templates/runbook-template_v_4_template.md` (mirror) and `skills/slo-plan/references/runbook-template_v_4_template.md` (primary), kept byte-identical and asserted by `xtasks/sast-verify`. The §5A Measurement Contract insertion (between §5 and §6, using a letter suffix to avoid renumbering) is the exact exemplar to copy.

**Carmack-style reliability goal**: Make invalid states unrepresentable (§4.5) — the section ships with explicit "optional; legacy runbooks remain valid" framing and `N/A — <reason>` escapes so an empty contract is a documented state, not a silent gap.

**Important design rule**: Insert §5B as a **letter-suffixed section between §5A and §6** — do NOT renumber §6/§10/§17. Edit both template copies in the same change.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | The five §5B sub-blocks defined in [interfaces §1](slo/design/secure-value-loop-interfaces.md#1-new-optional-v4-template-section-secure-value--security-contract); OWASP PC 2024 control names |
| Outputs | `docs/SECURE-VALUE-LOOP.md`; §5B in both template copies; tightened proactive-controls row; `svl_m1` structural test |
| Interfaces touched | v4 template section layout; Contract Block proactive-controls row wording |
| Files allowed to change | `docs/SECURE-VALUE-LOOP.md` (new), `docs/slo/templates/runbook-template_v_4_template.md`, `skills/slo-plan/references/runbook-template_v_4_template.md`, `xtasks/sast-verify/tests/svl_m1.rs` (new), `.gitignore` |
| Files to read before changing anything | both template copies (§5A and Contract Block rows), `xtasks/sast-verify/tests/mloop_m3_plan.rs` (test exemplar), [interfaces](slo/design/secure-value-loop-interfaces.md) |
| New files allowed | `docs/SECURE-VALUE-LOOP.md`, `xtasks/sast-verify/tests/svl_m1.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Legacy runbooks without §5B remain valid; §6/§10/§17 numbers unchanged; both copies byte-identical |
| Resource bounds introduced/changed | N/A — no runtime resource |
| Invariants/assertions required | `read(PRIMARY) == read(MIRROR)`; §6/§10/§17 headings present and unchanged; §5B contains all five sub-block headings; proactive-controls row contains an edition year + a named control |
| Debugger / inspection expectation | `diff` the two template copies; `cargo test -p sast-verify -- --nocapture` to read assertion output |
| Static analysis gates | `cargo fmt --all`, `cargo clippy -p sast-verify --all-targets`, `cargo test -p sast-verify` |
| Exemplar code to copy | `xtasks/sast-verify/tests/mloop_m3_plan.rs` (byte-identity + required-fields + no-renumber asserts); v4 template §5A block as the section-insertion shape |
| Anti-exemplar code not to copy | Do not copy any pattern that edits only one template copy; do not clobber repo-root `SECURITY.md` |
| Refactoring discipline | `N/A — no refactoring performed (additive insertion only)` |
| AI tolerance contract | `N/A — no AI component` |
| Forbidden shortcuts | Editing one template copy only; renumbering existing sections; bare-number OWASP citation; leaving a §5B sub-block heading out |
| Data classification | `Internal` — SLO process docs |
| Proactive controls in play | OWASP Proactive Controls 2024: `C4 Address Security from the Start` (the §5B section institutionalises early-lifecycle security), `C3 Validate all Input & Handle Exceptions` (the `~~~text` fence rule for user strings the section documents) |
| Abuse acceptance scenarios | `tm-secure-value-loop-abuse-1` (contract string injection) — §5B fields are author-written runbook prose, so the injection surface that actually matters is where a *skill generates an artifact from those strings*: the **concrete surfaces are (a) `/slo-resume` quoting carry-forward/ledger snippets (already `~~~text`-fenced) and (b) M5's `/slo-ship` quoting ledger rows into a PR body**. §5B documents the `~~~text` fence requirement scoped to those generation surfaces (not to inert author prose); `svl_m1` asserts the fence rule text + the named surfaces are present (F-SEC-3) |
| Measurement deliverables | `N/A — not value-bearing (tooling/process), see §5A` |

#### Out of Scope / Must Not Do

- Do NOT make `/slo-plan` require the section yet (that is M2).
- Do NOT add status-enum values or ledger wiring (M3/M4).
- Do NOT touch repo-root `SECURITY.md` or any root `ARCHITECTURE.md`.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/SECURE-VALUE-LOOP.md` | NEW: canonical envelope definition (10-section structure from the proposal, condensed), Bundle A–F table cited by name+edition, agent one-page prompt, ledger↔lane mapping |
| `docs/slo/templates/runbook-template_v_4_template.md` | Add §5B section; tighten proactive-controls Contract Block row |
| `skills/slo-plan/references/runbook-template_v_4_template.md` | Identical edit (byte-for-byte) |
| `xtasks/sast-verify/tests/svl_m1.rs` | NEW: structural-contract test |
| `.gitignore` | Add patterns if any test artifacts are generated |

#### Step-by-Step

1. Write `xtasks/sast-verify/tests/svl_m1.rs` first (asserts: §5B present in both copies; all five sub-block headings; byte-identity; §6/§10/§17 unchanged; proactive-controls row has edition+named-control; fence-rule text present). Confirm it fails.
2. Write `docs/SECURE-VALUE-LOOP.md`.
3. Insert §5B into the primary template copy; copy verbatim to the mirror.
4. Tighten the proactive-controls Contract Block row wording per [interfaces §6](slo/design/secure-value-loop-interfaces.md).
5. Run fmt/clippy/test; `diff` the two copies.
6. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Secure Value & Security Contract section exists and is consistent**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Section present both copies | happy path | both v4-template copies | `svl_m1` runs | §5B + all five sub-blocks asserted present in each |
| Byte-identity | compatibility | both copies edited | `diff` | empty output; test asserts `read(PRIMARY)==read(MIRROR)` |
| No renumber | compatibility | existing §6/§10/§17 | `svl_m1` runs | headings present and unchanged |
| Bare-number citation rejected | invalid input | proactive-controls row | `svl_m1` runs | row must contain edition year + a named control, not only `Cn` |
| Legacy runbook still valid | empty state | a `docs/RUNBOOK-*.md` without §5B | manual `/slo-resume` trace | resolves normally; §5B optional |
| Fence-rule documented | abuse case (`tm-secure-value-loop-abuse-1`) | §5B + SECURE-VALUE-LOOP.md | `svl_m1` runs | `~~~text` fence requirement for user strings is asserted present |

#### Regression Tests

- All existing `xtasks/sast-verify` tests (esp. `mloop_m3_plan.rs` byte-identity assertions) still pass.
- A legacy runbook without §5B still orients under `/slo-resume`.

#### Compatibility Checklist

- [ ] §6/§10/§17 numbers unchanged
- [ ] Both template copies byte-identical
- [ ] Legacy runbooks parse without §5B
- [ ] Existing structural tests green

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/svl_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `secure_value_section_present_and_identical` | §5B exists in both copies and they match | all asserts pass |
| `existing_sections_not_renumbered` | additive insertion didn't shift §6/§10/§17 | headings found unchanged |
| `proactive_controls_row_named_and_editioned` | citation-drift fix | row contains `2024` + a named control |

#### Smoke Tests

- [ ] `cargo test -p sast-verify` passes
- [ ] `diff` of the two template copies is empty
- [ ] `docs/SECURE-VALUE-LOOP.md` renders (headings, Bundle table, agent prompt present)
- [ ] `git status` clean of test artifacts

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Repo hygiene | `git rev-parse --abbrev-ref HEAD` | not on `main` | branch `slo/secure-value-loop` (created off `main`); tree had only this runbook's own untracked planning docs | Pass | task-scoped branch covers the whole runbook |
| Baseline tests | `cargo test -p sast-verify` | green | 21 test files green pre-change | Pass | |
| Structural test created | `svl_m1.rs` | fails for expected reason | 6/8 failed: missing `docs/SECURE-VALUE-LOOP.md` + §5B + 2024 row; 2 passed (copies still identical, sections untouched) | Pass | BDD-first confirmed |
| Implementation | §5B + doc + row | contract satisfied | `docs/SECURE-VALUE-LOOP.md` written; §5B inserted between §5A and §6 (letter suffix, no renumber); proactive-controls row → OWASP 2024 by-name | Pass | |
| Formatter | `cargo fmt --all` | clean | reformatted `svl_m1.rs` arrays only | Pass | |
| Lint | `cargo clippy -p sast-verify --tests` | no NEW warnings | only pre-existing warnings in `sap_imp_m3_standards`/bin; `svl_m1.rs` clean | Pass | pre-existing warnings out of scope (§4.7) |
| Full tests | `cargo test -p sast-verify` | green | 21 files green incl. `svl_m1` (8) and `mloop_m3_plan` byte-identity+no-renumber (F-ENG-4) | Pass | |
| Byte-identity | `diff` two template copies | empty | empty — "BYTE-IDENTICAL" | Pass | mirror `cp`-synced from primary |
| Compatibility | legacy runbook `/slo-resume` trace | resolves | §5B optional; `existing_sections_not_renumbered` asserts §5A/§6/§10/§17 intact → legacy runbooks parse unchanged | Pass | runtime trace deferred to /slo-verify |
| .gitignore review | `git status --short` | no stray artifacts | only intended files (templates M; doc/test/planning ??) | Pass | no new generated outputs |

#### Definition of Done

- §5B present in both copies, byte-identical; all five sub-blocks present.
- Proactive-controls row cites OWASP 2024 by name+edition.
- `docs/SECURE-VALUE-LOOP.md` exists with Bundle A–F table + agent prompt + ledger↔lane mapping.
- §6/§10/§17 unchanged; legacy runbooks still valid.
- `svl_m1` + full suite green; lessons + completion written; Tracker updated.

#### Post-Flight

- **ARCHITECTURE/overview**: note §5B shipped.
- **Other docs**: `docs/SECURE-VALUE-LOOP.md` created.

---

### Milestone 2 — `/slo-plan requires the Secure Value Contract`

**Goal**: Make `/slo-plan` author §5B for any **value-bearing or security-relevant** milestone (identity, secrets, PII, payment, cloud, AI, public/network boundary, CI/CD, infra), populating Value Wedge, Operator Readiness rows, Threat Model Summary (from the existing `/slo-architect` threat model), and Security Test Plan — while keeping legacy runbooks valid (forward-looking requirement, like the Measurement Contract).

**Context**: `skills/slo-plan/SKILL.md` already has a "Measurement Contract requirement" block with a deterministic "value-bearing" definition and a forward-looking, non-retroactive posture. Copy that exact pattern for §5B, adding the "security-relevant" trigger list.

**Carmack-style reliability goal**: Static analysis as design feedback (§4.2) — a new `svl_m2` test asserts the requirement text + trigger list + non-retroactive framing exist in the SKILL.

**Important design rule**: The requirement is **forward-looking** — `/slo-plan` flags a missing §5B for new value-bearing/security-relevant runbooks but never invalidates legacy ones. Reuse the existing Measurement-Contract requirement wording shape.

**Sequencing note (F-ENG-3, intentional)**: M2 lands the `/slo-plan` *mandate* before M3/M4 land the *consumers* (the Operator Readiness Gate and the ledger discipline). This is a deliberate, accepted inert window: between M2 and M3/M4 the readiness/ledger rows are authored but not yet enforced. `/slo-plan` MUST state in the generated §5B that "the Operator Readiness Gate is enforced by `/slo-execute` from the M3 release onward" so an author does not assume early enforcement. Reordering was considered and rejected: the mandate-first order lets real runbooks start carrying the contract immediately, and the additive posture means no harm from the inert window.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — only to co-locate the new requirement next to the Measurement Contract requirement.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `/slo-architect` threat-model output; the §5B section from M1; the security-relevant trigger list |
| Outputs | `/slo-plan` requirement text + trigger list; `svl_m2` test; SKILL SHA baseline update if pinned |
| Interfaces touched | `/slo-plan` authoring contract (Contract Block sentinels list) |
| Files allowed to change | `skills/slo-plan/SKILL.md`, `skills/slo-plan/references/methodology-milestone-authoring.md` (only if the sentinel list lives there), `xtasks/sast-verify/tests/svl_m2.rs` (new), any `xtasks/sast-verify` test pinning `/slo-plan` SHA |
| Files to read before changing anything | `skills/slo-plan/SKILL.md` (Measurement Contract requirement block), `xtasks/sast-verify/tests/mloop_m3_plan.rs`, [interfaces §1](slo/design/secure-value-loop-interfaces.md) |
| New files allowed | `xtasks/sast-verify/tests/svl_m2.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Legacy runbooks without §5B remain valid; `/slo-plan` for non-value-bearing/non-security work still marks §5B `N/A` |
| Resource bounds introduced/changed | N/A |
| Invariants/assertions required | SKILL contains the §5B requirement + the deterministic value-bearing/security-relevant trigger definition + the non-retroactive clause |
| Debugger / inspection expectation | `cargo test ... -- --nocapture`; grep for any pinned `/slo-plan` SHA baseline |
| Static analysis gates | fmt / clippy / `cargo test -p sast-verify` |
| Exemplar code to copy | the existing "Measurement Contract requirement" block in `skills/slo-plan/SKILL.md`; `mloop_m3_plan.rs` assert pattern |
| Anti-exemplar code not to copy | Do not make the requirement retroactive; do not invent a new section name |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md` — behavior-preserving co-location only |
| AI tolerance contract | `N/A — no AI component` |
| Forbidden shortcuts | Retroactive invalidation of legacy runbooks; skipping the SHA-baseline update if `/slo-plan` is pinned |
| Data classification | `Internal` |
| Proactive controls in play | OWASP PC 2024 `C4 Address Security from the Start` (planning-time security requirement) |
| Abuse acceptance scenarios | `tm-secure-value-loop-abuse-3` (disposition laundering) is partly mitigated here — §5B forces the Security Test Plan + Threat Model Summary so findings have a home; `svl_m2` asserts those sub-blocks are required by `/slo-plan` |
| Measurement deliverables | `N/A — not value-bearing (tooling/process), see §5A` |

#### Out of Scope / Must Not Do

- Do NOT add status-enum/ledger/verify wiring (M3/M4).
- Do NOT change the Measurement Contract requirement's behavior.

#### Files Allowed To Change / Step-by-Step / BDD / Regression / Compatibility / E2E / Smoke / Evidence Log / DoD

- **Files**: as listed in Contract Block; `.gitignore` if needed.
- **Step-by-Step**: (1) write `svl_m2.rs` asserting the requirement text/trigger/non-retroactive clause — confirm fail; (2) add the §5B requirement block to `/slo-plan` SKILL co-located with the Measurement Contract requirement; (3) update pinned SHA baseline if any; (4) fmt/clippy/test; (5) Self-Review.
- **BDD**: happy path (requirement present + trigger list complete), invalid input (a value-bearing runbook missing §5B → `/slo-plan` flags gap), empty state (non-value-bearing → §5B `N/A` accepted), compatibility (legacy runbook not invalidated), abuse (`tm-secure-value-loop-abuse-3` — forces Test Plan + Threat Model Summary).
- **Regression**: Measurement Contract requirement still enforced; existing `/slo-plan` SHA-pinned tests pass.
- **Compatibility**: legacy runbooks valid; non-security non-value work marks §5B `N/A`.
- **E2E** (`svl_m2.rs`): `plan_requires_secure_value_contract`, `requirement_is_forward_looking_not_retroactive`, `security_relevant_triggers_listed`.
- **Smoke**: `cargo test -p sast-verify` green; SHA baseline (if any) updated.
- **Evidence Log**: standard table (baseline / test-created / impl / fmt / clippy / tests / SHA-baseline / compatibility).
- **DoD**: `/slo-plan` requires §5B for value-bearing/security-relevant work, forward-looking; trigger list present; `svl_m2` + full suite green; SHA baseline updated; lessons+completion written; Tracker updated.

---

### Milestone 3 — `Additive status enum + Operator Readiness Gate`

**Goal**: Extend the milestone-status vocabulary additively (`human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk`) with a documented "unknown → `blocked`" fallback **enforced in both the skill prose AND the published `sldo-common::runbook` parser** (F-ENG-1), and add the **Operator Readiness Gate** to `/slo-execute`'s Global Entry (refuse to start a milestone whose Operator Readiness is `blocked` with `safe_to_continue_without_blockers: false`), with `/slo-resume` recognising the new states read-only. Also create the `operator-action-required` GitHub label (proposal §9 #10), tied to the gate.

**Context**: The status comment lives in the v4 template (line ~62) and is read by `/slo-resume` (branches on Status) and `/slo-execute` Step 1.5. **Critically, there is also a deterministic, published consumer:** [`crates/sldo-common/src/runbook.rs`](slo/../../crates/sldo-common/src/runbook.rs) — `enum MilestoneStatus` currently `{NotStarted, InProgress, Done}` (no `blocked`!), regex `` `(not_started|in_progress|done)` ``, `FromStr` unknown→`Err`, `parse_tracker` `.unwrap_or(NotStarted)`, and `all_done()`. `sldo-common` is published on crates.io (v0.1.2). Per F-ENG-1, leaving this unchanged lets `all_done()` report a runbook complete while a `blocked_by_operator` row is unfinished — violating this runbook's own "no silent `done`" rule. Carry-forward §10's "consumers fall back gracefully" posture is the prose exemplar; the Rust fix makes the fallback real.

**Carmack-style reliability goal**: Make invalid states unrepresentable + no silent failure (§4.5/§4.8) — the Rust enum becomes **total over the documented status set** (incl. the pre-existing `blocked` it never supported, F-ENG-2), unknown→`Blocked`, `all_done` treats any non-`Done` (incl. unknown) as not-done. The class (status the parser doesn't know) is *eliminated*, not just mitigated.

**Important design rule**: Additive only. The old three Rust variants + the documented `blocked` + the five new values MUST round-trip; any unrecognised value parses to `Blocked`, NEVER silently `done`/`NotStarted`. Edit both template copies (status comment) byte-identically. Record the crates.io version bump (0.1.2 → 0.1.3) per version discipline.

**Refactor budget**: `Targeted refactor permitted for extending sldo-common::runbook MilestoneStatus to be total over the documented status set (F-ENG-1)`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | New status values; Operator Readiness sub-block from §5B (M1); `safe_to_continue_without_blockers` flag; the existing `sldo-common::runbook` parser |
| Outputs | Extended status comment (both template copies); extended `sldo-common::runbook` enum/regex/`FromStr`/`all_done` + round-trip unit tests; crates.io 0.1.2→0.1.3 bump; `/slo-execute` Global Entry readiness check; `/slo-resume` new-state handling; `operator-action-required` label; `svl_m3` test; SHA baselines for `/slo-execute` + `/slo-resume` if pinned |
| Interfaces touched | Milestone-status enum (Markdown comment AND `sldo-common::runbook::MilestoneStatus`); `/slo-execute` Global Entry; `/slo-resume` status branching |
| Files allowed to change | `docs/slo/templates/runbook-template_v_4_template.md`, `skills/slo-plan/references/runbook-template_v_4_template.md`, `skills/slo-execute/SKILL.md`, `skills/slo-resume/SKILL.md`, `crates/sldo-common/src/runbook.rs`, `crates/sldo-common/Cargo.toml` (version), `Cargo.toml` (workspace version if shared), `xtasks/sast-verify/tests/svl_m3.rs` (new), `xtasks/sast-verify/tests/mloop_m3_plan.rs` (only if it hard-codes the old status set), any `xtasks/sast-verify` test pinning those SKILL SHAs |
| Files to read before changing anything | v4 template status comment + §10 carry-forward; `crates/sldo-common/src/runbook.rs` (lines 12, 33-36, 56-72, 113-119); `skills/slo-resume/SKILL.md` (status branch); `skills/slo-execute/SKILL.md` (Global Entry + Step 1.5); [interfaces §2](slo/design/secure-value-loop-interfaces.md); [critique appendix A-1](slo/critique/secure-value-loop.md) |
| New files allowed | `xtasks/sast-verify/tests/svl_m3.rs` |
| New dependencies allowed | `none` (extends existing `sldo-common`; no NEW crate) |
| Migration allowed | `no` (additive enum variants are backward compatible; semver bump recorded) |
| Compatibility commitments | Old three Rust variants + documented `blocked` + five new values all round-trip; `all_done`/`next_incomplete` semantics unchanged for legacy runbooks; legacy runbooks (no Operator Readiness sub-block) skip the gate; `/slo-resume` orients on legacy runbooks unchanged |
| Resource bounds introduced/changed | N/A |
| Invariants/assertions required | status comment lists old four AND new five; `MilestoneStatus` is **total** over the documented set; `FromStr`/`parse_tracker` map unknown→`Blocked` (NOT `NotStarted`); `all_done` returns false if any row is non-`Done` (incl. unknown/blocked); no tracker row is silently dropped by the regex; `/slo-execute` + `/slo-resume` prose encodes "unknown → `blocked`"; Global Entry refuses start on `safe_to_continue_without_blockers: false`; both template copies byte-identical |
| Debugger / inspection expectation | `cargo test -p sldo-common -- --nocapture` round-trip tests; trace `/slo-resume` against a runbook using each new status; `diff` template copies |
| Static analysis gates | fmt / clippy / `cargo test -p sldo-common -p sast-verify`; `cargo deny check` (crate edited + version bumped) |
| Exemplar code to copy | §10 Carry-forward backward-compat framing; `/slo-resume` existing status-branch block; the existing `runbook.rs` `mod tests` pattern (line 124) for round-trip tests |
| Anti-exemplar code not to copy | Any change that replaces (not extends) the enum; any path that maps an unknown value to `done`/`NotStarted`; the current regex that silently drops non-matching status rows (fix it) |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md` — behavior-preserving for the old three values, additive for the rest; pre/post round-trip test evidence |
| AI tolerance contract | `N/A — no AI component` |
| Forbidden shortcuts | Replacing the enum; mapping unknown → `done`/`NotStarted`; editing one template copy only; skipping the round-trip tests; skipping SHA-baseline updates; an unrecorded crates.io version bump |
| Data classification | `Internal` |
| Proactive controls in play | OWASP PC 2024 `C1 Implement Access Control` (the readiness gate is an authorization-to-proceed control), `C9 Implement Security Logging and Monitoring` (honest exit states = accurate status record), `C3 Validate all Input & Handle Exceptions` (total parser over the status domain) |
| Abuse acceptance scenarios | `tm-secure-value-loop-abuse-2` (readiness gate bypass) — `validation` column requires executable proof, gate fails closed; `tm-secure-value-loop-abuse-4` (additive-enum break) — unknown→`Blocked` fallback now enforced in the Rust parser, not only prose; `svl_m3` asserts the comment rule + a Rust round-trip test proves the fallback. **NOTE (F-SEC-2):** structural tests assert contract-text presence + the Rust round-trip; runtime *agent* adherence to the prose gate remains the documented accepted residual (thin/dishonest-contract risk, owner Sherif, review 2026-09-02). |
| Measurement deliverables | `N/A — not value-bearing (tooling/process), see §5A` |

#### Out of Scope / Must Not Do

- Do NOT wire the Detected Work Ledger or Bundle evidence (M4).
- Do NOT change how the old four values behave.

#### BDD / E2E / DoD (condensed — full tables filled at execution per §13)

- **BDD**: happy path (gate present, new states listed; every documented status round-trips through `MilestoneStatus`), invalid input (unknown status → `Blocked` in both prose and `FromStr`, asserted), empty state (legacy runbook without readiness sub-block → gate skipped, orients normally), dependency failure (`safe_to_continue_without_blockers: false` → `/slo-execute` refuses start), resource/abuse (a `blocked_by_operator` row no longer makes `all_done()` return `true` — the F-ENG-1 defect), compatibility (old three Rust variants + documented `blocked` parse; `/slo-resume` unchanged on legacy), abuse (`tm-...-abuse-2`, `tm-...-abuse-4`).
- **E2E** (`svl_m3.rs` structural + `crates/sldo-common/src/runbook.rs` `mod tests` round-trip): `status_enum_extended_additively_old_values_present`, `unknown_status_maps_to_blocked_not_done_or_notstarted`, `all_done_false_when_any_row_blocked_or_unknown` (the F-ENG-1 regression), `every_documented_status_roundtrips`, `execute_global_entry_has_operator_readiness_gate`, `resume_recognizes_new_states`, `templates_byte_identical`.
- **Regression**: existing `mloop_m3_plan.rs` byte-identity + no-renumber asserts stay green (F-ENG-4); existing `runbook.rs` unit tests still pass; `/slo-execute` + `/slo-resume` SHA-pinned tests pass (baselines updated); `cargo test -p sldo-common` green.
- **DoD**: `MilestoneStatus` total over the documented set with unknown→`Blocked`; `all_done` cannot return true with a non-`Done` row (F-ENG-1 fixed, regression test proves it); `blocked` now supported (F-ENG-2); Operator Readiness Gate in Global Entry fails closed; `operator-action-required` label created; `/slo-resume` recognises new states read-only; both template copies byte-identical and existing byte-identity test green (F-ENG-4); crates.io 0.1.3 bump recorded; SHA baselines updated; `svl_m3` + `cargo test -p sldo-common -p sast-verify` green; lessons+completion; Tracker updated.

---

### Milestone 4 — `Detected Work Ledger + Bundle A–F security evidence`

**Goal**: Add the **Detected Work Ledger** discipline to `/slo-execute` (open/update the §5B ledger during a run; every finding gets exactly one of the five dispositions; refuse `done` while a row is undisposed), reconcile the five dispositions so they **introduce no new `/slo-retro` lane verb** (each disposition routes to an existing mechanism per the interfaces mapping — precise framing per F-SEC-1, not the looser "no taxonomy"), make `/slo-verify` record **Bundle A–F** security tests as first-class evidence rows (`pass | not_applicable | waived_with_reason`), and create the `security-review-required` GitHub label (proposal §9 #9) tied to the M2 trigger list.

**Context**: `/slo-retro` already has a lane vocabulary + filing discipline (dedupe, `~~~text` fence, 40/hr cap) in `skills/slo-retro/references/issue-filing-discipline.md`. `/slo-verify` Pass 4/5 already detect surface and run the matrix; this adds *evidence-row recording* keyed to the bundle, not a new runner. The disposition↔lane mapping is fixed in [interfaces §3](slo/design/secure-value-loop-interfaces.md).

**Carmack-style reliability goal**: No silent failure (§4.8) — an undisposed ledger row blocks `done`; a bundle test with no evidence must be `not_applicable`/`waived_with_reason`, never blank.

**Important design rule**: The five dispositions are **names that resolve to existing `/slo-retro` lanes** per the interfaces mapping — introduce no new lane verb. `/slo-retro` re-reads the ledger at retro and files `file_github_issue`/`upstream_feedback` rows through the existing flow (cap + dedupe apply).

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | §5B Detected Work Ledger + Security Test Plan (M1); the disposition↔lane mapping; `/slo-retro` filing discipline; `/slo-verify` Pass 4/5 surface detection |
| Outputs | `/slo-execute` ledger discipline; `/slo-retro` ledger-reconciliation step; `/slo-verify` bundle evidence rows; `svl_m4` test; SHA baselines for the three SKILLs if pinned |
| Interfaces touched | `/slo-execute` execution rules; `/slo-retro` issue-filing flow (reuse); `/slo-verify` evidence-row format |
| Files allowed to change | `skills/slo-execute/SKILL.md`, `skills/slo-retro/SKILL.md`, `skills/slo-retro/references/issue-filing-discipline.md`, `skills/slo-verify/SKILL.md`, `xtasks/sast-verify/tests/svl_m4.rs` (new), `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` + `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs` (the tests that pin `/slo-verify` + `/slo-retro` consumer baselines — update if edits move them) |
| Files to read before changing anything | `skills/slo-retro/references/issue-filing-discipline.md`, `skills/slo-execute/SKILL.md` (allow-list + Step 1.5), `skills/slo-verify/SKILL.md` (Pass 4/5), `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` + `mloop_m4_verify_retro.rs` (named pinned baselines — F-ENG-5), [interfaces §3 + §4](slo/design/secure-value-loop-interfaces.md) |
| New files allowed | `xtasks/sast-verify/tests/svl_m4.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Legacy runbooks without a ledger skip it; `/slo-retro` lane verbs + cap + dedupe unchanged; `/slo-verify` Pass 4/5 internals unchanged (only evidence recording added) |
| Resource bounds introduced/changed | Reuse `/slo-retro` 40 issues/hr cap — do NOT bypass or change it |
| Invariants/assertions required | `/slo-execute` refuses `done` with an undisposed ledger row; all five dispositions present, each with a documented route, AND the `/slo-retro` lane verbs (`product\|upstream-OSS\|slo-process`) are **unchanged** — `svl_m4` asserts no new lane verb is introduced (F-SEC-1, the enforceable invariant); `/slo-verify` bundle rows are `pass\|not_applicable\|waived_with_reason` (no blank); `/slo-retro` re-reads the ledger and routes through existing filing; `security-review-required` label exists |
| Debugger / inspection expectation | trace a finding from `/slo-execute` ledger → disposition → `/slo-retro` filing; grep for new lane verbs (must be none) |
| Static analysis gates | fmt / clippy / `cargo test -p sast-verify` |
| Exemplar code to copy | `skills/slo-retro/references/issue-filing-discipline.md` (lanes, cap, fence); `/slo-verify` existing Pass 4 evidence note pattern |
| Anti-exemplar code not to copy | Any new disposition verb outside the five; any path that re-implements issue filing instead of reusing `/slo-retro`; any blank bundle evidence cell |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md` |
| AI tolerance contract | `N/A — no AI component` |
| Forbidden shortcuts | New taxonomy; bypassing the 40/hr cap; blank evidence rows; leaving a finding "observed" |
| Data classification | `Internal` |
| Proactive controls in play | OWASP PC 2024 `C9 Implement Security Logging and Monitoring` (ledger = auditable finding record), `C6 Keep your Components Secure` (`upstream_feedback` disposition feeds dependency fixes) |
| Abuse acceptance scenarios | `tm-secure-value-loop-abuse-3` (disposition laundering) — `fix_now` reserved for safe/local/in-allow-list; `/slo-execute` refuses `done` on undisposed row; `/slo-retro` re-reads ledger; `svl_m4` asserts these rules |
| Measurement deliverables | `N/A — not value-bearing (tooling/process), see §5A` |

#### Out of Scope / Must Not Do

- Do NOT add a `.slo.json` ledger companion (deferred, see reversibility).
- Do NOT change `/slo-retro` lane verbs, dedupe, or the cap.
- Do NOT add a new `/slo-verify` pass — only evidence-row recording.

#### BDD / E2E / DoD (condensed)

- **BDD**: happy path (ledger row → disposition → done allowed), invalid input (undisposed row → `done` refused), empty state (no findings → empty ledger accepted), dependency failure (`file_github_issue` over the 40/hr cap → spill per existing rule), compatibility (legacy runbook without ledger; `/slo-retro` lanes unchanged), abuse (`tm-...-abuse-3`).
- **E2E** (`svl_m4.rs`): `execute_refuses_done_on_undisposed_ledger_row`, `five_dispositions_present_each_routed`, `no_new_retro_lane_verb_introduced` (F-SEC-1), `retro_reuses_filing_discipline_and_cap`, `verify_bundle_rows_have_no_blank_state`, `security_review_required_label_documented`.
- **Regression**: `slo_tm_m2_consumers.rs` + `mloop_m4_verify_retro.rs` baselines pass (named, F-ENG-5); `/slo-retro` filing tests, `/slo-verify` Pass 4/5 tests, all SHA-pinned tests pass (baselines updated).
- **DoD**: ledger discipline enforced; five dispositions routed with **no new `/slo-retro` lane verb** (F-SEC-1); bundle evidence rows first-class with no blanks; `/slo-retro` reuses filing+cap; `security-review-required` label created; named pinned baselines + SHA baselines updated; `svl_m4` + suite green; lessons+completion; Tracker updated. Structural tests assert contract-text presence; runtime agent adherence is the documented accepted residual (F-SEC-2).

---

### Milestone 5 — `LOOPS docs + /slo-ship secure-release checklist + dogfood`

**Goal**: Patch `docs/LOOPS-ENGINEERING.md` (primary) and `docs/LOOPS-BUSINESS.md` (cross-ref) so every sprint stage names its cybersecurity output; add the secure-release checklist + `ship_state` vocabulary + conditional SBOM/provenance to `/slo-ship`; and **dogfood** the full Secure Value Contract on one small real runbook to prove it is usable, not merely declared.

**Context**: `docs/LOOPS-ENGINEERING.md` already documents Security-tuning and Secure-construction loops; `docs/LOOPS-BUSINESS.md` has no security stages. `/slo-ship` opens a PR and runs the suite but has no secure-release checklist. Dogfooding closes the proposal §11 "model is adopted when…" loop.

**Carmack-style reliability goal**: Evidence over claims (§6.10) — the dogfood produces a real filled §5B as proof the contract is fillable; SBOM/provenance stays conditional (`not_applicable` for markdown).

**Important design rule**: SBOM/provenance is a **`when applicable`** checklist row, never a hard gate for markdown milestones. The dogfood target must be a genuinely small, low-risk runbook (or this runbook's own retro) — do not invent scope.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Per-stage security outputs (from the proposal §1 table); `ship_state` vocabulary; SBOM/provenance conditional rule; a small dogfood runbook |
| Outputs | LOOPS-ENGINEERING/BUSINESS edits; `/slo-ship` secure-release checklist + `ship_state`; one filled §5B dogfood artifact; `svl_m5` test; `/slo-ship` SHA baseline if pinned |
| Interfaces touched | Loop catalog; `/slo-ship` checklist + `ship_state` |
| Files allowed to change | `docs/LOOPS-ENGINEERING.md`, `docs/LOOPS-BUSINESS.md`, `skills/slo-ship/SKILL.md`, `docs/SECURE-VALUE-LOOP.md` (cross-link only), the dogfood runbook file, `xtasks/sast-verify/tests/svl_m5.rs` (new), any test pinning `/slo-ship` SHA |
| Files to read before changing anything | `docs/LOOPS-ENGINEERING.md` (loop entries), `docs/LOOPS-BUSINESS.md`, `skills/slo-ship/SKILL.md`, [overview](slo/design/secure-value-loop-overview.md), [interfaces §5](slo/design/secure-value-loop-interfaces.md) |
| New files allowed | `xtasks/sast-verify/tests/svl_m5.rs`; the dogfood §5B artifact (if a new small runbook) |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `/slo-ship` still opens-but-never-merges; existing loop entries preserved; SBOM/provenance non-blocking for markdown |
| Resource bounds introduced/changed | N/A |
| Invariants/assertions required | each sprint stage in LOOPS-ENGINEERING names a security output; `/slo-ship` has the checklist + closed `ship_state` enum; SBOM/provenance row is conditional; dogfood §5B is filled (no placeholder rows) |
| Debugger / inspection expectation | read the dogfood §5B end-to-end; confirm `ship_state` enum closed |
| Static analysis gates | fmt / clippy / `cargo test -p sast-verify` |
| Exemplar code to copy | existing LOOPS-ENGINEERING loop-entry format; `/slo-ship` existing checklist shape |
| Anti-exemplar code not to copy | Making SBOM/provenance a hard gate; a dogfood with placeholder rows; `/slo-ship` auto-merge |
| Refactoring discipline | `N/A — no refactoring performed` |
| AI tolerance contract | `N/A — no AI component` |
| Forbidden shortcuts | Hard-gating SBOM on markdown; fake/placeholder dogfood; skipping `/slo-ship` SHA baseline if pinned |
| Data classification | `Internal` |
| Proactive controls in play | OWASP PC 2024 `C9 Implement Security Logging and Monitoring` (ship monitoring/canary/rollback row), `C6 Keep your Components Secure` (SBOM/provenance when applicable) |
| Abuse acceptance scenarios | `N/A — no new attack surface introduced; LOOPS/ship edits are documentation + checklist; the dogfood reuses M1–M4 surfaces already modelled (tm-secure-value-loop-abuse-1..4)` |
| Measurement deliverables | `N/A — not value-bearing (tooling/process), see §5A` |

#### Out of Scope / Must Not Do

- Do NOT make SBOM/provenance mandatory for non-release-artifact milestones.
- Do NOT have `/slo-ship` merge.
- Do NOT pick a large dogfood target — small/low-risk only.

#### BDD / E2E / DoD (condensed)

- **BDD**: happy path (each stage names a security output; `ship_state` enum closed; dogfood §5B filled), invalid input (SBOM forced on markdown → rejected as anti-pattern), empty state (a docs-only milestone → Bundle A + `ship_state: docs_only`), compatibility (existing loop entries + `/slo-ship` no-merge preserved), abuse (`N/A` per Contract Block).
- **E2E** (`svl_m5.rs`): `loops_engineering_names_security_output_per_stage`, `ship_has_secure_release_checklist_and_closed_ship_state`, `sbom_provenance_is_conditional`, `dogfood_secure_value_contract_is_filled_no_placeholders`.
- **Regression**: `/slo-ship` SHA-pinned tests pass (baseline updated); existing LOOPS structural tests pass.
- **DoD**: LOOPS docs name per-stage security outputs; `/slo-ship` has the checklist + `ship_state`; SBOM/provenance conditional; one real filled dogfood §5B exists; `svl_m5` + full suite green; lessons+completion; Tracker updated; proposal §11 adoption criteria reviewed in the retro.

---

## 18. Documentation Update Table

| Milestone | ARCHITECTURE/overview Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | note §5B shipped | — | test artifacts if any | `docs/SECURE-VALUE-LOOP.md` (new) |
| 2 | note `/slo-plan` requirement | — | — | — |
| 3 | note status enum + readiness gate + sldo-common fix | — | — | template status comment; `crates/sldo-common/CHANGELOG`/README (0.1.3, MilestoneStatus extension) |
| 4 | note ledger + bundle evidence | — | — | `slo-retro` issue-filing-discipline |
| 5 | note loop docs + ship checklist | `/slo-ship` capability if user-facing | — | `docs/LOOPS-ENGINEERING.md`, `docs/LOOPS-BUSINESS.md` |

---

## 19. Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, the additive/backward-compat constraint, the dual-template byte-identity rule, the no-new-taxonomy rule, the unknown-status→blocked fallback, any pinned SKILL.md SHA baseline to update, the required structural test, and the exact Definition of Done. Then list the smallest additive change that satisfies the contract without renumbering sections or forking vocabulary.

---

## 20. Source Basis

External proposal `~/Downloads/sunlit_orchestra_secure_value_loop(1).md`; research dossier/synthesis under `docs/slo/research/secure-value-loop/`; design under `docs/slo/design/secure-value-loop-*.md`; v4 template; the measurement-loop runbook as the precedent meta-change. External standards: OWASP Proactive Controls 2024, OWASP ASVS 5.0 / MASVS / API Security Top 10 (2023) / LLM Top 10 (2025), NIST SSDF SP 800-218 (+800-218A), SLSA/SBOM, OpenAI Symphony.
