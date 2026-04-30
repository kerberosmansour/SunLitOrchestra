# AI-First Runbook Template — Carmack-Style Reliability Edition

> **Purpose**: Provide a language-independent runbook template for AI coding agents executing planned software changes with strict reliability, scope, validation, and evidence requirements.  
> **Audience**: AI coding agents first, humans second. The template is designed to reduce ambiguity, suppress scope drift, and force the same code-quality discipline from any capable agent.  
> **Core philosophy**: Prefer automated guardrails over developer intention. Prefer direct inspection over guessing. Prefer executable assumptions over comments. Prefer bounded design over silent growth. Prefer evidence over claims.

---

## 0. How To Use This Template

1. Fill out the metadata, architecture, and milestone plan before implementation starts.
2. Work milestones sequentially.
3. Before each milestone, complete the Global Entry Protocol.
4. During implementation, follow the Carmack-Style Development Best Practices and milestone contract.
5. After each milestone, complete the Global Exit Protocol and Evidence Log.
6. Do not mark a milestone done until the Definition of Done is objectively satisfied.

An AI agent must treat this document as an execution contract, not as guidance that can be loosely interpreted.

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `[short-id]` |
| Project name | `[project]` |
| Primary language/runtime | `[language/runtime]` |
| Primary package/app names | `[package names]` |
| Prefix for tests and lesson files | `[prefix]` |
| Default unit test command | `[command]` |
| Default integration/BDD test command | `[command]` |
| Default E2E/runtime validation command | `[command]` |
| Default build/boot command | `[command]` |
| Default formatter command | `[command]` |
| Default static analysis/lint command | `[command]` |
| Default dependency/security audit command | `[command]` |
| Allowed new dependencies by default | `none` |
| Schema/config migration allowed by default | `no` |
| Public interfaces stable by default | `yes` |

### Public Interfaces That Must Remain Stable Unless Explicitly Changed

- `[API / command / event / route / public type / state file / config key]`
- `[API / command / event / route / public type / state file / config key]`

---

## 2. Milestone Tracker

Update this table as each milestone completes. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `[Milestone title]` | `not_started` | | | | |
| 2 | `[Milestone title]` | `not_started` | | | | |
| 3 | `[Milestone title]` | `not_started` | | | | |

Allowed statuses: `not_started`, `in_progress`, `blocked`, `done`.

---

## 3. End-to-End Architecture Diagram

Provide the target architecture after all milestones are complete.

### Diagram Requirements

- Show all major actors, components, services, and processes.
- Show data flow direction with labeled arrows.
- Show persistence boundaries.
- Show trust boundaries and external integration points.
- Show API, IPC, event, queue, and file boundaries.
- Distinguish existing behavior from new behavior.
- Include a legend.

```text
[Replace with ASCII or Mermaid architecture diagram]

Legend:
── existing
- - new
══ external
▶ data flow
║ trust boundary
```

### Component Summary

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `[component]` | `[responsibility]` | `[existing/new/changed]` | `M[N]` | `[interface]` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| `[flow]` | `[source]` | `[target]` | `[API/event/file/etc.]` | `[yes/no]` | `[behavior]` | `M[N]` |

---

## 4. Carmack-Style Development Best Practices

These rules apply to every language and every milestone.

### 4.1 Inspect State, Do Not Guess

Logging is useful for production observability, but it is not a substitute for interactive debugging and state inspection.

Required per project:

| Requirement | Project-Specific Tool/Command | Evidence Required |
|---|---|---|
| Interactive debugger available | `[debugger / IDE / command]` | `[how verified]` |
| Breakpoints can be set in changed code | `[how]` | `[screenshot/log/note if needed]` |
| Runtime state can be inspected | `[how]` | `[what was inspected]` |
| Tests can be debugged | `[how]` | `[test/debug command]` |

Agent rules:

- If a failure is not explained by compiler, test assertion, or stack trace, use a debugger or equivalent state-inspection tool before making speculative changes.
- Do not add permanent print/debug statements to production paths.
- If logging is added, it must be structured, intentional, and useful in production.
- Remove temporary debug output before completing the milestone.

### 4.2 Static Analysis Is Mandatory

Every milestone must run the project’s static analysis and lint tools. The agent must treat tool findings as design feedback, not personal criticism.

Required checks:

| Check | Command | Required Level | Notes |
|---|---|---|---|
| Formatter | `[formatter command]` | must pass | No style-only churn outside changed files unless allowed |
| Type check / compile check | `[typecheck command]` | must pass | Must include all changed targets |
| Static analyzer / linter | `[lint command]` | must pass | Warnings fail CI unless explicitly waived |
| Security/dependency audit | `[audit command]` | must pass or documented exception | Required if dependency graph changes |

Waiver rule:

- A static-analysis waiver must be local, minimal, and justified in code or the Evidence Log.
- Global disables are forbidden unless explicitly approved in the milestone contract.

### 4.3 Assertions Are Executable Comments

Assertions document and enforce assumptions. They should be used to catch incorrect mental models early.

Use assertions for:

- internal invariants
- unreachable states that should be impossible by design
- size and capacity assumptions
- ordering assumptions
- preconditions inside internal APIs
- postconditions after transformations

Do not use assertions for:

- normal user input validation
- expected network, filesystem, or external service failures
- recoverable business-rule failures

Assertion policy:

| Assertion Type | Use For | Production Behavior |
|---|---|---|
| Development-only assertion | Expensive or diagnostic invariant checks | Disabled or lower-cost in production if language supports it |
| Runtime assertion | Invariants that must never be violated | Active in production |
| Contract validation | Public boundary checks | Return structured errors, not crashes |

### 4.4 Prefer Bounded Resources Over Silent Growth

Unbounded collections, queues, retries, caches, recursion, and concurrency can hide architectural failures until production.

Every milestone must identify newly introduced or modified resource growth.

| Resource | Expected Bound | Hard Limit | Behavior At Limit | Evidence/Test |
|---|---:|---:|---|---|
| `[queue/cache/list/etc.]` | `[N]` | `[N]` | `[reject/backpressure/error]` | `[test]` |

Rules:

- If a maximum is known, encode it.
- If a maximum is not known, document why and add observability around growth.
- Dynamic collections must have explicit expected-size assumptions in tests or assertions.
- Retries must be bounded.
- Queues must have backpressure, rejection, or shedding behavior.
- Caches must have eviction or explicit lifecycle rules.

### 4.5 Make Invalid States Unrepresentable

Use the language’s strongest available mechanisms to encode domain constraints.

Examples by concept:

| Concept | Prefer | Avoid |
|---|---|---|
| Domain IDs | dedicated ID type/value object | raw string/int everywhere |
| State machines | enum/sum type/tagged union/classes with restricted transitions | loose string states |
| Optional data | explicit optional/maybe type | sentinel values |
| Validated strings | constrained constructor | free-form string reuse |
| Units | unit-specific type | raw numbers without unit |
| Protocol messages | schema-validated typed messages | ad hoc maps/dictionaries |

Agent rule: before implementing a feature, identify at least one invalid state the design should prevent. If none exists, state why.

### 4.6 Preserve Compatibility Until Explicitly Broken

Compatibility checks are part of correctness.

Must verify:

- public APIs
- CLI/commands/events/routes
- persisted state and migration behavior
- configuration keys and defaults
- user-facing behavior
- integration contracts

A milestone may break compatibility only if the contract block explicitly says so and includes migration, documentation, and tests.

### 4.7 Prefer Small, Local, Reviewable Changes

The agent must optimize for minimal safe change.

Required behavior:

- Change only allowed files.
- Prefer extending existing patterns.
- Do not rewrite subsystems unless the milestone contract explicitly permits it.
- Do not rename public symbols for style reasons.
- Do not combine refactor and feature work unless the refactor is required and listed.

### 4.8 No Silent Failure

The following are forbidden in production paths unless explicitly permitted:

- swallowed exceptions/errors
- silent fallbacks that hide broken behavior
- default values that mask corruption
- fake implementations after tests pass
- temporary mocks in real code paths
- TODO placeholder logic
- commented-out dead code
- hard-coded secrets or unsafe defaults

All failure modes must be visible through one or more of:

- returned structured error
- user-visible error state
- structured log/event/metric
- retry with bounded policy
- explicit degraded-mode behavior

---

## 5. High-Level Design for Formal Verification / State Modeling

Fill this section before implementation when the system includes concurrency, distributed state, resource ownership, ordering guarantees, retries, queues, idempotency, persistence recovery, or irreversible actions.

For simple CRUD with no meaningful concurrency or failure recovery risk, mark `N/A` and explain why.

### 5.1 System Goal

`[Correctness-focused goal, not implementation detail.]`

### 5.2 Main Components

| Component | Protocol Role | Key State | Visible Actions |
|---|---|---|---|
| `[component]` | `[role]` | `[state]` | `[actions]` |

### 5.3 Abstract State

| Variable | Abstract Type | Why Needed | Bound | Explosion Risk |
|---|---|---|---|---|
| `[var]` | `[type]` | `[property]` | `[N]` | `[low/medium/high]` |

### 5.4 Actions / Transitions

| Action | Preconditions | State Updates | Failure / Interleaving Notes |
|---|---|---|---|
| `[action]` | `[preconditions]` | `[updates]` | `[notes]` |

### 5.5 Safety Properties

- **No duplicate ownership**: `[specific invariant]`
- **No lost accepted work**: `[specific invariant]`
- **No invalid persisted state**: `[specific invariant]`
- **Bound never exceeded silently**: `[specific invariant]`

### 5.6 Liveness / Progress Assumptions

- **Eventual completion or visible rejection**: `[fairness assumptions]`
- **Bounded retry exhaustion**: `[fairness assumptions]`

### 5.7 Simplifications

| Simplification | Why It Still Catches Relevant Bugs |
|---|---|
| `[simplification]` | `[reason]` |

---

## 6. Global Execution Rules

These apply to every milestone without exception.

### 6.1 Stay Inside Scope

- Only change files listed in the current milestone unless the milestone explicitly allows one additional file.
- Do not refactor unrelated code.
- Do not rename public APIs, routes, events, commands, persisted state, schemas, or config keys unless explicitly allowed.
- Do not add dependencies unless explicitly allowed.
- Do not change database schema, file formats, or migration behavior unless explicitly allowed.

### 6.2 Tests Define the Contract

- Write BDD tests before production code.
- Write E2E/runtime validation stubs before production code.
- Confirm new tests fail for the expected reason.
- A milestone is complete only when the declared behavior is satisfied and evidence is recorded.

### 6.3 Evidence Over Claims

All meaningful checks must be recorded with:

- command/check run
- file/test involved
- expected result
- actual result
- pass/fail
- notes

The agent must never claim a command passed unless it ran or the limitation is explicitly stated.

### 6.4 Cleanup Is Part of Correctness

- Tests must not leave generated files in the working tree.
- Temporary files must use temp directories or language/framework cleanup hooks.
- `git status` must be clean of test artifacts after the milestone.
- `.gitignore` must be updated for new generated outputs and stale patterns removed.

---

## 7. Global Entry Protocol

Complete this before every milestone.

1. Read the previous milestone’s lessons file, if one exists.
2. Read the current milestone fully.
3. Re-state the milestone constraints in working notes:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - dependency/migration rules
   - required tests
   - Definition of Done
4. Run the full existing baseline test suite.
5. If the baseline is red, stop and fix or report the baseline before feature work.
6. Read all files listed under “Files To Read Before Changing Anything.”
7. Update the Milestone Tracker to `in_progress` and record the start date.
8. Create BDD test files first.
9. Create E2E/runtime validation stubs first.
10. Begin the Evidence Log.

---

## 8. Global Exit Protocol

Complete this after every milestone.

1. Run formatter.
2. Run typecheck/build check.
3. Run static analysis/lints.
4. Run the full test suite.
5. Run milestone E2E/runtime validation.
6. Verify build/boot.
7. Run smoke tests.
8. Verify compatibility checklist.
9. Complete self-review gate.
10. Remove temporary debug code, mocks, placeholders, and dead code.
11. Verify test artifact cleanup with `git status` or project equivalent.
12. Review `.gitignore`.
13. Update architecture documentation.
14. Update README or user-facing docs if behavior changed.
15. Write lessons file.
16. Write completion summary.
17. Update Milestone Tracker to `done`.
18. Re-read next milestone and record assumption changes.

---

## 9. BDD and Runtime Validation Rules

### 9.1 Required Coverage Categories

Every milestone must cover applicable categories:

- happy path
- invalid input
- empty/first-run state
- dependency failure
- partial failure
- retry/rollback behavior
- concurrency/race behavior
- resource limit behavior
- persistence/restore behavior
- backward compatibility behavior

If a category does not apply, state why in the milestone notes.

### 9.2 Scenario Format

```text
Scenario: [name]
Given [precondition]
When [action]
Then [observable outcome]
And [failure/resource/compatibility expectation if relevant]
```

### 9.3 Runtime Validation Requirements

E2E/runtime tests must prove:

- the application starts or the changed component initializes
- runtime contracts are met across boundaries
- changed behavior works outside isolated unit tests
- degraded/failure states are safe and visible
- no unhandled exceptions, panics, crashes, or silent failures occur

---

## 10. Dependency, Migration, and Refactor Policy

### 10.1 Dependency Policy

A new dependency is allowed only if the milestone explicitly includes:

- dependency name
- version/range if known
- why existing code/tools are insufficient
- security and maintenance rationale
- license rationale if applicable
- build/runtime cost rationale
- tests covering the integration
- rollback/removal path if the dependency proves unsuitable

### 10.2 Migration Policy

Any schema, config, or persisted-state change requires:

- migration plan
- backward compatibility strategy
- migration tests
- rollback strategy if relevant
- documentation updates
- old-version fixture or compatibility test where possible

### 10.3 Refactor Budget

Each milestone must choose exactly one:

- `No refactor permitted beyond direct implementation`
- `Minimal local refactor permitted in listed files only`
- `Targeted refactor permitted for [specific reason]`

---

## 11. Milestone Template

### Milestone N — `[Title]`

**Goal**: `[One sentence: capability that exists after this milestone.]`

**Context**: `[2–4 sentences with current state, exact files/modules, and why change is needed.]`

**Carmack-style reliability goal**: `[Which guardrail is strengthened: debugger visibility, static analysis, assertions, bounded resources, type/schema safety, compatibility, etc.]`

**Important design rule**: `[One decision that must guide implementation.]`

**Refactor budget**: `[No refactor permitted beyond direct implementation | Minimal local refactor permitted in listed files only | Targeted refactor permitted for ...]`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `[user input / command input / event / state / file]` |
| Outputs | `[UI state / return values / persisted state / events]` |
| Interfaces touched | `[APIs / routes / commands / events / types / files]` |
| Files allowed to change | `[explicit list]` |
| Files to read before changing anything | `[explicit list]` |
| New files allowed | `[explicit list or none]` |
| New dependencies allowed | `[explicit list or none]` |
| Migration allowed | `[yes/no]` |
| Compatibility commitments | `[what must still work]` |
| Resource bounds introduced/changed | `[bounds and behavior at limit]` |
| Invariants/assertions required | `[list]` |
| Debugger/inspection expectation | `[what must be inspectable]` |
| Static analysis gates | `[commands]` |
| Forbidden shortcuts | `[mocks in prod, TODOs, silent fallback, broad refactor, etc.]` |

#### Out of Scope / Must Not Do

- `[explicit non-goal]`
- `[explicit non-goal]`
- `[explicit non-goal]`

#### Pre-Flight

1. Complete Global Entry Protocol.
2. Read previous lessons.
3. Read all allowed files before editing.
4. Copy Evidence Log into working notes.
5. Re-state constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `[path]` | `[change]` |
| `.gitignore` | `Add/remove generated artifact patterns if needed` |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `[name]` | happy path | `[state]` | `[action]` | `[result]` |
| `[name]` | invalid input | `[state]` | `[action]` | `[visible error]` |
| `[name]` | resource bound | `[near limit]` | `[operation]` | `[bounded behavior]` |
| `[name]` | partial failure | `[dependency failure]` | `[operation]` | `[safe visible failure]` |
| `[name]` | compatibility | `[old behavior/state]` | `[operation]` | `[still works]` |

#### Runtime Validation

| Test | Boundary Exercised | What It Proves | Pass Criteria |
|---|---|---|---|
| `[test]` | `[API/IPC/event/file/UI]` | `[runtime behavior]` | `[observable pass]` |

#### Smoke Tests

- [ ] `[manual or automated check]`
- [ ] App/component boots or initializes without error
- [ ] Static analysis passes
- [ ] Full tests pass
- [ ] Runtime validation passes
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` is current

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `[command]` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | fail for expected reason | | | |
| Runtime stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Formatter | `[command]` | clean | | | |
| Typecheck/build check | `[command]` | clean | | | |
| Static analysis | `[command]` | clean | | | |
| Full tests | `[command]` | green | | | |
| Runtime validation | `[command]` | green | | | |
| Build/boot | `[command]` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Compatibility checks | `[checks]` | no regressions | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | current, no stale entries | | | |

#### Self-Review Gate

Before marking done, answer every question:

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve public interfaces unless explicitly changed?
- Did I add tests for failure modes, not just happy paths?
- Did I add or update assertions/invariants where assumptions matter?
- Did I bound new resource growth or document why it cannot be bounded?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update docs to match implementation?
- Is every assumption verified or explicitly documented as unresolved?
- Do all tests clean up artifacts?
- Is `.gitignore` current?
- Is the milestone done according to its Definition of Done?

If any answer is “no,” the milestone is not complete.

#### Definition of Done

- All BDD scenarios pass.
- All E2E/runtime validations pass.
- Full existing test suite remains green.
- Formatter and static analysis pass.
- Smoke tests are checked off.
- Compatibility checklist is complete.
- No forbidden shortcuts remain.
- Resource bounds and invariant checks are implemented or explicitly justified.
- `git status` shows no untracked test artifacts.
- `.gitignore` is up to date.
- Documentation is updated.
- Lessons file is written.
- Completion summary is written.
- Milestone Tracker is updated.

---

## 12. Lessons-Learned File Template

Path: `docs/lessons/<prefix>-m<N>.md`

```md
# Lessons Learned — <prefix> Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Assumptions that were verified
- [assumption] — [evidence]

## Assumptions that remain unresolved
- [assumption] — [risk / follow-up]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Invariants/assertions added
- [invariant]

## Resource bounds established
- [bound]

## Debugging/inspection notes
- [what was inspected]

## Test patterns that worked well
- [pattern]

## Missing tests that should exist now
- [test]

## Rules for the next milestone
- [rule]

## Template improvements suggested
- [improvement]
```

---

## 13. Completion Summary Template

Path: `docs/completion/<prefix>-m<N>.md`

```md
# Completion Summary — <prefix> Milestone <N>

## Goal completed
- [capability now exists]

## Files changed
- [file]

## Tests added
- [test]

## Runtime validations added
- [e2e/runtime file]

## Static analysis and formatter evidence
- [command and result]

## Compatibility checks performed
- [check]

## Invariants/assertions added
- [invariant]

## Resource bounds added or verified
- [bound]

## Documentation updated
- [doc and section]

## .gitignore changes
- [patterns added/removed]

## Test artifact cleanup verified
- [confirmation]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## 14. Optional Fast-Fail Prompt for AI Agents

Use before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, dependency/migration rules, required tests, required runtime validation, resource bounds, invariants/assertions, static-analysis gates, and Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## 15. Source Basis

This template adapts the AI-first runbook structure provided by the user and adds language-independent Carmack-style reliability controls: debugger-first inspection, static-analysis enforcement, assertion-driven invariants, bounded resource design, type/schema guardrails, and stricter evidence capture.
