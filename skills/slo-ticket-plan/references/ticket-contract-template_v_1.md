# [Ticket Title] - SLO Ticket Contract v1

> **Purpose**: Execute one issue-sized change with v4 SLO rigor, without requiring a full multi-milestone runbook.
> **Audience**: AI coding agents first, humans second.
> **Source template**: Derived from `docs/slo/templates/runbook-template_v_4_template.md`. Use the full v4 runbook when this contract cannot stay issue-sized.

---

## 1. Ticket Metadata

| Field | Value |
|---|---|
| Ticket Contract ID | `[ticket-<issue-number>-<slug>]` |
| Source tracker | `GitHub Issues` |
| Source issue | `[#<number>](<url>)` |
| Issue title | `[title]` |
| Labels | `[labels]` |
| Assignee / owner | `[owner]` |
| Target branch | `[slo/ticket-<issue-number>-<slug>]` |
| Primary stack | `[e.g., Rust, TypeScript, Python]` |
| Default formatter command | `[command]` |
| Default typecheck / build command | `[command]` |
| Default static analysis / lint command | `[command]` |
| Default unit / BDD command | `[command]` |
| Default runtime validation command | `[command or N/A]` |
| Default dependency / security audit command | `[command or N/A]` |
| Default debugger or state-inspection tool | `[debugger / command / N/A with reason]` |
| Public interfaces stable by default | `yes` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |

### Public interfaces that must remain stable unless explicitly listed otherwise

- `[API / command / event / route / public type / state file / config key]`

---

## 2. Sizing Gate

| Check | Answer |
|---|---|
| User-visible outcome fits in one sentence | `[yes/no]` |
| Expected changed files <= 8 | `[yes/no]` |
| New public surfaces <= 1 | `[yes/no]` |
| No schema migration unless explicitly approved | `[yes/no]` |
| No cross-subsystem rewrite | `[yes/no]` |
| Can be reviewed as one PR | `[yes/no]` |
| Requires full v4 runbook instead | `[yes/no + reason]` |

If any answer means this is not bite-sized, stop and escalate to `/slo-plan` with the full v4 runbook template.

---

## 3. Issue Context

### Problem

[Restate the issue in concrete, testable language. Quote user-supplied issue text only in fenced blocks.]

### Acceptance Criteria From Issue

- [ ] `[criterion copied or normalized from issue]`
- [ ] `[criterion]`

### Non-Goals

- `[explicitly out of scope]`

### Reproduction / Current Signal

| Signal | Evidence |
|---|---|
| Baseline command / UI path / failing test | `[command, screenshot path, or observation]` |
| Current result | `[what happens today]` |
| Expected result | `[what should happen]` |

---

## 4. Compact Architecture Delta

For docs-only or test-only work, write `N/A - no architecture delta` with a one-line reason.

| Component | Existing behavior | Change | Interface / trust boundary touched |
|---|---|---|---|
| `[component]` | `[today]` | `[planned]` | `[interface or N/A]` |

### Data Flow Delta

```text
[ASCII or short text. Show only the changed flow, not the whole system.]
```

---

## 5. Contract Block

| Contract Row | Value |
|---|---|
| Inputs | `[issue, files, APIs, fixtures]` |
| Outputs | `[code, tests, docs, PR]` |
| Interfaces touched | `[public interfaces or N/A]` |
| Files allowed to change | `[exact allow-list]` |
| Files to read before changing | `[exact read-list]` |
| New files allowed | `[paths or none]` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `[what must keep working]` |
| Data classification | `Public` \| `Internal` \| `Confidential` \| `Restricted` |
| Proactive controls in play | `[OWASP C1-C10 / stack-specific control names / N/A with reason]` |
| Abuse acceptance scenarios | `[BDD rows below or N/A - no new surface introduced, because ...]` |
| Resource bounds introduced/changed | `[queue/cache/list/retry/concurrency bound or N/A]` |
| Invariants/assertions required | `[assertions, typed states, pre/postconditions or N/A]` |
| Debugger / inspection expectation | `[when debugger/state inspection is required, or N/A with reason]` |
| Static analysis gates | `[formatter, typecheck, lint/static analyzer, audit if deps changed]` |
| Reversibility / rollback path | `[rollback/migration path for hard-to-change decisions, or N/A - additive/simple change]` |
| Exemplar code to copy | `[paths/patterns to follow, or N/A - docs-only/no brownfield exemplar]` |
| Anti-exemplar code not to copy | `[risky paths/patterns to avoid, or N/A - no anti-exemplar identified, because ...]` |
| IAM secrets→role→trust-policy mapping | `[REQUIRED when the allow-list or read-list includes IAM trust-policy JSON, AWS OIDC config, or a workflow YAML with role-to-assume:. List every secrets.<NAME> referenced in the touched workflow(s), the IAM role ARN/name each resolves to, and the trust policy attached to that role. Different role-to-assume: secrets = different roles = different trust policies — do not conflate.]` \| `N/A - no IAM trust policy touched` |
| Refactoring discipline | `[pre-test/microstep/post-test proof if refactor permitted, or N/A - no refactoring performed]` |
| AI tolerance contract | `[accepted variance, deterministic boundary, eval evidence, bounded sample budget, or N/A - no AI component]` |
| Forbidden shortcuts | `[no placeholder logic, no silent fallback, no broad refactor, ...]` |

---

## 6. Implementation Plan

Keep this to 10 steps or fewer.

1. `[read/orient step]`
2. `[write failing test step]`
3. `[smallest implementation step]`
4. `[verification step]`

---

## 7. BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `[name]` | `happy path` | `[state]` | `[action]` | `[observable result]` | `[test/runtime check]` |
| `[name]` | `invalid input` | `[state]` | `[action]` | `[structured failure]` | `[test/runtime check]` |
| `[name]` | `empty / degraded state` | `[state]` | `[action]` | `[observable result]` | `[test/runtime check]` |
| `[name]` | `abuse case or N/A` | `[attacker role / state]` | `[step]` | `[blocked outcome]` | `[test/runtime check]` |

---

## 8. Validation Plan

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Baseline before change | `[command]` | `[green or known failure captured]` | | `pending` | |
| New tests fail first | `[command]` | `[fails for expected reason]` | | `pending` | |
| Formatter | `[command]` | `passes` | | `pending` | |
| Typecheck / build | `[command]` | `passes` | | `pending` | |
| Static analysis / lint | `[command]` | `passes` | | `pending` | |
| Unit / BDD tests | `[command]` | `passes` | | `pending` | |
| Runtime validation | `[command/action]` | `passes or N/A` | | `pending` | |
| Dependency / security audit | `[command or N/A]` | `passes or documented skip` | | `pending` | |
| Resource bound / invariant check | `[test/assertion]` | `passes or N/A` | | `pending` | |
| Compatibility check | `[command/action]` | `passes` | | `pending` | |
| `.gitignore` / artifact cleanup | `git status --short` | `no stray artifacts` | | `pending` | |

---

## 9. Workpad / Tracker Updates

Use one persistent issue comment as the workpad when tracker writes are available.

### Workpad Shape

```markdown
<!-- slo-ticket-workpad:v1 -->
### Plan
- [ ] ...

### Acceptance Criteria
- [ ] ...

### Validation
- [ ] ...

### Evidence
- ...

### Confusions
- ...
```

---

## 10. Self-Review Gate

- [ ] Did I stay inside the file allow-list?
- [ ] Did I write or update BDD tests before production code?
- [ ] Did I confirm new tests failed for the right reason before implementing?
- [ ] Did I preserve public interfaces unless explicitly allowed to change them?
- [ ] Did I add or strengthen assertions/invariants where the contract required them?
- [ ] Did I bound new resource growth or document why no bound applies?
- [ ] Did I run formatter, typecheck/build, and static analysis?
- [ ] Did I use a debugger or state-inspection tool when failure evidence was ambiguous?
- [ ] Did I remove temporary proof edits, debug output, and placeholder logic?
- [ ] Did I record evidence rather than claims?
- [ ] Did I update the issue workpad and PR handoff notes?

---

## 11. Closure Summary

### Completed

- `[what changed]`

### Tests And Validation

- `[commands/actions with results]`

### Lessons / Follow-Ups

- `[lesson or N/A with reason]`

### PR / Issue Links

- PR: `[url]`
- Issue: `[url]`
