---
name: fowler-ai-architecture-slo-improvements
researched: 2026-05-07
incomplete: false
---

# Research Dossier — Fowler AI Architecture SLO Improvements

## Market

This is an internal product-improvement runbook for SunLit Orchestra's public skill pack. The buyer/user is Sherif as maintainer and any engineer using SLO skills to let an AI agent plan and execute changes safely. The relevant market signal is not competitor pricing; it is the current shift in AI-assisted delivery practices and the need to retain software-engineering discipline as AI accelerates code generation (S4).

## Direct competitors

| Name | Price | Key feature | Gap vs our wedge |
|---|---:|---|---|
| Generic AI coding agents | Varies by provider | Generate and modify code quickly | Usually do not carry project-specific runbook contracts, evidence logs, or artifact feedback loops. |
| Thoughtworks Technology Radar practice guidance | Free public guidance | Curates techniques such as GenAI-assisted legacy understanding | Guidance is not executable inside Sherif's SLO skill contracts. |
| Agile architecture practice / architecture kata style work | Mostly consulting/training | Makes architecture collaborative and adaptable | Does not produce installable Codex/Copilot/Claude skill-pack artifacts. |

## Adjacent tools

| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|
| Architecture decision records (ADR) templates | Record decisions and consequences | Could capture reversibility, but ADRs do not enforce runbook/test gates by themselves. |
| Code owners / reviewer rules | Route review to maintainers | Could enforce human review, but not BDD-first planning or AI tolerance contracts. |
| Static analysis / dependency scanners | Catch known bad patterns after code exists | They do not teach the planning step which exemplar code to copy or which decisions must stay reversible. |

## Technical prior art

- Fowler frames architecture as the core/hard-to-change elements of the system and argues evolutionary design needs refactoring, simple design, patterns, and ongoing design will (S1).
- Fowler's refactoring definition is specifically small behavior-preserving transformations with frequent testing, not broad cleanup or feature work under a nicer label (S2).
- Dishman/Fowler's agile architecture keynote summary emphasizes reducing irreversibility, adaptable architecture communication, hands-on architects, and code/architecture exemplars that developers copy (S6).
- Thoughtworks Radar says GenAI-assisted legacy understanding has become an essential technique and highlights knowledge-graph/RAG approaches for less cohesive codebases (S3).
- Thoughtworks Radar also warns that AI-generated code can create codebase cognitive debt when teams adopt solutions without mental models, making systems harder to reason about, debug, and evolve (S4).

## Regulatory / legal

- None directly apply to this runbook because it modifies public skill-pack Markdown and Rust structural tests, not confidential founder artifacts or regulated data flows. Existing SLO security and business-pack gates still apply if downstream runs touch those domains (R1, R7).

## Open questions that research did not answer

- Whether the exact Fowler AI interview transcript would add more nuance than the user-provided notes. The YouTube page is accessible as a locator (S7), but this dossier relies on official Fowler/Thoughtworks pages and InfoQ's textual summary for source-backed claims.
