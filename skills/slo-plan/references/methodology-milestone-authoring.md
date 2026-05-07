---
name: slo-plan-methodology-milestone-authoring
source_skill: skills/slo-plan/SKILL.md
description: Per-milestone authoring procedure for /slo-plan.
---

# /slo-plan Methodology — Milestone Authoring

For milestone N, write the full section:

1. **Goal** — one sentence: what capability exists at the end that didn't before.
2. **Context** — 2–4 sentences, reference specific files.
3. **Important design rule** — one key decision.
4. **Refactor budget** — one of three options.
5. **Contract Block** — the full table. Base rows: Inputs, Outputs, Interfaces touched, Files allowed to change, Files to read before changing, New files allowed, New dependencies allowed, Migration allowed, Compatibility commitments, Forbidden shortcuts. Add these architecture and safety rows:
   - **Exemplar code to copy**: concrete paths and code shapes worth following, preferably from `docs/slo/design/<slug>-code-map.md`. If no code exemplar applies, use `N/A — docs-only` or `N/A — no brownfield exemplar needed, see <reason>`; silent omission is forbidden.
   - **Anti-exemplar code not to copy**: concrete risky paths or patterns agents must not imitate. If none applies, use `N/A — no anti-exemplar identified, see <reason>`.
   - **Refactoring discipline**: when **Refactor budget** is anything except `No refactor permitted beyond direct implementation`, cite [`references/refactoring-discipline.md`](refactoring-discipline.md) and require behavior-preserving microsteps with pre-test and post-test proof.
   - **AI tolerance contract**: required when the milestone introduces, modifies, or verifies AI/LLM behavior (`ai_component: true`, prompt/tool-choice changes, eval harnesses, generated-output behavior, or AI-agent flows). Cite [`references/ai-tolerance-contract.md`](ai-tolerance-contract.md) and fill accepted variance, deterministic boundary, eval evidence, retry / fallback, must-never outcomes, and bounded sample budget. For non-AI work, write `N/A — no AI component`.
   - **Data classification**: one of the fixed four values `Public`, `Internal`, `Confidential`, `Restricted`. The full enum and its rules live in [`references/proactive-controls-vocabulary.md`](proactive-controls-vocabulary.md). A milestone that handles `Confidential` or higher data MUST additionally cite a relevant control in the next row and include at least one abuse-case scenario.
   - **Proactive controls in play**: stack-aware vocabulary from [`references/proactive-controls-vocabulary.md`](proactive-controls-vocabulary.md). For Rust-axum targets with `security_libs_required: true`, cite SunLitSecurityLibraries crate names + OWASP C-numbers (e.g. `C5 secure_boundary::SecureJson`). For Pulumi/AWS, cite Hulumi components (e.g. `@hulumi/baseline.aws.SecureBucket`). For other stacks, cite OWASP Proactive Controls v3 category names directly (C1–C10).
   - **Abuse acceptance scenarios**: pointer into the milestone's BDD table for the specific abuse-case rows seeded from `docs/slo/design/<slug>-threat-model.md` and the example pool at [`references/abuse-case-examples.md`](abuse-case-examples.md). Row format includes a `tm-<slug>-abuse-N` citation back to the threat-model row. **Required when the milestone introduces a new surface** (endpoint / IPC handler / file write / subprocess / outbound request / persisted state). When the milestone introduces no new surface (pure-documentation, refactor-only), fill the row with `N/A — no new surface introduced, see <reason>` — silent omission is forbidden.
6. **Out of Scope / Must Not Do** — explicit non-goals.
7. **Files Allowed to Change** — the table with planned changes.
8. **Step-by-Step** — numbered, 10 or fewer.
9. **BDD Acceptance Scenarios** — cover happy path, invalid input, empty state, dependency failure, and whichever of {retry, concurrency, persistence, backward compat, **abuse case**} apply. The `abuse case` category is required whenever the milestone introduces a new surface; the rows are seeded from `docs/slo/design/<slug>-threat-model.md` via [`references/abuse-case-examples.md`](abuse-case-examples.md). Every abuse case cites a threat-model row (`tm-<slug>-abuse-N`) and names a concrete attacker-role + step + outcome. N/A-with-reason is acceptable only when no new surface is introduced.
10. **Regression Tests** — specific tests that must still pass.
11. **Compatibility Checklist** — checkboxes.
12. **E2E Runtime Validation** — test functions and pass criteria.
13. **Smoke Tests** — manual verification steps.
14. **Evidence Log** — copy the template.
15. **Definition of Done** — the standard checklist.

After writing milestone N, confirm with the user:

- Does the scope feel achievable in one pass?
- Is the file allow-list complete?
- Are the BDD scenarios specific enough?

Do not start milestone N+1 until N is confirmed.
