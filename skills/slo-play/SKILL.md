---
name: slo-play
description: >
  Use this skill after /slo-sandbox has filled §3, to fill §4 Play Log — generate
  many raw probes from the sandbox's probe seeds WITHOUT premature judgment. Mode
  is divergent: the goal is to map possibilities and surface surprises and dead
  ends, not to pick a winner. This is the joy-preserving phase. Do NOT rank,
  optimize, or turn a probe into a product plan — that is /slo-pattern's and
  /slo-curate's job. Hands off to /slo-pattern.
---

# /slo-play — generate raw probes, defer judgment

You are playing, not planning. The sandbox is set; your job is to run lots of
small probes and write down what happened — especially the surprises and the
dead ends — without deciding which is "best" yet. Mode: **divergent**.

## The one rule (the joy guard)

During this phase you **judge safety only**; defer quality judgment. No ranking,
no winner-picking, no "this probe is the one", no turning a probe into a product
plan. A dead end is a valid, valuable output. The ONLY judgment allowed here is
the safety check. (Convergence is `/slo-pattern` and `/slo-curate`'s job — not
yours.)

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §3 Sandbox Charter + probe seeds.
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§4 body).

## Output

Fill **§4 Play Log** of `docs/slo/experiments/<slug>/EXPERIMENT.md`. Produces the
`ProbeLedger` (probe cards) + `DeadEndList` + `StrangeButInterestingList`.

## Method — fill §4, opening with the Phase Contract (Mode: divergent)

1. For each probe seed (and new ones that emerge), run a probe and write a row in
   the **Probe Board**: id, probe, type, setup, observation, surprise, reusable?,
   safety note. Generate MORE probes than you think you need.
2. Tag each probe with a frozen **type**: `mechanism_probe | interaction_probe |
   failure_probe | security_probe | data_probe | latency_probe | magic_probe |
   composition_probe` (composition = combine two unrelated tricks).
3. **Raw observations**: bullets, unfiltered.
4. **Strange but interesting**: things not useful yet that might matter later.
5. **Dead ends**: what failed, what it taught, any reusable fragment. Capture
   these deliberately — a good play session produces dead ends.
6. **Candidate patterns**: a raw list for `/slo-pattern` (names only — do NOT
   evaluate or rank them here).
7. Update the §1 tracker (phase 2 `complete`), append a Safety Check, hand off.

## Gate

The output is not "we found the good idea". The output is "we now have enough
material — surprises, dead ends, candidate patterns — to name patterns". If you
catch yourself ranking, stop: that is the next phase.

## Handoff

Suggest **`/slo-pattern <slug>`** — name the reusable tricks (convergent).

## Anti-patterns

- Behaving like `/slo-plan` — optimizing, reducing, criticizing quality too early.
- Discarding a dead end (it is evidence, not waste).
- Collapsing a probe into a feature plan.
- Stopping after three probes — diverge widely; the vocabulary comes from volume.

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
