# SLO threat-model JSON schema (`<slug>-threat-model.slo.json`)

The SLO-owned, minimal machine-readable companion to
`docs/slo/design/<slug>-threat-model.md`. It serializes the **existing**
Markdown threat-model structure (STRIDE cells, `tm-<slug>-abuse-N` abuse-case
rows, residual-risk table, compliance mapping, provenance) so downstream
skills read a stable artifact instead of re-deriving it.

Read by `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` and, from runbook M2,
by `/slo-critique` and `/slo-verify`. Strict-parsed via `serde_json` with
`#[serde(deny_unknown_fields)]` — strict unknown-field rejection: an
undocumented key is a hard parse error, never a warning.
This doc is the sibling of [`../sast/manifest-schema.md`](../sast/manifest-schema.md);
it deliberately mirrors that file's shape and discipline.

This is **not** OTM. OTM 0.2.0 cannot natively express SLO's
accepted-residual distinction or compliance mapping (research synthesis), and
CycloneDX TM-BOM is unreleased; an OTM/TM-BOM **export** may be added later,
additively, but the SLO schema is the canonical store.

## Required structure

```jsonc
{
  "slo_schema_version": "0.1.0",      // semver; bumped only on a breaking field change
  "slug": "<feature-slug>",            // matches the docs/slo/design/<slug>-* family
  "sensitivity": "public",             // public | internal | confidential | restricted
  "otm_compatible": false,             // informational: is a lossless OTM export believed possible
  "provenance": {
    "produced_by": "slo-architect",    // producing skill name
    "producer_skill_sha": "<git sha of the producing SKILL.md at emit>",
    "generated_at": "2026-05-19",      // ISO-8601 date
    "inputs": [                          // input docs + their git blob shas (staleness detectable)
      { "path": "docs/slo/design/<slug>-threat-model.md", "sha": "<git blob sha>" }
    ]
  },
  "stride": [
    { "component": "...", "class": "Tampering",
      "state": "mitigated", "control_or_reason": "..." }
  ],
  "abuse_cases": [
    { "id": "tm-<slug>-abuse-1", "surface": "...", "attacker": "...",
      "attack_step": "...", "attacker_outcome": "...", "control": "...",
      "status": "active", "superseded_by": null, "supersede_reason": null,
      "classification": "internal" }
  ],
  "residual_risks": [
    { "risk": "...", "exploit_path": "...", "compensating_control": "...",
      "accepted_residual": true, "owner": "...", "review_by": "2026-08-19",
      "classification": "internal" }
  ],
  "compliance": [
    { "control": "...", "soc2": "CC7.1", "asvs": "V5.1" }
  ]
}
```

## Field rules

- **`slo_schema_version`** — semver string. The wedge ships `0.1.0`. A
  breaking field change bumps the major; export adapters are additive and do
  not bump it.
- **`slug`** — the feature slug; matches the `docs/slo/design/<slug>-*` family.
- **`sensitivity`** / **`*.classification`** — exactly one of
  `public | internal | confidential | restricted`. Drives the two-tier
  publication discipline: a `confidential`/`restricted` artifact must follow
  the gitignored-tier convention (the biz pack's `docs/biz/` vs
  `docs/biz-public/` precedent). The wedge mandates the field; it does not
  implement a redaction engine (accepted residual; see the design docs).
- **`provenance.producer_skill_sha`** — git SHA of the producing skill's
  `SKILL.md` at emit time. Same idiom as `manifest-schema.md`'s
  `sldo-<skill>-version`. **Not** a content hash.
- **`provenance.inputs[].sha`** — git blob SHA of each input doc, so a
  consumer can detect a model that has drifted from the docs that produced it.
  Non-empty.
- **`abuse_cases[].id`** — matches `^tm-<slug>-abuse-\d+$`. **Frozen**:
  assigned once, never renumbered. The numbers are contiguous from 1. A change
  to an abuse case **supersedes** it; it does not renumber or delete it.
- **`abuse_cases[].status`** — `active | superseded`. A `superseded` row MUST
  carry non-empty `superseded_by` (the replacing id) and `supersede_reason`.
  Superseded rows stay in the file — supersede-don't-renumber, no silent drop.
- **`residual_risks[].accepted_residual`** — `true` ⇒ a knowingly accepted
  risk, NOT a missing control. This is the field `/slo-critique` and
  `/slo-verify` use to distinguish accepted risk from missing coverage.
- **String fields** (`attacker`, `attack_step`, `risk`, ...) are **literal
  data**. A consumer renders them inside a literal/quoted block (the
  `~~~text` fence discipline of the Markdown template); they are never
  interpolated as instructions. (Runbook M2 / SEC-1.)

## Forbidden fields

- **Any undocumented top-level or nested key.** Parsing uses
  `deny_unknown_fields`; an unknown key is a hard parse error, not a warning.
  This is the anti-prompt-injection and anti-scope-creep boundary.
- **Inline examples.** Examples live in the paired `.slo.json` fixture
  (`docs/slo/design/<slug>-threat-model.slo.json`), never in this schema doc —
  same rule as the SAST manifest schema.
- **A content-hash provenance field.** Provenance is the producing SKILL.md
  git SHA + input git blob SHAs. Do not add a parallel content-hash idiom.
- **A new `AC-N` id scheme.** The frozen id convention is the pre-existing
  `tm-<slug>-abuse-N`. A parallel scheme is forbidden.

## Validation chain

1. `serde_json::from_str` with `deny_unknown_fields` (rejects unknown keys and
   malformed/non-JSON input — a hard error, never string-scanned around).
2. Enum checks: `sensitivity` and every `*.classification` in
   `{public,internal,confidential,restricted}`; `abuse_cases[].status` in
   `{active,superseded}`.
3. Frozen-ID invariant: every `abuse_cases[].id` matches
   `^tm-[a-z0-9-]+-abuse-\d+$`, ids unique, numbers contiguous from 1, a
   `superseded` row carries non-empty `superseded_by` + `supersede_reason`.
4. Provenance present: `producer_skill_sha` and non-empty `inputs[].sha`.

A fixture that fails any step is rejected by
`xtasks/sast-verify/tests/slo_tm_m1_schema.rs` (M1) and, from M2, halts
`/slo-critique` / `/slo-verify` rather than letting them silently re-derive.
