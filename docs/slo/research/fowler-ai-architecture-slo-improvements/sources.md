---
name: fowler-ai-architecture-slo-improvements
researched: 2026-05-07
incomplete: false
---

# Sources — Fowler AI Architecture SLO Improvements

## External sources

| id | Source | URL | Accessed | Used for |
|---|---|---|---|---|
| S1 | Martin Fowler, *Is Design Dead?* | https://martinfowler.com/articles/designDead.html | 2026-05-07 | Evolutionary design, architecture as hard-to-change core, reversibility, throwaway experiments, will to design. |
| S2 | Martin Fowler, *Refactoring* book page | https://martinfowler.com/books/refactoring.html | 2026-05-07 | Refactoring as small behavior-preserving transformations and testing discipline. |
| S3 | Thoughtworks Technology Radar, *Using GenAI to understand legacy codebases* | https://www.thoughtworks.com/en-gb/radar/techniques/using-genai-to-understand-legacy-codebases | 2026-05-07 | GenAI-assisted codebase comprehension as an Adopt/Trial technique and knowledge-graph/RAG framing. |
| S4 | Thoughtworks Technology Radar landing page | https://www.thoughtworks.com/en-ca/radar | 2026-05-07 | AI-generated code cognitive debt, retaining engineering principles, and feedback-cycle evolution. |
| S5 | Martin Fowler, *Who Needs an Architect?* PDF | https://martinfowler.com/ieeeSoftware/whoNeedsArchitect.pdf | 2026-05-07 | Architecture, irreversibility, and complexity trade-offs. |
| S6 | InfoQ, *Agile Architecture: Reversibility, Communication and Collaboration* | https://www.infoq.com/news/2015/05/agile-architecture/ | 2026-05-07 | Agile architecture summary: reversibility, adaptable docs, code/architecture exemplars, hands-on architects. |
| S7 | YouTube, *How AI will change software engineering – with Martin Fowler* | https://www.youtube.com/watch?v=CQmI4XKTa0U | 2026-05-07 | User-provided notes about AI nondeterminism, vibe coding, refactoring, and feedback loops; video page used as locator, not transcript authority. |
| S8 | YouTube, *Agile Architecture - Molly Dishman & Martin Fowler Keynote* | https://www.youtube.com/watch?v=VjKYO6DP3fo | 2026-05-07 | User-provided notes about agile architecture; InfoQ summary S6 used as the sourced textual authority. |

## Repo-local sources

| id | Artifact | Used for |
|---|---|---|
| R1 | `docs/LOOPS-ENGINEERING.md` | Current sprint, ticket, security-tuning, and lessons loops. |
| R2 | `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md` | Existing SLO philosophy: LLM-driven process can carry more discipline if context windows are respected. |
| R3 | `skills/slo-architect/SKILL.md` | Current architecture, threat-model, AI-component, and interface lock-in behavior. |
| R4 | `skills/slo-plan/SKILL.md` and `skills/slo-plan/references/methodology-milestone-authoring.md` | Current per-milestone contract authoring discipline. |
| R5 | `docs/slo/templates/runbook-template_v_4_template.md` | Current v4 runbook rows for resource bounds, invariants, static analysis, refactor budget, and evidence. |
| R6 | `skills/slo-execute/SKILL.md` | Current BDD-first, allow-list, and smallest-safe-change execution contract. |
| R7 | `skills/slo-verify/SKILL.md` | Current runtime QA and security Pass 4 behavior. |
| R8 | `skills/slo-critique/SKILL.md` and personas | Current adversarial plan review behavior. |
| R9 | `skills/slo-ticket-*` and `docs/slo/templates/ticket-contract-template_v_1.md` | Current ticket-sized SLO contract shape. |
