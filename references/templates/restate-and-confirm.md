---
name: restate-and-confirm
status: stable-template
created: 2026-05-04
audience: skills that make commitments, route work, or write artifacts from user-provided context
purpose: Shared correction loop before acting on interpreted user input.
---

# Restate And Confirm Template

This pattern prevents silent reinterpretation. It cites `references/templates/citation-discipline.md` because confirmed user context and sourced claims must be kept distinct.

## When To Use

Use before:

- writing a durable artifact;
- evaluating gates or hard-block predicates;
- switching modes;
- running a mutating tool;
- extending a milestone allow-list;
- filing or updating tracker state.

## Format

```markdown
I am going to proceed with:
- goal: <one sentence>
- scope: <files, artifact, repo, issue, or milestone>
- constraints: <compatibility, risk, authority, budget>
- unknowns: <none | list>

Did I hear that right?
```

## Correction Loop

If the user corrects anything, update the structured restatement and repeat only the changed fields. Do not treat a correction as permission to widen unrelated scope.

## Failure Mode

When the user cannot confirm a high-risk field, stop and route to the relevant intake, research, legal, accounting, security, or architecture step. "Insufficient information" is a valid third state.
