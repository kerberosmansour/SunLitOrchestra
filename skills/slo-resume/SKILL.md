---
name: slo-resume
description: >
  Use this skill to pick up an interrupted runbook. Invoke as "/slo-resume"
  (no args) and it reads the current runbook's Milestone Tracker to identify
  the first non-done milestone, then suggests the next skill to run for that
  milestone. Does not modify state — it only orients. Good for "I stepped away
  yesterday, where was I" scenarios.
---

# /slo-resume — orient after an interruption

You are returning to a runbook after a pause. Your job is to figure out what state the work is in and suggest the next concrete action, not to do the action yourself.

## Inputs

- The current directory's `docs/RUNBOOK-*.md` files. If there's more than one, ask the user which.

## Method

1. Read the Milestone Tracker table at the top of the runbook.
2. Find the first row whose Status is NOT `done`:
   - `not_started` → suggest `/slo-execute M<N>` (or earlier: `/slo-plan` if the milestone is sparse).
   - `in_progress` → check the Evidence Log. If BDD scenarios are untested at runtime, suggest `/slo-verify`. Otherwise suggest finishing `/slo-execute`.
   - `blocked` → print the blocker (from the row's Notes column or the last lessons file's "Mistakes made" section) and ask the user what to do.
3. If all rows are `done`, suggest `/slo-ship` or confirm the runbook is complete.

## Output

A short message in chat:

> You are at Milestone N (<title>), status: <status>.
> Next action: <suggestion>.
> Context: <one-line summary of what was done last>.

Nothing else. Do not start the next action — the user decides.

## Gates

- Refuse to proceed if there is no runbook in the current directory.
- If multiple runbooks exist, ask which one before orienting.

## Anti-patterns

- Auto-starting the next skill. `/slo-resume` is for orientation, not execution.
- Rewriting the tracker based on "what should have happened" — read the tracker as-is.
- Guessing when an in-progress milestone's evidence log is ambiguous — ask the user.

## Handoff

After printing the next action, exit. The user invokes whatever skill they want.
