---
filed_issues: []
---

# Lessons Learned — slo-threat-model Milestone 1

## What changed

- New SLO-owned threat-model JSON schema reference at
  `references/security/threat-model-schema.md` (sibling of
  `references/sast/manifest-schema.md`).
- New hand-authored fixture
  `docs/slo/design/slo-security-embedding-threat-model.slo.json` serializing
  the existing dogfood Markdown threat model losslessly.
- New Rust structural-contract test
  `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` binding the schema shape,
  the frozen-ID invariant, the classification field, and strict
  unknown-field/parse rejection.

## Design decisions and why

- **Preserved the real abuse-case IDs `tm-slo-sec-abuse-1..8`, not the
  runbook's assumed `tm-slo-security-embedding-abuse-N`.** The source dogfood
  model uses the short slug. Using the runbook's literal-but-mistaken string
  would itself have been a *renumber/rename* — the exact failure this whole
  feature exists to prevent. Honoring the frozen-ID *intent* over the
  literal-but-wrong string is the disciplined choice, and it validated the
  wedge's core thesis on its very first fixture.
- **Test regex kept general `^tm-[a-z0-9-]+-abuse-\d+$`** so it binds the
  invariant (shape + uniqueness + contiguity) rather than a slug literal —
  this is also what M2's consumer contract needs.
- **`serde_json` strict structs with `#[serde(deny_unknown_fields)]`** rather
  than hand-rolled parsing (critique ENG-2): the frozen-ID uniqueness check
  would be silently defeated by a substring match in a hand-rolled scanner.

## Mistakes made

- The runbook (and the design docs) assumed the dogfood model's abuse-id slug
  without reading the actual file first. Caught at execution Step 1 by reading
  the source model before serializing — but it should have been caught at
  `/slo-architect` / `/slo-plan` time.

## Root causes

- `/slo-architect` and `/slo-plan` reasoned about the abuse-id convention from
  the *template* (`tm-<slug>-abuse-N`) and the feature slug, not from the
  concrete target fixture file. The convention is real; the slug substitution
  was an unverified inference.

## What was harder than expected

- Nothing was materially harder. The biggest time cost was a working-directory
  slip during critique (a `cd` into the skills dir persisted), which produced a
  transient false "Cargo.toml missing" reading — resolved by always using the
  absolute repo path. Verifying before asserting prevented a false critique
  finding (serde_json was actually present).

## Naming conventions established

- Schema doc: `references/security/threat-model-schema.md`.
- Companion artifact: `docs/slo/design/<slug>-threat-model.slo.json`.
- Frozen abuse-id: `^tm-<slug>-abuse-\d+$`, contiguous from 1,
  supersede-don't-renumber (`status` ∈ {active,superseded}; superseded ⇒
  non-empty `superseded_by` + `supersede_reason`).
- Test file: `xtasks/sast-verify/tests/slo_tm_m<N>_<feature>.rs`
  (snake-case; one per milestone, matching the `sap_imp_*` precedent).
- Provenance idiom: `producer_skill_sha` (git SHA of producing SKILL.md) +
  `inputs[].sha` (git blob SHAs) — matches the SAST manifest schema; no
  content-hash idiom.

## Test patterns that worked well

- `workspace_root()` cwd-or-`CARGO_MANIFEST_DIR`-parent-parent helper copied
  verbatim from `sap_imp_m5_agents.rs` — zero friction.
- An in-memory `strict_parse_rejects_unknown_and_malformed` test that needs no
  fixture, so it stays green even in the BDD-first red phase — cleanly
  separates "logic is right" from "artifact exists yet".
- Verification by **mutating a copy of the fixture** and observing each guard
  fail, then restoring byte-identically (SHA-256 round-trip check). This
  proved the guard *bites*, not merely that it passes on good input.

## Missing tests that should exist now

- A live `status: superseded` fixture row is not exercised by the dogfood
  model (it legitimately has none). The supersede branch is covered by test
  logic but not by a real superseded row. M2 / the producer runbook should add
  a fixture with a superseded entry. Recorded as a coverage note, not an M1
  gap.

## Rules for the next milestone (M2)

- **Read the actual file before asserting its shape.** M2 edits
  `slo-critique/SKILL.md` (F-ENG-6 governed) and `slo-verify/SKILL.md` — read
  both fully and `rg` for any other SHA-pin before editing.
- **M2's consumer test must assert the SEC-1 fence rule and the ENG-1
  additive-edit anchors**, not just the presence of phrases — the M1 lesson is
  that a guard must bind the invariant, not the prose around it.
- **Update the `sap_imp_m5_agents.rs` SHA-256 constant in the same milestone**
  as the `slo-critique/SKILL.md` edit, with the F-ENG-6 amendment recorded —
  the constant must be on the M2 allow-list (it is).
- Keep using `serde_json` for any JSON the consumer contract describes; never
  hand-roll.

## Template improvements suggested

- `/slo-plan` / `/slo-architect` should resolve the *concrete* abuse-id slug
  from the target threat-model file when one exists, rather than substituting
  the feature slug into the `tm-<slug>-abuse-N` template. Captured as a
  process nuance here; below the bar for a standalone tracked issue (it was
  self-correcting at execution time and is now documented). The consequential
  carry-forwards (producer runbook, SEC-2 redaction) are explicit M2-retro
  obligations, not M1's.
