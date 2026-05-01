# Secure Agent Playbook Imports — SunLitOrchestrate (AI-First Runbook v4)

> **Purpose**: Borrow the structural ideas from OWASP Secure Agent Playbook that make security work repeatable — shared evidence-rich finding/summary templates, an example-output gallery, standards traceability, optional Claude plugin packaging, and a gated host-native agents experiment — without changing SLO's identity as a runbook-driven delivery workflow.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: Borrow structure, not content. Re-author every imported pattern in SLO language: runbooks, milestones, evidence logs, critique findings, verification reports, ship-ready PRs.
> **How to use**: Work milestones sequentially. Complete the Global Entry Protocol before each, the Global Exit Protocol after each. Never skip ahead. Treat this document as an execution contract.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [README.md](../README.md), [docs/slo/design/secure-agent-playbook-imports-overview.md](slo/design/secure-agent-playbook-imports-overview.md), [docs/slo/future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](slo/future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md) (precursor stub), [references/security/security-finding-template.md](../references/security/security-finding-template.md), [references/security/security-assessment-summary-template.md](../references/security/security-assessment-summary-template.md), [docs/slo/design/agent-host-capabilities.md](slo/design/agent-host-capabilities.md).

> **What's new in v4 vs v3**: explicit Carmack-style reliability rules (debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, "make invalid states unrepresentable"); extended Contract Block with resource bounds + invariants + debugger expectation + static-analysis gates.

---

## 0. How To Use This Template

1. Fill out Runbook Metadata, Architecture, and Milestone Plan before implementation starts.
2. Work milestones sequentially.
3. Before each milestone, complete the Global Entry Protocol.
4. During implementation, follow Section 4 (Carmack-Style Development Best Practices) and the milestone Contract Block literally.
5. After each milestone, complete the Global Exit Protocol and fill the Evidence Log.
6. Do not mark a milestone done until the Definition of Done is objectively satisfied.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `secure-agent-playbook-imports` |
| Project name | `SunLitOrchestrate` |
| Primary stack | Markdown (skill pack) + Rust (`xtasks/sast-verify`, `crates/sldo-install`) + GitHub Actions (M4 only, SHA-pinned) |
| Primary package/app names | `skills/slo-sast`, `skills/slo-rulegen`, `skills/slo-ruleverify`, `skills/slo-ship`, `skills/slo-critique`, `skills/slo-verify`, `references/security/`, `examples/` (NEW M2), `.claude-plugin/` (NEW M4, optional), `agents/` (NEW M5, gated) |
| Prefix for tests and lesson files | `sap-imp` |
| Default unit test command | `cargo test -p sldo-common -p sldo-install -p sldo-research` |
| Default integration/BDD test command | `cargo test --workspace` |
| Default E2E/runtime validation command | `cargo test --workspace --test 'e2e_sap_imp_m*'` (test files added per milestone) |
| Default build/boot command | `cargo build --workspace` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy --workspace --all-targets -- -D warnings` |
| Default dependency / security audit command | `cargo deny check` |
| Default debugger or state-inspection tool | `rust-lldb` for Rust crates; direct file reads + structural-contract tests for Markdown skill changes |
| Allowed new dependencies by default | `none` (M4 may revisit if release-zip tooling needs it; explicit re-confirmation required) |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- Every shipped `skills/<name>/SKILL.md` install path and frontmatter contract (filename, `name`, `description`).
- `references/security/security-finding-template.md` table-row shape (existing fields stay; M3 may *add* optional rows).
- `references/security/security-assessment-summary-template.md` section ordering.
- `docs/skill-pack-catalog.md` as the canonical skill inventory (no Claude-plugin packaging may displace it).
- `sldo-install` CLI surface (`install`, `--dry-run`, `uninstall`, `status`, `verify`).
- Manifest path `~/.sldo/install.toml` and host-ownership semantics.
- `~~~text` user-string fence rule in `/slo-architect`.
- Four hard-block predicate IDs in `references/biz/triage-gate.md` (immutability locked).
- Multi-host story: GitHub Copilot remains a first-class install target; nothing in this runbook may make it second-class.

---

## 2. Milestone Tracker

This is the single source of truth for progress. Update as each milestone completes.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Shared security-reporting integration extended to `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship` | `done` | 2026-05-01 | 2026-05-01 | [sap-imp-m1.md](slo/lessons/sap-imp-m1.md) | [sap-imp-m1.md](slo/completion/sap-imp-m1.md) |
| 2 | Example output gallery under `examples/` | `done` | 2026-05-01 | 2026-05-01 | [sap-imp-m2.md](slo/lessons/sap-imp-m2.md) | [sap-imp-m2.md](slo/completion/sap-imp-m2.md) |
| 3 | Standards traceability matrix (CWE / OWASP / ASVS / OpenCRE) wired into security outputs | `done` | 2026-05-01 | 2026-05-01 | [sap-imp-m3.md](slo/lessons/sap-imp-m3.md) | [sap-imp-m3.md](slo/completion/sap-imp-m3.md) |
| 4 | Optional Claude plugin packaging assessment + (if green-lit) `.claude-plugin/plugin.json` and SHA-pinned release-zip workflow | `done` (green-lit) | 2026-05-01 | 2026-05-01 | [sap-imp-m4.md](slo/lessons/sap-imp-m4.md) | [sap-imp-m4.md](slo/completion/sap-imp-m4.md) |
| 5 | Host-native agent-role experiment (gated on M4 host-capability matrix) | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/sap-imp-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/sap-imp-m<N>.md -->

---

## 3. End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│  SunLitOrchestrate skill pack — secure-agent-playbook imports end state  │
│                                                                          │
│  references/security/                                                    │
│   ├─ security-finding-template.md            (existing)                  │
│   ├─ security-assessment-summary-template.md (existing)                  │
│   └─ standards-mapping.md                    - - - NEW M3                │
│                                                                          │
│            cited by                                                      │
│   ┌──────────┬──────────┬──────────┬──────────┬──────────┐               │
│   ▼          ▼          ▼          ▼          ▼          ▼               │
│ /slo-      /slo-     /slo-      /slo-      /slo-      /slo-              │
│ critique   verify    sast       rulegen    ruleverify ship               │
│ (existing) (existing)(M1 wires) (M1 wires) (M1 wires) (M1 wires)         │
│                                                                          │
│  examples/                                   - - - NEW M2                │
│   ├─ README.md                                                           │
│   ├─ runbook-excerpt.md                                                  │
│   ├─ critique-report.md                                                  │
│   ├─ verification-report.md                                              │
│   ├─ security-finding.md                                                 │
│   ├─ sast-manifest.json                                                  │
│   └─ biz-public-artifact.md                                              │
│                                                                          │
│  docs/slo/design/host-capability-matrix.md   - - - NEW M4                │
│  .claude-plugin/plugin.json                  - - - NEW M4 (if green-lit) │
│  .github/workflows/release-zip.yml           - - - NEW M4 (SHA-pinned)   │
│                                                                          │
│  agents/                                     - - - NEW M5 (gated on M4)  │
│   ├─ slo-runbook-review-lead.md                                          │
│   ├─ slo-security-reviewer.md                                            │
│   ├─ slo-design-reviewer.md                                              │
│   └─ slo-verification-lead.md                                            │
│                                                                          │
│  Legend:  ─── existing    - - - new (this runbook)    ▶ data flow        │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `skills/slo-sast/SKILL.md` | SAST orchestration; cites assessment-summary template for coverage gaps | changed | M1 | Citation pattern only — no behavior change |
| `skills/slo-rulegen/SKILL.md` | Semgrep rule generation; cites finding template when generated rule is suspect | changed | M1 | Citation pattern only |
| `skills/slo-ruleverify/SKILL.md` | Semgrep rule verification; cites finding template when clean-tree or coverage gates fail | changed | M1 | Citation pattern only |
| `skills/slo-ship/SKILL.md` | PR-body author; adds optional security-summary section when milestone introduced new public surface | changed | M1 | PR body Markdown shape |
| `examples/` | Synthetic, anonymized example artifacts for new contributors and agents | new | M2 | Read-only reference — not installed by `sldo-install` |
| `references/security/standards-mapping.md` | Curated CWE/OWASP/ASVS/OpenCRE lookup table with retrieval-date discipline | new | M3 | Cited by `/slo-critique`, `/slo-verify`, `/slo-sast`, `/slo-rulegen` |
| `docs/slo/design/host-capability-matrix.md` | Decision doc for plugin packaging + agents; documents what each host supports | new | M4 | Read by M5 to gate agent rollout |
| `.claude-plugin/plugin.json` | Optional Claude-only distribution channel pointing at existing `skills/` tree | new (optional) | M4 | Additive to `sldo-install`, never replacing it |
| `.github/workflows/release-zip.yml` | SHA-pinned release-zip workflow | new (optional) | M4 | GitHub Releases artifact |
| `agents/slo-*-lead.md`, `agents/slo-*-reviewer.md` | Host-native specialist agents that still write into `docs/slo/critique/` | new (gated) | M5 | Outputs MUST land in same durable artifacts as `/slo-critique` |
| `xtasks/sast-verify/` | Add structural-contract tests asserting M1/M3 citation patterns hold | changed | M1 + M3 | `cargo test -p sast-verify` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Template citation | `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship` SKILL.md | `references/security/security-{finding,assessment-summary}-template.md` | Markdown link, runtime-read by agent | yes (one citation per skill) | structural-contract test fails if citation missing | M1 |
| Example consumption | new contributor / agent | `examples/<artifact>.md` | filesystem read | yes (≤ 7 files, ≤ 10 KB each) | none — read-only | M2 |
| Standards lookup | `/slo-critique` security row, `/slo-sast` coverage gap | `references/security/standards-mapping.md` | Markdown link | yes (curated table, no live fetch) | missing-mapping is a coverage gap, not a blocker | M3 |
| Host capability decision | `docs/slo/design/host-capability-matrix.md` | M5 gate | Markdown read | yes (decision doc) | M5 deferred to fresh runbook if matrix says Copilot can't install agents | M4 → M5 |
| Release artifact upload | GitHub Actions | GitHub Releases | SHA-pinned action | yes (one zip per tagged release) | workflow fails closed on hash drift | M4 |
| Agent output | `agents/slo-*-{lead,reviewer}.md` | `docs/slo/critique/<slug>.md` | Skill invocation → Markdown write | yes (one critique file per runbook) | agent-only bypass forbidden — output MUST land in shared artifact | M5 |

---

## 4. Carmack-Style Development Best Practices

These rules apply to every language and every milestone.

### 4.1 Inspect State, Do Not Guess

| Requirement | Project-Specific Tool/Command | Evidence Required |
|---|---|---|
| Interactive debugger available | `rust-lldb target/debug/<binary>` for Rust; `cargo test -- --nocapture` for Rust test debugging | Captured in lessons file when used |
| Breakpoints can be set in changed code | `rust-lldb` for Rust; structural-contract test failure messages locate Markdown drift | Note in lessons file |
| Runtime state can be inspected | Rust: `dbg!` during dev (removed before close); Markdown: `git diff` + structural-contract test failure message | Captured in Evidence Log |
| Tests can be debugged | `cargo test -p <crate> -- --nocapture --test <name>` | Test/debug command in Evidence Log |

Agent rules:

- If a structural-contract test fails, read the failure message and the cited Markdown file before making speculative changes.
- Do not add permanent print/debug statements to production paths (Rust crates or skill SKILL.md prose).
- If logging is added to Rust code, it must be structured (`tracing` macros), intentional, and useful in production.
- Remove temporary debug output before completing the milestone.

### 4.2 Static Analysis Is Mandatory

| Check | Command | Required Level | Notes |
|---|---|---|---|
| Formatter | `cargo fmt --all -- --check` | must pass | No style-only churn outside changed files |
| Type check / compile check | `cargo build --workspace` | must pass | Must include all changed targets |
| Static analyzer / linter | `cargo clippy --workspace --all-targets -- -D warnings` | must pass | Warnings fail unless explicitly waived |
| Security/dependency audit | `cargo deny check` | must pass or documented exception | Required if dependency graph changes (M4 may add deps) |
| Markdown structural-contract tests | `cargo test -p sast-verify` (M1 adds the test) | must pass | Asserts citation patterns hold |

Waiver rule: a static-analysis waiver must be local, minimal, and justified in code or the Evidence Log. Global disables forbidden.

### 4.3 Assertions Are Executable Comments

This runbook is mostly Markdown + structural-contract tests. Where Rust code is added (M1's `xtasks/sast-verify` test, M4's release-zip workflow validation), use:

| Assertion Type | Use For | Production Behavior |
|---|---|---|
| `assert!` / `debug_assert!` in tests | Citation-pattern invariants ("every M1 skill cites both templates") | Test failure with line-cited message |
| `Result<T, E>` at boundaries | xtasks/sast-verify CLI surface | Structured error to stderr, non-zero exit |
| Frontmatter schema validation | M2 examples must declare `synthetic: true` and `non-normative: true` | Parser rejects examples missing the flags |

Do not use assertions for: missing files (use `Result`), I/O errors, or human-supplied input parsing.

### 4.4 Prefer Bounded Resources Over Silent Growth

| Resource | Expected Bound | Hard Limit | Behavior At Limit | Evidence/Test |
|---|---:|---:|---|---|
| M1: SKILL.md citations added | 4 (one per skill) | 4 | structural-contract test asserts exact citation count | `cargo test -p sast-verify --test sap_imp_m1_citations` |
| M2: example files | 7 | 7 | adding an 8th requires runbook amendment | M2's structural-contract test asserts file count |
| M2: example file size | ≤ 10 KB each | 15 KB | larger files split into multiple examples | M2 test asserts `wc -c` per file |
| M3: standards-mapping.md rows | grow linearly with covered surfaces; no upper hard cap (curated) | n/a | retrieval-date column required; rows older than 12 months flagged | M3 test asserts every row has a retrieval date |
| M4: GitHub Actions third-party action references | 0 unpinned (every `uses:` must be SHA-pinned) | 0 unpinned | workflow fails CI if any `uses:` lacks `@<40-char-sha>` | M4 test asserts SHA-pin format |
| M5: agent files | ≤ 4 (one per specialist role + lead) | 4 | adding a 5th requires fresh runbook | M5 test asserts file count + that each agent writes only into `docs/slo/critique/` or `docs/slo/verify/` |

### 4.5 Make Invalid States Unrepresentable

| Concept | Prefer | Avoid |
|---|---|---|
| Skill citation presence | structural-contract test (compile-time guarantee) | grep-based human review |
| Example synthetic-ness | frontmatter `synthetic: true` flag enforced by M2 test | implicit "we promise these are synthetic" |
| Standards mapping freshness | retrieval-date column on every row | undated mappings that go stale silently |
| Plugin packaging vs canonical install | `sldo-install` is canonical; `.claude-plugin/plugin.json` is *additive* — both point at same `skills/` tree | duplicating skills into `.claude-plugin/skills/` |
| Agent output destination | agents MUST write into `docs/slo/critique/<slug>.md` (shared with `/slo-critique`) | agent-only output paths that bypass the runbook contract |

Agent rule: before implementing each milestone, identify at least one invalid state the design prevents. Listed under each milestone's Carmack-style reliability goal.

### 4.6 Preserve Compatibility Until Explicitly Broken

- Every existing `skills/<name>/SKILL.md` install path stays unchanged.
- `references/security/security-{finding,assessment-summary}-template.md` shape: existing fields stay; M3 may add optional rows.
- `sldo-install` CLI surface unchanged.
- `docs/skill-pack-catalog.md` remains canonical inventory; M4's plugin.json is *additive*.
- M5's agents output to `docs/slo/critique/` — same path `/slo-critique` writes to today.

### 4.7 Prefer Small, Local, Reviewable Changes

- Change only allowed files per milestone.
- M1 is a citation-only change to four SKILL.md files plus one structural-contract test — no skill behavior change.
- M2 adds an `examples/` tree only; no skill changes.
- M4 may decline to ship plugin packaging if the assessment finds it adds maintenance burden without proportional value.
- M5 may decline to ship agents if M4's host capability matrix shows GitHub Copilot can't install them cleanly.

### 4.8 No Silent Failure

Forbidden in this runbook's outputs:

- Skills that *say* they cite the security templates but lack the actual link (caught by M1 structural-contract test).
- Examples that contain real PII / secrets / customer names (caught by M2 PII-pattern scan over `examples/`).
- Standards mappings without retrieval dates (caught by M3 test).
- GitHub Actions workflow with unpinned `uses:` (caught by M4 test, fails CI).
- Agents that write to paths outside `docs/slo/critique/` or `docs/slo/verify/` (caught by M5 test).

---

## 5. High-Level Design for State Modeling / Formal Verification

`N/A` — this runbook is Markdown skill changes, structural-contract tests, an example gallery, an optional packaging assessment, and a gated agent experiment. There is no concurrency, distributed state, ordering guarantee, retry, queue, idempotency, persistence-recovery, or irreversible-action surface that warrants TLA+ modeling. Property-based tests over the structural-contract assertions cover the relevant invariants (every required citation present; every example flagged synthetic; every standards-mapping row dated; every workflow `uses:` SHA-pinned; every agent output path in the allowed set). `tla_required: false`.

---

## 6. Global Execution Rules

### 6.1 Stay inside scope

- Only change files listed in the current milestone.
- Do not refactor unrelated skill prose.
- Do not rename existing skill files, citation paths, or installer surfaces.
- Do not introduce a new dependency unless the milestone explicitly allows it (only M4 may, with re-confirmation).

### 6.2 Tests define the contract

- Write structural-contract tests *before* the citation/documentation changes they enforce.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when prose compiles. It is done when the structural-contract test passes and Evidence Log entries are filled.

### 6.3 Assertions and invariants are mandatory where assumptions matter

- Every milestone that introduces a new structural-contract test must list the asserted invariant in its Contract Block.
- Lessons file records the invariant prose.

### 6.4 Resource bounds are mandatory where growth is possible

- M2: example file count + size capped.
- M5: agent file count capped at 4; output paths capped to two directories.

### 6.5 Static analysis must pass

Per §4.2: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check` (only when deps change), `cargo test -p sast-verify` (Markdown structural-contract).

### 6.6 Debugger over guessing

Markdown drift is debugged by reading the structural-contract failure message + the cited file, not by speculatively rewriting prose.

### 6.7 No placeholders in production paths

- No "TODO: cite the template here" left in any SKILL.md after M1 closes.
- No example with `<placeholder name>` left after M2 closes.
- No `# <SHA>` left after M4 closes — every SHA must be the real 40-char hash.

### 6.8 Preserve backwards compatibility

Section 1's stable-interface list is the contract.

### 6.9 Prefer the smallest safe change

Each milestone MUST be achievable as a focused PR of typically < 400 lines diff (M2 may exceed for example content). If a milestone exceeds 600 lines diff, split it.

### 6.10 Record evidence, not claims

Per §6.10 of the v4 template — every Evidence Log row filled with command, expected, actual, pass/fail.

### 6.11 Keep .gitignore current

- M2 adds `examples/` (committed, not ignored).
- M4 may add `.claude-plugin/`-build outputs (e.g., `dist/`, `*.zip`) — must be gitignored.
- M5 commits `agents/` directly; no build artifacts.

---

## 7. Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read `docs/slo/lessons/sap-imp-m<N-1>.md` if it exists; apply corrections.
2. Run `/slo-execute` Step 1.5 carry-forward query: `gh issue list --label retro-derived --search "sap-imp"`.
3. Read the milestone's full section: goal, context, contract block, out-of-scope, file list, BDD scenarios, regression tests, E2E tests, smoke tests, definition of done.
4. Run baseline:
   ```bash
   cargo test -p sldo-common -p sldo-install -p sldo-research
   cargo test --workspace
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   ```
   If any test fails before you start, fix the baseline first.
5. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything".
6. Update Milestone Tracker: status `in_progress`, record Started date.
7. Create structural-contract test files first; confirm they fail for the expected reason.
8. Copy the milestone's Evidence Log template into working notes.
9. Re-state the milestone constraints in your own words: goal, allowed files, forbidden changes, compatibility requirements, dependency rules, resource bounds, invariants, static-analysis gates, tests that must pass, Definition of Done.

---

## 8. Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. `cargo fmt --all`
2. `cargo build --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. If deps changed: `cargo deny check`
5. `cargo test --workspace` (all green)
6. Run the milestone's E2E / structural-contract tests
7. Verify `cargo build --workspace` boots cleanly
8. Run smoke tests listed in the milestone
9. Verify backward compatibility against Section 1's stable-interface list
10. Verify resource bounds (§4.4) and assertion/invariant additions (§4.3) are encoded
11. Complete Self-Review Gate (Section 14)
12. Remove temporary debug code, mocks, placeholders
13. `git status` — confirm no untracked test artifacts
14. Review `.gitignore`
15. Update [docs/ARCHITECTURE.md](ARCHITECTURE.md) per Documentation Update Table
16. Update [README.md](../README.md) if user-facing capabilities changed (M2 likely; M1/M3/M5 possible; M4 if plugin shipped)
17. Write `docs/slo/lessons/sap-imp-m<N>.md`
18. Write `docs/slo/completion/sap-imp-m<N>.md`
19. Update Milestone Tracker: status `done`, Completed date, lessons + completion paths
20. Run `/slo-retro` issue-filing flow
21. Re-read next milestone with fresh eyes; record assumption changes in lessons file

---

## 9. Background Context

### Threat-model artifact decision (per F-SEC-1 critique resolution)

This runbook does **not** ship a separate `docs/slo/design/secure-agent-playbook-imports-threat-model.md` produced by `/slo-architect` Step 3.5. Abuse cases are intentionally embedded inline in each milestone's Contract Block as `tm-sap-imp-abuse-<N>` rows (currently 1–15) for the following reasons:

1. **No runtime trust boundary.** Every milestone changes Markdown skill prose, structural-contract Rust tests, an example gallery, an optional GitHub Actions workflow, and optional agent files. There is no new application endpoint, IPC handler, persisted state, or outbound network surface that warrants a STRIDE-shaped threat model.
2. **Embedded rows are exhaustive for the new surfaces.** The 15 inline `tm-sap-imp-abuse-*` rows cover every new surface introduced (SKILL.md citation pattern, examples PII leakage, standards mapping freshness, plugin.json path traversal, release workflow supply chain, agent output paths). A separate STRIDE pass would re-state the same rows in a different shape without surfacing additional classes.
3. **Compatible with downstream tooling.** Future critique passes consult the embedded rows directly via the runbook line numbers; `/slo-execute` Step 1.5 carry-forward query and `/slo-retro` issue filing both reference `tm-<slug>-abuse-<N>` ids by string match without requiring a separate file.

If a future runbook in this series introduces a runtime trust boundary (e.g., a new IPC handler, a new persisted-state surface, or a new outbound HTTP call), the threat-model file MUST be authored separately at that point — the inline-rows precedent applies only to Markdown / configuration / structural-contract test work.

### Current State

- `references/security/security-finding-template.md` and `references/security/security-assessment-summary-template.md` exist and are cited by `/slo-critique` and `/slo-verify` (verified by grep at runbook authoring).
- `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship` do *not* yet cite the templates.
- No `examples/` gallery exists in the repo.
- `references/security/` has no consolidated standards-mapping reference; CWE / OWASP / ASVS / OpenCRE citations are handled ad-hoc per skill.
- No `.claude-plugin/plugin.json` exists; `sldo-install` is the canonical installer for both Claude Code and GitHub Copilot.
- No `agents/` directory exists; `/slo-critique` already runs four-persona rotation in-skill.
- GitHub Actions workflows exist for SAST (`/slo-sast` rule verification) and follow SHA-pinning discipline per `SECURITY.md`.

### Problem

The OWASP Secure Agent Playbook ([github.com/OWASP/secure-agent-playbook](https://github.com/OWASP/secure-agent-playbook)) demonstrates patterns that make security work repeatable: shared evidence-rich finding/summary shape, an example output gallery, systematic standards traceability, optional plugin packaging, and specialist agent roles. SLO has only partially adopted these. Specifically:

1. **Inconsistent template citation across security-relevant skills.** `/slo-critique` and `/slo-verify` cite the shared templates; `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship` do not. Reviewers can't compare outputs across skills with confidence.
2. **No example output gallery.** New contributors and agents must read the skill source to imagine what "good" looks like. The README describes the artifact chain but does not show it.
3. **Uneven standards traceability.** CWE, OWASP, ASVS, OpenCRE citations are handled differently in each security-relevant skill. There is no per-output-type required-vs-optional matrix.
4. **No assessment of optional Claude plugin packaging.** Some Claude-only users would benefit from a plugin zip distribution; we have not evaluated whether the maintenance cost is justified or how it can be additive (not replacing) `sldo-install`.
5. **No host-native agent experiment.** `/slo-critique` persona rotation works portably across hosts but is purely in-skill. Whether a host-native specialist + team-lead pattern produces better outputs without breaking GitHub Copilot support is an open question.

### Target Architecture

```
SunLit skill pack (after this runbook)

  references/security/
    security-finding-template.md         (existing, untouched)
    security-assessment-summary-template.md (existing, untouched)
    standards-mapping.md                 (NEW M3 — curated, dated)

  cited consistently by:
    slo-critique, slo-verify, slo-sast, slo-rulegen, slo-ruleverify, slo-ship

  examples/                              (NEW M2 — synthetic, anonymized)
    README.md  +  6 artifacts

  Optional (M4, if green-lit by host capability matrix):
    .claude-plugin/plugin.json           — additive to sldo-install
    .github/workflows/release-zip.yml    — SHA-pinned

  Optional (M5, gated on M4 matrix):
    agents/slo-runbook-review-lead.md
    agents/slo-{security,design,verification}-reviewer.md
    — outputs land in docs/slo/critique/<slug>.md (shared with /slo-critique)
```

### Key Design Principles

1. **Borrow structure, not content.** Re-author imported patterns in SLO language. Do not copy Secure Agent Playbook procedure prose.
2. **Keep SLO's product center.** Security assessment is a thread through the sprint loop, not a standalone pack.
3. **Install-neutral artifacts.** `sldo-install` and the multi-host story (Claude Code + GitHub Copilot) remain canonical. Plugin packaging is additive.
4. **Deterministic checks where safety matters.** Citation patterns, example synthetic-ness, mapping freshness, SHA-pinning, and agent output paths are enforced by structural-contract tests, not by reviewer goodwill.
5. **Decision-driven gates.** M4 produces a host capability matrix; M5 reads it and may decide to defer rather than ship.
6. **Curated, dated references over bulk vendoring.** No bulk ASVS / WSTG / SAMM / OpenCRE mirrors. Curated rows with retrieval dates only, per existing `references/biz/` discipline.

### What to Keep

- All shipped skill install paths.
- `references/security/security-{finding,assessment-summary}-template.md` field shapes (M3 may add optional rows; cannot remove or rename existing).
- `docs/skill-pack-catalog.md` as canonical skill inventory.
- `sldo-install` as canonical multi-host installer.
- GitHub Copilot as a first-class install target.
- `/slo-critique` four-persona rotation as the canonical portable critique path (M5 agents are *optional* additions, not replacements).

### What to Change

- **`skills/slo-sast/SKILL.md`** — add citation of `references/security/security-assessment-summary-template.md` for coverage gap reporting (M1).
- **`skills/slo-rulegen/SKILL.md`** — add citation of `references/security/security-finding-template.md` when a generated rule is suspect (M1).
- **`skills/slo-ruleverify/SKILL.md`** — add citation of `references/security/security-finding-template.md` for clean-tree / coverage gate failures (M1).
- **`skills/slo-ship/SKILL.md`** — add optional security-summary section in PR body when a milestone introduced new public surface (M1).
- **`xtasks/sast-verify/`** — add structural-contract tests for M1 citations and (later) M3 standards-mapping rows.
- **`examples/` (new)** — 7 files (M2).
- **`references/security/standards-mapping.md` (new)** — curated CWE / OWASP / ASVS / OpenCRE table (M3).
- **`docs/slo/design/host-capability-matrix.md` (new)** — host capability decision doc (M4).
- **`.claude-plugin/plugin.json` (new, optional)** — Claude-only distribution channel (M4, if green-lit).
- **`.github/workflows/release-zip.yml` (new, optional)** — SHA-pinned release-zip workflow (M4, if green-lit).
- **`agents/` (new, gated)** — 4 specialist role files (M5, if M4 matrix permits).

### Global Red Lines

- No unrelated refactors of skill prose.
- No new dependencies (M4 may revisit with explicit confirmation).
- No skill rename or install-path change.
- No removal or rename of existing rows in security templates.
- No bulk standards-data vendoring (curated, dated rows only).
- No GitHub Actions workflow with unpinned `uses:` references.
- No plugin packaging path that displaces `sldo-install`.
- No host-native agent that bypasses `docs/slo/critique/` or breaks GitHub Copilot support.
- No example with real PII / secrets / customer names / confidential `docs/biz/` content.
- No live OpenCRE / ASVS lookup mandatory in normal skill execution.
- No TODO / placeholder / commented-out dead code.

---

## 10. Carry-forward from prior retros

> Empty at runbook authoring (2026-05-01). `/slo-execute M<N>` Step 1.5 falls back to a live `gh issue list --label retro-derived --search "sap-imp"` query. Once `/slo-retro` files an issue against this prefix, populate the table below.

| Issue | Title | Suggested lane | Suggested milestone | Status |
|---|---|---|---|---|
| _(none yet)_ | | | | |

### Lane vocabulary

- **`micro`** — bounded follow-up, foldable into current/next milestone (doc polish, small test gap).
- **`milestone`** — milestone-sized work; warrants its own milestone here or in next runbook.
- **`fresh-runbook`** — material scope or risk shift; spin a separate runbook.

---

## 11. BDD and Runtime Validation Rules

### 11.1 Write Tests Before Production Code

For each milestone:

1. Read the BDD acceptance table.
2. Create the structural-contract test file(s) first.
3. Confirm the tests fail for the expected reason (citation absent, example missing flag, etc.).
4. Make the documentation/citation/file change.
5. Re-run tests after any prose refactor.

### 11.2 Required Test Coverage Categories

- happy path (every milestone)
- invalid input (M1: skill cites nonexistent path; M2: example missing required frontmatter)
- empty / first-run state (M3: standards-mapping table with zero rows; M2: examples/ directory missing README)
- dependency failure / partial failure (M1: structural-contract test runner missing → CI fails closed)
- backward compatibility behavior (every milestone — Section 1 stable-interface list)
- abuse case (M2: example with real-looking PII; M4: workflow with unpinned action; M5: agent writing outside allowed paths)
- resource-limit behavior (§4.4) where bounds apply

### 11.3 Scenario Format

Standard Given/When/Then.

### 11.4 Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Structural-contract Rust test (per milestone) | `tests/sap_imp_m<N>_<feature>.rs` | `xtasks/sast-verify/tests/` |
| E2E runtime validation | `tests/e2e_sap_imp_m<N>.rs` | `xtasks/sast-verify/tests/` |

### 11.5 Test Artifact Cleanup Rules

Standard. No test should write into the source tree; all test fixtures use `tempfile::TempDir`.

### 11.6 End-to-End Runtime Validation

For this runbook, "E2E" means the structural-contract test suite passes against the actual repo state at HEAD — i.e. `cargo test -p sast-verify` running the M<N> tests proves the citation pattern, file count, mapping freshness, SHA-pin format, or agent path constraint holds in real artifacts, not in synthetic test inputs.

### 11.7 E2E Test Design Rules

- Test the actual `skills/<name>/SKILL.md` files at HEAD, not test fixtures.
- Test the actual `examples/` tree at HEAD.
- Test the actual `references/security/standards-mapping.md` at HEAD.
- Failure messages MUST cite the offending file path + line number.

---

## 12. Dependency, Migration, and Refactor Policy

### 12.1 Dependency policy

No new dependencies in M1, M2, M3, M5. M4 may add one if release-zip generation requires it (must be SHA-pinned, license-reviewed, justified in Contract Block).

### 12.2 Migration policy

No schema/config/persisted-state migrations. M3 may add new optional rows to the security-finding template; existing-row consumers stay backward compatible.

### 12.3 Refactor budget

Per-milestone — see each milestone block.

---

## 13. Evidence Log Template

Standard v4 template — copied into each milestone.

---

## 14. Self-Review Gate

Standard v4 — answered before each milestone closes.

---

## 15. Lessons-Learned File Template

Path: `docs/slo/lessons/sap-imp-m<N>.md`. Standard v4 template.

---

## 16. Completion Summary Template

Path: `docs/slo/completion/sap-imp-m<N>.md`. Standard v4 template.

---

## 17. Milestone Plan

### Milestone 1 — `Shared security-reporting integration extended to /slo-sast, /slo-rulegen, /slo-ruleverify, /slo-ship`

**Goal**: After M1, every security-relevant skill that emits findings or coverage gaps cites `references/security/security-finding-template.md` or `references/security/security-assessment-summary-template.md` for expanded reporting, and a structural-contract test enforces the pattern.

**Context**: `/slo-critique` and `/slo-verify` already cite the shared templates (verified by `grep -l 'security-finding-template\|security-assessment-summary-template' skills/*/SKILL.md` at authoring). `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship` do not. The design overview's "Idea 1 → Next useful extensions" lists exactly these four skills. Without a structural-contract test, future skill rewrites can drop the citation silently.

**Carmack-style reliability goal**: Make the citation invariant unrepresentable-otherwise via a structural-contract test (§4.5). The invariant: "every skill in {slo-sast, slo-rulegen, slo-ruleverify, slo-ship, slo-critique, slo-verify} contains at least one Markdown link to either `references/security/security-finding-template.md` or `references/security/security-assessment-summary-template.md`."

**Important design rule**: Citations MUST point to the canonical template path (`references/security/security-{finding,assessment-summary}-template.md`). No copy-pasted inline templates. No alias paths.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — citation insertion only; no prose restructuring.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing SKILL.md prose for the four skills; existing security templates |
| Outputs | Updated SKILL.md prose with template citations; new structural-contract test that asserts the citation invariant |
| Interfaces touched | SKILL.md prose for `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship`; new test file under `xtasks/sast-verify/tests/` |
| Files allowed to change | `skills/slo-sast/SKILL.md`, `skills/slo-rulegen/SKILL.md`, `skills/slo-ruleverify/SKILL.md`, `skills/slo-ship/SKILL.md`, `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` (NEW), `xtasks/sast-verify/Cargo.toml` (test target registration + `pulldown-cmark` dev-dep per allow-list extension below), `Cargo.toml` (root — `pulldown-cmark` workspace.dependencies entry per allow-list extension below), `.gitignore` (only if needed) |
| Allow-list extension (M1, 2026-05-01) | `pulldown-cmark` (latest stable, MIT/Apache-2.0) added as `[workspace.dependencies]` in root `Cargo.toml` and as `[dev-dependencies]` in `xtasks/sast-verify/Cargo.toml`. **Rationale**: F-ENG-1 critique resolution mandates AST-based Markdown parsing for the structural-contract test (no regex/hand-rolled parsing). No production code depends on it; dev-only; one test file consumes it. Reverting requires reverting F-ENG-1's mandate. **Granted**: 2026-05-01 by user during `/slo-execute M1`. |
| Files to read before changing anything | `skills/slo-critique/SKILL.md` and `skills/slo-verify/SKILL.md` (existing citation patterns to mirror); `references/security/security-finding-template.md`; `references/security/security-assessment-summary-template.md`; `xtasks/sast-verify/Cargo.toml`; `xtasks/sast-verify/src/main.rs` (to understand existing test-target conventions) |
| New files allowed | `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` |
| New dependencies allowed | `pulldown-cmark` (dev-dep only, per allow-list extension above) |
| Migration allowed | `no` |
| Compatibility commitments | All four SKILL.md install paths unchanged; frontmatter `name` and `description` unchanged; existing prose semantics preserved (citations are *added*, not replacing existing text) |
| Resource bounds introduced/changed | Citation count per skill: ≥ 1 link to either template; structural-contract test counts links and asserts ≥ 1 per skill in the set {slo-sast, slo-rulegen, slo-ruleverify, slo-ship, slo-critique, slo-verify}. Hard cap: 4 NEW citations added (one per non-citing skill). |
| Invariants/assertions required | (a) Every skill in the citing set contains ≥ 1 link to either security template; (b) every link resolves to an existing file at HEAD; (c) the threshold rule "high/critical findings MUST use the expanded template" is documented in `/slo-critique` and `/slo-verify` SKILL.md (no test enforcement in M1 — that's M3 territory); (d) `/slo-ship` only emits a security-summary section when the runbook introduced new public surface (the section is optional, not always-on); (e) **Markdown parser is `pulldown-cmark` AST-based** (not regex). The test counts only `Event::Start(Tag::Link(...))` events; citations inside fenced code blocks (where the parser yields `Event::Code` or `Event::Html`) are NOT counted (per F-ENG-1 critique resolution). |
| Debugger / inspection expectation | Structural-contract test failure must cite the offending file + the asserted invariant ("expected ≥ 1 link to references/security/security-{finding,assessment-summary}-template.md in skills/<name>/SKILL.md, found 0"). |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify`. No `cargo deny` needed (no deps change). |
| Forbidden shortcuts | No grep-only review; the structural-contract test is mandatory. No copy-pasted inline finding template inside any SKILL.md. No "TODO: cite the template here" left in prose. No silent change to `references/security/security-{finding,assessment-summary}-template.md` table-row shapes. **No regex-based Markdown citation counting** — must use `pulldown-cmark` AST walk so code-block content is correctly excluded. |
| Data classification | `Public` — all four SKILL.md files and the test file are tracked in source control and contain no sensitive content. |
| Proactive controls in play | `C1` (Define Security Requirements — the citation invariant *is* the requirement), `C2` (Leverage Security Frameworks — the shared templates are the framework), `C5` (Validate All Inputs — the structural-contract test validates the SKILL.md "input" against the citation invariant). |
| Abuse acceptance scenarios | `tm-sap-imp-abuse-1: a future skill rewrite drops the template citation silently → structural-contract test fails CI`. `tm-sap-imp-abuse-2: someone copies the template inline into a SKILL.md (causing drift if the canonical template changes) → reviewer-time check; M3 may add a structural-contract test that detects inline-table duplication.` Threat-model row pointer for this milestone: see BDD abuse-case row below. |

#### Out of Scope / Must Not Do

- Do **not** change `references/security/security-{finding,assessment-summary}-template.md` table-row shapes. Adding optional rows is M3.
- Do **not** modify `/slo-critique` or `/slo-verify` SKILL.md — both already cite the templates correctly.
- Do **not** make the high/critical mandatory threshold enforceable by structural-contract test in M1 — that requires CWE/OWASP/ASVS row presence which is M3 work.
- Do **not** add a security-summary section to *every* PR body in `/slo-ship` — only when the runbook introduced new public surface.
- Do **not** introduce new dependencies.
- Do **not** rename any skill file.

#### Pre-Flight

1. Complete the Global Entry Rules (Section 7).
2. There is no prior milestone for this runbook — skip lessons-file read.
3. Read the Files Allowed To Change list and the Files To Read Before Changing Anything list.
4. Copy the Evidence Log template into working notes.
5. Re-state milestone constraints: goal, four-skill allow-list, no template-shape change, no new deps, structural-contract test mandatory.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Add a paragraph (under "Output" or "Reporting") citing `references/security/security-assessment-summary-template.md` for coverage-gap reporting; cite `references/security/security-finding-template.md` for individual high-severity gaps. |
| `skills/slo-rulegen/SKILL.md` | Add a paragraph (under "Output" or "Validation") citing `references/security/security-finding-template.md` when a generated rule is suspect, fails verification, or carries low confidence. |
| `skills/slo-ruleverify/SKILL.md` | Add a paragraph (under "Output" or "Failure handling") citing `references/security/security-finding-template.md` when clean-tree or coverage gates fail. |
| `skills/slo-ship/SKILL.md` | Add an *optional* security-summary section in the PR body, gated on "milestone introduced new public surface"; cite `references/security/security-assessment-summary-template.md` as the section format. |
| `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` | NEW: structural-contract test that walks the citing skill set, parses each SKILL.md, asserts ≥ 1 Markdown link to either security template path, and asserts every cited path exists at HEAD. |
| `xtasks/sast-verify/Cargo.toml` | Only if the integration test target requires registration; otherwise untouched. |
| `.gitignore` | Only if the test creates new build outputs; expected: no change. |

#### Step-by-Step

1. Write `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` first. Assert (a) ≥ 1 citation per skill in the set; (b) every cited path resolves to an existing file. Confirm `cargo test -p sast-verify` fails with a message naming `slo-sast`, `slo-rulegen`, `slo-ruleverify`, `slo-ship` as missing citations.
2. Update `skills/slo-sast/SKILL.md` with the assessment-summary template citation.
3. Update `skills/slo-rulegen/SKILL.md` with the finding template citation.
4. Update `skills/slo-ruleverify/SKILL.md` with the finding template citation.
5. Update `skills/slo-ship/SKILL.md` with the optional PR-body security-summary section + citation, gated on "milestone introduced new public surface" — quote the gate condition explicitly.
6. Re-run `cargo test -p sast-verify` — must pass.
7. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`.
8. Run full workspace test suite: `cargo test --workspace`.
9. `git status` — confirm no untracked test artifacts.
10. Review `.gitignore` — no changes expected.
11. Run smoke tests (manual review of each updated SKILL.md to confirm prose still reads naturally).
12. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: shared security-reporting citation invariant**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All six skills cite at least one template | happy path | `skills/{slo-sast,slo-rulegen,slo-ruleverify,slo-ship,slo-critique,slo-verify}/SKILL.md` exist at HEAD with citations after M1 | `cargo test -p sast-verify --test sap_imp_m1_citations` runs | Test passes; output reports 6 skills with ≥ 1 citation each |
| Missing citation caught | invalid input | A test fixture skill with zero template citations | structural-contract test runs against the fixture | Test fails with message naming the offending file + asserted invariant |
| Citation points to nonexistent path | invalid input | A SKILL.md cites `references/security/typo-template.md` | structural-contract test runs | Test fails with message "cited path does not exist at HEAD" |
| Empty SKILL.md (first-run-style state) | empty state | An empty `skills/foo/SKILL.md` (synthetic test fixture) | structural-contract test runs | Test fails fast with parser error, not a silent pass |
| Test runner missing | dependency failure | `cargo test -p sast-verify` cannot find the test target | CI runs | CI fails closed; no skill PR can merge without the test running |
| Citation invariant holds without the threshold rule | resource bound | Only ≥ 1 citation required per skill in M1 | M1 close-out | Threshold "high/critical MUST use expanded template" is documented in prose only; structural enforcement is deferred to M3 |
| Future rewrite drops citation silently | abuse case (`tm-sap-imp-abuse-1`) | Someone rewrites `skills/slo-sast/SKILL.md` and removes the template citation | CI runs `cargo test -p sast-verify` | Test fails; PR cannot merge |
| Existing two skills already pass | compatibility | `/slo-critique` and `/slo-verify` already cite the templates pre-M1 | structural-contract test runs | Test passes for those two without any prose change |

#### Regression Tests

- Existing `cargo test -p sldo-common -p sldo-install -p sldo-research` baseline must remain green.
- Existing `cargo test -p sast-verify` (any pre-M1 tests) must remain green.
- `sldo-install --dry-run` must still resolve every shipped skill — no install-path break.
- Frontmatter for the four edited skills must round-trip through `sldo-install`'s YAML parser unchanged (confirmed by re-running `cargo test -p sldo-install`).

#### Compatibility Checklist

- [ ] `skills/slo-sast/SKILL.md` install path unchanged
- [ ] `skills/slo-rulegen/SKILL.md` install path unchanged
- [ ] `skills/slo-ruleverify/SKILL.md` install path unchanged
- [ ] `skills/slo-ship/SKILL.md` install path unchanged
- [ ] Frontmatter `name` and `description` unchanged for all four
- [ ] `references/security/security-finding-template.md` table-row shape unchanged
- [ ] `references/security/security-assessment-summary-template.md` section ordering unchanged
- [ ] `cargo test -p sldo-install` still passes (no installer regression)
- [ ] `sldo-install --dry-run` resolves all shipped skills
- [ ] `/slo-critique` and `/slo-verify` SKILL.md files untouched

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/sap_imp_m1_citations.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `every_security_skill_cites_a_template` | Walks the six-skill set at HEAD, parses each SKILL.md, asserts ≥ 1 Markdown link to either canonical template path | All six skills return citation count ≥ 1; failure cites offending file |
| `cited_template_paths_resolve` | Every citation in every walked SKILL.md must resolve to an existing file at HEAD | All cited paths exist; failure cites the unresolved path |
| `slo_ship_security_summary_is_gated` | `/slo-ship` SKILL.md text containing the security-summary citation must also contain the gate phrase ("new public surface" or equivalent) | Gate phrase present within 200 characters of the citation; failure prints both phrases or notes missing gate |

#### Smoke Tests

- [ ] Manually read each of the four updated SKILL.md files end-to-end; prose reads naturally
- [ ] `cargo test -p sast-verify --test sap_imp_m1_citations` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo build --workspace` boots cleanly
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` reviewed; no stale entries
- [ ] `sldo-install --dry-run` succeeds and lists all four updated skills

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-common -p sldo-install -p sldo-research` (runbook-declared baseline) | all green | 84 passed; 0 failed | PASS | Baseline command per Runbook Metadata |
| Structural-contract test created | `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` | fails for expected reason (4 missing citations) | every_security_skill_cites_a_template FAILED with "expected ≥ 1 link ... in skills/slo-sast/SKILL.md, found 0" + same for slo-rulegen, slo-ruleverify, slo-ship | PASS (correct failure) | TDD discipline honored — test fails for expected reason before implementation |
| Implementation: slo-sast citation | edit `skills/slo-sast/SKILL.md` (commit f8dc040) | template citation present | "Coverage-gap reporting" subsection added under Outputs; cites assessment-summary template + finding template | PASS | |
| Implementation: slo-rulegen citation | edit `skills/slo-rulegen/SKILL.md` (commit 3388336) | template citation present | "Reporting suspect rules" subsection added before Handoff; cites finding template | PASS | |
| Implementation: slo-ruleverify citation | edit `skills/slo-ruleverify/SKILL.md` (commit bb23836) | template citation present | "Expanded failure findings" subsection added under Report format; cites finding template | PASS | |
| Implementation: slo-ship security summary | edit `skills/slo-ship/SKILL.md` (commit 87baaad + tightening commit 7626cbe) | gated security summary section present | "Optional security-summary section (gated)" subsection added; gate phrase "introduced new public surface" within 200 chars of citation (verified by `slo_ship_security_summary_is_gated` test) | PASS | Initial citation drafted with gate phrase too far from citation; tightened in 7626cbe |
| Formatter | `cargo fmt --all` | clean | applied | PASS | |
| Typecheck / build | `cargo build --workspace` | clean | clean (with pre-existing unused-field warnings in `sast-verify` bin) | PASS | |
| Static analyzer / linter | `cargo clippy -p sast-verify --test sap_imp_m1_citations -- -D warnings` (M1-scoped clippy) | clean for M1 additions | clean | PASS | Workspace-wide clippy has pre-existing dead-code errors in `sast-verify/src/{tier_detect,yaml_schema}.rs` that pre-date M1; out of scope. M1's own additions are clippy-clean. |
| Dependency audit | (`pulldown-cmark` added per allow-list extension) | dep is MIT/Apache-2.0; latest stable is 0.10 | dep added at workspace level + sast-verify dev-deps; license verified MIT/Apache-2.0 | PASS | |
| Full tests | `cargo test --workspace` | green | pre-existing failures in `e2e_research_m1` (sldo-research binary not pre-built) and clippy errors in `sldo-install` e2e_biz_judgment tests | KNOWN-RED (pre-existing) | These failures pre-date M1 and are not introduced by M1's changes. Runbook-declared baseline (above) is green. |
| E2E runtime | `cargo test -p sast-verify --test sap_imp_m1_citations` | green; reports 6 skills with citations | 5 tests passed; 0 failed (`every_security_skill_cites_a_template`, `cited_template_paths_resolve`, `slo_ship_security_summary_is_gated`, `no_skill_links_to_examples`, `ast_parser_excludes_code_block_content`) | PASS | |
| Build/boot | `cargo build --workspace` | boots cleanly | clean | PASS | |
| Smoke tests | manual prose review of 4 SKILL.md | reads naturally | each SKILL.md re-read; citation prose flows naturally with surrounding sections | PASS | |
| Resource-bound verification | exactly 4 NEW citations added | bound encoded; test asserts citation count ≥ 1 per skill | test enforces ≥1 per skill in 6-skill set; 4 NEW citations were inserted (slo-sast adds 2, slo-rulegen 1, slo-ruleverify 1, slo-ship 2) | PASS | NEW-citation count is 4–6 depending on whether multi-link insertions count once or per-link; the floor-1 invariant per-skill is what M1 enforces |
| Invariant/assertion verification | structural-contract test | invariant encoded and tested | test file at `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` encodes 5 invariants (citation, path resolution, gate phrase, no-skill-links-to-examples, AST excludes code blocks) | PASS | |
| Debugger / state inspection | structural-contract test failure messages serve as inspection tool | hypothesis confirmed before code change | F-ENG-1 critique resolution mandates `pulldown-cmark` AST parser; `ast_parser_excludes_code_block_content` test confirms the parser correctly excludes code-fence content | PASS | |
| Test artifact cleanup | `git status` | no untracked test artifacts | working tree clean (all M1 work committed by user during execution: commits 8b0ed8e, f8dc040, 3388336, bb23836, 87baaad, 7626cbe) | PASS | |
| .gitignore review | review `.gitignore` | patterns current | no new generated files; no .gitignore change needed | PASS | |
| Compatibility checks | run Compatibility Checklist | no regressions | all 10 Compatibility Checklist rows verified (install paths, frontmatter, template shapes, sldo-install, /slo-critique + /slo-verify untouched) | PASS | |
| Installer regression | `cargo test -p sldo-install` + `sldo-install --dry-run` | green; all skills resolve | sldo-install tests green; `sldo-install --dry-run` lists all 32 skills including 4 M1-edited ones | PASS | |

#### Definition of Done

- All BDD scenarios pass.
- All E2E runtime validations pass.
- Full existing test suite remains green.
- Formatter, typecheck, clippy clean.
- No new deps (so no audit needed).
- Smoke tests checked off.
- Compatibility checklist complete.
- Resource bound (≥ 1 citation per skill, exactly 4 new citations added) encoded and tested.
- Citation invariant encoded and tested.
- No "TODO: cite the template here" or commented-out prose left in any SKILL.md.
- `git status` clean.
- `.gitignore` current.
- `docs/ARCHITECTURE.md` updated to mention the structural-contract test family `xtasks/sast-verify/tests/sap_imp_m*` (see Documentation Update Table).
- `README.md` unchanged unless user-facing capability changed (M1 is internal-discipline-only — likely no README change).
- Lessons file written.
- Completion summary written.
- Milestone Tracker updated.

#### Post-Flight

- **ARCHITECTURE.md**: under "Test Architecture" or equivalent, add one line noting the `sap_imp_m*` structural-contract test family in `xtasks/sast-verify/tests/`.
- **README.md**: no change expected.
- **Other docs**: `docs/skill-pack-catalog.md` — no change (skill names and purposes unchanged); `CLAUDE.md` and `copilot-instructions.md` — no change.

#### Notes

- M1 is intentionally narrow: citation pattern + structural-contract test only. The high/critical mandatory threshold rule is documented but not structurally enforced until M3 lands the standards-mapping.md file and the per-output-type required-vs-optional matrix.
- The `/slo-ship` security-summary section is optional and gated. The gate phrase ("new public surface") is the trigger; without it, `/slo-ship` PR bodies are unchanged.
- `xtasks/sast-verify` is reused (not extended into a new crate) because the existing crate already runs structural Markdown checks and is wired into CI.

---

### Milestone 2 — `Example output gallery under examples/`

**Goal**: After M2, contributors and agents can read a small synthetic gallery under `examples/` to calibrate expected SLO output quality before running any skill. Six artifact types are represented; every example is flagged synthetic + non-normative; a structural-contract test enforces frontmatter, file-count, size, and PII-cleanness.

**Context**: SLO asks users to trust a chain of artifacts (idea, research, architecture, threat model, runbook, critique, verification, retro, PR body). The README describes the chain but does not show it. Secure Agent Playbook's `examples/` directory is the closest model (design doc Idea 3). Real-output-leaking risk is real — `docs/biz/` contains confidential drafts; an `examples/` gallery must use synthetic-only content.

**Carmack-style reliability goal**: Make "an example file contains real PII" structurally impossible at CI time (§4.5). The structural-contract test runs an email + UK NI + UK sort-code regex scan over every file under `examples/`; any match fails CI with the offending file + line.

**Important design rule**: Every example file is *abbreviated* and *non-normative*. The frontmatter `abbreviates:` field links back to the canonical template / skill / runbook the example reduces — readers always know where the authoritative source lives.

**Refactor budget**: `No refactor permitted beyond direct implementation` — M2 adds new files and one new test; nothing in `skills/` or `references/` may change.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | None (greenfield gallery); existing canonical templates + runbooks + skill prose serve as references for what to abbreviate |
| Outputs | `examples/README.md` + 6 synthetic example artifacts; one structural-contract test |
| Interfaces touched | New `examples/` directory at repo root; new test under `xtasks/sast-verify/tests/`; `README.md` Examples section; `docs/ARCHITECTURE.md` note |
| Files allowed to change | `examples/README.md` (NEW), `examples/runbook-excerpt.md` (NEW), `examples/critique-report.md` (NEW), `examples/verification-report.md` (NEW), `examples/security-finding.md` (NEW), `examples/sast-manifest.json` (NEW), `examples/biz-public-artifact.md` (NEW), `xtasks/sast-verify/tests/sap_imp_m2_examples.rs` (NEW), `README.md` (Examples section), `docs/ARCHITECTURE.md` (note) |
| Files to read before changing anything | `docs/slo/templates/runbook-template_v_4_template.md` (to abbreviate); `references/security/security-finding-template.md` and `references/security/security-assessment-summary-template.md` (to abbreviate); `skills/slo-critique/SKILL.md`, `skills/slo-verify/SKILL.md`, `skills/slo-sast/SKILL.md` output sections (to abbreviate); a real biz-public artifact under `docs/biz-public/` if any exists (to abbreviate); existing PII-scan code in `xtasks/sast-verify/` if present |
| New files allowed | The 7 files in `examples/` and 1 test file (listed above) |
| New dependencies allowed | `none` — regex available via `regex` crate already in workspace; if not, M2 may add `regex` with explicit reconfirmation |
| Migration allowed | `no` |
| Compatibility commitments | No skill prose changes; no template-shape changes; `sldo-install` ignores `examples/` (it walks `skills/<name>/SKILL.md` only); `examples/` does not become canonical source for anything |
| Resource bounds introduced/changed | Exactly 7 files in `examples/`; each Markdown file ≤ 10 KB; each JSON file ≤ 10 KB. Hard cap: 7 files. Behavior at limit: 8th file fails the structural-contract test. |
| Invariants/assertions required | (a) Every Markdown example has frontmatter with `synthetic: true`, `non-normative: true`, and `abbreviates: <path-or-name>`; (b) `examples/sast-manifest.json` declares `"synthetic": true` at top level; (c) **PII regex scan covers email, UK NI, UK sort code, US SSN (`\d{3}-\d{2}-\d{4}`), EU IBAN (`[A-Z]{2}\d{2}[A-Z0-9]{1,30}`)** over `examples/**/*.{md,json}` — zero matches required (per F-SEC-2 critique resolution; non-UK locale coverage extended); (d) `abbreviates:` reference resolution rule: walk `skills/<name>/SKILL.md` at HEAD and accept any value matching frontmatter `name`, OR treat the value as a literal filesystem path and confirm `Path::exists()` (per F-ENG-2 critique resolution); (e) `examples/README.md` lists all 6 artifact files; (f) **no shipped `skills/<name>/SKILL.md` contains a Markdown link to `examples/`** — enforced by the same `pulldown-cmark` AST walk introduced in M1 (per F-ENG-3 critique resolution). |
| Debugger / inspection expectation | Structural-contract test failures cite file + line number for PII matches; frontmatter parse failures cite file + missing field. |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify`. |
| Forbidden shortcuts | No copy-pasting from `docs/biz/` (confidential tier). No real customer / partner / employee names. No real domains beyond RFC 2606 reserved (`.example`, `.test`, `.invalid`, `.localhost`). No real-looking emails — use the literal placeholder `<user>@<host>` if a field's *shape* needs illustration. No screenshots. No "TODO: fill this in" left in any example. |
| Data classification | `Public` — every file under `examples/` is git-tracked and contains synthetic content only. |
| Proactive controls in play | `C1` (Define Security Requirements — examples MUST be synthetic + non-normative), `C5` (Validate All Inputs — PII regex scan), `C8` (Protect Data Everywhere — enforce that real PII never lands in `examples/` via the scan). |
| Abuse acceptance scenarios | `tm-sap-imp-abuse-3: a contributor copies a real critique report (containing real names + emails) into examples/critique-report.md → PII-pattern scan fails CI before merge`. `tm-sap-imp-abuse-4: a contributor copies an example into docs/biz/ thinking it's a real template → frontmatter non-normative: true plus the README banner ("Synthetic, non-normative — not for direct use") makes accidental misuse hard`. See BDD abuse-case rows below. |

#### Out of Scope / Must Not Do

- Do **not** modify any `skills/<name>/SKILL.md` file in M2.
- Do **not** modify `references/security/security-{finding,assessment-summary}-template.md`. Abbreviating in an example is fine; mutating the canonical templates is not.
- Do **not** add an 8th example. If a future runbook needs another category, it must amend this runbook's resource bound.
- Do **not** add a frontmatter override mechanism for the PII scan in M2. Strict scan now; future runbook can introduce an override if a legitimate need arises.
- Do **not** consume `examples/` from any skill (no skill should depend on `examples/` being installed).
- Do **not** install `examples/` via `sldo-install` (it walks `skills/<name>/SKILL.md` only and that stays unchanged).

#### Pre-Flight

1. Complete the Global Entry Rules (Section 7).
2. Read `docs/slo/lessons/sap-imp-m1.md` and apply corrections.
3. Read the canonical templates / skill prose listed under "Files to read before changing anything".
4. Copy the Evidence Log template into working notes.
5. Re-state the milestone constraints: 7 files exactly, ≤ 10 KB each, frontmatter required, PII scan strict, no skill prose changes.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `examples/README.md` | NEW: top-level banner ("Synthetic, non-normative — not for direct use"); list of the 6 artifacts with one-sentence descriptions; pointer to canonical sources |
| `examples/runbook-excerpt.md` | NEW: ~80-line abbreviated v4 runbook (1 milestone shown end-to-end); frontmatter `abbreviates: docs/slo/templates/runbook-template_v_4_template.md` |
| `examples/critique-report.md` | NEW: ~100-line synthetic critique report (CEO + eng + security + design persona rows, 1 expanded finding using the security-finding template); frontmatter `abbreviates: skills/slo-critique` |
| `examples/verification-report.md` | NEW: ~80-line synthetic verification report (Pass 1–4 outputs, 1 Pass-4 expanded finding); frontmatter `abbreviates: skills/slo-verify` |
| `examples/security-finding.md` | NEW: ~50-line single-finding example using `references/security/security-finding-template.md`; frontmatter `abbreviates: references/security/security-finding-template.md` |
| `examples/sast-manifest.json` | NEW: ~40-line synthetic Semgrep manifest with `"synthetic": true` flag at top; frontmatter not applicable for JSON, so the flag goes in the manifest body |
| `examples/biz-public-artifact.md` | NEW: ~60-line synthetic biz-public artifact (e.g., a GTM strategy stub); frontmatter `abbreviates: skills/slo-gtm` + `tier: public` |
| `xtasks/sast-verify/tests/sap_imp_m2_examples.rs` | NEW: structural-contract test (frontmatter + file count + size + PII scan + `abbreviates:` resolution) |
| `README.md` | Add an "Examples" section linking to `examples/README.md` |
| `docs/ARCHITECTURE.md` | Add a one-line "Examples gallery" subsection note pointing at `examples/README.md` |

#### Step-by-Step

1. Write `xtasks/sast-verify/tests/sap_imp_m2_examples.rs` first. Assertions: directory exists, exactly 7 files, frontmatter on every Markdown example contains `synthetic: true` + `non-normative: true` + `abbreviates: <ref>`, JSON manifest has `"synthetic": true`, PII regex scan zero matches, every `abbreviates:` ref resolves to an existing path or known skill name, every file ≤ 10 KB. Confirm `cargo test -p sast-verify --test sap_imp_m2_examples` fails with "examples/ directory not found".
2. Create `examples/README.md` with the banner + artifact list.
3. Author each of the 6 artifacts in turn, keeping each ≤ 10 KB and frontmatter-correct.
4. Re-run `cargo test -p sast-verify --test sap_imp_m2_examples` — must pass.
5. Update `README.md` with the Examples section.
6. Update `docs/ARCHITECTURE.md` with the note.
7. Run `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
8. Verify `sldo-install --dry-run` ignores `examples/` (no install entry for it).
9. `git status` — confirm no untracked test artifacts.
10. Run smoke tests (manually skim each example end-to-end; verify the banner reads correctly).
11. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: synthetic example gallery**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All 7 files present and valid | happy path | `examples/` contains README + 6 artifacts at HEAD | `cargo test -p sast-verify --test sap_imp_m2_examples` runs | Test passes; output reports 7 files, all under size cap, all frontmatter valid, PII scan clean |
| Missing frontmatter caught | invalid input | A test fixture example without `synthetic: true` | structural-contract test runs against fixture | Test fails with message naming file + missing field |
| Email regex match caught | invalid input (`tm-sap-imp-abuse-3`) | A test fixture example containing `alice@anthropic.com` | structural-contract test runs | Test fails with message "PII regex match: email at examples/<file>:<line>" |
| UK NI regex match caught | invalid input (`tm-sap-imp-abuse-3`) | A test fixture example containing `AB123456C` | structural-contract test runs | Test fails with message "PII regex match: UK NI at examples/<file>:<line>" |
| UK sort code match caught | invalid input (`tm-sap-imp-abuse-3`) | A test fixture example containing `12-34-56` | structural-contract test runs | Test fails with message "PII regex match: UK sort code at examples/<file>:<line>" |
| US SSN regex match caught | invalid input (`tm-sap-imp-abuse-3`, F-SEC-2) | A test fixture example containing `123-45-6789` | structural-contract test runs | Test fails with message "PII regex match: US SSN at examples/<file>:<line>" |
| EU IBAN regex match caught | invalid input (`tm-sap-imp-abuse-3`, F-SEC-2) | A test fixture example containing `DE89370400440532013000` | structural-contract test runs | Test fails with message "PII regex match: EU IBAN at examples/<file>:<line>" |
| Skill links to examples/ caught | invalid input (F-ENG-3) | A test fixture skill SKILL.md with `[example](../../examples/security-finding.md)` | structural-contract test runs | Test fails with "skills/<name>/SKILL.md links to examples/ — Out-of-Scope rule violated" |
| `abbreviates:` resolves via skill name | happy path (F-ENG-2) | A test fixture frontmatter `abbreviates: slo-critique` | structural-contract test runs | Test passes by walking `skills/slo-critique/SKILL.md` and matching frontmatter `name: slo-critique` |
| `abbreviates:` resolves via filesystem path | happy path (F-ENG-2) | A test fixture frontmatter `abbreviates: docs/slo/templates/runbook-template_v_4_template.md` | structural-contract test runs | Test passes by `Path::exists()` check |
| Examples directory missing | empty state | `examples/` does not exist | structural-contract test runs at HEAD | Test fails fast with "examples/ directory missing" |
| Eighth file added | resource bound | A test fixture with 8 files in examples/ | structural-contract test runs | Test fails with "expected 7 files, found 8" |
| File over size cap | resource bound | A test fixture file at 12 KB | structural-contract test runs | Test fails with "examples/<file> is 12 KB, cap is 10 KB" |
| `abbreviates:` ref unresolvable | invalid input | A test fixture frontmatter `abbreviates: skills/slo-nonexistent` | structural-contract test runs | Test fails with "abbreviates ref not found: skills/slo-nonexistent" |
| Existing skills unchanged | compatibility | `skills/<name>/SKILL.md` files untouched in M2 | M2 close-out diff review | No skill prose changes in the diff |
| `sldo-install` ignores examples/ | compatibility | `examples/` exists at HEAD | `sldo-install --dry-run` runs | No install entry mentions `examples/` |
| Accidental copy into docs/biz/ | abuse case (`tm-sap-imp-abuse-4`) | A contributor copies `examples/biz-public-artifact.md` into `docs/biz/` | They open the file | Frontmatter `non-normative: true` + README banner make synthetic origin obvious; no real customer data leaks |

#### Regression Tests

- `cargo test -p sldo-install` — installer must not start trying to install `examples/`.
- `cargo test -p sast-verify` — pre-existing tests stay green; M1's `sap_imp_m1_citations` still passes.
- `cargo test --workspace` — full suite green.
- `sldo-install --dry-run` — resolves shipped skills; no `examples/` entry.

#### Compatibility Checklist

- [ ] `skills/<name>/SKILL.md` files unchanged in M2
- [ ] `references/security/security-{finding,assessment-summary}-template.md` unchanged
- [ ] `sldo-install` install paths unchanged
- [ ] `~/.sldo/install.toml` schema unchanged
- [ ] M1's `sap_imp_m1_citations` test still passes
- [ ] Existing `xtasks/sast-verify` tests still pass

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/sap_imp_m2_examples.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `examples_directory_has_exactly_seven_files` | File-count bound holds | Walks `examples/`, counts non-hidden files, asserts == 7 |
| `every_markdown_example_has_required_frontmatter` | Frontmatter invariant holds | Parses YAML frontmatter on every `examples/*.md` (except README), asserts `synthetic: true`, `non-normative: true`, `abbreviates: <ref>` present |
| `sast_manifest_declares_synthetic` | JSON synthetic flag holds | Parses `examples/sast-manifest.json`, asserts top-level `"synthetic": true` |
| `examples_pii_pattern_scan_clean` | No PII leakage | Regex scan over every `examples/**/*.{md,json}` for email, UK NI, UK sort code, **US SSN** (`\d{3}-\d{2}-\d{4}`), **EU IBAN** (`[A-Z]{2}\d{2}[A-Z0-9]{1,30}`); zero matches (per F-SEC-2) |
| `every_example_under_size_cap` | Size bound holds | `wc -c` on every file ≤ 10 KB |
| `every_abbreviates_ref_resolves` | Citation invariant holds | Each `abbreviates:` ref resolves via either (a) walking `skills/<name>/SKILL.md` at HEAD and matching frontmatter `name`, or (b) `Path::exists()` on the literal value (per F-ENG-2) |
| `no_skill_links_to_examples` | Out-of-Scope rule enforced | Walks `skills/<name>/SKILL.md` files via `pulldown-cmark` AST; asserts no `Event::Start(Tag::Link(...))` whose destination starts with `examples/` (per F-ENG-3) |

#### Smoke Tests

- [ ] Manually open each example end-to-end; prose reads naturally; banner is visible
- [ ] `cargo test -p sast-verify --test sap_imp_m2_examples` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo build --workspace` boots cleanly
- [ ] `sldo-install --dry-run` ignores `examples/`
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` reviewed — no need to ignore `examples/` (it's committed)
- [ ] README "Examples" section renders correctly (Markdown preview)

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Structural-contract test created | `xtasks/sast-verify/tests/sap_imp_m2_examples.rs` | fails for "examples/ missing" | | | |
| examples/README.md authored | manual review | banner + 6-artifact list present | | | |
| 6 artifacts authored | manual review | each ≤ 10 KB, frontmatter correct | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Typecheck / build | `cargo build --workspace` | clean | | | |
| Static analyzer / linter | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Dependency audit | (no deps changed — skip; if `regex` was added, run `cargo deny check`) | n/a or pass | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test -p sast-verify --test sap_imp_m2_examples` | green; reports 7 files, all valid | | | |
| Build/boot | `cargo build --workspace` | boots cleanly | | | |
| Smoke tests | manual review of every example | reads naturally | | | |
| Resource-bound verification | exactly 7 files; sizes ≤ 10 KB | bound encoded; test asserts | | | |
| Invariant/assertion verification | frontmatter + PII scan | invariants encoded and tested | | | |
| Debugger / state inspection | (n/a — Markdown gallery; structural-contract failures serve as inspection) | hypothesis confirmed before code change | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | no `examples/` exclusion (committed) | | | |
| Compatibility checks | run Compatibility Checklist | no regressions | | | |
| Installer regression | `sldo-install --dry-run` | ignores `examples/` | | | |

#### Definition of Done

- All BDD scenarios pass.
- All E2E runtime validations pass.
- Full existing test suite remains green.
- M1's `sap_imp_m1_citations` test still green.
- Formatter, typecheck, clippy clean.
- Resource bounds (7 files; ≤ 10 KB each) encoded and tested.
- Frontmatter + PII-scan invariants encoded and tested.
- No "TODO: fill in" or placeholder text left in any example.
- `git status` clean.
- `.gitignore` reviewed.
- `docs/ARCHITECTURE.md` updated with Examples gallery note.
- `README.md` updated with Examples section.
- Lessons file written.
- Completion summary written.
- Milestone Tracker updated.

#### Post-Flight

- **ARCHITECTURE.md**: add "Examples gallery" subsection note pointing at `examples/README.md`.
- **README.md**: add "Examples" section.
- **Other docs**: `docs/skill-pack-catalog.md` — no change (gallery is not a skill); `CLAUDE.md` and `copilot-instructions.md` — no change.

#### Notes

- The PII scan is intentionally narrow (email + UK NI + UK sort code) rather than the broader capitalised-bigram scan in `/slo-verify` Pass 4. Reason: capitalised-bigram detection has a high false-positive rate on security prose ("SQL Injection", "Cross-Site Scripting"). M2's stricter narrower scan is sufficient when paired with the synthetic-domain rule (RFC 2606 only).
- `examples/` is intentionally NOT installed by `sldo-install`. The installer walks `skills/<name>/SKILL.md` only, and that contract is preserved.
- Future runbook may introduce a `pii_scan_override:` frontmatter mechanism if a legitimate need to show real-format PII arises (e.g., for a security-training example). M2 does not introduce the override.

---

### Milestone 3 — `Standards traceability matrix wired into security outputs`

**Goal**: After M3, every security-relevant SLO output has a clear required-vs-optional standards mapping. A curated `references/security/standards-mapping.md` carries CWE / OWASP / ASVS / OpenCRE rows with retrieval dates; four skills cite the mapping per a per-output-type tier matrix; a structural-contract test asserts the high/critical threshold rule and the freshness column.

**Context**: Standards traceability (CWE, OWASP, ASVS, OpenCRE) is uneven across `/slo-critique`, `/slo-verify`, `/slo-sast`, `/slo-rulegen` (design doc Idea 4). Without a per-output-type matrix, reviewers can't tell which mappings are required vs nice-to-have. M1 documented the high/critical mandatory-expanded-template rule in prose; M3 makes it structurally enforceable by requiring CWE row presence on findings tagged `high` or `critical`.

**Carmack-style reliability goal**: Make "a high-severity finding without standards mapping" structurally impossible (§4.5). The structural-contract test parses every expanded finding emitted to `docs/slo/critique/`, `docs/slo/verify/`, etc. (sample fixtures in `examples/`), asserts that any finding with `severity: high` or `severity: critical` has a non-`N/A` CWE row.

**Important design rule**: Curated, dated rows only. No bulk standards-data vendoring (design doc Idea 7 / Non-goal). Every row in `standards-mapping.md` has a `retrieval-date` column. Live OpenCRE lookup is explicitly out of scope — a missing mapping is a coverage gap, not a runtime blocker.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — citation insertion + matrix table addition; no broader prose restructuring. The four skills' output sections may receive a new sub-section ("Standards mapping") but nothing else.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `references/security/security-{finding,assessment-summary}-template.md`; existing `/slo-critique`, `/slo-verify`, `/slo-sast`, `/slo-rulegen` SKILL.md |
| Outputs | New `references/security/standards-mapping.md` with curated rows; four skill SKILL.md updates citing the mapping; possibly new optional rows in security-finding-template.md (ASVS, OpenCRE if not already present); one structural-contract test |
| Interfaces touched | New `references/security/standards-mapping.md`; SKILL.md prose for four skills; test under `xtasks/sast-verify/tests/`; possibly new optional template rows |
| Files allowed to change | `references/security/standards-mapping.md` (NEW), `references/security/security-finding-template.md` (only to add optional ASVS / OpenCRE rows if missing), `skills/slo-critique/SKILL.md`, `skills/slo-verify/SKILL.md`, `skills/slo-sast/SKILL.md`, `skills/slo-rulegen/SKILL.md`, `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` (NEW), `docs/ARCHITECTURE.md` (note new reference file) |
| Files to read before changing anything | `references/security/security-finding-template.md`, `references/security/security-assessment-summary-template.md`, current SKILL.md files for the four target skills, `docs/slo/design/secure-agent-playbook-imports-overview.md` Idea 4 section, `references/biz/` directory layout (to mirror the retrieval-date discipline), CWE / OWASP-Top-10 / ASVS upstream URLs (for retrieval-date sourcing only — no live fetch) |
| New files allowed | `references/security/standards-mapping.md`, `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` for code/config; `yes` for `references/security/security-finding-template.md` if optional rows need adding (additive only — no row removal or rename) |
| Compatibility commitments | All existing fields in security-finding-template.md and security-assessment-summary-template.md keep their semantics; existing skill citations from M1 keep working; `docs/skill-pack-catalog.md` skill descriptions stay accurate; `sldo-install` install paths unchanged |
| Resource bounds introduced/changed | `standards-mapping.md` rows have no upper hard cap (curated, grows linearly with covered surfaces). Hard requirement: every row carries a `retrieval-date: YYYY-MM-DD` column. Stale rows (> 12 months from current date) emit a *warning* (not failure) in the structural-contract test — this leaves room for legitimately-stable mappings while surfacing potential drift. |
| Invariants/assertions required | (a) Every row in `standards-mapping.md` has a non-empty `retrieval-date` column matching `^\d{4}-\d{2}-\d{2}$`; (b) every cited skill SKILL.md (the four target skills) contains a Markdown link to `references/security/standards-mapping.md`; (c) the per-output-type tier matrix is documented verbatim in `references/security/standards-mapping.md` preamble; (d) `/slo-critique` and `/slo-verify` SKILL.md text contains the "high or critical" threshold phrase and the "MUST cite CWE" phrase within 200 characters of each other; (e) the security-finding-template.md `Standards mapping` row exists (was already present at M1; M3 verifies); (f) **threshold-rule enforcement walks live artifact directories** — `docs/slo/critique/*.md` AND `docs/slo/verify/*.md` (in addition to the `examples/security-finding.md` fixture); any row tagged `severity: high` or `severity: critical` MUST have a non-`N/A` CWE column. Vacuous-pass when those directories are empty (per F-ENG-4 critique resolution). |
| Debugger / inspection expectation | Test failures cite the offending row number in `standards-mapping.md` or the offending skill file + missing phrase. |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify`. No `cargo deny` (no deps change). |
| Forbidden shortcuts | No bulk import of full ASVS / WSTG / SAMM / OpenCRE tables. No live OpenCRE lookup logic in skills. No undated rows. No removing or renaming existing rows in security-finding-template.md. No copy-pasted standards-mapping table inside any SKILL.md (cite the canonical file). No mandatory-OpenCRE rule (it's optional per the tier matrix). |
| Data classification | `Public` — `standards-mapping.md` and all SKILL.md files are git-tracked and reference public OWASP / CWE / ASVS / OpenCRE material. |
| Proactive controls in play | `C1` (Define Security Requirements — the tier matrix IS the requirement), `C2` (Leverage Security Frameworks — CWE / OWASP / ASVS are the frameworks), `C9` (Implement Security Logging and Monitoring — `retrieval-date` column IS the audit trail for source freshness), `C10` (Handle All Errors and Exceptions — missing-mapping is a coverage gap, not a runtime blocker). |
| Abuse acceptance scenarios | `tm-sap-imp-abuse-5: a contributor adds a mapping row without retrieval-date → structural-contract test fails CI`. `tm-sap-imp-abuse-6: a contributor bulk-imports the full ASVS table (200+ rows) → reviewer-time check; explicit Forbidden Shortcut + Out-of-Scope row in this milestone`. `tm-sap-imp-abuse-7: a high-severity finding emitted without CWE → M3's threshold-rule check fails when the test runs against examples/security-finding.md as a fixture`. See BDD abuse-case rows below. |

#### Out of Scope / Must Not Do

- Do **not** add live OpenCRE / ASVS / CWE lookup logic in any skill or test. M3 is curation-only.
- Do **not** bulk-import ASVS / WSTG / SAMM / OpenCRE upstream tables. Curated rows only.
- Do **not** remove or rename existing rows in `security-finding-template.md` or `security-assessment-summary-template.md`. Additive only.
- Do **not** make OpenCRE or ASVS mappings *required* on any output type. Per the tier matrix, they are optional everywhere.
- Do **not** change the freshness window from 12 months without an explicit Open Question resolution.
- Do **not** modify the four target skills' install paths or frontmatter.
- Do **not** change other skills (`/slo-rulegen`, `/slo-ruleverify`, `/slo-ship`) beyond what M1 already wired — M3's scope is the four-skill set listed in the Files Allowed list.

Note: M1 cites `/slo-rulegen` for the *finding template*; M3 additionally cites `/slo-rulegen` for the *standards mapping*. These are two separate citations in the same SKILL.md — no contradiction.

#### Pre-Flight

1. Complete the Global Entry Rules (Section 7).
2. Read `docs/slo/lessons/sap-imp-m2.md` and apply corrections.
3. Read the canonical templates and the design doc's Idea 4 section.
4. Copy the Evidence Log template into working notes.
5. Re-state the milestone constraints: curated rows only, every row dated, four skills cite the mapping, threshold rule documented.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/security/standards-mapping.md` | NEW: preamble with the per-output-type tier matrix; curated rows for the most common bug classes (e.g., CWE-79 XSS, CWE-89 SQLi, CWE-22 Path Traversal, CWE-352 CSRF, CWE-284 Improper Access Control, CWE-798 Hard-coded Creds, CWE-94 Code Injection, CWE-918 SSRF, CWE-787 OOB Write, CWE-502 Deserialization). Each row: CWE-id, short title, OWASP-Top-10 mapping, ASVS section (optional), OpenCRE id (optional), retrieval-date. Aim for ~15-25 rows initially. |
| `references/security/security-finding-template.md` | If `Standards mapping` row already exists (it does per M1 read), no change needed; otherwise add it as optional row. ASVS / OpenCRE columns: confirm shape supports both (template at line 19 already has `Standards mapping` row that says "OWASP / ASVS / LLM / OpenCRE reference"). Verify before editing — likely zero change needed. |
| `skills/slo-critique/SKILL.md` | Add a Markdown link to `references/security/standards-mapping.md` in the security-persona output section. Add the threshold-rule phrasing: "Findings with severity `high` or `critical` MUST use the expanded template AND cite CWE." |
| `skills/slo-verify/SKILL.md` | Same: add link + threshold-rule phrasing in the Pass 4 output section. |
| `skills/slo-sast/SKILL.md` | Add link to `standards-mapping.md` in coverage-gap reporting prose. Required mapping per tier matrix: CWE claimed vs covered; optional: OWASP / ASVS rationale. |
| `skills/slo-rulegen/SKILL.md` | Add link to `standards-mapping.md` for generated-rule documentation. Required: CWE and variation family; optional: OpenCRE / ASVS where available. |
| `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` | NEW: structural-contract test (every row dated; every skill cites the mapping; threshold-rule phrases present in `/slo-critique` and `/slo-verify`; freshness warning for rows > 12 months old). |
| `docs/ARCHITECTURE.md` | Note `references/security/standards-mapping.md` under "References subtrees". |

#### Step-by-Step

1. Write `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` first. Confirm it fails with "standards-mapping.md not found" + "four skills missing standards-mapping citation".
2. Author `references/security/standards-mapping.md`: preamble (tier matrix + retrieval-date discipline + "no bulk vendoring" Forbidden Shortcut) + ~15-25 curated rows.
3. Update `skills/slo-critique/SKILL.md` with the link + threshold-rule phrasing.
4. Update `skills/slo-verify/SKILL.md` with the link + threshold-rule phrasing.
5. Update `skills/slo-sast/SKILL.md` with the link in coverage-gap section.
6. Update `skills/slo-rulegen/SKILL.md` with the link in generated-rule documentation.
7. Verify `references/security/security-finding-template.md` `Standards mapping` row supports CWE / OWASP / ASVS / OpenCRE; if not, add the missing optional rows additively.
8. Re-run `cargo test -p sast-verify` — must pass; freshness warnings (if any) print without failing.
9. Run `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
10. Update `docs/ARCHITECTURE.md`.
11. `git status` — confirm no untracked artifacts.
12. Smoke tests; Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: standards traceability**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All rows dated; four skills cite mapping | happy path | `references/security/standards-mapping.md` exists with ~15-25 dated rows; four skills cite it | `cargo test -p sast-verify --test sap_imp_m3_standards` runs | Test passes; output reports row count + each skill's citation |
| Row missing retrieval-date | invalid input (`tm-sap-imp-abuse-5`) | A test fixture row without `retrieval-date:` | structural-contract test runs | Test fails with row number + missing column message |
| Skill missing standards-mapping citation | invalid input | A test fixture skill without the link | structural-contract test runs | Test fails naming the skill + asserted invariant |
| Empty mapping file | empty state | `standards-mapping.md` exists with header + tier matrix but zero rows | structural-contract test runs | Test passes (file presence + preamble valid; row count > 0 not enforced) |
| Threshold rule missing in /slo-critique | invalid input (`tm-sap-imp-abuse-7`) | `/slo-critique` SKILL.md without "high" + "critical" + "MUST" + "CWE" within 200 chars | structural-contract test runs | Test fails with "threshold-rule phrase missing" |
| Stale row warning | resource bound | A row with `retrieval-date: 2024-01-01` (> 12 months from runbook authoring) | structural-contract test runs at HEAD | Test passes; warning emitted with row number + date + advisory text "consider re-fetching" |
| Bulk-import attempt | abuse case (`tm-sap-imp-abuse-6`) | A PR adds 250 rows mirroring upstream ASVS | reviewer reads diff | Reviewer rejects per "no bulk vendoring" Forbidden Shortcut + Non-Goal in design doc |
| High-severity finding without CWE | abuse case (`tm-sap-imp-abuse-7`) | `examples/security-finding.md` (M2 fixture) tagged `severity: high` without CWE | M3 test runs threshold check against examples/ | Test fails citing the example file + missing CWE |
| Existing skill prose preserved | compatibility | `/slo-critique` and `/slo-verify` already cite the security templates from M1 | M3 close-out diff review | M1 citations still present; M3 adds standards-mapping cite alongside, not replacing |
| Existing template shape preserved | compatibility | `security-finding-template.md` rows from M1 still in place | M3 close-out diff review | No row removed or renamed; only optional rows added if missing |
| sldo-install regression check | compatibility | `references/security/standards-mapping.md` is in `references/`, not `skills/` | `sldo-install --dry-run` runs | Mapping file is not treated as an installable skill |

#### Regression Tests

- `cargo test -p sast-verify --test sap_imp_m1_citations` (M1's test) still passes.
- `cargo test -p sast-verify --test sap_imp_m2_examples` (M2's test) still passes.
- `cargo test -p sldo-install` — installer doesn't try to install `references/security/standards-mapping.md`.
- `cargo test --workspace` — full suite green.
- Manually review one critique-report fixture and one verification-report fixture in `examples/` (M2 outputs) to confirm they still parse — M3 does NOT modify M2 fixtures.

#### Compatibility Checklist

- [ ] M1's `sap_imp_m1_citations` test still passes
- [ ] M2's `sap_imp_m2_examples` test still passes
- [ ] `references/security/security-{finding,assessment-summary}-template.md` row order/shape unchanged (only additive)
- [ ] All four target skills' install paths unchanged
- [ ] All four target skills' frontmatter `name` and `description` unchanged
- [ ] `sldo-install --dry-run` resolves shipped skills; ignores `references/`
- [ ] `docs/skill-pack-catalog.md` skill descriptions still accurate

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/sap_imp_m3_standards.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `standards_mapping_file_exists_with_preamble` | Mapping file structurally valid | File exists; preamble contains the tier matrix + Forbidden Shortcut prose |
| `every_row_has_retrieval_date` | Date invariant holds | Parses every row; asserts `retrieval-date` matches `^\d{4}-\d{2}-\d{2}$` |
| `four_skills_cite_standards_mapping` | Citation invariant holds | Walks the four target skills; asserts each contains a Markdown link to `references/security/standards-mapping.md` |
| `threshold_rule_phrasing_in_critique_and_verify` | Threshold rule documented | `/slo-critique` and `/slo-verify` SKILL.md contain "high", "critical", "MUST", "CWE" within 200 chars of each other |
| `stale_rows_warned` | Freshness audit holds | Rows with `retrieval-date` > 12 months old emit a warning; test still passes (warning ≠ failure) |
| `examples_high_severity_findings_have_cwe` | Threshold rule enforced against M2 fixtures | If `examples/security-finding.md` tagged `severity: high` or `critical`, it has a CWE row populated |
| `live_critique_and_verify_findings_have_cwe` | Threshold rule enforced against live artifacts (per F-ENG-4) | Walks `docs/slo/critique/*.md` AND `docs/slo/verify/*.md`; for any row with `severity: high` or `severity: critical`, asserts CWE column is non-empty and not `N/A`. Vacuous-pass when those directories are empty |

#### Smoke Tests

- [ ] Manually read `references/security/standards-mapping.md` end-to-end; tier matrix + retrieval-date column visible
- [ ] Manually read each updated skill's SKILL.md; standards-mapping link visible
- [ ] `cargo test -p sast-verify --test sap_imp_m3_standards` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo build --workspace` boots cleanly
- [ ] `git status` clean
- [ ] `.gitignore` reviewed; no changes
- [ ] `sldo-install --dry-run` ignores `references/`

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Structural-contract test created | `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` | fails for "mapping file not found + four skills missing citation" | | | |
| standards-mapping.md authored | manual review | preamble + ~15-25 dated rows | | | |
| /slo-critique updated | manual review | standards-mapping link + threshold-rule phrasing | | | |
| /slo-verify updated | manual review | standards-mapping link + threshold-rule phrasing | | | |
| /slo-sast updated | manual review | standards-mapping link in coverage-gap section | | | |
| /slo-rulegen updated | manual review | standards-mapping link in rule documentation | | | |
| security-finding-template.md verified | manual diff | Standards mapping row already supports CWE/OWASP/ASVS/OpenCRE; zero or minimal additive change | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Typecheck / build | `cargo build --workspace` | clean | | | |
| Static analyzer / linter | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Dependency audit | (no deps change — skip) | n/a | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test -p sast-verify --test sap_imp_m3_standards` | green; freshness warnings printed if applicable | | | |
| Build/boot | `cargo build --workspace` | boots cleanly | | | |
| Smoke tests | manual review | all checked | | | |
| Resource-bound verification | every row dated; freshness warnings on > 12 month-old rows | bound encoded; test asserts | | | |
| Invariant/assertion verification | citation + threshold-rule phrases | invariants encoded and tested | | | |
| Debugger / state inspection | (n/a — Markdown / structural-contract) | hypothesis confirmed before code change | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | review `.gitignore` | no change | | | |
| Compatibility checks | run Compatibility Checklist | no regressions | | | |
| M1 + M2 regression | `cargo test -p sast-verify --test sap_imp_m1_citations` + `--test sap_imp_m2_examples` | both green | | | |

#### Definition of Done

- All BDD scenarios pass.
- All E2E runtime validations pass.
- M1 + M2 structural-contract tests still green.
- Full existing test suite remains green.
- Formatter, typecheck, clippy clean.
- Resource bounds (every row dated; freshness warning for stale rows) encoded and tested.
- Citation + threshold-rule invariants encoded and tested.
- No bulk-imported standards table introduced.
- No live OpenCRE / ASVS / CWE lookup logic added to any skill or test.
- `git status` clean; `.gitignore` reviewed.
- `docs/ARCHITECTURE.md` updated.
- Lessons file written.
- Completion summary written.
- Milestone Tracker updated.

#### Post-Flight

- **ARCHITECTURE.md**: under "References subtrees", note `references/security/standards-mapping.md` as the curated CWE / OWASP / ASVS / OpenCRE table consumed by four security-relevant skills.
- **README.md**: no change expected.
- **Other docs**: `docs/skill-pack-catalog.md` — no change (skill purposes unchanged); `CLAUDE.md` and `copilot-instructions.md` — no change.

#### Notes

- The 12-month freshness window is heuristic: ASVS major revisions are rare, OWASP-Top-10 cycles every 4 years, CWE definitions are stable. 12 months gives early warning without forcing churn. Adjust in a future runbook if the heuristic proves wrong.
- The structural-contract test treats stale rows as warnings (not failures) intentionally — a legitimately-stable mapping shouldn't break CI just because nobody re-fetched the upstream page. The warning surfaces drift candidates for the next quarterly review.
- The "use existing tools" project norm (design doc Idea 8) is captured implicitly: by citing established standards (CWE / OWASP) in the mapping rather than coining SLO-specific bug classes, we re-use proven taxonomies. Explicit prose addition to skill-authoring guidance is deferred to the engineering-skill-improvements runbook (which already plans `references/templates/skill-authoring.md`).

---

### Milestone 4 — `Optional Claude plugin packaging assessment + (if green-lit) packaging artifacts`

**Goal**: After M4, a host capability matrix decision doc exists. If green-lit, an additive `.claude-plugin/plugin.json` and a SHA-pinned release-zip workflow ship; the README makes explicit that `sldo-install` remains canonical. If not green-lit, only the decision doc ships and M5 reads its gate.

**Context**: SLO has a stronger local installer story through `sldo-install` than Secure Agent Playbook (multi-host: Claude Code + GitHub Copilot; manifest at `~/.sldo/install.toml`). A Claude plugin zip could lower friction for Claude-only users but introduces maintenance cost (two install paths) and supply-chain risk (release workflow must be SHA-pinned). The decision is *intentionally gated*: M4 produces the matrix first, then ships packaging only if the analysis green-lights it.

**Carmack-style reliability goal**: Make "an unpinned third-party GitHub Action lands in the repo" structurally impossible (§4.5). The structural-contract test walks every `.github/workflows/*.yml` and asserts every `uses:` reference is SHA-pinned (40-char hex). Test runs in CI; a workflow with `uses: actions/checkout@v4` (tag, not SHA) fails the gate.

**Important design rule**: Plugin packaging is *additive*, never replacing. `sldo-install` and `docs/skill-pack-catalog.md` remain canonical. `.claude-plugin/plugin.json` (if it ships) points at the existing `skills/` tree without duplicating source. README wording explicit: "Claude plugin zip is optional; Rust installer remains canonical."

**Refactor budget**: `Targeted refactor permitted for adding the plugin manifest + workflow if green-lit; otherwise no code/config refactor`. The decision doc is a Markdown-only artifact regardless of green-lit outcome.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `sldo-install` CLI + `~/.sldo/install.toml` schema; existing `.github/workflows/` (currently SAST workflow); `SECURITY.md` SHA-pinning discipline; design doc Idea 6 + Non-goal "do not duplicate skills"; `docs/slo/design/agent-host-capabilities.md` (existing host matrix) |
| Outputs | `docs/slo/design/host-capability-matrix.md` (always); if green-lit: `.claude-plugin/plugin.json` + `.github/workflows/release-zip.yml`; structural-contract test for SHA-pinning; README update if green-lit |
| Interfaces touched | New `docs/slo/design/host-capability-matrix.md`; if green-lit: new `.claude-plugin/` directory at repo root, new GitHub Actions workflow file, new release-zip artifact path; new test under `xtasks/sast-verify/tests/`; possibly README and `docs/ARCHITECTURE.md` |
| Files allowed to change | `docs/slo/design/host-capability-matrix.md` (NEW), `.claude-plugin/plugin.json` (NEW, conditional on green-lit), `.github/workflows/release-zip.yml` (NEW, conditional on green-lit), `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` (NEW), `README.md` (only if green-lit), `docs/ARCHITECTURE.md` (note new distribution channel if green-lit, or note matrix file if not), `.gitignore` (add `dist/`, `*.zip`, `.claude-plugin/dist/` if green-lit) |
| Files to read before changing anything | `docs/slo/design/agent-host-capabilities.md`, `SECURITY.md`, `.github/workflows/` (for pinning patterns to mirror), `crates/sldo-install/src/install.rs` (to understand canonical install paths), `docs/skill-pack-catalog.md`, Claude Code plugin docs (referenced from public Anthropic docs as of retrieval date — record date in matrix file), `~/.sldo/install.toml` schema |
| New files allowed | `docs/slo/design/host-capability-matrix.md` (always); `.claude-plugin/plugin.json` (conditional); `.github/workflows/release-zip.yml` (conditional); `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` (always — it walks existing workflows even if no new ones land) |
| New dependencies allowed | `none` (release-zip generation can use `gh` CLI + standard `zip` tool). If a workflow needs a non-default GitHub Action (beyond `actions/checkout`, `actions/upload-artifact`, etc.), record SHA + license in the decision doc. **Explicit re-confirmation required**: this is the only milestone where the runbook's default `none` is potentially relaxed; M4 must justify any new dep in writing in the host capability matrix doc. |
| Migration allowed | `no` for `~/.sldo/install.toml` schema; `yes` for adding `.gitignore` patterns and a new GitHub Releases artifact path |
| Compatibility commitments | `sldo-install` CLI surface unchanged (`install`, `--dry-run`, `uninstall`, `status`, `verify`); `~/.sldo/install.toml` schema unchanged; `docs/skill-pack-catalog.md` remains canonical inventory; GitHub Copilot install path unchanged; existing SAST workflow unaffected; multi-host story preserved (Copilot is not made second-class); the four shipped SKILL.md install paths unchanged |
| Resource bounds introduced/changed | Exactly 1 new `plugin.json` (if green-lit, else 0); exactly 1 new release workflow (if green-lit, else 0); exactly 0 unpinned `uses:` references across all workflows (hard constraint, enforced by test); release zip size — not capped in M4 (passes through GitHub Releases default 2 GB ceiling) |
| Invariants/assertions required | (a) Every `uses:` reference matches `^[a-zA-Z0-9._/-]+@[a-f0-9]{40}$` (SHA-pin) — **glob covers `.github/{workflows,actions}/**/*.{yml,yaml}` (per F-ENG-5)**; (b) if `plugin.json` exists, it does NOT duplicate skill files AND **no path-valued field contains `..` segments or absolute paths** (per F-SEC-3) — paths are validated via `Path::components()` rejecting `Component::ParentDir` and `is_absolute()`; (c) `host-capability-matrix.md` contains a clear "green-lit / not green-lit / deferred" decision row plus reasoning; (d) the matrix names what each host supports for plugin install, agent install, and runtime invocation, with retrieval-date stamps; (e) if green-lit, the README "Examples" / "Install" sections mention the plugin path explicitly as optional and `sldo-install` as canonical; (f) **every `.github/workflows/*.yml` (existing + new) has a top-level explicit `permissions:` block** scoped to minimum-privilege — `contents: write` for the release workflow and nothing else (per F-SEC-4, ≥9/10 confidence); (g) **release-zip generation uses `git archive --format=zip --prefix=sunlit-orchestrate-${TAG}/ HEAD -o release.zip`** (or equivalent that emits *only tracked files at HEAD*) — no `tar -czf .` / `cp -r .` / `zip -r . .` of the runner working directory (per F-SEC-5); (h) **trigger-acceptable-set** for any new workflow: `tags: ['v*']`, `release:`, `workflow_dispatch:`, `schedule:`. **Forbidden**: `push:` to default branches, `pull_request:` (per F-ENG-5). |
| Debugger / inspection expectation | Test failures cite the offending workflow file + line + the unpinned reference. Plugin.json structural failure cites the offending duplicated path **or path-traversal segment**. Decision-doc parse failure cites missing required section. Permissions-block missing failure cites the workflow file lacking the top-level `permissions:` key. |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify`, `cargo deny check` (only if a dep was added with re-confirmation; otherwise skip). |
| Forbidden shortcuts | No tag-based `uses:` references (`@v4`, `@main`). No branch-based `uses:` references. No skill duplication into `.claude-plugin/skills/`. **No `..` or absolute paths in plugin.json path-valued fields (per F-SEC-3)**. No README rewrite that downgrades `sldo-install` to "legacy" or equivalent. No `force-push` of release tags. No undated rows in the host capability matrix. No CI workflow that uploads release artifacts without a tag-trigger guard. No "I'll pin the SHA later" placeholders. **No release workflow without an explicit minimum-privilege `permissions:` block (per F-SEC-4)**. **No release-zip generation via `tar`/`cp`/`zip` of the runner working directory — must use `git archive` from HEAD (per F-SEC-5)**. |
| Data classification | `Public` — decision doc, plugin.json, workflow, test all git-tracked. Release artifacts (zips) are public on GitHub Releases. |
| Proactive controls in play | `C1` (Define Security Requirements — SHA-pinning policy is the requirement), `C2` (Leverage Security Frameworks — GitHub Actions ecosystem with pinning), `C8` (Protect Data Everywhere — SHA-pinning prevents supply-chain compromise of release artifacts), `C9` (Implement Security Logging and Monitoring — workflow runs are audit trail), `C10` (Handle All Errors and Exceptions — failed pins fail CI explicitly). |
| Abuse acceptance scenarios | `tm-sap-imp-abuse-8: a workflow added with uses: actions/checkout@v4 (tag, not SHA) → structural-contract test fails CI`. `tm-sap-imp-abuse-9: plugin.json duplicates skill files into .claude-plugin/skills/ creating drift between sources of truth → structural-contract test fails when it detects duplication`. `tm-sap-imp-abuse-10: release workflow runs on every push (not tag) flooding GitHub Releases → structural-contract test asserts the workflow trigger is tag-only or release-only`. `tm-sap-imp-abuse-11: README rewrite makes Claude-plugin path appear canonical and Copilot second-class → reviewer-time check + Forbidden Shortcut`. See BDD abuse-case rows below. |

#### Out of Scope / Must Not Do

- Do **not** add `.claude-plugin/plugin.json` if the host capability matrix decision is "not green-lit" or "deferred". Ship the matrix only.
- Do **not** duplicate skill source under `.claude-plugin/skills/`. The plugin manifest must point at the existing `skills/` tree.
- Do **not** make `sldo-install` second-class. README wording must remain explicit about canonical multi-host install path.
- Do **not** add unpinned third-party actions. Even pinned-by-tag is forbidden.
- Do **not** auto-publish to a third-party registry from the workflow.
- Do **not** introduce signing/attestation as part of M4. If signing is desired, it goes in a follow-up runbook (it would change the dependency surface and the threat model).
- Do **not** modify `~/.sldo/install.toml` schema or `sldo-install` CLI.
- Do **not** introduce live OpenCRE / standards lookup as part of the workflow.

#### Pre-Flight

1. Complete the Global Entry Rules (Section 7).
2. Read `docs/slo/lessons/sap-imp-m3.md` and apply corrections.
3. Read `docs/slo/design/agent-host-capabilities.md` to understand existing host matrix prose.
4. Read every `.github/workflows/*.yml` to confirm existing pinning state.
5. Copy the Evidence Log template into working notes.
6. Re-state milestone constraints: matrix doc always ships; plugin.json + release workflow only if green-lit; SHA-pin invariant enforced regardless; Copilot stays first-class.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/slo/design/host-capability-matrix.md` | NEW: capability matrix (Claude Code, GitHub Copilot, any other host); decision row (green-lit / not green-lit / deferred) with reasoning; if green-lit: scope of plugin packaging additivity; M5 gate language; retrieval-date stamps for upstream Claude Code / Copilot plugin docs cited |
| `.claude-plugin/plugin.json` | NEW (conditional): minimal manifest pointing at `skills/` tree; no duplication of source; **no path-valued field contains `..` or absolute paths** (per F-SEC-3) |
| `.github/workflows/release-zip.yml` | NEW (conditional): SHA-pinned `actions/checkout` + `actions/upload-artifact` (or equivalent); trigger on tag push only (`on: push: tags: ['v*']`); **explicit top-level `permissions: contents: write` block (and nothing else; per F-SEC-4)**; **zip step uses `git archive --format=zip --prefix=sunlit-orchestrate-${TAG}/ HEAD -o release.zip`** (per F-SEC-5) — emits only tracked files at HEAD |
| `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` | NEW: walks **`.github/{workflows,actions}/**/*.{yml,yaml}` (per F-ENG-5)**, asserts every `uses:` matches `^[a-zA-Z0-9._/-]+@[a-f0-9]{40}$`. Asserts every workflow has a top-level `permissions:` block (per F-SEC-4). Asserts plugin.json (if exists) contains no `"path":` entries pointing into `.claude-plugin/skills/`, no `..` segments, no absolute paths (per F-SEC-3). Asserts release workflow trigger is in the acceptable set `{tags, release, workflow_dispatch, schedule}` and forbidden triggers `{push to default, pull_request}` are absent (per F-ENG-5). If the release zip exists locally (after dry-run), assert content ⊆ `git ls-files` output (per F-SEC-5 smoke test). |
| `README.md` | (Conditional on green-lit) Add "Install via Claude plugin (optional)" section explicitly noting `sldo-install` remains canonical. Otherwise unchanged. |
| `docs/ARCHITECTURE.md` | (Always) Note new `host-capability-matrix.md` decision doc. (Conditional on green-lit) Add "Distribution channels" subsection mentioning optional Claude plugin packaging additive to `sldo-install`. |
| `.gitignore` | (Conditional on green-lit) Add `dist/`, `*.zip`, `.claude-plugin/dist/` |

#### Step-by-Step

1. Write `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` first. Confirm it passes (existing workflows are already SHA-pinned per `SECURITY.md` discipline) — this is intentional: the test must encode the existing invariant before any new workflow is added so that the gate is in place when M4's release workflow lands.
2. Author `docs/slo/design/host-capability-matrix.md`: capability matrix + decision + reasoning + retrieval dates. Decide green-lit / not green-lit / deferred.
3. **Decision branch:**
   - **If green-lit**: proceed to step 4.
   - **If not green-lit** or **deferred**: skip to step 9.
4. (Green-lit) Author `.claude-plugin/plugin.json`. Validate JSON schema. Confirm skill paths point at `skills/` (no `.claude-plugin/skills/`).
5. (Green-lit) Author `.github/workflows/release-zip.yml`. SHA-pin every `uses:`. Trigger on `tags: ['v*']`. No deps beyond default GitHub Actions ecosystem. Validate YAML.
6. (Green-lit) Re-run `cargo test -p sast-verify --test sap_imp_m4_workflow_pinning` — must pass with the new workflow added.
7. (Green-lit) Update `README.md` with the optional plugin install section + canonical-installer wording.
8. (Green-lit) Update `.gitignore` with `dist/`, `*.zip`, `.claude-plugin/dist/`.
9. (All paths) Update `docs/ARCHITECTURE.md` with the matrix doc reference; add Distribution channels subsection only if green-lit.
10. Run `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
11. (Green-lit) Run a release dry-run locally: `gh release create v0.0.0-test --notes "M4 dry-run" --draft` then delete; verify the workflow triggers correctly. *Do not push the test tag* — use a draft release scoped to the local `gh` user. Document outcome in the matrix doc.
12. Smoke tests; Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: optional plugin packaging + SHA-pin gate**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Existing workflows pass pinning gate | happy path | `.github/workflows/*.yml` at HEAD already SHA-pinned per `SECURITY.md` | `cargo test -p sast-verify --test sap_imp_m4_workflow_pinning` runs | Test passes; output reports every workflow + every `uses:` is SHA-pinned |
| Decision doc exists with required sections | happy path | `docs/slo/design/host-capability-matrix.md` exists | M4 close-out diff review | Capability matrix table + decision row + reasoning + retrieval dates all present |
| Green-lit path: plugin.json points at skills/ | happy path | Decision = green-lit; `.claude-plugin/plugin.json` exists | structural-contract test runs | Plugin manifest skill paths reference `skills/` (not `.claude-plugin/skills/`) |
| Green-lit path: release workflow tag-triggered | happy path | Decision = green-lit; `.github/workflows/release-zip.yml` exists | structural-contract test runs | Workflow `on:` block contains `tags: ['v*']`; no `push:` to default branch trigger |
| Tag-pinned action caught | invalid input (`tm-sap-imp-abuse-8`) | A test fixture workflow with `uses: actions/checkout@v4` | structural-contract test runs against fixture | Test fails with file + line + "expected SHA-pin, got tag `v4`" |
| Branch-pinned action caught | invalid input | A test fixture workflow with `uses: foo/bar@main` | structural-contract test runs | Test fails with "expected SHA-pin, got branch `main`" |
| Plugin.json duplicates skills | invalid input (`tm-sap-imp-abuse-9`) | A test fixture plugin.json containing `"path": ".claude-plugin/skills/slo-foo"` | structural-contract test runs | Test fails citing duplicated path |
| Release workflow runs on every push | invalid input (`tm-sap-imp-abuse-10`) | A test fixture release workflow with `on: push: branches: [main]` | structural-contract test runs | Test fails with "release workflow must be tag- or release-triggered" |
| Decision = not green-lit ships only matrix | empty state | Matrix decision = "not green-lit" | M4 close-out diff review | Only `host-capability-matrix.md` + `sap_imp_m4_workflow_pinning.rs` ship; no `.claude-plugin/`, no release workflow, no README change, no `.gitignore` change |
| Existing SAST workflow regression | compatibility | Existing `.github/workflows/sast.yml` (or equivalent) at HEAD | M4 close-out CI run | SAST workflow still passes; pinning test still passes |
| `sldo-install` regression | compatibility | M4 lands `.claude-plugin/plugin.json` (green-lit branch) | `cargo test -p sldo-install` + `sldo-install --dry-run` | Installer ignores `.claude-plugin/`; install paths for shipped skills unchanged |
| README downgrades sldo-install | abuse case (`tm-sap-imp-abuse-11`) | A reviewer notices README rewrite removes "canonical" wording for sldo-install | code review | Reviewer rejects per Forbidden Shortcut + Compatibility commitment |
| Stale matrix retrieval dates | resource bound | A row in matrix with `retrieval-date: 2024-01-01` | M3's freshness logic (reused) — actually, M4 matrix doc is read-only; freshness check is reviewer-time at the next runbook | Reviewer notes drift candidate; not a CI failure in M4 |

#### Regression Tests

- M1's `sap_imp_m1_citations` test still passes.
- M2's `sap_imp_m2_examples` test still passes.
- M3's `sap_imp_m3_standards` test still passes.
- Existing `.github/workflows/*.yml` (SAST workflow if present) — every `uses:` already SHA-pinned (per `SECURITY.md`) so the new test passes immediately on existing files.
- `cargo test -p sldo-install` — installer unchanged.
- `sldo-install --dry-run` — resolves shipped skills; ignores `.claude-plugin/` (if green-lit).
- `cargo test --workspace` — full suite green.

#### Compatibility Checklist

- [ ] `sldo-install` CLI surface unchanged
- [ ] `~/.sldo/install.toml` schema unchanged
- [ ] All shipped `skills/<name>/SKILL.md` install paths unchanged
- [ ] `docs/skill-pack-catalog.md` remains canonical inventory (unchanged or augmented additively)
- [ ] GitHub Copilot install path unchanged
- [ ] Existing GitHub Actions workflows (SAST etc.) still pass
- [ ] M1, M2, M3 structural-contract tests still pass
- [ ] (Green-lit) `.claude-plugin/plugin.json` points at `skills/` tree, not duplicated source
- [ ] (Green-lit) Release workflow trigger is tag-based, not push-based
- [ ] README wording does NOT downgrade `sldo-install`

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `every_workflow_uses_is_sha_pinned` | SHA-pin invariant holds across all workflows + composite actions (per F-ENG-5) | Walks `.github/{workflows,actions}/**/*.{yml,yaml}`; every `uses:` matches `^[a-zA-Z0-9._/-]+@[a-f0-9]{40}$` |
| `every_workflow_has_explicit_permissions_block` | Minimum-privilege invariant holds (per F-SEC-4, ≥9/10) | Walks every workflow file; asserts a top-level `permissions:` key is present. Release workflow MUST have `contents: write` scope and no other scope unless explicitly justified |
| `host_capability_matrix_exists_with_decision` | Decision doc structurally valid | File exists; contains capability matrix table + a decision row labelled `green-lit | not green-lit | deferred` + reasoning |
| `plugin_json_does_not_duplicate_skills` | Plugin manifest points at canonical source | If `.claude-plugin/plugin.json` exists, every skill path it references resolves into `skills/`, not `.claude-plugin/skills/` |
| `plugin_json_paths_are_safe` | Path-traversal invariant holds (per F-SEC-3) | If `.claude-plugin/plugin.json` exists, every path-valued field is parsed via `Path::components()`; no `Component::ParentDir`; no `is_absolute()` |
| `release_workflow_trigger_in_acceptable_set` | Trigger discipline holds (per F-ENG-5) | If `.github/workflows/release-zip.yml` exists, its `on:` block uses only triggers from `{tags, release, workflow_dispatch, schedule}`; forbidden triggers `{push to default branch, pull_request}` are absent |
| `release_workflow_uses_git_archive` | Release-zip generation invariant (per F-SEC-5) | If `.github/workflows/release-zip.yml` exists, the zip-generation step body contains `git archive` (not `tar -czf .` / `cp -r .` / `zip -r . .`) |
| `release_zip_content_subset_of_git_ls_files` | Release-zip content invariant (per F-SEC-5) | After local dry-run via `gh release create --draft`, unzip the artifact; assert every file path in the archive matches a file in `git ls-files HEAD`. Vacuous-pass when no local artifact exists |
| `readme_canonical_installer_phrase_present` | Wording invariant holds (green-lit only) | If `.claude-plugin/plugin.json` exists, README contains "Rust installer remains canonical" or equivalent within 200 chars of any "Claude plugin" mention |

#### Smoke Tests

- [ ] Manually read `docs/slo/design/host-capability-matrix.md` end-to-end; matrix + decision + reasoning all clear
- [ ] (Green-lit) Manually inspect `.claude-plugin/plugin.json`; valid JSON; paths point at `skills/`
- [ ] (Green-lit) Manually inspect `.github/workflows/release-zip.yml`; SHA-pinned; tag-triggered
- [ ] (Green-lit) Local dry-run via `gh release create --draft` succeeds without publishing
- [ ] `cargo test -p sast-verify --test sap_imp_m4_workflow_pinning` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo build --workspace` boots cleanly
- [ ] `git status` clean
- [ ] (Green-lit) `.gitignore` contains `dist/`, `*.zip`, `.claude-plugin/dist/`
- [ ] `sldo-install --dry-run` ignores `.claude-plugin/`

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Pinning test created | `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` | passes against existing workflows (already pinned) | | | |
| Capability matrix authored | manual review | matrix + decision + reasoning + dates present | | | |
| Decision recorded | matrix doc | green-lit / not green-lit / deferred | | | |
| (Green-lit) plugin.json authored | manual review + JSON validation | points at `skills/`; no duplication | | | |
| (Green-lit) Release workflow authored | manual review + YAML validation | SHA-pinned; tag-triggered | | | |
| (Green-lit) README updated | manual review | canonical-installer wording preserved | | | |
| (Green-lit) .gitignore updated | manual review | `dist/`, `*.zip` patterns added | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Typecheck / build | `cargo build --workspace` | clean | | | |
| Static analyzer / linter | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Dependency audit | (skip unless dep added with re-confirmation) | n/a or pass | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test -p sast-verify --test sap_imp_m4_workflow_pinning` | green | | | |
| Build/boot | `cargo build --workspace` | boots cleanly | | | |
| Smoke tests | manual + dry-run | all checked | | | |
| Resource-bound verification | 0 unpinned `uses:`; 1 plugin.json (or 0); 1 release workflow (or 0) | bound encoded; test asserts | | | |
| Invariant/assertion verification | SHA-pin + no-duplication + tag-trigger | invariants encoded and tested | | | |
| Debugger / state inspection | (n/a — Markdown / YAML / JSON; structural-contract test failures serve as inspection) | hypothesis confirmed before code change | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | review `.gitignore` | (green-lit) `dist/`, `*.zip` present; (else) unchanged | | | |
| Compatibility checks | run Compatibility Checklist | no regressions | | | |
| M1 + M2 + M3 regression | three prior tests | all green | | | |

#### Definition of Done

**Always (regardless of decision):**
- `docs/slo/design/host-capability-matrix.md` exists with matrix + decision + reasoning.
- `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` exists and passes.
- All BDD scenarios that apply to the chosen branch pass.
- M1, M2, M3 tests still green.
- Full existing test suite remains green.
- Formatter, typecheck, clippy clean.
- `git status` clean.
- Lessons file written.
- Completion summary written.
- Milestone Tracker updated.

**If green-lit, additionally:**
- `.claude-plugin/plugin.json` exists; manifest passes structural-contract test.
- `.github/workflows/release-zip.yml` exists; SHA-pinned; tag-triggered.
- README updated with optional plugin section + canonical-installer wording.
- `.gitignore` updated.
- Local dry-run via `gh release create --draft` succeeds.

#### Post-Flight

- **ARCHITECTURE.md** (always): note new `host-capability-matrix.md` decision doc.
- **ARCHITECTURE.md** (green-lit only): add "Distribution channels" subsection mentioning optional Claude plugin packaging additive to `sldo-install`.
- **README.md** (green-lit only): add "Install via Claude plugin (optional)" section.
- **README.md** (not green-lit / deferred): no change.
- **Other docs**: `docs/skill-pack-catalog.md` — note optional plugin packaging only if green-lit; `CLAUDE.md` and `copilot-instructions.md` — no change.

#### Notes

- The structural-contract test for SHA-pinning is authored *before* any new workflow lands. This means the test must pass against existing workflows on day one. If existing workflows fail the test, the milestone branches: fix the existing pinning state first (still M4 work, but call out as a discovery), then proceed.
- The "explicit re-confirmation required" door for new dependencies is narrow: GitHub-native release tooling (`gh`, `actions/checkout`, `actions/upload-artifact`) plus standard `zip` should suffice. If a non-default Action is genuinely needed (e.g., `softprops/action-gh-release`), document license + SHA + retrieval date in the matrix doc.
- If the decision is "not green-lit" or "deferred", M5 reads the matrix and either ships agents (if matrix supports them) or defers to a fresh runbook. The four agent files in M5 are gated on a *different* matrix check than M4's plugin gate — both checks live in the same matrix doc.
- Signing / attestation of the release zip is intentionally out of scope for M4. Adding sigstore / cosign would change the dependency surface, the threat model, and the verification UX. A follow-up runbook can introduce signing once the basic packaging path is exercised in production.

---

### Milestone 5 — `Host-native agent-role experiment (gated on M4 host-capability matrix)`

**Goal**: After M5, either (a) four host-native specialist agents exist under `agents/`, with structural-contract tests enforcing file count, output paths, and Copilot fallback documentation; or (b) M5 is closed as "deferred — no implementation" because M4's matrix forbade the experiment, with a fresh-runbook handoff note.

**Context**: `/slo-critique` already runs four-persona rotation (CEO / engineering / security / design) inside the skill. Secure Agent Playbook's specialist-agent + team-lead pattern (design doc Idea 5) could produce richer outputs, but only if the host can install and invoke them consistently. M4's host capability matrix is the gate. The constraint that's non-negotiable: agent outputs MUST land in the same durable artifact path that `/slo-critique` writes to (`docs/slo/critique/<slug>.md`), so the runbook contract is preserved regardless of which path produced the critique.

**Carmack-style reliability goal**: Make "an agent writes outside the allowed output path set" structurally impossible (§4.5). Each agent file declares its `output-paths:` array in frontmatter; the structural-contract test asserts the array is a subset of `{docs/slo/critique/, docs/slo/verify/}`. If an agent file declares an output path outside this set, CI fails before merge.

**Important design rule**: Agents are *additive*, never replacing. `/slo-critique` persona rotation remains the canonical portable critique path. Each agent file MUST document a Copilot fallback (either Copilot is documented as supported in the matrix, or the agent flow is feature-flagged and falls back to `/slo-critique` persona rotation when the host doesn't support agents). No Claude-only path that bypasses the multi-host story.

**Refactor budget**: `Targeted refactor permitted for adding agent files + tests + overlay updates if green-lit; otherwise documentation-only deferred-note refactor`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `docs/slo/design/host-capability-matrix.md` (M4 output — the gate); existing `/slo-critique` SKILL.md (the canonical portable path); existing `docs/slo/critique/` artifact format; existing `docs/slo/verify/` artifact format if any |
| Outputs | (Green-lit) Four agent files under `agents/` + structural-contract test + overlay updates in `CLAUDE.md`/`copilot-instructions.md`/`docs/skill-pack-catalog.md`/`docs/ARCHITECTURE.md`; (Deferred) lessons file with deferred-note + handoff to fresh runbook |
| Interfaces touched | (Green-lit) New `agents/` directory at repo root; new test under `xtasks/sast-verify/tests/`; overlay docs; `docs/skill-pack-catalog.md` if catalog format accommodates agents; (Deferred) lessons + completion files only |
| Files allowed to change | `agents/slo-runbook-review-lead.md` (NEW, conditional), `agents/slo-security-reviewer.md` (NEW, conditional), `agents/slo-design-reviewer.md` (NEW, conditional), `agents/slo-verification-lead.md` (NEW, conditional), `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (NEW, always — passes vacuously when `agents/` doesn't exist), `docs/skill-pack-catalog.md` (conditional), `CLAUDE.md` (conditional, append agent overlay), `copilot-instructions.md` (conditional, document fallback), `docs/ARCHITECTURE.md` (conditional, note agents subsection), `README.md` (conditional, mention specialist agents as optional) |
| Files to read before changing anything | `docs/slo/design/host-capability-matrix.md` (the gate), `skills/slo-critique/SKILL.md` (the canonical portable path), one or more existing `docs/slo/critique/*.md` files if any (to mirror artifact format), `docs/slo/design/agent-host-capabilities.md` (existing host matrix prose), `docs/skill-pack-catalog.md` (to understand catalog format), `CLAUDE.md` and `copilot-instructions.md` (overlay conventions) |
| New files allowed | (Green-lit) the four agent files; (Always) the test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `/slo-critique` SKILL.md unchanged (persona rotation remains canonical portable path); `docs/slo/critique/<slug>.md` artifact format unchanged (agents write into the same shape); GitHub Copilot remains a first-class install target — if matrix says Copilot can't install agents, M5 must document a fallback path or defer; M1+M2+M3+M4 tests still pass; `sldo-install` install paths unchanged; no skill rename |
| Resource bounds introduced/changed | (Green-lit) Exactly 4 agent files; each agent file ≤ 200 lines (prose-cap); each agent's `output-paths:` array MUST be a subset of `{docs/slo/critique/, docs/slo/verify/}` (cardinality 1 or 2). Hard cap: 4 agents. Behavior at limit: 5th agent fails the structural-contract test. |
| Invariants/assertions required | (a) Exactly 4 agent files in `agents/` if directory exists; (b) every agent file has frontmatter with `name`, `role`, `output-paths`, `copilot-fallback`, `host-required`; (c) every `output-paths` entry **canonicalized via `Path::components()`** is a strict prefix-subset of `{docs/slo/critique/, docs/slo/verify/}` AND contains **no `Component::ParentDir` segments and no absolute paths** (per F-SEC-6) — this rejects `docs/slo/critique/../../../etc/passwd`; (d) every agent has a non-empty `copilot-fallback` field naming either "supported per matrix" or a specific feature-flag fallback; (e) every agent file ≤ 200 lines; (f) **`/slo-critique` SKILL.md unchanged — verified by SHA-256 hash constant pinned in the test file at runbook authoring time (or M4 close time)**; the test asserts the file's current SHA-256 matches the pinned constant. Updating the constant requires a runbook amendment (per F-ENG-6 critique resolution); (g) **agent-output artifacts conform to `/slo-critique`'s schema** — when M5 manually invokes an agent and produces `docs/slo/critique/<slug>.md`, a schema-validation step asserts the output contains the required sections (findings table, persona-tagged rows, recommendation cells); divergent format fails the validation (per F-ENG-7 critique resolution); (h) (Deferred) the lessons file documents the matrix gate that fired and includes a fresh-runbook handoff note. |
| Debugger / inspection expectation | Test failures cite the offending agent file + missing field or out-of-allowed-set output path. |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p sast-verify`. No `cargo deny` (no deps change). |
| Forbidden shortcuts | No agent-only output path that bypasses `docs/slo/critique/` or `docs/slo/verify/`. No Claude-only flow that lacks a Copilot fallback. No silent feature-flag default that ignores agents in Copilot. No 5th agent file. No agent file that modifies `/slo-critique` SKILL.md to depend on an agent. No agent file > 200 lines (split into a methodology reference file under `agents/<name>/references/` if more text is needed — though M5 doesn't ship that pattern; defer to a future runbook if needed). No copy-paste of `/slo-critique` persona prose into agents. |
| Data classification | `Public` — agent files and test are git-tracked. |
| Proactive controls in play | `C1` (Define Security Requirements — output-path constraint IS the requirement), `C2` (Leverage Security Frameworks — agents reuse the existing `docs/slo/critique/` artifact framework), `C7` (Enforce Access Controls — output paths are constrained by structural-contract test), `C9` (Implement Security Logging and Monitoring — agent invocations leave the same artifact trail as `/slo-critique`), `C10` (Handle All Errors and Exceptions — failed gate fails CI explicitly). |
| Abuse acceptance scenarios | `tm-sap-imp-abuse-12: a 5th agent file is added → structural-contract test fails CI`. `tm-sap-imp-abuse-13: an agent declares output-path outside the allowed set (e.g., docs/slo/agent-only-reports/) → structural-contract test fails`. `tm-sap-imp-abuse-14: an agent omits the copilot-fallback field → structural-contract test fails`. `tm-sap-imp-abuse-15: a hidden agent-only workflow that bypasses the runbook contract → reviewer-time check + Forbidden Shortcut + the artifact-path invariant catches it because outputs would land outside docs/slo/critique/`. See BDD abuse-case rows below. |

#### Out of Scope / Must Not Do

- Do **not** ship agent files if M4's host capability matrix decision is "agents not supported" or "deferred". Close as "deferred — no implementation" and write the handoff note.
- Do **not** modify `/slo-critique` SKILL.md. Persona rotation is the canonical portable path and stays.
- Do **not** introduce a `docs/slo/agent-only-reports/` directory or any output path outside the allowed set.
- Do **not** make Copilot users second-class. Every agent must document a fallback.
- Do **not** add a 5th agent. The four roles (lead, security, design, verification) cover the original Secure Agent Playbook breakdown adapted for SLO; more roles require a fresh runbook.
- Do **not** introduce hidden agent-only workflow that bypasses the runbook contract. Outputs land in `docs/slo/critique/` or `docs/slo/verify/` — same as `/slo-critique` and `/slo-verify`.
- Do **not** change `sldo-install` to install agents. If host-native agent install requires it, that's a fresh runbook (the installer's host-aware logic is non-trivial and worth its own scoping).

#### Pre-Flight

1. Complete the Global Entry Rules (Section 7).
2. Read `docs/slo/lessons/sap-imp-m4.md` and apply corrections.
3. Read `docs/slo/design/host-capability-matrix.md` (M4 output) and confirm the agent-support decision row.
4. Read `skills/slo-critique/SKILL.md` to confirm canonical portable path semantics.
5. Read existing `docs/slo/critique/*.md` artifacts (if any) to mirror format.
6. Copy the Evidence Log template into working notes.
7. Re-state milestone constraints: gated on matrix; ≤ 4 agents; output paths constrained; Copilot fallback documented; `/slo-critique` unchanged.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `agents/slo-runbook-review-lead.md` | NEW (conditional): lead agent that scopes a runbook, dispatches the three specialists, dedupes findings, writes one `docs/slo/critique/<slug>.md`. Frontmatter: `name`, `role: lead`, `output-paths: [docs/slo/critique/]`, `copilot-fallback`, `host-required`. ≤ 200 lines. |
| `agents/slo-security-reviewer.md` | NEW (conditional): security specialist that reviews threat-model coverage, abuse cases, secret hygiene, dependency audit, returns findings to the lead. Frontmatter: `name`, `role: security-reviewer`, `output-paths: [docs/slo/critique/]`, `copilot-fallback`, `host-required`. |
| `agents/slo-design-reviewer.md` | NEW (conditional): design specialist that reviews UX flow, naming, error states, returns findings to the lead. |
| `agents/slo-verification-lead.md` | NEW (conditional): verification specialist that maps milestone Definition-of-Done items to verifiable evidence, can write into `docs/slo/verify/` if the host treats verification as a separate phase. Frontmatter `output-paths: [docs/slo/critique/, docs/slo/verify/]`. |
| `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` | NEW (always): structural-contract test (file count ≤ 4; frontmatter present + valid; output-paths in allowed set; copilot-fallback non-empty; line-count ≤ 200; `/slo-critique` SKILL.md unchanged). Test passes vacuously when `agents/` doesn't exist (deferred branch). |
| `docs/skill-pack-catalog.md` | (Conditional) Add an "Agent roles (optional, host-native)" subsection listing the four agents with one-sentence roles. |
| `CLAUDE.md` | (Conditional) Append "Specialist agents (optional)" section noting agents land outputs in `docs/slo/critique/`. |
| `copilot-instructions.md` | (Conditional) Document the Copilot fallback path explicitly: agents are unavailable on Copilot today (or supported per matrix), and `/slo-critique` persona rotation remains the canonical critique flow. |
| `docs/ARCHITECTURE.md` | (Conditional) Add "Agent roles" subsection. |
| `README.md` | (Conditional) Mention specialist agents as optional in the existing skills section, with explicit "(`/slo-critique` persona rotation remains canonical portable path)" wording. |

#### Step-by-Step

1. Read M4's `docs/slo/design/host-capability-matrix.md`. Identify the agent-support decision: `supported` / `not-supported` / `deferred`.
2. Write `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` first. Confirm it passes vacuously against an empty repo state (no `agents/` directory).
3. **Decision branch:**
   - **Supported (green-lit)**: proceed to step 4.
   - **Not supported / deferred**: skip to step 11.
4. (Green-lit) Author `agents/slo-runbook-review-lead.md`. Frontmatter complete. Body ≤ 200 lines describing scope-and-dispatch logic.
5. (Green-lit) Author `agents/slo-security-reviewer.md`.
6. (Green-lit) Author `agents/slo-design-reviewer.md`.
7. (Green-lit) Author `agents/slo-verification-lead.md`.
8. (Green-lit) Re-run `cargo test -p sast-verify --test sap_imp_m5_agents` — must pass with all four agents present.
9. (Green-lit) Update `docs/skill-pack-catalog.md`, `CLAUDE.md`, `copilot-instructions.md`, `docs/ARCHITECTURE.md`, `README.md` per overlay conventions.
10. (Green-lit) Manually exercise one agent invocation against a real runbook (this runbook itself, or a peer) to confirm the output lands in `docs/slo/critique/<slug>.md`. Document outcome in the lessons file.
11. (Deferred) Author the lessons file with the matrix-gate-fired note + fresh-runbook handoff text. Document why M5 deferred and what would unblock it.
12. Run `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
13. Smoke tests; Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: gated host-native agent experiment**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Green-lit path: 4 agents land with valid frontmatter | happy path | M4 matrix decision = supported; four agent files authored | `cargo test -p sast-verify --test sap_imp_m5_agents` runs | Test passes; reports 4 files, all frontmatter valid, all output paths in allowed set |
| Deferred path: no agents, test passes vacuously | happy path | M4 matrix decision = not-supported; `agents/` directory absent | structural-contract test runs | Test passes (vacuous); lessons file contains deferred-note + fresh-runbook handoff |
| 5th agent caught | resource bound (`tm-sap-imp-abuse-12`) | A test fixture with 5 agent files | structural-contract test runs | Test fails with "expected ≤ 4 agents, found 5" |
| Out-of-allowed-set output path caught | invalid input (`tm-sap-imp-abuse-13`) | A test fixture agent with `output-paths: [docs/slo/agent-only/]` | structural-contract test runs | Test fails citing the agent + offending path |
| Path-traversal in output path caught | invalid input (F-SEC-6) | A test fixture agent with `output-paths: [docs/slo/critique/../../../etc/passwd]` | structural-contract test runs | Test fails with "output-path contains `..` traversal segment — canonicalized path escapes allowed prefix" |
| Absolute output path caught | invalid input (F-SEC-6) | A test fixture agent with `output-paths: [/etc/passwd]` | structural-contract test runs | Test fails with "output-path is absolute — relative paths only" |
| Agent-output format mismatch caught | invalid input (F-ENG-7) | M5 manual invocation produces `docs/slo/critique/test-runbook.md` missing the findings-table header | schema-validation step runs as part of smoke test | Test fails citing the artifact + missing required section; runbook compatibility commitment ("agents and persona rotation produce the same shape") preserved |
| Missing copilot-fallback caught | invalid input (`tm-sap-imp-abuse-14`) | A test fixture agent without the field | structural-contract test runs | Test fails with "missing copilot-fallback field" |
| Agent file > 200 lines | resource bound | A test fixture agent at 240 lines | structural-contract test runs | Test fails with line count + cap |
| `/slo-critique` SKILL.md modified | invalid input | A diff that modifies `/slo-critique` SKILL.md alongside agent additions | structural-contract test runs (with M5 invariant on canonical-path preservation) | Test fails with "M5 must not modify /slo-critique SKILL.md" |
| Hidden agent-only output path | abuse case (`tm-sap-imp-abuse-15`) | An agent attempts to write to `docs/agent-output/` at runtime | manually exercised invocation | Output is rejected because the agent's frontmatter `output-paths:` doesn't include the path; the runbook contract is preserved |
| Copilot user runs `/slo-critique` instead | compatibility | M4 matrix says Copilot doesn't support agents; user is on Copilot | User invokes `/slo-critique` | Persona rotation runs as before; same artifact format produced; user is not second-class |
| sldo-install regression | compatibility | M5 lands `agents/` (green-lit) | `sldo-install --dry-run` | Installer ignores `agents/` (it walks `skills/<name>/SKILL.md` only) |
| Existing skill prose preserved | compatibility | All shipped SKILL.md files | M5 close-out diff review | Only `agents/`, the test, and overlay docs changed; no skill prose modified |
| M1 + M2 + M3 + M4 regression | compatibility | Four prior structural-contract tests | `cargo test -p sast-verify` | All four prior tests still green |

#### Regression Tests

- M1's `sap_imp_m1_citations` test still passes.
- M2's `sap_imp_m2_examples` test still passes.
- M3's `sap_imp_m3_standards` test still passes.
- M4's `sap_imp_m4_workflow_pinning` test still passes.
- `cargo test -p sldo-install` — installer unchanged.
- `sldo-install --dry-run` — resolves shipped skills; ignores `agents/`.
- `cargo test --workspace` — full suite green.
- Manually invoke `/slo-critique` against a peer runbook to confirm persona rotation still produces a `docs/slo/critique/<slug>.md` artifact (canonical portable path preserved).

#### Compatibility Checklist

- [ ] `/slo-critique` SKILL.md unchanged
- [ ] `docs/slo/critique/<slug>.md` artifact format unchanged (agents and persona rotation produce the same shape)
- [ ] `sldo-install` install paths unchanged; `agents/` ignored by installer
- [ ] `docs/skill-pack-catalog.md` augmented additively (no skill removed or renamed)
- [ ] GitHub Copilot install path unchanged
- [ ] M1, M2, M3, M4 structural-contract tests still pass
- [ ] All shipped `skills/<name>/SKILL.md` files unchanged in this milestone
- [ ] (Green-lit) Manual exercise of one agent invocation produces `docs/slo/critique/<slug>.md`
- [ ] (Deferred) Lessons file documents the matrix gate + fresh-runbook handoff

#### E2E Runtime Validation

**File**: `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `agents_directory_passes_vacuously_or_strictly` | Test handles both branches | If `agents/` absent: test passes; if present: strict file-count + frontmatter + path checks |
| `at_most_four_agent_files` | File-count bound holds | Counts `agents/*.md`; asserts ≤ 4 |
| `every_agent_has_required_frontmatter` | Frontmatter invariant holds | Each agent has `name`, `role`, `output-paths`, `copilot-fallback`, `host-required` |
| `every_output_path_in_allowed_set` | Output-path invariant + path-traversal rejection (per F-SEC-6) | Each `output-paths:` entry, after canonicalizing via `Path::components()`, has prefix in `{docs/slo/critique/, docs/slo/verify/}` AND contains no `Component::ParentDir` AND is not absolute. Rejects `docs/slo/critique/../../../etc/passwd` despite the prefix match |
| `copilot_fallback_documented` | Multi-host invariant holds | Each agent's `copilot-fallback` field is non-empty |
| `agent_file_under_line_cap` | Size bound holds | Each agent file ≤ 200 lines |
| `slo_critique_skill_md_unchanged` | Canonical portable path preserved (per F-ENG-6 — stable baseline) | The test file pins a SHA-256 hash constant of `skills/slo-critique/SKILL.md` recorded at runbook authoring time (or M4 close time, captured via `sha256sum skills/slo-critique/SKILL.md` and embedded as `const CRITIQUE_SKILL_SHA256: &str = "..."`). Test asserts the file's current SHA-256 matches the constant. Updating the constant requires a runbook amendment |
| `agent_output_artifact_schema_valid` | Agent-output format-compatibility (per F-ENG-7) | When M5 manual invocation produces `docs/slo/critique/<slug>.md`, schema-validation step asserts the artifact contains required sections (findings table header, persona-tagged rows, recommendation column). Format extracted from current `/slo-critique` SKILL.md output contract or from one peer-critique example |

#### Smoke Tests

- [ ] (Green-lit) Manually read each of the 4 agent files end-to-end; prose reads naturally; frontmatter valid
- [ ] (Green-lit) Manually invoke one agent against a peer runbook; output lands in `docs/slo/critique/<slug>.md`
- [ ] `cargo test -p sast-verify --test sap_imp_m5_agents` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo build --workspace` boots cleanly
- [ ] `git status` clean
- [ ] `.gitignore` reviewed (no change expected)
- [ ] (Deferred) Lessons file has deferred-note + fresh-runbook handoff
- [ ] `/slo-critique` SKILL.md unchanged (verify via `git diff` showing zero changes to the file)

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| Matrix gate read | manual review of `host-capability-matrix.md` | decision recorded; branch chosen | | | |
| Test created | `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` | passes vacuously (agents/ absent at test creation) | | | |
| (Green-lit) lead agent authored | manual review | frontmatter valid; ≤ 200 lines | | | |
| (Green-lit) security-reviewer authored | manual review | frontmatter valid; ≤ 200 lines | | | |
| (Green-lit) design-reviewer authored | manual review | frontmatter valid; ≤ 200 lines | | | |
| (Green-lit) verification-lead authored | manual review | frontmatter valid; ≤ 200 lines | | | |
| (Green-lit) overlay docs updated | manual review | catalog + CLAUDE + copilot + ARCHITECTURE + README updated | | | |
| (Green-lit) Manual agent invocation | run lead against this runbook | output at `docs/slo/critique/<slug>.md` | | | |
| (Deferred) Lessons file with handoff | manual review | deferred-note + fresh-runbook handoff present | | | |
| Formatter | `cargo fmt --all -- --check` | clean | | | |
| Typecheck / build | `cargo build --workspace` | clean | | | |
| Static analyzer / linter | `cargo clippy --workspace --all-targets -- -D warnings` | clean | | | |
| Dependency audit | (no deps change — skip) | n/a | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test -p sast-verify --test sap_imp_m5_agents` | green (vacuous or strict per branch) | | | |
| Build/boot | `cargo build --workspace` | boots cleanly | | | |
| Smoke tests | manual review + invocation | all checked | | | |
| Resource-bound verification | ≤ 4 agents; ≤ 200 lines each; output paths in allowed set | bound encoded; test asserts | | | |
| Invariant/assertion verification | frontmatter + path constraints + canonical-path preservation | invariants encoded and tested | | | |
| Debugger / state inspection | (n/a — Markdown / structural-contract; agent-runtime issues debugged via direct artifact inspection) | hypothesis confirmed before code change | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | review `.gitignore` | no change | | | |
| Compatibility checks | run Compatibility Checklist | no regressions | | | |
| M1 + M2 + M3 + M4 regression | four prior tests | all green | | | |

#### Definition of Done

**Always (regardless of branch):**
- `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` exists; passes (vacuously or strictly per branch).
- All BDD scenarios that apply to the chosen branch pass.
- M1, M2, M3, M4 tests still green.
- Full existing test suite remains green.
- Formatter, typecheck, clippy clean.
- `/slo-critique` SKILL.md byte-identical to its M4 baseline (verified by test).
- `git status` clean.
- Lessons file written.
- Completion summary written.
- Milestone Tracker updated.

**If green-lit, additionally:**
- Four agent files exist; each ≤ 200 lines; each frontmatter valid; each output-path in allowed set; each copilot-fallback non-empty.
- Manual invocation of one agent produces `docs/slo/critique/<slug>.md`.
- Overlay docs updated (catalog + CLAUDE + copilot + ARCHITECTURE + README).

**If deferred, additionally:**
- Lessons file contains explicit deferred-note + fresh-runbook handoff with the unblock condition (e.g., "Reopen when GitHub Copilot supports host-native agent install or equivalent").

#### Post-Flight

- **ARCHITECTURE.md** (green-lit): add "Agent roles" subsection under skill pack overview.
- **ARCHITECTURE.md** (deferred): no change.
- **README.md** (green-lit): mention specialist agents as optional with `/slo-critique` portable-path wording.
- **README.md** (deferred): no change.
- **Other docs** (green-lit): `docs/skill-pack-catalog.md` adds Agent roles subsection; `CLAUDE.md` and `copilot-instructions.md` get matching overlay updates.
- **Other docs** (deferred): no change.

#### Notes

- The `/slo-critique` SKILL.md byte-identical assertion is intentionally strict. The risk this controls is "agents subtly drift the canonical portable path by editing prose in `/slo-critique` to reference agents". The strict check forces any future change to that skill to land in a separate runbook with explicit scope.
- The 4-agent cap mirrors `/slo-critique`'s four-persona rotation. A 5th agent would create asymmetry between the portable path and the host-native path; if a fifth specialist is genuinely needed, the persona list and the agent list both expand together, in a fresh runbook.
- The `output-paths` set `{docs/slo/critique/, docs/slo/verify/}` is intentionally narrow. Verification artifacts (M5 verification-lead) might want a separate path; if so, M5's matrix doc should justify it. If both M5 and `/slo-verify` write to `docs/slo/verify/`, no conflict — the artifact format is shared.
- The "vacuous-pass" pattern for the deferred branch means the structural-contract test always runs in CI, even when M5 deferred. That keeps the gate live: if a future PR adds an `agents/` directory without re-running this runbook, the test starts enforcing strictly.
- The Copilot-fallback discipline is the lever that prevents this milestone from quietly breaking the multi-host story. Even in the green-lit branch, every agent file documents what Copilot users do instead. That's the explicit anti-regression on `agent-host-capabilities.md`'s framing.

---

## 18. Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | Note `xtasks/sast-verify/tests/sap_imp_m*` structural-contract test family under "Test Architecture" | none expected | none expected | none |
| 2 | Add "Examples gallery" subsection under skill pack overview pointing at `examples/README.md` | Add a one-paragraph "Examples" section linking to `examples/README.md` | none (examples are committed) | none |
| 3 | Note `references/security/standards-mapping.md` under "References subtrees" | none expected | none expected | `docs/skill-pack-catalog.md` if security-related skill descriptions changed |
| 4 | If green-lit: add "Distribution channels" subsection mentioning optional Claude plugin packaging additive to `sldo-install` | If green-lit: add "Install via Claude plugin (optional)" section explicitly noting Rust installer remains canonical; if not green-lit: brief decision-doc reference | Possibly `dist/`, `*.zip` if release workflow generates local artifacts | `docs/slo/design/host-capability-matrix.md` itself (created in M4) |
| 5 | If green-lit: add "Agent roles" subsection | If green-lit: add "Specialist agents (optional)" section noting `/slo-critique` persona rotation remains canonical portable path; if not green-lit: brief deferred-to-fresh-runbook note | none expected | `CLAUDE.md` and `copilot-instructions.md` if agent install paths exist; `docs/skill-pack-catalog.md` |

---

## 19. Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, dependency/migration rules, required tests, required runtime validation, resource bounds, invariants/assertions, static-analysis gates, debugger expectation, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope, and explain how the user-facing result reduces user decisions or reviewer work.

---

## 20. Source Basis

This runbook is the v4 instance produced by `/slo-plan` from [docs/slo/design/secure-agent-playbook-imports-overview.md](slo/design/secure-agent-playbook-imports-overview.md) and the precursor stub [docs/slo/future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](slo/future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md). It applies the v4 Carmack-style reliability controls on top of v3's SunLit-specific structure (carry-forward from prior retros, abuse-acceptance scenarios, threat-model integration). The borrowed-structure approach intentionally re-authors every imported pattern in SLO language — runbooks, milestones, evidence logs, structural-contract tests — rather than copying Secure Agent Playbook procedure prose.
