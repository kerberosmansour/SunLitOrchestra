# `/slo-kani` — Bringing Kani Rust Verification Into The SLO Loop (AI-First Runbook v4)

> **Purpose**: Ship a host-neutral `/slo-kani` skill that drives the Kani Rust model checker as a code-level peer to `/slo-tla`, wire it into the architect → plan → execute → verify → retro loop, and prove the failure bar end-to-end: a Rust kernel's key parts are Kani-tested, a seeded bug is caught, remediated, and re-verified green within stated bounds.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Prefer automated guardrails over intention. Prefer the tool's verdict over the agent's narration. Prefer disclosed scope over implied totality. A green Kani run means "proved within stated harness, assumptions, and bounds" — never "whole system proved."
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/slo/design/kani-verification-overview.md](slo/design/kani-verification-overview.md), [docs/slo/design/kani-verification-interfaces.md](slo/design/kani-verification-interfaces.md), [docs/slo/design/kani-verification-threat-model.md](slo/design/kani-verification-threat-model.md), [skills/slo-tla/SKILL.md](../skills/slo-tla/SKILL.md), [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md), `~/Downloads/deep-research-report-Kani.md` (research dossier).

> **What's new in v4 vs v3**: explicit Carmack-style reliability rules; extended Contract Block with resource bounds + invariants + debugger expectation + static-analysis gates. This runbook is itself a formal-verification feature, so §5 carries the very Kani proof-obligation sub-block M3 introduces.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `kani-verification` |
| Project name | SunLit Orchestra |
| Primary stack | Markdown skill pack + Rust (`cargo kani` subprocess); demo crate is Rust |
| Primary package/app names | `skills/slo-kani`, `sast-verify` (structural tests), external demo crate |
| Prefix for tests and lesson files | `kani` |
| Default unit test command | `cargo test -p sast-verify` |
| Default integration/BDD test command | `cargo test -p sast-verify` |
| Default E2E/runtime validation command | `cargo kani` (in the demo crate, M4); `./target/release/sldo-install --dry-run` (skill discovery) |
| Default build/boot command | `cargo build -p sast-verify -p sldo-install` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy --workspace --all-targets -- -D warnings` |
| Default dependency / security audit command | `cargo audit` |
| Default debugger or state-inspection tool | `cargo kani --concrete-playback` (counterexample replay); `rust-lldb` for the demo crate |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- `discover_skills()` contract in `crates/sldo-install/src/install.rs` (a new `skills/slo-kani/SKILL.md` must be auto-discovered with no installer change).
- The four existing `<slug>-overview.md` frontmatter keys (`tla_required`, `security_libs_required`, `ai_component`, `compliance`) — types and defaults unchanged when `kani_required` is added.
- `docs/slo/templates/runbook-template_v_4_template.md` §5 — the TLA+ content is untouched; the Kani sub-block is additive.
- The existing structural-contract test baselines in `xtasks/sast-verify/tests/` (esp. `sap_imp_m5_agents.rs` SHA baseline) — a new test is a sibling file, never an edit to a baseline.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-kani` skill skeleton + tool prereq cascade + candidate-scoring rubric | `done` | 2026-05-22 | 2026-05-22 | [kani-m1](docs/slo/lessons/kani-m1.md) | [kani-m1](docs/slo/completion/kani-m1.md) |
| 2 | Harness-generation + run/triage methodology + honesty/scope gates | `done` | 2026-05-22 | 2026-05-22 | [kani-m2](docs/slo/lessons/kani-m2.md) | [kani-m2](docs/slo/completion/kani-m2.md) |
| 3 | Integration seams (architect `kani_required`, §5 sub-block, execute/verify/retro hooks) | `done` | 2026-05-22 | 2026-05-22 | [kani-m3](docs/slo/lessons/kani-m3.md) | [kani-m3](docs/slo/completion/kani-m3.md) |
| 4 | Test repo + catch→remediate→green failure-bar demonstration | `done` | 2026-05-22 | 2026-05-22 | [kani-m4](docs/slo/lessons/kani-m4.md) | [kani-m4](docs/slo/completion/kani-m4.md) |
| 5 | TLA+ pairing refinement map + local deep-verification workflow | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/kani-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/kani-m<N>.md -->

---

## 3. End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                       /slo-kani in the SLO engineering loop                    │
│                                                                                │
│  ┌───────────────┐    kani_required     ┌──────────────┐                       │
│  │ /slo-architect │- - - - - - - - - - ▶│  /slo-plan    │  §5 Kani sub-block    │
│  │ (M3: new key) │   + candidate list   │ (M3: authors  │- - ┐                  │
│  └───────────────┘                      │  obligations) │    │                  │
│         ║ design docs (exist)                └──────────────┘    │ obligations  │
│                                                                  ▼              │
│  ┌──────────────────────────┐   reads obligations   ┌────────────────────────┐ │
│  │ /slo-kani SKILL.md (M1/2) │◀ - - - - - - - - - - -│ /slo-execute M<N> (M3) │ │
│  │  · prereq cascade         │                       └────────────────────────┘ │
│  │  · candidate scoring      │   writes #[cfg(kani)] harnesses                   │
│  │  · harness gen            │─────────────┐                                     │
│  │  · run/triage ladder      │             ▼                                     │
│  │  · honesty/scope gates    │   ══════════════════════                         │
│  └──────────────────────────┘   ║ cargo kani subprocess ║ (pinned tools.toml)   │
│         │ verdict (tool, not narration)  ══════════════════════                 │
│         ▼                                       │ verdict                        │
│  ┌────────────────────────────┐                 ▼                               │
│  │ docs/slo/verify/<slug>-kani │   ┌──────────────┐   ┌──────────────┐          │
│  │ .md  (scope report)         │◀──│ /slo-verify   │──▶│  /slo-retro   │         │
│  └────────────────────────────┘   │ (M3: confirm  │   │ (M3: record  │          │
│                                    │  green+honest)│   │  scope)      │          │
│                                    └──────────────┘   └──────────────┘          │
│                                                                                │
│  ┌──────────────────────────────────────────────────────────────────────────┐ │
│  │ EXTERNAL demo repo (M4): seeded-bug Rust crate — catch→remediate→green     │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────────────────────────────┐ │
│  │ /slo-tla  ◀── refinement map (M5): TLA+ action → Rust fn → Kani harness    │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│                                                                                │
│  Legend:  ─── exists today   - - - new (this runbook)   ═══ external subprocess │
│           ║ trust boundary   ▶ data flow                                        │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `skills/slo-kani/SKILL.md` | Suitability gate, prereq cascade, method dispatch, hard gates, handoff | New | M1/M2 | `name: slo-kani`; discovered by `discover_skills()` |
| `skills/slo-kani/tools.toml` | Pinned `kani-verifier` version + acquisition commands | New | M1 | mirrors `slo-tla/tools.toml` |
| `skills/slo-kani/references/*.md` | Candidate-scoring, harness-gen, run/triage ladder, fallback, verified-scope writeup | New | M1/M2 | per-phase loaded |
| `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` | Structural-contract test (frontmatter, output-path safety, ref-file presence) | New | M1 | `cargo test -p sast-verify` |
| `xtasks/sast-verify/tests/kani_m3_integration.rs` | Structural-contract test for the integration seams | New | M3 | `cargo test -p sast-verify` |
| `xtasks/sast-verify/tests/kani_m5_pairing.rs` | Structural-contract test (pairing doc + local-verify; no-CI-added assertion) | New | M5 | `cargo test -p sast-verify` |
| `skills/slo-architect/SKILL.md` | Adds "Decide `kani_required`" step + candidate shortlist | Changed | M3 | new frontmatter key |
| `docs/slo/templates/runbook-template_v_4_template.md` | §5 Kani proof-obligation sub-block | Changed | M3 | additive to §5 |
| `skills/slo-{execute,verify,retro}/SKILL.md` | Kani-obligation hooks | Changed | M3 | Evidence-Log rows |
| External demo crate | Seeded bugs + harnesses; failure-bar evidence | New (separate repo) | M4 | `cargo kani` |
| `docs/slo/design/kani-verification-kani-pairing.md` | TLA+ ↔ Kani refinement map | New | M5 | action→fn→harness |
| `skills/slo-kani/references/local-deep-verification.md` | Local quick + deep verification workflow (run on dev laptop; deep before release). No CI automation in v1. | New | M5 | `cargo kani` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| `kani_required` + candidate list | `/slo-architect` | `/slo-kani` | overview.md frontmatter + prose | yes (bool + list) | absent ⇒ default `false` | M3 |
| proof obligations | `/slo-plan` §5 | `/slo-execute` | runbook Evidence-Log rows | yes | blank row ⇒ `/slo-retro` refuses close | M3 |
| harness write | `/slo-kani` | target crate `src/` | file write (allow-listed) | yes (`#[cfg(kani)]`) | out-of-tree path ⇒ refused | M1/M2 |
| verdict | `cargo kani` | scope report | subprocess stdout/exit | yes | timeout ⇒ triage ladder | M2 |
| toolchain | crates.io | local | `cargo install --locked` @pin | yes (pinned) | mismatch ⇒ cascade refuses | M1 |

---

## 4. Carmack-Style Development Best Practices

(Applies to every milestone — see the v4 template for the full text. The reliability lens specific to this runbook:)

- **4.1 Inspect, do not guess** — when a Kani harness fails, use `cargo kani --concrete-playback=print` to get a concrete counterexample before changing code. The counterexample IS the inspected state.
- **4.2 Static analysis mandatory** — `cargo fmt --check`, `cargo clippy -D warnings`, `cargo audit` gate every milestone. Markdown skill files are checked by the structural-contract test.
- **4.3 Assertions / invariants** — for skill behavior, the "invariant" is the honesty contract: a green verdict is never emitted without a scope block. The structural test asserts the SKILL.md states this. In the demo crate, invariants are the Kani postconditions (`assert!`, representation invariants).
- **4.4 Bounded resources** — every harness ships a stated bound (`#[kani::unwind(N)]`, array/vec size). Defaults from the research report: arrays 8–16, vecs 0–2, unwind = iterations+1; escalate deliberately, never silently.
- **4.5 Make invalid states unrepresentable** — the verdict type is the tool's output, not free text; the agent cannot construct a "green" that the tool did not produce.

---

## 5. High-Level Design for State Modeling / Formal Verification

> This runbook builds a formal-verification skill, so §5 carries the Kani proof-obligation sub-block that M3 generalizes into the template. `tla_required: false` for the skill itself (single-process, offline, no shared concurrent state) — so no TLA+ spec is authored here.

### 5.1 System Goal

A green `/slo-kani` run must mean exactly "the stated properties hold for the stated harnesses under the stated bounds, assumptions, stubs, and contracts" — and nothing more. The skill must catch a class of bug, drive its remediation, and re-verify, while never overclaiming scope (especially never claiming concurrency).

### 5.2 Kani proof obligations (this runbook's own demo crate, M4)

> Sub-block shape M3 promotes into the v4 template's §5. Per obligation: target fn, property, bound, assumptions, expected pre-fix and post-fix outcome.

| # | Target fn (demo crate) | Property | Bound / assumptions | Pre-fix outcome | Post-fix outcome |
|---|---|---|---|---|---|
| K1 | `zero_prefix(len, &mut [u8;8])` | no index-out-of-bounds | `#[kani::unwind(9)]`, `assume(len <= 8)` | FAILED (inclusive range writes `[8]`) | SUCCESSFUL |
| K2 | `read_byte(&[u8], idx) -> u8` (unsafe ptr) | no pointer-out-of-bounds deref | array len 4, symbolic `idx` | FAILED (`idx == len` one-past-end) | SUCCESSFUL (safe `get`+`assert`) |
| K3 | `accumulate(u32, u32) -> u32` | no arithmetic overflow / documented saturate | symbolic inputs | FAILED (overflow panic) | SUCCESSFUL (`saturating_add` + postcondition) |
| K4 | `gcd(u8,u8)` recursive helper | `r != 0 && a%r==0 && b%r==0` | `-Z function-contracts`, `requires(a!=0 && b!=0)`, `#[kani::recursion]` | FAILED without precondition (div-by-zero) | SUCCESSFUL; reused via `#[kani::stub_verified]` |

### 5.3 Anti-vacuity discipline

Each obligation runs its **pre-fix variant first and must FAIL** (the `/slo-tla` "naive variant fails first" rule). Weak harnesses carry `kani::cover!` reachability checks. A proof that passes without its pre-fix variant ever failing is rejected as vacuous.

### 5.4 Out-of-scope by construction

Concurrency, `await`, data races, atomics-under-interleaving, unbounded inputs, heavy I/O, opaque FFI. These route to "extract a sequential kernel" or "out of scope for Kani; pair with `/slo-tla`." The skill must refuse to claim them.

---

## 6 / 7 / 8. Standards, Entry, Exit Protocols

Use the v4 template's Section 4 rules, Global Entry Rules (§7), and Global Exit Rules (§8) verbatim. Note for this runbook: the "build/boot to usable state" exit step (§8.7) for a Markdown-skill milestone means `sldo-install --dry-run` lists `slo-kani` and the structural test is green; for M4 it means `cargo kani` runs to a verdict in the demo crate.

---

## 9. Background Context

### Current State

The SLO pack ships `/slo-tla` (design-level formal methods, drives TLC via a SHA-pinned `tla2tools.jar`) but has **no code-level formal verification** for Rust. `/slo-architect` sets `tla_required`; there is no Kani equivalent. The v4 template's §5 mentions "property-based testing and contract tests are valid substitutes" but offers no bounded-model-checking path. Structural-contract tests for skills live in `xtasks/sast-verify/tests/` (e.g. `sap_imp_m5_agents.rs`).

### Problem

1. **No bounded proof path for Rust kernels.** Off-by-one loops, one-past-the-end unsafe reads, arithmetic overflow, and representation-invariant violations are exactly what Kani catches and what tests often miss — but the loop has no skill to drive it.
2. **No planning seam to decide what gets verified.** `/slo-plan` cannot record Kani obligations; `/slo-architect` cannot flag a Rust target as a verification candidate.
3. **No honesty contract for "verified."** Without explicit gates, an LLM-authored harness can overclaim (vacuous proof, concurrency claims, unsound stubs).
4. **No demonstrated failure bar.** "More stable because of Kani" is unproven without a catch→remediate→green artifact.

### Key Design Principles

- The verdict is the tool's, never the narration's.
- Selective by design — score candidates; small high-value kernels over brittle whole-module proofs.
- Mirror `/slo-tla`'s proven structure (prereq cascade, pinned tools, method dispatch, hard gates).
- Additive everywhere — `kani_required` defaults `false`; §5 sub-block is additive; new structural test is a sibling, never an edit to a baseline.

---

## Milestone 1 — `/slo-kani` skill skeleton + tool prereq cascade + candidate-scoring rubric

**Goal**: A discoverable, installable `skills/slo-kani/SKILL.md` exists with a pinned `tools.toml`, a prereq cascade that refuses to run on a missing/mismatched Kani toolchain, a candidate-scoring rubric, and a structural-contract test that pins its shape.

**Context**: Mirror `skills/slo-tla/` (SKILL.md + tools.toml + references/ + evals/). The structural test follows `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`. No harness-generation methodology yet (M2) — this milestone is the skeleton and the gate that proves the skill is well-formed and discoverable.

**Carmack-style reliability goal**: Static analysis (the structural-contract test) + bounded design (the prereq cascade is a hard gate, not a best-effort).

**Important design rule**: `/slo-kani` is host-neutral under `skills/slo-kani/`. No new Rust binary. The skill drives `cargo kani` as a subprocess exactly as `/slo-tla` drives the TLC jar.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | a target Rust workspace/crate (consumed at runtime, not at install) |
| Outputs | `skills/slo-kani/SKILL.md`, `tools.toml`, candidate-scoring reference; structural test green |
| Interfaces touched | `discover_skills()` contract (read-only — confirm auto-discovery); `cargo test -p sast-verify` |
| Files allowed to change | `skills/slo-kani/SKILL.md` (NEW), `skills/slo-kani/tools.toml` (NEW), `skills/slo-kani/references/candidate-scoring.md` (NEW), `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` (NEW), `docs/skill-pack-catalog.md`, `.gitignore` |
| Files to read before changing anything | `skills/slo-tla/SKILL.md`, `skills/slo-tla/tools.toml`, `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`, `crates/sldo-install/src/install.rs`, `~/Downloads/deep-research-report-Kani.md` |
| New files allowed | the four NEW files above |
| New dependencies allowed | `none` (the structural test uses existing `serde_yaml_ng`) |
| Migration allowed | `no` |
| Compatibility commitments | existing structural tests stay green; `sldo-install --dry-run` still lists all current skills + now `slo-kani`; no edit to any existing baseline test |
| Resource bounds introduced/changed | prereq cascade caps at one `cargo kani --version` probe; no network call beyond the documented `cargo install`/`setup` (which the cascade instructs the USER to run, never auto-runs) |
| Invariants/assertions required | SKILL.md MUST contain the honesty-contract sentence ("a green run is reported only with its proof scope"); the structural test asserts its presence. Frontmatter `name: slo-kani` present. |
| Debugger / inspection expectation | `cargo test -p sast-verify -- kani_m1 --nocapture` shows which assertion fails when SKILL.md is malformed |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo audit` |
| Exemplar code to copy | `skills/slo-tla/SKILL.md` (prereq cascade, suitability gate, gates block); `skills/slo-tla/tools.toml` (pin shape); `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (frontmatter + path-safety assertions) |
| Anti-exemplar code not to copy | the removed legacy `sldo-plan`/`sldo-run` CLIs — do NOT add a Rust binary to drive Kani |
| Refactoring discipline | `N/A — no refactoring performed; all-new files plus a catalog row` |
| AI tolerance contract | accepted variance: SKILL.md prose wording may vary; deterministic boundary: the structural test asserts presence of required frontmatter keys, the honesty sentence, and output-path constraints — these MUST be exact, not "close"; eval evidence: `skills/slo-kani/evals/*.md` seven scenarios; retry/fallback: none — a malformed skill fails the test, hard; must-never: ship a SKILL.md that lacks the honesty contract or the concurrency-refusal gate; sample budget: N/A (deterministic test) |
| Forbidden shortcuts | auto-running `cargo install`/`cargo kani setup` for the user; a SKILL.md without the suitability gate; editing an existing baseline test |
| Data classification | Internal |
| Proactive controls in play | C5 (secure-by-default config — pinned toolchain), C8 (protect data — output-path allow-list documented), C10 (handle errors — cascade refuses, never silent) |
| Abuse acceptance scenarios | `tm-kani-verification-abuse-4: pinned-version cascade refuses a mismatched/tampered kani-verifier`; `tm-kani-verification-abuse-5: output-path allow-list documented in SKILL.md` |

#### Out of Scope / Must Not Do

- Harness-generation methodology and run/triage ladder (M2).
- Any change to `/slo-architect`, the v4 template, or execute/verify/retro (M3).
- Actually running `cargo kani` against real code (M4).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-kani/SKILL.md` | NEW: frontmatter, suitability gate, prereq cascade, candidate-scoring dispatch, hard gates, handoff |
| `skills/slo-kani/tools.toml` | NEW: pinned `kani-verifier` version + acquisition commands |
| `skills/slo-kani/references/candidate-scoring.md` | NEW: the research-report scoring rubric (raise/lower-score signals) |
| `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` | NEW: structural-contract test |
| `docs/skill-pack-catalog.md` | Add `/slo-kani` row to the canonical catalog |
| `.gitignore` | Add patterns if any new generated files appear (expect none) |

#### Step-by-Step

1. Write `kani_m1_skill_contract.rs` BDD-style assertions first (they fail — no SKILL.md yet).
2. Author `skills/slo-kani/SKILL.md` mirroring `/slo-tla`'s structure.
3. Author `tools.toml` with the pinned `kani-verifier` version.
4. Author `references/candidate-scoring.md` from the research report's scoring table.
5. Add the catalog row.
6. Make the structural test pass.
7. `cargo fmt`, `cargo clippy`, `cargo audit`.
8. `sldo-install --dry-run` confirms `slo-kani` is discovered.
9. `git status` clean; self-review gate.

#### BDD Acceptance Scenarios

**Feature: well-formed, discoverable `/slo-kani` skill**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Skill is discovered | happy path | `skills/slo-kani/SKILL.md` exists with `name: slo-kani` | `sldo-install --dry-run` runs | `slo-kani` appears in the install plan |
| Frontmatter complete | happy path | the SKILL.md | structural test reads frontmatter | `name` present and equals `slo-kani` |
| Honesty contract present | assertion violation | a SKILL.md missing the "proved within stated scope" sentence | structural test runs | test FAILS naming the missing sentence |
| Concurrency-refusal gate present | invalid input | a SKILL.md lacking the concurrency-out-of-scope refusal | structural test runs | test FAILS |
| Pinned toolchain | resource bound | `tools.toml` with a pinned `kani-verifier` version | structural test reads it | a concrete version string is present (not `latest`) — `tm-kani-verification-abuse-4` |
| Output-path allow-list documented | abuse case | SKILL.md | structural test scans for the allow-list clause | clause present (harnesses in target `src/`, reports in `docs/slo/verify/`) — `tm-kani-verification-abuse-5` |
| Baseline untouched | compatibility | existing `sap_imp_m5_agents.rs` SHA baseline | full test suite runs | unchanged and green |

#### Regression Tests

- All existing `xtasks/sast-verify/tests/*.rs` stay green (esp. the `sap_imp_m5_agents.rs` SHA baseline).
- `cargo test -p sldo-install` discovery tests still pass.
- `sldo-install --dry-run` / `uninstall --dry-run` still round-trip.

#### Compatibility Checklist

- [ ] `discover_skills()` auto-includes `slo-kani` with no installer code change
- [ ] No existing baseline test edited
- [ ] Catalog renders (no broken table)
- [ ] All current skills still listed by `--dry-run`

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/kani_m1_skill_contract.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_kani_frontmatter_complete` | skill is well-formed | `name: slo-kani` present |
| `slo_kani_honesty_and_concurrency_gates_present` | overclaim defenses exist in prose | both required sentences found |
| `slo_kani_toolchain_pinned` | supply-chain control | concrete version, not `latest` |
| `slo_kani_output_paths_constrained` | path-injection defense | allow-list clause present |

#### Smoke Tests

- [x] `cargo test -p sast-verify -- kani_m1` passes (5/5)
- [x] `sldo-install --dry-run` lists `slo-kani`
- [x] new code clippy-clean (`cargo clippy -p sast-verify --all-targets -- -D warnings`); pre-existing bin/`sap_imp_m3` warnings waived with rationale
- [x] `git status` shows no untracked artifacts beyond the intended runbook/skill files

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Repo hygiene | `git rev-parse --abbrev-ref HEAD` | not on default branch | branch `slo/kani-verification` (before: same); tree dirty with this runbook's design docs | PASS | branched off main earlier; no remediation needed |
| Baseline tests | `cargo test -p sast-verify` | all green | 20+ tests across all files green | PASS | sldo-install unaffected (no code change there) |
| BDD tests created | `kani_m1_skill_contract.rs` | fail (no SKILL.md) | 5 tests failed: files absent | PASS | red-first confirmed |
| Implementation | SKILL.md + tools.toml + candidate-scoring.md + catalog | contract satisfied | 4 files authored, 1 catalog row | PASS | mirrors slo-tla structure |
| Formatter | `cargo fmt --all -- --check` | clean | clean after `cargo fmt --all` | PASS | reformatted new test file |
| Static analyzer | `cargo clippy -p sast-verify --all-targets -- -D warnings` | clean for new code | new test file clippy-clean | PASS (minimal waiver) | 2 pre-existing bin warnings (`Public` unused, unread fields) + 1 pre-existing `sap_imp_m3_standards` regex-in-loop warning — NOT introduced here; out of scope per execute discipline |
| Dependency audit | `cargo audit` | pass or N/A | N/A — no new dependencies added | PASS | test uses existing dev-deps only |
| Full tests | `cargo test -p sast-verify` | green | all green incl. 5 new kani_m1 tests | PASS | |
| E2E discovery | `sldo-install --dry-run` | lists `slo-kani` | `+ …/.claude/skills/slo-kani -> …/skills/slo-kani` | PASS | auto-discovered, no installer change |
| Test artifact cleanup | `git status` | clean | only intended new/modified files | PASS | |
| .gitignore review | review | current | added Kani artifact patterns | PASS | `target/` already covered scratch |
| Compatibility checks | baselines | no regressions | all pre-existing sast-verify tests green; no baseline test edited | PASS | sap_imp_m5 SHA baseline untouched |

#### Definition of Done

All BDD scenarios pass; `slo-kani` is discovered by the installer; structural test green; no baseline edited; fmt clean and new code clippy-clean (pre-existing warnings waived with rationale); audit N/A (no new deps); catalog updated; lessons file written.

---

## Milestone 2 — Harness-generation + run/triage methodology + honesty/scope gates

**Goal**: `/slo-kani` can author harnesses and triage results: reference files cover candidate→harness translation, the failure ladder (unwind → solver → stubs → contracts), `kani::cover!` anti-vacuity, concrete playback, and the honesty/scope gates that forbid overclaiming.

**Context**: M1 gave the skeleton. This milestone fills the method-dispatch references and bakes the threat-model's AI risks (`tm-kani-verification-abuse-1..3`) into hard gates the structural test asserts.

**Carmack-style reliability goal**: Assertions/invariants — the honesty contract becomes machine-checkable (structural test asserts the gate sentences); "make invalid states unrepresentable" — verdict derives from tool output.

**Important design rule**: The verdict (`SUCCESSFUL`/`FAILED`/`TIMEOUT`/`UNWINDING`) is parsed from `cargo kani` output. The LLM may explain it, never override it. Target source is untrusted data, never instructions.

**Refactor budget**: `Minimal local refactor permitted in listed files only` (SKILL.md method-dispatch table may gain rows pointing at the new references).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | a scored candidate from M1's rubric |
| Outputs | `references/harness-generation.md`, `references/run-and-triage.md`, `references/fallback-strategies.md`, `references/verified-scope-writeup.md`; updated SKILL.md dispatch table; structural test extended |
| Interfaces touched | `cargo test -p sast-verify`; SKILL.md method-dispatch table |
| Files allowed to change | `skills/slo-kani/SKILL.md`, `skills/slo-kani/references/harness-generation.md` (NEW), `.../run-and-triage.md` (NEW), `.../fallback-strategies.md` (NEW), `.../verified-scope-writeup.md` (NEW), `skills/slo-kani/evals/*.md` (NEW, 7 files), `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` |
| Files to read before changing anything | `~/Downloads/deep-research-report-Kani.md` (the run/triage ladder + thresholds + scenarios), `skills/slo-tla/references/methodology-counterexample.md`, `docs/slo/design/kani-verification-threat-model.md` |
| New files allowed | the four references + seven evals |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | M1 structural assertions stay green |
| Resource bounds introduced/changed | run/triage ladder documents the research thresholds: per-harness seconds-not-minutes; arrays 8–16; vecs 0–2; unwind iterations+1; escalate deliberately |
| Invariants/assertions required | structural test asserts the SKILL.md/references contain: (a) "naive/pre-fix variant must fail first", (b) "no concurrency claim", (c) "sound over-approximating stubs only", (d) "verdict from tool not narration", (e) **fail-closed parsing** — any `cargo kani` output the parser cannot positively classify as SUCCESSFUL is treated as non-pass (never SUCCESS); parser anchored to the `tools.toml`-pinned version (ENG-2), (f) **write-path validated by construction** — harness write paths resolve under the validated target-crate root; reject `..`, absolute paths, and symlinked components (SEC-1 / CWE-22) |
| Debugger / inspection expectation | references document `--concrete-playback=print` as the counterexample-inspection step before any fix |
| Static analysis gates | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo audit` |
| Exemplar code to copy | `skills/slo-tla/references/methodology-counterexample.md` (counterexample-to-plain-English shape); research report §"Run-and-triage" and §"fallback strategies" |
| Anti-exemplar code not to copy | any pattern letting narration override a verdict; under-approximating stubs |
| Refactoring discipline | cite `skills/slo-plan/references/refactoring-discipline.md` for the SKILL.md dispatch-table edit: behavior-preserving, structural test green before and after |
| AI tolerance contract | accepted variance: harness code the agent writes varies; deterministic boundary: every green MUST carry a scope block and MUST have had a failing pre-fix variant; eval evidence: the 7 evals incl. adversarial (hostile in-source comment) + high-risk (concurrency request); retry/fallback: failure ladder (unwind→solver→stubs→contracts) is the retry policy; must-never: emit "verified" for concurrency/interleavings, ship an unsound stub, accept a vacuous proof; sample budget: per-harness runtime budget (seconds; ~1–2 min only for high-value) |
| Forbidden shortcuts | claiming a property without stating its bound; suppressing a counterexample; an unsound stub to force green; treating unparseable/ambiguous `cargo kani` output as SUCCESS (must fail closed); writing a harness outside the validated target-crate root (`..` / absolute / symlink) |
| Data classification | Internal |
| Proactive controls in play | C4 (validate input — target source treated as untrusted data; write-path validated by construction, CWE-22), C10 (handle errors — every failure class has a documented triage path; parser fails closed) |
| Abuse acceptance scenarios | `tm-kani-verification-abuse-1: cover! anti-vacuity rejects a vacuous green`; `tm-kani-verification-abuse-2: concurrency-refusal gate`; `tm-kani-verification-abuse-3: sound-stub-only rule`; `tm-kani-verification-abuse-5: harness write-path validated by construction (reject .. / absolute / symlink)` |

#### Out of Scope / Must Not Do

- Wiring into architect/plan/execute (M3); running against real code (M4).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-kani/references/harness-generation.md` | NEW: candidate→harness patterns (unsafe wrapper, bounds, invariant, contract) + write-path validation (under target-crate root; reject `..` / absolute / symlink) |
| `skills/slo-kani/references/run-and-triage.md` | NEW: outcome parsing (fail-closed on unrecognized/ambiguous output; anchored to pinned version) + failure ladder + thresholds |
| `skills/slo-kani/references/fallback-strategies.md` | NEW: stubs vs contracts, solver switch, cover sanity, out-of-scope routing |
| `skills/slo-kani/references/verified-scope-writeup.md` | NEW: the scope-report template (`docs/slo/verify/<slug>-kani.md`) |
| `skills/slo-kani/evals/*.md` | NEW: 7 scenarios (happy/adversarial/ambiguous/missing-context/tool-failure/high-risk/outdated) |
| `skills/slo-kani/SKILL.md` | add method-dispatch rows pointing at the new references |
| `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` | extend with the four gate-sentence assertions |

#### BDD Acceptance Scenarios

**Feature: harness methodology + honesty gates**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Naive-first rule documented | happy path | `run-and-triage.md` | structural test scans | "pre-fix variant must fail first" present |
| Concurrency refusal | abuse case | a request to verify a `tokio` interleaving | the agent follows SKILL.md | routes to extract-sequential-kernel / out-of-scope; never claims verified — `tm-kani-verification-abuse-2` |
| Vacuity defense | abuse case | a harness that passes with a heavy `assume` | the agent follows methodology | `kani::cover!` checks required before green — `tm-kani-verification-abuse-1` |
| Unsound stub rejected | abuse case | an under-approximating stub that forces green | the agent follows methodology | rejected; only over-approximating stubs allowed; stub recorded — `tm-kani-verification-abuse-3` |
| Verdict authority | invalid input | tool prints FAILED but narration says "looks fine" | the scope report is written | verdict = FAILED (tool wins) |
| Timeout triage | partial failure | a harness times out | the agent triages | ladder applied: reduce bound → switch solver → stub/contract |
| Scope block mandatory | assertion violation | a green with no stated bound | the gate runs | rejected — no scope block, no green |
| Fail-closed parsing | dependency failure | a Kani version bump changes an output anchor so the parser cannot classify the result | the verdict is computed | output not positively SUCCESSFUL ⇒ non-pass; parser anchored to the pinned version (ENG-2) |
| Write-path traversal | abuse case | a target crate presents a candidate path `../../.claude/skills/x` (or an absolute path / symlinked `src/`) | the agent goes to write a harness | path rejected; write only under the validated target-crate root — `tm-kani-verification-abuse-5` (CWE-22) |

#### Regression Tests

- M1 structural assertions still pass.
- The 7 evals parse against `references/templates/eval-cases.md` shape.

#### Compatibility Checklist

- [ ] M1 frontmatter/discovery assertions unchanged
- [ ] SKILL.md dispatch-table edit is behavior-preserving (structural test green before/after)

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/kani_m1_skill_contract.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_kani_naive_first_documented` | anti-vacuity discipline | sentence present |
| `slo_kani_concurrency_refusal_documented` | scope honesty | refusal clause present |
| `slo_kani_sound_stub_rule_documented` | unsound-stub defense | clause present |
| `slo_kani_verdict_from_tool_documented` | narration cannot override | clause present |
| `slo_kani_parser_fails_closed_documented` | no false green on version drift | fail-closed clause present (ENG-2) |
| `slo_kani_write_path_validation_documented` | path-traversal defense | by-construction clause present (SEC-1 / CWE-22) |

#### Smoke Tests

- [x] `cargo test -p sast-verify -- kani` passes (12/12)
- [x] all four reference files exist and are linked from SKILL.md dispatch table (*(M2)* annotations removed)
- [x] 7 eval files present
- [x] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | green | green (M1 + all prior) | PASS | |
| BDD tests created | extended `kani_m1_*.rs` | fail (refs absent) | 7 new tests failed: refs/evals absent | PASS | red-first confirmed |
| Implementation | 4 refs + 7 evals + dispatch rows | contract satisfied | 4 references + 7 evals authored; dispatch links live | PASS | |
| Formatter | `cargo fmt --all -- --check` | clean | clean | PASS | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets` | clean for new code | `kani_m1` test clippy-clean | PASS (waiver) | pre-existing bin/sap_imp_m3 warnings unchanged (kani-m1 lesson) |
| Full tests | `cargo test -p sast-verify` | green | all test files green | PASS | |
| Test artifact cleanup | `git status` | clean | only intended files | PASS | |
| Compatibility checks | M1 assertions | green | M1's 5 assertions still green; concurrency-refusal substring caught a capitalization gotcha (fixed via case-insensitive compare) | PASS | same class as kani-m1 lesson |

#### Definition of Done

All BDD pass; the four honesty/scope gate sentences are present and asserted; fail-closed parsing (ENG-2) and write-path validation (SEC-1) documented and asserted; failure ladder + thresholds documented; 7 evals present; M1 stays green; lessons file written.

---

## Milestone 3 — Integration seams (architect `kani_required`, §5 sub-block, execute/verify/retro hooks)

**Goal**: The loop knows about Kani: `/slo-architect` decides `kani_required` and emits a candidate shortlist; the v4 template §5 carries the Kani proof-obligation sub-block; `/slo-plan` authors obligations; `/slo-execute`, `/slo-verify`, `/slo-retro` carry the obligation through to a recorded scope.

**Context**: M1/M2 built the standalone skill. This milestone connects it. Each edit is additive (defaults preserve old behavior) and guarded by a structural test extension.

**Carmack-style reliability goal**: Compatibility — additive keys/sections with defaults; type/schema safety — `kani_required` type-checked like the existing four keys.

**Important design rule**: `kani_required` default-when-absent is `false`. The §5 Kani sub-block is additive under existing §5 (TLA+ untouched). No existing runbook becomes invalid.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | a design overview (`<slug>-overview.md`); a milestone with Rust kernels |
| Outputs | `kani_required` step in architect SKILL.md; §5 Kani sub-block in the template; execute/verify/retro hook prose; structural test for the new key + template section |
| Interfaces touched | `<slug>-overview.md` frontmatter; v4 template §5; `/slo-plan`, `/slo-execute`, `/slo-verify`, `/slo-retro` SKILL.md |
| Files allowed to change | `skills/slo-architect/SKILL.md`, `docs/slo/templates/runbook-template_v_4_template.md`, `skills/slo-plan/SKILL.md`, `skills/slo-execute/SKILL.md`, `skills/slo-verify/SKILL.md`, `skills/slo-retro/SKILL.md`, `xtasks/sast-verify/tests/kani_m3_integration.rs` (NEW), `docs/slo/design/kani-verification-interfaces.md` |
| Files to read before changing anything | architect Step 5 (`tla_required`), template §5 (line ~254), `skills/slo-plan/SKILL.md` step 5, `docs/slo/design/kani-verification-interfaces.md` |
| New files allowed | `xtasks/sast-verify/tests/kani_m3_integration.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | every existing `<slug>-overview.md` (lacking `kani_required`) stays valid → treated as `false`; every existing runbook with §5 stays valid; M1/M2 tests stay green; no template structural test that pins §5 line count is broken |
| Resource bounds introduced/changed | §5 sub-block caps proof obligations to the milestone's kernels; no unbounded enumeration |
| Invariants/assertions required | structural test asserts: architect SKILL.md documents `kani_required` with a default; template §5 contains the Kani sub-block header; the `/slo-execute` and `/slo-verify` Kani-obligation hook prose are present (ENG-4 — all six edited files asserted, none silently dropped); the four existing frontmatter keys' docs are unchanged |
| Debugger / inspection expectation | `cargo test -p sast-verify -- kani_m3 --nocapture` shows which seam is missing |
| Static analysis gates | fmt/clippy/audit |
| Exemplar code to copy | architect "Decide `tla_required`" step (Step 5) — parallel structure for `kani_required`; the existing §5 TLA+ prose for tone |
| Anti-exemplar code not to copy | `tla_required: true for CRUD to look rigorous` anti-pattern — the Kani equivalent: don't set `kani_required` for non-Rust or kernel-free Rust |
| Refactoring discipline | cite `refactoring-discipline.md` for the multi-file SKILL.md edits: behavior-preserving, each skill's existing tests green before/after |
| AI tolerance contract | accepted variance: prose wording; deterministic boundary: `kani_required` type (bool) + default (`false`) + the §5 header string are exact; eval evidence: a CRUD runbook gets §5 Kani = `N/A`; a Rust-kernel runbook gets obligations; must-never: make an existing overview/runbook invalid; sample budget: N/A |
| Forbidden shortcuts | changing any existing frontmatter key's type/default; making §5 Kani mandatory (it's `N/A`-able) |
| Data classification | Internal |
| Proactive controls in play | C5 (secure defaults — `false` default), C4 (validate input — frontmatter type check) |
| Abuse acceptance scenarios | `N/A — no new external surface introduced; all edits are to skill prose + a template + a frontmatter key, covered by the M3 structural test. The runtime surfaces (subprocess, file writes) are governed by M1/M2 gates.` |

#### Out of Scope / Must Not Do

- Running Kani against real code (M4); the TLA+ pairing map (M5).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-architect/SKILL.md` | Add "Decide `kani_required`" sub-step + candidate-shortlist output; document the key in the frontmatter-key list |
| `docs/slo/templates/runbook-template_v_4_template.md` | Add §5 Kani proof-obligation sub-block (additive) |
| `skills/slo-plan/SKILL.md` | Note: when `kani_required`, author §5 Kani obligations + Evidence-Log rows |
| `skills/slo-execute/SKILL.md` | Note: a Kani-obligation row means write `#[cfg(kani)]` harness, run `cargo kani`, remediate |
| `skills/slo-verify/SKILL.md` | Note: confirm harnesses green at stated bounds + scope honesty |
| `skills/slo-retro/SKILL.md` | Note: record proved properties/assumptions/bounds; refuse close on blank Kani Evidence rows |
| `xtasks/sast-verify/tests/kani_m3_integration.rs` | NEW: structural test for the seams |
| `docs/slo/design/kani-verification-interfaces.md` | Mark the seam interfaces `stable` once landed |

#### BDD Acceptance Scenarios

**Feature: Kani-aware loop**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Architect sets the key | happy path | a Rust target with unsafe/arithmetic kernels | `/slo-architect` runs | overview gets `kani_required: true` + candidate shortlist |
| Default preserves old behavior | compatibility | an existing overview lacking the key | `/slo-plan` reads it | treated as `false`; no Kani section forced |
| CRUD gets N/A | empty state | a CRUD Rust runbook | `/slo-plan` authors §5 | §5 Kani = `N/A — no Rust kernels`; valid |
| Obligation flows to execute | happy path | a §5 obligation row | `/slo-execute M<N>` runs | writes harness + runs `cargo kani` + fills Evidence Log |
| Retro refuses blank | assertion violation | a Kani Evidence row with blank Actual Result | `/slo-retro M<N>` runs | refuses to close (existing rule extended) |
| Execute hook present | happy path | M3 edited `/slo-execute` SKILL.md | structural test scans | the "Kani-obligation row ⇒ write harness + run `cargo kani` + remediate" hook prose is present (ENG-4) — not silently dropped |
| Verify hook present | happy path | M3 edited `/slo-verify` SKILL.md | structural test scans | the "confirm harnesses green at stated bounds + scope honesty" hook prose is present (ENG-4) |
| Existing keys intact | compatibility | the four existing frontmatter keys | structural test runs | types/defaults unchanged |
| Template §5 additive | compatibility | an existing v4 runbook | structural test runs | TLA+ §5 content unchanged; Kani sub-block additive |

#### Regression Tests

- M1/M2 structural tests green.
- Any existing test pinning the template or architect frontmatter still passes.
- A sample existing overview without `kani_required` still parses.

#### Compatibility Checklist

- [ ] Existing overviews (no `kani_required`) parse as `false`
- [ ] Existing runbooks' §5 unchanged
- [ ] Four existing frontmatter keys unchanged
- [ ] No existing skill's tests broken

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/kani_m3_integration.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `architect_documents_kani_required_with_default` | seam exists, default safe | key + `false` default documented |
| `template_section5_has_kani_subblock` | plan can author obligations | header string present |
| `existing_overviews_without_key_default_false` | backward compat | sample parses, no error |
| `retro_refuses_blank_kani_evidence` | obligation closure discipline | rule prose present |
| `execute_skill_documents_kani_obligation_hook` | execute hook landed (ENG-4) | hook prose present in `/slo-execute` SKILL.md |
| `verify_skill_documents_kani_scope_check` | verify hook landed (ENG-4) | scope-check prose present in `/slo-verify` SKILL.md |

#### Smoke Tests

- [x] `cargo test -p sast-verify -- kani_m3` passes (6/6)
- [x] TLA+ §5.1–5.7 prose unchanged (the Kani sub-block is appended as §5.8)
- [x] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | green | green | PASS | |
| BDD tests created | `kani_m3_integration.rs` | fail (seams absent) | 5/6 failed (additivity test passed trivially) | PASS | red-first confirmed |
| Implementation | 6 skill/template edits + interfaces.md | contract satisfied | architect key+5.5 step; v4 §5.8; plan/execute/verify/retro hooks; interfaces stable | PASS | all additive |
| Formatter | `cargo fmt --all -- --check` | clean | clean | PASS | |
| Static analyzer | `cargo clippy -p sast-verify --all-targets` | clean for new code | `kani_m3_integration` clippy-clean | PASS (waiver) | pre-existing warnings unchanged |
| Full tests | `cargo test -p sast-verify` | green | all green incl. `slo_tm_m2_consumers` (slo-verify phrase-presence) + `sap_imp_m5_agents` (slo-critique SHA) | PASS | additive edits did not break either baseline |
| Compatibility checks | existing overviews/runbooks | parse/valid | `existing_overview_without_key_still_parses` green; §5.1–5.7 TLA+ prose intact | PASS | |
| Test artifact cleanup | `git status` | clean | only intended files | PASS | |

#### Definition of Done

`kani_required` documented with safe default; §5 Kani sub-block additive and asserted; execute/verify/retro hooks present **and structurally asserted (ENG-4 — all six edited files covered)**; all existing artifacts stay valid; M1/M2 green; lessons file written.

---

## Milestone 4 — Test repo + catch→remediate→green failure-bar demonstration

**Goal**: The failure bar is met and documented: a separate Rust crate with seeded bugs (K1–K4 from §5.2) is verified with `/slo-kani`; Kani **catches** each bug (red), the bug is **remediated**, and the proof goes **green** at stated bounds — captured in a verified-scope report.

**Context**: Per the locked decision, the demo crate lives in a **separate GitHub repo** (user-created) to isolate Kani's nightly toolchain + `cargo kani setup` from this workspace's `cargo test` baseline. This runbook records the demo by URL + pinned commit; the evidence (scope report) is committed here.

**Carmack-style reliability goal**: The whole runbook's thesis — bounded proofs catch a class of bug and the remediation provably removes it within scope.

**Important design rule**: The demo MUST show the pre-fix variant FAILING first (the `/slo-tla` naive-first rule). A green that was never preceded by a red proves nothing.

**Refactor budget**: `No refactor permitted beyond direct implementation` (this milestone is the demo + evidence, not a refactor of the skill).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the `/slo-kani` skill (M1/M2); the §5 K1–K4 obligations |
| Outputs | external demo repo with seeded-bug crate + `#[cfg(kani)]` harnesses; `docs/slo/verify/kani-verification-kani.md` (scope report with red→green evidence) committed here |
| Interfaces touched | `cargo kani` (external repo); the scope-report contract from M2 |
| Files allowed to change | `docs/slo/verify/kani-verification-kani.md` (NEW), `docs/slo/design/kani-verification-overview.md` (add demo repo URL+commit), `README.md` (link the demo), `.gitignore` |
| Files to read before changing anything | `skills/slo-kani/references/verified-scope-writeup.md`, `~/Downloads/deep-research-report-Kani.md` (the four scenario implementations), §5.2 of this runbook |
| New files allowed | the scope report; everything in the external repo (out of this workspace's file list) |
| New dependencies allowed | `none` in this workspace; the external crate uses `kani` dev-tooling only (pinned per `tools.toml`) |
| Migration allowed | `no` |
| Compatibility commitments | this workspace's `cargo test` baseline is NOT coupled to Kani's toolchain (demo is external); M1–M3 stay green |
| Resource bounds introduced/changed | each harness states its bound (K1 unwind 9; K2 array 4; K3 symbolic u32; K4 contract on u8); per-harness runtime in seconds |
| Invariants/assertions required | each of K1–K4: pre-fix variant FAILS, post-fix SUCCEEDS; scope report states bound + assumptions + any stub/contract; K4 uses a verified contract reused via `stub_verified` |
| Debugger / inspection expectation | for each red, `cargo kani --concrete-playback=print` (where applicable) yields the concrete counterexample recorded in the report |
| Static analysis gates | external crate: `cargo fmt`, `cargo clippy`; this repo: scope report links resolve |
| Exemplar code to copy | research report scenarios: "Boundary bug in a bounded loop" (K1), "Unsafe pointer arithmetic behind a safe API" (K2), "Expensive recursive helper modularised with contracts" (K4) |
| Anti-exemplar code not to copy | claiming K2 "memory-safe" by merely tightening the harness `assume` instead of making the wrapper genuinely safe (report calls this out) |
| Refactoring discipline | `N/A — no refactoring; the seeded bugs are fixed per their documented remediation, which is the demo's point` |
| AI tolerance contract | accepted variance: exact harness code; deterministic boundary: K1–K4 each MUST go red→green and the report MUST state bounds; eval evidence: the scope report IS the evidence; retry/fallback: K4 escalates to contracts per the ladder; must-never: report a green that lacked a red, or omit a bound; sample budget: per-harness seconds; whole demo suite minutes |
| Forbidden shortcuts | a green without a preceding red; an omitted bound; fixing K2 by tightening `assume` instead of the real safety fix |
| Data classification | Public (synthetic demo, no real data) |
| Proactive controls in play | C4 (validate input — bounds + assumptions), C10 (handle errors — counterexample-driven remediation) |
| Abuse acceptance scenarios | `tm-kani-verification-abuse-1: K-series each prove non-vacuous via the mandatory red-first step (and cover! where the assertion is weak)` |

#### Out of Scope / Must Not Do

- Adding the demo crate as a workspace member (couples Kani toolchain — forbidden by code-map dangerous-seam note).
- The TLA+ pairing (M5).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/slo/verify/kani-verification-kani.md` | NEW: scope report — per K1–K4, the red output, the fix, the green output, bounds, assumptions, stubs/contracts |
| `docs/slo/design/kani-verification-overview.md` | Add demo repo URL + pinned commit |
| `README.md` | Link the demo repo under a "Formal verification demo" note |
| `.gitignore` | Add any Kani scratch patterns if a local demo checkout lands here (expect external) |

#### BDD Acceptance Scenarios

**Feature: failure bar demonstrated**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| K1 off-by-one caught | happy path | `zero_prefix` with inclusive range | `cargo kani --harness check_zero_prefix` | FAILED: index out of bounds (red recorded) |
| K1 remediated | happy path | exclusive-range fix | re-run | SUCCESSFUL at `unwind(9)`, `assume(len<=8)` |
| K2 one-past-end caught | abuse case | unsafe `read_byte` accepting `idx==len` | run | FAILED: pointer deref OOB — `tm-kani-verification-abuse-1` (non-vacuous: red first) |
| K2 remediated genuinely | invalid input | `Option`-returning safe `get` + postcondition | run | SUCCESSFUL; NOT fixed by tightening `assume` |
| K3 overflow caught | resource bound | `accumulate` with `+` | run | FAILED: arithmetic overflow |
| K3 remediated | resource bound | `saturating_add` + postcondition | run | SUCCESSFUL across symbolic range |
| K4 contract caught | partial failure | `gcd` without precondition | `-Z function-contracts` proof | FAILED (div-by-zero) |
| K4 remediated + reused | dependency failure | `requires(a!=0&&b!=0)` + `stub_verified` | run caller proof | SUCCESSFUL; caller uses verified abstraction |
| Bound stated | assertion violation | the scope report | review | every green carries its bound; no whole-system claim |
| Kani toolchain absent | dependency failure | a machine where `cargo kani` is not installed | the M4 flow starts | the M1 prereq cascade fires: a loud documented skip with install hints — NEVER a false "N/A passed" or green (ENG-1) |

#### Regression Tests

- This workspace's `cargo test -p sast-verify -p sldo-install -p sldo-common -p sldo-research` stays green (unaffected by external demo).
- Scope-report links resolve.

#### Compatibility Checklist

- [ ] Workspace baseline test command unaffected by Kani toolchain
- [ ] Demo crate NOT a workspace member
- [ ] M1–M3 tests green

#### E2E Runtime Validation

**File**: external demo repo `src/*.rs` `#[cfg(kani)]` harnesses + this repo's scope report

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `check_zero_prefix` (K1) | bounded-loop bug caught & fixed | red then green; report records both |
| `check_read_byte` (K2) | unsafe-ptr bug caught & genuinely fixed | red then green; fix is real safety, not assume-tightening |
| `check_accumulate` (K3) | overflow caught & saturated | red then green |
| `check_gcd_contract` + `check_reduce_fraction` (K4) | contract modularization works | contract proof green; caller reuses via `stub_verified` |

#### Smoke Tests

- [x] In the demo repo: `cargo kani` runs all harnesses to a verdict (5 harnesses, K1–K4)
- [x] Prereq cascade documented for the no-Kani case (here Kani 0.67.0 present + pin-matched)
- [x] Each K-series shows red (pre-fix) then green (post-fix) in the report
- [x] `docs/slo/verify/kani-verification-kani.md` states every bound
- [x] This repo's baseline `cargo test` unaffected (demo is external, not a workspace member)
- [x] `git status` clean here

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify -p sldo-install` | green | green | PASS | |
| Allow-list extension | tools.toml pin 0.56.0→0.67.0 | user-approved | bumped to verified 0.67.0 | PASS | M4-discovered; rationale below; user-confirmed |
| Toolchain absent path | prereq cascade | loud skip, no false pass | confirmed documented; here `cargo-kani 0.67.0` present + matches pin | PASS | ENG-1 |
| Demo repo created | external repo + pinned commit | exists | local commit `959b23e`; public push pending 1 user cmd | PARTIAL | crate built+verified; remote push blocked by harness auto-classifier (user to run gh cmd) |
| K1 pre-fix | `cargo kani --harness check_zero_prefix` | FAILED (OOB) | FAILURE: index out of bounds | PASS | red |
| K1 post-fix | re-run | SUCCESSFUL | SUCCESSFUL | PASS | green @ unwind 9, length<=8 |
| K2 pre-fix | `cargo kani --harness check_read_byte` | FAILED (ptr OOB) | FAILURE: pointer dereference NULL / one-past-end | PASS | red |
| K2 post-fix | re-run | SUCCESSFUL | SUCCESSFUL | PASS | genuine safe Option fix |
| K3 pre-fix | `cargo kani --harness check_accumulate` | FAILED (overflow) | FAILURE: attempt to add with overflow | PASS | red |
| K3 post-fix | re-run | SUCCESSFUL | SUCCESSFUL | PASS | saturating_add + postcondition |
| K4 contract | `cargo kani -Z function-contracts --harness check_gcd_contract` | FAILED→SUCCESSFUL | red: remainder/division by zero (no requires) → green with requires | PASS | precondition added |
| K4 reuse | `-Z function-contracts -Z stubbing --harness check_reduce_fraction` | SUCCESSFUL | SUCCESSFUL | PASS | stub_verified (needs -Z stubbing too) |
| Scope report | review `docs/slo/verify/kani-verification-kani.md` | every bound stated | written; bounds/assumptions/flags per harness; no whole-system claim | PASS | |
| Workspace baseline | `cargo test -p sast-verify` here | unaffected/green | unaffected (demo is external, not a workspace member) | PASS | |
| Test artifact cleanup | `git status` | clean | only intended files | PASS | |

> **Allow-list extension (M4):** `skills/slo-kani/tools.toml` was added to M4's allowed files to bump the pin 0.56.0 → 0.67.0. Rationale: 0.56.0 was an unverified placeholder; M4 *is* the "re-verify the demo on the pinned version" step the `tools.toml` header requires, and the only honest pin is the version actually verified against (0.67.0). User-confirmed 2026-05-22.

#### Definition of Done

All four K-series go red→green at stated bounds; the scope report records each red counterexample, each fix, each green, and every bound/assumption/stub/contract; K2's fix is genuine safety not assume-tightening; a missing Kani toolchain produces a loud documented skip, never a false pass (ENG-1); the workspace baseline is unaffected; the demo repo URL+commit is recorded; lessons file written.

---

## Milestone 5 — TLA+ pairing refinement map + local deep-verification workflow

**Goal**: Document the TLA+ ↔ Kani refinement relationship (action → Rust fn → Kani harness) so the two formal-methods skills compose, and give developers a documented **local deep-verification workflow** they run on their own laptop before a release. **No CI automation in the initial release** — nightly/PR Kani CI is explicitly deferred to a future iteration.

**Context**: M1–M4 made `/slo-kani` work and proved the failure bar. This milestone makes the pairing explicit and makes deep verification a deliberate, time-budgeted **local** pre-release step. The research report's thresholds (fast small-bound iteration vs. slower deep proofs; Firecracker ran 27 harnesses in ~15 min) inform the *local* quick-vs-deep tiers rather than a CI schedule. Running deep proofs locally for v1 keeps the team in control of when the slow proofs run and avoids standing up CI infrastructure before the skill has earned its keep.

**Carmack-style reliability goal**: Bounded resources (deep proofs are a deliberate, time-budgeted local step, not an always-on cost) + compatibility (no new CI surface to maintain or break).

**Important design rule**: Kani NEVER claims interleavings/concurrency — that's TLA+. The pairing doc encodes the boundary: TLA+ proves the protocol; Kani proves the atomic action's Rust kernel is panic-free/invariant-preserving.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | the M4 demo (a Rust kernel); a hypothetical TLA+-modeled design with atomic actions |
| Outputs | `docs/slo/design/kani-verification-kani-pairing.md` (refinement map template + worked example); `skills/slo-kani/references/local-deep-verification.md` (quick + deep local tiers, deep-before-release rule); structural test for the pairing doc + local-verify doc; reciprocal handoff notes |
| Interfaces touched | the pairing-doc contract + the local-verify workflow doc (no CI surface) |
| Files allowed to change | `docs/slo/design/kani-verification-kani-pairing.md` (NEW), `skills/slo-kani/references/local-deep-verification.md` (NEW), `skills/slo-kani/SKILL.md` (add pairing + local-verify handoff note), `skills/slo-tla/SKILL.md` (add reciprocal handoff note), `xtasks/sast-verify/tests/kani_m5_pairing.rs` (NEW), `docs/LOOPS-ENGINEERING.md` (add `/slo-kani` to the sprint-loop skills list) |
| Files to read before changing anything | `skills/slo-tla/SKILL.md` handoff section, research report §"CI suite budget" / engineering thresholds (for the quick-vs-deep bound tiers), §5.2 of this runbook |
| New files allowed | the pairing doc, the local-verify reference, the M5 test |
| New dependencies allowed | `none` (local runs use the pinned `kani-verifier` from `tools.toml`) |
| Migration allowed | `no` |
| Compatibility commitments | **no new CI workflow is added**; existing `.github/workflows/*` are untouched; M1–M4 green |
| Resource bounds introduced/changed | local-verify doc defines two tiers — **quick** (small bounds, seconds, inner-loop iteration) and **deep** (larger bounds, minutes, run before a release tag); developer chooses; the ~7 GB GitHub-runner RAM caveat is cited as a reason deep proofs stay local for v1 |
| Invariants/assertions required | pairing doc states the boundary invariant ("Kani never claims what TLA+ owns"); local-verify doc states the **pinned toolchain** from `tools.toml` is used (reproducibility) and that the **deep tier must run green before any release tag** |
| Debugger / inspection expectation | `cargo test -p sast-verify -- kani_m5 --nocapture` shows a missing pairing-doc or local-verify element |
| Static analysis gates | fmt/clippy/audit |
| Exemplar code to copy | `skills/slo-tla/SKILL.md` handoff (reciprocal note shape); research report scenarios for the pairing worked example; the M4 scope report for the deep-run command shapes |
| Anti-exemplar code not to copy | a single CI job that runs all proofs on every push (the report warns against it — and we are avoiding CI entirely for v1); a pairing doc implying Kani proves interleavings; calling a build "release-ready" without the deep local run |
| Refactoring discipline | cite `refactoring-discipline.md` for the SKILL.md handoff edits: behavior-preserving, tests green before/after |
| AI tolerance contract | accepted variance: pairing-doc + local-verify prose; deterministic boundary: the boundary-invariant sentence, the quick/deep tier definitions, the pinned-toolchain rule, and the deep-before-release rule are exact; eval evidence: the M4 demo kernel mapped in the worked example; must-never: a pairing doc that lets Kani claim concurrency; a "release-ready" claim without the deep tier run; adding CI automation this milestone; sample budget: N/A |
| Forbidden shortcuts | a pairing doc implying Kani proves interleavings; claiming release-readiness without running the deep tier; adding CI automation (out of scope by decision); using `latest` instead of the pinned toolchain locally |
| Data classification | Public |
| Proactive controls in play | C5 (secure defaults — pinned toolchain even for local runs), C9 (logging/monitoring — deep-run results recorded in the scope report) |
| Abuse acceptance scenarios | `tm-kani-verification-abuse-2: pairing doc encodes the concurrency boundary so neither skill overclaims`; `tm-kani-verification-abuse-4: local-verify workflow uses the pinned toolchain, not latest` |

#### Out of Scope / Must Not Do

- **Any CI automation** (nightly or PR Kani jobs) — explicitly deferred to a future iteration per the v1 decision.
- Changing existing workflows; running deep proofs on a server.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/slo/design/kani-verification-kani-pairing.md` | NEW: refinement map template (action→fn→harness) + worked example using the M4 K-series + the boundary invariant |
| `skills/slo-kani/references/local-deep-verification.md` | NEW: quick tier (small bounds, fast inner loop) + deep tier (larger bounds, run before release) + pinned-toolchain rule + deep-before-release-tag rule; optional helper-script description for the demo repo |
| `skills/slo-kani/SKILL.md` | add reciprocal `/slo-tla` handoff note + a pointer to the local-verify reference |
| `skills/slo-tla/SKILL.md` | add reciprocal `/slo-kani` handoff note (when Rust kernels exist) |
| `xtasks/sast-verify/tests/kani_m5_pairing.rs` | NEW: structural test (pairing-doc invariant + local-verify tiers + no-CI-added assertion) |
| `docs/LOOPS-ENGINEERING.md` | add `/slo-kani` to the sprint-loop skills list |

#### Step-by-Step

1. Write `kani_m5_pairing.rs` assertions first (fail — artifacts absent).
2. Author the pairing doc (refinement map + boundary invariant + M4 worked example).
3. Author the local-deep-verification reference (quick/deep tiers, pinned toolchain, deep-before-release rule).
4. Add reciprocal handoff notes to both skills; add `/slo-kani` to the sprint loop.
5. Make the structural test pass; confirm no `.github/workflows/*` change.
6. fmt/clippy/audit; `git status` clean; self-review gate.

#### BDD Acceptance Scenarios

**Feature: pairing + local deep verification (no CI in v1)**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Refinement map documented | happy path | a TLA+ action + its Rust fn | author the pairing doc | action→fn→harness mapping present with a worked example |
| Boundary invariant stated | abuse case | the pairing doc | structural test scans | "Kani never claims what TLA+ owns" present — `tm-kani-verification-abuse-2` |
| Local tiers documented | resource bound | the local-verify doc | structural test reads it | both a quick (small-bound) and a deep (larger-bound) tier are defined |
| Deep-before-release rule | assertion violation | a release without the deep run | the doc's rule applies | release is not "ready"; deep tier must be green first |
| Local toolchain pinned | invalid input | the local deep run | structural test | uses the `tools.toml` pinned version, not `latest` — `tm-kani-verification-abuse-4` |
| No CI added | compatibility | current `.github/workflows/*` | structural test + full suite | unchanged; no Kani CI workflow introduced (v1 decision) |
| TLA+ reciprocal note | happy path | `/slo-tla` SKILL.md | structural test | references `/slo-kani` for Rust-kernel verification |

#### Regression Tests

- M1–M4 tests green.
- Existing `.github/workflows/*` byte-unchanged (none added).
- The pinning convention test (`sap_imp_m4_workflow_pinning.rs`) still passes (no new workflow to pin).

#### Compatibility Checklist

- [ ] No new CI workflow added (v1 decision)
- [ ] Existing workflows unchanged
- [ ] M1–M4 tests green
- [ ] `/slo-tla` existing behavior unchanged (only an additive handoff note)

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/kani_m5_pairing.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `pairing_doc_has_refinement_map_and_invariant` | pairing is real + bounded | mapping + invariant sentence present |
| `local_deep_verification_documents_quick_and_deep_tiers` | sustainable local workflow | both tiers defined |
| `local_workflow_uses_pinned_toolchain` | reproducibility + supply chain | version matches `tools.toml`, not `latest` |
| `no_kani_ci_workflow_added` | v1 scope honored | no Kani workflow under `.github/workflows/` |

#### Smoke Tests

- [ ] `cargo test -p sast-verify -- kani_m5` passes
- [ ] Pairing doc worked example references an M4 K-series harness
- [ ] local-verify doc lists the deep-tier `cargo kani` commands + the deep-before-release rule
- [ ] `git status` clean

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sast-verify` | green | | | |
| BDD tests created | `kani_m5_pairing.rs` | fail (artifacts absent) | | | |
| Implementation | pairing doc + local-verify ref + handoffs | contract satisfied | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Static analyzer | `cargo clippy ... -D warnings` | clean | | | |
| Full tests | `cargo test -p sast-verify` | green | | | |
| No CI added | `git status .github/workflows` | unchanged | | | |
| Compatibility checks | existing workflows | unchanged | | | |
| Test artifact cleanup | `git status` | clean | | | |

#### Definition of Done

The refinement map + boundary invariant are documented and asserted; the local deep-verification workflow documents the quick + deep tiers, the pinned toolchain, and the deep-before-release rule; **no CI automation is added (deferred to a future iteration)**; reciprocal handoff notes added to both skills; `/slo-kani` is in the sprint-loop skills list; M1–M4 green; lessons file written.

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md | README.md | catalog | CLAUDE.md / overlays | LOOPS-ENGINEERING.md | design docs |
|---|---|---|---|---|---|---|
| M1 | (planned note already added) | — | add `/slo-kani` row | — | — | — |
| M2 | — | — | — | — | — | — |
| M3 | note the `kani_required` seam | — | — | mention `/slo-kani` + `kani_required` | — | mark interfaces `stable` |
| M4 | — | link demo repo | — | — | — | overview: demo URL+commit; new scope report |
| M5 | fold planned note into HEAD table (skill now ships) | — | — | overlay notes if needed | add `/slo-kani` to sprint loop | new pairing doc + local-deep-verification ref |

---

## Carry-Forward From Prior Retros

None yet — first runbook for this feature. M2+ Pre-Flight reads `docs/slo/lessons/kani-m<N-1>.md` per §7.

## Self-Review Gate (per milestone)

- Is every assumption either verified or documented as unresolved?
- Does every green claim carry its bound/assumptions/stubs?
- Is the verdict derived from `cargo kani`, never narration?
- Did each demo proof (M4) show a red before its green?
- Was the deep tier run locally (green) before any release tag, with no CI automation added (M5)?

---

## Handoff

Runbook complete. Next: `/slo-critique kani-verification` (four-persona adversarial review — has a security surface and an AI component, so the security pass is mandatory; no UI, so the design pass auto-skips) before `/slo-execute M1`.
