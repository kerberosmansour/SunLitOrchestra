# Interfaces — Fowler AI Architecture SLO Improvements

## Stable Interfaces

| Interface | Stability | Description |
|---|---|---|
| `skills/slo-architect/SKILL.md` | stable | Existing skill path and invocation contract remain unchanged. |
| `skills/slo-plan/SKILL.md` | stable | Existing skill path and output path remain unchanged. |
| `skills/slo-verify/SKILL.md` | stable | Existing runtime QA entry point remains unchanged. |
| `skills/slo-critique/SKILL.md` | stable | Existing four-persona critique entry point remains unchanged. |
| `skills/slo-ticket-*` | stable | Existing issue-sized workflow remains unchanged at the command/path level. |
| `docs/slo/templates/runbook-template_v_4_template.md` | evolving | New rows may be added additively; existing rows must not be renamed or removed. |
| `docs/slo/templates/ticket-contract-template_v_1.md` | evolving | New rows may be added additively; existing rows must not be renamed or removed. |

## New Additive Interfaces

| Interface | Stability | Description |
|---|---|---|
| `docs/slo/design/<slug>-reversibility.md` | evolving | New `/slo-architect` output listing hard-to-change decisions, reversibility tactic, rollback path, and proof. |
| `docs/slo/design/<slug>-code-map.md` | evolving | New `/slo-architect` output for brownfield repos: four-object summary, exemplars, anti-exemplars, seams, and coverage gaps. |
| `skills/slo-plan/references/refactoring-discipline.md` | evolving | New skill-local reference defining behavior-preserving refactoring microstep discipline. |
| `skills/slo-plan/references/ai-tolerance-contract.md` | evolving | New skill-local reference defining AI/LLM nondeterminism tolerance contracts. |
| `Contract Block: Exemplar code to copy` | evolving | New runbook/ticket row listing code patterns the agent should copy. |
| `Contract Block: Anti-exemplar code not to copy` | evolving | New runbook/ticket row listing legacy or partial patterns to avoid. |
| `Contract Block: Reversibility / rollback path` | evolving | New row citing the reversibility matrix and migration/rollback proof. |
| `Contract Block: AI tolerance contract` | evolving | New row required when `ai_component: true` or a milestone touches AI/LLM behavior. |

## Compatibility Commitments

- Existing runbooks without the new rows remain historical artifacts; `/slo-plan` emits the new rows for new v4 runbooks.
- Ticket contracts authored before this runbook remain valid.
- The additions must be additive and structurally tested; no existing Contract Block field names are removed.
- Missing new rows in future runbooks are treated as planning/critique findings, not as parser-breaking schema errors.
