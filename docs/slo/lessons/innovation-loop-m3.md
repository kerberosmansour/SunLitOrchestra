# Lessons Learned — innovation-loop Milestone 3

## What changed
- Shipped the converge + measure phases: `skills/slo-pattern/SKILL.md` (§5 convergent — name tricks, cite probe IDs, ≤5 cap, next-curve + DICEE) and `skills/slo-precision/SKILL.md` (§6 measurement — handles + accept/kill thresholds + resource bounds + security invariants). Test `innovation_loop_m3_converge.rs` (4 assertions). Catalog 44→46.

## Design decisions and why
- **Both thresholds required, not one.** `/slo-precision` must carry an accept AND a kill threshold per claim — being able to stop is as important as being able to win. The test asserts both.
- **Cite-probe-IDs cap is the anti-fabrication gate (tm-...-abuse-3).** `/slo-pattern` ≤5 + cite-probe-IDs is what stops the agent inventing evidence-free patterns; the test asserts the rule's presence.

## Assumptions verified
- discover_skills() lists pattern + precision; both count tests re-pointed 44→46; M1+M2 tests still green.

## Rules for the next milestone
- M4 `/slo-spike` is the only code phase — mandate budget + delete-or-promote + scratch confinement + no-production-promotion; the test must assert these (the AI tolerance headline). Keep re-pointing both count tests (46→47).

## Invariants/assertions added
- `/slo-pattern` convergent + ≤5 cap + cite-probe-IDs + DICEE; `/slo-precision` measurement + accept threshold + kill threshold + reject "feels better"; both target §5/§6; output-path safety on both.
