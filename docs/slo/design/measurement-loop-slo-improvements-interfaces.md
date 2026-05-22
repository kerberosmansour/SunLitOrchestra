# Interfaces — measurement-loop-slo-improvements

Stability levels: `stable` (frozen — rename/reshape requires migration work), `evolving` (may change with
migration), `internal` (fair game). Mirrors the convention in
[biz-skill-pack-interfaces.md](biz-skill-pack-interfaces.md) and
[fowler-ai-architecture-slo-improvements-interfaces.md](fowler-ai-architecture-slo-improvements-interfaces.md).

## Stable interfaces this work MUST NOT break

| Interface | Stability | Description |
|---|---|---|
| `/slo-product` command verb + `mode_arg: roadmap \| metrics \| okrs` | `stable` | Output `docs/biz-public/product/<artifact>.md`. PM-side metrics only. Unchanged. |
| `/slo-metrics` command verb + `mode_arg: consumer \| b2b` | `stable` | Output `docs/biz-public/metrics.md`. Financial KPIs only. Unchanged. |
| `/slo-product` vs `/slo-metrics` responsibility split | `stable` | PM-side product KPIs vs financial/commercial KPIs — recorded critique decision; preserved. |
| Existing biz artifact frontmatter keys (`name`, `created`, `tier`, `archetype`, `skill`, `mode_arg`, …) | `stable` | `references/biz/artifact-schema.md`. Not renamed or removed. |
| Existing v4 template sections (§1–§20) | `stable` | `docs/slo/templates/runbook-template_v_4_template.md`. Not renumbered or removed; additions are insertions. |
| `/slo-ideate` six/seven forcing questions + idea-doc section names | `stable` | Existing sections (`## The pain`, `## Top risks`, approach blocks, `## Recommendation`, `## Open questions for /slo-research`) preserved. |
| `/slo-verify` Pass 1–5 contract (incl. Pass 4 PII scan) | `stable` | New measurement checks are additive, not a renumber of existing passes. |
| `/slo-retro` lessons + completion file paths and existing section names | `stable` | New section is additive to the lessons template. |
| `xtasks/sast-verify` structural-contract gate | `stable` | Every edited SKILL.md keeps the gate green; SHA baselines updated in the same milestone. |

## New additive interfaces introduced by this work

| Interface | Stability | Description |
|---|---|---|
| Idea-doc `## Success thesis` section | `evolving` | Leading metric, lagging metric, top guardrails, review window, "what evidence tells us the problem is technical vs pricing vs UX vs demand". Optional on legacy idea docs. |
| `/slo-ideate` Q3 reframing | `stable` (semantic) | "Smallest wedge" → "smallest *complete* value slice": complete enough to experience core value AND to learn why it worked/failed. Question slot count unchanged. |
| `/slo-product metrics` `## Feature measurement specification` section | `evolving` | Per-feature: north-star link, primary leading + lagging metric, guardrails, activation/completion funnel, adoption thresholds, diagnostic questions, segmentation, experiment backlog, telemetry requirements. |
| `feature_measurement_spec: bool` frontmatter key | `stable` (additive) | **The single authorized schema addition.** On `/slo-product metrics` artifacts. Default-absent = `false` (backward compatible). Read by `/slo-verify` measurement pass and future tooling to *detect* a feature spec without parsing prose. |
| v4 runbook **Measurement Contract** section | `evolving` | Runbook-level table: value hypothesis, review windows (24h/7d/28d), primary leading metric, primary lagging metric, guardrails, telemetry deliverables, rollout plan, diagnosis plan, experiment plan, privacy controls. Optional section (legacy runbooks remain valid — mirrors the v4 "Carry-forward from prior retros" optional-section precedent). |
| Contract Block row `Measurement deliverables` | `evolving` | Per-milestone: which named events / runtime metrics / saved queries this milestone ships, the guardrail owner, and the readout date. Optional row. |
| `/slo-verify` measurement pass (heuristic) | `evolving` | Event/schema presence smoke check; telemetry PII/masking check; failure-path emission check; replay-tagging check where enabled. Presence + pattern based (like Pass 4), **not** schema-parsing. |
| `/slo-retro` `## Results vs thesis` section | `evolving` | Did leading metrics move? Lagging? Implication for next milestone/runbook? |
| `docs/LOOPS-ENGINEERING.md` **Feature-performance loop** entry | `evolving` | New loop in the standard loop-entry format; cross-referenced from `docs/LOOPS-BUSINESS.md`. |

## Explicitly deferred interface (NOT created in v1)

| Deferred interface | Why deferred | Promotion trigger |
|---|---|---|
| `<slug>-measurement.slo.json` machine-readable telemetry/event schema companion | No real telemetry fixtures exist; premature freezing = stable-interface debt with no consumer. Mirrors deferral of the `/slo-verify` PII scan and the threat-model `.slo.json` companion until fixtures matured. | A future `/slo-architect` pass once real measurement contracts + emitted-event fixtures exist in a dogfooded runbook. |

## Compatibility commitments

- **Every addition is optional and additive.** Legacy idea docs without a success thesis, legacy runbooks
  without a Measurement Contract, and legacy `/slo-product metrics` artifacts without
  `feature_measurement_spec` all remain valid. `/slo-plan` *flags* a missing measurement contract for a
  value-bearing feature; it does not retroactively invalidate prior artifacts.
- **No command verb or output path changes.** Downstream tooling that reads `docs/biz-public/product/` or
  `docs/biz-public/metrics.md` is unaffected.
- **The one new frontmatter key is backward compatible** by absence-defaulting to `false`, and is registered
  in `references/biz/artifact-schema.md` per that file's own rule that adding keys is a `/slo-architect`
  decision (this doc is that decision).
- **Structural-test gate stays green.** Edited SKILL.md SHA baselines and any frontmatter-assertion fixtures
  in `xtasks/sast-verify` are updated in the same milestone as the edit, never waived.
