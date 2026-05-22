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
5. Fill the Documentation Update Table, architecture diagram, and §5 Formal-Verification section. When the design has `kani_required: true`, author the §5.8 Kani proof-obligation sub-block (target fn, property, bound, expected pre-fix/post-fix) and add a matching Kani-obligation row to each relevant milestone's Evidence Log; otherwise mark §5.8 `N/A`.

## Contract Block Sentinels

Every milestone Contract Block includes the base rows plus:

- **Exemplar code to copy**: cite concrete paths or write `N/A — docs-only` / `N/A — no brownfield exemplar needed` with a reason.
- **Anti-exemplar code not to copy**: cite risky legacy paths or patterns agents must not imitate; N/A with reason is valid.
- **AI tolerance contract**: required when the milestone introduces, modifies, or verifies AI/LLM behavior (`ai_component: true`, prompt/tool-choice changes, eval harnesses, generated-output behavior, or AI-agent flows). Cite [`references/ai-tolerance-contract.md`](references/ai-tolerance-contract.md) and fill accepted variance, deterministic boundary, eval evidence, retry / fallback, must-never outcomes, and bounded sample budget. For non-AI work, write `N/A — no AI component`.
- **Data classification**: `Public`, `Internal`, `Confidential`, or `Restricted`; see [`references/proactive-controls-vocabulary.md`](references/proactive-controls-vocabulary.md).
- **Proactive controls in play**: cite stack-aware controls from [`references/proactive-controls-vocabulary.md`](references/proactive-controls-vocabulary.md) as actionable implementation constraints, using [`references/secure-construction-matrix.md`](references/secure-construction-matrix.md) to map each touched surface to secure defaults and tests.
- **Abuse acceptance scenarios**: cite [`references/abuse-case-examples.md`](references/abuse-case-examples.md); required for every new surface. If no new surface, write `N/A — no new surface introduced, see <reason>`. Silent omission is forbidden.
- **Measurement deliverables**: required for any **value-bearing** milestone. Cite the runbook's §5A Measurement Contract and name this milestone's slice (events / runtime metrics / saved queries + guardrail owner + readout date). For non-value-bearing milestones, write `N/A — not value-bearing, see <reason>`.

## Measurement Contract requirement

A runbook for a **value-bearing feature** MUST carry the §5A Measurement Contract before it is complete. **"Value-bearing" is defined deterministically**: the work *introduces or changes user-facing capability*. It **EXCLUDES** internal refactor, docs-only, and test-only work (those mark §5A `N/A — not a value-bearing feature`). Pull the per-feature inputs forward from the idea doc's `## Success thesis` (`/slo-ideate`) and the `/slo-product metrics` feature measurement spec (`feature_measurement_spec: true`). If a value-bearing runbook reaches completion without a filled Measurement Contract, **flag the gap** and require it before handoff — but do **not** retroactively invalidate legacy runbooks authored before this section existed (they remain valid; the requirement is forward-looking, mirroring the §10 Carry-forward backward-compat posture).

When the Refactor budget is anything except `No refactor permitted beyond direct implementation`, apply [`references/refactoring-discipline.md`](references/refactoring-discipline.md): behavior-preserving microsteps, pre-test evidence, and post-test proof.

BDD includes happy path, invalid input, empty state, dependency failure, retry/concurrency/persistence/backward compat as applicable, and **abuse case** for new surfaces.

## Gates

Refuse when file ownership is unclear, BDD is generic, Definition of Done or Evidence Log is absent, or Forbidden shortcuts is empty.

## Anti-Patterns

- Generic BDD, 30-row allow-lists, deferred Evidence Logs, scope beyond 5 milestones, silent omission of the three security rows, vague abuse cases, or new data/control vocabulary.

## Handoff

After the runbook is complete, suggest `/slo-critique` before execution starts.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
