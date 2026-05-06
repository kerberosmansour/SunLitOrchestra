---
name: slo-tla-methodology-elicitation
source_skill: skills/slo-tla/SKILL.md
description: Elicitation questions and staged spec drafting for /slo-tla.
---

# /slo-tla Methodology — Elicitation And Spec Drafting

## Elicitation — The First Real Work

The design doc is almost never directly translatable to TLA+ — it over-specifies
(timestamps, UUIDs, payloads) and under-specifies (actions as prose, not
transitions). You reduce it.

Ask, in order:

### Q1. What Property Are We Trying To Prove?

Make the user name ONE safety property as a crisp sentence. If they name more
than one, force ranking — start with the most load-bearing one.

### Q2. What's The Smallest State That Can Violate It?

Example: mutual exclusion — two actors in critical section. No need to model
timers, payloads, or tokens unless they're the mechanism that prevents the
violation.

### Q3. Who Are The Actors, And How Many?

Force a bound. "An unbounded number of workers" → start with 3. If the property
holds at 3, think about whether it's a symmetry argument.

### Q4. What Are The Atomic Actions?

List them. Each action is a TLA+ next-state relation. Merge the ones that share
preconditions and effects. Usually 4–8 actions is the right range.

### Q5. What Fails, And How?

Network drops? Process crashes? Duplicate delivery? For each failure mode,
either model it as an action or explicitly exclude it from the bound.

### Q6. Liveness — What Must Eventually Happen?

Only after safety holds. Every liveness property needs a fairness assumption
(weak or strong, on which actions). Force this explicitly — liveness without
fairness is a source of silent bugs.

## Spec Drafting

Draft in stages. Do not try to write the whole spec in one go.

### Stage A — Variables And Init

Declare the state variables. Write Init. Run TLC with Next == FALSE (no
transitions) just to check Init is reachable. One state.

### Stage B — One Action At A Time

Add one action. Run TLC. Assert at least one invariant that's obviously true
(TypeOK). Grow the state graph.

### Stage C — The Invariant

Add the safety property from Q1 as an invariant. Run TLC. If it passes at a tiny
bound, increase the bound. If it fails, translate the counterexample and fix the
spec or the design before proceeding.

### Stage D — Liveness

Only after safety is solid. Add the temporal property and fairness. Run TLC with
`-deadlock` and the `PROPERTIES` line in `.cfg`.

### Stage E — Bound And Declare

Once TLC is green at your chosen bound, record the bound explicitly in the
verified-design doc. N actors, M requests, K failures — whatever parameters the
spec uses, state them.
