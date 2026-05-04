---
name: slo-tla-methodology-counterexample
source_skill: skills/slo-tla/SKILL.md
description: Counterexample translation procedure and trace shape for /slo-tla.
---

# /slo-tla Methodology — Counterexample Translation

## Counterexample Translation — This Is The Product

Raw TLC output is a sequence of states with state variables. That is not a
design finding. Translate:

1. Read the trace step by step. Name each actor's action ("A sends REQUEST", "B
   crashes", "A retries").
2. Identify the fork: the state in which the invariant first fails.
3. Write it as a short narrative: "Actor A sends REQUEST. Before B
   acknowledges, B crashes. A retries. B comes back up and processes twice."
4. Name the design assumption that broke: "We assumed at-most-once delivery, but
   the retry introduces at-least-once."
5. Propose the fix in the DESIGN, not the spec: "Add an idempotency key",
   "Require B to persist state before ack", etc.
6. Re-verify after the design fix lands in the spec.

The trace markdown file is shaped like:

```markdown
# Trace — <property name> violation

## Property
<crisp sentence>

## Counterexample
1. <Actor A: action>
2. <Actor B: action>
...
N. <the step at which the property fails>

## Fork point
<state N-1 → N: what changed, why it matters>

## Broken design assumption
<one sentence>

## Proposed fix
<one paragraph, design-level, not spec-level>

## Status
- [ ] design fix applied to spec
- [ ] TLC re-run, green at bound (N=…, M=…, K=…)
```

## Anti-Patterns

- Translating counterexamples mechanically instead of narratively — "state 5:
  pc[A]=inCS, pc[B]=inCS" is not a finding; "A and B both got in because the
  lock check and acquire are not atomic" is.
- Throwing a counterexample back at `/slo-architect` without a fix proposal. The
  trace is the raw material; the design fix is the product. Spec the fix in the
  Proposed-fix section before re-running TLC.
