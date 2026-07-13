---
name: innovation-loop
created: 2026-06-07
status: design-locked
tla_required: false
kani_required: false
security_libs_required: false
ai_component: true
compliance: [soc2, asvs]
---

# Design overview — Innovation Sandbox loop (Experiment Book v1)

Single source of truth for downstream skills (`/slo-tla` reads `tla_required`,
`/slo-kani` reads `kani_required`, `/slo-plan` reads scope, `/slo-critique`
reads `compliance` and `ai_component`, `/slo-verify` cites the threat-model row
IDs).

## What this is

A new **host-neutral** skill pack of **8 Markdown skills** that adds a
**discovery lane** *before* the existing Sprint loop. The Sprint loop turns a
*decision* into shippable work; this loop turns a *fuzzy technical hunch* into
either a promotable idea/ticket/research/runbook candidate **or** a documented
dead-end with reusable lessons — without breaking the creative, exploratory
nature of experimentation.

The heart of the loop is one durable, contract-driven artifact — the
**Experiment Book** (`docs/slo/experiments/<slug>/EXPERIMENT.md`) — that is to
experimentation what the v4 runbook is to delivery: lighter, but still
artifact-driven, gated, safety-aware, and explicit about phase-by-phase
handoffs. Each phase skill fills one section of the Experiment Book behind a
**light "Experiment Phase Contract"** (not the full v4 production Contract
Block), and hands a named output object to the next phase.

The 8 skills:

| # | Skill | Role | Fills |
|---|---|---|---|
| 0 | `/slo-experiment` | umbrella — open/resume the Experiment Book | §0–§2 + tracker |
| 1 | `/slo-sandbox` | choose the material before the feature | §3 Sandbox Charter |
| 2 | `/slo-play` | generate raw probes, defer judgment (divergent) | §4 Play Log |
| 3 | `/slo-pattern` | name reusable tricks + next-curve + DICEE | §5 Pattern Catalog |
| 4 | `/slo-precision` | make variables measurable and freeze the confirmatory protocol | §6 Precision Model + Protocol Freeze |
| 5 | `/slo-spike` | separate bounded discovery from held-out validation (the only code phase) | §7 Discovery/Validation Records |
| 6 | `/slo-curate` | calibrate confidence, ablations/failures, and one disposition each | §8 Curation Decision |
| 7 | `/slo-demo` | package method, limitations, confidence, and typed handoff | §9 Demo + §10 RecommendationPacket |

The eight-skill wedge is shipped. The additive experiment-rigor update preserves
the creative front half and strengthens the confirmatory back half without
changing the Experiment Book v1 path, §0–§11 order, or route vocabulary.

## Frontmatter rationale

- `tla_required: false` — the loop is offline, single-process, interactive
  Markdown authoring. No concurrent shared state, no consensus, no leases, no
  cross-process ordering guarantees. Nothing to model-check at the design level.
- `kani_required: false` — **no Rust kernel is produced.** The deliverables are
  Markdown `SKILL.md` files + one Markdown template. The only Rust touched is a
  *structural-contract test* in `xtasks/sast-verify/tests/` that asserts the new
  skills' frontmatter + output-path safety (a test, not a verifiable kernel).
  `/slo-spike` may run *scratch* code in any language during an experiment, but
  that is the user's transient experiment, not a shipped bounded kernel.
- `security_libs_required: false` — no service surface, no auth, no
  crypto-confidentiality requirement. Local skill pack + Markdown artifacts only.
  SunLitSecurityLibraries / Hulumi components do not apply.
- `ai_component: true` — **the key flag.** Every phase skill drives an LLM agent
  (Claude / Codex / Copilot) to *generate probes, name patterns, propose
  measurement handles, and author spike code/evidence*. The agent can
  hallucinate a vacuous probe, overstate a "surprise", fabricate evidence, or be
  steered by a crafted hunch string. MITRE ATLAS + OWASP LLM Top 10 + NIST AI
  RMF triad applies. The threat model names the specific surfaces (evidence
  fabrication, scope/`promote_*` overclaim, prompt-injection via hunch/sandbox
  strings, scratch-code-as-exfil). `/slo-plan` and `/slo-verify` therefore
  require an **AI tolerance contract** for the nondeterministic phases.
- `compliance: [soc2, asvs]` — defaults. The skill pack itself processes no
  personal data; the realistic PII/secret exposure is content a user *pastes
  into* an Experiment Book, governed by the data-classification field + the
  existing `/slo-verify` Pass-4 PII scan, not by a new compliance framework.

## Why a separate loop (not just `/slo-ideate`)

`/slo-ideate` is YC-style interrogation that presumes a roughly-formed feature
(it forces: whose pain, smallest value slice, three approaches, worst day,
success thesis). Creative engineering often starts *earlier* — with a rich
material and no feature yet. Forcing that into `/slo-ideate` collapses the
exploratory phase prematurely. So the Innovation Sandbox loop **feeds**
`/slo-ideate`; it does not replace it. The promotion bridge (`promote_to_idea`)
is the seam between the two loops.

## Definition of Learned (the inversion)

A production runbook closes on **Definition of Done**. The Experiment Book
closes on **Definition of Learned**: *what did we learn, what surprised us, what
is reusable, and what deserves promotion?* Every experiment ends in exactly one
honest exit state:

`promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook
| needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`

`unknown` is treated as `blocked_by_unknown`, never silently terminal — the same
honest-exit-state discipline the Secure Value Loop overlay already enforces.

## Inputs

- `docs/slo/idea/innovation-loop.md` — condensed idea doc (pain, capabilities,
  Top risks, three approaches, recommendation, success thesis). Stands in for
  `/slo-ideate`; itself condensed from two founder-authored dossiers (in-session,
  not repo-tracked) covering the OK-Go creative-process translation and the
  Experiment-Book-v1 convergence.
- This repo's HEAD state — `skills/slo-ideate/` and `skills/slo-plan/`
  (style + interactive-authoring exemplars), `skills/slo-architect/`,
  `skills/slo-execute/`, `skills/slo-verify/`, `skills/slo-retro/`,
  `docs/slo/templates/runbook-template_v_4_template.md` (the artifact this
  template is the experimentation peer of),
  `crates/sldo-install/src/install.rs` (`discover_skills`),
  `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (structural-contract test
  exemplar), `docs/LOOPS-ENGINEERING.md`, `docs/skill-pack-catalog.md`,
  `SECURITY.md`, `docs/ARCHITECTURE.md`.

## Outputs (this skill, this run)

- [docs/ARCHITECTURE.md](../../ARCHITECTURE.md) — shipped Innovation Sandbox loop
  (Experiment Book v1) component view.
- [docs/slo/design/innovation-loop-stack-decision.md](innovation-loop-stack-decision.md)
- [docs/slo/design/innovation-loop-interfaces.md](innovation-loop-interfaces.md)
- [docs/slo/design/innovation-loop-experiment-book-spec.md](innovation-loop-experiment-book-spec.md) — the authoritative Creative-Experiment-Contract-v1 template spec (Definition of Learned, Judgment Timing Rule / phase moods, per-phase output objects, Safety Rails, promotion-seed tables) that M1 implements.
- [SECURITY.md](../../SECURITY.md) — merged "Innovation Sandbox loop —
  additional rules" section (existing sections preserved verbatim).
- [docs/slo/design/innovation-loop-threat-model.md](innovation-loop-threat-model.md)
- [docs/slo/design/innovation-loop-threat-model.slo.json](innovation-loop-threat-model.slo.json)
- [docs/slo/design/innovation-loop-reversibility.md](innovation-loop-reversibility.md)
- [docs/slo/design/innovation-loop-code-map.md](innovation-loop-code-map.md)
- This file.

## Handoff

The initial architecture/runbook is complete (no TLA — `tla_required: false`; no
Kani — `kani_required: false`). Future changes consume the stable interfaces and
use the normal SLO plan/critique/execute/verify gates.
