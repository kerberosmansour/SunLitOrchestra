---
name: slo-demo
description: >
  Use this skill after /slo-curate has filled §8, to fill §9 Demo Pack + §10
  Handoff Contract — make the discovery communicable and emit the promotion
  packet. Mode is communication. Package the one-sentence magic + before/after +
  evidence + security posture, then fill the ONE promotion-seed table matching the
  disposition. Promotion is a SUGGESTION the human accepts — never an
  auto-invocation of the next skill. Closes the Innovation Sandbox loop.
---

# /slo-demo — make the discovery communicable

A discovery that lives only in chat dies in chat. Your job is to package the
promoted candidate so the next SLO skill — or a future agent — can pick it up
without relying on conversation memory, and to route it with a typed handoff.
Mode: **communication** (judge clarity).

## Inputs

- `docs/slo/experiments/<slug>/EXPERIMENT.md` — §8 Curation Decision (the promoted
  candidate + disposition).
- The binding spec `docs/slo/design/innovation-loop-experiment-book-spec.md` (§9/§10).

## Output

Fill **§9 Demo Pack** + **§10 Handoff Contract** of the Experiment Book. Produces
the `PromotionPacket`.

## Method — fill §9 then §10 (Mode: communication)

1. **§9 Demo Pack**: one-sentence magic · before · after · demo path · evidence
   table · **Security posture** (data exposure / secret handling / network calls /
   abuse / resource use) · productization route (choose exactly one).
2. **§10 Handoff Contract**: fill the ONE promotion-seed table that matches the
   disposition, so the receiving skill starts warm:
   - `promote_to_idea` → **Idea Seed** → next artifact `docs/slo/idea/<slug>.md` → `/slo-ideate`
   - `promote_to_ticket` → **Ticket Seed** → `docs/slo/tickets/ticket-<issue>-<slug>.md` → `/slo-ticket-plan`
   - `promote_to_research` → **Research Seed** → `docs/slo/research/<slug>/` → `/slo-research`
   - `promote_to_runbook` → **Runbook Seed** → `docs/RUNBOOK-<feature>.md` → `/slo-plan`
   - `killed_but_reusable` / `archive_no_action` → **Compost Entry** in §11.
3. Update the §1 tracker (phase 7 `complete`); set the experiment-level exit state
   in §11; append a Safety Check.

## Promotion is a suggestion (load-bearing — tm-innovation-loop-abuse-6)

You **suggest** the next skill and fill its seed table; you do **not** invoke it.
The human decides whether and when to run `/slo-ideate` / `/slo-ticket-plan` /
`/slo-research` / `/slo-plan`. Never auto-invoke a downstream skill, and never
promote to production — that only happens by re-entering the Sprint or Ticket loop.

## Gate

The demo can be handed to the next SLO skill without relying on chat memory, and
exactly one promotion-seed table (matching the disposition) is filled.

## Handoff

Suggest the matching next skill (above) as a **recommendation** for the human to
run, or close the experiment to §11 compost. The loop is complete when the
Experiment Book carries exactly one of the frozen 8 exit states in §11.

## Anti-patterns

- Auto-invoking `/slo-ideate` / `/slo-plan` / etc. (promotion is a suggestion).
- Filling a seed table that does not match the disposition.
- Promoting to production directly (must re-enter the Sprint/Ticket gates).

---

**Loops**: Innovation Sandbox loop — see [docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop](../../docs/LOOPS-ENGINEERING.md#innovation-sandbox-loop).
