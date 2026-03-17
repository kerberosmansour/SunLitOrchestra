# [Runbook Title] — [Project Name] (AI-First Runbook v2)

> **Purpose**: [One-sentence description of what this runbook accomplishes end-to-end.]  
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.  
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [README.md](../README.md), [relevant design docs]

---

## Runbook Metadata

- **Runbook ID**: `[short-id]`
- **Prefix for test files and lessons files**: `[prefix]`
- **Primary stack**: `[e.g. Rust + Tauri + React + TypeScript]`
- **Primary package/app names**: `[package names]`
- **Default test commands**:
  - Backend: `[command]`
  - Frontend: `[command]`
  - E2E backend: `[command]`
  - E2E frontend: `[command]`
  - Build/boot: `[command]`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `[API/command/event/state file/UI route/public type]`
  - `[API/command/event/state file/UI route/public type]`

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | [Milestone 1 title] | `not_started` | | | | |
| 2 | [Milestone 2 title] | `not_started` | | | | |
| 3 | [Milestone 3 title] | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/<prefix>-m<N>.md -->
<!-- Completion summaries go in docs/completion/<prefix>-m<N>.md -->

---

## Global Execution Rules

These rules apply to every milestone without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone unless a listed step explicitly requires one additional file.
- Do not refactor unrelated code.
- Do not rename public APIs, commands, routes, events, persisted state shapes, or config keys unless the milestone explicitly says so.
- Do not introduce a new dependency unless the milestone explicitly allows it.
- Do not change database schema, file formats, or migration behavior unless the milestone explicitly includes migration work and migration tests.

### 2) Tests define the contract

- Write BDD tests before production code.
- Write E2E runtime validation stubs before production code.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when code compiles. It is done when the declared contract is satisfied and evidence is recorded.

### 3) No placeholders in production paths

The following are not allowed unless explicitly permitted in the milestone:

- TODO or placeholder logic in production code
- silent fallbacks that hide errors
- swallowed errors without structured logging or user-visible handling
- fake implementations left in place after tests pass
- commented-out dead code
- temporary mocks in production paths
- hard-coded secrets, test keys, or unsafe defaults

### 4) Preserve backwards compatibility

Every milestone must explicitly verify that previously working user flows, commands, routes, persisted state, and public interfaces still work unless the milestone explicitly replaces them.

### 5) Prefer smallest safe change

- Prefer narrow, local modifications over broad rewrites.
- Prefer extending existing patterns over inventing new abstractions.
- Prefer deleting complexity over adding new layers.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

All meaningful checks must be recorded in the milestone Evidence Log:

- command run
- relevant file or test
- expected result
- actual result
- pass/fail
- notes

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, test strategy improvements, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   [backend test command]
   [frontend test command]
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create BDD test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   [backend test command]
   [frontend test command]
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   [backend E2E test command]
   [frontend E2E test command]
   ```
3. Verify the app builds and boots to a usable state.
   ```
   [build/boot commands]
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. Update ARCHITECTURE.md following the Documentation Update Table.
8. Update README.md if user-facing capabilities changed.
9. Write a lessons-learned file at `docs/lessons/<prefix>-m<N>.md`.
10. Write a completion summary at `docs/completion/<prefix>-m<N>.md`.
11. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
12. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

[Describe the current state of the system. What exists today? What works? List major subsystems and their capabilities. Be specific — reference file paths, module names, major entry points, and concrete data where relevant.]

### Problem

[List the specific gaps this runbook addresses. Number each gap and describe it concretely — reference specific code, UI behavior, test gaps, and user impact. Avoid vague generalities.]

1. **[Gap title]**: [Description referencing concrete code and behavior.]
2. **[Gap title]**: [Description.]

### Target Architecture

```
[ASCII diagram or description of the target end state after all milestones are complete.
Show major components, data flow, boundaries, persistence, and integration points.]
```

### Key Design Principles

These are system-wide rules the AI agent must follow when making implementation decisions.

1. **[Principle name]**: [Explanation.]
2. **[Principle name]**: [Explanation.]
3. **[Principle name]**: [Explanation.]

### What to Keep

Explicitly list existing subsystems, patterns, and code that must not be changed or broken.

- [Subsystem / module / pattern to preserve]
- [Subsystem / module / pattern to preserve]

### What to Change

List the specific files, modules, or behaviors that will be modified across milestones.

- **[File or module]** — [summary of change]
- **[File or module]** — [summary of change]

### Global Red Lines

These are forbidden unless explicitly overridden inside a milestone.

- No unrelated refactors
- No new dependencies
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first.
3. Confirm the tests fail for the expected reason.
4. Write production code to make the tests pass.
5. Re-run tests after any refactor.

### Required Test Coverage Categories

Every milestone must explicitly cover the categories that apply:

- happy path
- invalid input
- empty state / first-run state
- dependency failure / partial failure
- retry or rollback behavior if relevant
- concurrency or race behavior if relevant
- persistence / restore behavior if relevant
- backward compatibility behavior

If a category does not apply, state why.

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition]
    // When: [action]
    // Then: [expected outcome]
}
```

```typescript
it("descriptive test name", () => {
  // Given: [precondition]
  // When: [action]
  // Then: [expected outcome]
});
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Backend unit tests | `#[cfg(test)] mod tests` inside the source file | Same file as production code |
| Backend integration/BDD tests | `tests/<prefix>_<feature>.rs` | `src-tauri/tests/` (or equivalent) |
| Frontend unit tests | `<module>.test.ts` | Co-located with source file |
| Frontend page tests | `<Page>.test.tsx` | Co-located with component |
| Scenario/e2e tests | `tests/scenarios/<prefix>_scenario_<name>.rs` | `src-tauri/tests/scenarios/` (or equivalent) |
| E2E runtime validation (backend) | `tests/e2e_<prefix>_m<N>.rs` | `src-tauri/tests/` (or equivalent) |
| E2E runtime validation (frontend) | `e2e/<feature>.e2e.test.tsx` | `src/e2e/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go beyond compilation and verify that the system works correctly at runtime. These tests prove:

1. the app boots without errors
2. runtime contracts are met across IPC/API boundaries
3. BDD scenarios work at runtime, not just in isolation
4. there are no runtime panics, unhandled rejections, or silent failures
5. degraded states behave safely and visibly

### E2E Test Design Rules

1. Test runtime behavior, not just types.
2. Test the full stack where possible.
3. Test degraded and failure states, not just the happy path.
4. Assert against observable behavior.
5. Prefer at least one test that crosses the backend/frontend boundary when both layers changed.

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

A new dependency is allowed only if the milestone explicitly includes:

- package/crate name
- why existing dependencies are insufficient
- security and maintenance rationale
- build/runtime cost rationale
- tests covering the new integration

### Migration policy

Any schema, config, or persisted-state change requires:

- migration plan
- backward compatibility strategy
- migration tests
- rollback strategy if relevant
- documentation updates

### Refactor budget

Each milestone must state one of the following:

- `No refactor permitted beyond direct implementation`
- `Minimal local refactor permitted in listed files only`
- `Targeted refactor permitted for [specific reason]`

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `[command]` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `[command]` | green | | | |
| E2E runtime | `[command]` | green | | | |
| Build/boot | `[command]` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces and compatibility requirements?
- Did I add tests for failure modes, not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update documentation to match the implementation?
- Is every assumption either verified or explicitly documented as unresolved?
- Is the milestone truly done according to its Definition of Done?

If any answer is "no", the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/lessons/<prefix>-m<N>.md`

```md
# Lessons Learned — <prefix> Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Naming conventions established
- [types, files, tests, events, commands]

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

## Completion Summary Template

Path: `docs/completion/<prefix>-m<N>.md`

```md
# Completion Summary — <prefix> Milestone <N>

## Goal completed
- [what capability now exists]

## Files changed
- [file]
- [file]

## Tests added
- [test file]
- [test file]

## Runtime validations added
- [e2e file]

## Compatibility checks performed
- [check]

## Documentation updated
- [doc and section]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## Milestone Plan

<!-- Copy the milestone template below for each milestone. -->

### Milestone N — [Title]

**Goal**: [One-sentence description of what this milestone accomplishes. What capability exists at the end that did not exist before?]

**Context**: [2–4 sentences describing the current state relevant to this milestone. Reference specific files, comments, interfaces, and why this change is needed.]

**Important design rule**: [One key design decision that must guide implementation.]

**Refactor budget**: `[No refactor permitted beyond direct implementation | Minimal local refactor permitted in listed files only | Targeted refactor permitted for ...]`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | [user input, command input, event input, state input] |
| Outputs | [UI state, return values, persisted state, events] |
| Interfaces touched | [commands, APIs, routes, events, structs, files] |
| Files allowed to change | [explicit list] |
| Files to read before changing anything | [explicit list] |
| New files allowed | [explicit list or `none`] |
| New dependencies allowed | [explicit list or `none`] |
| Migration allowed | [`yes` or `no`] |
| Compatibility commitments | [what must still work] |
| Forbidden shortcuts | [mocks in prod, TODOs, silent fallbacks, broad refactor, etc.] |

#### Out of Scope / Must Not Do

- [Explicit non-goal]
- [Explicit non-goal]
- [Explicit non-goal]

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/<prefix>-m<N-1>.md` and apply relevant corrections.
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `[existing file path]` | [summary of change] |
| `[new file path if allowed]` | NEW: [what this file does] |

#### Step-by-Step

1. Write BDD test stubs first for all scenarios below.
2. Write E2E runtime validation stubs first for all tests below.
3. Implement the smallest safe change that satisfies the contract.
4. Make all BDD tests pass.
5. Run the full test suite.
6. Run E2E runtime validation.
7. Run smoke tests.
8. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: [feature name]**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| [Scenario name] | happy path | [Precondition] | [Action] | [Expected outcome] |
| [Scenario name] | invalid input | [Precondition] | [Action] | [Expected outcome] |
| [Scenario name] | empty state | [Precondition] | [Action] | [Expected outcome] |
| [Scenario name] | partial failure | [Precondition] | [Action] | [Expected outcome] |

Add more rows as needed. If a category does not apply, state why under Notes.

#### Regression Tests

- [Existing test suite or feature that must still pass]
- [Specific edge case to verify]
- [Backward compatibility check]
- [Persistence/config/state compatibility check if relevant]

#### Compatibility Checklist

- [ ] [Public API/command still behaves the same]
- [ ] [Existing route/page still renders correctly]
- [ ] [Persisted state remains readable]
- [ ] [Existing tests for related features still pass]

#### E2E Runtime Validation

**File**: `[backend E2E test file path]`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `[test_function_name]` | [Runtime behavior validated] | [Specific assertion criteria] |
| `[test_function_name]` | [Runtime behavior validated] | [Specific assertion criteria] |

**File**: `[frontend E2E test file path]`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `[test name]` | [Runtime behavior validated] | [Specific assertion criteria] |
| `[test name]` | [Runtime behavior validated] | [Specific assertion criteria] |

#### Smoke Tests

- [ ] [Manual verification step — what to do and what to observe]
- [ ] [Manual verification step]
- [ ] `[test command]` passes
- [ ] App launches without errors

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `[command]` | all green | | | |
| BDD tests created | `[files]` | fail for expected reason | | | |
| E2E stubs created | `[files]` | fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `[command]` | green | | | |
| E2E runtime | `[command]` | green | | | |
| Build/boot | `[command]` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- all listed BDD scenarios pass
- all listed E2E runtime validations pass
- full existing test suite remains green
- smoke tests are checked off
- compatibility checklist is complete
- no forbidden shortcuts remain in production code
- docs are updated to match implementation
- lessons file is written
- completion summary is written
- Milestone Tracker is updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: [What to document]
- **README.md**: [What to update]
- **Other docs**: [What to update]

#### Notes

- [Why certain coverage categories do not apply]
- [Any explicit deferred work for future milestone]

---

<!-- Repeat the “### Milestone N” template for each subsequent milestone. -->

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | ARCHITECTURE.md Update | README.md Update | Other Docs |
|---|---|---|---|
| 1 | [Section to add/update] | [Section to add/update] | [Section/file] |
| 2 | [Section to add/update] | [Section to add/update] | [Section/file] |
| 3 | [Section to add/update] | [Section to add/update] | [Section/file] |

---

## Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

