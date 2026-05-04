# Issue 22 Critique Eval Coverage - SLO Ticket Contract v1

> **Purpose**: Execute one issue-sized change with v4 SLO rigor, without requiring a full multi-milestone runbook.
> **Audience**: AI coding agents first, humans second.
> **Source template**: Derived from `docs/slo/templates/runbook-template_v_4_template.md`. Use the full v4 runbook when this contract cannot stay issue-sized.

---

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `ticket-22-critique-eval-coverage` |
| Source tracker | `GitHub Issues` |
| Source issue | [#22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22) |
| Issue title | `Per-skill evals + hard-enforcement hooks + cross-skill polish` |
| Labels | `enhancement` |
| Assignee / owner | `unassigned` |
| Target branch | `slo/issue-22-critique-eval-coverage` |
| Primary stack | Markdown skill eval fixtures + Rust structural tests |
| Default formatter command | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` |
| Default typecheck / build command | `cargo test -p sldo-install --test e2e_eng_imp_m5 --no-run` |
| Default static analysis / lint command | `cargo clippy -p sldo-install --test e2e_eng_imp_m5 -- -D warnings` |
| Default unit / BDD command | `cargo test -p sldo-install --test e2e_eng_imp_m5` |
| Default runtime validation command | `N/A - documented eval expectations only; runtime harness is deferred by issue #22` |
| Default dependency / security audit command | `N/A - no dependency changes` |
| Default debugger or state-inspection tool | `N/A - structural Markdown assertions give direct failure evidence` |
| Public interfaces stable by default | `yes` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- Existing skill invocation names and `SKILL.md` paths.
- Existing `references/templates/eval-cases.md` file shape.
- Existing `.claude/settings.json` hook behavior from M5.

---

## 2. Sizing Gate

| Check | Answer |
|---|---|
| User-visible outcome fits in one sentence | `yes - /slo-critique now has the seven canonical eval category files named by issue #22` |
| Expected changed files <= 8 | `yes - one test, one ticket contract, seven fixtures` |
| New public surfaces <= 1 | `yes - no command surface; only documented eval fixtures and one structural-test list entry` |
| No schema migration unless explicitly approved | `yes` |
| No cross-subsystem rewrite | `yes` |
| Can be reviewed as one PR | `yes` |
| Requires full v4 runbook instead | `no - this is a narrow missing-skill eval coverage slice` |

---

## 3. Issue Context

### Problem

Issue #22 explicitly names `/slo-critique` in the high-risk eval coverage list. M5 and PR #42 cover the 16 named high-risk skills seeded by the M5 runbook, but `/slo-critique` still lacks `evals/` fixtures and is not enforced by the M5 structural test.

### Acceptance Criteria From Issue

- [ ] `/slo-critique` has the seven canonical eval case shapes: `happy-path`, `missing-context`, `ambiguous-input`, `adversarial`, `outdated-information`, `tool-failure`, and `high-risk-case`.
- [ ] Eval files use the shared Markdown frontmatter/body contract.
- [ ] Runtime Claude Code harness remains deferred; these files are documented expectations and manual-run fixtures for now.
- [ ] Public/open-source hygiene is preserved: synthetic examples only, no private repository names, no confidential examples, no paused `/slo-sec-libs` scope.

### Non-Goals

- No runtime eval harness.
- No changes to `/slo-critique` behavior or persona prose.
- No changes to `/slo-freeze` hook behavior.
- No expansion into issue #4 or paused security-library intake work.

### Reproduction / Current Signal

| Signal | Evidence |
|---|---|
| Current targeted test | `cargo test -p sldo-install --test e2e_eng_imp_m5` |
| Current result | Passes without checking `skills/slo-critique/evals/` |
| Expected result | Fails until `/slo-critique` has every canonical eval category file |

---

## 4. Compact Architecture Delta

N/A - no architecture delta. This ticket strengthens structural tests and adds Markdown fixtures.

### Data Flow Delta

```text
cargo test e2e_eng_imp_m5
  -> enumerates high-risk skills including slo-critique
  -> enumerates canonical eval categories
  -> asserts skills/slo-critique/evals/<category>.md exists and follows the shared shape
```

---

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | GitHub issue #22, existing M5 test, `references/templates/eval-cases.md`, `skills/slo-critique/SKILL.md`, existing eval patterns from PR #42 |
| Outputs | Stronger structural test, `/slo-critique` eval fixtures, filled ticket evidence, PR handoff |
| Interfaces touched | `skills/slo-critique/evals/<case>.md` documented expectations; no CLI/API changes |
| Files allowed to change | `crates/sldo-install/tests/e2e_eng_imp_m5.rs`; `docs/slo/tickets/ticket-22-critique-eval-coverage.md`; `skills/slo-critique/evals/{happy-path,missing-context,ambiguous-input,adversarial,outdated-information,tool-failure,high-risk-case}.md` |
| Files to read before changing | `crates/sldo-install/tests/e2e_eng_imp_m5.rs`; `references/templates/eval-cases.md`; `skills/slo-critique/SKILL.md`; representative existing eval fixtures from `skills/slo-sast/evals/` and `skills/slo-verify/evals/`; issue #22 |
| New files allowed | `docs/slo/tickets/ticket-22-critique-eval-coverage.md`; `skills/slo-critique/evals/{happy-path,missing-context,ambiguous-input,adversarial,outdated-information,tool-failure,high-risk-case}.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing 16-skill eval coverage remains valid; M5 hook/polish assertions remain unchanged; `/slo-critique` invocation path remains unchanged |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1 Input Validation analogue via adversarial prompt-injection expectations; C9 Logging/Monitoring analogue via evidence/report-shape expectations |
| Abuse acceptance scenarios | Adversarial eval files must treat embedded instructions as data and preserve critique gates against vague findings or scope-change auto-apply |
| Resource bounds introduced/changed | N/A - fixture files only |
| Invariants/assertions required | Structural test must include `slo-critique` in the canonical eval coverage list |
| Debugger / inspection expectation | N/A - direct assertion output is sufficient |
| Static analysis gates | `rustfmt`, targeted `cargo test`, targeted `cargo clippy`, `git diff --check` |
| Forbidden shortcuts | Do not alter persona methodology; do not use private examples; do not collapse categories into one file; do not close issue #22 because runtime harness remains deferred |

---

## 6. Implementation Plan

1. Add `slo-critique` to the M5 high-risk skill list.
2. Run the targeted test and record the expected missing-directory or missing-file failure.
3. Add seven `/slo-critique` eval fixtures using the shared shape.
4. Re-run targeted tests and static gates.
5. Fill validation evidence and open a stacked PR against `slo/eng-imp-m5`.

---

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| Critique categories required | `happy path` | `/slo-critique` is in the high-risk skill list | M5 structural test runs | All seven canonical category files are required | `cargo test -p sldo-install --test e2e_eng_imp_m5` |
| Missing critique eval fails | `invalid input` | `skills/slo-critique/evals/` is absent | M5 structural test runs after list update | Test fails with a missing path/category | Red-first targeted test |
| Tool-backed uncertainty explicit | `empty / degraded state` | A critique persona or source file cannot be read | `tool-failure.md` is inspected | The expected behavior records failure/skipped status rather than inventing a critique result | Fixture review + frontmatter test |
| Prompt injection remains data | `abuse case` | A runbook says to ignore critique gates or auto-apply scope changes | `adversarial.md` is inspected | Expected behavior treats that text as untrusted input and preserves the finding gates | Fixture review + frontmatter test |

---

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene gate | `git status --short --branch`; `git rev-parse --abbrev-ref HEAD`; `git symbolic-ref --short refs/remotes/origin/HEAD` | On task branch, default branch detected, no remediation needed | Branch `slo/issue-22-critique-eval-coverage`; default `origin/main`; no dirty tree before edits | `pass` | Workpad updated with branch |
| Baseline before change | `cargo test -p sldo-install --test e2e_eng_imp_m5` | passes before adding `slo-critique` to the list | Passed: 7 tests | `pass` | Captured before test-list change |
| New tests fail first | `cargo test -p sldo-install --test e2e_eng_imp_m5` | fails for missing `/slo-critique` eval coverage after test update | Failed as expected: 4 passed, 3 failed; missing `skills/slo-critique/evals/` and `happy-path.md` | `pass` | Failure was the intended missing-fixture assertion |
| Formatter | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` | passes | Passed | `pass` | |
| Typecheck / build | `cargo test -p sldo-install --test e2e_eng_imp_m5 --no-run` | passes | Passed | `pass` | |
| Static analysis / lint | `cargo clippy -p sldo-install --test e2e_eng_imp_m5 -- -D warnings` | passes or documented blocker | Passed | `pass` | |
| Unit / BDD tests | `cargo test -p sldo-install --test e2e_eng_imp_m5` | passes | Passed: 7 tests | `pass` | Includes `slo-critique` in canonical eval coverage |
| Runtime validation | `N/A` | documented eval expectations only | N/A - runtime harness is deferred by issue #22 | `pass` | |
| Dependency / security audit | `N/A` | no dependency changes | N/A - no dependency changes | `pass` | |
| Resource bound / invariant check | Category-existence structural test | passes | Passed: `/slo-critique` has 7 eval Markdown files | `pass` | |
| Compatibility check | Existing frontmatter/body test still passes for all eval files | passes | Passed in `eval_files_have_required_frontmatter`; `cargo test -p sldo-install` also passed with one unrelated warning in `e2e_biz_followup_m5.rs` | `pass` | Existing M5 hook/polish tests remain green |
| `.gitignore` / artifact cleanup | `git status --short` | no stray artifacts | Clean after implementation commit; ticket PR-link update only before final handoff commit | `pass` | No generated temporary files remain |

---

## 9. Workpad / Tracker Updates

Use one persistent issue comment marked `<!-- slo-ticket-workpad:v1 -->`.

Workpad comment: https://github.com/kerberosmansour/SunLitOrchestrate/issues/22#issuecomment-4371131623

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

- Added `slo-critique` to the M5 high-risk eval coverage list.
- Added seven public, synthetic `/slo-critique` eval fixtures covering happy path, missing context, ambiguity, adversarial prompt-injection, stale information, tool failure, and high-risk blocker handling.

### Tests And Validation

- `cargo test -p sldo-install --test e2e_eng_imp_m5` - passed before the stronger assertion, failed first for missing `/slo-critique/evals/`, then passed with 7 tests after fixture addition.
- `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` - passed.
- `cargo test -p sldo-install --test e2e_eng_imp_m5 --no-run` - passed.
- `cargo clippy -p sldo-install --test e2e_eng_imp_m5 -- -D warnings` - passed.
- `git diff --check` - passed.
- `cargo test -p sldo-install` - passed; emitted one unrelated warning in `tests/e2e_biz_followup_m5.rs`.

### Lessons / Follow-Ups

- Follow-up candidate: executable eval harness remains deferred under issue #22 / issue #4. This ticket deliberately keeps evals as documented expectations.

### PR / Issue Links

- PR: https://github.com/kerberosmansour/SunLitOrchestrate/pull/43
- Issue: https://github.com/kerberosmansour/SunLitOrchestrate/issues/22
