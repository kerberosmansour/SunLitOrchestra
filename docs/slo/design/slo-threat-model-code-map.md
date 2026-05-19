# Code Map — slo-threat-model (brownfield)

The target is the existing SunLitOrchestra skill-pack repo. This wedge touches
the skill-pack + structural-test surfaces, not the Rust product crates.

## Four-object summary

1. **Schema reference doc** (`references/security/threat-model-schema.md`,
   new) — the SLO-owned JSON schema. Sibling-in-spirit to
   `references/sast/manifest-schema.md`: a strict-parsed, provenance-carrying
   schema consumed by deterministic verification.
2. **The `.slo.json` fixture** (`docs/slo/design/
   slo-security-embedding-threat-model.slo.json`, new) — one hand-authored
   serialization of an existing dogfood Markdown threat model; the proof the
   schema can carry a real model without losing content.
3. **Consumer skills** (`skills/slo-critique/SKILL.md`,
   `skills/slo-verify/SKILL.md`, edited) — gain read-side halt-not-re-derive
   contract language.
4. **Structural-contract test** (`xtasks/sast-verify/tests/<prefix>_m<N>_
   threat_model_contract.rs`, new) — the deterministic guard binding the
   schema, frozen-ID invariant, `classification` presence, and SKILL.md
   read-side language.

## Exemplar code to copy

- **`references/sast/manifest-schema.md`** — copy this shape for the schema
  doc: required-structure block, per-field rules, *forbidden fields* section
  (e.g. "examples live in the paired fixture, not in the schema doc"), strict
  `deny_unknown_fields` parse, provenance via `sldo-<skill>-version: <git-sha
  of SKILL.md>`. The threat-model schema doc should read like a sibling of it.
- **`xtasks/sast-verify/tests/sap_imp_m1_citations.rs`** — copy this for the
  structural-contract test: `pulldown-cmark` AST walking of shipped Markdown,
  asserting cited paths resolve at HEAD. Use it for "both consumer SKILL.md
  files contain the read-side halt language" and "schema doc has required
  sections".
- **`xtasks/sast-verify/tests/sap_imp_m5_agents.rs`** — copy this for the
  byte-identical-baseline discipline and JSON-path-safety assertions; the
  closest precedent for asserting an invariant over a JSON/structured artifact.
- **`skills/slo-architect/references/threat-model-template.md`** — the
  structure the JSON must serialize 1:1 (STRIDE four-state cells, abuse-case
  table with `tm-<slug>-abuse-N`, residual-risk table, provenance). Do not
  invent fields it does not have.

## Anti-exemplar code not to copy

- **Removed `sldo-plan` / `sldo-run` CLIs** — the binary-heavy, compiled-flow
  pattern the 2026-04 cleanup deleted. This wedge stays Markdown-first + one
  test; do not introduce a crate.
- **Free-Markdown interpolation of user strings** — the explicit anti-pattern
  in `threat-model-template.md`. JSON string fields are literal data; never
  splice them into a prompt or unfenced Markdown.
- **Inline `metadata.examples`** (forbidden in the SAST manifest schema) —
  keep examples in the paired `.slo.json` fixture, never in the schema doc.

## Dangerous seams (inspect before editing)

- **⚠ `skills/slo-critique/SKILL.md` is governed by invariant F-ENG-6.** Its
  SHA-256 is a pinned constant at `xtasks/sast-verify/tests/
  sap_imp_m5_agents.rs:19` (captured 2026-05-07). The test failure message is
  explicit: the canonical portable critique path **must not be modified except
  via a documented runbook amendment that updates the constant** (per
  F-ENG-6). Editing it to add read-side language is therefore *permitted but
  governed*: the milestone that touches `slo-critique/SKILL.md` MUST, in the
  same change, (a) recompute and update the SHA-256 constant in
  `sap_imp_m5_agents.rs`, and (b) record an explicit F-ENG-6 runbook amendment
  noting the canonical critique path was deliberately extended with the
  threat-model read-side contract. The allow-list for that milestone MUST
  include `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`. This is the single
  highest-risk seam in the wedge — mishandling it silently breaks a
  cross-host governance invariant.
- **`xtasks/sast-verify/tests/` naming contract** — one test file per
  milestone, `<prefix>_m<N>_<feature>.rs`. Do not fold assertions into an
  existing `sap_imp_*` file.
- **`docs/ARCHITECTURE.md` is reality-first** — do not add this planned work
  to it; the structural tests and repo invariants treat ARCHITECTURE.md as
  HEAD-only.
- **Repo-root `SECURITY.md`** — repo-wide contract; this feature must not
  rewrite it (architect decision: skip). Touching it triggers `sap_imp_m1`
  expectations around the security-summary surface.
- **`/slo-execute` allow-list + freeze hook** — every new/edited path
  (`references/security/...`, `docs/slo/design/...slo.json`, the two SKILL.md
  files, the test, the `sap_imp_m5` baseline) must be on the milestone
  allow-list or the edit is blocked.
