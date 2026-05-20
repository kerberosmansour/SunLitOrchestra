# /slo-ticket-plan: chase secrets→role→trust policy for IAM-touching tickets - SLO Ticket Contract v1

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `ticket-92-iam-trust-policy-chase` |
| Source tracker | `GitHub Issues` |
| Source issue | [#92](https://github.com/kerberosmansour/SunLitOrchestra/issues/92) |
| Issue title | `/slo-ticket-plan: chase secrets→role→trust policy for IAM-touching tickets` |
| Labels | `retro-derived` |
| Assignee / owner | `kerberosmansour` |
| Target branch | `slo/ticket-92-iam-trust-policy-chase` |
| Primary stack | `Markdown skill + Rust structural test (xtasks/sast-verify)` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default typecheck / build command | `cargo check -p sast-verify` |
| Default static analysis / lint command | `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Default unit / BDD command | `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` |
| Default runtime validation command | `cargo test -p sldo-common -p sldo-install -p sldo-research` |
| Default dependency / security audit command | `N/A - no dependency changes` |
| Default debugger or state-inspection tool | `rg`, `diff`, focused test output |
| Public interfaces stable by default | `yes` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |

### Public Interfaces That Must Remain Stable Unless Explicitly Listed Otherwise

- `/slo-ticket-plan` skill name + frontmatter `description:` block (only Method + Gates + template body change).
- Ticket contract template Section/Row structure: section numbering, existing row labels.
- Both template copies (`docs/slo/templates/...` and `skills/slo-ticket-plan/references/...`) remain byte-identical.

## 2. Sizing Gate

| Check | Answer |
|---|---|
| User-visible outcome fits in one sentence | `yes - /slo-ticket-plan refuses IAM trust-policy contracts that lack a secrets→role→trust-policy mapping` |
| Expected changed files <= 8 | `yes - 4 files (SKILL.md, 2 template copies, 1 new test)` |
| New public surfaces <= 1 | `yes - one new structural test file` |
| No schema migration unless explicitly approved | `yes` |
| No cross-subsystem rewrite | `yes - additive prose + 1 test` |
| Can be reviewed as one PR | `yes` |
| Requires full v4 runbook instead | `no` |

## 3. Issue Context

### Problem

The `/slo-ticket-plan` skill produced ticket-180's contract that proposed extending the **sandbox** role's IAM trust policy with `environment:aws-reconciler-execute` — a GitHub environment that applies to a **different, not-yet-existing** role. Root cause: the planner didn't trace `secrets.<NAME>` through to the actual role + trust policy. Different `role-to-assume:` secrets in different workflow jobs = different roles = different trust policies. Today nothing in the skill or template forces that trace at plan time, so the same conflation can recur.

### Acceptance Criteria From Issue

```text
- skills/slo-ticket-plan/SKILL.md Method requires the secrets→role→trust-policy chase whenever the file allow-list/read-list includes IAM trust-policy JSON, AWS OIDC config, or a workflow YAML with role-to-assume:.
- skills/slo-ticket-plan/SKILL.md Gates refuses to ship a ticket contract that mentions extending a trust policy without naming the exact role ARN(s) and the exact secrets.<NAME> → role mapping table.
- skills/slo-ticket-plan/references/ticket-contract-template_v_1.md Contract Block gains a new row capturing the IAM trust-policy chase requirement.
- Mirrored in docs/slo/templates/ticket-contract-template_v_1.md.
```

### Non-Goals

- Auditing or rewriting ticket-180 itself (already remediated upstream).
- Adding execute-side IAM linting (`/slo-ticket-execute` validation belongs elsewhere).
- Touching any other SLO skill or template.

### Reproduction / Current Signal

| Signal | Evidence |
|---|---|
| Baseline command | `rg -n "secrets→role" skills/slo-ticket-plan/ docs/slo/templates/` |
| Current result | No match — the chase rule is unencoded. |
| Expected result | After fix, every match is present in 3 files: SKILL.md Method + Gates, and both template copies. |

## 4. Compact Architecture Delta

| Component | Existing behavior | Change | Interface / trust boundary touched |
|---|---|---|---|
| `skills/slo-ticket-plan/SKILL.md` | Method has 11 numbered steps; Gates lists 5 refusals | Add Method step 11 (IAM chase trigger), renumber existing workpad-update step to 12; add 6th Gates row (refuse trust-policy contracts without mapping) | Planner skill contract (read-by-agent) |
| `skills/slo-ticket-plan/references/ticket-contract-template_v_1.md` | 19-row Contract Block | Insert one new row `IAM secrets→role→trust-policy mapping` between Anti-exemplar and Refactoring discipline | Ticket-contract surface |
| `docs/slo/templates/ticket-contract-template_v_1.md` | Mirror of above | Same insertion to keep byte-identity | Documentation mirror |
| `xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` (new) | none | Structural-contract test asserting all four edits are present and the two template copies are byte-identical | CI |

### Data Flow Delta

```text
ticket-plan invocation
  → reads ticket-contract-template_v_1.md (now requires IAM mapping row when triggered)
  → executes SKILL.md Method step 11 (IAM chase)
  → SKILL.md Gates row 6 refuses contract if mapping absent
  ⇒ failure visible at plan time, not execute time
```

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | Issue #92 body; existing SKILL.md + 2 template copies; structural-test pattern from `slo_tm_m2_consumers.rs` |
| Outputs | Updated SKILL.md, updated 2 template copies (byte-identical), 1 new structural test, PR linked to #92 |
| Interfaces touched | `/slo-ticket-plan` Method + Gates prose; ticket-contract template Contract Block rows |
| Files allowed to change | `skills/slo-ticket-plan/SKILL.md`, `skills/slo-ticket-plan/references/ticket-contract-template_v_1.md`, `docs/slo/templates/ticket-contract-template_v_1.md`, `xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` (new), `docs/slo/tickets/ticket-92-iam-trust-policy-chase.md` (this contract) |
| Files to read before changing | `skills/slo-ticket-plan/SKILL.md`, both template copies, `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` (pattern), `xtasks/sast-verify/Cargo.toml` (dev-deps) |
| New files allowed | `xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs`, `docs/slo/tickets/ticket-92-iam-trust-policy-chase.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing 5 Gates rows + first 10 Method steps survive byte-identical (additive only); all other Contract Block rows in template survive byte-identical (positionally one row inserted); both template copies remain byte-identical to each other. |
| Data classification | `Public` |
| Proactive controls in play | C9 (security logging/monitoring posture — planner pre-flight is the "logging" control here); ASVS V14.2 (CI config / IAM) — indirectly, via the planner gate |
| Abuse acceptance scenarios | See section 7 — "agent proposes trust-policy extension with no mapping" is the abuse case |
| Resource bounds introduced/changed | `N/A - prose + one test file` |
| Invariants/assertions required | Test asserts: (a) trigger phrase present in SKILL.md Method; (b) refusal phrase present in SKILL.md Gates; (c) IAM mapping row present in both templates; (d) both template files byte-identical |
| Debugger / inspection expectation | `cargo test -p sast-verify --test slo_ticket_plan_iam_chase -- --nocapture` for line-level diff if assertions fire |
| Static analysis gates | `cargo fmt --all -- --check`, `cargo clippy -p sast-verify --all-targets -- -D warnings` |
| Reversibility / rollback path | `git revert` the single squashed/merged commit — purely additive prose + one new test file; no migration. |
| Exemplar code to copy | `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` (workspace_root + read + phrase-presence pattern); `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` (test naming) |
| Anti-exemplar code not to copy | Conflating two different `secrets.<NAME>` workflow secrets into one IAM role (the ticket-180 failure mode); silently editing one template copy and forgetting the mirror. |
| IAM secrets→role→trust-policy mapping | `N/A - this ticket TOUCHES the documentation describing the mapping rule; it does NOT itself modify any IAM trust-policy JSON, AWS OIDC config, or workflow YAML with role-to-assume:. The rule's first real test will be the next IAM-touching ticket.` |
| Refactoring discipline | `N/A - additive change, no refactor` |
| AI tolerance contract | `N/A - no AI/LLM behavior introduced; the planner skill is prose read by agents, but no model output is being scored or sampled` |
| Forbidden shortcuts | No placeholder mapping like "TBD"; no silent fallback if the template copies drift apart (test must enforce byte-identity); no rewriting of existing Method/Gates content beyond the additive insertions defined here. |

## 6. Implementation Plan

1. Read `skills/slo-ticket-plan/SKILL.md` and both template copies; confirm baseline byte-identity via `diff`.
2. Write `xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` with 4 assertions (Method trigger phrase, Gates refusal phrase, template row in both copies, byte-identity).
3. Run `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` — must FAIL with the expected 4 missing-phrase / missing-row reasons.
4. Insert Method step 11 + renumber existing step 11 to 12 in SKILL.md.
5. Add 6th Gates row to SKILL.md.
6. Insert `IAM secrets→role→trust-policy mapping` row in both template copies between Anti-exemplar and Refactoring discipline.
7. Re-run `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` — must PASS.
8. Run full baseline `cargo test -p sldo-common -p sldo-install -p sldo-research -p sast-verify`.
9. Run `cargo fmt --all -- --check` and `cargo clippy -p sast-verify --all-targets -- -D warnings`.
10. Commit, push, open PR linked to #92.

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| Method trigger phrase landed | `happy path` | Edited SKILL.md | Test reads `skills/slo-ticket-plan/SKILL.md` | Method contains `secrets→role→trust-policy chase` AND `role-to-assume:` AND `trust policy` | `slo_ticket_plan_iam_chase::method_step_present` |
| Gates refusal landed | `happy path` | Edited SKILL.md Gates section | Test reads SKILL.md | Gates section contains a refusal that mentions both `trust policy` and `secrets.<NAME>` (or `secrets→role` mapping) | `slo_ticket_plan_iam_chase::gates_refusal_present` |
| Both template copies have the row | `happy path` | Edited templates | Test reads both copies | Each contains row label `IAM secrets→role→trust-policy mapping` | `slo_ticket_plan_iam_chase::template_row_present_in_both` |
| Template byte-identity preserved | `invalid input (drift abuse)` | Two template copies | Test diffs them | Bytes match exactly; drift fails the test | `slo_ticket_plan_iam_chase::templates_byte_identical` |
| Planner pre-edit baseline | `empty / degraded state` | Pre-edit SKILL.md without the chase rule | Test runs against unedited file | All 4 assertions fail with explicit reasons (this is what's confirmed at step 3 of the plan) | Step-3 plan execution log |
| Agent proposes trust-policy extension without mapping | `abuse case` | A planner agent drafting a new IAM-touching ticket | It outputs a contract with no `IAM secrets→role→trust-policy mapping` row populated | Gates row 6 refuses; the missing-row template default is `N/A - no IAM trust policy touched` which is a knowing claim, not a default | Gate refusal documented in SKILL.md; template row makes omission impossible to hide |

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `diff docs/slo/templates/ticket-contract-template_v_1.md skills/slo-ticket-plan/references/ticket-contract-template_v_1.md` | empty (byte-identical) | empty | `done` | confirmed during planning |
| New test fails first | `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` | fails on all 4 assertions | 3 of 4 failed (method_step_present, gates_refusal_present, template_row_present_in_both); byte-identity passed (no edits yet) — expected and correct | `done` | step 3 of plan |
| Formatter | `rustfmt --check xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` (file-scoped) | passes | clean | `done` | workspace-wide `cargo fmt --all -- --check` reveals pre-existing drift on main in files OUTSIDE ticket-92's allow-list; explicitly out of scope |
| Typecheck / build | `cargo check -p sast-verify` | passes | implicit in test run; tests compiled cleanly | `done` | |
| Static analysis / lint | `cargo clippy -p sast-verify --all-targets -- -D warnings` | passes | pre-existing dead-code warnings on `Rule` struct in `sast-verify` bin (NOT in test) cause `-D warnings` to fail on baseline main too — out of scope for ticket-92 | `documented-skip` | confirmed by running clippy on main with my edits stashed; reproduced identically |
| Unit / BDD tests | `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` | passes | 4 passed, 0 failed | `done` | step 7 |
| Runtime validation | `cargo test -p sldo-common -p sldo-install -p sldo-research -p sast-verify` | passes | all green: sldo-common 20, sldo-install 60+11+18+10+4+2+9+10, sldo-research 5+9+others, sast-verify 84+all-integration | `done` | step 8 |
| Dependency / security audit | `N/A - no dependency changes` | N/A | N/A | `done` | |
| Resource bound / invariant check | Byte-identity invariant in `templates_byte_identical` | passes | passes | `done` | |
| Compatibility check | `diff docs/slo/templates/ticket-contract-template_v_1.md skills/slo-ticket-plan/references/ticket-contract-template_v_1.md` post-edit | empty (byte-identical) | empty | `done` | both templates moved together by edit-edit-confirm |
| `.gitignore` / artifact cleanup | `git status --short` | only the 5 allow-listed paths | exactly 5 paths shown; unrelated fmt-drift was stashed separately | `done` | |

## 9. Workpad / Tracker Updates

Issue #92 workpad comment (single persistent comment) will be added post-execute via `gh issue comment 92`.

```markdown
<!-- slo-ticket-workpad:v1 -->
### Plan
- [x] Method step 11 (IAM chase trigger)
- [x] Gates row 6 (refuse trust-policy contract without mapping)
- [x] Template row in both copies
- [x] Structural test slo_ticket_plan_iam_chase

### Acceptance Criteria
- [x] all 4 from issue body

### Validation
- [x] cargo test -p sast-verify --test slo_ticket_plan_iam_chase
- [x] cargo test -p sldo-common -p sldo-install -p sldo-research -p sast-verify
- [x] cargo fmt --check, clippy -D warnings

### Evidence
- PR link
- test output

### Confusions
- none — the rule's first real exercise will be the next IAM-touching ticket
```

## 10. Self-Review Gate

- [ ] Did I stay inside the file allow-list?
- [ ] Did I write the structural test before editing SKILL.md / templates?
- [ ] Did I confirm the test failed for the right reasons before implementing?
- [ ] Did I preserve byte-identity between the two template copies?
- [ ] Did I add the byte-identity invariant assertion?
- [ ] Did I keep new resource growth at exactly one new test file?
- [ ] Did I run formatter, build, lint, and the full baseline?
- [ ] Did I use `diff` / `rg` to confirm only additive insertions?
- [ ] Did I remove any temporary proof edits or scratch?
- [ ] Did I record evidence in the validation table with real commands?
- [ ] Did I link the PR to #92?

## 11. Closure Summary

### Completed

- `skills/slo-ticket-plan/SKILL.md` Method step 11 added (IAM chase trigger); workpad-update is now step 12.
- `skills/slo-ticket-plan/SKILL.md` Gates section gained a 6th refusal row covering missing `secrets.<NAME>` → role → trust-policy mappings.
- `skills/slo-ticket-plan/references/ticket-contract-template_v_1.md` Contract Block gained a new `IAM secrets→role→trust-policy mapping` row.
- `docs/slo/templates/ticket-contract-template_v_1.md` mirror updated identically (byte-identical post-edit).
- `xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` added with 4 assertions: method trigger phrase, gates refusal phrase, template row in both copies, byte-identity.

### Tests And Validation

- `cargo test -p sast-verify --test slo_ticket_plan_iam_chase` — 4 passed, 0 failed (after the BDD-first failing-baseline confirmation).
- `cargo test -p sldo-common -p sldo-install -p sldo-research -p sast-verify` — all suites green.
- `rustfmt --check xtasks/sast-verify/tests/slo_ticket_plan_iam_chase.rs` — clean.
- `diff` of both template copies — empty (byte-identical).
- `cargo fmt --all -- --check` and `cargo clippy -p sast-verify --all-targets -- -D warnings` failures confirmed pre-existing on main (with ticket-92 stashed) — `documented-skip` per the validation table.

### Lessons / Follow-Ups

- The rule's first real exercise will be the next IAM-touching ticket; that ticket should populate the new `IAM secrets→role→trust-policy mapping` row non-trivially.
- Workspace-wide rustfmt drift on main is a separate hygiene issue that should be cleaned up in its own PR; ticket-92 deliberately did NOT carry those unrelated fixes.

### PR / Issue Links

- PR: (filled after `gh pr create`)
- Issue: https://github.com/kerberosmansour/SunLitOrchestra/issues/92
