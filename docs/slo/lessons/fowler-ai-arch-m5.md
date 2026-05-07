# Lessons Learned - fowler-ai-arch Milestone 5

## What changed

- Ticket contract templates now include compact parity rows for reversibility, exemplar code, anti-exemplar code, refactoring discipline, and AI tolerance.
- `/slo-ticket-plan`, `/slo-ticket-execute`, and `/slo-ticket-verify` now consume and restate those rows.
- `docs/skill-pack-catalog.md` and `docs/ARCHITECTURE.md` now orient readers to ticket parity without becoming duplicate contract catalogs.
- `crates/sldo-install/tests/e2e_fowler_ai_arch_m5.rs` locks the M5 ticket-flow contract.

## Design decisions and why

- Keep ticket rows compact. Ticket-sized work should inherit the vocabulary of sprint-flow contracts without copying runbook-only sections.
- Keep N/A paths first-class. Simple docs-only, no-refactor, and non-AI tickets should still be valid when they explain why a row does not apply.
- Test both template mirrors. The docs template and skill-local template are separate files, so the parity test reads both.

## Mistakes made

- An early M5 test draft pulled README acknowledgements into the milestone scope. That was removed before commit so M5 remains limited to ticket-flow parity and catalog / architecture orientation.

## Root causes

- "Docs orientation" can sprawl into adjacent project prose if the test boundary is not explicit. Structural tests should name the exact docs that matter for the milestone.

## What was harder than expected

- Balancing parity with compactness. The ticket flow needed the new safety rows, but not the full v4 runbook framing.

## Naming conventions established

- Ticket parity rows use the same row names as sprint-flow rows where possible.
- M5 verification report: `docs/slo/verify/fowler-ai-arch-m5.md`.

## Test patterns that worked well

- A line-count cap and forbidden-section checks kept the ticket template from becoming a copied runbook.
- A word-diff check made the mirror-template changes easy to inspect.

## Missing tests that should exist now

- A future issue-workpad integration test should run `/slo-ticket-plan` output through `/slo-ticket-execute` and `/slo-ticket-verify` fixtures to prove the compact rows survive a full ticket lifecycle.

## Rules for the next runbook

- Scope documentation-orientation tests to named files only.
- When adding contract rows to the sprint flow, add the ticket-flow parity test in the same runbook.

## Template improvements suggested

- Consider a shared helper in structural tests for asserting mirrored template rows across skill-local and docs mirrors.
