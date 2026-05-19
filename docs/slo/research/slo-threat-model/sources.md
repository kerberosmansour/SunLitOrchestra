# Sources — /slo-threat-model research

All URLs accessed 2026-05-19 via host-native web search/fetch. Repo-local artifacts are
labeled and cited by path at the commit checked out on `main` on 2026-05-19.

## OTM (Open Threat Model) 0.2.0

- [iriusrisk/OpenThreatModel — README](https://github.com/iriusrisk/OpenThreatModel/blob/main/README.md?plain=1) — top-level object model (otmVersion / project / assets / components / threats / mitigations); SemVer 2.0.0; schema 0.2.0; README does not model accepted/residual risk or compliance mapping.
- [iriusrisk/OpenThreatModel — otm_schema.json](https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json) — canonical JSON schema; Apache-2.0; required `otmVersion` + `project`.
- [Introduction to the Open Threat Model standard — IriusRisk](https://www.iriusrisk.com/resources-blog/introduction-to-the-open-threat-model-standard) — mitigations carry `riskReduction`; optional `attributes` (e.g. `OWASP-ASVS`); follows OpenAPI-style JSON approach.
- [Open Threat Model (OTM) — StartLeft docs](https://iriusrisk.github.io/startleft/Open-Threat-Model-(OTM)/) — OTM as JSON/YAML interchange; assets carry sensitivity (PCI/PII).

## Threat-modeling tools landscape

- [OWASP Threat Dragon](https://owasp.org/www-project-threat-dragon/) — free, OWASP; diagram-first; own JSON file format.
- [pytm — OWASP Developer Guide](https://devguide.owasp.org/en/04-design/01-threat-modeling/02-pytm/) — threat-modeling-as-code; source is the model, diagrams/reports are outputs.
- [A Dragon and Python walk into an OWASP card game — threatmodeling.dev](https://threatmodeling.dev/dragpyt/) — Threat Dragon format incompatible with pytm/Threagile/OTM; Threat Dragon + pytm participating in CycloneDX TMBOM effort.
- [Threat Modeling Tools: A Taxonomy (BU)](https://people.bu.edu/staro/Threat_Modeling_Tools_Survey.pdf) — taxonomy of threat-modeling tooling/automation.
- [Top 10 Threat Modeling Tools Compared 2024 — daily.dev](https://daily.dev/blog/top-10-threat-modeling-tools-compared-2024) — IriusRisk commercial, automated threat identification, multiple methodologies.

## CycloneDX TM-BOM (prior art / industry direction)

- [CycloneDX/specification](https://github.com/cyclonedx/specification) — full-stack BOM standard; TM-BOM under working-group development.
- [CycloneDX](https://cyclonedx.org/) — TM-BOM part of the ECMA-424 standardization track; 2026 modular file format unifying threat/risk modeling; release pending as of Jan 2026.
- [Threat Model Library — OWASP Developer Guide](https://devguide.owasp.org/en/04-design/01-threat-modeling/06-threat-model-library/) — OWASP threat-model library context.

## Public-repo threat-model exposure (breach risk)

- [GitHub Repositories Threat Model — GitHub Well-Architected](https://wellarchitected.github.com/library/application-security/recommendations/threat-model/) — repository as a detailed map of environment, trust relationships, infrastructure.
- [GitHub Under Attack — Truesec](https://www.truesec.com/hub/blog/github-under-attack) — repos enable long-term reconnaissance; small exposures snowball.
- [Addressing Inherent Risks in Code Repositories — Infosecurity Magazine](https://www.infosecurity-magazine.com/opinions/inherent-risks-code-repositories-1/) — dev platforms give attackers enough to craft targeted attacks.
- [Exposed Git Repos — Pentera](https://pentera.io/blog/git-repo-security-exposed-secrets/) — truffleHog/git-hound style repo intelligence gathering.
- [OWASP/threat-model-cookbook](https://github.com/OWASP/threat-model-cookbook) — deliberately *public* threat-model examples (synthetic/educational) — establishes the synthetic-vs-real publication split.
- [TalEliyahu/Threat_Model_Examples](https://github.com/TalEliyahu/Threat_Model_Examples) — public real-world-style threat-model examples; same synthetic-publication nuance.

## Repo-local artifacts (cited by path, `main` @ 2026-05-19)

- `skills/slo-architect/references/threat-model-template.md` — existing abuse-case row IDs `tm-<slug>-abuse-N`; four-state STRIDE cell vocabulary; `Residual risks` table; `Provenance` section; structural-contract-test-enforced required sections; no-silent-clobber re-run rule; `{{ARCHITECT_VERSION}}` = git SHA of `skills/slo-architect/SKILL.md` at emit.
- `references/sast/manifest-schema.md` — provenance convention `sldo-<skill>-version: <git-sha-of-SKILL.md-at-emit>`; `source-of-bug-shape` trail; strict `serde_yaml_ng` `deny_unknown_fields` parsing.
