# Outcome First Engineering (Outcome Validation gate) — SunLit Orchestra (AI-First Runbook v4)

> **Purpose**: elevate user outcomes to first-class, testable artifacts across the SLO loop — make BDD/E2E the *primary* Definition of Done so a milestone is complete only when the promised user outcome exists AND existing important outcomes still exist.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Code completion alone is insufficient. Prove the outcome, preserve the outcomes. Additive over breaking; evidence over claims; the highest-authority test is the one that proves the user got value.
> **How to use**: Work milestones M1→M5 sequentially. Each milestone leaves the loop dogfoodable. Never one-shot; never silently widen scope.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md), [docs/slo/design/outcome-first-overview.md](slo/design/outcome-first-overview.md), [docs/slo/design/outcome-first-interfaces.md](slo/design/outcome-first-interfaces.md), [docs/slo/design/outcome-first-threat-model.md](slo/design/outcome-first-threat-model.md), [docs/slo/design/outcome-first-code-map.md](slo/design/outcome-first-code-map.md), [docs/slo/templates/runbook-template_v_4_template.md](slo/templates/runbook-template_v_4_template.md).

---

## 0. How To Use This Template

1. Fill Runbook Metadata, Architecture, and Milestone Plan before implementation starts. (M1 done at authoring; M2–M5 contracts authored interactively per `/slo-plan` discipline.)
2. Work milestones M1→M5 sequentially.
3. Before each milestone, complete the Global Entry Protocol (§7).
4. During implementation, follow §4 (Carmack-Style Best Practices) and the milestone Contract Block literally.
5. After each milestone, complete the Global Exit Protocol (§8) and fill the Evidence Log.
6. Do not mark a milestone done until its Definition of Done is objectively satisfied.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `outcome-first` |
| Project name | `SunLit Orchestra` |
| Primary stack | Host-neutral Markdown skill pack (`skills/<name>/SKILL.md`) + the v4 Markdown template + one Rust structural-contract test family in the existing `sast-verify` xtask crate. No new crate, no service, no UI. |
| Primary package/app names | `skills/slo-plan`, `skills/slo-execute`, `skills/slo-verify`, `skills/slo-retro`, `skills/slo-critique`; `docs/slo/templates/runbook-template_v_4_template.md`; test crate `sast-verify` |
| Prefix for tests and lesson files | `outcome-first` (lessons: `docs/slo/lessons/outcome-first-m<N>.md`; tests: `xtasks/sast-verify/tests/outcome_first_m<N>_*.rs`) |
| Default unit test command | `cargo test -p sast-verify` |
| Default integration/BDD test command | `cargo test -p sast-verify outcome_first` |
| Default E2E/runtime validation command | `cargo test -p sast-verify outcome_first_m<N>` (the structural-contract test IS the runtime gate — there is no app to boot) |
| Default build/boot command | `N/A — Markdown skill pack + a Rust test; the "build" is the test crate compiling. Install smoke: \`cargo run -p sldo-install -- --dry-run\`` |
| Default formatter command | `cargo fmt -p sast-verify -- --check` |
| Default static analysis / lint command | `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Default dependency / security audit command | `cargo audit` (only if a dependency is added — none planned) |
| Default debugger or state-inspection tool | `cargo test -p sast-verify <name> -- --nocapture`; rust-analyzer |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- All existing v4 template sections **§1–§20** (incl. §5A, §5B) — additive insertions only; **no section renumbered or removed**. §5C inserts after §5B; §17 gains sub-sections; §11 gains a layer row; §6 gains a rule.
- The v3 template (`runbook-template_v_3_template.md`) — untouched historical artifact.
- `/slo-verify` command verb + `/slo-verify M<N>` argument shape; existing **Pass 1–6 content + resolution vocabulary** (`pass | not_applicable | waived_with_reason`). The new Outcome Validation pass is a **non-renumbering leading "Pass 0"** so existing `Pass 4/5/6` citations stay valid.
- `/slo-plan` §5A / §5B requirement-trigger logic (the new §5C requirement reuses the same trigger shape).
- `/slo-retro` existing refusal gates + lessons/completion file paths.
- `/slo-critique` four-persona structure + finding format.
- Threat-model frozen ID scheme `tm-<slug>-abuse-N`; the new `oc-<slug>-N` / `cuj-<slug>-N` schemes follow the same frozen-contiguous discipline.
- `discover_skills()` contract — **untouched** (no new skill directory; elevate-in-place).
- `xtasks/sast-verify` structural-contract gate stays green every milestone.

---

## 2. Milestone Tracker

This is the single source of truth for progress. Update as each milestone completes.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Template contract — v4 §5C + §17 sub-sections + §11 Outcome layer + §6 rule + ID schemes + structural test | `done` | 2026-06-17 | 2026-06-17 | `docs/slo/lessons/outcome-first-m1.md` | `docs/slo/completion/outcome-first-m1.md` |
| 2 | Plan enforcement — `/slo-plan` requires the outcome contract for value-bearing milestones | `done` | 2026-06-17 | 2026-06-17 | `docs/slo/lessons/outcome-first-m2.md` | `docs/slo/completion/outcome-first-m2.md` |
| 3 | Outcome Validation gate — `/slo-verify` Pass 0 (highest authority, non-renumbering) | `done` | 2026-06-17 | 2026-06-17 | `docs/slo/lessons/outcome-first-m3.md` | `docs/slo/completion/outcome-first-m3.md` |
| 4 | Close the loop — `/slo-retro` refusal gate + `## Outcome vs promise` + `/slo-execute` outcome-first + `/slo-critique` theatre review | `done` | 2026-06-17 | 2026-06-17 | `docs/slo/lessons/outcome-first-m4.md` | `docs/slo/completion/outcome-first-m4.md` |
| 5 | Principle + loop docs — Outcome First Engineering in operating-contract + catalog + LOOPS-ENGINEERING (inverted pyramid) | `done` | 2026-06-17 | 2026-06-17 | `docs/slo/lessons/outcome-first-m5.md` | `docs/slo/completion/outcome-first-m5.md` |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Honest exit states (additive, optional): human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk -->
<!-- Any consumer that does not recognise a status value MUST treat it as `blocked`. -->
<!-- Lessons files: docs/slo/lessons/outcome-first-m<N>.md -->
<!-- Completion summaries: docs/slo/completion/outcome-first-m<N>.md -->

---

## 3. End-to-End Architecture Diagram

### Architecture Diagram

```
┌────────────────────────────────────────────────────────────────────────────────┐
│            Outcome First Engineering — Outcome Validation gate                    │
│                                                                                  │
│  v4 TEMPLATE (the contract authors fill)              built in M1                 │
│    - - §5C Outcome Validation Contract  ──┐          (after §5B)                  │
│    - - §17 Outcome Scenarios   (oc-N)     │  required for value-bearing           │
│    - - §17 Critical User Journeys (cuj-N) │  milestones, enforced by /slo-plan    │
│    - - §17 Core Capability Regression Mx  │  (M2)                                 │
│    - - §11 Outcome test layer             │                                      │
│    - - §6  "Outcome outranks unit" rule  ─┘                                       │
│                       │                                                           │
│  /slo-ideate ─► /slo-research ─► /slo-architect ─► /slo-plan ─► /slo-critique     │
│                                       (enforce §5C, M2)   (theatre review, M4)    │
│                                                               │                   │
│                                                               ▼                   │
│  /slo-retro ◄──────────────── /slo-verify ◄────────────── /slo-execute           │
│  (REFUSE close while          ║ Pass 0: OUTCOME VALIDATION   (write outcome +     │
│   outcome/journey/regression   ║ = HIGHEST AUTHORITY  (M3)   journey tests first, │
│   row unproven, M4)            Pass 1..6 base pyramid (M4)                        │
│        │                       unchanged numbers                                  │
│        ▼                                                                          │
│  lessons ## Outcome vs promise (M4)                                               │
│                                                                                  │
│  AUTHORITY INVERSION:   OUTCOME  >  E2E  >  Integration  >  Unit                  │
│     1000 unit tests pass  +  1 outcome row fails   ⇒   milestone FAILS            │
│                                                                                  │
│  ── existing   - - - new/changed   ║ gate                                         │
│  Enforcement: xtasks/sast-verify/tests/outcome_first_m*_*.rs  (the runtime gate)  │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `runbook-template_v_4_template.md` | carry §5C + §17 sub-sections + §11 Outcome layer + §6 rule + `oc-`/`cuj-` ID schemes | changed | M1 | template sections |
| `outcome_first_m<N>_*.rs` | structural-contract test: section presence + edited SKILL.md SHA baselines + Pass-0 naming | new | M1–M5 | `cargo test -p sast-verify` |
| `skills/slo-plan` | require the outcome contract for value-bearing milestones; extend BDD-specificity to outcome-shape | changed | M2 | `/slo-plan` |
| `skills/slo-verify` | insert Pass 0 Outcome Validation (highest authority, non-renumbering) | changed | M3 | `/slo-verify M<N>` |
| `skills/slo-retro` | outcome refusal gate + `## Outcome vs promise` lessons section | changed | M4 | `/slo-retro M<N>` |
| `skills/slo-execute` | write Outcome Scenario + Critical Journey tests first | changed | M4 | `/slo-execute M<N>` |
| `skills/slo-critique` | flag vacuous/theatre outcome scenarios as `ask` | changed | M4 | `/slo-critique` |
| `references/agent/operating-contract.md`, `docs/skill-pack-catalog.md`, `docs/LOOPS-ENGINEERING.md` | register the Outcome First principle + inverted pyramid + gate | changed | M5 | Markdown |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Author the contract | milestone author (`/slo-plan`) | runbook §5C / §17 rows | Markdown authoring | yes — required only for value-bearing M; else `N/A — <reason>` | `/slo-plan` flags missing contract; legacy runbooks remain valid | M1, M2 |
| Run the outcome proof | `/slo-verify` Pass 0 | Evidence Log actuals | runtime test run (Playwright for UI) | yes — bounded to the milestone's declared oc-/cuj-/regression rows | Pass 0 fail ⇒ milestone fails regardless of other passes; regression-test-first flow on bug | M3 |
| Gate the close | `/slo-retro` | tracker `done` | refusal gate on unproven rows | yes — value-bearing M only | refuse close while any outcome/journey/regression row unproven or reasonless-waived | M4 |
| Enforce the contract shape | `outcome_first_m*_*.rs` | CI / `sast-verify gate` | Rust assertions over Markdown | yes — bounded file set | section absent / SHA drift ⇒ red gate | M1–M5 |

---

## 4. Carmack-Style Development Best Practices

The full §4.1–§4.8 rules of the v4 template apply. This is a Markdown-skill-pack + template + one Rust test family; the language-independent reads as:

- **§4.1 Inspect state, do not guess** — when a structural-test assertion fails, print the parsed frontmatter / the actual section text under `cargo test ... -- --nocapture` before editing the skill/template. "Inspect state" = read the actual installed file, never assume its shape.
- **§4.2 Static analysis is mandatory** — `cargo fmt -p sast-verify -- --check` + `cargo clippy -p sast-verify --all-targets -- -D warnings` every milestone.
- **§4.3 Assertions are executable comments** — the structural test's assertions ARE the executable invariants (section present, SHA pinned, Pass 0 named, resolution-vocabulary present, fence-rule present).
- **§4.4 Bounded resources** — N/A at runtime (no service); the test scans a bounded file set.
- **§4.5 Make invalid states unrepresentable** — the resolution column is a fixed enum (`pass | not_applicable | waived_with_reason`); frozen `oc-`/`cuj-`/`tm-` ID schemes; a value-bearing milestone cannot reach `done` with an unproven outcome row (§6 rule + retro gate).
- **§4.6 Preserve compatibility** — every edit is additive; no v4 section renumbered; Pass 0 is non-renumbering; legacy runbooks remain valid.
- **§4.7 Small, local, reviewable** — one template + at most three closely-related SKILL.md edits per milestone; SHA baselines move in lockstep with edits.
- **§4.8 No silent failure** — the gate is fail-closed: an unknown/blank resolution is treated as not-passing, never silently `done`.

---

## 5. High-Level Design for State Modeling / Formal Verification

`N/A — no concurrency, distributed state, leases, ordering, or recovery protocol.` The correctness properties (new sections present; no v4 section renumbered; SHA baselines pinned to edits; Pass 0 is highest authority and non-renumbering; resolution enum + frozen ID schemes; fail-closed gate) are *structural* properties of Markdown files + skill prose, enforced by the structural-contract test (the v4-sanctioned property/contract-test substitute for formal modeling on simple systems), not by TLC. `tla_required: false` (set by `/slo-architect`).

### 5.8 Kani proof obligations

`N/A — no Rust kernels. kani_required: false.` The only Rust is a structural-contract test (assertions over parsed Markdown), with no `unsafe`, no arithmetic kernel, no representation invariant worth a bounded proof.

---

## 5A. Measurement Contract

This runbook is **value-bearing**: it changes user-facing capability of the SLO loop (a new authority gate + new required contract sections that authors fill and skills enforce). There is **no runtime telemetry surface** — measurement is dogfooding, mirroring the innovation-loop runbook precedent.

| Field | Value |
|---|---|
| Value hypothesis | Outcome Validation catches "code works but the user outcome is absent / an adjacent outcome regressed" — the AI-assisted-development failure mode the old loop missed. |
| Review windows | **Mid-stream dogfood checkpoint after M3** (theme A / CEO-1 + ENG-3): run Pass 0 on one real value-bearing milestone with a deliberately-unproven outcome row and confirm `/slo-retro` refuses + Pass 0 reports fail — *before* committing M4/M5. Then the first 2–3 real feature runbooks after M5 ships. |
| Primary leading metric | ≥1 dogfood runbook authors §5C + Outcome Scenarios + Critical User Journeys + a Core Capability Regression Matrix, and `/slo-verify` Pass 0 runs them front-to-end. |
| Primary lagging metric | An Outcome Validation / regression-matrix failure blocks a milestone that the pre-change loop would have marked `done` (i.e. the gate changes an outcome at least once). |
| Guardrails | (1) no legacy v4 runbook invalidated (owner: Sherif); (2) `cargo test -p sast-verify` green every milestone (owner: Sherif); (3) the outcome contract never becomes heavier than the value it adds and never double-gates §5A/§5B (owner: Sherif). |
| Telemetry deliverables | No runtime telemetry (local OSS skill pack). The "saved query" is `grep` for the new template sections + the edited SKILL.md behaviors, plus a read of the first dogfood runbook's Pass 0 Evidence-Log rows + `/slo-retro` `## Outcome vs promise`. No behavioural events emitted. |
| Rollout plan | Ship M1→M3; **run the mid-stream dogfood acceptance checkpoint** (a real Pass 0 actually blocks a real milestone — the executable "the gate fires" proof, not just "the contract is documented"); then ship M4→M5; dogfood on the next real feature runbook immediately. |
| Diagnosis plan | If the leading metric misses, distinguish **too-heavy** (authors `N/A` everything = friction) from **too-light** (outcome scenarios are vacuous/single-`And` = theatre) from **wrong-placement** (Pass 0 is redundant with existing passes). Evidence: where authors stopped filling §5C/§17, and whether closed milestones carry real front-to-end Pass 0 actuals. |
| Experiment plan | Too-heavy → trim a §5C field. Too-light → strengthen the Front-to-End ordered-step requirement + the `/slo-critique` theatre gate. Wrong-placement → merge Pass 0 into Pass 1. |
| Privacy controls | Authored outcome/journey strings may carry PII → mandatory data-classification row + `/slo-verify` Pass 4 PII scan (detective second line) + `~~~text` fence in any generated security artifact. No personal data processed by the pack itself. |

Each value-bearing milestone names its slice in its Contract Block **Measurement deliverables** row. `/slo-retro` records actual-vs-thesis movement at M5 close.

---

## 5B. Secure Value and Security Contract

This runbook is **security-relevant**: it touches the agent-authoring surface (prompt-injection via authored strings), the CI structural-test gate, and the security-BDD discipline. Threat model: [docs/slo/design/outcome-first-threat-model.md](slo/design/outcome-first-threat-model.md) + [.slo.json](slo/design/outcome-first-threat-model.slo.json).

### Value Wedge

| Field | Value |
|---|---|
| Value hypothesis | A milestone cannot close as `done` while the promised user outcome is unproven or an adjacent outcome regressed. |
| Smallest valuable wedge | One dogfood milestone where Pass 0 runs an Outcome Scenario + Regression Matrix front-to-end and the close is gated on the result. |
| User-visible proof of value | An author sees `/slo-retro` refuse to close on an unproven outcome row. |
| Security-visible proof of safety | `cargo test -p sast-verify outcome_first` is green: new template sections present; edited SKILL.md SHA baselines pinned; the `~~~text` fence-rule + no-control-field-selection note present in §5C/§17. |
| What would make this wedge too small to matter? | If Pass 0 can be satisfied by a single-layer or mock-only assertion (theatre) — then the gate proves nothing the old loop didn't. |

### Security Definition of Ready (Operator Readiness)

| Prerequisite | Owner (`human \| agent \| upstream`) | Needed by | Validation (executable proof) | Status (`ready \| partially_ready \| blocked`) |
|---|---|---|---|---|
| `sast-verify` baseline green before adding a test | agent | M1 | `cargo test -p sast-verify` exits 0 | ready |
| SHA-baseline mechanism understood before editing any SKILL.md | agent | M2 | read `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` baseline constant + helper | ready |
| Every repo-wide `Pass 4/5/6` citation enumerated before touching `/slo-verify` | agent | M3 | `rg -n "Pass [0-9]"` over `skills/` + `docs/` lists all citations; Pass 0 chosen to avoid renumber | partially_ready |
| `discover_skills()` confirmed to need no change (no new skill dir) | agent | M2 | `cargo run -p sldo-install -- --dry-run` skill count unchanged by this runbook | ready |

`safe_to_continue_without_blockers: true`

### Threat Model Summary

| Area | Summary |
|---|---|
| Assets | the integrity of the Definition-of-Done gate; authored outcome/journey content; the SLO loop's trustworthiness ("done means done") |
| Actors | the milestone author; the LLM agent authoring outcome tests; a crafted authored string |
| Trust boundaries | author/agent vs. the gate's verdict; authored descriptive text vs. control fields (ids/resolution verbs/gate outcomes) |
| Entry points | authored §5C/§17 strings; the Regression Matrix resolution column; the `/slo-retro` close gate |
| Abuse cases | `tm-outcome-first-abuse-1` (content-injection via authored strings), `tm-outcome-first-abuse-2` (outcome-test theatre / green-but-vacuous), `tm-outcome-first-abuse-3` (regression-matrix / gate gaming), `tm-outcome-first-abuse-4` (weakening the critique SHA-pin test — added from `/slo-critique` SEC-1) |
| Required controls | `~~~text` fence + no-control-field-selection for authored strings; ordered cross-layer Front-to-End steps; resolution enum never-blank; Pass 0 runtime re-run; fail-closed retro gate; `/slo-critique` theatre review |
| Residual risks | author can still hand-author a thin/dishonest contract (R1); SLO ships discipline not a runner (R2); pasted PII is detective-not-preventive (R3) — owner: Sherif; review by 2026-09-17 |

### Security Test Plan

| Test | Required? | Command/tool | Evidence path | Waiver if not applicable |
|---|---|---|---|---|
| SAST | yes | `cargo clippy -p sast-verify --all-targets -- -D warnings` | per-milestone Evidence Log | — |
| SCA/dependency audit | not_applicable | — | — | no new dependency added |
| Secrets scan | yes | the `outcome_first_m1` test's `~~~text`-fence + PII/secret-pattern assertion over the template's authored example rows | M1 Evidence Log | — |
| IaC scan | not_applicable | — | — | no infrastructure |
| Container/image scan | not_applicable | — | — | no container |
| DAST/API security | not_applicable | — | — | no network service |
| Authn/authz negative tests | not_applicable | — | — | the methodology *produces* security-BDD rows; it has no auth surface of its own (covered by abuse-case tests) |
| Abuse-case tests | yes | BDD `abuse case` rows per milestone (injection string, theatre scenario, matrix-gaming) citing `tm-outcome-first-abuse-N` | per-milestone BDD | — |
| Privacy/telemetry tests | yes | M1 PII-pattern + data-classification-presence assertions over authored example rows | M1 Evidence Log | — |
| Fuzz/property/formal tests | partially | the structural-contract test is the property test (section presence, SHA pin, enum vocab, fence rule) | per-milestone test | full fuzzing N/A — no parser surface |

### Detected Work Ledger

| ID | Finding | Severity | Disposition | Owner | Evidence/link | Due |
|---|---|---:|---|---|---|---|
| DW-001 | Re-indexing `/slo-verify` pass numbers would break repo-wide `Pass 4/5/6` citations (reversibility D5 seam). Mitigation chosen at planning: implement Outcome Validation as a non-renumbering **Pass 0**. | medium | `fix_now` | agent | M3 Contract Block "Important design rule" | M3 |
| DW-002 | Pre-existing `cargo clippy -p sast-verify --all-targets -- -D warnings` debt (3 errors), present at baseline, OUTSIDE every outcome-first allow-list: `tests/sap_imp_m3_standards.rs:274` (regex-in-loop), `src/tier_detect.rs:28` (unused `Public` variant), `src/yaml_schema.rs:20` (never-read fields). My new `outcome_first_m*` test files are clippy-clean. Same debt the innovation-loop runbook recorded as its DW-001. | low | `file_github_issue` | Sherif | M1 Evidence Log "Static analyzer" row | `/slo-retro` files via `slo-process` lane |
| DW-003 | Plan/critique (theme C / ENG-4) assumed `slo-critique/SKILL.md` is pinned by TWO baseline constants. Reality discovered in M4: there is ONE constant (`CRITIQUE_SKILL_SHA256` in `sap_imp_m5_agents.rs`); `slo_tm_m2_consumers.rs::feng6_sha_constant_in_lockstep` DERIVES it by regex (no second constant). So there is no two-constant half-update risk; M4 bumped only the single constant and did NOT edit `slo_tm_m2_consumers.rs` (allow-listed but unneeded). The `outcome_first_m4` cross-check was corrected to assert single-source consistency + that slo_tm_m2 keeps deriving. | low | `fix_now` | agent | M4 Evidence Log "Critique baseline updated" row + `outcome_first_m4_consumers.rs` doc comment | done in M4 |
| DW-004 | M2's §5C additions pushed `skills/slo-plan/SKILL.md` to 82 lines, breaking the pre-existing ≤80-line cap in `crates/sldo-install/tests/e2e_eng_imp_m4.rs::slo_plan_skill_md_decomposed`. Missed per-milestone because only the `sast-verify` half of the runbook baseline was run, not the `sldo-*` half (the cap test lives in `sldo-install`). Caught at ship-readiness by the full baseline. Fixed: compressed the §5C requirement to a bold-lead paragraph + trimmed the Contract Block sentinel → 80 lines; re-pinned the M2 SHA (`337581d…`). Process fix: run BOTH baseline halves per milestone. | medium | `fix_now` | agent | ship-readiness full-baseline run + M2 re-pin | done |

---

## 6. Global Execution Rules

§6.1–§6.11 of the v4 template apply verbatim. Project-specific reads:

- **Scope (§6.1)**: change only files in the current milestone's allow-list. Editing a SKILL.md REQUIRES updating its `sast-verify` SHA baseline in the same milestone (and vice-versa) — they move in lockstep.
- **Tests define the contract (§6.2)**: the milestone's structural-contract test is written and confirmed failing (section absent / old baseline) before the template/SKILL.md edit.
- **Static analysis (§6.5)**: `cargo fmt --check` + `cargo clippy -D warnings` on the test crate every milestone.
- **No placeholders (§6.7)**: no "TODO: fill later" in a shipped template section or SKILL.md; a section ships complete or not at all.
- **Backward compatibility (§6.8)**: every milestone confirms no v4 section was renumbered, the v3 template is untouched, and legacy runbooks/skills still parse.

---

## 7. Global Entry Rules (Pre-Milestone Protocol)

Follow §7 of the v4 template. Key per-milestone reads:

1. Read the prior milestone's lessons file and apply corrections.
2. (`/slo-execute` Step 1.5) surface open `retro-derived` issues for prefix `outcome-first`.
3. Read the current milestone fully.
4. Run the baseline: `cargo test -p sldo-common -p sldo-install -p sldo-research && cargo test -p sast-verify`. Do not start on red.
5. Read the allow-listed files + the exemplars in [outcome-first-code-map.md](slo/design/outcome-first-code-map.md).
6. Set the tracker row to `in_progress`.
7. Write the structural-contract test FIRST (it fails: section/behavior/baseline absent).
8. (No E2E app stub — the structural test is the runtime gate.)
9. Copy the Evidence Log template into working notes.
10. Re-state milestone constraints in your own words before authoring.

---

## 8. Global Exit Rules (Post-Milestone Protocol)

Follow §8 of the v4 template. Key per-milestone reads: run fmt + clippy + the full `sast-verify` suite; confirm the new structural assertions pass and pre-existing ones stay green; confirm no v4 section renumbered and v3 untouched; update SHA baselines in lockstep with edits; `git status` clean of untracked artifacts; write the lessons + completion files; update the tracker.

§9–§16 of the v4 template (Background, Carry-forward, BDD rules, Dependency policy, Evidence Log template, Self-Review Gate, Lessons/Completion templates) apply verbatim and are not restated here.

---

## 17. Milestone Plan

### Milestone 1 — `Template contract: v4 §5C + §17 sub-sections + §11 Outcome layer + §6 rule + ID schemes + structural test`

**Goal**: the v4 template carries the full Outcome-First contract surface (a §5C Outcome Validation Contract, §17 Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix sub-sections, a §11 Outcome test layer, a §6 "Outcome outranks unit" rule, and the `oc-`/`cuj-` ID schemes), pinned by a structural-contract test — so every future runbook *can* express outcomes as the primary Definition of Done.

**Context**: `docs/slo/templates/runbook-template_v_4_template.md` today has BDD Acceptance Scenarios (§11.3, §17), E2E Runtime Validation (§11.6, §17), Regression Tests, and a Definition of Done (§17) — but it has no *outcome-shaped* contract, no cross-capability regression matrix, and no authority inversion. This milestone adds those as **additive, optional sections** following the exact `> **Optional section.** Legacy runbooks ... remain valid` posture of §5A (line ~316) and §5B (line ~341). No section is renumbered.

**Carmack-style reliability goal**: make invalid states unrepresentable (fixed resolution enum + frozen ID schemes + "outcome row unproven ⇒ not done" §6 rule) and static-analysis-enforced structure (the new test pins section presence + the fence rule).

**Important design rule**: the v4 template exists as **two byte-identical copies** — the skill-primary `skills/slo-plan/references/runbook-template_v_4_template.md` (the one `/slo-plan` actually uses) and the repo mirror `docs/slo/templates/runbook-template_v_4_template.md`. The existing `svl_m1.rs::template_copies_stay_byte_identical()` test ENFORCES they stay byte-identical, so **every M1 edit must be applied identically to both copies**. Copy the §5A/§5B optional-section blockquote **verbatim** for §5C; insert §5C *after* §5B (do not renumber §6+); §17 sub-sections are inserted into the milestone template alongside the existing BDD Acceptance Scenarios table (not replacing it — the existing table stays as lower-authority detail). Every authored example string in the new sections is rendered as plain Markdown body and the sections carry the `~~~text` fence instruction for user-supplied strings.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the locked design (`docs/slo/design/outcome-first-*`); the current v4 template |
| Outputs | edited v4 template with §5C + §17 sub-sections + §11 layer + §6 rule + ID-scheme definitions; new structural-contract test |
| Interfaces touched | v4 template sections (additive); new `oc-<slug>-N` / `cuj-<slug>-N` ID schemes; `sast-verify` test surface |
| Files allowed to change | `skills/slo-plan/references/runbook-template_v_4_template.md` (skill-primary) **and** `docs/slo/templates/runbook-template_v_4_template.md` (mirror) — edited identically; `xtasks/sast-verify/tests/outcome_first_m1_template.rs` (NEW); `.gitignore` (only if a new artifact pattern is needed) |
| Files to read before changing anything | both v4 template copies (§5A/§5B/§6/§11/§17), `xtasks/sast-verify/tests/svl_m1.rs` (the byte-identical sync test), `docs/slo/design/outcome-first-interfaces.md`, `docs/slo/design/outcome-first-code-map.md`, `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (test-shape exemplar) |
| New files allowed | `xtasks/sast-verify/tests/outcome_first_m1_template.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | every existing v4 section §1–§20 still present + not renumbered; v3 template untouched; the two v4 copies stay byte-identical (`svl_m1` green); a legacy v4 runbook with no §5C/§17-outcome rows still satisfies the template (sections are optional) |
| Resource bounds introduced/changed | `N/A — Markdown sections + a bounded-file-set test` |
| Invariants/assertions required | test asserts (against the skill-primary copy): §5C heading present; §5C Front-to-End steps are **per-layer `applicable \| not_applicable(reason)`** and the **"≥1 real cross-layer assertion"** rule text is present (theme B); §17 Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix sub-headings present; §11 Outcome layer row present; §6 "Outcome outranks unit" rule present; resolution enum string `pass \| not_applicable \| waived_with_reason` present in the matrix; `~~~text` fence-rule note present; §5A/§5B/§6 headings still present (no renumber); v3 template has NO §5C (backward-compat). The two copies' byte-identity is covered by the existing `svl_m1::template_copies_stay_byte_identical` (must stay green) |
| Debugger / inspection expectation | on assertion failure, print the matched/!matched section text under `--nocapture` before editing |
| Static analysis gates | `cargo fmt -p sast-verify -- --check`; `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Exemplar code to copy | the §5A/§5B optional-section blockquote + the §5B Bundle resolution-enum row (`docs/slo/templates/runbook-template_v_4_template.md` lines ~316, ~341, ~383); the test shape in `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (frontmatter/section extraction + assertion-collection loop) |
| Anti-exemplar code not to copy | do NOT mint a "v5" template or renumber any v4 section; do NOT add a `skills/slo-outcome/` directory (per code-map anti-exemplars) |
| Refactoring discipline | `N/A — no refactoring; additive insertions + one new test file` |
| AI tolerance contract | `N/A — no AI component` (Markdown + a deterministic Rust test) |
| Forbidden shortcuts | no "TODO" sections; no renumber; no SKILL.md edits in this milestone (template-only); no blank resolution cells in the example matrix |
| Data classification | `Internal` |
| Proactive controls in play | `C4 Address Security from the Start` (the fence rule + no-control-field-selection baked into the section design); `C5 Validate All Inputs` (authored strings rendered as descriptive body only) |
| Abuse acceptance scenarios | `tm-outcome-first-abuse-1` — authored §5C/§17 string injection; mitigation = `~~~text` fence + no-control-field-selection note, asserted by the M1 test (BDD row below) |
| Measurement deliverables | the new template sections + the M1 structural test (the "saved query" = `grep` for the §5C/§17 headings); readout at M5 dogfood; guardrail owner: Sherif. Ties to §5A leading metric. |

#### Out of Scope / Must Not Do

- No SKILL.md edits (those are M2–M5).
- No `/slo-verify` pass changes, no `/slo-plan` enforcement (M2/M3).
- No renumbering of any existing v4 section; no v3 edits.
- No new skill directory.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-plan/references/runbook-template_v_4_template.md` | Insert §5C Outcome Validation Contract (after §5B) — its **Front-to-End Validation steps are PER-LAYER `applicable \| not_applicable(reason)`** (seed→backend→persisted→API/IPC→UI) with **≥1 real cross-layer assertion required** for value-bearing milestones, never a single mock (theme B / ENG-2 + SEC-2); add §17 sub-sections (Outcome Scenarios with `oc-` IDs, Critical User Journeys with `cuj-` IDs, Core Capability Regression Matrix with the resolution enum); add §11 Outcome test-layer row + narrative; add §6 "Outcome outranks unit" rule; add Definition-of-Done lines referencing outcome rows; add Contract Block rows (Outcome Validation deliverables / Critical journeys) |
| `docs/slo/templates/runbook-template_v_4_template.md` | Apply the **identical** edits (the byte-identical mirror; `svl_m1` enforces parity) |
| `xtasks/sast-verify/tests/outcome_first_m1_template.rs` | NEW: assert all the above are present + the no-renumber/back-compat invariants |
| `.gitignore` | review only; add a pattern only if a new generated artifact appears |

#### Step-by-Step

1. Write `outcome_first_m1_template.rs` FIRST with all assertions; confirm it fails (sections absent).
2. In the **skill-primary** copy: copy the §5A/§5B optional-section blockquote verbatim; author §5C after §5B.
3. Add the §17 sub-sections (Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix) with the resolution enum + `oc-`/`cuj-` ID notes + the `~~~text` fence instruction.
4. Add the §11 Outcome test-layer row + narrative; add the §6 "Outcome outranks unit" rule; extend the §17 Definition of Done.
5. Apply the **identical** diff to the repo-mirror copy (`docs/slo/templates/...`); confirm `diff` of the two copies is empty.
6. Run fmt + clippy; make the M1 test green; confirm the existing template-sync tests (`svl_m1`, `svl_m3`, `mloop_m3_plan`) stay green.
7. Verify no v4 section was renumbered and the v3 template is untouched.
8. `git status` clean; review `.gitignore`; complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Outcome-First v4 template contract**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Sections present | happy path | the edited v4 template | the M1 test runs | §5C + §17 Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix + §11 Outcome layer + §6 rule are all present |
| Resolution enum encoded | happy path | the Core Capability Regression Matrix | the M1 test runs | the `pass \| not_applicable \| waived_with_reason` enum string is present and "never blank" is stated |
| No renumber | backward compat | the edited v4 template | the M1 test runs | §5A, §5B, §6, §11, §17 headings all still present; §5C is an insertion after §5B |
| v3 untouched | backward compat | the v3 template | the M1 test runs | v3 has NO §5C heading |
| Authored-string injection | abuse case (`tm-outcome-first-abuse-1`) | an author pastes Markdown/YAML metacharacters into an Outcome statement | the section is rendered/consumed | §5C/§17 carry the `~~~text` fence instruction + the "authored text never selects control fields" note; the M1 test asserts the note is present |
| Per-layer front-to-end (no-UI target) | partial failure / theatre (theme B) | a library/CLI milestone with no UI layer | an author fills §5C Front-to-End | each layer is `applicable \| not_applicable(reason)` and ≥1 real cross-layer assertion (e.g. backend→persisted) is still required — a whole-row N/A or single mock is non-conformant (M1 test asserts the rule text) |
| Empty/first-run | empty state | a brand-new runbook from the template | an author leaves outcome sections unfilled | the sections are optional; the template still parses (no test failure for absence in a consumer) |

#### Regression Tests

- All existing `sast-verify` tests still pass (`cargo test -p sast-verify`).
- The existing **template-sync tests** stay green: `svl_m1::template_copies_stay_byte_identical`, plus the byte-identical checks in `svl_m3` and `mloop_m3_plan`.
- The existing template-marker tests (`mloop_m3_plan` §5A, `svl_m3` §5B markers) still pass — §5C is additive, removes nothing.
- `cargo run -p sldo-install -- --dry-run` skill count unchanged.

#### Compatibility Checklist

- [ ] Every existing v4 section §1–§20 still present and not renumbered
- [ ] The two template copies are byte-identical (`diff` empty; `svl_m1`/`svl_m3`/`mloop_m3_plan` green)
- [ ] v3 template byte-unchanged
- [ ] A legacy v4 runbook without outcome sections still satisfies the template
- [ ] `sast-verify` baseline green

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/outcome_first_m1_template.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `template_has_outcome_first_sections` | the contract surface exists | all §5C/§17/§11/§6 anchors found in the skill-primary copy |
| `template_sections_not_renumbered` | additive-only | §5A/§5B/§6/§11/§17 anchors all present; §5C after §5B |
| `template_carries_fence_rule` | anti-injection (`tm-outcome-first-abuse-1`) | the `~~~text` fence + no-control-field note present |
| `both_template_copies_have_5c` | dual-copy parity | the mirror copy also contains §5C (belt-and-braces beyond `svl_m1`'s byte check) |
| `v3_template_untouched_no_5c` | back-compat | v3 has no §5C |

#### Smoke Tests

- [ ] `cargo test -p sast-verify outcome_first_m1` passes
- [ ] `rg -n "Outcome Validation Contract|Outcome Scenarios|Critical User Journeys|Core Capability Regression Matrix" skills/slo-plan/references/runbook-template_v_4_template.md` shows the new headings
- [ ] `diff skills/slo-plan/references/runbook-template_v_4_template.md docs/slo/templates/runbook-template_v_4_template.md` is empty
- [ ] `cargo clippy -p sast-verify --all-targets -- -D warnings` clean
- [ ] `git status` shows no untracked test artifacts

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 29 suites, 0 failed | Pass | |
| Test created (fails) | `outcome_first_m1_template.rs` | fails: sections absent | 7 failed / 1 pass (v3-untouched correctly passed) for the right reason | Pass | BDD-first confirmed |
| Implementation | §5C/§17/§11/§6.12 edits in BOTH template copies | contract present in both | edited mirror; `cp` → skill-primary | Pass | |
| Copies identical | `diff` skill-primary vs mirror | empty | IDENTICAL | Pass | |
| Formatter | `cargo fmt -p sast-verify -- --check` | clean | exit 0 after formatting the new test file | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | new test file clippy-clean; 3 PRE-EXISTING bin/test errors outside allow-list | Pass (scoped) | → DW-002 `file_github_issue` |
| Full tests | `cargo test -p sast-verify` | green (incl. svl_m1/svl_m3/mloop_m3_plan sync) | 30 suites, 0 failed | Pass | M1 test = +1 suite |
| No-renumber check | `template_5c_after_5b_no_renumber` | all present; §5C between §5B and §6 | green | Pass | |
| v3 untouched | `v3_template_untouched_no_5c` | v3 has no §5C | green | Pass | |
| Fence-rule present | `template_carries_authored_string_fence_rule` | `~~~text` + no-control-field present | green | Pass | tm-outcome-first-abuse-1 |
| Test artifact cleanup | `git status` | clean | only allow-listed files + planning artifacts | Pass | |
| .gitignore review | review `.gitignore` | current | no new generated artifacts | Pass | |

#### Definition of Done

- §5C + §17 Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix + §11 Outcome layer + §6 rule present in BOTH template copies, with `oc-`/`cuj-` ID schemes + resolution enum + fence rule
- the two template copies are byte-identical (`svl_m1`/`svl_m3`/`mloop_m3_plan` green)
- `outcome_first_m1_template.rs` green; full `sast-verify` suite green
- no v4 section renumbered; v3 untouched; legacy runbooks remain valid
- fmt + clippy clean; `git status` clean
- lessons + completion files written; tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: no change yet (reality-first; the loop ships across milestones — update at M5/ship).
- **Other docs**: none in M1 (catalog/loops are M5).

#### Notes

- Concurrency / resource-bound BDD categories are `N/A — Markdown sections + a bounded-file-set test`.

---

### Milestone 2 — `Plan enforcement: /slo-plan requires the outcome contract for value-bearing milestones`

**Goal**: `/slo-plan` requires §5C + §17 Outcome Scenarios + Critical User Journeys + Core Capability Regression Matrix for every value-bearing milestone (a peer of the existing §5A/§5B requirement, same deterministic trigger), and extends its BDD-specificity gate to reject vacuous/single-`And` outcome scenarios — so a runbook *cannot be authored* without the outcome contract when the work is value-bearing.

**Context**: `skills/slo-plan/SKILL.md` already carries a "Measurement Contract requirement" block (§5A) and a "Secure Value & Security Contract requirement" block (§5B), both keyed off the deterministic "value-bearing" definition (introduces/changes user-facing capability; excludes refactor/docs/test-only) and both **forward-looking** (flag a gap; never invalidate legacy runbooks). This milestone adds a parallel "Outcome Validation Contract requirement" block and a new authoring reference, reusing that exact trigger + backward-compat posture. It also tightens the existing "Refuse when … BDD is generic" gate to cover outcome-shape (an Outcome Scenario with no follow-on `And` / no observable user outcome is "generic").

**Carmack-style reliability goal**: make invalid states unrepresentable at authoring time — a value-bearing runbook with no outcome contract, or a vacuous outcome scenario, is refused by `/slo-plan` rather than silently produced.

**Important design rule**: reuse the §5A/§5B trigger + forward-looking posture **verbatim in shape**; do NOT invent a new "value-bearing" definition. The M2 structural test asserts the new behavior via stable marker substrings **and** pins the edited `slo-plan/SKILL.md` with a **SHA-256 byte-identical baseline** (the `sap_imp_m5_agents.rs` pattern). Per founder direction (2026-06-17), the orchestration framework matures slowly and is edited rarely, so SHA pinning is the standard tamper-evident baseline for every edited SKILL.md in this runbook; the baseline constant moves in lockstep with the edit, inside the same milestone's allow-list.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — only to fold the §5C requirement into the existing requirement-block prose coherently; cite [`skills/slo-plan/references/refactoring-discipline.md`](../skills/slo-plan/references/refactoring-discipline.md).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the M1 v4 template (§5C/§17 sections now exist); `docs/slo/design/outcome-first-interfaces.md` |
| Outputs | edited `skills/slo-plan/SKILL.md` with an Outcome Validation Contract requirement block + tightened specificity gate; a new authoring reference; the M2 structural test |
| Interfaces touched | `/slo-plan` requirement logic (additive block); new reference file; `sast-verify` test surface |
| Files allowed to change | `skills/slo-plan/SKILL.md`; `skills/slo-plan/references/outcome-validation-contract.md` (NEW); `xtasks/sast-verify/tests/outcome_first_m2_plan.rs` (NEW) |
| Files to read before changing anything | `skills/slo-plan/SKILL.md` (the §5A/§5B requirement blocks + the "Gates" + "BDD includes …" lines), `skills/slo-plan/references/secure-value-contract.md` (shape exemplar for the new reference), `docs/slo/design/outcome-first-interfaces.md`, `docs/slo/design/outcome-first-code-map.md`, `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` |
| New files allowed | `skills/slo-plan/references/outcome-validation-contract.md`, `xtasks/sast-verify/tests/outcome_first_m2_plan.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | legacy runbooks without §5C/outcome sections remain valid (`/slo-plan` *flags*, never invalidates); §5A/§5B requirement logic unchanged; `/slo-plan` command verb + interactive-authoring flow unchanged; `discover_skills()` count unchanged |
| Resource bounds introduced/changed | `N/A — skill prose + a bounded-file-set test` |
| Invariants/assertions required | test asserts: SKILL.md contains the Outcome Validation Contract requirement clause keyed to "value-bearing"; SKILL.md states the forward-looking/flag-not-invalidate posture; the specificity gate covers vacuous/single-`And` outcome scenarios; the new reference file exists and contains the **per-layer Front-to-End steps + the "≥1 real cross-layer assertion" rule** (theme B) + the never-blank resolution enum + the anti-theatre rule + the `oc-`/`cuj-` ID rules; **`slo-plan/SKILL.md` SHA-256 equals the pinned baseline constant** (byte-identical baseline, per founder SHA-pin direction) |
| Debugger / inspection expectation | on assertion failure, print the actual SKILL.md slice / reference-file slice under `--nocapture` before editing |
| Static analysis gates | `cargo fmt -p sast-verify -- --check`; `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Exemplar code to copy | the §5A "Measurement Contract requirement" + §5B "Secure Value & Security Contract requirement" blocks in `skills/slo-plan/SKILL.md` (trigger + forward-looking posture); `skills/slo-plan/references/secure-value-contract.md` (reference-file shape) |
| Anti-exemplar code not to copy | do NOT introduce a new/divergent "value-bearing" definition; do NOT make the requirement retroactive on legacy runbooks; do NOT edit the SKILL.md without moving its SHA baseline in the same milestone |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md`: behavior-preserving microsteps, the existing slo-plan tests green before and after |
| AI tolerance contract | `N/A — no AI component` (skill prose + a deterministic Rust test) |
| Forbidden shortcuts | no new value-bearing definition; no retroactive invalidation; SKILL.md edit without a lockstep SHA-baseline update; no TODO prose in the shipped SKILL.md |
| Data classification | `Internal` |
| Proactive controls in play | `C4 Address Security from the Start` (the requirement forces the outcome+security+reliability rows up front); `C5 Validate All Inputs` (the specificity gate rejects vacuous scenarios) |
| Abuse acceptance scenarios | `tm-outcome-first-abuse-2` — vacuous/theatre outcome scenario; mitigation = the tightened specificity gate + the reference's anti-theatre rules (BDD row below) |
| Measurement deliverables | the requirement clause + the M2 test (saved query = `grep` for the requirement marker in the SKILL.md); readout at M5 dogfood; guardrail owner: Sherif. Ties to §5A leading metric. |

#### Out of Scope / Must Not Do

- No `/slo-verify`, `/slo-execute`, `/slo-retro`, `/slo-critique` edits (M3/M4).
- No template edits (M1 owns the template).
- No change to the §5A/§5B requirement logic or the "value-bearing" definition.
- No new skill directory.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-plan/SKILL.md` | Add an "Outcome Validation Contract requirement" block (peer to §5A/§5B, same trigger + forward-looking posture); add a pointer to the new reference; tighten the "Gates"/"BDD includes …" lines so vacuous/single-`And` outcome scenarios are refused |
| `skills/slo-plan/references/outcome-validation-contract.md` | NEW: how to author §5C — **per-layer `applicable \| not_applicable(reason)` Front-to-End steps with ≥1 real cross-layer assertion** (theme B), `oc-`/`cuj-` frozen ID rules, never-blank resolution enum, security-BDD rows cite `tm-<slug>-abuse-N`, reliability-BDD rows, anti-theatre rules |
| `xtasks/sast-verify/tests/outcome_first_m2_plan.rs` | NEW: marker-substring assertions over the SKILL.md + the new reference file |

#### Step-by-Step

1. Write `outcome_first_m2_plan.rs` FIRST (markers absent ⇒ fails); confirm baseline `cargo test -p sast-verify` green first.
2. Author `references/outcome-validation-contract.md` (ordered front-to-end steps + ID rules + never-blank enum + anti-theatre rules).
3. Add the Outcome Validation Contract requirement block to `slo-plan/SKILL.md` (copy §5A/§5B trigger + forward-looking shape) + the reference pointer.
4. Tighten the specificity gate lines to cover outcome-shape.
5. Run fmt + clippy; make the M2 test green.
6. Confirm the existing slo-plan-related tests + full `sast-verify` suite still green.
7. `git status` clean; `.gitignore` review.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: /slo-plan outcome-contract enforcement**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Require for value-bearing | happy path | a value-bearing milestone being authored | `/slo-plan` runs | the SKILL.md requires §5C + Outcome Scenarios + Critical User Journeys + Regression Matrix (test asserts the requirement clause present) |
| N/A for non-value-bearing | empty state | a docs-only / refactor milestone | `/slo-plan` runs | §5C may be marked `N/A — <reason>`; no false block (SKILL.md states the N/A path) |
| Reject vacuous outcome scenario | abuse case (`tm-outcome-first-abuse-2`) | an Outcome Scenario with no follow-on `And` / no observable user outcome | `/slo-plan` specificity gate runs | it is refused as generic (SKILL.md gate language asserts this) |
| Legacy not invalidated | backward compat | a legacy v4 runbook with no §5C | `/slo-plan` reads it | it remains valid; `/slo-plan` flags the gap forward-looking, does not reject the artifact |
| Reference completeness | happy path | the new reference file | the M2 test runs | it contains ordered Front-to-End steps + never-blank enum + `oc-`/`cuj-` rules + anti-theatre rules |

#### Regression Tests

- Full `sast-verify` suite green (incl. M1's `outcome_first_m1_template.rs`).
- The §5A/§5B requirement language in slo-plan is unchanged (test/diff check).
- `cargo run -p sldo-install -- --dry-run` skill count unchanged.

#### Compatibility Checklist

- [ ] §5A/§5B requirement logic + "value-bearing" definition unchanged
- [ ] Legacy runbooks without §5C still valid
- [ ] `/slo-plan` interactive-authoring flow + command verb unchanged
- [ ] `sast-verify` suite green

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/outcome_first_m2_plan.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `plan_requires_outcome_contract_for_value_bearing` | enforcement exists | requirement-clause marker present, keyed to "value-bearing" |
| `plan_specificity_gate_covers_outcome_shape` | anti-theatre at authoring (`tm-outcome-first-abuse-2`) | the gate phrase covering vacuous/single-`And` outcome scenarios present |
| `plan_forward_looking_not_retroactive` | back-compat | the flag-not-invalidate phrase present |
| `plan_skill_md_sha_pinned` | tamper-evident baseline | `slo-plan/SKILL.md` SHA-256 == pinned baseline constant |
| `outcome_validation_contract_reference_complete` | the how-to exists | reference file contains per-layer front-to-end + ≥1-cross-layer-assertion + enum + ID + anti-theatre markers |

#### Smoke Tests

- [ ] `cargo test -p sast-verify outcome_first_m2` passes
- [ ] `rg -n "Outcome Validation Contract requirement" skills/slo-plan/SKILL.md` shows the block
- [ ] `cargo clippy -p sast-verify --all-targets -- -D warnings` clean
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 30 suites, 0 failed | Pass | |
| Test created (fails) | `outcome_first_m2_plan.rs` | fails: markers absent | 4 failed for the right reason (forward-looking pre-satisfied by §5A/§5B) | Pass | BDD-first confirmed |
| Reference authored | `references/outcome-validation-contract.md` | complete | per-layer + cross-layer + enum + ids + anti-theatre | Pass | |
| Implementation | slo-plan requirement block + gate | present | §5C requirement section + Contract Block sentinel + tightened Gates/BDD | Pass | |
| Formatter | `cargo fmt -p sast-verify -- --check` | clean | exit 0 | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | new test clippy-clean; pre-existing debt = DW-002 | Pass (scoped) | |
| Full tests | `cargo test -p sast-verify` | green | 31 suites, 0 failed | Pass | M2 test = +1 suite |
| §5A/§5B unchanged | mloop_m3_plan + svl_m2/m3 markers green | unchanged | reader tests green | Pass | trigger reused, not changed |
| SHA baseline pinned | M2 test `plan_skill_md_sha_pinned` | SKILL.md SHA == baseline (moved in lockstep) | `337581d…` pinned (re-pinned at ship after the DW-004 ≤80-line fix); green | Pass | founder SHA-pin direction |
| Test artifact cleanup | `git status` | clean | only allow-listed files | Pass | |
| .gitignore review | review `.gitignore` | current | no new artifacts | Pass | |

#### Definition of Done

- `/slo-plan` requires the outcome contract for value-bearing milestones (peer to §5A/§5B, same trigger), with the new reference complete
- the specificity gate refuses vacuous/single-`And` outcome scenarios
- legacy runbooks remain valid (forward-looking flag, not retroactive)
- the edited `slo-plan/SKILL.md` SHA-256 is pinned to its lockstep baseline in the M2 test
- `outcome_first_m2_plan.rs` green; full `sast-verify` suite green; fmt + clippy clean; `git status` clean
- lessons + completion files written; tracker updated

#### Post-Flight

- **Other docs**: none in M2 (catalog/loops are M5; the reference lives under the skill).

#### Notes

- Concurrency / resource-bound BDD categories are `N/A — skill prose + a bounded-file-set test`.

---

### Milestone 3 — `Outcome Validation gate: /slo-verify Pass 0 (highest authority, non-renumbering)`

**Goal**: `/slo-verify` gains a leading **Pass 0: Outcome Validation** — the highest-authority pass — that runs the milestone's Outcome Scenarios + Critical User Journeys front-to-end (Playwright for UI) + the Core Capability Regression Matrix (incl. security & reliability outcome rows), at runtime; a Pass 0 failure fails the milestone regardless of any other pass, reusing the existing regression-test-first bug flow. This is the operational realization of the authority inversion ("the promised user outcome exists AND existing important outcomes still exist").

**Context**: `skills/slo-verify/SKILL.md` today runs Pass 1 (happy path), Pass 2 (empty/degraded), Pass 3 (partial failure), Pass 4 (security Bundles A–F), Pass 5 (AI tolerance), Pass 6 (measurement), plus the "STOP → regression test first → hand back to `/slo-execute` → re-verify" bug flow and gates that reject if any BDD scenario is untested at runtime. This milestone **inserts Pass 0 ahead of Pass 1** and leaves Passes 1–6 numbered and worded exactly as they are — the non-renumbering choice from DW-001, which avoids breaking the many repo-wide `Pass 4/5/6` citations (the reversibility D5 seam). Pass 0 is additive; on a legacy/non-value-bearing milestone with no outcome rows, Pass 0 resolves `not_applicable` and the existing passes run unchanged.

**Carmack-style reliability goal**: no silent failure — a green unit/integration/security run can no longer mask an absent user outcome, because Pass 0 is evaluated first and is authoritative; its verdict is recorded in the Evidence Log.

**Important design rule**: implement Pass 0 as a **non-renumbering insertion** (DW-001). Before editing, enumerate every `Pass [0-9]` citation across `skills/` + `docs/` (operator-readiness row) and confirm none requires renumbering. Pass 0 **reuses** the existing bug-found flow verbatim (do not fork a second flow). Pass 0 runs journeys **front-to-end over the highest *applicable* layer chain** (Playwright for UI), honoring §5C's per-layer applicability (theme B), so a mock-only/single-layer assertion cannot satisfy it — a value-bearing milestone still needs ≥1 real cross-layer assertion (`tm-outcome-first-abuse-2`). The authority rule is explicit in the SKILL.md: *a Pass 0 failure fails the milestone even if Passes 1–6 are green.*

**Refactor budget**: `Minimal local refactor permitted in listed files only` — to insert Pass 0, extend the gates list, and add the authority note in `slo-verify/SKILL.md`; cite [`skills/slo-plan/references/refactoring-discipline.md`](../skills/slo-plan/references/refactoring-discipline.md).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the M1 template (§5C/§17 exist) + M2 enforcement; a milestone's authored Outcome Scenarios / Critical User Journeys / Regression Matrix |
| Outputs | edited `slo-verify/SKILL.md` with Pass 0 + authority rule + extended gates; a new Pass-0 procedure reference; the M3 structural test |
| Interfaces touched | `/slo-verify` pass set (additive leading Pass 0); the gates list; `sast-verify` test surface |
| Files allowed to change | `skills/slo-verify/SKILL.md`; `skills/slo-verify/references/outcome-validation-pass.md` (NEW); `xtasks/sast-verify/tests/outcome_first_m3_verify.rs` (NEW) |
| Files to read before changing anything | `skills/slo-verify/SKILL.md` (Passes 1–6, the bug-found flow, the gates, the threat-model read-side contract), `skills/slo-verify/references/security-pass-commands.md` (reference shape), `docs/slo/design/outcome-first-interfaces.md`, `docs/slo/design/outcome-first-code-map.md`; `rg -n "Pass [0-9]" skills/ docs/` |
| New files allowed | `skills/slo-verify/references/outcome-validation-pass.md`, `xtasks/sast-verify/tests/outcome_first_m3_verify.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Passes 1–6 keep their numbers + content + resolution vocabulary; the bug-found flow is reused not forked; `/slo-verify M<N>` command verb + the threat-model read-side contract unchanged; legacy/non-value-bearing milestones run unchanged (Pass 0 ⇒ `not_applicable`) |
| Resource bounds introduced/changed | `N/A — skill prose; Pass 0 itself is bounded to the milestone's declared rows (stated in the SKILL.md)` |
| Invariants/assertions required | test asserts: a "Pass 0" / "Outcome Validation" section exists and is positioned before Pass 1; the highest-authority/override rule text present ("fails the milestone even if Passes 1–6 are green"); the front-to-end (Playwright-for-UI) requirement text present; Pass 0 reuses the existing regression-test-first flow (text present); Passes 4/5/6 still present and not renumbered; **`slo-verify/SKILL.md` SHA-256 == pinned baseline** (lockstep) |
| Debugger / inspection expectation | on assertion failure, print the actual pass-list slice under `--nocapture`; before editing, paste the `rg "Pass [0-9]"` enumeration into the lessons file |
| Static analysis gates | `cargo fmt -p sast-verify -- --check`; `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Exemplar code to copy | the existing Pass 4/5/6 prose shape + the bug-found flow + the gates list in `slo-verify/SKILL.md`; `references/security-pass-commands.md` (reference-file shape) |
| Anti-exemplar code not to copy | do NOT renumber Passes 1–6; do NOT fork a second bug-found flow; do NOT let Pass 0 be satisfied by a mock-only assertion; do NOT edit the SKILL.md without a lockstep SHA-baseline update |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md`: insert Pass 0 as a contained block; existing slo-verify-related tests green before/after |
| AI tolerance contract | `N/A — no AI component` in the deliverable (Pass 0 *verifies* AI behavior via the existing Pass 5 when present, but this milestone ships skill prose + a deterministic test) |
| Forbidden shortcuts | no renumber; no forked flow; no mock-only Pass 0; SKILL.md edit without lockstep SHA baseline; no TODO prose |
| Data classification | `Internal` |
| Proactive controls in play | `C9 Implement Security Logging and Monitoring` (Pass 0 verdict recorded in the Evidence Log); `C4 Address Security from the Start` (security-BDD outcome rows run in Pass 0, citing `tm-<slug>-abuse-N`) |
| Abuse acceptance scenarios | `tm-outcome-first-abuse-2` — outcome-test theatre; mitigation = Pass 0 runs front-to-end at runtime (Playwright for UI), defeating mock-only/single-layer green (BDD row below) |
| Measurement deliverables | Pass 0 + the M3 test (saved query = `rg "Pass 0" slo-verify/SKILL.md`); readout at M5 dogfood; guardrail owner: Sherif. Ties to §5A leading + lagging metric (the gate changing an outcome). |

#### Out of Scope / Must Not Do

- No `/slo-retro`, `/slo-execute`, `/slo-critique` edits (M4).
- No template or `/slo-plan` edits (M1/M2).
- No renumbering of Passes 1–6; no change to Pass 4/5/6 content or the threat-model read-side contract.
- No new skill directory.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-verify/SKILL.md` | Insert a leading **Pass 0: Outcome Validation** block (runs Outcome Scenarios + Critical User Journeys front-to-end + Regression Matrix + security/reliability outcome rows); add the highest-authority/override rule; extend the gates list (reject if any outcome/journey/regression row is untested at runtime); point Pass 0 at the existing bug-found flow |
| `skills/slo-verify/references/outcome-validation-pass.md` | NEW: the Pass 0 procedure — front-to-end ordering (seed→backend→persisted→API/IPC→UI), Playwright-for-UI, regression-matrix re-run, resolution vocabulary, regression-test-first on a Pass 0 finding |
| `xtasks/sast-verify/tests/outcome_first_m3_verify.rs` | NEW: Pass 0 presence/position + authority + front-to-end + no-renumber assertions + SHA pin |

#### Step-by-Step

1. Confirm baseline green; `rg -n "Pass [0-9]" skills/ docs/` and record the enumeration.
2. Write `outcome_first_m3_verify.rs` FIRST (Pass 0 absent ⇒ fails).
3. Author `references/outcome-validation-pass.md` (front-to-end procedure + matrix re-run + finding flow).
4. Insert the Pass 0 block + authority rule + extended gates in `slo-verify/SKILL.md`; point at the existing bug-found flow.
5. Update the M3 test's `slo-verify/SKILL.md` SHA baseline to the post-edit hash (lockstep).
6. Run fmt + clippy; make the M3 test green; confirm Passes 4/5/6 assertions still pass (no renumber).
7. `git status` clean; `.gitignore` review.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: /slo-verify Pass 0 Outcome Validation**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Pass 0 runs first | happy path | a value-bearing milestone with Outcome Scenarios | `/slo-verify M<N>` runs | Pass 0 runs the outcome scenarios + critical journeys front-to-end before Pass 1 (SKILL.md asserts Pass 0 position + procedure) |
| Authority override | partial failure | Passes 1–6 green but a Pass 0 outcome row fails | the milestone is evaluated | the milestone FAILS (SKILL.md asserts "fails even if Passes 1–6 are green") |
| Regression matrix catch | backward compat / cross-capability | a change that breaks an adjacent capability | Pass 0 re-runs the Core Capability Regression Matrix | the regressed journey fails → milestone blocked |
| Theatre defeated | abuse case (`tm-outcome-first-abuse-2`) | an Outcome test that only asserts a mock return | Pass 0 runs it front-to-end (Playwright for UI) | the mock-only test does not satisfy Pass 0 (SKILL.md asserts front-to-end requirement) |
| Bug-found reuse | retry/rollback | Pass 0 surfaces a failing outcome | the bug-found flow triggers | STOP → regression test first → hand back to `/slo-execute` → re-verify (reused, not forked) |
| No renumber | backward compat | the edited slo-verify SKILL.md | the M3 test runs | Pass 4/5/6 still present + named; numbers unchanged |
| Non-applicable | empty state | a docs-only milestone (no outcome rows) | `/slo-verify` runs | Pass 0 resolves `not_applicable`; Passes 1–6 run unchanged |

#### Regression Tests

- Full `sast-verify` suite green (incl. M1/M2 tests).
- Any existing test asserting slo-verify Pass 4/5/6 still passes.
- The threat-model read-side contract text in slo-verify is unchanged (diff check).

#### Compatibility Checklist

- [ ] Passes 1–6 numbers + content unchanged; threat-model read-side contract intact
- [ ] The bug-found flow is reused, not duplicated
- [ ] `/slo-verify M<N>` command verb unchanged
- [ ] `sast-verify` suite green; slo-verify SHA baseline moved in lockstep

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/outcome_first_m3_verify.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `verify_has_pass_0_outcome_validation` | the gate exists, positioned first | Pass 0 / Outcome Validation block present before Pass 1 |
| `pass_0_is_highest_authority` | authority inversion | override-rule text present |
| `pass_0_runs_front_to_end` | anti-theatre (`tm-outcome-first-abuse-2`) | front-to-end / Playwright-for-UI requirement text present |
| `verify_passes_not_renumbered` | back-compat (D5) | Pass 4/5/6 anchors all present |
| `verify_skill_md_sha_pinned` | tamper-evident baseline | `slo-verify/SKILL.md` SHA-256 == pinned baseline |

#### Smoke Tests

- [ ] `cargo test -p sast-verify outcome_first_m3` passes
- [ ] `rg -n "Pass 0|Outcome Validation" skills/slo-verify/SKILL.md` shows the block
- [ ] `rg -n "Pass 4|Pass 5|Pass 6" skills/slo-verify/SKILL.md` still present (no renumber)
- [ ] `cargo clippy -p sast-verify --all-targets -- -D warnings` clean
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 31 suites, 0 failed | Pass | |
| Pass-citation enumeration | non-renumber approach | none needs renumber | Pass 0 inserted ahead of Pass 1; Pass 1/4/5/6 untouched; 5 reader tests green | Pass | DW-001 closed |
| Test created (fails) | `outcome_first_m3_verify.rs` | fails: Pass 0 absent | 6 failed for the right reason (no-renumber invariant pre-satisfied) | Pass | BDD-first confirmed |
| Reference authored | `references/outcome-validation-pass.md` | complete | front-to-end + matrix re-run + regression-test-first | Pass | |
| Implementation | Pass 0 block + authority + gates | present | leading Pass 0 + override rule + extended Gates | Pass | |
| Formatter | `cargo fmt -p sast-verify -- --check` | clean | exit 0 | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | new tests clippy-clean; pre-existing debt = DW-002 | Pass (scoped) | |
| Full tests | `cargo test -p sast-verify` | green | 32 suites, 0 failed | Pass | M3 test + dogfood = +2 suites |
| No-renumber check | `verify_passes_not_renumbered` + reader tests | all present | svl_m4/kani_m3/mloop_m4/slo_tm_m2_consumers/sap_imp_m3_standards green | Pass | |
| SHA baseline pinned | M3 test `verify_skill_md_sha_pinned` | SKILL.md SHA == baseline (lockstep) | `3f6ca77…` pinned; green | Pass | re-pinned after wording fix |
| Theme-A dogfood (gate FIRES) | `outcome_first_dogfood` | blocks unproven, passes proven | 3/3 green; gate non-vacuous | Pass | executable "gate fires" proof (CEO-1/ENG-3) |
| Test artifact cleanup | `git status` | clean | only allow-listed + fixtures | Pass | |
| .gitignore review | review `.gitignore` | current | no new artifacts | Pass | |

#### Definition of Done

- `/slo-verify` has a leading Pass 0 Outcome Validation that runs outcome scenarios + critical journeys front-to-end + the regression matrix, and is the highest authority
- Pass 0 reuses the existing regression-test-first flow; Passes 1–6 unchanged + not renumbered
- a Pass 0 failure fails the milestone regardless of other passes (asserted)
- the edited `slo-verify/SKILL.md` SHA-256 is pinned to its lockstep baseline
- `outcome_first_m3_verify.rs` green; full `sast-verify` suite green; fmt + clippy clean; `git status` clean
- lessons + completion files written; tracker updated

#### Post-Flight

- **Mid-stream dogfood checkpoint (theme A / CEO-1 + ENG-3):** after M3 closes, run the §5A dogfood acceptance checkpoint — drive a real `/slo-verify` Pass 0 on one real value-bearing milestone with a deliberately-unproven outcome row and confirm Pass 0 reports fail + `/slo-retro` refuses to close. This is the executable "the gate fires" proof (the structural tests only prove the contract is *documented*). Record the result before starting M4.
- **Other docs**: none in M3 (catalog/loops are M5; the reference lives under the skill).

#### Notes

- Concurrency / resource-bound BDD categories are `N/A — skill prose + a bounded-file-set test`; Pass 0's own runtime bound (declared rows only) is stated in the SKILL.md.

---

### Milestone 4 — `Close the loop: /slo-retro gate + ## Outcome vs promise + /slo-execute outcome-first + /slo-critique theatre review`

**Goal**: the three back-end consumers of the outcome contract enforce it — `/slo-retro` REFUSES to close a value-bearing milestone while any Outcome Scenario / Critical User Journey / Regression Matrix row is unproven / blank / reasonless-waived (additive to its existing refusal gates) and gains a `## Outcome vs promise` lessons section; `/slo-execute` writes Outcome Scenario + Critical Journey tests first (extends its BDD-first step); `/slo-critique` flags vacuous/single-`And`/mock-only outcome scenarios as `ask` (folded into the existing eng-lead + security passes, no new persona).

**Context**: `skills/slo-retro/SKILL.md` already refuses on blank Evidence-Log actuals / pending BDD / blank Kani row / untracked artifacts and has a `## Results vs thesis` lessons section — the new gate + `## Outcome vs promise` section are additive peers. `skills/slo-execute/SKILL.md` already writes BDD tests first (Step 1) + E2E stubs (Step 2) — Outcome Scenario + Critical Journey tests join Step 1. `skills/slo-critique/SKILL.md` has four personas (CEO / Eng lead / Security / Design) + a `## Rotation order` heading — the theatre review folds into eng-lead (vacuous scenario) + security (security-BDD abuse-citation) without adding a persona. **Seam (DW-001 family):** `slo-critique/SKILL.md` is SHA-pinned by TWO existing tests (`sap_imp_m5_agents.rs` F-ENG-6 + `slo_tm_m2_consumers.rs` F-ENG-6 lockstep) which also assert the `## Rotation order` heading + the four persona anchors survive; both baselines must move in lockstep and those anchors must be preserved.

**Carmack-style reliability goal**: no silent failure + make invalid states unrepresentable — the loop cannot reach `done` with an unproven outcome (retro fail-closed), the outcome tests exist before the code (execute), and theatre is caught before execution (critique).

**Important design rule**: all three edits are **additive** — preserve every existing refusal/marker the reader tests assert (`svl_m4`/`kani_m3`/`mloop_m4` read retro+execute+verify markers; `slo_tm_m2_consumers`/`sap_imp_m3_standards` read critique markers). The retro gate fires for **value-bearing** milestones only (same trigger as §5C), so docs-only runbooks still close. SHA-pin `slo-retro` + `slo-execute` afresh in the M4 test; for `slo-critique`, **update the two existing baselines** rather than adding a third pin (single source of truth per file).

**Refactor budget**: `Minimal local refactor permitted in listed files only` — to fold each addition in coherently; cite [`skills/slo-plan/references/refactoring-discipline.md`](../skills/slo-plan/references/refactoring-discipline.md).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the M1 template + M2 enforcement + M3 Pass 0; a milestone's Outcome Scenarios / Critical User Journeys / Regression Matrix + Pass 0 verdict |
| Outputs | edited `slo-retro` (gate + `## Outcome vs promise`), `slo-execute` (outcome-first authoring), `slo-critique` (theatre review); the M4 test; updated `slo-critique` SHA baselines in the two existing tests |
| Interfaces touched | `/slo-retro` refusal-gate list + lessons template; `/slo-execute` test-first step; `/slo-critique` eng-lead/security review prose; `sast-verify` test surface |
| Files allowed to change | `skills/slo-retro/SKILL.md`; `skills/slo-execute/SKILL.md`; `skills/slo-critique/SKILL.md`; `xtasks/sast-verify/tests/outcome_first_m4_consumers.rs` (NEW); `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (update the `slo-critique` SHA baseline constant ONLY); `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` (update the `slo-critique` SHA baseline constant ONLY) |
| Files to read before changing anything | `skills/slo-retro/SKILL.md` (refusal gates + `## Results vs thesis`), `skills/slo-execute/SKILL.md` (Step 1/Step 2 BDD-first), `skills/slo-critique/SKILL.md` (`## Rotation order` + personas), `xtasks/sast-verify/tests/{sap_imp_m5_agents,slo_tm_m2_consumers,svl_m4,kani_m3,mloop_m4}.rs`, `docs/slo/design/outcome-first-code-map.md` (seams #1–#2) |
| New files allowed | `xtasks/sast-verify/tests/outcome_first_m4_consumers.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | every existing `/slo-retro` refusal gate + `## Results vs thesis` preserved; `/slo-execute` Step 1/2 + Operator-Readiness/Detected-Work-Ledger flows preserved; `/slo-critique` `## Rotation order` + 4 persona anchors preserved; reader tests (`svl_m4`/`kani_m3`/`mloop_m4`/`slo_tm_m2_consumers`/`sap_imp_m3_standards`) stay green; legacy/non-value-bearing milestones still close |
| Resource bounds introduced/changed | `N/A — skill prose + a bounded-file-set test` |
| Invariants/assertions required | M4 test asserts: `slo-retro` has the outcome refusal gate (value-bearing) + `## Outcome vs promise`; `slo-execute` writes outcome/journey tests first; `slo-critique` flags vacuous/theatre outcome scenarios as `ask`; existing critique anchors survive; **`slo-retro` + `slo-execute` SHA-256 == new pinned baselines**, and **the M4 test cross-checks that BOTH critique baseline constants (in `sap_imp_m5_agents.rs` + `slo_tm_m2_consumers.rs`) equal the live `slo-critique/SKILL.md` SHA** (theme C / ENG-4 — a half-update fails loudly here, not just in the old test) |
| Debugger / inspection expectation | on assertion failure, print the offending SKILL.md slice under `--nocapture`; recompute the `slo-critique` SHA before/after to confirm the lockstep update |
| Static analysis gates | `cargo fmt -p sast-verify -- --check`; `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Exemplar code to copy | the existing `/slo-retro` refusal-gate list + `## Results vs thesis` (peer shape); `/slo-execute` Step 1 BDD-first; the `slo-critique` finding-category `ask` pattern; the SHA-baseline constant + comparison in `sap_imp_m5_agents.rs` |
| Anti-exemplar code not to copy | do NOT remove any existing refusal gate / persona anchor / `## Rotation order`; do NOT add a third SHA pin for `slo-critique` (update the two existing); do NOT make the retro gate fire on non-value-bearing milestones |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md`: additive microsteps; the reader tests green before and after each edit |
| AI tolerance contract | `N/A — no AI component` in the deliverable (skill prose + a deterministic Rust test) |
| Forbidden shortcuts | removing an existing gate/anchor; a third critique pin; **weakening/removing any assertion in the two pin files — only the baseline-constant VALUE may change there** (theme C / SEC-1 / `tm-outcome-first-abuse-4`); retro gate on docs-only milestones; SKILL.md edit without lockstep SHA update; TODO prose |
| Data classification | `Internal` |
| Proactive controls in play | `C9 Implement Security Logging and Monitoring` (retro records outcome-vs-promise + refusal reasons); `C4 Address Security from the Start` (execute writes security-BDD outcome tests first; critique reviews them) |
| Abuse acceptance scenarios | `tm-outcome-first-abuse-2` (theatre — critique flags vacuous outcome scenarios as `ask`), `tm-outcome-first-abuse-3` (gate gaming — retro refuses on blank/reasonless-waived rows), and `tm-outcome-first-abuse-4` (weakening the critique SHA-pin test — M4 constrains the two pin files to constant-only + cross-checks both baselines; theme C / SEC-1); BDD rows below |
| Measurement deliverables | the three enforcement edits + the M4 test (saved query = `rg "Outcome vs promise" slo-retro/SKILL.md`); readout at M5 dogfood; guardrail owner: Sherif. Ties to §5A lagging metric (gate changes an outcome). |

#### Out of Scope / Must Not Do

- No template / `/slo-plan` / `/slo-verify` edits (M1/M2/M3).
- No catalog / loop-doc / operating-contract edits (M5).
- No change to any existing refusal gate, persona, `## Rotation order`, or reader-test marker.
- No new skill directory; no third SHA pin for `slo-critique`.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-retro/SKILL.md` | Add the outcome refusal gate (value-bearing only; refuse close while any Outcome Scenario / Critical Journey / Regression Matrix row is unproven/blank/reasonless-waived) + a `## Outcome vs promise` lessons-template section |
| `skills/slo-execute/SKILL.md` | Extend Step 1 (write tests first) to include Outcome Scenario + Critical User Journey tests, citing §5C |
| `skills/slo-critique/SKILL.md` | Fold outcome-theatre review into the eng-lead pass (vacuous/single-`And`/mock-only ⇒ `ask`) + security pass (security-BDD must cite `tm-<slug>-abuse-N`); preserve `## Rotation order` + 4 personas |
| `xtasks/sast-verify/tests/outcome_first_m4_consumers.rs` | NEW: behavior-marker + SHA assertions for all three skills |
| `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` | Update the `slo-critique` SHA-256 baseline constant to the post-edit hash (lockstep; no other change) |
| `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` | Update the `slo-critique` SHA-256 baseline constant to the post-edit hash (lockstep; no other change) |

#### Step-by-Step

1. Confirm baseline green; read the two critique-pin tests + the reader tests; record current `slo-critique` SHA.
2. Write `outcome_first_m4_consumers.rs` FIRST (markers absent ⇒ fails).
3. Edit `slo-retro` (gate + `## Outcome vs promise`); edit `slo-execute` (outcome-first Step 1); edit `slo-critique` (theatre review, anchors preserved).
4. Recompute the new `slo-critique` SHA; update the baseline constant in BOTH `sap_imp_m5_agents.rs` and `slo_tm_m2_consumers.rs`; set the new `slo-retro` + `slo-execute` baselines in the M4 test.
5. Run fmt + clippy; make the M4 test green; confirm `sap_imp_m5`, `slo_tm_m2_consumers`, `svl_m4`, `kani_m3`, `mloop_m4` all green.
6. `git status` clean; `.gitignore` review.
7. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: outcome-contract enforcement at the back of the loop**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Retro refuses unproven outcome | abuse case (`tm-outcome-first-abuse-3`) | a value-bearing milestone with an unproven Outcome Scenario row | `/slo-retro M<N>` runs | it refuses to close (SKILL.md gate asserts this) |
| Retro refuses reasonless waiver | abuse case (`tm-outcome-first-abuse-3`) | a Regression Matrix row `waived_with_reason` with empty reason | `/slo-retro` runs | it refuses to close |
| Outcome vs promise recorded | happy path | a closed value-bearing milestone | `/slo-retro` writes lessons | a `## Outcome vs promise` section is present |
| Execute writes outcome tests first | happy path | a milestone with Outcome Scenarios | `/slo-execute M<N>` starts | it creates the outcome/journey test files before production code (SKILL.md Step 1 asserts this) |
| Critique flags theatre | abuse case (`tm-outcome-first-abuse-2`) | a vacuous/single-`And`/mock-only Outcome Scenario | `/slo-critique` reviews | it is flagged `ask` (SKILL.md eng-lead pass asserts this) |
| Critique anchors survive | backward compat | the edited `slo-critique` | the existing pin tests run | `## Rotation order` + CEO/Eng lead/Security/Design present; SHA == updated baseline |
| Pin test not weakened | abuse case (`tm-outcome-first-abuse-4`) | the two SHA-pin files are edited in M4 | the M4 test runs | only the baseline-constant value changed; the F-ENG-6 assertions are intact; both critique baselines == the live `slo-critique` SHA |
| Docs-only still closes | empty state | a non-value-bearing milestone | `/slo-retro` runs | the outcome gate is `N/A`; close proceeds |

#### Regression Tests

- Full `sast-verify` suite green, specifically `sap_imp_m5_agents`, `slo_tm_m2_consumers`, `svl_m4`, `kani_m3`, `mloop_m4`.
- Every pre-existing `/slo-retro` refusal gate still present (diff check).
- `/slo-execute` Operator-Readiness + Detected-Work-Ledger flows unchanged.

#### Compatibility Checklist

- [ ] All existing `/slo-retro` refusal gates + `## Results vs thesis` preserved
- [ ] `/slo-execute` Step 1/2 + readiness/ledger flows preserved
- [ ] `/slo-critique` `## Rotation order` + 4 personas preserved; both SHA baselines updated in lockstep
- [ ] Reader tests (`svl_m4`/`kani_m3`/`mloop_m4`/`slo_tm_m2_consumers`/`sap_imp_m3_standards`) green

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/outcome_first_m4_consumers.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `retro_refuses_unproven_outcome` | fail-closed close gate (`tm-outcome-first-abuse-3`) | gate text present, value-bearing-scoped |
| `retro_has_outcome_vs_promise` | learning loop | `## Outcome vs promise` present |
| `execute_writes_outcome_tests_first` | test-first outcomes | Step 1 outcome/journey text present |
| `critique_flags_outcome_theatre` | anti-theatre (`tm-outcome-first-abuse-2`) | eng-lead `ask` rule present |
| `critique_anchors_and_sha_preserved` | back-compat | `## Rotation order` + personas present; SHA == updated baseline |
| `retro_execute_sha_pinned` | tamper-evident baselines | `slo-retro` + `slo-execute` SHA == new baselines |
| `critique_baselines_cross_checked` | half-update fails loudly (theme C / `tm-outcome-first-abuse-4`) | both `sap_imp_m5` + `slo_tm_m2_consumers` critique constants == live `slo-critique` SHA |

#### Smoke Tests

- [ ] `cargo test -p sast-verify outcome_first_m4` passes
- [ ] `cargo test -p sast-verify sap_imp_m5 slo_tm_m2_consumers` green (critique pins updated)
- [ ] `rg -n "Outcome vs promise" skills/slo-retro/SKILL.md` present
- [ ] `cargo clippy -p sast-verify --all-targets -- -D warnings` clean
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 32 suites, 0 failed | Pass | |
| Critique SHA recorded | `shasum -a 256 skills/slo-critique/SKILL.md` | pre-edit hash noted | pre: `9e31b7dd…` | Pass | |
| Test created (fails) | `outcome_first_m4_consumers.rs` | fails: markers absent | 6 failed for the right reason | Pass | BDD-first confirmed |
| Implementation | retro gate + `## Outcome vs promise` + execute Step 1 + critique theatre review | present | 3 SKILL.md edits, anchors preserved | Pass | |
| Critique baseline updated | bump the SINGLE `CRITIQUE_SKILL_SHA256` in `sap_imp_m5` | live == constant | `c6d1ede5…`; slo_tm_m2 DERIVES it (not edited) | Pass | **DW-003** — single source of truth, not two |
| Retro/execute pinned | new baselines in M4 test | == post-edit SHAs | retro `5c636264…`, execute `b85d4790…` | Pass | fresh pins |
| Formatter | `cargo fmt -p sast-verify -- --check` | clean | exit 0 | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | new test clippy-clean; pre-existing debt = DW-002 | Pass (scoped) | |
| Full tests | `cargo test -p sast-verify` | green (incl. reader tests) | 33 suites, 0 failed | Pass | sap_imp_m5/slo_tm_m2/svl_m4/kani_m3/mloop_m4 green |
| Anchors preserved | `critique_anchors_preserved` + the two pin tests | all present | `## Rotation order` + 4 personas; both pins track | Pass | tm-outcome-first-abuse-4 guarded |
| Test artifact cleanup | `git status` | clean | only allow-listed files | Pass | slo_tm_m2 allow-listed but unneeded (DW-003) |
| .gitignore review | review `.gitignore` | current | no new artifacts | Pass | |

#### Definition of Done

- `/slo-retro` refuses to close a value-bearing milestone with an unproven/blank/reasonless-waived outcome row, and writes `## Outcome vs promise`
- `/slo-execute` writes Outcome Scenario + Critical Journey tests first
- `/slo-critique` flags vacuous/theatre outcome scenarios as `ask` while preserving `## Rotation order` + 4 personas
- `slo-retro` + `slo-execute` newly SHA-pinned; `slo-critique` SHA baselines updated in lockstep in both existing tests
- `outcome_first_m4_consumers.rs` + the full `sast-verify` suite green; fmt + clippy clean; `git status` clean
- lessons + completion files written; tracker updated

#### Post-Flight

- **Other docs**: none in M4 (catalog/loops/principle are M5).

#### Notes

- **Entry depends on the theme-A mid-stream dogfood checkpoint** (after M3) having passed — the executable proof that Pass 0 actually blocks a real milestone — before M4 invests in the three consumer edits.
- Concurrency / resource-bound BDD categories are `N/A — skill prose + a bounded-file-set test`.

---

### Milestone 5 — `Principle + loop docs: Outcome First Engineering in operating-contract + catalog + LOOPS-ENGINEERING`

**Goal**: the **Outcome First Engineering** principle becomes a named, host-neutral rule of the methodology — landed in `references/agent/operating-contract.md` (the host-neutral contract Copilot/Codex/Claude all read), summarized in `docs/skill-pack-catalog.md`, and documented in `docs/LOOPS-ENGINEERING.md` as a Sprint-loop Outcome-First overlay with the inverted-authority pyramid — so the change is discoverable and binding, not just implicit in the skills.

**Context**: `references/agent/operating-contract.md` carries the host-neutral agent rules (ask when ambiguous, smallest safe change, verify before claiming). `docs/skill-pack-catalog.md` has the Sprint-flow table. `docs/LOOPS-ENGINEERING.md` documents the Sprint loop + a Secure Value overlay (lines ~70–87). This milestone adds the principle + an Outcome-First overlay table + the inverted-pyramid diagram alongside (not replacing) the Secure Value overlay, and a catalog one-liner. It is **docs-only** (no skill behavior change) and is the capstone where `/slo-retro` records the runbook's §5A actual-vs-thesis.

**Carmack-style reliability goal**: make the rule explicit and host-neutral — "code completion alone is insufficient" is written into the contract every agent reads, not left as tribal knowledge.

**Important design rule**: the principle text MUST be host-neutral in `operating-contract.md` (no Claude/Playwright-only assumptions — those live in the Claude overlay/skill prose). Add the Outcome-First overlay as a peer of the Secure Value overlay (keep them visibly distinct: security envelope vs. outcome authority). Do **not** touch the catalog's skill-count line or add a "Start here" loop row — Outcome-First is an overlay on the Sprint loop, not a new loop.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — to insert the principle + overlay coherently; cite [`skills/slo-plan/references/refactoring-discipline.md`](../skills/slo-plan/references/refactoring-discipline.md).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1–M4 (template + the three skills now enforce the contract); `docs/slo/design/outcome-first-overview.md` (principle wording + pyramid) |
| Outputs | the principle in `operating-contract.md`; a catalog one-liner; a LOOPS-ENGINEERING Outcome-First overlay + inverted-pyramid diagram; the M5 test; the §5A readout |
| Interfaces touched | `operating-contract.md` (additive principle); `skill-pack-catalog.md` (additive line); `LOOPS-ENGINEERING.md` (additive overlay); `sast-verify` test surface |
| Files allowed to change | `references/agent/operating-contract.md`; `docs/skill-pack-catalog.md`; `docs/LOOPS-ENGINEERING.md`; `xtasks/sast-verify/tests/outcome_first_m5_principle.rs` (NEW) |
| Files to read before changing anything | `references/agent/operating-contract.md`, `docs/skill-pack-catalog.md` (Sprint-flow table + skill-count line), `docs/LOOPS-ENGINEERING.md` (Sprint loop + Secure Value overlay), `docs/slo/design/outcome-first-overview.md`, `docs/slo/design/outcome-first-code-map.md` |
| New files allowed | `xtasks/sast-verify/tests/outcome_first_m5_principle.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | the catalog skill-count line + Sprint-flow rows unchanged; the Secure Value overlay preserved; existing LOOPS sections + any loops-doc marker tests preserved; operating-contract existing rules preserved; host-neutral wording (no host-specific assumptions) |
| Resource bounds introduced/changed | `N/A — Markdown docs + a bounded-file-set test` |
| Invariants/assertions required | M5 test asserts: the Outcome First Engineering principle text ("code completion alone is insufficient" + "user outcome exists AND existing important outcomes still exist") present in `operating-contract.md`; a catalog line naming the principle/gate present; the LOOPS Outcome-First overlay + inverted-pyramid present; the catalog skill-count line + the Secure Value overlay heading still present (no clobber) |
| Debugger / inspection expectation | on assertion failure, print the matched/!matched doc slice under `--nocapture` |
| Static analysis gates | `cargo fmt -p sast-verify -- --check`; `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Exemplar code to copy | the Secure Value overlay table + the loop-section format in `docs/LOOPS-ENGINEERING.md`; the host-neutral rule phrasing in `operating-contract.md` |
| Anti-exemplar code not to copy | do NOT change the catalog skill-count line; do NOT add a "Start here" loop row; do NOT phrase the principle host-specifically |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md`: additive insertions; existing loops/catalog tests green before/after |
| AI tolerance contract | `N/A — no AI component` (docs + a deterministic Rust test) |
| Forbidden shortcuts | touching the skill-count line; host-specific principle wording; clobbering the Secure Value overlay; TODO prose |
| Data classification | `Internal` |
| Proactive controls in play | `C4 Address Security from the Start` (the principle binds outcome+security+reliability together at the contract level) |
| Abuse acceptance scenarios | `N/A — no new surface introduced` (first-party doc edits; no user-supplied string interpolation, so `tm-outcome-first-abuse-1` injection does not apply here) |
| Measurement deliverables | the principle-presence M5 test + the runbook's §5A **actual-vs-thesis readout recorded at this close** (the measurement moment for the whole runbook); guardrail owner: Sherif |

#### Out of Scope / Must Not Do

- No skill (`SKILL.md`) edits, no template edits (M1–M4 own those).
- No change to the catalog skill-count line or a new "Start here" loop row.
- No new skill directory; no host-specific principle wording in the host-neutral contract.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/agent/operating-contract.md` | Add the host-neutral "Outcome First Engineering" principle (code completion alone is insufficient; done = promised user outcome exists AND existing important outcomes still exist) |
| `docs/skill-pack-catalog.md` | Add a one-line note in the Sprint-flow context naming the Outcome Validation gate + the principle (no skill-count change) |
| `docs/LOOPS-ENGINEERING.md` | Add a Sprint-loop **Outcome-First overlay** table (peer of the Secure Value overlay) + the inverted-authority pyramid diagram |
| `xtasks/sast-verify/tests/outcome_first_m5_principle.rs` | NEW: assert the principle + catalog line + overlay/pyramid present; skill-count + Secure Value overlay preserved |

#### Step-by-Step

1. Write `outcome_first_m5_principle.rs` FIRST (text absent ⇒ fails).
2. Add the principle to `operating-contract.md` (host-neutral wording).
3. Add the catalog one-liner (do not touch the skill-count line).
4. Add the LOOPS Outcome-First overlay + inverted-pyramid (peer of Secure Value overlay).
5. Run fmt + clippy; make the M5 test green; confirm catalog/loops marker tests still green.
6. `/slo-retro` records the §5A actual-vs-thesis readout at close.
7. `git status` clean; `.gitignore` review; Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Outcome First Engineering principle + loop docs**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Principle in contract | happy path | the edited `operating-contract.md` | the M5 test runs | the "code completion alone is insufficient" + "outcome exists AND existing outcomes still exist" text is present, host-neutral |
| Catalog names the gate | happy path | the edited catalog | the M5 test runs | a Sprint-flow line names the Outcome Validation gate/principle |
| Loop overlay + pyramid | happy path | the edited `LOOPS-ENGINEERING.md` | the M5 test runs | the Outcome-First overlay table + inverted-pyramid present |
| Skill-count untouched | backward compat | the catalog | the M5 test runs | the skill-count line is unchanged |
| Secure Value overlay preserved | backward compat | `LOOPS-ENGINEERING.md` | the M5 test runs | the Secure Value overlay heading still present |
| Host-neutral | invalid input | the principle text | reviewed for host assumptions | no Claude/Playwright-only wording in `operating-contract.md` |

#### Regression Tests

- Full `sast-verify` suite green (incl. M1–M4 tests + any catalog/loops marker tests).
- `cargo run -p sldo-install -- --dry-run` skill count unchanged.
- The Secure Value overlay + Feature-performance loop sections in LOOPS still present.

#### Compatibility Checklist

- [ ] Catalog skill-count line + Sprint-flow rows unchanged
- [ ] Secure Value overlay + existing LOOPS sections preserved
- [ ] `operating-contract.md` existing rules preserved; new principle host-neutral
- [ ] `sast-verify` suite green

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/outcome_first_m5_principle.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `operating_contract_has_outcome_first_principle` | the rule is binding + host-neutral | principle text present; no host-specific token |
| `catalog_names_outcome_gate` | discoverable | Sprint-flow line present |
| `loops_has_outcome_first_overlay_and_pyramid` | documented | overlay table + inverted-pyramid present |
| `catalog_skill_count_and_secure_overlay_preserved` | back-compat | skill-count line + Secure Value overlay heading unchanged/present |

#### Smoke Tests

- [ ] `cargo test -p sast-verify outcome_first_m5` passes
- [ ] `rg -n "Outcome First Engineering" references/agent/operating-contract.md docs/skill-pack-catalog.md docs/LOOPS-ENGINEERING.md` shows all three
- [ ] `cargo clippy -p sast-verify --all-targets -- -D warnings` clean
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | all green | 33 suites, 0 failed | Pass | |
| Test created (fails) | `outcome_first_m5_principle.rs` | fails: text absent | 4 failed for the right reason (host-neutral test re-scoped to the principle section) | Pass | BDD-first confirmed |
| Implementation | principle + catalog line + overlay/pyramid | present | operating-contract + catalog + LOOPS edits | Pass | |
| Skill-count untouched | `catalog_skill_count_preserved` | unchanged | "Shipped skills at HEAD: 49" intact | Pass | |
| Formatter | `cargo fmt -p sast-verify -- --check` | clean | exit 0 | Pass | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean | M5 test clippy-clean; pre-existing debt = DW-002 | Pass (scoped) | |
| Full tests | `cargo test -p sast-verify` | green | 34 suites, 0 failed | Pass | M5 test = +1 suite |
| §5A readout | `/slo-retro` records actual-vs-thesis | recorded | dogfood (M3) demonstrated the gate fires; full enforcement live end-to-end | Pass | see M5 lessons `## Results vs thesis` |
| Test artifact cleanup | `git status` | clean | only allow-listed files | Pass | |
| .gitignore review | review `.gitignore` | current | no new artifacts | Pass | |

#### Definition of Done

- the Outcome First Engineering principle is present (host-neutral) in `operating-contract.md`, summarized in the catalog, and documented as a LOOPS Outcome-First overlay + inverted pyramid
- the catalog skill-count line + the Secure Value overlay are preserved
- `outcome_first_m5_principle.rs` + the full `sast-verify` suite green; fmt + clippy clean; `git status` clean
- the §5A actual-vs-thesis readout is recorded at close
- lessons + completion files written; tracker updated; runbook complete

#### Post-Flight

- **ARCHITECTURE.md**: add the "Outcome First Engineering / Outcome Validation gate" to the loop description (the loop now ships end-to-end).
- **README.md**: note the new gate if user-facing capabilities are summarized.
- **Other docs**: this milestone IS the docs milestone.

#### Notes

- Concurrency / resource-bound BDD categories are `N/A — Markdown docs + a bounded-file-set test`.
- Abuse-case category is `N/A — no new surface` (first-party doc edits; no user-string interpolation).

---

## 18. Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | none (reality-first) | none | review only | v4 template |
| 2 | none | none | review only | `skills/slo-plan` |
| 3 | none | none | review only | `skills/slo-verify` |
| 4 | none | none | review only | `skills/slo-retro`, `skills/slo-execute`, `skills/slo-critique` |
| 5 | add "Outcome First Engineering / Outcome Validation gate" to the loop description | note the new gate if user-facing capabilities are summarized | review only | `operating-contract.md`, `skill-pack-catalog.md`, `LOOPS-ENGINEERING.md` |

---

## 19. Optional Fast-Fail Review Prompt for Agents

Use §19 of the v4 template before writing production code for any milestone.

---

## 20. Source Basis

Authored by `/slo-plan` from the `/slo-architect` design family `docs/slo/design/outcome-first-*` (overview, stack-decision, interfaces, threat-model + `.slo.json`, reversibility, code-map). Implements the founder's "Outcome First Engineering" 11-change proposal. Decisions: elevate-in-place (Outcome Validation = `/slo-verify` leading pass, no new skill); additive/optional template sections (the §5A/§5B precedent); non-renumbering Pass 0.
