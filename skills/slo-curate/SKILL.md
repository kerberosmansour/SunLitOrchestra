---
name: slo-curate
description: >
  Use this skill after /slo-spike has filled §7, to fill §8 Curation Decision —
  decide honestly. Mode is convergent. Give every candidate exactly one
  disposition (one of the frozen 8 route decisions), each citing a probe/spike;
  no vague maybes survive; dead ends route to §11 compost with a reusable lesson.
  This is the honesty gate of the loop. Hands off to /slo-demo for promoted
  candidates.
---

# /slo-curate — decide honestly

The experiment has play, patterns, measurements, and spike evidence. Your job is
to decide what is worth carrying forward — kill, pivot, combine, or promote — and
to refuse to leave anything as a vague "maybe". Mode: **convergent** (judge value,
risk, promotion).

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §3–§7 (all evidence).
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§8 body).

## Output

Fill **§8 Curation Decision** of the Experiment Book. Produces `CurationDecision`
+ `CompostEntries`.

## Method — fill §8, opening with the Phase Contract (Mode: convergent)

1. **Candidate board**: candidate, evidence, surprise, value, risk, decision.
2. **Decision rubric**: Meaning · User value · Surprise · Reliability · Security ·
   Strategic fit · Reuse · Evidence quality · Elegance.
3. **Final disposition**: give every candidate **exactly one disposition** from the
   frozen 8 — `promote_to_idea | promote_to_ticket | promote_to_research |
   promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable
   | archive_no_action` — and **cite the probe/spike** that justifies it. A
   candidate the founder cannot yet decide is `blocked_by_unknown`, never silently
   dropped or left as "maybe".
4. **Compost**: every killed candidate gets a reusable lesson or archive reason in
   §11.
5. Set the experiment-level exit state in §11; update the §1 tracker (phase 6
   `complete`); append a Safety Check; hand off promoted candidates.

## Curation Definition of Learned

Curation is complete only when: every candidate has exactly one disposition; every
promoted candidate has a next SLO route; every killed candidate has a reusable
lesson or archive reason; no vague "maybe" remains unowned.

## Gate

Every candidate receives **exactly one** disposition, each citing evidence. No
undisposed candidate; no vague maybe.

## Handoff

Suggest **`/slo-demo <slug>`** — package the promoted candidate(s) for handoff.

## Anti-patterns

- Keeping vague maybes alive forever (the whole point of this gate is to decide).
- A disposition with no cited probe/spike (fabricated promotion).
- Dropping a dead end without composting what it taught.

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
