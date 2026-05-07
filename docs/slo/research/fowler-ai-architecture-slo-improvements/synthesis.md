---
name: fowler-ai-architecture-slo-improvements
researched: 2026-05-07
incomplete: false
---

# Research Synthesis — Fowler AI Architecture SLO Improvements

The SLO sprint loop already embodies Fowler's feedback-loop argument: it plans one runbook, critiques before execution, and then cycles each milestone through execute, verify, and retro. The design must preserve that loop rather than inventing a parallel workflow because `docs/LOOPS-ENGINEERING.md` already defines sprint, ticket, security-tuning, and lessons loops as the user-visible way SLO improves itself (R1).

The biggest missing contract is architectural reversibility. Fowler's writing and the Agile Architecture talk both frame architecture around hard-to-change decisions and reducing irreversibility, including experiments that test how hard future changes would be. The design must add a reversibility artifact to `/slo-architect` because the current skill locks stack and interfaces but does not require a per-decision reversibility tactic or rollback proof (S1, S5, S6, R3).

The second missing contract is exemplar code. Dishman/Fowler's agile architecture summary says related good code examples should be highlighted when stories are defined because developers copy what is visible in the codebase. The design must add exemplar and anti-exemplar rows to `/slo-plan` and ticket contracts because current Contract Blocks list files to read and change but do not identify which code shape an AI agent should copy (S6, R4, R9).

The third missing contract is brownfield comprehension before modification. Thoughtworks Radar says GenAI-assisted understanding of legacy codebases is now practical/default and highlights code knowledge graphs/RAG for poorly documented systems. The design must add a brownfield code-map output before implementation because SLO's current brownfield handling detects manifests and preserves stack, but does not require a system map, "four objects" coherence test, or dangerous/legacy areas not to copy (S3, R3).

The fourth missing contract is true refactoring semantics. Fowler defines refactoring as small behavior-preserving transformations with tests, and the current v4 template has only a three-choice refactor budget. The design must add a refactoring discipline reference and microstep/evidence requirements because otherwise "targeted refactor" can be misused to hide broad behavior changes (S2, R5).

The fifth missing contract is AI nondeterminism tolerance. The user-provided Fowler interview notes emphasize the shift from deterministic software to nondeterministic AI behavior, and Thoughtworks Radar warns that AI tools can accelerate cognitive debt when teams lack mental models. The design must require explicit AI tolerance contracts for `ai_component: true` systems because SLO currently gates AI-specific security threat-model sections but does not define acceptable behavioral variance, golden evals, or "must never" outcomes (S4, S7, R3, R7).

The final runbook should be five milestones: `/slo-architect` reversibility + code map; `/slo-plan` exemplar/refactoring rows; AI tolerance across architect/plan/verify; `/slo-critique` architecture coherence checks; ticket-flow parity. The design must split ticket parity into its own milestone because changing sprint-flow templates without the issue-sized flow would create two SLO planning standards that drift immediately (R1, R9).
