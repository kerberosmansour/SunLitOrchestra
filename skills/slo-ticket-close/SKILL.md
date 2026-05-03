---
name: slo-ticket-close
description: >
  Use this skill after /slo-ticket-verify. It performs the ticket-sized closeout:
  final evidence check, lessons/follow-ups, PR handoff, GitHub issue workpad
  update, and review-state transition. It opens or updates a PR but does not
  merge or close the issue without explicit user instruction.
---

# /slo-ticket-close - hand off one verified ticket

You are the release handoff lead for one ticket. Your job is to turn completed ticket work into a reviewable PR and durable tracker state.

## Inputs

- Verified `docs/slo/tickets/ticket-<issue>-<slug>.md`.
- Current branch and git diff.
- Linked GitHub issue and workpad comment, if available.

## Output

- PR opened or updated.
- Ticket contract Closure Summary filled.
- Issue workpad final checklist and evidence updated.
- Optional follow-up issues suggested, never auto-filed.

## Pre-flight

1. Read the ticket contract and confirm all Validation Plan rows are pass/N/A with reason.
2. Confirm every Self-Review Gate item is checked or has an explicit blocker.
3. Run `git status --short` and identify all changed files.
4. Confirm changed files are inside the contract allow-list, plus the ticket contract itself and permitted evidence docs.
5. Refuse to proceed on `main` or `master`.

## Method

1. Fill `## 11. Closure Summary` in the ticket contract:
   - completed behavior
   - tests and validation
   - lessons/follow-ups
   - PR/issue links
2. Commit the work if the user asked this skill to handle commits. Use a message shaped like:

```text
ticket-<issue>: <short outcome>
```

3. Push the current branch. Do not force-push.
4. Open or update one PR. PR body shape:

```markdown
## Summary
<one paragraph>

## Issue
Closes or refs #<number>

## SLO ticket contract
- docs/slo/tickets/ticket-<issue>-<slug>.md

## Validation
- <command/action>: <result>

## Risk and compatibility
- <data classification, public surfaces, compatibility notes>

## Follow-ups
- <deferred items or N/A>

Generated with /slo-ticket-close
```

5. Link the PR in the issue workpad.
6. Mark workpad checklist items complete.
7. Move tracker state to review when permitted:
   - add `slo:review`
   - remove `slo:in-progress`
   - leave closing/merge decisions to the human unless explicitly instructed.

## Follow-up discipline

- Surface follow-ups with lane: `micro`, `milestone`, or `fresh-runbook`.
- Do not auto-file follow-up issues.
- Security or data-loss follow-ups must be called out separately as blocking or non-blocking.

## Gates

- Refuse if validation evidence is incomplete.
- Refuse if on `main` or `master`.
- Refuse if unrelated files are present in the diff.
- Refuse auto-merge.
- Refuse auto-close unless the user explicitly asked for closeout after PR merge.

## Anti-patterns

- PR body as diff-stat instead of evidence summary.
- Closing the issue while PR is still unreviewed.
- Dropping lessons because the ticket was small.
- Filing follow-ups without confirmation.
- Marking review-ready while the issue workpad contradicts the contract.

## Handoff

Print the PR URL and the issue URL. Stop there.

---

**Loops**: Ticket loop - see [docs/LOOPS-ENGINEERING.md#ticket-loop](../../docs/LOOPS-ENGINEERING.md#ticket-loop).
