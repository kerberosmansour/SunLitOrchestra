# Lessons Learned — loops Milestone 5

## What changed
- Extended `skills/slo-resume/SKILL.md` so the orientation reads (a) the runbook's Milestone Tracker first, then (b) the optional "Carry-forward from prior retros" section if present, classifies the recommended next action with a lane (`micro | milestone | fresh-runbook`), surfaces the top 3 carry-forward items inline plus a `... N more` summary, fence-wraps any quoted issue body, and stays strictly read-only.
- Added structural-contract tests at `crates/sldo-install/tests/e2e_loops_m5.rs` (10 tests covering carry-forward read, lane vocabulary, top-3 cap, no-auto-start invariant, empty-section handling, blocked-milestone handling, fence-wrap rule, and a backward-compat guard for M4's runbook template).

## Design decisions and why
- Strengthened the existing `/slo-resume` verb instead of minting a new `/slo-help` skill — the runbook explicitly called out the BMAD adoption lesson "one obvious 'what next?' surface, not more verbs". Added a structural-contract test that asserts no `skills/slo-help/` directory exists, so future agents do not accidentally re-fragment the orientation surface.
- Compact output format with the `lane:` field on the very first line. The runbook's anti-process-theatre check forbids dumping the carry-forward table; capping inline at top 3 plus a `... N more` summary is the structural rule.
- The skill explicitly states: when a runbook has a "Carry-forward from prior retros" section, prefer those rows over a live `gh issue list` query. Reasoning: the runbook section is the user-curated view; the live `gh` query is the auto-curated view; orientation should respect the user's curation.
- Fence rule on quoted issue body snippets is the same `~~~text` discipline used by `/slo-architect`'s SECURITY.md template, M3 issue-filing-discipline, and M4 carry-forward surface. Loaded across the whole loop end-to-end.
- The "Empty carry-forward section is fine" rule keeps an M1 (first milestone of a new runbook) from breaking under the new orientation — explicit in prose AND asserted by the structural-contract test.

## Mistakes made
- None at this milestone — five test failures on initial run were the EXPECTED red→green flow. Test stubs first, then the SKILL.md edit, then re-run.

## Root causes
- N/A this milestone.

## What was harder than expected
- The runbook explicitly forbids inventing a new public verb but called for a richer orientation. Resolved by structuring the SKILL.md as additive: the existing tracker-first behavior is preserved AND new carry-forward + lane logic are layered on top, so `/slo-resume` against an old runbook (no carry-forward section) still produces the original output.

## Naming conventions established
- Lane vocabulary is locked in three places now (M3 issue-filing reference file, M4 SKILL.md + runbook template, M5 SKILL.md). All three use the exact triple `micro | milestone | fresh-runbook`. Drift would be caught by both `runbook_template_carry_forward_lane_column` (M4 test) and `slo_resume_lane_vocabulary_documented` (M5 test).
- Output skeleton: `You are at Milestone N (<title>), status: <status>, lane: <lane>.` — the lane is on the first line, not buried.

## Test patterns that worked well
- Per-rule grep tests with multiple acceptable phrasings (e.g., `output_stays_short` accepts `top 3` OR `top-3`, AND any of `one screen` / `compact` / `short message`). Robust to legitimate prose variation while still failing when the rule is removed.
- A negative-existence test (`slo_resume_no_new_skill_minted`) that fails if `skills/slo-help/` ever exists. Cheap to write, expensive to forget.

## Missing tests that should exist now
- A property-style test that asserts `/slo-resume` output never exceeds N lines (or N characters). The runbook calls for "one screen"; today the test asserts the rule is documented but not the output shape itself.
- A runtime test that synthesizes a runbook with 12 carry-forward rows and asserts `/slo-resume` output truncates to top 3 + `... 9 more` exactly. Same shape as M4's BDD scenario `tm-loops-abuse-8`.

## Rules for the next milestone
- N/A — M5 is the last milestone in this runbook. The next iteration of the lessons loop happens when the next runbook (after this one ships) opens and `/slo-execute` Step 1.5 reads any retro-derived issues that get filed against this repo.

## Template improvements suggested
- The runbook template's M5-style "this is the last milestone" closure could include an optional "post-runbook" section that names the next iteration of the loop the runbook just shipped. For loops-and-lessons-closure, the next iteration is "the next runbook to open will be the first to consume `/slo-resume`'s carry-forward orientation". Documenting this explicitly inside the runbook would make the loop's continuity visible.
- The five-milestone hard cap from `/slo-plan` worked here. The runbook fits cleanly in 5 milestones with no scope spillover.
