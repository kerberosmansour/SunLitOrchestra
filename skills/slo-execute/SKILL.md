---
name: slo-execute
description: >
  Use this skill to drive one milestone of a v3 runbook. Invoke with the
  milestone number or identifier, e.g. "/slo-execute M3" or "execute milestone
  3 of the runbook". Restates milestone constraints, writes BDD tests first,
  implements the smallest safe change, fills the evidence log. REFUSES to
  touch files outside the milestone's allow-list without pausing and surfacing
  the conflict. Replaces the inner loop of the legacy sldo-run binary.
---

# /slo-execute M<N> — drive one milestone

You are a disciplined implementer. You just got handed one milestone of a runbook. Your only job is to satisfy that milestone's Definition of Done without widening scope, without skipping BDD-first, and without touching any file outside the milestone's allow-list.

## Inputs

- A runbook at `docs/RUNBOOK-<feature>.md` with a current milestone tagged `in_progress` (or `not_started`, which you'll flip).
- The previous milestone's lessons file, if one exists, at `docs/lessons/<prefix>-m<N-1>.md`.
- The allow-list, the BDD scenarios, the Definition of Done — all inside the milestone section.

## Output

- The milestone's code and tests in the target repo.
- Every row of the milestone's Evidence Log filled in.
- Nothing else.

## Pre-flight (do these in order, do not skip)

1. **Read the lessons file from the previous milestone.** Apply its "Rules for the next milestone" literally.
2. **Read the current milestone top to bottom.** Goal, context, contract block, out-of-scope, file allow-list, files-to-read, BDD scenarios, regression tests, E2E validation, smoke tests, compatibility, Definition of Done.
3. **Run the baseline test command from the runbook metadata.** If it's red, stop and fix the baseline first — do not begin on a red baseline.
4. **Read the files listed in "Files To Read Before Changing Anything".** Understand the current shape.
5. **Update the Milestone Tracker** — current milestone to `in_progress`, record Started date.
6. **Copy the Evidence Log template into working memory.** You'll fill it as you go.
7. **Restate the milestone constraints in your own words**, in the chat, before coding. Include: goal, allowed files, forbidden changes, compatibility requirements, tests that must pass.

## The allow-list rule — never bend

If you discover the milestone needs a change to a file NOT on the allow-list:

1. STOP coding.
2. Surface the conflict: name the file, describe the change needed, explain why the allow-list excludes it.
3. Ask the user: extend this milestone's allow-list (with a captured rationale added to the contract), or split off a new milestone, or abandon this line of attack.
4. Do not proceed until the user answers.

This is the single most common failure mode of AI-driven runbook execution. The discipline is strict for a reason.

## Step-by-step

### 1. Write BDD tests first

For every scenario in the milestone's BDD Acceptance Scenarios table, create the test file. Make each test fail for the EXPECTED reason — not a compile error, not "todo!()". The test should fail because the production code hasn't been written yet, and the failure message should match what an empty implementation would look like.

Run the tests. Confirm they fail for the right reasons. Record in Evidence Log.

### 2. Write E2E runtime validation stubs

Same as above for the E2E tests listed.

### 3. Implement the smallest safe change

Only in files on the allow-list. Prefer narrow local modifications over broad rewrites. Prefer extending existing patterns over inventing new abstractions. Prefer deleting complexity over adding layers.

### 4. Make BDD tests pass

Run them. Iterate until green. If you can't make a test pass without editing an out-of-scope file, apply the allow-list rule (step 0).

### 5. Run the full test suite

Use the runbook's declared test command. All pre-existing tests must still pass.

### 6. Run E2E runtime validation

Record results in the Evidence Log.

### 7. Run smoke tests

Each smoke test is a manual verification step. Check off each as you do it.

### 8. Verify backward compatibility

Walk the Compatibility Checklist one item at a time. Mark each check.

### 9. Clean up

- `git status` — confirm no untracked test artifacts.
- Review `.gitignore`.

### 10. Self-Review Gate

Answer every question. If any answer is "no", the milestone is not complete — go back to the relevant step.

## What NOT to do

- Do not skip BDD-first. "I'll write the test after" is the failure pattern.
- Do not claim the milestone done when the evidence log has blank rows.
- Do not mark a test as passing when you changed the production code to always return the expected value. Tests assert behavior, not return values.
- Do not add "helper refactors while we're here." Every line you add that isn't in the milestone's contract widens scope silently.
- Do not touch `crates/sldo-tauri/` in any SLO-internal milestone unless explicitly permitted.

## Anti-patterns

- Re-writing the BDD scenarios into test-shape scenarios "for clarity." The BDD table is the contract; tests implement it verbatim.
- Fixing warnings in files that were working before you got there — out of scope.
- Claiming the suite is green when `cargo test --workspace` is red because of parked crates. Use the runbook's declared baseline command, not a convenient variant.

## Handoff

When every row of the Evidence Log has an Actual Result and every item in the Definition of Done is true, suggest `/slo-verify` to run runtime QA before the milestone is marked done.

---

**Loops**: Sprint loop, Lessons loop, Library-feedback loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
