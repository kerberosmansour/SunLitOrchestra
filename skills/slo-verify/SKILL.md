---
name: slo-verify
description: >
  Use this skill after /slo-execute finishes a milestone, before /slo-retro
  closes it. Invoke with the milestone number — "/slo-verify M3". Exercises
  BDD scenarios at runtime, including UI paths via Playwright if the milestone
  has a UI surface. For every bug found, writes a regression test FIRST, then
  hands the fix back to /slo-execute, then re-verifies. Do not use for static
  code review — that is /slo-critique.
---

# /slo-verify — runtime QA on a completed milestone

You are the QA lead. A milestone just finished implementation. Compilation and unit tests passed. Your job is to prove the thing actually works at runtime, including states the happy path never hits.

## Inputs

- A runbook at `docs/RUNBOOK-<feature>.md` with milestone N in `in_progress` state.
- The milestone's BDD Acceptance Scenarios and E2E Runtime Validation sections.
- The evidence log (you will add rows).

## Output

- `docs/verify/<prefix>-m<N>.md` — verification report.
- Regression tests for every bug you find (committed BEFORE the fix).
- Evidence log rows filled with runtime-check results.

## Prereq cascade (if UI)

If the milestone touches a UI surface:

1. `which node` — install hint if missing.
2. `npm ls playwright` in the target project — if absent, run `npx playwright install` and record it.
3. `which chromium` — Playwright ships its own Chromium; but confirm the binary is reachable.

If it's a pure backend / CLI milestone, skip the UI cascade and stick to runtime E2E.

## Method — three passes

### Pass 1. Happy path

Run every happy-path scenario from the BDD table at runtime. If the target is a UI, drive it with Playwright. If it's a CLI, exec the binary with realistic inputs. If it's an IPC/API, issue real calls.

Record: what you did, what you observed, pass/fail.

### Pass 2. Empty and degraded states

From the BDD table, run the empty-state, invalid-input, and dependency-failure scenarios. This is where most bugs live.

### Pass 3. Partial failures and boundary conditions

For any scenario that has a "partial failure" category, construct the failure (pull the plug, kill the dep, starve the queue). Observe what the system does. Every unexpected observation is a candidate bug.

## When you find a bug

1. **STOP** and write a regression test that reproduces it. The test should fail today.
2. Commit the regression test on its own — do not bundle with the fix.
3. Hand the bug back to `/slo-execute` or a human to fix (do not fix it yourself in this skill — separation of concerns).
4. Once fixed, re-run the regression test; it should now pass.
5. Re-run the full milestone verification to confirm no regression in other scenarios.

## Gates — do not mark verified when

- Any BDD scenario is untested at runtime (including empty-state).
- A regression test was added without a fix being applied in the same branch.
- The milestone's Evidence Log still has blank runtime rows.
- Playwright traces / screenshots from failing scenarios weren't captured.

## Verification report shape

```markdown
# Verification Report — <prefix> Milestone <N>

## What was exercised
| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|

## Bugs found
| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|

## Environment
- OS, browser, Node version, platform.

## Coverage gaps
- <scenarios NOT exercised, with reason>
```

## Anti-patterns

- Re-running unit tests and calling it "verification." Unit tests are the input; runtime is what this skill checks.
- Finding a bug, fixing it inline, and never adding a regression test.
- Skipping empty-state because "it's just a screenshot" — empty states are where AI-slop lives.
- Batching multiple bugs into one "fix and re-verify" cycle. Do one at a time so the regression test per bug is clean.

## Handoff

When every BDD scenario has a runtime row with a `pass` result, suggest `/slo-retro M<N>` to close out the milestone. If bugs were found and fixed, the retro should mention them as "missing coverage" in the lessons file.
