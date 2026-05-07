---
name: fowler-ai-architecture-slo-improvements
created: 2026-05-07
status: ideation
tla_required: false
---

# Fowler AI Architecture SLO Improvements

## The pain

Sherif has a working SLO skill pack whose engineering loop already enforces small slices, BDD-first implementation, critique, runtime verification, and retros. After reviewing Martin Fowler material on AI-assisted software engineering and agile architecture, the remaining pain is that the SLO skills treat several Fowler-shaped disciplines as implicit judgment rather than explicit contract: reversibility of architectural decisions, exemplar code as a driver of architecture, brownfield code understanding before modification, true behavior-preserving refactoring, and tolerance design for nondeterministic AI components.

## Five capabilities the user described without realizing

- `/slo-architect` should identify hard-to-change decisions and document how each can be made reversible before downstream milestones lock interfaces.
- Brownfield planning should produce a short code map and named exemplar files before any AI agent modifies a legacy or existing codebase.
- `/slo-plan` should make exemplar code and anti-exemplar code first-class Contract Block rows.
- Refactor budgets should distinguish true behavior-preserving micro-refactoring from ordinary feature work or broad cleanup.
- AI/LLM components should carry explicit nondeterminism tolerance contracts that `/slo-verify` can exercise at runtime.

## Top risks

- **Breach**: prompt-injection or poisoned user/source notes could be copied into durable skill prose or threat-model defaults unless all user-provided text is fenced and sourced.
- **Compliance fine**: low direct regulatory exposure; the skill pack is public engineering process content, but business-pack artifacts generated later could mishandle confidential founder/PII material if AI tolerance and exemplar rules drift.
- **Prolonged outage**: skill-pack drift could break install or execution expectations across Claude Code, GitHub Copilot, and Codex, leaving users with a confusing or unusable SLO flow.

## Approach A — conservative

- **Effort**: 5 milestones.
- **Wedge**: add reversibility and brownfield code-map outputs to `/slo-architect`, then propagate the resulting contracts through `/slo-plan`, `/slo-critique`, `/slo-verify`, and the ticket flow.
- **Risks**: touching too many skill contracts at once; mitigated by keeping every milestone docs-first, adding structural-contract tests, and preserving existing skill paths.

## Approach B — cloud / SaaS

- **Effort**: not applicable.
- **Wedge**: not applicable; this is a local Markdown skill-pack evolution, not a hosted product.
- **Risks**: externalizing the discipline into a service would conflict with the repo's host-neutral Markdown contract.

## Approach C — local / desktop

- **Effort**: 3-4 milestones.
- **Wedge**: build only local runbook-template changes and leave skills unchanged.
- **Risks**: template-only changes would not reliably affect installed skills, especially when agents use skill-local references rather than repo mirrors.

## Recommendation

Use Approach A. It follows the existing SLO pattern: add the upstream architectural artifact first, then make planning, critique, verification, and ticket-sized work consume it. The first shippable wedge is `/slo-architect` emitting reversibility and brownfield code-map artifacts for new feature work.

## Open questions for /slo-research

1. Which Fowler/Thoughtworks claims should become SLO skill contracts rather than background prose?
2. Where does the current SLO skill pack already cover those claims?
3. Which artifacts should change: skill contracts, runbook templates, ticket templates, reference files, or critique personas?
4. What is the smallest 5-milestone runbook that can land the improvements without destabilizing current install/runtime boundaries?
