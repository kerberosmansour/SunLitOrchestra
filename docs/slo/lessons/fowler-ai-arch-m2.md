# Lessons Learned — fowler-ai-arch Milestone 2

## What changed

- `/slo-plan` now surfaces exemplar and anti-exemplar rows as Contract Block sentinels.
- The milestone-authoring methodology now requires exemplar, anti-exemplar, and refactoring-discipline rows with N/A-with-reason paths.
- The v4 runbook template mirrors include `Exemplar code to copy`, `Anti-exemplar code not to copy`, and `Refactoring discipline`.
- `skills/slo-plan/references/refactoring-discipline.md` defines true behavior-preserving refactoring proof.
- `crates/sldo-install/tests/e2e_fowler_ai_arch_m2.rs` locks the M2 contract.

## Design decisions and why

- Add `Refactoring discipline` as a Contract Block row in addition to preserving the existing `Refactor budget` heading. The budget says whether refactoring is permitted; the row says what proof is required when it is.
- Keep docs-only N/A wording in the methodology so generated runbooks can be honest without inventing exemplars.
- Keep the docs and skill-local v4 templates byte-aligned so installed `/slo-plan` behavior matches repo docs.

## Mistakes made

- The first formatter check caught two line-wraps in the new M2 Rust test. Fixed only the new file and left unrelated formatter drift untouched.

## Root causes

- Rustfmt covers the entire workspace, so any new test must be locally rustfmt-clean even when old unrelated files remain red.

## What was harder than expected

- The existing `Refactor budget` is outside the Contract Block while the new proof row belongs inside it; the split is useful but needs explicit prose so agents do not treat the new row as a replacement.

## Naming conventions established

- Refactoring discipline reference path: `skills/slo-plan/references/refactoring-discipline.md`.
- M2 verification report: `docs/slo/verify/fowler-ai-arch-m2.md`.

## Test patterns that worked well

- A single test asserted the two v4 template mirrors together, which made drift visible immediately.
- The existing `e2e_v4_template` mirror test stayed useful as a compatibility guard after the row additions.

## Missing tests that should exist now

- A future runtime harness should ask `/slo-plan` to draft a docs-only milestone and assert it emits the N/A exemplar/refactoring rows without prompting.

## Rules for the next milestone

- M3 must update both v4 template mirrors in the same patch for the AI tolerance row.
- Keep the AI tolerance row gated; non-AI milestones must have an explicit `N/A — no AI component` path.
- Continue writing Semgrep outputs to `/tmp` to avoid repo-local Pass 4 artifacts.

## Template improvements suggested

- Consider adding a short glossary note distinguishing `Refactor budget` from `Refactoring discipline` if future users conflate them.
