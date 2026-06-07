---
name: innovation-loop
created: 2026-06-07
status: ideation
tla_required: false
---

# Innovation Sandbox loop — a creative-experimentation lane for SunLit Orchestra

> Source: this idea doc condenses two founder-authored dossiers (an OK-Go-inspired
> "creative engineering loop" proposal and a follow-up that converged on an
> "Experiment Book v1 / Creative Experiment Contract" as the experimentation-tuned
> peer of the v4 runbook). The dossiers stand in for `/slo-ideate` + `/slo-research`;
> they are in-session inputs, not repo-tracked.

## The pain

The SLO Sprint loop (idea → research → architect → plan → critique → execute →
verify → retro → ship) is excellent at turning a *decision* into reliable,
secure, shippable work. But it begins at `/slo-ideate`, which is YC-style
interrogation that presumes a roughly-formed product or feature. There is no
home for the **pre-idea** phase: the founder has a rich technical material (a new
API, an ML technique, a security primitive, a strange data source, a device
capability, a "what if?") but **not yet a feature**. Forcing that fuzzy material
into `/slo-ideate` too early kills the exploratory part — the OK Go lesson is
that ideas are often *found by playing inside a rich sandbox*, not designed from
a blank page. Today that play happens in chat and dies in chat: cool discoveries
are never captured as durable, promotable, reusable artifacts.

## Five capabilities the user described without realizing

- A durable per-experiment artifact (an "Experiment Book") that is to
  experimentation what the v4 runbook is to delivery — lighter, but still
  contract-driven, gated, safety-aware, and explicit about phase handoffs.
- A bounded "play" phase that defers judgment (divergent), distinct from the
  convergent discipline of `/slo-plan`.
- A way to make invisible variables measurable (latency, false-positive rate,
  friction, confidence) before declaring an idea magical or impossible.
- Honest exit states for every experiment, mirroring SLO's existing
  honest-exit-state discipline (no silent "done").
- A clean promotion bridge from a discovery into the normal secure delivery
  loop (`/slo-ideate`, `/slo-research`, `/slo-plan`, or the ticket flow) — and a
  compost lane for useful dead-ends.

## Top risks

- **Breach**: a spike or play-log captures a real secret, real PII, or
  production data, and that data is committed under `docs/slo/experiments/` or
  pasted into a demo artifact and pushed to a public remote. Surface: every
  skill that writes the Experiment Book, especially `/slo-spike` (the only phase
  that may run scratch code) and `/slo-demo` (evidence packs with screenshots).
- **Compliance fine**: N/A at the skill-pack level — SLO is an OSS skill pack
  with no data processing of its own. The exposure is downstream and is the same
  PII/secret-leakage class as the Breach row; the data-classification field on
  the Experiment Book plus the `/slo-verify` Pass-4 PII scan are the controls.
- **Prolonged outage**: N/A — the loop is offline, single-process, interactive
  Markdown authoring. There is no service to take down. The realistic
  "outage-shaped" failure is **prototype-becomes-production drift**: a scratch
  spike silently becomes a real dependency, bypassing plan/critique/execute. The
  hard rule "no production promotion without the normal SLO Sprint or Ticket
  loop" is the control.

## Approach A — conservative (chosen)

- **Effort**: ~1 runbook, ≤5 milestones.
- **Wedge**: ship the full 8-skill loop (`/slo-experiment` umbrella + 7 phase
  skills) as host-neutral Markdown `SKILL.md` files plus one new template
  (`experiment-book-template_v_1.md`), registered in the catalog and
  LOOPS-ENGINEERING.md, and passing the same structural-contract tests every
  other skill passes. No new Rust crate; no runtime; no service.
- **Risks**: scope sprawl across 8 skills; the contract becoming so heavy it
  kills the joy the loop exists to protect. Mitigated by the "lighter phase
  contract" design and a single durable artifact.

## Approach B — cloud / SaaS

- A hosted experiment tracker with a web UI and shared experiment database.
- **Rejected**: violates the host-neutral, offline, Markdown-first nature of the
  pack; adds a service surface (auth, storage, DoS) for zero added value at this
  stage. The artifact is the product.

## Approach C — minimal first cut

- Ship only `/slo-experiment` + `/slo-sandbox` + `/slo-play` + `/slo-curate`,
  defer precision/spike/pattern/demo to a follow-up runbook.
- **Considered and explicitly declined by the founder** in favour of the full
  8-skill loop (they want to run several experiments end-to-end immediately).

## Recommendation

Approach A. The smallest complete value slice is **one experiment that can run
end-to-end** — sandbox → play → pattern → precision → spike → curate → demo —
producing one Experiment Book with an honest exit state. That is complete enough
to dogfood the loop and to learn whether the phase contracts are too heavy or
too light.

## Success thesis

The loop succeeds when a founder can take a fuzzy hunch and, without leaving SLO
discipline, end at exactly one honest route decision with a durable, reviewable
Experiment Book — and at least one real experiment teaches something non-obvious.

- **Leading metric**: ≥1 Experiment Book reaches a terminal exit state
  (`promote_* | killed_but_reusable | archive_no_action`) within the first
  dogfood session.
- **Lagging metric**: a `promote_to_*` experiment actually enters the Sprint or
  Ticket loop and becomes shipped work.
- **Guardrails**: no secret/PII committed under `docs/slo/experiments/`; no
  scratch code promoted to production outside plan/critique/execute; the phase
  contracts do not become heavier than the v4 runbook.
- **Review window**: first 2–3 dogfood experiments.
- **If it misses**: distinguish *too-heavy contract* (founder abandons mid-loop —
  process friction) from *too-light contract* (experiments end vague, no honest
  decision — insufficient structure) from *wrong lane* (everything routes
  straight to `/slo-ideate`, meaning the loop adds no pre-idea value).

## Open questions for /slo-research

(Already answered by the founder's dossiers — recorded for completeness.)

1. What is the right artifact granularity — one `EXPERIMENT.md` vs. a folder of
   probe/spike files? → Single `EXPERIMENT.md`; split out files only when a
   real spike produces screenshots/metrics/code.
2. Where does the loop register — LOOPS-ENGINEERING.md or a skill reference file?
   → A new "Innovation Sandbox loop" section in LOOPS-ENGINEERING.md, because it
   produces a distinct user-visible outcome the Sprint loop does not.
