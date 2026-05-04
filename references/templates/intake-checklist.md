---
name: intake-checklist
status: stable-template
created: 2026-05-04
audience: skills that need structured context before producing an artifact
purpose: Shared conversational intake pattern for turning vague user input into explicit, checkable fields.
---

# Intake Checklist Template

Use this pattern when a skill needs context that cannot be safely inferred. This template cites `references/templates/citation-discipline.md` because intake answers often become claims in downstream artifacts.

## Conversation Shape

Ask one question at a time. Do not dump a form. Push on vague answers until the answer names a concrete actor, date, file, surface, counterparty, stack, or risk. The precedent is `/slo-ideate`'s seven forcing questions: each question narrows the problem before any artifact is written.

## Minimum Fields

- `goal:` one sentence describing the desired outcome.
- `actor:` user, founder, engineer, attacker, or system role.
- `current_state:` what exists today.
- `constraint:` budget, stack, jurisdiction, compatibility, time, or risk boundary.
- `evidence_source:` repo file, user answer, issue URL, or researched source.
- `unknowns:` fields that remain unresolved.

## Restate Checkpoint

Before writing, restate the intake summary and ask for correction. If the user corrects a field, update the structured summary instead of silently reinterpreting the earlier answer.

## Rapid Intake

`--rapid-intake` may compress the questions only when the skill already has high-quality structured input, such as a runbook Contract Block or GitHub issue acceptance criteria. It must still identify unknowns.

## Anti-Patterns

- Treating "sounds good" as confirmation of fields never restated.
- Collapsing ambiguity into a default.
- Asking for every field at once.
- Accepting hypothetical users or risks where a concrete example is required.
