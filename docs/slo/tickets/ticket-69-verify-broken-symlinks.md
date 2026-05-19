# Verify Broken Symlinks - SLO Ticket Contract v1

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `ticket-69-verify-broken-symlinks` |
| Source tracker | `GitHub Issues` |
| Source issue | [#69](https://github.com/kerberosmansour/SunLitOrchestra/issues/69) |
| Issue title | `sldo-install verify reports broken symlinks as ok (no link-liveness check)` |
| Labels | `none` |
| Assignee / owner | `kerberosmansour` |
| Target branch | `slo/ticket-69-verify-broken-symlinks` |
| Primary stack | `Rust CLI` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default typecheck / build command | `cargo check -p sldo-install` |
| Default static analysis / lint command | `cargo clippy -p sldo-install --all-targets -- -D warnings` |
| Default unit / BDD command | `cargo test -p sldo-install --test e2e_agent_host_m1 test_verify_rejects_broken_manifest_source_link` |
| Default runtime validation command | `cargo test -p sldo-install --test e2e_agent_host_m1` |
| Default dependency / security audit command | `N/A - no dependency changes` |
| Default debugger or state-inspection tool | `rg`, `git diff`, and focused CLI test output |
| Public interfaces stable by default | `yes` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |

### Public Interfaces That Must Remain Stable Unless Explicitly Listed Otherwise

- `sldo-install verify` remains the public CLI command.
- Existing host selection and manifest layout remain unchanged.

## 2. Sizing Gate

| Check | Answer |
|---|---|
| User-visible outcome fits in one sentence | `yes - verify fails when a recorded source symlink no longer resolves` |
| Expected changed files <= 8 | `yes` |
| New public surfaces <= 1 | `yes - stricter existing verify behavior only` |
| No schema migration unless explicitly approved | `yes` |
| No cross-subsystem rewrite | `yes` |
| Can be reviewed as one PR | `yes` |
| Requires full v4 runbook instead | `no` |

## 3. Issue Context

### Problem

`sldo-install verify` currently checks that a managed link points at the manifest source path, but it does not check whether that source path still exists. A stale checkout rename can therefore leave broken symlinks that verify reports as `ok`.

### Acceptance Criteria From Issue

- [x] Add a regression test for a manifest source path that no longer resolves.
- [x] `verify` returns a non-zero result for the broken-link case.
- [x] The failure explains that the source path does not resolve.
- [x] Existing status/verify host behavior remains intact.

### Non-Goals

- Change install, status, or uninstall behavior.
- Change manifest schema.
- Add dependencies.

## 4. Compact Architecture Delta

| Component | Existing behavior | Change | Interface / trust boundary touched |
|---|---|---|---|
| `crates/sldo-install/src/install.rs::verify` | Confirms managed link target text matches manifest source | Also follows the recorded source path and checks `SKILL.md` exists | Local filesystem trust boundary |

### Data Flow Delta

```text
manifest entry -> managed link target -> recorded source path -> metadata/SKILL.md liveness check
```

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | GitHub issue #69, installer verify code, existing host e2e tests |
| Outputs | Focused regression test, stricter verify liveness check, PR |
| Interfaces touched | Existing `sldo-install verify` CLI behavior |
| Files allowed to change | `crates/sldo-install/src/install.rs`; `crates/sldo-install/tests/e2e_agent_host_m1.rs`; `docs/slo/tickets/ticket-69-verify-broken-symlinks.md` |
| Files to read before changing | `crates/sldo-install/src/install.rs`; `crates/sldo-install/tests/e2e_agent_host_m1.rs`; GitHub issue #69 |
| New files allowed | `docs/slo/tickets/ticket-69-verify-broken-symlinks.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Valid existing installs still verify successfully; host roots and manifest paths stay stable |
| Data classification | `Public` |
| Proactive controls in play | OWASP C1 security requirements; C9 audit trail through explicit verification evidence |
| Abuse acceptance scenarios | Broken symlink or stale checkout path must not be reported as healthy |
| Resource bounds introduced/changed | `N/A - one metadata lookup and one SKILL.md file check per manifest entry` |
| Invariants/assertions required | `verify` must only print `ok` after root, managed-link, target-match, source-liveness, and `SKILL.md` checks pass |
| Debugger / inspection expectation | `git diff` and focused test output are sufficient |
| Static analysis gates | `cargo fmt --all -- --check`; `cargo check -p sldo-install`; `cargo clippy -p sldo-install --all-targets -- -D warnings`; focused and file-level tests |
| Reversibility / rollback path | Revert the liveness check and regression test if stricter verify proves too noisy |
| Exemplar code to copy | Existing `test_install_root_escape_refused_on_verify` setup and verify failure assertions |
| Anti-exemplar code not to copy | Do not add broad filesystem scans or alter install planning |
| Refactoring discipline | `N/A - no refactoring performed` |
| AI tolerance contract | `N/A - no AI component` |
| Forbidden shortcuts | Do not silently skip missing sources; do not require `--skills-dir` to equal the original source for verify; do not change manifest schema |

## 6. Implementation Plan

1. Run focused baseline test for existing verify behavior.
2. Add failing regression test for a removed manifest source skill directory.
3. Add source liveness and `SKILL.md` checks before printing `ok`.
4. Run formatter, focused tests, host verify tests, check, and clippy.
5. Update evidence and open PR.

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| Broken manifest source rejected | `empty / degraded state` | Installed Codex skill whose source directory was deleted | `sldo-install --host codex verify` runs | Command exits non-zero and explains the source does not resolve | `test_verify_rejects_broken_manifest_source_link` |
| Existing verify still works | `happy path` | Installed Codex skill with live source | `sldo-install --host codex verify` runs | Command succeeds and reports `codex` | `test_codex_status_and_verify_report_selected_host` |
| Host-root escape remains blocked | `abuse case` | Manifest host is tampered to another host root | `verify` runs | Command exits non-zero before liveness success | `test_install_root_escape_refused_on_verify` |

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `cargo test -p sldo-install --test e2e_agent_host_m1 test_codex_status_and_verify_report_selected_host` | passes | `1 passed` | `pass` | Existing behavior green before edits |
| New tests fail first | `cargo test -p sldo-install --test e2e_agent_host_m1 test_verify_rejects_broken_manifest_source_link` | fails before implementation | failed because `verify` still exited successfully | `pass` | Expected red signal captured |
| Formatter | `cargo fmt --all -- --check` | passes | failed on unrelated pre-existing formatting drift; `rustfmt --check crates/sldo-install/src/install.rs crates/sldo-install/tests/e2e_agent_host_m1.rs` passed | `pass-with-notes` | Avoided formatting unrelated files |
| Typecheck / build | `cargo check -p sldo-install` | passes | passed | `pass` | |
| Static analysis / lint | `cargo clippy -p sldo-install --all-targets -- -D warnings` | passes | failed on unrelated pre-existing lint in `e2e_biz_followup_m5.rs` and `common/claude_runtime.rs`; `cargo clippy -p sldo-install --test e2e_agent_host_m1 -- -D warnings` passed | `pass-with-notes` | Scoped lint evidence for touched test target |
| Unit / BDD tests | `cargo test -p sldo-install --test e2e_agent_host_m1` | passes | `10 passed` | `pass` | |
| Runtime validation | Focused CLI integration tests above | passes | focused broken-source test and full host e2e file passed | `pass` | |
| Dependency / security audit | `N/A - no dependency changes` | documented skip | `N/A` | `pass` | |
| Resource bound / invariant check | Code inspection + focused tests | only one liveness check per entry | one `metadata` check and one `SKILL.md` check per verified entry | `pass` | |
| Compatibility check | `cargo test -p sldo-install --test e2e_agent_host_m1 test_codex_status_and_verify_report_selected_host` | passes | passed before edits and covered again in full e2e file | `pass` | |
| `.gitignore` / artifact cleanup | `git status --short` | no stray artifacts beyond ticket changes | only `install.rs`, `e2e_agent_host_m1.rs`, and this ticket contract changed | `pass` | |

## 9. Workpad / Tracker Updates

Issue #69 will be updated with the branch, plan, validation evidence, and PR link.

## 10. Self-Review Gate

- [x] Did I stay inside the file allow-list?
- [x] Did I write or update BDD tests before production code?
- [x] Did I confirm new tests failed for the right reason before implementing?
- [x] Did I preserve public interfaces unless explicitly allowed to change them?
- [x] Did I add or strengthen assertions/invariants where the contract required them?
- [x] Did I bound new resource growth or document why no bound applies?
- [x] Did I run formatter, typecheck/build, and static analysis?
- [x] Did I remove temporary proof edits, debug output, and placeholder logic?
- [x] Did I record evidence rather than claims?
- [x] Did I update the issue workpad and PR handoff notes?

## 11. Closure Summary

### Completed

- Added a Codex-host regression test that removes an installed skill source directory and expects `verify` to fail.
- Updated `verify` so a manifest entry is not reported `ok` until the recorded source path resolves and contains `SKILL.md`.

### Tests And Validation

- `cargo test -p sldo-install --test e2e_agent_host_m1 test_verify_rejects_broken_manifest_source_link`: failed before implementation, passed after implementation.
- `rustfmt --check crates/sldo-install/src/install.rs crates/sldo-install/tests/e2e_agent_host_m1.rs`: passed.
- `cargo check -p sldo-install`: passed.
- `cargo clippy -p sldo-install --test e2e_agent_host_m1 -- -D warnings`: passed.
- `cargo test -p sldo-install --test e2e_agent_host_m1`: passed, 10 tests.
- `cargo fmt --all -- --check`: blocked by unrelated pre-existing formatting drift outside this ticket.
- `cargo clippy -p sldo-install --all-targets -- -D warnings`: blocked by unrelated pre-existing lint outside this ticket.

### Lessons / Follow-Ups

- Follow-up: repo-wide formatter and clippy drift should be cleaned separately to restore broad gates as reliable PR evidence.

### PR / Issue Links

- Issue: [#69](https://github.com/kerberosmansour/SunLitOrchestra/issues/69)
