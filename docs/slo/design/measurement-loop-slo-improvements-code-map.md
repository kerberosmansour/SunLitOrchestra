# Code Map — measurement-loop-slo-improvements (brownfield)

## Four-object summary

1. **`skills/<name>/SKILL.md` contracts** — the portable unit of behaviour. Each skill is a Markdown contract
   read by the agent at invocation. This work edits five: `slo-ideate`, `slo-product`, `slo-plan`,
   `slo-verify`, `slo-retro`. They reference shared `references/` files for detailed methodology.
2. **`docs/slo/templates/runbook-template_v_4_template.md`** — the output contract of `/slo-plan`. The
   Measurement Contract section is inserted here as an additive, optional section (+ a Contract Block row).
   v3 template remains a historical artifact; do not edit v3.
3. **`references/biz/artifact-schema.md`** — the stable frontmatter contract for biz artifacts (the only place
   the one new `feature_measurement_spec` key is registered). Its own rules say adding keys is a
   `/slo-architect` decision and that the `confidential|public` tier enum must not be extended unilaterally.
4. **`docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md`** — the loop catalog. The new feature-performance
   loop is documented here in the established loop-entry format; structural-test gate in `xtasks/sast-verify`
   enforces SKILL.md shape/SHA baselines for edited skills.

## Exemplar code to copy

- **Prior cross-skill meta-change design set:** `docs/slo/design/fowler-ai-architecture-slo-improvements-*`
  (overview frontmatter keys, interfaces-doc three-table shape, additive-only enrichment discipline). This is
  the closest precedent to the present work — copy its structure.
- **Loop-entry shape:** any existing entry in `docs/LOOPS-ENGINEERING.md` (Sprint loop, Lessons loop) — copy
  the User-visible outcome / Trigger / Steps / Exit condition / Artifacts / Skills involved / ASCII-diagram
  format exactly.
- **Optional-section precedent in v4:** §10 "Carry-forward from prior retros" in the v4 template — copy its
  "this section is optional; legacy runbooks without it remain valid; the driver falls back" framing for the
  new Measurement Contract section.
- **Additive frontmatter key precedent:** the `pii_scan_override` / `tier_override_reason` rows in
  `references/biz/artifact-schema.md` — copy their "optional, paired, read by /slo-verify" style for
  `feature_measurement_spec`.
- **Heuristic verify pass precedent:** the `/slo-verify` Pass 4 biz-pack PII-pattern scan over
  `docs/biz-public/` — copy its presence/pattern (non-schema-parsing) approach for the new measurement pass.
- **`~~~text` fence rule:** any generated `SECURITY.md` / threat-model in `docs/slo/design/` — copy the
  user-string fencing discipline when any new generated artifact interpolates author-provided strings.

## Anti-exemplar code not to copy

- **`docs/slo/templates/runbook-template_v_3_template.md`** — historical; do not edit or extend it. New
  measurement work lands only in v4.
- **`crates/sldo-tauri/` (parked)** — per CLAUDE.md and root SECURITY.md, parked since 2026-04; not in scope.
- **The removed legacy `sldo-plan` / `sldo-run` CLIs** — do not reintroduce binary-driven planning; the
  skills are the canonical interface now.
- **Any pattern that adds a *cluster* of new frontmatter keys** — the schema rule and reversibility analysis
  authorize exactly one (`feature_measurement_spec`). Resist scope creep into a telemetry-schema-in-frontmatter.

## Dangerous seams (inspect before editing)

- **`xtasks/sast-verify/tests/`** — structural-contract tests pin SKILL.md frontmatter, output-path safety,
  and (for `/slo-critique`) a byte-identical SHA baseline. Editing any SKILL.md will trip the gate; the
  baseline update must happen in the *same* milestone as the edit. Inspect `sap_imp_m5_agents.rs` and any
  per-skill SHA fixtures before editing.
- **`references/biz/artifact-schema.md` enum rows** — the `tier` enum is `confidential|public` only; do not
  extend it. The new key is a `bool`, not a tier value.
- **`/slo-product` vs `/slo-metrics` boundary** — the spec text in both SKILL.md files cross-references the
  other to enforce the split. Edits to `/slo-product metrics` must not absorb financial KPIs (CAC/LTV/NDR/burn)
  — those stay in `/slo-metrics`. The biz-skill-pack-interfaces doc marks both as stable.
- **`docs/slo/design/biz-skill-pack-interfaces.md`** — declares `/slo-product` and `/slo-metrics` command
  surfaces + output paths as `stable`. Any change here is a signal you've broken a stable interface; this work
  should leave it unchanged except possibly to *note* the additive feature-spec section.
- **Loop-doc cross-links** — `docs/LOOPS-ENGINEERING.md` and `docs/LOOPS-BUSINESS.md` are referenced from the
  v4 template's prerequisite-reading list and CLAUDE.md; keep anchors stable when adding the new loop.
