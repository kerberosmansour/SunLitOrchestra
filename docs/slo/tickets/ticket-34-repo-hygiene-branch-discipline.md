# Repo Hygiene And Branch Discipline - SLO Ticket Contract v1

> **Purpose**: Execute one issue-sized change with v4 SLO rigor, without requiring a full multi-milestone runbook.
> **Audience**: AI coding agents first, humans second.
> **Source template**: Derived from `docs/slo/templates/runbook-template_v_4_template.md`. Use the full v4 runbook when this contract cannot stay issue-sized.

---

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `ticket-34-repo-hygiene-branch-discipline` |
| Source tracker | `GitHub Issues` |
| Source issue | [#34](https://github.com/kerberosmansour/SunLitOrchestrate/issues/34) |
| Issue title | `Add repo hygiene and branch discipline to SLO execution skills` |
| Labels | `enhancement`, `retro-derived` |
| Assignee / owner | `unassigned` |
| Target branch | `ticket/34-repo-hygiene-branch-discipline` |
| Primary stack | Markdown skill contracts + Rust structural tests |
| Default formatter command | `cargo fmt --check -p sldo-install` |
| Default typecheck / build command | `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene --no-run` |
| Default static analysis / lint command | `cargo clippy -p sldo-install --tests -- -D warnings` |
| Default unit / BDD command | `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene` |
| Default runtime validation command | `N/A - docs-only skill-contract change` |
| Default dependency / security audit command | `N/A - no dependency changes` |
| Default debugger or state-inspection tool | `N/A - structural Markdown assertions are enough for this ticket` |
| Public interfaces stable by default | `yes` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- `/slo-execute`
- `/slo-ticket-execute`

---

## 2. Sizing Gate

| Check | Answer |
|---|---|
| User-visible outcome fits in one sentence | `yes - execution skills now block or remediate unsafe default-branch work before edits` |
| Expected changed files <= 8 | `yes - 4 files` |
| New public surfaces <= 1 | `yes - no new command surface; existing skill contract text only` |
| No schema migration unless explicitly approved | `yes` |
| No cross-subsystem rewrite | `yes` |
| Can be reviewed as one PR | `yes` |
| Requires full v4 runbook instead | `no - issue-sized process hardening` |

---

## 3. Issue Context

### Problem

The execution skills already enforce BDD and allow-list discipline, but they do not consistently stop work that starts on `main` or another default/protected branch. They also do not record branch remediation evidence clearly enough for later review.

Quoted issue context:

~~~text
Add a pre-flight repo hygiene gate to both execution paths:

1. Run and record `git status --short --branch` before edits.
2. Detect the current branch and default branch, including `main`, `master`, and `origin/HEAD` where available.
3. If execution is on the default/protected branch, stop before file edits and create/switch to a task branch, unless the user explicitly instructed otherwise.
4. If uncommitted work already exists on the default branch, preserve it by switching to a new branch immediately and record the remediation.
5. Use predictable branch names for runbooks and tickets, e.g. `slo/<runbook-prefix>-m<N>` or `ticket/<issue>-<slug>` when no branch is specified.
6. Add an evidence row for repo hygiene: branch before, branch after, dirty-tree state, and whether any remediation was needed.
7. Clarify commit discipline: execution may prepare the working tree, but commits/pushes happen only when the active workflow or user explicitly asks for them.
8. Ensure `/slo-ticket-execute` updates the issue workpad with the branch name once selected.
~~~

### Acceptance Criteria From Issue

- [ ] `/slo-execute` refuses to continue on `main`/default branch until a feature branch exists.
- [ ] `/slo-ticket-execute` has the same branch and dirty-tree guardrails as `/slo-execute`.
- [ ] Both skills document how to preserve already-started work that accidentally began on `main`.
- [ ] Both skills document evidence-log/workpad fields for repo hygiene.
- [ ] Tests or docs validation cover the new pre-flight expectations if the repo has a suitable skill validation harness.

### Non-Goals

- No runtime hook implementation.
- No branch creation, commit, or PR automation changes beyond skill contract prose.
- No changes to the full runbook or ticket contract templates.

### Reproduction / Current Signal

| Signal | Evidence |
|---|---|
| Current `/slo-execute` pre-flight | Does not require `git status --short --branch`, default-branch detection, or branch remediation before file edits |
| Current `/slo-ticket-execute` pre-flight | Only says to create or switch to the target branch |
| Expected result | Both skills document a repo hygiene gate before edits, branch naming, dirty-tree preservation, evidence/workpad fields, and commit discipline |

---

## 4. Compact Architecture Delta

N/A - no architecture delta. This ticket hardens Markdown skill contracts and structural tests only.

### Data Flow Delta

```text
agent invokes execution skill
  -> repo hygiene pre-flight checks git state before edits
  -> branch remediation is recorded in evidence/workpad
  -> implementation proceeds only on a task branch unless user explicitly overrides
```

---

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | GitHub issue #34, existing skill prose, structural-test conventions |
| Outputs | Updated skill contracts, new structural-contract test, filled ticket evidence |
| Interfaces touched | `/slo-execute`, `/slo-ticket-execute` prose contracts |
| Files allowed to change | `skills/slo-execute/SKILL.md`, `skills/slo-ticket-execute/SKILL.md`, `crates/sldo-install/tests/e2e_ticket_issue34_repo_hygiene.rs`, `docs/slo/tickets/ticket-34-repo-hygiene-branch-discipline.md` |
| Files to read before changing | `skills/slo-execute/SKILL.md`, `skills/slo-ticket-execute/SKILL.md`, `crates/sldo-install/tests/e2e_ticket_flow.rs`, `docs/slo/design/ticket-sized-slo-workflow.md`, `docs/ARCHITECTURE.md` |
| New files allowed | `crates/sldo-install/tests/e2e_ticket_issue34_repo_hygiene.rs`, `docs/slo/tickets/ticket-34-repo-hygiene-branch-discipline.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing execution skill flows, allow-list rule, ticket-loop contract, and no auto-commit behavior remain stable |
| Data classification | `Public` |
| Proactive controls in play | N/A - process/documentation control, no product security surface |
| Abuse acceptance scenarios | Branch names must be deterministic and not include an agent name; default-branch writes are blocked unless explicitly overridden |
| Resource bounds introduced/changed | N/A - no runtime resource growth |
| Invariants/assertions required | Structural tests assert repo hygiene markers, branch naming, evidence/workpad fields, preservation of dirty default-branch work, and commit discipline |
| Debugger / inspection expectation | N/A - failing structural tests provide direct evidence |
| Static analysis gates | `cargo fmt --check -p sldo-install`, `cargo clippy -p sldo-install --tests -- -D warnings`, targeted integration test |
| Forbidden shortcuts | Do not only update one execution skill; do not use agent-named branch examples; do not imply commits/pushes happen during execution without explicit user/workflow request |

---

## 6. Implementation Plan

1. Add a failing structural-contract test for #34.
2. Run the targeted test and record the expected failure.
3. Add repo hygiene pre-flight prose to `/slo-execute`.
4. Add matching branch/dirty-tree/workpad prose to `/slo-ticket-execute`.
5. Run formatter, targeted test, and build/static gates as practical.
6. Fill validation evidence in this ticket contract and update the issue workpad.

---

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| Sprint execution blocks default branch | `happy path` | `/slo-execute` is invoked on `main`, `master`, or `origin/HEAD` default | Pre-flight runs before edits | The skill requires a task branch such as `slo/<runbook-prefix>-m<N>` before continuing | Structural test |
| Ticket execution uses issue branch | `happy path` | `/slo-ticket-execute` has no existing target branch | Pre-flight chooses a branch | The skill documents `ticket/<issue>-<slug>` and updates the issue workpad | Structural test |
| Dirty default branch is preserved | `empty / degraded state` | Uncommitted work exists on the default branch | Pre-flight detects it | The skill says to switch/create a task branch immediately and record remediation | Structural test |
| Agent-named branches are avoided | `abuse case` | A coding agent is tempted to create a branch with host or agent branding | Pre-flight derives a branch name | Branch examples remain task-scoped and omit agent names | Structural test |

---

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install --test e2e_ticket_flow` | passes | Passed: 5 tests | `pass` | Existing ticket-flow baseline |
| New tests fail first | `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene` | fails for missing repo hygiene prose before implementation | Failed as expected before skill edits: 0 passed, 6 failed, missing repo hygiene markers | `pass` | BDD-first structural test |
| Formatter | `cargo fmt --check -p sldo-install` | passes | Package-level check blocked by pre-existing unrelated formatting drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`; targeted `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_ticket_issue34_repo_hygiene.rs` passed | `pass-with-note` | Did not reformat unrelated files outside the ticket allow-list |
| Typecheck / build | `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene --no-run` | passes | Passed | `pass` | |
| Static analysis / lint | `cargo clippy -p sldo-install --tests -- -D warnings` | passes or documented existing warnings | Package-level clippy blocked by pre-existing unrelated `doc_lazy_continuation` warnings in `tests/common/claude_runtime.rs`; targeted `cargo clippy -p sldo-install --test e2e_ticket_issue34_repo_hygiene -- -D warnings` passed | `pass-with-note` | Did not edit unrelated test helper outside the allow-list |
| Unit / BDD tests | `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene` | passes | Passed: 6 tests | `pass` | |
| Runtime validation | `N/A` | docs-only skill-contract change | N/A - no runtime surface | `pass` | |
| Dependency / security audit | `N/A` | no dependency changes | N/A - no dependency changes | `pass` | |
| Resource bound / invariant check | Structural test branch-name and evidence-field assertions | passes | Passed in `e2e_ticket_issue34_repo_hygiene` | `pass` | |
| Compatibility check | Review `skills/slo-execute/SKILL.md` and `skills/slo-ticket-execute/SKILL.md` allow-list sections remain intact | passes | Passed: allow-list and anti-placeholder markers still present in both skills | `pass` | |
| `.gitignore` / artifact cleanup | `git status --short` | no stray artifacts | Clean except intended changes on `ticket/34-repo-hygiene-branch-discipline` | `pass` | |

---

## 9. Workpad / Tracker Updates

Use one persistent issue comment marked `<!-- slo-ticket-workpad:v1 -->`.

---

## 10. Self-Review Gate

- [x] Did I stay inside the file allow-list?
- [x] Did I write or update BDD tests before production code?
- [x] Did I confirm new tests failed for the right reason before implementing?
- [x] Did I preserve public interfaces unless explicitly allowed to change them?
- [x] Did I add or strengthen assertions/invariants where the contract required them?
- [x] Did I bound new resource growth or document why no bound applies?
- [x] Did I run formatter, typecheck/build, and static analysis?
- [x] Did I use a debugger or state-inspection tool when failure evidence was ambiguous?
- [x] Did I remove temporary proof edits, debug output, and placeholder logic?
- [x] Did I record evidence rather than claims?
- [x] Did I update the issue workpad and PR handoff notes?

---

## 11. Closure Summary

### Completed

- Added a repo hygiene pre-flight gate to `/slo-execute`.
- Added matching branch, dirty-tree, workpad, and commit-discipline guidance to `/slo-ticket-execute`.
- Added structural-contract tests for issue #34.

### Tests And Validation

- `cargo test -p sldo-install --test e2e_ticket_flow` - passed.
- `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene` - failed first for missing markers, then passed after implementation.
- `cargo test -p sldo-install --test e2e_ticket_issue34_repo_hygiene --no-run` - passed.
- `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_ticket_issue34_repo_hygiene.rs` - passed.
- `cargo clippy -p sldo-install --test e2e_ticket_issue34_repo_hygiene -- -D warnings` - passed.
- Package-level `cargo fmt --check -p sldo-install` and `cargo clippy -p sldo-install --tests -- -D warnings` are blocked by unrelated pre-existing drift outside this ticket allow-list, documented above.

### Lessons / Follow-Ups

- Follow-up candidate: fix existing `cargo fmt` drift in `e2e_biz_imp_m1.rs` / `e2e_biz_imp_m2.rs` and clippy `doc_lazy_continuation` warnings in `tests/common/claude_runtime.rs` in a separate cleanup ticket.

### PR / Issue Links

- PR: https://github.com/kerberosmansour/SunLitOrchestrate/pull/36
- Issue: https://github.com/kerberosmansour/SunLitOrchestrate/issues/34
