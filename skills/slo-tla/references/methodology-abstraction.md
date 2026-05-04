---
name: slo-tla-methodology-abstraction
source_skill: skills/slo-tla/SKILL.md
description: Abstraction balance and state-explosion triage for /slo-tla.
---

# /slo-tla Methodology — Abstraction And State Explosion

## Abstraction Balance — The Central Tradeoff

TLA+ specs fail in two opposite directions, and the first draft usually lands
on one of them:

**Too concrete (state explosion):** the spec carries implementation detail that
does not affect the race — timestamps, unique sequence numbers, multiple
resources when the race is resource-local, authoring paths when the race is
triage-time, snapshot/commit splits when the race is visible at either point.
TLC takes minutes or runs out of heap. Symptom: you find yourself reducing
`MaxSeq` or `MaxResources` to keep TLC tractable. That is the spec telling you
to **abstract, not shrink**.

**Too abstract (trivially proved):** the spec collapses the variables that cause
the race into a single boolean, and the fix is correct by construction without
exploring any interleavings. TLC finishes in 5 states and reports no violation.
Symptom: the safety property holds even when you comment out the fix. The spec
is measuring nothing.

The sweet spot is **the smallest model that still exhibits the bug on the pre-fix design**. Procedure:

1. Write the minimal spec — single resource, booleans where possible, no history
   variables, no implementation-orthogonal paths.
2. Run TLC with the **naive / pre-fix** verdict logic. If TLC passes, the spec
   is too abstract — add a variable or an action until TLC finds the race.
3. Apply the design fix. If TLC still fails, the fix is wrong — iterate on the
   design, not the spec.
4. If TLC passes in milliseconds, you are done. If TLC passes in seconds to a
   minute, acceptable. If TLC takes over ~2 minutes at the
   minimum-bug-exhibiting bound, the model is still too concrete — go back to
   step 1.

**State-space budget rule of thumb:** the Naive/broken spec should fall over in
**under 1000 reachable states and under 10 seconds**. If it takes longer than
that to find the bug, future readers will struggle to iterate on the spec; the
abstraction is not paying rent.

## State-Explosion Triage

When TLC runs for more than ~2 minutes at your minimum bound, do not simply add
`-workers auto` and wait. Diagnose and cut:

1. **Drop liveness first.** Temporal properties amortise a tableau over the
   entire state graph — each pass scales roughly linearly with states generated.
   Run safety-only first to confirm the core race, then add liveness at the
   smallest bound that expresses it.
2. **Eliminate history variables.** If a variable exists only to express the
   safety predicate (e.g. `trueConsoleMut` recording ground truth), try
   replacing it with a boolean ghost flag (`sawMutation`) or a direct reference
   to an existing state variable.
3. **Collapse snapshot/commit into one action** if the fix does not depend on
   the read-to-commit gap. You can always re-split later if a second
   counterexample emerges.
4. **Cut from N resources to 1** if the race is resource-local. Argue symmetry
   in the verified-design doc rather than exhaustively verifying N > 1.
5. **Remove orthogonal paths.** If your spec models both an authoring path and a
   triage path but the safety property is triage-only, drop the authoring path.
6. **Replace `Seq(X)` with a single `head ∈ X ∪ {None}`** if the queue's length
   never actually matters. Sequence permutations explode state space.
7. **Power-set subsets become presence booleans.** `deliveredEvents ⊆ Event`
   with up to 6 events has 64 subsets; if the classifier only cares "any
   delivered console event exists," replace with `anyConsoleDelivered ∈ BOOLEAN`.

After every cut, re-run the Naive/broken spec and confirm it still fails. If a
cut silently passes the Naive spec, you cut something the race needed — restore
it.
