# Lessons Learned — outcome-first Milestone 3

## What changed
- `/slo-verify` gained a leading, highest-authority **Pass 0: Outcome Validation** (inserted non-renumbering ahead of Pass 1; Passes 1–6 unchanged), the override rule ("fails even if Passes 1–6 are green"), an extended Gates list, and a new `references/outcome-validation-pass.md`. Added `outcome_first_m3_verify.rs` (SHA-pinned) and the theme-A dogfood (`outcome_first_dogfood.rs` + fixture pair).

## Results vs thesis
- **Theme-A leading-metric proof achieved early**: the dogfood mechanically shows the gate blocks an unproven milestone and passes a remediated one. The runbook's §5A lagging metric ("the gate changes an outcome the old loop would have passed") is now demonstrated on a fixture — the strongest evidence available pre-real-feature-dogfood.

## Design decisions and why
- **Pass 0, not renumber (DW-001 closed)** — inserting "Pass 0" ahead of Pass 1 left every `Pass 4/5/6` citation valid; all 5 reader tests stayed green. The renumber seam was *avoided*, not managed.
- **Dogfood as a fixture-pair gate test** — Pass 0 is agent-run skill prose, not a Rust function, so the honest executable proof is a `gate_blocks()` re-implementation of the two hardest criteria over a bad/good fixture pair (the measurement-loop failure-bar precedent). This is non-vacuous: bad blocks, good passes.

## Mistakes made
- Two self-caught authoring nits within M3 (not runtime bugs): (1) markdown emphasis `*applicable*` broke a plain-substring test match — removed the inner italics; (2) the bug-flow markers needed exact wording ("reuses the existing" / "regression-test-first"). Both fixed before close; the SHA was re-pinned after each SKILL.md edit.

## Root causes
- Plain-substring structural assertions are sensitive to markdown emphasis inside the matched phrase. Rule below.

## What was harder than expected
- Keeping the SHA-pin in lockstep across two edit rounds — each SKILL.md change invalidates the pin until recomputed. Cheap but must not be forgotten.

## Naming conventions established
- Non-renumbering pass insertion idiom: "Pass 0" as a leading highest-authority pass.
- Dogfood fixtures: `xtasks/sast-verify/tests/fixtures/<prefix>_dogfood/{blocked,proven}.md`.

## Test patterns that worked well
- **Fixture-pair non-vacuity** (bad fails, good passes) for proving a gate "fires" when the gate itself is agent-run prose. The `gate_blocks()` helper is a faithful, minimal re-implementation of the two hardest mechanically-checkable criteria.

## Missing tests that should exist now
- None for M3.

## Rules for the next milestone
- **When a structural assertion matches a plain phrase, keep that phrase free of markdown emphasis** (`*`, `_`) in the source prose — or assert on a deliberately emphasis-free token.
- **Re-pin the SHA after EVERY edit to a pinned SKILL.md**, not just the last one.
- **M4 edits `slo-critique` which is double-SHA-pinned** (`sap_imp_m5_agents.rs` + `slo_tm_m2_consumers.rs`) — update BOTH constants in lockstep, preserve `## Rotation order` + 4 personas, and only the constant VALUE may change in those two files (tm-outcome-first-abuse-4). Also SHA-pin `slo-retro` + `slo-execute` fresh.

## Template improvements suggested
- None.

---

## Detected Work Ledger disposition (this milestone)
- **DW-001** (Pass-0 non-renumber seam) — **CLOSED**: realized as the non-renumbering Pass 0; `fix_now` done in M3 as planned.
- **DW-002** (pre-existing clippy debt) — unchanged; `file_github_issue` (slo-process), at ship.
