---
name: slo-retro
description: >
  Use this skill at the END of every milestone, after /slo-execute and
  /slo-verify have finished. Writes the milestone's lessons-learned file,
  completion summary, and updates the runbook's Milestone Tracker. Requires a
  filled-in Evidence Log. Refuses to run on milestones whose Evidence Log has
  blank "Actual Result" cells. Also usable standalone to close out a milestone
  that was executed by hand.
---

# /slo-retro — close out a milestone

You are the engineering manager running the post-mortem. The milestone just finished. You have one job: turn the evidence log plus the observed behavior into two files and a tracker update, following the v3 runbook templates literally.

## Inputs

- A runbook at `docs/RUNBOOK-<feature>.md` with a current milestone in `in_progress` state.
- That milestone's Evidence Log (inside the runbook).
- Optional: a verification report at `docs/verify/<prefix>-m<N>.md` if `/slo-verify` ran.
- Optional: the previous milestone's lessons file, for comparison.

## Outputs

Write exactly three things, then run the additive issue-filing flow (see "Issue filing" below):

1. `docs/lessons/<prefix>-m<N>.md` — lessons-learned file (v3 template). **Always written first**, even when `gh` is unavailable.
2. `docs/completion/<prefix>-m<N>.md` — completion summary (v3 template).
3. Inline edits to the runbook: Milestone Tracker row updated to `done`, with Completed date and paths.

After those three are on disk, run the issue-filing flow as described under "Issue filing". If issue filing fails for any reason, the three artifacts above are still safely written — issue filing is strictly additive.

## Pre-conditions

Refuse to run and list the blockers if any of these are true:

- The Evidence Log has blank "Actual Result" cells.
- The Self-Review Gate questions in the runbook template are not all "yes".
- Any BDD scenario in the milestone is still marked pending.
- `git status` shows untracked test artifacts.

Do not fix these yourself. Tell the user which rows are blank, which questions are not answered, which scenarios are pending. They decide whether to go finish execution or override.

## Method

1. Read the runbook's milestone section top to bottom. Extract: goal, files changed, BDD scenarios, Evidence Log, Definition of Done.
2. Read the previous lessons file (if one exists). Note which rules it established — were they followed? Note in the new lessons.
3. Diff the actual files changed vs. "Files Allowed To Change" in the contract. Flag any out-of-scope edits in the lessons file.
4. Write the lessons file per the template below.
5. Write the completion summary per the template below.
6. Edit the runbook: change the milestone tracker row, add the Completed date, record the paths to both new files.
7. Run the runbook's post-flight commands (tests, build) one more time and record in the evidence log if they haven't been already.

## Lessons file template

```markdown
# Lessons Learned — <prefix> Milestone <N>

## What changed
- <summary>

## Design decisions and why
- <decision> — <reason>

## Mistakes made
- <mistake>

## Root causes
- <root cause>

## What was harder than expected
- <note>

## Naming conventions established
- <types, files, tests, events, commands>

## Test patterns that worked well
- <pattern>

## Missing tests that should exist now
- <test>

## Rules for the next milestone
- <rule>

## Template improvements suggested
- <improvement>
```

## Completion summary template

```markdown
# Completion Summary — <prefix> Milestone <N>

## Goal completed
- <what capability now exists>

## Files changed
- <file>

## Tests added
- <test file>

## Runtime validations added
- <e2e file>

## Compatibility checks performed
- <check>

## Documentation updated
- <doc and section>

## .gitignore changes
- <patterns added or removed>

## Test artifact cleanup verified
- <confirmation that git status is clean>

## Deferred follow-ups
- <follow-up>

## Known non-blocking limitations
- <limitation>
```

## Anti-patterns

- Writing platitudes — "it went well", "nothing to note". The template fields exist because honest post-mortems find things. If a field is truly N/A, write the one-line reason it's N/A.
- Closing a milestone with out-of-scope edits un-flagged — the lessons file is where scope breaches get documented so the next milestone's allow-list can learn.
- Skipping the "rules for the next milestone" section. This is the single most valuable part of the lessons file and the next milestone will read it first.

## Handoff

After writing, suggest the next step: `/slo-execute M<N+1>` or, if the runbook's last milestone is now done, `/slo-ship` to open the PR.

---

**Loops**: Sprint loop, Lessons loop, Library-feedback loop — see [docs/LOOPS-ENGINEERING.md#lessons-loop](../../docs/LOOPS-ENGINEERING.md#lessons-loop).
