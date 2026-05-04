---
name: slo-tla-methodology-verified-design
source_skill: skills/slo-tla/SKILL.md
description: Verified-design document shape, refusal gates, and handoff rules.
---

# /slo-tla Methodology — Verified Design

## Verified-Design Doc Shape

```markdown
---
name: <slug>
verified_at: <YYYY-MM-DD>
tlc_bound: "N=3, M=5, K=2"
tool: "TLC 1.8.0"
---

# Verified Design — <title>

## System goal
<one paragraph>

## Abstract state
<variable list>

## Actions
<list>

## Safety properties checked
- <property> — PASS at <bound>

## Liveness properties checked (with fairness)
- <property> — PASS at <bound>, fairness: <weak|strong> on <actions>

## Simplifications from the real design
- <what was abstracted, why it still covers the real bug>

## Open questions
- <thing we did not model, and why it's acceptable>
```

## Gates — Refuse To Mark As Verified When

- Bound is not stated.
- Fairness is not declared for any liveness property.
- Any invariant was weakened silently (e.g., "no two in CS" → "no two in CS in
  the same step" — that's a different, weaker property).
- A counterexample was suppressed rather than addressed.
- The Naive / pre-fix variant passes silently — the spec is measuring nothing;
  either the race is not there or the model is too abstract.
- TLC at the minimum-bug-exhibiting bound takes over ~2 minutes — the model is
  too concrete; future readers will not iterate on it.
- "Simplifications from the real design" section is absent or hand-wavy. Each
  abstraction must name what was dropped and why it is sound to drop.

## Anti-Patterns

- Running TLC once, finding no violations, declaring victory — always iterate:
  add an action, re-run, grow the model. **Always run the Naive / pre-fix
  variant first** and confirm it fails; only then verify the Hardened variant
  passes.
- Using Apalache when TLC would work — Apalache is for state explosion, not
  default.
- Dropping the design simplifications paragraph — that's where future readers
  learn what the spec does NOT cover.
- **Adding variables to a spec without asking "does the race still exhibit
  without this?"** Every variable that is not load-bearing multiplies the state
  space.
- **Skipping the suitability gate.** Running TLA+ on a single-threaded CRUD
  function to "look rigorous" consumes a milestone and produces no information.
  Signal "not a good fit" instead.
- **Adding `-workers auto` before cutting the model down.** Parallelism hides a
  too-concrete spec behind hardware; the spec is still too concrete. Cut first,
  parallelise second.

## Handoff

After `-verified.md` is written and TLC is green at the declared bound, suggest
`/slo-plan` if a runbook does not yet exist or `/slo-critique` if it does so the
plan reviewers can read the verification.

If the suitability gate short-circuited ("TLA+ is not the right tool here"), the
handoff still works: suggest `/slo-plan` directly and recommend the alternative
verification approach (property-based tests, schema review, etc.) be included as
a milestone in the runbook.
