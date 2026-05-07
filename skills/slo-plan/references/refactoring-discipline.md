---
name: slo-plan-refactoring-discipline
source_skill: skills/slo-plan/SKILL.md
description: Behavior-preserving refactoring discipline for v4 milestone contracts.
---

# Refactoring Discipline

Refactoring means behavior-preserving code change. It is not generic cleanup, opportunistic redesign, or feature work hidden behind a tidy name.

Apply this reference when a milestone's **Refactor budget** is anything except `No refactor permitted beyond direct implementation`.

## Required Proof

| Proof item | Requirement |
|---|---|
| Pre-test | Name the test or command that proves current behavior before the refactor starts. |
| Microstep | Split the refactor into one behavior-preserving transformation at a time. |
| Post-test proof | Re-run the same test or command after each microstep or tightly grouped set of microsteps. |
| Behavior boundary | State which externally visible behavior must not change. |

## Allowed Refactor Shapes

- Rename or extract within the allowed file list while preserving public interfaces.
- Move repeated logic behind a local helper when the before/after tests are the same.
- Tighten internal structure only when the milestone goal needs it and the Contract Block names it.

## Forbidden Shortcuts

- Mixing behavior change and refactor without naming both in the Contract Block.
- Calling broad cleanup a refactor when there is no pre-test and post-test proof.
- Touching files outside the milestone allow-list because they look adjacent.

## N/A Path

For docs-only, template-only, or test-only milestones, write `N/A — docs-only` or `N/A — no refactoring performed, see <reason>` in the Contract Block.
