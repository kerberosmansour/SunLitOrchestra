# Stack Decision — measurement-loop-slo-improvements

## Detected stack (brownfield)

The repo is a **Rust 2021 workspace** (`sldo-common`, `sldo-install`, `sldo-research`, `xtasks/sast-verify`)
plus a **Markdown skill pack** consumed by Claude Code / Copilot / Codex. The canonical interface is the set
of `skills/<name>/SKILL.md` contracts and the `docs/slo/templates/` runbook templates; structural invariants
are enforced by Rust tests in `xtasks/sast-verify`. I keep this stack — there is no reason to change it.

## Chosen approach

**Enrichment-only, inline-first Markdown contract changes**, governed by the existing `/slo-architect →
/slo-plan → /slo-critique → /slo-execute → /slo-verify → /slo-retro` discipline, with:

- additive Markdown sections in five SKILL.md files, the v4 template, and the loop docs;
- exactly one additive optional frontmatter key (`feature_measurement_spec: bool`) in
  `references/biz/artifact-schema.md`;
- structural-test baseline updates in `xtasks/sast-verify` for every edited SKILL.md (SHA baselines /
  frontmatter assertions).

## Reason

The deep-research report's own conclusion is that the missing piece is a **loop discipline**, not a new tool
or service. The repo already treats `SKILL.md` contracts and the v4 template as the portable units of
behaviour, and the business-interfaces doc + artifact schema explicitly mark command verbs, output paths, and
frontmatter as **stable interfaces** — so the safe path is to *enrich existing outputs and templates without
breaking command surfaces*. The report also recommends a phased rollout (enrich skills/template first; add
verification + loop docs second; decide on schema surfaces third). This decision encodes exactly that
ordering and resolves phase three now (inline-first; defer the machine schema) so `/slo-plan` has an
unambiguous contract to build against.

## Rejected alternatives

- **Full machine-readable telemetry schema + `<slug>-measurement.slo.json` companion in v1** — rejected: no
  real telemetry fixtures exist yet, so it would create stable-interface debt with no consumer. SLO's own
  precedent is Markdown/heuristic-first: the `/slo-verify` PII scan was deferred until `/slo-talk-to-users`
  produced real PII-shaped fixtures, and the threat-model `.slo.json` companion was added only after the
  Markdown threat model matured. Promote to a schema in a later architect pass once fixtures exist.
- **Merging `/slo-product` and `/slo-metrics` into one metrics skill** — rejected: directly contradicts a
  recorded prior critique decision (PM-side product KPIs vs financial/commercial KPIs are intentionally
  separated). Merging would break two stable command surfaces for no benefit.
- **A new standalone `/slo-measure` skill** — rejected: the report is explicit that the loop should *sit
  across existing commands rather than replace them*. A new skill would fragment the contract and duplicate
  the ideate/plan/verify/retro touchpoints instead of threading measurement through them.
- **Adding a vendored analytics SDK (PostHog / OpenTelemetry / Clarity) to this repo** — rejected: these are
  cited as *architecture anchors* the generated measurement contract may name for a target product; SLO
  itself ships no telemetry runtime, so installing one here is out of scope and would add an unjustified
  dependency.

## Non-negotiables (downstream cannot change these without migration)

- `/slo-product` and `/slo-metrics` command verbs and output paths stay **stable** (no rename, no path move).
- The `/slo-product` (PM) vs `/slo-metrics` (financial) responsibility split is preserved.
- Existing v4 template sections and existing artifact-schema keys are **not renamed or removed**; the
  Measurement Contract section, the Contract Block row, and the `feature_measurement_spec` key are all
  **additive and optional** so prior idea docs / runbooks / artifacts remain valid.
- Every edited `SKILL.md` must keep its structural-test gate green (`xtasks/sast-verify`) — SHA baselines are
  updated deliberately in the same milestone as the edit, never bypassed.
