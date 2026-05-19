---
name: slo-threat-model
researched: 2026-05-19
incomplete: false
---

# Research Dossier — /slo-threat-model shared read-first threat-model contract

**Wedge:** a minimal SLO-owned threat-model JSON schema with frozen abuse-case IDs and an
accepted-residual marker that `/slo-critique` and `/slo-verify` read and halt on rather than
re-derive.

**Target user:** the person running the SLO sprint who shipped a wrong test because abuse-case
`AC-3` drifted between `/slo-plan` and `/slo-execute`.

## Market

Nobody buys "a threat-model JSON schema" directly. The displaced spend is threefold: (1)
commercial threat-modeling platforms in the IriusRisk class, sold on enterprise quote-based
licensing, which SLO deliberately does not buy; (2) security-consultant threat-modeling
engagements that re-author the model by hand each cycle; (3) the internal rework cost of the
exact failure in the idea doc — a wrong test shipped because abuse-case IDs were not stable
across skill invocations. The proxy spend the wedge attacks is (3): reconciliation tax and
wrong-test rework inside the SLO sprint loop, not a license SLO would otherwise pay.

## Direct competitors

These are threat-model *formats / tools*, since the artifact under design is a machine-readable
threat model, not a SaaS product.

| Name | Price | Key feature | Gap vs our wedge |
|---|---|---|---|
| Open Threat Model (OTM) 0.2.0 (IriusRisk) | Free, Apache-2.0 spec | Platform-independent JSON/YAML threat-model interchange; components/assets/representations carry author-controlled `id` strings; mitigations carry `riskReduction` and an optional `attributes` block (e.g. `OWASP-ASVS`) | README confirms **no first-class accepted/residual-risk state** and **no compliance-mapping object**; no statement that re-runs preserve IDs. SLO needs accepted-vs-missing distinction + SOC2/ASVS/GDPR mapping as first-class — OTM forces an adapter + extension fields. "Too brittle" hypothesis borne out. |
| OWASP pytm | Free, OWASP/MIT | Threat-modeling-as-code: the Python source *is* the model; diagrams/reports are outputs, not inputs | Model is code, not a static contract artifact other skills can read without executing Python; format incompatible with OTM/Threat Dragon. SLO needs a static JSON other skills read, not an executable model. |
| OWASP Threat Dragon | Free, OWASP | Visual drag-and-drop diagram-first modeling with its own JSON file format | Threat Dragon's file format is explicitly incompatible with pytm, Threagile, and OTM; diagram-first, not contract-first; no stable cross-skill abuse-case ID guarantee. |
| IriusRisk (commercial) | Commercial, enterprise quote-based | Automated threat identification, multiple methodologies, originator/maintainer of OTM | Heavyweight commercial platform; reintroduces exactly the external dependency the SLO 2026-04 cleanup thesis rejects. Out of scope as canonical store. |

## Adjacent tools

| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|
| threatspec | Threat modeling *from* source-code annotations, not a standalone reviewable contract artifact | No — annotation-driven; would require the target product to be annotated, which SLO's skill-to-skill contract cannot assume. |
| CycloneDX TM-BOM (Threat Model Bill of Materials) | Industry-standard machine-readable threat-model format, under active CycloneDX working-group development toward ECMA-424; **release pending as of Jan 2026** | Eventually yes, as an *optional export target* — Threat Dragon and pytm are already converging on it. Not a canonical store today because it is unreleased; this validates SLO-owned-now / export-later. |
| OWASP threat-model-cookbook / TalEliyahu Threat_Model_Examples | Public GitHub repos that deliberately publish threat models | Not a tool to consume — but they establish the practice split that informs the breach risk: *synthetic/educational* threat models are published publicly; *real product* threat models with real residual-risk lists are treated as sensitive. |

## Technical prior art

- **OTM schema** — `https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json`, Apache-2.0. Reference for the object shape (otmVersion / project / assets / components / threats / mitigations) and for what to deliberately *not* inherit (no residual-risk state, no compliance object).
- **CycloneDX TM-BOM** — `https://github.com/CycloneDX/specification`. The industry direction; design the SLO schema so a future TM-BOM export is additive, mirroring the idea doc's "OTM only ever an optional export" stance.
- **OWASP pytm** — `https://owasp.org/www-project-pytm/`. Prior art for the threat-modeling-as-code philosophy SLO is *rejecting* for the contract artifact (model must be statically readable by skills, not executed).
- **Repo-local: `/slo-architect` threat-model template** — `skills/slo-architect/references/threat-model-template.md`. Already defines sequential abuse-case row IDs `tm-<slug>-abuse-N`, a four-state STRIDE cell vocabulary (`eliminated by` / `mitigated by` / `N/A — reason` / `residual risk — exploit path`), a dedicated `Residual risks` table (Risk / Exploit path / Compensating control / Owner / Review by), a `Provenance` section, structural-contract-test-enforced required sections, and a "no silent clobber — surface the diff, prompt overwrite/merge/skip" re-run rule. The schema must serialize *this existing structure*, not invent a parallel one.
- **Repo-local: SAST rule manifest schema** — `references/sast/manifest-schema.md`. Establishes the repo's provenance convention: `sldo-<skill>-version: <git-sha-of-skills/<skill>/SKILL.md-at-emit>`, plus a `source-of-bug-shape` provenance trail, strict-parsed with `serde_yaml_ng` `deny_unknown_fields`. The threat-model JSON should match this pattern (git-SHA-of-producing-SKILL.md, strict unknown-field rejection), not introduce a novel content-hash scheme.

## Regulatory / legal

- **Licensing: no blocker.** OTM is Apache-2.0; CycloneDX is permissive (ECMA-424 track). SLO can reference, adapt structure from, and optionally export to either without a license constraint.
- **Information-disclosure / breach flag: real and confirmed.** Industry consensus is explicit that threat models document architecture, trust boundaries, identified weaknesses and *control gaps*, that public git repos are long-term reconnaissance sources, and that threat models should be treated as sensitive and kept in private/restricted repos. A machine-readable artifact with `accepted_residual: true` entries in a public repo is a structured, scrapeable list of known-unfixed weaknesses. The design must therefore apply the repo's existing two-tier split (the biz pack's gitignored `docs/biz/` vs git-tracked `docs/biz-public/`) to `*-threat-model.slo.json`, OR emit a redacted public companion — real residual-risk detail must not be the default-committed surface.
- **UK GDPR / DUAA 2025 false-assurance flag (from idea doc, not contradicted by research):** a stale frozen JSON that lets `/slo-critique` greenlight uncovered `AC-N` in a downstream UK product, then is cited by the SLO audit-defense manifest, converts the wrong-test failure into a regulator-facing false attestation. Research did not find a counter-practice; the schema must carry provenance (producing-skill git SHA + input SHAs) so a stale model is detectable, not silently trusted.

## Open questions that research did not answer

- **Exact OTM threat-object field names** — the public README is truncated above the `threats[]` structure; confirming whether OTM threats carry an author-stable `id` (vs generated) requires reading `otm_schema.json` directly during `/slo-architect`. Low risk: OTM is an *optional later export*, not the canonical store, so this does not block the wedge.
- **TM-BOM field shape** — unreleased; cannot design an export adapter against it yet. Correct posture is to keep the SLO schema minimal and additive, re-evaluate when ECMA-424 TM-BOM ships.
- **Whether to redact-in-place vs two-tier-split** the public companion — this is a design decision for `/slo-architect`, not a fact research can source; both are viable and the repo has precedent for the two-tier split.
