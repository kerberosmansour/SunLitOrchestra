---
name: slo-plan-ai-tolerance-contract
source_skill: skills/slo-plan/SKILL.md
description: Required tolerance contract for milestones that introduce, modify, or verify AI/LLM behavior.
---

# AI Tolerance Contract

Use this reference when a milestone introduces, modifies, or verifies AI/LLM behavior, including agentic planning, model prompts, embedding retrieval, tool-choice logic, eval harnesses, or AI-generated output.

Non-AI milestones write `N/A — no AI component` in the Contract Block. Do not invent an AI tolerance row for ordinary deterministic code.

## Required Fields

| Field | Requirement |
|---|---|
| Accepted variance | State what output variance is acceptable, with concrete examples or measurable thresholds. |
| Deterministic boundary | Name the code, config, data, schema, safety rule, or interface that must stay deterministic even when model output varies. |
| Eval evidence | Cite the golden set, scenario table, fixture, or command that proves the behavior stays inside the accepted variance. |
| Retry / fallback | Define the bounded retry policy and fallback behavior when samples exceed the tolerance boundary. |
| Must-never outcomes | List banned outputs or behaviors, especially safety, security, compliance, privacy, and data-integrity failures. |
| Sample budget | Set a bounded sample/eval count and time budget. Unbounded retries, open-ended sampling, and "try until good" loops are forbidden. |

## Contract Block Row

```markdown
| AI tolerance contract | Accepted variance: <...>; deterministic boundary: <...>; eval evidence: <...>; retry / fallback: <bounded policy>; must-never outcomes: <...>; sample budget: <bounded count/time>. |
```

## Verification Expectations

- `/slo-verify` checks the AI tolerance row after the normal runtime and security passes.
- Eval evidence must be reproducible enough for the milestone: a deterministic fixture, golden scenario list, fixed prompt/version pair, or documented model/provider setting.
- A single successful sample is not evidence. Use the bounded sample budget declared in the contract.
- If the AI component is stubbed or unavailable, verification records `skipped` only when the milestone contract allows it and names the missing provider/runtime.

## N/A Path

For deterministic, docs-only, template-only, or non-AI milestones, write:

```markdown
| AI tolerance contract | N/A — no AI component |
```
