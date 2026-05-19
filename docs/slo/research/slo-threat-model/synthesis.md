# Synthesis — what the research means for the /slo-threat-model design

## Do not invent a schema; serialize the one that already exists

The decisive finding is repo-local: `/slo-architect`'s threat-model template already defines
the exact structure the wedge needs — sequential abuse-case row IDs `tm-<slug>-abuse-N`, a
four-state STRIDE cell vocabulary (`eliminated by` / `mitigated by` / `N/A — reason` /
`residual risk — exploit path`), a first-class `Residual risks` table with owner and review
date, a `Provenance` section, and a structural-contract test that already asserts those
sections exist. The accepted-risk-vs-missing-coverage distinction the idea doc treats as a gap
is already modeled in prose; it is just not machine-readable or ID-stable. The design must
serialize the existing template structure (including the `tm-<slug>-abuse-N` ID convention,
*not* a new `AC-N` form) rather than introduce a parallel schema, because
`skills/slo-architect/references/threat-model-template.md` is the artifact every downstream
skill already cites.

## Stable IDs are a schema invariant no off-the-shelf format guarantees

OTM carries author-controlled `id` strings on components/assets but its README models no
accepted/residual-risk state, no compliance mapping, and makes no re-run ID-preservation
guarantee; pytm's model is executable Python, not a static contract; Threat Dragon's format is
explicitly incompatible with the others. None of the surveyed formats provides the one property
the idea doc's pain depends on — an abuse-case ID that survives a re-run unrenumbered. The
design must make "frozen `tm-<slug>-abuse-N`, supersede-don't-renumber" an explicit
schema-level invariant enforced by the structural-contract test, because no external format
(OTM / pytm / Threat Dragon) supplies it and prose discipline already failed once
(idea-doc pain + `https://owasp.org/www-project-threat-dragon/`).

## SLO-owned-now, optional-export-later is externally validated, not just a preference

CycloneDX TM-BOM — the industry convergence point that Threat Dragon and pytm are already
moving toward — is under ECMA-424 working-group development with release pending as of
Jan 2026, and OTM's README confirms it cannot natively express SLO's residual-risk and
compliance needs. Betting the canonical store on either is premature. The design must keep the
SLO-owned JSON minimal and additive so a future TM-BOM (or OTM) export is a non-breaking add-on
rather than a migration, because TM-BOM is unreleased and OTM is structurally insufficient
(`https://cyclonedx.org/` + OTM README).

## The machine-readable artifact must not be the default public surface

Industry consensus is unambiguous that threat models document trust boundaries, weaknesses and
control gaps, that public repos are durable reconnaissance sources, and that threat models
should be treated as sensitive; the only public threat models in practice are synthetic
educational ones (OWASP threat-model-cookbook). A `*-threat-model.slo.json` carrying
`accepted_residual: true` rows in a public repo is a scrapeable list of known-unfixed
weaknesses. The design must apply the repo's existing two-tier precedent (gitignored
`docs/biz/` vs git-tracked `docs/biz-public/`) — or emit a redacted public companion — so the
full residual-risk detail is never the default-committed surface, because public-repo threat
models are confirmed reconnaissance material
(GitHub Well-Architected + Truesec + OWASP threat-model-cookbook).

## Provenance must match the repo's existing convention, not a new scheme

The repo already has a settled provenance pattern in two places: the SAST manifest's
`sldo-<skill>-version: <git-sha-of-SKILL.md-at-emit>` plus strict `deny_unknown_fields`
parsing, and the threat-model template's `{{ARCHITECT_VERSION}}` = git SHA of the producing
SKILL.md. The idea doc's open question of "content hash vs commit SHA" is therefore already
answered by precedent. The design must record provenance as the producing skill's SKILL.md git
SHA plus the input doc SHAs, and parse the JSON with strict unknown-field rejection, because
that is the established repo convention (`references/sast/manifest-schema.md` +
`skills/slo-architect/references/threat-model-template.md`) and divergence would create a
second, inconsistent provenance idiom.

## Threat-model diffs already have a home — surface them, don't store them

The existing `/slo-architect` re-run rule is "detect existing, surface the diff, prompt the
user to overwrite/merge/skip — no silent clobber." Research found no external practice that
contradicts inline diff-surfacing, and a separate `docs/slo/threat-model-diffs/` tree would be
a new artifact class with no precedent. The design must reuse the surface-and-prompt re-run
behavior and represent supersession inside the JSON (a superseded `tm-<slug>-abuse-N` carries
its replacement and reason) rather than creating a diff directory, because the no-silent-clobber
convention already exists and no source justifies a parallel diff store
(`skills/slo-architect/references/threat-model-template.md`).
