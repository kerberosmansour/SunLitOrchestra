# Agent Operating Contract - SunLitOrchestra (AI-First Runbook v4)

> **Purpose**: Add a small, host-portable operating contract for AI coding agents, wire Claude Code, Codex, and GitHub Copilot overlays to it, and protect the wiring with structural tests.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Keep always-on instructions short, route detailed behavior through SLO skills, and test host-support claims so stale platform assumptions do not quietly become product behavior.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/skill-pack-catalog.md](skill-pack-catalog.md), [docs/slo/design/agent-host-capabilities.md](slo/design/agent-host-capabilities.md), [docs/slo/design/host-capability-matrix.md](slo/design/host-capability-matrix.md), [CLAUDE.md](../CLAUDE.md), [AGENTS.md](../AGENTS.md), [copilot-instructions.md](../copilot-instructions.md).

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `agent-operating-contract` |
| Project name | `SunLitOrchestra` |
| Primary stack | Markdown skills and overlays + Rust structural tests |
| Primary package/app names | `skills/`, `sldo-install`, `docs/`, host overlay files |
| Prefix for tests and lesson files | `agent-operating-contract` |
| Default unit test command | `cargo test -p sldo-install --test e2e_agent_operating_contract` |
| Default integration/BDD test command | `cargo test -p sldo-install --test e2e_agent_operating_contract` |
| Default E2E/runtime validation command | `N/A - documentation and structural tests only in M1` |
| Default build/boot command | `cargo test -p sldo-install --test e2e_agent_operating_contract --no-run` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy -p sldo-install --tests -- -D warnings` |
| Default dependency / security audit command | `N/A - no dependency changes allowed` |
| Default debugger or state-inspection tool | `rg` plus targeted file reads; Rust test failure output for structural state |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |
| Master GitHub issue | [#87](https://github.com/kerberosmansour/SunLitOrchestra/issues/87) |

### Public Interfaces That Must Remain Stable

- Existing skill names under `skills/<name>/SKILL.md`.
- Existing installer host ids: `claude-code`, `github-copilot`, `codex`.
- Existing default `sldo-install` behavior targeting Claude Code.
- Existing global install roots documented today: `~/.claude/skills/`, `~/.copilot/skills/`, `~/.codex/skills/`.
- Existing local install roots documented today: `./.claude/skills/`, `./.copilot/skills/`, `./.codex/skills/`. If Copilot project-skill support changes, add compatibility instead of silently replacing the old root.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Operating contract and overlay wiring | `done` | 2026-05-19 | 2026-05-19 | | M1 shipped shared contract, overlay wiring, Copilot repo-wide instructions, and structural tests. |
| 2 | Host capability refresh and Copilot install-root decision | `done` | 2026-05-19 | 2026-05-19 | `docs/slo/lessons/agent-operating-contract-m2.md` | M2 refreshed host capability truth docs, separated official roots from SLO compatibility roots, and guarded the distinction with structural tests. |
| 3 | Optional host-native agent profile parity | `done` | 2026-05-19 | 2026-05-19 | `docs/slo/lessons/agent-operating-contract-m3.md` | M3 shipped bounded Copilot custom-agent profiles while preserving Codex and portable skill fallbacks. |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/agent-operating-contract-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/agent-operating-contract-m<N>.md -->

---

## 3. End-To-End Architecture Diagram

```text
User in Claude Code / Codex / GitHub Copilot
  |
  | reads always-on host instructions
  v
+-------------------------------------------------------------+
| SunLitOrchestra repo                                         |
|                                                             |
|  CLAUDE.md  ---------+                                      |
|  AGENTS.md  ---------+----> references/agent/operating-     |
|  copilot-instructions.md     contract.md                    |
|  .github/copilot-instructions.md --+                        |
|                                    |                        |
|                                    v                        |
|                       docs/skill-pack-catalog.md            |
|                                    |                        |
|                                    v                        |
|                       skills/<name>/SKILL.md                |
|                                    |                        |
|                                    v                        |
|                       host-specific skill roots             |
|                                                             |
|  Rust structural tests guard the links and stale claims.     |
+-------------------------------------------------------------+

Legend: solid arrows are documentation guidance; tests enforce selected edges.
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `references/agent/operating-contract.md` | Shared tiny behavioral contract for agents | New | M1 | Markdown rules linked from overlays |
| `CLAUDE.md` | Claude Code overlay | Changed | M1 | Project memory document |
| `AGENTS.md` | Codex and cross-agent instruction file | Changed | M1 | Project instruction document |
| `copilot-instructions.md` | Human-readable Copilot overlay kept for existing links | Changed | M1 | Existing root overlay path |
| `.github/copilot-instructions.md` | Copilot repo-wide custom instructions path | New | M1 | GitHub-documented Copilot repository custom instructions |
| `docs/slo/design/*host*capabilit*.md` | Host-support truth docs | Changed later | M2 | Capability matrices and source dates |
| `agents/` / future `.github/agents/` | Host-native specialist agent profiles | Optional later | M3 | Claude agent files, possible Copilot profiles |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Always-on behavior | Host overlay | `references/agent/operating-contract.md` | Markdown link + summary | yes | overlay drift | M1 |
| Skill execution | User prompt | `skills/<name>/SKILL.md` | Host-specific skill loading | yes | hidden Claude/Copilot assumption | M1-M2 |
| Capability claims | Current platform docs | host matrices | Manual update + structural tests | yes | stale claim becomes product docs | M2 |
| Agent role parity | Claude agents / Copilot agents | host-specific agent profiles | Host-native profile files | yes | second-class fallback or false parity | M3 |

---

## 4. TLA+ Section

N/A - this runbook changes Markdown instruction files and structural tests. There is no shared mutable state, distributed ordering guarantee, resource lease, or failure-recovery protocol to model. Correctness is better guarded by structural tests that read the shipped artifacts at HEAD.

---

## 5. Milestone 1 - Operating Contract And Overlay Wiring

### Goal

At the end of M1, every supported host has a short always-on path to the same agent operating contract, and a Rust structural test fails if that wiring drifts.

### Context

The repo already has `CLAUDE.md`, `AGENTS.md`, and `copilot-instructions.md`, but only the Claude and Codex filenames are loaded by their primary hosts as-is. GitHub's current Copilot documentation names `.github/copilot-instructions.md` as the repository-wide custom-instructions path and also recognizes `AGENTS.md` for agent instructions. This milestone adds the missing Copilot path without changing the installer or any skill name.

### Important Design Rule

The operating contract must stay tiny. Put stable cross-host behavior there; keep detailed procedures in SLO skills and host-specific install/runtime notes in the overlays and capability matrices.

### Refactor Budget

No refactor permitted beyond direct implementation.

### Contract Block

| Contract Row | Value |
|---|---|
| Inputs | Current host overlays, GitHub/OpenAI/Anthropic instruction docs cited in research notes, existing agent-host tests |
| Outputs | `references/agent/operating-contract.md`, `.github/copilot-instructions.md`, overlay edits, structural test |
| Interfaces touched | Markdown instruction files only |
| Files allowed to change | `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md`, `docs/slo/critique/agent-operating-contract.md`, `references/agent/operating-contract.md`, `CLAUDE.md`, `AGENTS.md`, `copilot-instructions.md`, `.github/copilot-instructions.md`, `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md`, `crates/sldo-install/tests/e2e_agent_operating_contract.rs` |
| Files to read before changing | `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md`, `CLAUDE.md`, `AGENTS.md`, `copilot-instructions.md`, `crates/sldo-install/tests/e2e_agent_host_m2.rs`, `crates/sldo-install/tests/e2e_agent_host_m3.rs`, `crates/sldo-install/tests/e2e_agent_host_m5.rs` |
| New files allowed | `references/agent/operating-contract.md`, `.github/copilot-instructions.md`, `crates/sldo-install/tests/e2e_agent_operating_contract.rs`, `docs/slo/critique/agent-operating-contract.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Do not remove or rename root `copilot-instructions.md`; do not change skill names; do not change installer host ids or roots in M1 |
| Exemplar code to copy | `crates/sldo-install/tests/e2e_agent_host_m2.rs` for repo-root fixture reads and clear assertion messages |
| Anti-exemplar code not to copy | `N/A - no anti-exemplar identified; existing structural tests are the local pattern` |
| Refactoring discipline | N/A - no refactoring performed |
| AI tolerance contract | N/A - no LLM runtime behavior or eval harness is introduced |
| Data classification | Public |
| Proactive controls in play | OWASP C1 Define Security Requirements - instruction sources and host boundaries must be explicit; C5 Validate Inputs - structural tests validate paths and required phrases |
| Abuse acceptance scenarios | `stale_host_claim_rejected` and `copilot_path_not_silent` BDD rows below |
| Resource bounds introduced/changed | Operating contract should remain <= 120 nonblank lines; always-on Copilot instructions should remain <= 120 nonblank lines |
| Invariants/assertions required | Structural test asserts every overlay mentions `references/agent/operating-contract.md`; Copilot repo-wide instructions exist under `.github/`; contract contains the named core rules |
| Debugger / inspection expectation | N/A - test failure output and direct file reads are sufficient state inspection |
| Static-analysis gates | `cargo fmt --all -- --check`; `cargo test -p sldo-install --test e2e_agent_operating_contract`; `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` |
| Reversibility / rollback path | Remove the new contract/test/Copilot file and revert overlay references; no persisted data or installer behavior changes |
| Forbidden shortcuts | Do not paste a long generic agent template; do not remove host-specific limitations; do not claim Copilot/Codex headless runtime parity; do not replace root Copilot overlay in M1 |

### Out Of Scope / Must Not Do

- Do not change `sldo-install` path resolution in M1.
- Do not port Claude `agents/` into `.github/agents/` in M1.
- Do not add a new host-runtime abstraction.
- Do not weaken existing "Claude-only" labels where the runtime really is Claude-only.

### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/agent/operating-contract.md` | New small host-neutral operating contract |
| `CLAUDE.md` | Add concise pointer to the operating contract |
| `AGENTS.md` | Add concise pointer to the operating contract |
| `copilot-instructions.md` | Add pointer to the operating contract and `.github/` companion |
| `.github/copilot-instructions.md` | New repo-wide Copilot custom instructions file |
| `docs/ARCHITECTURE.md` | Mention the shared operating contract and Copilot repo-wide instructions |
| `docs/skill-pack-catalog.md` | Mention the shared operating contract under shared invariants |
| `crates/sldo-install/tests/e2e_agent_operating_contract.rs` | New structural test |
| `docs/slo/critique/agent-operating-contract.md` | M1 critique artifact |

### Step-By-Step

1. Add failing structural test for the operating-contract and overlay-wiring invariants.
2. Run the new test and confirm it fails because files/references are missing.
3. Add the shared operating contract.
4. Wire `CLAUDE.md`, `AGENTS.md`, root `copilot-instructions.md`, and `.github/copilot-instructions.md`.
5. Update architecture/catalog references.
6. Run the new test and existing agent-host tests.
7. Run formatter.
8. Record evidence in this runbook.

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `overlays_share_one_contract` | happy path | A contributor opens any host overlay | The structural test reads `CLAUDE.md`, `AGENTS.md`, `copilot-instructions.md`, and `.github/copilot-instructions.md` | Each points to `references/agent/operating-contract.md` | `cargo test -p sldo-install --test e2e_agent_operating_contract` |
| `contract_stays_small` | invalid input | A future edit bloats the always-on operating contract | The structural test counts nonblank lines | The test fails above the configured cap | same test |
| `copilot_path_not_silent` | empty / degraded state | GitHub Copilot ignores the root overlay path | The repo contains `.github/copilot-instructions.md` | Copilot has a documented repo-wide custom-instructions file that points at SLO sources | same test |
| `stale_host_claim_rejected` | abuse case | A future doc implies Copilot/Codex runtime parity without capability review | The operating contract and Copilot instructions are read | They route runtime assumptions to the host capability docs instead of making a blanket promise | same test + existing agent-host tests |

### Regression Tests

- `cargo test -p sldo-install --test e2e_agent_host_m2`
- `cargo test -p sldo-install --test e2e_agent_host_m3`
- `cargo test -p sldo-install --test e2e_agent_host_m5`

### Compatibility Checklist

- [x] Root `copilot-instructions.md` remains present.
- [x] Existing overlay tests still pass.
- [x] No installer path or host id changes.
- [x] No skill names or `SKILL.md` contracts change.

### E2E Runtime Validation

N/A - M1 is documentation and structural-test only. Host-session smoke belongs in M2 after capability-matrix refresh.

### Smoke Tests

- Read `.github/copilot-instructions.md` and confirm it is short enough for always-on Copilot context.
- Read `AGENTS.md` and confirm Codex still sees Codex-specific install guidance.
- Read `CLAUDE.md` and confirm Claude-specific catalog details remain intact.

### Evidence Log

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| New test fails first | `cargo test -p sldo-install --test e2e_agent_operating_contract` | fails before implementation | Failed with 4 expected failures: missing `references/agent/operating-contract.md`, missing `.github/copilot-instructions.md`, missing overlay links, missing architecture/catalog invariant | `pass` | Confirms the test was red for the intended contract gaps |
| Formatter | `cargo fmt --all -- --check` | passes | Failed on pre-existing rustfmt drift in unrelated Rust test files; new `e2e_agent_operating_contract.rs` was not listed in the diff | `blocked-unrelated` | Did not run `cargo fmt` because it would churn unrelated files outside the M1 allow-list |
| Scoped clippy | `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings` | passes | Passed | `pass` | New Rust test is clean |
| Full clippy | `cargo clippy -p sldo-install --tests -- -D warnings` | passes | Failed on pre-existing unrelated warnings in `tests/common/claude_runtime.rs` and `tests/e2e_biz_followup_m5.rs` | `blocked-unrelated` | Recorded for follow-up; not fixed in M1 because outside allow-list |
| New structural test | `cargo test -p sldo-install --test e2e_agent_operating_contract` | passes | Passed: 4 tests | `pass` | |
| Regression tests | `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` | passes | Passed: 8 tests across 3 test binaries | `pass` | Existing agent-host contracts preserved |
| Compatibility check | `git status --short` | only intended files changed | Only M1 allow-list files changed | `pass` | No installer path or skill-name changes |

### Definition Of Done

- [x] Shared operating contract exists and stays small.
- [x] All four host-facing instruction files link to it.
- [x] `.github/copilot-instructions.md` exists for GitHub's repo-wide custom-instructions path.
- [x] Structural test fails before implementation and passes after.
- [x] Existing agent-host regression tests pass.

---

## 6. Milestone 2 - Host Capability Refresh And Copilot Install-Root Decision

### Goal

At the end of M2, the living host-capability docs distinguish official host-native instruction/skill/agent paths from SunLit's existing `sldo-install` compatibility roots, and the repo has structural tests that fail if those distinctions disappear.

### Context

M1 added `.github/copilot-instructions.md` and the shared agent operating contract. While doing that work, current official docs showed that Copilot's documented project skill roots include `.github/skills`, `.claude/skills`, and `.agents/skills`; Codex's documented repository skill root is `.agents/skills`; and Claude Code reads `CLAUDE.md`, not `AGENTS.md`, unless `CLAUDE.md` imports it. SunLit still has shipped installer compatibility roots at `./.copilot/skills` and `./.codex/skills`, plus global roots under `~/.copilot/skills` and `~/.codex/skills`. M2 updates the truth docs and onboarding language without changing installer behavior.

### Important Design Rule

Do not silently migrate or remove an existing install root. M2 is a documentation and structural-guard milestone: it records the official host-native paths and the existing SLO compatibility roots, then defers any installer migration to a separate explicit contract.

### Refactor Budget

No refactor permitted beyond direct implementation.

### Contract Block

| Contract Row | Value |
|---|---|
| Inputs | Official host docs retrieved on 2026-05-19; current `sldo-install` host descriptor table; existing M1 artifacts; issue #87 |
| Outputs | Refreshed capability matrices, onboarding docs, installer README notes, structural tests, issue progress comment |
| Interfaces touched | Documentation and structural tests only; no CLI behavior changes |
| Files allowed to change | `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md`, `docs/slo/critique/agent-operating-contract.md`, `docs/slo/design/agent-host-capabilities.md`, `docs/slo/design/host-capability-matrix.md`, `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md`, `README.md`, `docs/getting-started.md`, `skills/README.md`, `crates/sldo-install/README.md`, `crates/sldo-install/tests/e2e_agent_operating_contract.rs`, `docs/slo/verify/agent-operating-contract-m2.md`, `docs/slo/lessons/agent-operating-contract-m2.md`, `docs/slo/completion/agent-operating-contract-m2.md` |
| Files to read before changing | `crates/sldo-install/src/host.rs`, `crates/sldo-install/src/paths.rs`, `README.md`, `docs/getting-started.md`, `skills/README.md`, `crates/sldo-install/README.md`, `docs/slo/design/agent-host-capabilities.md`, `docs/slo/design/host-capability-matrix.md`, `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md` |
| New files allowed | none |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `sldo-install --host github-copilot --local` still documents `./.copilot/skills`; `sldo-install --host codex --local` still documents `./.codex/skills`; no host id, manifest, or symlink behavior changes in M2 |
| Exemplar code to copy | `crates/sldo-install/tests/e2e_agent_operating_contract.rs` for structural assertions over living docs |
| Anti-exemplar code not to copy | Do not copy old matrix rows that say Copilot has "no equivalent" agent support; current docs say Copilot custom agents exist in preview |
| Refactoring discipline | N/A - no refactoring performed |
| AI tolerance contract | N/A - no LLM runtime behavior or eval harness is introduced |
| Data classification | Public |
| Proactive controls in play | OWASP C1 Define Security Requirements - official host paths and SLO compatibility roots must be named explicitly; C5 Validate Inputs - tests guard path claims and "no runtime harness" wording |
| Abuse acceptance scenarios | `official_roots_not_erased`, `compatibility_roots_not_removed`, and `cloud_agent_not_confused_with_slo_runtime_harness` BDD rows below |
| Resource bounds introduced/changed | No runtime resources; docs should avoid duplicating a full catalog and link to the canonical catalog instead |
| Invariants/assertions required | Structural test asserts docs mention `.github/skills`, `.agents/skills`, `.copilot/skills`, `.codex/skills`, `.github/agents`, and "no Copilot or Codex runtime harness is shipped today" |
| Debugger / inspection expectation | N/A - test failure output and direct file reads are sufficient |
| Static-analysis gates | `cargo test -p sldo-install --test e2e_agent_operating_contract`; `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`; `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings`; `cargo fmt --all -- --check` recorded honestly if still blocked by unrelated drift |
| Reversibility / rollback path | Revert M2 doc/test edits; installer behavior is untouched |
| Forbidden shortcuts | Do not change `crates/sldo-install/src/host.rs` or `paths.rs`; do not claim Copilot/Codex have a shipped SLO headless runtime harness; do not remove `.copilot/skills` or `.codex/skills` from existing install docs; do not add `.github/agents` profiles in M2 |

### Out Of Scope / Must Not Do

- Do not implement installer migration to `.github/skills` or `.agents/skills`.
- Do not generate Copilot custom-agent profiles; that is M3.
- Do not alter `.claude-plugin/plugin.json` or release packaging.
- Do not close issue #87 or mark M3 planned before M2 evidence is recorded.

### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-install/tests/e2e_agent_operating_contract.rs` | Add M2 structural assertions |
| `docs/slo/design/agent-host-capabilities.md` | Refresh capability matrix and per-skill/host notes |
| `docs/slo/design/host-capability-matrix.md` | Update source dates, official path rows, and Copilot agent preview status |
| `README.md` | Clarify SLO installer compatibility roots versus official host-native roots |
| `docs/getting-started.md` | Same clarification for first-run users |
| `skills/README.md` | Same clarification for skill-pack readers |
| `crates/sldo-install/README.md` | Same clarification for installer users |
| `docs/ARCHITECTURE.md` | Record the compatibility-root versus official-root distinction |
| `docs/skill-pack-catalog.md` | Record the invariant without duplicating the matrices |
| `docs/slo/critique/agent-operating-contract.md` | Add M2 critique rows if needed |
| `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md` | Track evidence and milestone status |
| `docs/slo/verify/agent-operating-contract-m2.md` | M2 verification report |
| `docs/slo/lessons/agent-operating-contract-m2.md` | M2 lessons learned |
| `docs/slo/completion/agent-operating-contract-m2.md` | M2 completion summary |

### Step-By-Step

1. Update issue #87 with the working branch and M2 start.
2. Add failing structural assertions for official host-native roots and SLO compatibility roots.
3. Run the targeted test and confirm it fails for missing M2 wording.
4. Update the capability docs and onboarding docs using official source dates.
5. Run the targeted structural test and agent-host regression tests.
6. Run scoped clippy for the changed test.
7. Run formatter and record any unrelated blocker.
8. Fill the M2 Evidence Log and update issue #87.

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `official_roots_not_erased` | happy path | A reader checks current host-native skill roots | The structural test reads capability docs | Docs name Copilot `.github/skills` and `.agents/skills`, and Codex `.agents/skills` | `cargo test -p sldo-install --test e2e_agent_operating_contract` |
| `compatibility_roots_not_removed` | backward compat | A current SLO user follows existing installer docs | The structural test reads onboarding docs | Docs still name `.copilot/skills` and `.codex/skills` as SLO installer compatibility roots | same test |
| `unknown_migration_not_silent` | invalid input | A future edit removes compatibility-root language while leaving installer code unchanged | The structural test runs | The test fails before docs can imply a migration happened | same test |
| `cloud_agent_not_confused_with_slo_runtime_harness` | abuse case | A future doc sees Copilot cloud/custom-agent support and calls it SLO runtime automation | The structural test reads capability docs | Docs still say no Copilot or Codex SLO runtime harness is shipped today | same test + existing agent-host tests |

### Regression Tests

- `cargo test -p sldo-install --test e2e_agent_operating_contract`
- `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`

### Compatibility Checklist

- [x] `sldo-install` behavior and host descriptors unchanged.
- [x] Existing `.copilot/skills` docs retained as compatibility roots.
- [x] Existing `.codex/skills` docs retained as compatibility roots.
- [x] Official `.github/skills` / `.agents/skills` paths recorded in capability docs.
- [x] Copilot custom-agent support recorded as preview / future-M3 input, not shipped SLO parity.

### E2E Runtime Validation

N/A - M2 is documentation and structural-test only. No live host session or installer mutation is required.

### Smoke Tests

- Read `docs/slo/design/agent-host-capabilities.md` and confirm a new user can distinguish "official host-native root" from "SLO installer compatibility root".
- Read `README.md` and confirm existing `sldo-install --host github-copilot --local` users are not told their install path vanished.
- Read `docs/slo/design/host-capability-matrix.md` and confirm M3 has enough information to decide on `.github/agents/*.agent.md`.

### Evidence Log

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Issue workpad update | Comment on issue #87 | Branch and M2 start recorded | Commented on issue #87 with branch `slo/agent-operating-contract` and M2 start. | `pass` | Existing master issue remains open for M3. |
| New M2 test fails first | `cargo test -p sldo-install --test e2e_agent_operating_contract` | fails before docs update | Failed with 2 expected M2 failures: capability docs did not yet name `.github/skills`; onboarding docs did not yet use `compatibility root` wording. | `pass` | Confirms the new assertions were red for the intended gap. |
| New structural test | `cargo test -p sldo-install --test e2e_agent_operating_contract` | passes | Passed: 6 tests. | `pass` | Covers M1 wiring plus M2 root/capability distinctions. |
| Regression tests | `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` | passes | Passed: 8 tests across 3 binaries after preserving the existing `Headless runtime automation` anchor. | `pass` | The first run caught an over-eager row rename; fixed in docs before recording green. |
| Scoped clippy | `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings` | passes | Passed. | `pass` | New structural test is lint-clean. |
| Formatter | `cargo fmt --all -- --check` | passes or same unrelated blocker recorded | Failed on pre-existing rustfmt drift in unrelated Rust test files. | `blocked-unrelated` | Did not run global `cargo fmt` because it would churn files outside M2's allow-list. |
| Verification report | Write `docs/slo/verify/agent-operating-contract-m2.md` | Runtime/static verification recorded | Written with docs-only runtime scope, static test evidence, and DAST marked N/A. | `pass` | No live Copilot/Codex harness exists for this milestone. |
| Compatibility check | `git diff --stat && git status --short` | only intended files changed | Intended M1/M2 docs, one Rust test, and SLO closeout artifacts changed; no `crates/sldo-install/src/host.rs` or `paths.rs` changes. | `pass` | Installer behavior unchanged. |

### Definition Of Done

- [x] M2 official host docs refreshed with 2026-05-19 source date.
- [x] Official host-native roots and SLO compatibility roots are both documented.
- [x] No installer behavior changed.
- [x] M2 structural test fails before docs update and passes after.
- [x] Issue #87 has an M2 progress/evidence comment.

## 7. Milestone 3 - Optional Host-Native Agent Profile Parity

### Goal

At the end of M3, GitHub Copilot has host-native custom-agent profiles for the four existing SLO review/verification roles, and the docs/tests make clear that these profiles are a Copilot preview convenience, not a shipped Copilot or Codex SLO runtime harness. Codex keeps using the canonical portable SLO skill path.

### Context

M2 established the current host story: Copilot supports repository custom-agent profiles under `.github/agents/*.agent.md` in public preview, while Codex has no shipped SLO host-native agent-profile equivalent. Current GitHub docs retrieved on 2026-05-19 say a Copilot custom agent profile is a Markdown file with YAML frontmatter; `description` is required; `tools` is optional and can restrict access; and `target: github-copilot` can scope the profile to Copilot. The existing SLO Claude-oriented agent files under `agents/` already define four bounded roles with explicit `copilot-fallback` fields. M3 ports those roles into GitHub's native profile shape while preserving the portable fallback.

### Important Design Rule

Custom-agent files may improve Copilot UX, but they do not create automatic SLO multi-agent orchestration on Codex or Copilot. The portable contract remains `/slo-critique` and `/slo-verify`; `.github/agents/*.agent.md` profiles must say so in their own prompt bodies.

### Refactor Budget

No refactor permitted beyond direct profile/docs/test implementation.

### Contract Block

| Contract Row | Value |
|---|---|
| Inputs | M2 lessons file; existing `agents/*.md`; GitHub Copilot custom-agent docs retrieved 2026-05-19; capability matrices; issue #87 |
| Outputs | Four `.github/agents/*.agent.md` profiles, structural M3 tests, capability-doc updates, issue progress comment, M3 verification/retro artifacts |
| Interfaces touched | GitHub Copilot repository custom-agent profile files; Markdown docs; structural tests |
| Files allowed to change | `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md`, `docs/slo/critique/agent-operating-contract.md`, `crates/sldo-install/tests/e2e_agent_operating_contract.rs`, `.github/agents/slo-runbook-review-lead.agent.md`, `.github/agents/slo-security-reviewer.agent.md`, `.github/agents/slo-design-reviewer.agent.md`, `.github/agents/slo-verification-lead.agent.md`, `docs/slo/design/agent-host-capabilities.md`, `docs/slo/design/host-capability-matrix.md`, `.github/copilot-instructions.md`, `copilot-instructions.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md`, `docs/slo/verify/agent-operating-contract-m3.md`, `docs/slo/lessons/agent-operating-contract-m3.md`, `docs/slo/completion/agent-operating-contract-m3.md` |
| Files to read before changing | `docs/slo/lessons/agent-operating-contract-m2.md`, `agents/slo-runbook-review-lead.md`, `agents/slo-security-reviewer.md`, `agents/slo-design-reviewer.md`, `agents/slo-verification-lead.md`, `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`, `crates/sldo-install/tests/e2e_agent_operating_contract.rs`, `docs/slo/design/agent-host-capabilities.md`, `docs/slo/design/host-capability-matrix.md`, `.github/copilot-instructions.md`, `copilot-instructions.md`, `docs/ARCHITECTURE.md`, `docs/skill-pack-catalog.md` |
| New files allowed | `.github/agents/slo-runbook-review-lead.agent.md`, `.github/agents/slo-security-reviewer.agent.md`, `.github/agents/slo-design-reviewer.agent.md`, `.github/agents/slo-verification-lead.agent.md`, `docs/slo/verify/agent-operating-contract-m3.md`, `docs/slo/lessons/agent-operating-contract-m3.md`, `docs/slo/completion/agent-operating-contract-m3.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing `agents/*.md` files remain Claude-oriented and keep their `copilot-fallback` fields; no installer roots or host ids change; Codex guidance remains the portable SLO skill path |
| Exemplar code to copy | Existing `agents/*.md` role boundaries and `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` path-boundary assertions |
| Anti-exemplar code not to copy | Do not copy Claude-specific `host-required: claude-code` frontmatter into Copilot profiles; GitHub custom-agent frontmatter uses GitHub's `description`, `tools`, and optional `target` fields |
| Refactoring discipline | N/A - no production refactoring |
| AI tolerance contract | N/A - profiles route human-invoked Copilot behavior; no deterministic LLM eval harness is introduced |
| Data classification | Public |
| Proactive controls in play | OWASP C1 Define Security Requirements - profile scopes and fallback paths are explicit; C5 Validate Inputs - tests validate profile path, tool, line-cap, and no-runtime-parity wording |
| Abuse acceptance scenarios | `copilot_profiles_are_bounded`, `portable_fallback_not_erased`, `codex_not_promised_agent_parity`, and `preview_support_not_runtime_harness` BDD rows below |
| Resource bounds introduced/changed | Each Copilot profile must stay <= 180 nonblank lines; profile prompt bodies must avoid embedding full SLO skill prose |
| Invariants/assertions required | Structural test asserts all four `.github/agents/*.agent.md` files exist, include required frontmatter, restrict tools, mention the canonical portable fallback, avoid path traversal, and preserve the no-Copilot/no-Codex runtime-harness boundary |
| Debugger / inspection expectation | N/A - test failure output and direct file reads are sufficient |
| Static-analysis gates | `cargo test -p sldo-install --test e2e_agent_operating_contract`; `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`; `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings`; `cargo fmt --all -- --check` recorded honestly if still blocked by unrelated drift |
| Reversibility / rollback path | Delete `.github/agents/*.agent.md`, revert M3 docs/test edits, and leave existing Claude agents plus portable SLO skills unchanged |
| Forbidden shortcuts | Do not edit `skills/slo-critique/SKILL.md` or `skills/slo-verify/SKILL.md`; do not add a Copilot/Codex headless harness; do not change `sldo-install` roots; do not claim Codex custom-agent parity |

### Out Of Scope / Must Not Do

- Do not change existing Claude `agents/*.md` files.
- Do not add installer support for `.github/agents/`.
- Do not attempt to test GitHub Copilot cloud execution live.
- Do not add MCP servers or secrets to custom-agent frontmatter.
- Do not change SLO skill prose for `/slo-critique` or `/slo-verify`.

### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-install/tests/e2e_agent_operating_contract.rs` | Add M3 structural assertions |
| `.github/agents/slo-runbook-review-lead.agent.md` | New Copilot custom-agent profile for consolidated runbook critique |
| `.github/agents/slo-security-reviewer.agent.md` | New Copilot custom-agent profile for security review |
| `.github/agents/slo-design-reviewer.agent.md` | New Copilot custom-agent profile for UI/design review |
| `.github/agents/slo-verification-lead.agent.md` | New Copilot custom-agent profile for verification review/runtime QA |
| `docs/slo/design/agent-host-capabilities.md` | Record shipped Copilot custom-agent profiles while preserving runtime-harness boundary |
| `docs/slo/design/host-capability-matrix.md` | Update M3 decision/status |
| `.github/copilot-instructions.md` | Mention optional Copilot custom-agent profiles without making them always-on requirements |
| `copilot-instructions.md` | Same companion note for humans |
| `README.md` | User-requested acknowledgement of Karpathy's four-rule CLAUDE.md framing |
| `docs/ARCHITECTURE.md` | Record Copilot profile layer |
| `docs/skill-pack-catalog.md` | Record portable fallback invariant for profiles |
| `docs/slo/critique/agent-operating-contract.md` | Add M3 critique rows/resolution notes |
| `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md` | Track M3 evidence and status |
| `docs/slo/verify/agent-operating-contract-m3.md` | M3 verification report |
| `docs/slo/lessons/agent-operating-contract-m3.md` | M3 lessons learned |
| `docs/slo/completion/agent-operating-contract-m3.md` | M3 completion summary |

### Step-By-Step

1. Read M2 lessons and prior-retro issue carry-forward.
2. Run repo hygiene gate and baseline structural test.
3. Add failing M3 structural tests for the four Copilot profiles and fallback/runtime-boundary wording.
4. Run the targeted test and confirm it fails because `.github/agents/*.agent.md` profiles are absent.
5. Add the four bounded Copilot custom-agent profiles.
6. Update capability docs, Copilot overlays, architecture, catalog, and critique notes.
7. Run the targeted structural test and agent-host regression tests.
8. Run scoped clippy and formatter; record unrelated formatter blocker if unchanged.
9. Fill the M3 Evidence Log, write verification/lessons/completion, update issue #87.

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `copilot_profiles_are_bounded` | happy path | The repo has Copilot custom-agent files | The structural test reads `.github/agents/*.agent.md` | All four expected profiles exist, have GitHub frontmatter, bounded tools, line caps, and no path traversal | `cargo test -p sldo-install --test e2e_agent_operating_contract` |
| `portable_fallback_not_erased` | backward compat | A non-Copilot user reads host docs or profiles | The structural test reads profiles and capability docs | Profiles route users to `/slo-critique` and `/slo-verify` as canonical portable paths | same test |
| `codex_not_promised_agent_parity` | invalid input | A future doc claims Codex has these host-native profiles | The structural test reads capability docs | Docs continue to say Codex has no shipped SLO host-native custom-agent equivalent | same test |
| `preview_support_not_runtime_harness` | abuse case | A future edit conflates Copilot custom agents with SLO runtime automation | The structural test reads docs and profiles | Docs/profiles keep the no-Copilot/no-Codex runtime-harness boundary | same test + existing agent-host tests |

### Regression Tests

- `cargo test -p sldo-install --test e2e_agent_operating_contract`
- `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`

### Compatibility Checklist

- [x] Existing Claude `agents/*.md` files unchanged.
- [x] Existing `copilot-fallback` fields in Claude agents remain intact.
- [x] No installer host id, root, manifest, or symlink behavior changed.
- [x] Codex remains documented as using the canonical portable SLO skills path.
- [x] Copilot custom-agent profiles are marked preview/convenience, not runtime harness parity.

### E2E Runtime Validation

No live Copilot cloud-agent execution is required in M3. Verification is structural because the repo cannot deterministically run GitHub Copilot custom agents from local CI.

### Smoke Tests

- Read `.github/agents/slo-runbook-review-lead.agent.md` and confirm it can only write the consolidated critique artifact.
- Read `.github/agents/slo-security-reviewer.agent.md` and `.github/agents/slo-design-reviewer.agent.md` and confirm they are read/search-only reviewers.
- Read `.github/agents/slo-verification-lead.agent.md` and confirm it points to `/slo-verify` and bounded report paths.
- Read `docs/slo/design/agent-host-capabilities.md` and confirm Codex is not promised custom-agent parity.

### Evidence Log

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Previous lessons reviewed | Read `docs/slo/lessons/agent-operating-contract-m2.md` | M2 rules applied | Read; M3 keeps runtime-harness boundary and separates profiles from installer roots. | `pass` | |
| Prior-retro carry-forward | `gh issue list --label retro-derived --search "agent-operating-contract" --state open --json number,title,body,url` | prior retro issues surfaced or empty | Empty result. | `pass` | No carry-forward candidates. |
| Repo hygiene | `git status --short --branch`; `git rev-parse --abbrev-ref HEAD`; `git symbolic-ref --short refs/remotes/origin/HEAD` | work continues on task branch | Branch before/after: `slo/agent-operating-contract`; default branch `origin/main`; dirty tree contains in-progress M1/M2/M3 files. | `pass` | No branch switch needed. |
| Baseline structural test | `cargo test -p sldo-install --test e2e_agent_operating_contract` | passes before M3 edits | Passed: 6 tests. | `pass` | Baseline from completed M2. |
| New M3 test fails first | `cargo test -p sldo-install --test e2e_agent_operating_contract` | fails before profiles exist | Failed with 3 expected M3 failures: missing `.github/agents/slo-runbook-review-lead.agent.md` and capability docs missing shipped Copilot profile names. | `pass` | Confirms the new assertions were red for intended profile/doc gaps. |
| New M3 structural test | `cargo test -p sldo-install --test e2e_agent_operating_contract` | passes | Passed: 9 tests. | `pass` | Covers M1, M2, and M3 invariants. |
| Regression tests | `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` | passes | Passed: 8 tests across 3 binaries. | `pass` | Existing host-boundary tests preserved. |
| Claude agent invariant regression | `cargo test -p sast-verify --test sap_imp_m5_agents` | passes | Passed: 7 tests; package compile emitted existing non-fatal warnings. | `pass` | Existing `agents/*.md` files remain structurally valid. |
| Scoped clippy | `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings` | passes | Passed. | `pass` | Changed Rust test target is lint-clean. |
| Formatter | `cargo fmt --all -- --check` | passes or same unrelated blocker recorded | Failed on pre-existing rustfmt drift in unrelated Rust test files. | `blocked-unrelated` | Same blocker as M1/M2; global formatting would churn files outside M3's allow-list. |
| Verification report | Write `docs/slo/verify/agent-operating-contract-m3.md` | Runtime/static verification recorded | Written with structural profile evidence and live-Copilot execution marked out of scope. | `pass` | |
| Compatibility check | `git diff --stat && git status --short` | only intended files changed | Intended M1-M3 docs/profiles/tests plus user-requested README acknowledgement changed; `git diff -- agents` is empty; no installer source changes. | `pass` | Existing Claude agents unchanged. |

### Definition Of Done

- [x] Four Copilot custom-agent profiles exist under `.github/agents/*.agent.md`.
- [x] M3 structural test fails before profiles and passes after.
- [x] Capability docs record the profiles without claiming Codex parity or runtime-harness support.
- [x] Existing Claude agent files and portable fallback paths remain intact.
- [x] Issue #87 has an M3 progress/evidence comment.
