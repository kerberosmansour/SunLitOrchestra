# Stack Decision — slo-threat-model

## Chosen stack

- **Schema artifact:** an SLO-owned Markdown schema reference doc at
  `references/security/threat-model-schema.md`, plus one hand-authored
  `.slo.json` fixture under `docs/slo/design/`.
- **Consumer enforcement:** Markdown SKILL.md edits to `skills/slo-critique/`
  and `skills/slo-verify/` (read-side contract language).
- **Guard:** one Rust structural-contract test under
  `xtasks/sast-verify/tests/`, run by `cargo test`.
- **No new crate, no new skill directory.**

## Reason

The research synthesis is decisive: the design must *serialize the structure
that already exists* in `skills/slo-architect/references/threat-model-template.md`
rather than invent one, because that template is the artifact every downstream
skill already cites. The repo's Markdown-first skill-contract non-negotiable
(ARCHITECTURE.md "Skill pack invariants") plus the existing precedent of
`references/sast/manifest-schema.md` (a strict-parsed schema reference consumed
by a deterministic gate) make a Markdown schema doc + Rust structural-contract
test the lowest-risk shape that still makes the frozen-ID invariant
*machine-checked* — which the research synthesis says is mandatory because
prose discipline already failed once. CycloneDX TM-BOM is unreleased
(ECMA-424, pending Jan 2026) and OTM 0.2.0 has no native accepted-residual or
compliance modelling, so an SLO-owned minimal schema with optional later
export is the externally-validated path, not a preference.

## Rejected alternatives

- **OTM 0.2.0 as canonical store** — README confirms no accepted/residual-risk
  state and no compliance object; would force an adapter + extension fields for
  the exact properties the wedge needs. Kept only as a possible later export.
- **CycloneDX TM-BOM as canonical store** — unreleased as of Jan 2026;
  cannot design a stable contract against a moving working-group target.
- **New `sldo-threat-model` crate / xtask with a typed validator** — heavier
  than a one-week read-side wedge needs and reintroduces a binary dependency
  the 2026-04 cleanup thesis minimized. The schema is deliberately shaped so a
  typed validator can wrap it later with no breaking change (reversibility doc).
- **A new `skills/slo-threat-model/` producer skill now** — violates the
  wedge's "prove the consumer contract before building the producer" discipline
  and the idea-doc's zero-new-skill-directories constraint.

## Non-negotiables (downstream cannot change these without migration)

- Schema doc location: `references/security/threat-model-schema.md`.
- Companion artifact name: `docs/slo/design/<slug>-threat-model.slo.json`.
- Abuse-case ID convention: `tm-<slug>-abuse-N` (existing), frozen,
  supersede-don't-renumber.
- Provenance: producing-skill SKILL.md git SHA + input-doc SHAs; strict
  unknown-field rejection (same idiom as `references/sast/manifest-schema.md`).
- One structural-contract test file per milestone, named
  `<prefix>_m<N>_<feature>.rs` under `xtasks/sast-verify/tests/`.
