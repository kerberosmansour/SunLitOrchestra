---
name: slo-threat-model
created: 2026-05-19
status: ideation
tla_required: false    # provisional — /slo-architect finalizes this
---

# /slo-threat-model — a shared, read-first threat-model contract

## The pain

The person running the SLO sprint ran `/slo-plan` then `/slo-execute` on a feature, and a milestone's BDD scenario verified the *wrong* abuse case: `AC-3` silently changed meaning between `/slo-plan` and `/slo-execute` because every skill (`/slo-architect` Step 3.5, `/slo-plan`, `/slo-critique`, `/slo-verify`, `/slo-sec-libs`) re-derives the threat model independently with no shared persisted source of truth, and `/slo-verify` greenlit the milestone anyway. The coping mechanism is worse than the bug: `/slo-plan` Contract Blocks now write "abuse cases" in prose instead of citing `AC-3`, because the ID will not survive to `/slo-critique` — so the abuse-case traceability the v4 runbook template promises is theatre. The originating tracker is GitHub issue #67.

## Five capabilities the user described without realizing

- A single persisted artifact pair (`<slug>-threat-model.md` + `<slug>-threat-model.slo.json`) that is the *one* source of truth all skills read instead of re-deriving.
- Frozen abuse-case IDs: `AC-N` is assigned once and never renumbered on re-run; changes supersede, they do not renumber.
- An explicit `accepted_residual` marker so `/slo-critique` / `/slo-verify` can distinguish *accepted risk* from *missing coverage* — the two are currently indistinguishable.
- Read-side enforcement that makes `/slo-critique` and `/slo-verify` **halt rather than silently re-derive** when the frozen JSON is missing or schema-invalid.
- Machine-checkable structural-contract tests over the schema and ID-freeze rule — because prose discipline is exactly what failed the first time.

## Top risks

- **Breach** (worst): `docs/slo/design/<slug>-threat-model.slo.json` is git-tracked in a public repo (SunLitOrchestra is public; downstream founder repos often are). A machine-readable artifact carrying `data_classification`, `auth_boundary`, and `accepted_residual: true` entries is a pre-written reconnaissance map — an opportunistic attacker scraping GitHub for `*-threat-model.slo.json` gets a structured, published list of the target's *known-unfixed* weaknesses. This feature creates that surface where prose Markdown partially obscured it. Adversary: opportunistic GitHub-scraping attacker against any public SLO-using repo.
- **Compliance fine**: `/slo-critique` trusts a stale frozen JSON and greenlights a milestone as "`AC-3` covered" when it never was, in a downstream UK founder product handling personal data. The SLO audit-defense manifest then cites a threat model asserting coverage that does not exist. Regulation: UK GDPR / DUAA 2025; data class: personal data in the user's product; scale: ICO enforcement against the *user's company*, with the false assurance traceable to our greenlight.
- **Prolonged outage**: read-side enforcement is too strict — `/slo-critique` and `/slo-verify` hard-halt whenever the frozen JSON is absent or schema-invalid. Every in-flight runbook predating the schema can no longer pass critique or verify. First to notice: the next person running `/slo-verify M<N>` on any existing runbook — it hard-stops with no migration path. Defection: they add an override or comment out the check, and the sprint loop is back to ID drift *plus* a false belief the contract is enforced.

## Approach A — conservative (SLO-owned Markdown + minimal JSON, test-enforced)

- **Effort**: ~1 person-week.
- **Wedge**: Define a minimal SLO-owned threat-model JSON schema with frozen `AC-N` IDs and an `accepted_residual` marker; hand-author one fixture from an existing dogfood threat model (e.g. `docs/slo/design/slo-security-embedding-threat-model.md`); add read-side contract language to `/slo-critique` and `/slo-verify` so they read the frozen JSON and halt rather than re-derive IDs; lock it with structural-contract tests. **Zero new skill directories** — no `skills/slo-threat-model/SKILL.md` yet. Schema deliberately shaped so a typed Rust validator can wrap it later with no breaking change, and so OTM 0.2.0 is only ever an optional later export.
- **Risks**: a schema designed around one consumer (`/slo-critique`) can ossify wrong; ID-freeze is enforced by a fixture + structural test rather than a runtime guard, so a skill author can still drift if the test does not bind tightly enough. Mitigation: the structural-contract test must assert the ID-freeze and publication-exposure rules directly, not the prose around them.

## Approach B — adopt OTM 0.2.0 as the canonical machine artifact

- **Effort**: ~2–3 person-weeks.
- **Wedge**: same consumer contract, but the canonical store is OTM 0.2.0 with SLO extension fields for stable `AC-N` IDs, `accepted_residual`, and compliance mapping.
- **Risks**: OTM does not natively carry SLO's stable IDs, accepted-residual distinction, or compliance mapping, so an adapter + extension fields ship anyway; `/slo-critique` reasons over an impedance-mismatched artifact; you inherit an external schema's churn for a problem it was not shaped for. The issue itself flags "if OTM is too brittle." Not the week-one path; viable only as a later optional *export* from the Approach A schema.

## Approach C — typed Rust enforcement crate / xtask

- **Effort**: ~2–3 person-weeks.
- **Wedge**: a `slo-threat-model` xtask owns parse + ID-freeze + supersede-don't-renumber + diff as typed invariants; `/slo-critique` and `/slo-verify` shell to `cargo xtask threat-model verify` the way `/slo-ruleverify` shells to `sast-verify gate`. ID-freeze becomes a machine-checked invariant, not prose.
- **Risks**: reintroduces a binary dependency into a skill pack whose 2026-04 cleanup thesis was "skills are the canonical interface, Rust is minimized." Heavier than the wedge needs. Best treated as a *later wrapper* over the Approach A schema once the consumer contract is proven — explicitly designed for, not built in week one.

## Recommendation

**Approach A now**, with the JSON schema deliberately shaped so Approach C can wrap it later with zero breaking change, and Approach B (OTM) only ever an optional later export — never the canonical store. The week-one wedge is read-side only and ships **zero new skill directories**: a schema reference doc, one hand-authored fixture from an existing dogfood threat model, read-side contract language in `/slo-critique` and `/slo-verify` that halts rather than re-derives IDs, and structural-contract tests that bind the ID-freeze and publication-exposure rules directly. The schema's design driver is `/slo-critique` (prevention before execution beats detection at verify). The central design tension carried into `/slo-architect`: prose discipline is exactly what failed the first time, so both the ID-freeze rule *and* the public-repo publication-exposure handling must be machine-checked in the structural-contract tests, not enforced by SKILL.md wording alone. Prove the consumer contract before building the producer; the standalone `/slo-threat-model` producer skill and `/slo-architect` Step 3.5 delegation are deliberately deferred past the wedge.

## Open questions for /slo-research

1. Is OTM 0.2.0's threat/mitigation model expressive enough to later carry SLO stable `AC-N` IDs, `accepted_residual`, and compliance mapping as extension fields without a lossy adapter — or is the issue's "too brittle" concern borne out by the spec?
2. What is the minimum stable schema surface (`abuse-case ID`, `accepted_residual`, exposure metadata, provenance hash) that downstream skills actually need, versus what would be speculative generality before the producer exists?
3. For the breach risk: what is the prevailing practice for committing machine-readable threat-model artifacts to public repos — is there an established redaction/`.gitignore` split (cf. the biz pack's `docs/biz/` vs `docs/biz-public/` two-tier model) that should apply to `*-threat-model.slo.json`?
4. Where should threat-model diffs live so `/slo-critique`, `/slo-verify`, and the planned `/slo-security-test` can all consume them: inline in the updated doc, under `docs/slo/threat-model-diffs/`, or inside verification/security-test reports?
5. How do comparable contract artifacts in this repo (e.g. the SAST audit-defense manifest, `xtasks/sast-verify` gate) encode provenance (content hash vs commit SHA), so the threat-model schema's provenance field matches an existing proven pattern rather than inventing a new one?
