---
name: slo-plan
description: >
  Use this skill after /slo-architect (and /slo-tla if tla_required), when the
  user says "write the runbook", "plan the milestones", "turn this into tasks".
  Authors a full v4 runbook INTERACTIVELY, one milestone at a time, confirming
  each contract before moving on. Maximum 5 milestones per runbook — if scope
  needs more, suggest splitting. Refuses to generate the whole runbook in one
  shot; this is deliberate discipline, not a limitation.
---

# /slo-plan — Write A V4 Runbook, Milestone By Milestone

Work one milestone at a time, confirming each contract block before the next.

## Inputs

- `docs/slo/idea/<slug>.md`
- `docs/slo/research/<slug>/synthesis.md`
- `ARCHITECTURE.md` or `docs/ARCHITECTURE.md`; if missing, auto-generate a reality-first `docs/ARCHITECTURE.md`.
- Optional `/slo-architect` outputs: stack decision, interfaces, and if `tla_required: true`, verified design + TLC results.

## Output

One file in the user's project: `docs/RUNBOOK-<kebab-slug>.md`. Use `references/runbook-template_v_4_template.md` first; the repo mirror is `docs/slo/templates/runbook-template_v_4_template.md`. The v3 template remains for old runbooks.

## Discipline — The One Rule

**NEVER generate a whole runbook in one shot.** If asked, refuse: one-shot runbooks are syntactically valid and strategically thin. The interactive walk is the value.

## Method

1. Scaffold v4 metadata: ID, prefix, stack, tests, stable interfaces, red lines.
2. Check architecture docs; generate a reality-first orientation doc if missing.
3. Propose 2–5 milestones; split the runbook if scope needs more than 5.
4. For each milestone, follow [`references/methodology-milestone-authoring.md`](references/methodology-milestone-authoring.md), then confirm scope, allow-list, and BDD specificity.
5. Fill the Documentation Update Table, architecture diagram, and TLA+ section.

## Contract Block Sentinels

Every milestone Contract Block includes the base rows plus:

- **Data classification**: `Public`, `Internal`, `Confidential`, or `Restricted`; see [`references/proactive-controls-vocabulary.md`](references/proactive-controls-vocabulary.md).
- **Proactive controls in play**: cite stack-aware controls from [`references/proactive-controls-vocabulary.md`](references/proactive-controls-vocabulary.md).
- **Abuse acceptance scenarios**: cite [`references/abuse-case-examples.md`](references/abuse-case-examples.md); required for every new surface. If no new surface, write `N/A — no new surface introduced, see <reason>`. Silent omission is forbidden.

BDD includes happy path, invalid input, empty state, dependency failure, retry/concurrency/persistence/backward compat as applicable, and **abuse case** for new surfaces.

## Gates

Refuse when file ownership is unclear, BDD is generic, Definition of Done or Evidence Log is absent, or Forbidden shortcuts is empty.

## Anti-Patterns

- Generic BDD, 30-row allow-lists, deferred Evidence Logs, scope beyond 5 milestones, silent omission of the three security rows, vague abuse cases, or new data/control vocabulary.

## Handoff

After the runbook is complete, suggest `/slo-critique` before execution starts.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
