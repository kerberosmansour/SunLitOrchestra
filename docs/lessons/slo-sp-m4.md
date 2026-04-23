# Lessons Learned — slo-sp Milestone 4

## What changed
- `/slo-architect` and `/slo-plan` skills authored. Together they take idea + research + (optional TLA+) and produce a v3 runbook.
- 7 new E2E tests pinning the static contracts: tla_required decision, interface lockdown, one-shot-refusal, five-milestone cap, v3 template reference.

## Design decisions and why
- **`tla_required` is decided by `/slo-architect`, not `/slo-plan`.** Rationale: the architect is the one with the full context of the design; the planner is downstream. Single source of truth prevents drift.
- **`/slo-plan` is interactive, not batch.** Per-milestone confirmation. Refuses one-shot. This is the single biggest departure from `sldo-plan` (the Rust binary). Rationale: one-shot runbooks are the documented failure mode that motivated the rebuild.
- **Five-milestone cap.** If scope needs more, suggest splitting. Rationale: runbooks longer than five milestones become aspirational; they don't get executed straight through.
- **Interfaces locked at M4-plan-stage.** The `interfaces.md` file from `/slo-architect` feeds into every subsequent milestone's Compatibility Checklist. This is how we prevent "oh I'll just rename that command" creep during execution.

## Mistakes made
- First draft of `/slo-plan` tried to describe the "for each milestone, do X" loop in code-style pseudocode. Replaced with natural prose because the skill body is read by Claude as context, not executed as logic.

## Root causes
- Thinking of the SKILL.md as a script rather than a role brief. Fixed.

## What was harder than expected
- Calibrating when to stop pushing back. If a user really wants 7 milestones, the skill body says "suggest splitting" — not "refuse until they split." Soft pushback, not brittle refusal. Got this wording right on the third try.

## Naming conventions established
- Design docs: `docs/design/<slug>-<artifact>.md`. Examples: `<slug>-overview.md` (summary + `tla_required`), `stack-decision.md`, `interfaces.md`, `<slug>-verified.md` (post-TLA+).
- Runbook path: `docs/RUNBOOK-<FEATURE-UPPER>.md`. Matches existing SLO convention.

## Test patterns that worked well
- Static grep-based tests continue to be useful. They catch the "someone removed the one-shot refusal" type regression without needing runtime.

## Missing tests that should exist now
- A golden runbook produced by the skill that can be diffed against v3 template section-by-section. Would require non-interactive Claude Code.
- A test that `/slo-architect` detects the right stack in a brownfield repo.

## Rules for the next milestone (M5 — /slo-tla)
- M5 is the hard one. Budget more time. The `/slo-tla` skill body already drafted is ~150 lines; do not let it bloat past 200.
- `tools.toml` ships with `sha256 = "UNSET"`. M5's Definition of Done does NOT require computing real SHAs; that happens on first real use (maintainer populates before publishing). Document this clearly.
- Do not attempt to bundle a TLC jar. The catalog Q4 decision is: fetch on first use, verify, cache.

## Template improvements suggested
- Add an explicit "Sanity checks before proceeding" block to the skill-authoring template. `/slo-architect` grew one organically ("confirm diagram with user before step 2"). Would make other skills more robust.
