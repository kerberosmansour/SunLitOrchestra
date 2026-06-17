---
name: slo-verify-outcome-validation-pass
source_skill: skills/slo-verify/SKILL.md
description: The /slo-verify Pass 0 (Outcome Validation) procedure — the highest-authority gate.
---

# Pass 0 — Outcome Validation (the highest-authority gate)

Pass 0 is the runtime half of **Outcome First Engineering** (template §6.12). It
is a **non-renumbering leading insertion**: it runs before Pass 1 and outranks
every other pass — **a Pass 0 failure fails the milestone even if Passes 1–6 are
green.** Run it only for **value-bearing** milestones; otherwise record
`N/A — not value-bearing`.

## Inputs

- The milestone's §5C Outcome Validation Contract (Outcome, Success Criteria,
  per-layer Front-to-End Validation, Regression Requirements).
- §17 **Outcome Scenarios** (`oc-<slug>-N`), **Critical User Journeys**
  (`cuj-<slug>-N`), and the **Core Capability Regression Matrix**.

## Procedure

1. **Outcome Scenarios** — run every `oc-<slug>-N` at runtime. Each must show the
   promised user outcome actually exists (the observable `Then` + follow-on
   `And`s), not just a 200 / a mock return.
2. **Critical User Journeys** — run every `cuj-<slug>-N` end-to-end.
3. **Core Capability Regression Matrix** — re-run every required capability's
   journey. "Did this milestone break anything important?" A required-row failure
   blocks completion; every row resolves to `pass | not_applicable |
   waived_with_reason`, never blank.

## Front-to-end, over the highest applicable layer chain

Drive each journey across the layers it actually has:
`seed test data → backend → persisted record → API/IPC response → UI display`
(Playwright for the UI layer when applicable). A value-bearing milestone needs
**at least one real cross-layer assertion** (e.g. `backend → persisted`) — a
single-layer or mock-only assertion does **not** satisfy Pass 0. On a no-UI /
library / CLI target, mark the UI layer `not_applicable(reason)` and assert the
highest applicable chain instead. This is the runtime defense against
outcome-test theatre (`tm-outcome-first-abuse-2`).

## When Pass 0 finds a failing outcome — reuse the existing flow

Pass 0 invents **no new** bug-found flow. Apply the existing one verbatim:

1. **STOP** and write a regression test that reproduces the missing/broken
   outcome (regression-test-first).
2. Commit the regression test on its own.
3. Hand the fix back to `/slo-execute`.
4. Re-run Pass 0 to confirm green.
5. Re-run Passes 1–6 to confirm no regression.

## Authority note

If 1000 unit tests pass but one Outcome Scenario, Critical User Journey, or
required Regression-Matrix row fails, the milestone fails. Pass 0 is the apex of
the inverted pyramid (template §11.8); `/slo-retro` refuses to close the
milestone while any Pass 0 row is unproven.
