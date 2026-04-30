# Lessons Learned — loops Milestone 1

## What changed
- Authored `docs/LOOPS-ENGINEERING.md` with the four engineering loops (sprint, security-tuning, lessons, library-feedback) plus an outcome-first "Start here" orienter.
- Added a new "Feedback loops" section to `docs/ARCHITECTURE.md` cross-linking the engineering and (forthcoming) business loops.
- Added a one-line back-link to `docs/LOOPS-ENGINEERING.md` at the bottom of every engineering SKILL.md cited in a loop section (13 skills).
- Added structural-contract tests at `crates/sldo-install/tests/e2e_loops_m1.rs` (5 tests, all green).
- Added the doc to the README documentation index.
- Pre-flight maintenance: fixed three pre-existing baseline failures (stale test references to `RUNBOOK-SAST-RULEGEN-A.md` plus removed `sldo-plan` / `sldo-run` binaries) so the green-baseline gate could be confirmed.

## Design decisions and why
- ASCII box-drawing diagrams over Mermaid — matches the existing `docs/design/scanner-orchestration-overview.md` style; renders identically in GitHub web + terminal without a JS pass.
- Outcome-first "Start here" table at the top of the doc — explicitly maps a question to a loop and to a first skill, so an interrupted user can act in one screen rather than learning loop theory first.
- Library-feedback loop documented as a placeholder with a "ships in Runbook 4" footnote rather than removed silently — the loop exists today through the lessons-loop's `upstream-OSS` classification; the dedicated `/slo-sec-libs` skill is the next iteration.
- Per-skill back-links go at the **bottom** of each SKILL.md after a `---` separator — adding text near the top would compete with the description for skill-loader attention, while a footer link is a cheap discoverability win.
- Skipped a per-loop "rationale" section — anti-process-theatre check: every visible field must reduce user decisions. The user-visible-outcome line already justifies the loop.

## Mistakes made
- Initial baseline run found two stale test references blocking green; the runbook entry rules required a green baseline. Had to detour through small maintenance fixes before M1 work could begin.
- First Bash attempt to batch-append the back-links used `declare -A` under `zsh`, which silently rejects bash-style associative arrays. Switched to `case` statements via `/bin/bash -c`.

## Root causes
- The 2026-04 cleanup commit (`8072a3e`) re-pointed two structural-contract tests at `RUNBOOK-SAST-RULEGEN-A.md`, but that runbook was never authored. The tests were authored against a planned-but-never-shipped artifact and never re-pointed when the plan changed.
- The `sldo-plan` / `sldo-run` binaries were removed in the same cleanup, but `tests/e2e_research_m7.rs` retained two backwards-compat smoke tests that drove those binaries — the test was on a different surface than the cleanup hit.

## What was harder than expected
- Identifying which engineering skills participate in which loop required reading every cited SKILL.md to confirm the loop assignments. The runbook listed four loops with named skills, but a few skills (e.g., `/slo-architect`) participate in more than one — settled by listing the secondary loop in the back-link line rather than picking one.

## Naming conventions established
- Loop section anchors: `#sprint-loop`, `#security-tuning-loop`, `#lessons-loop`, `#library-feedback-loop` (kebab-case from the section heading; matches GitHub's auto-anchor rules).
- Per-skill back-link footer pattern: `**Loops**: <loop names> — see [docs/LOOPS-ENGINEERING.md#<anchor>](../../docs/LOOPS-ENGINEERING.md#<anchor>).`
- Structural-contract test file naming: `crates/sldo-install/tests/e2e_loops_m<N>.rs` (matches the runbook's declared test file naming).

## Test patterns that worked well
- Writing the structural-contract test FIRST and confirming it failed for the right reason (file missing, then cross-reference missing) before authoring the doc and adding the cross-references. Caught at least two cross-reference omissions on first run.
- Using `grep -q "LOOPS-ENGINEERING.md"` in the back-link batch script to make the operation idempotent — re-running the script does nothing on already-linked files.

## Missing tests that should exist now
- A test that asserts every loop section in `docs/LOOPS-ENGINEERING.md` has a diagram (ASCII fenced block). Today the schema-marker test catches headers but not the diagram requirement directly; the BDD scenario implies it.
- A negative test that asserts a SKILL.md NOT cited under any loop (e.g., business-pack skills) does NOT contain a `LOOPS-ENGINEERING.md` link, to prevent over-cross-linking.

## Rules for the next milestone
- M2 (`docs/LOOPS-BUSINESS.md`) must match M1's diagram style — ASCII box-drawing, not Mermaid.
- M2 must follow M1's per-skill back-link footer pattern verbatim, but pointing at `LOOPS-BUSINESS.md`.
- M2's anti-PII rule (no real interview quotes) is the structural difference vs M1; bake it into the structural-contract test, not just a comment.
- The "Start here" orienter is load-bearing — keep it as the very first section after the title, not buried.

## Template improvements suggested
- The runbook template's "Files Allowed To Change" tables for milestones that touch multiple SKILL.md files at once should explicitly list "skills/<each-engineering-skill>/SKILL.md" with no expectation that the agent will enumerate individual files. M1's "each SKILL.md cited as part of a loop" left the agent to enumerate the cited list from the doc itself — which worked here but is brittle.
- Pre-flight should explicitly check for stale test references against deleted runbook fixtures. The 2026-04 cleanup ran past this check.
