# Lessons Learned — fowler-ai-arch Milestone 1

## What changed

- `/slo-architect` now documents two additive outputs: `docs/slo/design/<slug>-reversibility.md` and `docs/slo/design/<slug>-code-map.md`.
- The architect method now requires a reversibility matrix, a brownfield code map, a greenfield N/A path, and idempotency prompts before overwriting those artifacts.
- Architect evals now cover reversibility/code-map success and ambiguous brownfield context.
- `crates/sldo-install/tests/e2e_fowler_ai_arch_m1.rs` locks the Markdown contract.

## Design decisions and why

- Keep the M1 change inside `SKILL.md` instead of adding a new reference file because the milestone allow-list only permitted the skill and eval files.
- Make the code map N/A-capable for greenfield work because forcing fake brownfield context would train agents to invent architecture evidence.
- Test exact output paths and selected behavior words instead of snapshotting the whole skill file so future prose edits stay possible.

## Mistakes made

- The runbook tracker was not flipped to `in_progress` before the first M1 test write; the final evidence records the actual branch and test sequence, but future milestones should update the tracker before adding the failing test.
- `cargo fmt --all -- --check` is red on unrelated pre-existing Rust formatting drift. M1 did not format out-of-scope files.

## Root causes

- The current repo had uncommitted planning artifacts before execution began, so branch hygiene and commit scoping need extra care throughout the runbook.
- The formatter drift appears in old tests outside this milestone's allow-list, so a strict milestone cannot fix it without widening scope.

## What was harder than expected

- Balancing concise `SKILL.md` prose with enough specificity for structural tests; the result is a compact Step 3.6 instead of a broad methodology extraction.

## Naming conventions established

- Fowler structural tests use `crates/sldo-install/tests/e2e_fowler_ai_arch_m<N>.rs`.
- Lessons and verification artifacts use the runbook prefix `fowler-ai-arch`.

## Test patterns that worked well

- BDD-first presence tests against concrete path strings made the expected failure clean before implementation.
- Eval-file assertions caught the non-obvious requirement that behavior expectations change alongside `SKILL.md`.

## Missing tests that should exist now

- A future runtime harness could invoke `/slo-architect` directly and inspect generated artifacts; current Codex host limitations make structural tests the portable guard.

## Rules for the next milestone

- Flip the next milestone tracker row to `in_progress` before writing the failing test.
- Keep skill-local references and repo mirror templates aligned when M2 updates `/slo-plan`.
- Treat formatter failures outside the milestone allow-list as recorded external drift, not a reason to touch unrelated Rust tests.

## Template improvements suggested

- Add an explicit repo-hygiene evidence row to future runbook templates so branch creation is not only implicit in `/slo-execute`.
