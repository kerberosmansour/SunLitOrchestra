# Lessons Learned — loops Milestone 2

## What changed
- Authored `docs/LOOPS-BUSINESS.md` covering the four business loops (user-interview, GTM, pricing, founder-check) plus an outcome-first "Start here" orienter.
- Added a one-line back-link to `docs/LOOPS-BUSINESS.md` at the bottom of every cited business SKILL.md (15 skills).
- Added structural-contract tests at `crates/sldo-install/tests/e2e_loops_m2.rs` (5 tests including a backward-compat guard for M1 + a PII-pseudonym discipline test).
- Updated `README.md` docs index to point at the new business-loops doc.

## Design decisions and why
- Stuck to the same four loops the runbook called out (user-interview, GTM, pricing, founder-check). The runbook left "fundraise loop, cofounder loop, hiring loop, legal-triage loop" as open additions; on inspection, those collapse into the four canonical loops as supporting flows (e.g., hiring is a step in the founder-check loop with the IR35 gate, not a separate loop). Keeping it at four matches M1's count and avoids inventing loops that have no skill-flow anchor.
- Followed M1's per-skill back-link footer pattern verbatim, only swapping `LOOPS-ENGINEERING.md` for `LOOPS-BUSINESS.md`. Bidirectional consistency matters more than a more clever pattern would.
- Used Alice / Bob pseudonyms in the single example sentence and added a structural-contract test asserting the rule. The runbook's `tm-loops-abuse-2` mitigation is "no real interview quotes; all examples use Alice / Bob" — the test makes that rule load-bearing rather than a comment.
- Skipped a separate "fundraise loop" — `/slo-fundraise` participates in the Pricing loop (qualifying-trade pre-check) and the Founder-check loop (SAFE / pitch / AA pre-check), neither of which justifies a third loop. Anti-process-theatre check passed.
- Kept the "Start here" table to six rows; if it grows past 8-10 it stops being scannable. Rows are sorted by frequency-of-need rather than skill order.

## Mistakes made
- First test stub omitted a backward-compat guard for M1's invariants — added `m1_engineering_loops_doc_unchanged_and_cross_linked` after re-reading the runbook's "Compatibility commitments" row.

## Root causes
- M2's compatibility commitment "M1 work unchanged" needs an explicit test, not just a hand-checked invariant. Without the guard, an M2 edit that accidentally clobbers M1's section header would slip past CI.

## What was harder than expected
- Mapping skills to loops without over-claiming. `/slo-fundraise` legitimately participates in two loops (Pricing + Founder-check); decided to keep it in both rather than pick the dominant one — clarity beats single-membership in this case. Same call for `/slo-pricing` and `/slo-metrics` (Pricing + GTM).

## Naming conventions established
- Loop section anchors: `#user-interview-loop`, `#gtm-loop`, `#pricing-loop`, `#founder-check-loop`.
- Pseudonym pair for all interview-quote examples in `docs/biz-public/`: Alice (founder) / Bob (interviewee). Documented in the doc's preamble + asserted by the structural-contract test.

## Test patterns that worked well
- The PII pseudonym test pattern: trigger only when an example marker is present (`Example:` / `e.g., "` / `interview quote`). This way the test does not nag a doc that has zero examples but actively guards docs that DO show interview content.
- Reusing the M1 cross-reference grep helper — single line of test logic, idempotent re-runs.

## Missing tests that should exist now
- A loop-coverage test that asserts every business SKILL.md NOT cited in a loop also does not contain a `LOOPS-BUSINESS.md` link (negative invariant) — same gap noted at M1 close.
- A test that asserts the "Start here" table has a maximum-row cap (8-10) so future additions trip the test rather than silently turning the orienter into a dump.

## Rules for the next milestone
- M3's `/slo-retro` extension introduces issue filing. Follow the runbook's argv-list discipline + NO `--repo` flag rule **literally** — both have backward-compatible structural-contract tests already in M3's contract block.
- The lessons file and the M3 issue-filing flow are interleaved: the lessons file MUST be written FIRST (always), and issue filing happens AFTER. M3's extension to `/slo-retro` must preserve that ordering or the dependency graph breaks.
- Keep the `LESSONS-BACKLOG.md` row schema exactly as the runbook specifies — the cross-session dedupe via body_sha256 only works if the schema is uniform.

## Template improvements suggested
- The runbook template's per-milestone "Files Allowed To Change" schema would benefit from a `Reason:` column for entries like "each business SKILL.md cited as part of a loop" — it would force the agent to enumerate the cited list explicitly rather than infer it on the fly.
- `tm-loops-abuse-2` mitigation belongs in the runbook template's threat-model row schema as a first-class structural-rule field rather than a one-line comment in the contract block.
