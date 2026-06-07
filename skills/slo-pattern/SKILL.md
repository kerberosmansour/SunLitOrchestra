---
name: slo-pattern
description: >
  Use this skill after /slo-play has filled §4, to fill §5 Pattern Catalog — turn
  raw play into named reusable mechanisms (a vocabulary of tricks). Mode is
  convergent. Cite probe IDs for every pattern, run the next-curve check and the
  DICEE check, and narrow to at most five serious candidates. Do NOT promote
  everything. Hands off to /slo-precision.
---

# /slo-pattern — name the reusable tricks

The play log is full of raw probes. Your job is to name the reusable mechanisms
hiding in them — the "vocabulary of tricks" — and narrow the field enough to
measure. Mode: **convergent** (judge reusability).

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §4 Play Log (probe ledger, dead-ends).
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§5 body).

## Output

Fill **§5 Pattern Catalog** of the Experiment Book. Produces `PatternCatalog`
(+ NextCurveCandidates, ProductPull, ArchitecturePull).

## Method — fill §5, opening with the Phase Contract (Mode: convergent)

1. **Pattern candidates** table: name, mechanism, **probe evidence (cite probe
   IDs)**, why surprising, reuse cases, risks. **Cite probe IDs for every
   pattern** — a pattern with no probe evidence is rejected.
2. **Cap: at most five serious candidates** (≤5). If play produced more, narrow.
3. **Next-Curve check**: for each pattern, is it a 10% improvement or a
   category change? (Kawasaki's "jump to the next curve".)
4. **DICEE check**: Deep / Intelligent / Complete / Empowering / Elegant.
5. **Sunlit strategic fit**: B2C / B2B / secure-data / cybersecurity.
6. **Product pull** + **Architecture pull**: which patterns suggest a user-facing
   wedge vs. a reusable platform capability.
7. Update the §1 tracker (phase 3 `complete`), append a Safety Check, hand off.

## Gate

Do not proceed with more than five serious candidates, and never with a pattern
that cites no probe ID. The output should narrow the field enough to measure.

## Handoff

Suggest **`/slo-precision <slug>`** — make the surviving claims measurable.

## Anti-patterns

- Promoting everything (the cap exists to force a choice).
- Naming a pattern with no probe evidence (cite probe IDs).
- Confusing a 10% improvement with a next-curve move — the next-curve check
  exists to keep that honest.

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
