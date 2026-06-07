---
name: slo-precision
description: >
  Use this skill after /slo-pattern has filled §5, to fill §6 Precision Model —
  make the invisible measurable. Mode is measurement. Convert each promising
  pattern into a falsifiable claim with a measurement handle, an accept threshold
  AND a kill threshold, resource bounds, and security invariants. Do NOT accept
  "feels better" without a handle. This is the OK-Go "math" move for engineering.
  Hands off to /slo-spike.
---

# /slo-precision — make the invisible measurable

A pattern that "feels better" is not yet an experiment. Your job is to give every
surviving claim a measurable handle, an accept threshold AND a kill threshold,
resource bounds, and the security invariants that must never break — so
`/slo-spike` can test it with evidence. Mode: **measurement** (judge measurability).

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §5 Pattern Catalog.
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§6 body).

## Output

Fill **§6 Precision Model** of the Experiment Book. Produces `PrecisionModel`
(handles, accept/kill thresholds, resource bounds, security invariants).

## Method — fill §6, opening with the Phase Contract (Mode: measurement)

1. **Claims that need handles** table: claim, measurement handle, instrumentation,
   **accept threshold**, **kill threshold**. Every claim needs BOTH thresholds —
   accept (good enough to promote) and kill (bad enough to stop). Reject any claim
   that is "feels better" without a measurable handle.
2. **Invisible variables**: unit, expected range, hard bound, how measured.
3. **Reliability / compounding risk**: where small failures multiply.
4. **False positive / false negative plan**: required whenever classification,
   detection, retrieval, or ML judgment is involved.
5. **Resource budget**: expected bound, hard limit, behavior at limit.
6. **Security invariants**: what must never happen (no raw secret leaves device;
   no unredacted PII enters a demo artifact; etc.).
7. Update the §1 tracker (phase 4 `complete`), append a Safety Check, hand off.

## Gate

Every candidate that proceeds to spike has at least one **falsifiable** claim with
an accept threshold and a kill threshold. "Feels better" without a handle does not
proceed.

## Handoff

Suggest **`/slo-spike <slug>`** — build a bounded proof artifact for the claim.

## Anti-patterns

- Accepting "feels better" / "seems faster" without a measurement handle.
- Giving a claim an accept threshold but no kill threshold (you must be able to
  stop, not just to win).
- Skipping the false-positive / false-negative plan on a classification claim.

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
