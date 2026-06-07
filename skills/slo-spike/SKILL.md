---
name: slo-spike
description: >
  Use this skill after /slo-precision has filled §6, to fill §7 Spike Cards — run
  one or more BOUNDED proof artifacts that answer a falsifiable learning question
  with evidence. Mode is evidence. This is the ONLY phase that may run code, and
  even here it is scratch-only under experiments/<slug>/<spike-id>/ under a
  declared data/network/dependency/resource budget. A spike is done when the
  learning question is answered, not when the prototype is polished. Every spike
  ends with a delete-or-promote decision. NOTHING here becomes production without
  re-entering the normal SLO Sprint or Ticket loop. Hands off to /slo-curate.
---

# /slo-spike — build bounded proof artifacts

You are answering ONE learning question with evidence, cheaply and safely. This
is the only phase where code may run — and it is scratch-only, bounded, and never
promoted to production from here. Mode: **evidence** (judge evidence).

## Hard rules (load-bearing)

- **Scratch only, under `experiments/<slug>/<spike-id>/`** — the single canonical,
  git-ignored scratch root. Never write production files.
- **No production promotion from this phase.** A spike NEVER becomes a real
  package/dependency here; promotion happens only by re-entering the SLO Sprint or
  Ticket loop (plan → critique → execute → verify) via `/slo-curate` + `/slo-demo`.
- **Mandatory resource budget.** Every spike declares CPU / memory / time /
  network + behavior-at-limit, and refuses to run with no budget. If the run
  exceeds the budget, record actual-vs-declared and STOP (no silent continue).
- **Synthetic / redacted data by default.** No production data, no real secrets,
  no uncontrolled external calls.
- **The verdict derives from the recorded evidence log, NOT from narration.**
  Never fabricate an unobserved result; never mark `promote_*` without an
  evidence row.

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §6 Precision Model (the falsifiable
  claim + accept/kill thresholds + security invariants).
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§7 body).

## Output

Fill **§7 Spike Cards and Evidence** of the Experiment Book (one card per spike).
Optionally scratch code under `experiments/<slug>/<spike-id>/`. Produces the
`SpikeCard` + `EvidenceLog`.

## Method — fill §7, one Spike Card at a time (Mode: evidence)

1. **Phase Contract / Spike Card header**: learning question; scratch path
   `experiments/<slug>/<spike-id>/`; production files allowed = none by default;
   data = synthetic/redacted/generated; external calls = none/listed; dependency
   policy; **resource budget** (CPU/memory/time/network) + cleanup rule.
2. **Setup** + **Method**: what was built/simulated and how the claim was tested.
3. **Commands / Evidence** table: step, command/action, expected, actual, notes.
   This recorded **evidence log** is the source of the verdict.
4. **Results**: declared budget vs. actual recorded.
5. **Surprise**: what we did not expect.
6. **Safety Result** table: each security invariant from §6, result, evidence.
7. **Decision hint**: one of `promote_to_idea | promote_to_ticket |
   promote_to_research | needs_more_play | killed_but_reusable | archive_no_action`
   — **derived from the evidence**, plus the **delete-or-promote** decision for the
   scratch.
8. Update the §1 tracker (phase 5 `complete`), append a Safety Check, hand off.

## Evidence standard (tiered)

- Scratch-only no-code/prototype proof: evidence log + safety rails + cleanup.
- Code that may later be promoted: formatter/typecheck/tests before the promotion
  packet.
- Production code: **not allowed in this loop.**

## Gate

A spike is complete when its learning question is answered (or explicitly
blocked), the evidence is recorded, accept/kill thresholds were evaluated,
resource + safety bounds were checked, the scratch path is declared, no production
files were changed, and the card has a decision hint.

## Handoff

Suggest **`/slo-curate <slug>`** — decide promote / continue / kill / archive.

## Anti-patterns

- Polishing the prototype instead of answering the question.
- Running an unbounded spike, or pulling real data "just to see".
- Writing scratch outside `experiments/<slug>/`, or promoting it to production.
- Marking `promote_*` with an empty evidence log (fabricated green).

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
