# [Runbook Title] — [Project Name]

> **Purpose**: [One-sentence description of what this runbook accomplishes end-to-end.]  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section **and** the Pre-Milestone Protocol. After completing it, follow the Post-Milestone Protocol. Never skip ahead.  
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md)

---

## Milestone Tracker

Update this table as each milestone is completed. This is the **single source of truth** for progress.

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | [Milestone 1 title] | `not_started` | | | |
| 2 | [Milestone 2 title] | `not_started` | | | |
| 3 | [Milestone 3 title] | `not_started` | | | |

<!-- Status values: not_started | in_progress | done -->
<!-- Lessons files go in docs/lessons/<prefix>-m<N>.md -->

---

## Pre-Milestone Protocol

**Do this before every milestone — no exceptions.**

1. **Read the lessons file from the previous milestone** (if one exists). Its path is in the Milestone Tracker table. Apply any design corrections, naming changes, or test strategy improvements it calls for before writing new code.
2. **Read the current milestone section fully** — goal, context, change set, BDD scenarios, regression tests, and smoke tests — before writing any code.
3. **Run the full existing test suite** and confirm it passes. Record the baseline:
   ```
   [backend test command, e.g.: cargo test --workspace 2>&1 | tail -5]
   [frontend test command, e.g.: npx vitest run 2>&1 | tail -10]
   ```
   If any tests fail before you start, **stop and fix them first**. Do not begin a milestone on a red baseline.
4. **Read the files listed in "Files Most Likely Touched"** for the current milestone. Understand their current shape before changing them.
5. **Update the Milestone Tracker** in this file: set the current milestone's Status to `in_progress` and record the Started date.
6. **Create BDD test files first** — write the scenario tests from the acceptance table **before** writing production code. Tests declare the contract, then implementation satisfies it.
7. **Create E2E test stubs** — write the end-to-end runtime validation tests from the milestone's "E2E Runtime Validation" section as stubs before writing production code. These test that the system works at runtime, not just that it compiles.

---

## Post-Milestone Protocol

**Do this after every milestone — no exceptions.**

1. **Run the full test suite** (backend and frontend). Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   [backend test command]
   [frontend test command]
   ```
2. **Run the E2E runtime validation tests** for this milestone:
   ```
   [backend E2E test command, e.g.: cargo test -p <package> --test e2e_m*]
   [frontend E2E test command, e.g.: npx vitest run --include 'src/e2e/**']
   ```
3. **Verify the app builds and boots** — frontend compiles, backend compiles, and the app launches to a usable state:
   ```
   [build commands, e.g.: npm run build && cargo build -p <package>]
   ```
4. **Run the smoke tests** listed in the milestone. Check off each item in this runbook file.
5. **Verify backward compatibility**: [list existing features/commands/state files that must still function].
6. **Update ARCHITECTURE.md** following the Documentation Update table at the bottom of this runbook. Follow any documentation rules in the "Guidelines for AI Agents" section of ARCHITECTURE.md.
7. **Update README.md** if user-facing capabilities changed, following the Documentation Update table.
8. **Write a lessons-learned file** at `docs/lessons/<prefix>-m<N>.md` containing:
   - What design decisions were made and why
   - What was harder than expected
   - What naming conventions were established (type names, file names, test patterns)
   - What test patterns worked well or didn't
   - What the next milestone should do differently based on what was learned
   - Any BDD scenarios that should be retroactively added to earlier milestones
9. **Update the Milestone Tracker** in this file: set Status to `done`, record the Completed date, and fill in the Lessons File path.
10. **Re-read the next milestone's section** with fresh eyes, and note in the lessons file whether any of its assumptions need to change.

---

## Background Context

### Current State

[Describe the current state of the system. What exists today? What works? List major subsystems and their capabilities. Be specific — reference file paths, module names, and concrete data (e.g., "535+ canonical software records").]

### Problem

[List the specific gaps this runbook addresses. Number each gap and describe it concretely — reference specific code (file, line, comment), UI behavior, and user impact. Avoid vague generalities.]

1. **[Gap title]**: [Description referencing specific code and behavior.]
2. **[Gap title]**: [Description.]

### Target Architecture

```
[ASCII diagram or description of the target end state after all milestones are complete.
Show major components, data flow, and integration points.]
```

### Key Design Principles

[List the overarching design principles that apply across all milestones. These are the rules the AI agent must follow when making implementation decisions.]

1. **[Principle name]**: [Explanation.]
2. **[Principle name]**: [Explanation.]

### What to Keep

[Explicitly list existing subsystems, patterns, and code that must NOT be changed or broken. This prevents accidental regressions.]

- [Subsystem / module / pattern to preserve]

### What to Change

[List the specific files, modules, or behaviors that will be modified across milestones.]

- **[File or module]** — [summary of change]

---

## BDD Practices

Every milestone follows these rules. Apply them consistently.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first — backend `#[test]` modules for backend scenarios, frontend `describe`/`it` blocks for frontend scenarios.
3. Confirm the tests fail (they reference types/functions that don't exist yet).
4. Write the production code to make the tests pass.
5. Refactor if needed, re-run tests to confirm green.

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

Every milestone must include E2E tests that go **beyond compilation** and verify that the system works correctly **at runtime**. These tests prove:

1. **The app boots without errors** — backend initializes, managed state is wired, and the frontend loads without console errors or white screens.
2. **Runtime contracts are met** — types serialize/deserialize correctly over IPC/API boundaries, commands return expected shapes, and events fire with correct payloads.
3. **BDD scenarios work at runtime, not just in isolation** — integration between backend and frontend is tested across service boundaries.
4. **No runtime panics, unhandled rejections, or silent failures** — the app survives real-world usage patterns (navigation, concurrent operations, error conditions).

#### E2E Test Layers

| Layer | What It Proves | How to Run |
|---|---|---|
| **Backend integration E2E** | Backend services wire together, managed state initializes, calls execute through full pipeline, no panics under realistic workloads | `[backend E2E test command]` |
| **Frontend rendering E2E** | Pages render without errors, components mount/unmount cleanly, backend mocks return expected shapes, user interactions produce correct state transitions | `[frontend E2E test command]` |
| **Build-and-boot E2E** | Frontend builds without errors, backend compiles, services initialize in correct order | `[build commands]` |

#### E2E Test Design Rules

1. **Test runtime behavior, not just types**: A serialization round-trip test proves more than a type compiles. A command invocation that returns real data proves more than a mock returning `Ok(())`.
2. **Test the full stack where possible**: When a milestone changes both backend and frontend, write at least one test that exercises the IPC/API boundary.
3. **Test degradation, not just the happy path**: What happens when dependencies are missing? When state has never been initialized? These runtime conditions must not crash the app.
4. **Assert against observable behavior**: Events actually arrive. Cached data actually loads faster. Persisted state actually restores. Redirected routes actually resolve.
5. **Test file naming**: Backend E2E tests use `e2e_<prefix>_m<N>.rs`. Frontend E2E tests live in `src/e2e/` and use `<feature>.e2e.test.tsx`.

#### E2E Assertion Patterns

```rust
// Backend: verify the full pipeline doesn't panic at runtime
#[tokio::test]
async fn app_state_initializes_without_panic() {
    // Given: default configuration (no env vars, no persisted state)
    // When: all managed state objects are created
    // Then: no panic occurs, all state is accessible
}
```

```typescript
// Frontend: verify pages render and handle real-shaped data
describe("E2E: [Page] runtime", () => {
  it("renders without errors when no backend is available", () => {
    // Given: backend commands are mocked to return empty/error
    // When: page mounts
    // Then: no unhandled exceptions, user sees a graceful state
  });
});
```

---

## Milestone Plan

<!-- Copy the milestone template below for each milestone. -->

### Milestone N — [Title]

**Goal**: [One-sentence description of what this milestone accomplishes. What capability exists at the end that didn't exist before?]

**Context**: [2–4 sentences describing the current state relevant to this milestone. Reference specific files, line numbers, and code comments. Explain WHY this change is needed.]

**Important design rule**: [One key constraint or design decision that guides implementation for this milestone.]

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/<prefix>-m<N-1>.md`** — apply any corrections from the previous milestone.
3. Read these files before making changes:
   - `[file path]` — [what to understand about it]
   - `[file path]` — [what to understand about it]

#### Files Most Likely Touched

| File | Change |
|---|---|
| `[existing file path]` | [summary of change] |
| `[new file path]` | NEW: [what this file does] |

#### Step-by-Step

1. **Write BDD test stubs first** for all scenarios below. They should compile but fail.
2. **[Step description]**:
   - [Sub-step detail]
   - [Sub-step detail]
3. **[Step description]**:
   - [Sub-step detail]
4. **Make all BDD tests pass.**
5. **Run the full test suite** to confirm nothing is broken.

#### BDD Acceptance Scenarios

**Feature: [feature name]**

| Scenario | Given | When | Then |
|---|---|---|---|
| [Scenario name] | [Precondition] | [Action] | [Expected outcome] |
| [Scenario name] | [Precondition] | [Action] | [Expected outcome] |

**Feature: [feature name]**

| Scenario | Given | When | Then |
|---|---|---|---|
| [Scenario name] | [Precondition] | [Action] | [Expected outcome] |

#### Regression Tests

- [Existing test suite or feature that must still pass]
- [Specific edge case to verify]
- [Backward compatibility check]

#### E2E Runtime Validation

**File**: `[backend E2E test file path]`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `[test_function_name]` | [What runtime behavior this validates] | [Specific assertion criteria] |
| `[test_function_name]` | [What runtime behavior this validates] | [Specific assertion criteria] |

**File**: `[frontend E2E test file path]`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `[test name]` | [What runtime behavior this validates] | [Specific assertion criteria] |
| `[test name]` | [What runtime behavior this validates] | [Specific assertion criteria] |

#### Smoke Tests

- [ ] [Manual verification step — describe what to do and what to observe]
- [ ] [Manual verification step]
- [ ] `[test command]` passes
- [ ] App launches without errors

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **ARCHITECTURE.md**: [What to document.]
- **README.md**: [What to update.]

---

<!-- Repeat the "### Milestone N" template for each subsequent milestone. -->

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | ARCHITECTURE.md Update | README.md Update |
|---|---|---|
| 1 | [Section to add/update] | [Section to add/update] |
| 2 | [Section to add/update] | [Section to add/update] |
| 3 | [Section to add/update] | [Section to add/update] |
