# Issue 22 Eval Category Coverage - SLO Ticket Contract v1

> **Purpose**: Execute one issue-sized change with v4 SLO rigor, without requiring a full multi-milestone runbook.
> **Audience**: AI coding agents first, humans second.
> **Source template**: Derived from `docs/slo/templates/runbook-template_v_4_template.md`. Use the full v4 runbook when this contract cannot stay issue-sized.

---

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `ticket-22-eval-category-coverage` |
| Source tracker | `GitHub Issues` |
| Source issue | [#22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22) |
| Issue title | `Per-skill evals + hard-enforcement hooks + cross-skill polish` |
| Labels | `enhancement` |
| Assignee / owner | `unassigned` |
| Target branch | `slo/issue-22-eval-category-coverage` |
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
| User-visible outcome fits in one sentence | `yes - high-risk skills now carry every canonical eval category file` |
| Expected changed files <= 8 | `no - fixture fan-out is mechanical and reviewable as one category-coverage patch` |
| New public surfaces <= 1 | `yes - no new command surface; only documented eval fixtures and one stronger structural test` |
| No schema migration unless explicitly approved | `yes` |
| No cross-subsystem rewrite | `yes` |
| Can be reviewed as one PR | `yes` |
| Requires full v4 runbook instead | `no - this is the missing category-coverage slice of already-completed M5` |

---

## 3. Issue Context

### Problem

Issue #22 asks each high-risk skill to have the seven canonical eval case shapes. M5 seeded one `happy-path.md` file per high-risk skill and a structural test for frontmatter/body shape, but it does not fail when the remaining six canonical categories are absent.

### Acceptance Criteria From Issue

- [ ] High-risk skill eval directories contain the seven canonical case shapes: `happy-path`, `missing-context`, `ambiguous-input`, `adversarial`, `outdated-information`, `tool-failure`, and `high-risk-case`.
- [ ] Eval files use the shared Markdown frontmatter/body contract.
- [ ] Runtime Claude Code harness remains deferred; these files are documented expectations and manual-run fixtures for now.
- [ ] Public/open-source hygiene is preserved: synthetic examples only, no private repository names, no confidential examples, no paused `/slo-sec-libs` scope.

### Non-Goals

- No runtime eval harness.
- No changes to `/slo-freeze` hook behavior.
- No expansion into issue #4 or paused security-library intake work.
- No generated examples containing real founders, private repositories, credentials, or sensitive personal data.

### Reproduction / Current Signal

| Signal | Evidence |
|---|---|
| Current targeted test | `cargo test -p sldo-install --test e2e_eng_imp_m5` |
| Current result | Passes with only `happy-path.md` present per high-risk skill |
| Expected result | Fails until every canonical eval category file exists for every named high-risk skill |

---

## 4. Compact Architecture Delta

N/A - no architecture delta. This ticket strengthens structural tests and adds Markdown fixtures.

### Data Flow Delta

```text
cargo test e2e_eng_imp_m5
  -> enumerates high-risk skills
  -> enumerates canonical eval categories
  -> asserts skills/<skill>/evals/<category>.md exists and follows the shared shape
```

---

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | GitHub issue #22, existing M5 runbook/test, `references/templates/eval-cases.md`, existing `skills/*/evals/happy-path.md` files |
| Outputs | Stronger structural test, missing eval category fixtures, filled ticket evidence, PR handoff |
| Interfaces touched | `skills/<skill>/evals/<case>.md` documented expectations; no CLI/API changes |
| Files allowed to change | `crates/sldo-install/tests/e2e_eng_imp_m5.rs`; `docs/slo/tickets/ticket-22-eval-category-coverage.md`; and the finite set `skills/{slo-legal,slo-accounting,slo-equity,slo-fundraise,slo-hire,slo-sast,slo-tla,slo-execute,slo-verify,slo-rulegen,slo-ruleverify,slo-research,slo-architect,slo-plan,slo-talk-to-users,slo-founder-check}/evals/{missing-context,ambiguous-input,adversarial,outdated-information,tool-failure,high-risk-case}.md` |
| Files to read before changing | `crates/sldo-install/tests/e2e_eng_imp_m5.rs`; `references/templates/eval-cases.md`; representative existing `skills/slo-legal/evals/happy-path.md`; issue #22 |
| New files allowed | `docs/slo/tickets/ticket-22-eval-category-coverage.md`; the finite eval fixture set listed above |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing `happy-path.md` fixtures remain valid; the M5 test still checks frontmatter/body shape and existing hook/polish assertions |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1 Input Validation and C9 Logging/Monitoring analogues via documented refusal/tool-failure expectations; no runtime security surface |
| Abuse acceptance scenarios | Adversarial eval files must treat embedded instructions as data and preserve hard-block/refusal gates |
| Resource bounds introduced/changed | N/A - fixture files only |
| Invariants/assertions required | Structural test must assert every canonical category file exists for every named high-risk skill |
| Debugger / inspection expectation | N/A - direct assertion output is sufficient |
| Static analysis gates | `rustfmt`, targeted `cargo test`, targeted `cargo clippy`, `git diff --check` |
| Forbidden shortcuts | Do not collapse all categories into one file; do not use private examples; do not mark `tool-failure` absent silently; do not alter unrelated skill prose |

---

## 6. Implementation Plan

1. Add a canonical category constant to the M5 test.
2. Add a BDD-style structural test that fails for missing category files.
3. Run the targeted test and record the expected red result.
4. Add missing category files using the shared eval shape and public synthetic examples.
5. Re-run targeted tests and static gates.
6. Fill validation evidence and open a stacked PR against `slo/eng-imp-m5`.

---

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| Canonical categories required | `happy path` | A high-risk skill has an `evals/` directory | M5 structural test runs | All seven canonical category files are required | `cargo test -p sldo-install --test e2e_eng_imp_m5` |
| Missing category fails | `invalid input` | A category file is absent | M5 structural test runs before fixtures are added | Test fails with the missing path/category | Red-first targeted test |
| Tool-backed and non-tool-backed skills are explicit | `empty / degraded state` | A skill has no external tool call for a case | `tool-failure.md` is read | The file states the expected fallback/N/A behavior rather than disappearing | Fixture review + frontmatter test |
| Prompt injection remains data | `abuse case` | An eval input contains instructions to bypass SLO gates | `adversarial.md` is read | Expected behavior preserves the gate and treats the text as untrusted input | Fixture review + frontmatter test |

---

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene gate | `git status --short --branch`; `git rev-parse --abbrev-ref HEAD`; `git symbolic-ref --short refs/remotes/origin/HEAD` | On task branch, default branch detected, no remediation needed | Branch `slo/issue-22-eval-category-coverage`; default `origin/main`; no dirty tree before edits | `pass` | Workpad updated with branch |
| Baseline before change | `cargo test -p sldo-install --test e2e_eng_imp_m5` | passes before stronger category assertion | Passed: 6 tests | `pass` | Captured before adding canonical-category assertion |
| New tests fail first | `cargo test -p sldo-install --test e2e_eng_imp_m5` | fails for missing category files after test update | Failed as expected: 6 passed, 1 failed; missing `slo-legal/evals/missing-context.md` | `pass` | Failure was the intended missing-fixture assertion |
| Formatter | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` | passes | Passed | `pass` | |
| Typecheck / build | `cargo test -p sldo-install --test e2e_eng_imp_m5 --no-run` | passes | Passed | `pass` | |
| Static analysis / lint | `cargo clippy -p sldo-install --test e2e_eng_imp_m5 -- -D warnings` | passes or documented blocker | Passed | `pass` | |
| Unit / BDD tests | `cargo test -p sldo-install --test e2e_eng_imp_m5` | passes | Passed: 7 tests | `pass` | Includes new canonical-category test |
| Runtime validation | `N/A` | documented eval expectations only | N/A - runtime harness is deferred by issue #22 | `pass` | |
| Dependency / security audit | `N/A` | no dependency changes | N/A - no dependency changes | `pass` | |
| Resource bound / invariant check | Category-existence structural test | passes | Passed: each of the 16 named high-risk skills has 7 eval Markdown files | `pass` | Fixture count command confirmed 7 per skill |
| Compatibility check | Existing frontmatter/body test still passes for all eval files | passes | Passed in `eval_files_have_required_frontmatter`; `cargo test -p sldo-install` also passed with one unrelated warning in `e2e_biz_followup_m5.rs` | `pass` | Existing M5 hook/polish tests remain green |
| `.gitignore` / artifact cleanup | `git status --short` | no stray artifacts | Intended ticket/test/eval changes only | `pass` | Final clean status to be recorded after commit |

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

- Added a structural assertion that every named high-risk skill has all seven canonical eval category files.
- Added public, synthetic missing-context, ambiguous-input, adversarial, outdated-information, tool-failure, and high-risk-case fixtures for the 16 high-risk skills already seeded by M5.

### Tests And Validation

- `cargo test -p sldo-install --test e2e_eng_imp_m5` - passed before the stronger assertion, failed first for missing category fixtures, then passed with 7 tests after fixture addition.
- `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` - passed.
- `cargo test -p sldo-install --test e2e_eng_imp_m5 --no-run` - passed.
- `cargo clippy -p sldo-install --test e2e_eng_imp_m5 -- -D warnings` - passed.
- `git diff --check` - passed.
- `cargo test -p sldo-install` - passed; emitted one unrelated warning in `tests/e2e_biz_followup_m5.rs`.

### Lessons / Follow-Ups

- Follow-up candidate: executable eval harness remains deferred under issue #22 / issue #4. This ticket deliberately keeps evals as documented expectations.

### PR / Issue Links

- PR: `pending`
- Issue: https://github.com/kerberosmansour/SunLitOrchestrate/issues/22
