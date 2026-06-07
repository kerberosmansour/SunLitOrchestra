# Stack Decision — innovation-loop

## Chosen stack

- **8 host-neutral Markdown `SKILL.md` files** under `skills/slo-experiment/`,
  `skills/slo-sandbox/`, `skills/slo-play/`, `skills/slo-pattern/`,
  `skills/slo-precision/`, `skills/slo-spike/`, `skills/slo-curate/`,
  `skills/slo-demo/` — discovered by `discover_skills()` in
  `crates/sldo-install/src/install.rs` exactly like every existing `/slo-*`
  skill (the discovery gate is "directory contains `SKILL.md`").
- **One new Markdown template**:
  `docs/slo/templates/experiment-book-template_v_1.md` — the Experiment Book
  v1 / Creative Experiment Contract; the experimentation peer of
  `runbook-template_v_4_template.md`. Emitted by `/slo-experiment`.
- **One Rust structural-contract test** appended under
  `xtasks/sast-verify/tests/` (no new crate, no new binary) — asserts the 8 new
  skills parse valid YAML frontmatter, declare `name` + `description`, and that
  any declared output paths are traversal-safe and within the
  `docs/slo/experiments/` (+ `experiments/` scratch) allow-list. Same idiom as
  `sap_imp_m5_agents.rs`.
- **Documentation merges**: `docs/skill-pack-catalog.md` (new
  "Innovation-Sandbox flow" section), `docs/LOOPS-ENGINEERING.md` (new
  "Innovation Sandbox loop" section + a "Start here" row), `CLAUDE.md` /
  `copilot-instructions.md` / `AGENTS.md` overlay pointers, `SECURITY.md`
  merge, `docs/ARCHITECTURE.md` merge.

## Reason

This is a brownfield addition to a Markdown-skill-pack repo whose canonical
portable unit is `skills/<name>/SKILL.md` (per the skill-pack catalog: "the
portable unit is the Markdown `SKILL.md` contract"). The research-equivalent
dossiers converged on a *contract-driven Markdown artifact* (the Experiment
Book) as the right structure — light enough to preserve creative play, strict
enough to be repeatable, reviewable, and safe. Markdown skills satisfy every
constraint: host-neutral (Claude Code + Copilot + Codex consume the same
`SKILL.md`), offline, no service surface, and they inherit the existing install,
catalog, loops, and structural-test machinery for free. No runtime, database, or
service is needed because **the artifact is the product** — the same reason the
biz pack and the SAST rule-gen pack are Markdown-plus-a-test, not new services.

## Rejected alternatives

- **A new Rust crate / CLI (`sldo-experiment`)** — rejected: there is no
  deterministic computation to own. The legacy `sldo-plan` / `sldo-run` CLIs
  were *removed* in the 2026-04 cleanup precisely because "the skills are the
  canonical interface now." Adding a binary would re-introduce the dead surface.
- **A hosted SaaS experiment tracker (web UI + shared DB)** — rejected: adds an
  auth/storage/DoS service surface for zero added value at this stage, and
  breaks the host-neutral, offline, Markdown-first nature of the pack.
- **Folding the loop into `/slo-ideate` (no new skills)** — rejected: collapses
  the pre-idea exploratory phase into YC-style feature interrogation, killing
  the divergent play the loop exists to protect. The loop must *feed*
  `/slo-ideate`, not be it.

## Non-negotiables (downstream cannot change these without migration)

- The Experiment Book lives at `docs/slo/experiments/<slug>/EXPERIMENT.md` and
  follows `docs/slo/templates/experiment-book-template_v_1.md`. (Parallels the
  v4 runbook invariant.)
- Every skill is `skills/slo-<name>/SKILL.md` with at minimum `name` +
  `description` frontmatter; output paths are constrained to
  `docs/slo/experiments/` (durable artifacts) and `experiments/<slug>/`
  (scratch code) — no writes outside that allow-list, no `..` traversal, no
  absolute paths. Enforced by the structural-contract test.
- The 8 honest exit states are a **frozen vocabulary** (see interfaces doc).
  Adding/removing one is a migration, not an edit.
- **Hard rule**: nothing in this loop promotes to production without entering
  the normal SLO Sprint or Ticket loop. Promotion is a *handoff to another
  skill*, never an in-loop code merge.
- No new Rust crate; no new binary; no service surface introduced.
