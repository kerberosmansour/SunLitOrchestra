---
name: slo-plan-outcome-validation-contract
source_skill: skills/slo-plan/SKILL.md
description: How to author the v4 §5C Outcome Validation Contract + the §17 outcome sub-sections.
---

# Authoring the Outcome Validation Contract (§5C + §17)

The planning-time half of **Outcome First Engineering**. Required for every
**value-bearing** milestone (introduces or changes user-facing capability;
EXCLUDES internal refactor / docs-only / test-only — mark those
`N/A — not value-bearing, see <reason>`). The promised user outcome is the
**primary Definition of Done**: code completion alone is insufficient (§6.12).

## §5C Outcome Validation Contract (one per value-bearing milestone)

| Field | How to fill it |
|---|---|
| Outcome | One sentence naming the user value this milestone makes exist. |
| Success Criteria | Bulleted, each **independently observable** (discovered / classified / visible in UI / remediation shown / appears in history / survives restart). |
| Front-to-End Validation | The proof path authored **PER LAYER** — each step is `applicable \| not_applicable(reason)`: seed test data → run → verify backend result → verify persisted record → verify API/IPC response → verify UI display. |
| Regression Requirements | Which existing critical capabilities must still work (ties to the §17 Core Capability Regression Matrix). |

### The per-layer rule (theme B — do not skip)

The Front-to-End path is **not** a monolithic block. Each layer is marked
`applicable` or `not_applicable(reason)` so a no-UI / library / CLI target does
not have to fake a UI step or collapse the whole row to `N/A`. **At least one
real cross-layer assertion is required** (e.g. `backend → persisted`) for every
value-bearing milestone — a single-layer assertion, or one that only asserts a
mock's return value, does **not** satisfy §5C. This is what stops outcome-test
theatre (`tm-outcome-first-abuse-2`) on targets that lack a UI to drive.

### Authored-string safety

Authored §5C / §17 text is **descriptive Markdown only**. When any of it is
rendered into a generated security/threat artifact it is wrapped in a `~~~text`
fence (the `/slo-architect` rule). Authored text **never selects control fields**
(`oc-`/`cuj-`/`tm-` ids, resolution verbs, or gate outcomes) —
`tm-outcome-first-abuse-1`.

## §17 sub-sections

### Outcome Scenarios (`oc-<slug>-N`)

The **primary Definition of Done**. Frozen ids, contiguous from 1, never
renumbered. Outcome-shaped: one observable user outcome plus follow-on `And`s.
Include **security** rows (cite `tm-<slug>-abuse-N`) and **reliability** rows
(degraded / outage outcomes). A single-`And`, trivially-true, or mock-only
scenario is **non-conformant**: `/slo-plan` refuses it and `/slo-critique` flags
it `ask` (this is the anti-theatre gate).

### Critical User Journeys (`cuj-<slug>-N`)

Frozen ids, contiguous from 1, never renumbered. Each is an ordered front-to-end
path that becomes a mandatory automated test, run front-to-end by `/slo-verify`'s
Outcome Validation pass.

### Core Capability Regression Matrix

"Did this milestone break anything important?" Every core capability resolves to
exactly one of `pass | not_applicable | waived_with_reason` — **never blank**
(mirrors the §5B Bundle discipline). A `waived_with_reason` row needs a non-empty
reason. Failure of any required row blocks milestone completion (§6.12).

## Anti-theatre checklist (what `/slo-plan` refuses)

- A vacuous / single-`And` / mock-only Outcome Scenario.
- A monolithic Front-to-End path (no per-layer `not_applicable(reason)`, no
  cross-layer assertion).
- A blank Regression-Matrix resolution cell.
- An `oc-`/`cuj-` id that is renumbered rather than superseded.
