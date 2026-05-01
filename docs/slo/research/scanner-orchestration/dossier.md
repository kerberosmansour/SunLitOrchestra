---
name: scanner-orchestration
researched: 2026-04-26
incomplete: false
---

# Research Dossier — Scanner Orchestration (`/slo-sast` v1)

## Market

Solo and small-team OSS maintainers who run no SAST today and would only adopt one if (a) it tunes itself to their threat model so the findings are relevant, (b) it doesn't flood CI with legacy criticals on day one, and (c) it ships as a Claude Code skill they invoke once rather than infrastructure they maintain. The proxy spend is the time and reputation cost of post-disclosure clean-up: hours-to-days of patching, advisory drafting, and contributor-community reassurance per missed CVE. Commercial SAST products (Snyk Code, Checkmarx One, Veracode, GitHub Advanced Security) target enterprise procurement; none ingest threat models, and none of the OSS-friendly options (Semgrep CE, CodeQL via GHAS for OSS) close the loop from threat model to ruleset.

## Direct competitors

| Name | Price | Key feature | Gap vs our wedge |
|---|---|---|---|
| _(none found)_ | — | — | The pipeline found no published OTM-or-similar → Semgrep config converter in OSS or commercial tooling across multiple search rounds. The "threat-model edit → ruleset re-derivation" loop is the wedge; it remains unoccupied. See synthesis Q5 verdict. |

## Adjacent tools

| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|
| **SecOpsTM** (ellipse2v, OSS) | STRIDE-as-code with MITRE ATT&CK / D3FEND / CAPEC mapping; outputs reports + Navigator layers, not Semgrep configs. Closest adjacent player on the threat-model → automated-security-artifact spectrum. | Plausibly — they already parse threat models structurally. Would require adding a Semgrep emitter. |
| **Autogrep** (LambdaSec, research artifact) | Automates Semgrep rule *generation* from CVE patches via LLMs. Orthogonal axis: generation, not selection. | No — different problem. Tangentially relevant to a future `/slo-rulegen` integration. |
| **AppSec Untangled "Threat Modeling Handbook #5"** (Medium, 2023) | Manual practice — convert a threat model to custom Semgrep rules in CI. Documents the workflow but not as automation. | No — it's a methodology blog post, not a tool. |
| **Phoenix Security — Semgrep ASPM integration** (commercial, quote-required) | ASPM correlation / enrichment over scan output, not config generation. | Plausibly — they sit on Semgrep findings; could add upstream config control. |
| **Snyk Code policy-as-code** (commercial; OSS tier exists) | Policy / config files authored manually; does not consume threat models. | Yes if they wanted to — but their roadmap shows no signal toward threat-model intake. |
| **GitHub Advanced Security CodeQL config** (free for OSS, $) | Config files authored manually; does not consume threat models. | Yes if GitHub wanted to — most strategic threat. |
| **Checkmarx One presets** (commercial, quote-required) | Policy / preset files authored manually; does not consume threat models. | Yes — but they target enterprise SAST procurement, not OSS-maintainer wedge. |
| **Veracode policy framework** (commercial, quote-required) | Policy files authored manually; does not consume threat models. | Same as Checkmarx — enterprise focus. |
| **`sast-semgrep` Claude skill** (AgentSecOps, OSS) | Wires Semgrep into a project as a skill; does not read a threat model. | Yes — closest skill-pattern peer; could add threat-model intake. |
| **`claude-build-workflow-sast-semgrep` Claude skill** (rohunj, OSS) | Same shape — Semgrep into a project, no threat-model intake. | Same as above. |

## Technical prior art

- **Open Threat Model (OTM) spec 0.2.0** ([iriusrisk/OpenThreatModel](https://github.com/iriusrisk/OpenThreatModel)) — has `threats[].cwes` array; CC-BY-SA-4.0; stagnant since 2023; no SAST consumer documented. StartLeft converter is input-only (MTMT / Terraform / Visio → OTM), not OTM → SAST.
- **OWASP pytm** ([OWASP/pytm](https://github.com/OWASP/pytm)) — code-as-threat-model; uses internal IDs (`INP01`, `CR01`) rather than CWEs; outputs JSON / Graphviz / PlantUML / Markdown; no Semgrep adapter in-tree.
- **OWASP Threat Dragon** ([owasp.org/www-project-threat-dragon/](https://owasp.org/www-project-threat-dragon/)) — Apache-2.0 OWASP flagship; JSON + OTM exports; threats can carry CWE refs but population is uneven; TMF format incompatible with pytm/Threagile/OTM per its own wiki.
- **CycloneDX Threat-Model BOM (TM-BOM)** ([OWASP Threat Model Library](https://owasp.org/www-project-threat-model-library/)) — in-flight standard meant to supersede TMF; no shipping consumers as of 2026-04.
- **Semgrep registry rule schema** (`metadata.cwe`, `metadata.technology`, `metadata.owasp`) — mandatory on `category: security` rules per the [contributing guide](https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository); enforced by [`metadata-cwe.yaml` linter](https://github.com/returntocorp/semgrep-rules/blob/develop/yaml/semgrep/metadata-cwe.yaml). Long-form CWE strings (`"CWE-89: Improper Neutralization..."`).
- **Semgrep CI auto-baseline behavior** ([CI environment variables doc](https://semgrep.dev/docs/semgrep-ci/ci-environment-variables)) — `semgrep ci` auto-detects diff range from PR event payload on GHA / GitLab; `SEMGREP_BASELINE_REF` / `--baseline-commit` reserved for "other" CI providers.
- **Stand-alone severity-gating verdict** ([Configure blocking findings doc](https://semgrep.dev/docs/semgrep-ci/configuring-blocking-and-errors-in-ci)) — verbatim: *"If you do not use Semgrep AppSec Platform with Semgrep in CI or Semgrep Managed Scans (that is, you are using a stand-alone setup), all Semgrep findings are blocking findings."*
- **2025-12-08 GitHub `pull_request_target` mitigation** ([GitHub Changelog 2025-11-07](https://github.blog/changelog/2025-11-07-actions-pull_request_target-and-environment-branch-protections-changes/)) — workflow file and checkout commit forced to default branch regardless of PR base. Kills the "stale-base-branch workflow exploit" attack class but `pull_request_target` still confers full secret access.
- **`tj-actions/changed-files` compromise — CVE-2025-30066** ([advisory GHSA-mrrh-fwg8-r2c3](https://github.com/advisories/ghsa-mrrh-fwg8-r2c3), March 2025) — canonical "tag rewriting" supply-chain failure case. Motivates SHA pinning of every third-party action.
- **GitHub Security Lab "Preventing pwn requests"** ([Part 1](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/), [Part 4](https://securitylab.github.com/resources/github-actions-new-patterns-and-mitigations/)) — canonical guide on `pull_request_target` misuse and the two-workflow split pattern.
- **Trail of Bits Testing Handbook — Semgrep CI chapter** ([appsec.guide](https://appsec.guide/docs/static-analysis/semgrep/continuous-integration/)) — community recipe for Semgrep + GHA on noisy existing repos.
- **Academic: "Semgrep*: Improving the Limited Performance of SAST Tools"** ([ACM EASE 2024](https://dl.acm.org/doi/10.1145/3661167.3661262)) — hand-authored rule additions for +181% detection; tangential (rule authoring vs selection).
- **OWASP Top 10 2025 re-mapping** ([Semgrep blog](https://semgrep.dev/blog/2026/owasp-top-10-2025-whats-new/)) — completed early 2026; all 4,000+ registry rules carry OWASP 2025 + CWE metadata.

## Regulatory / legal

- **PCI DSS v4.0.1, Requirement 6.2.3** — *"Bespoke and custom software is reviewed prior to release into production or to customers, to identify and correct potential coding vulnerabilities."* Manual / automated / hybrid all acceptable. Reviewer must be other than the originating author, knowledgeable in code review and secure coding, with management approval prior to release. **Note**: the idea doc cites 6.3.2 — that's v3.2.1 numbering; v4.0.1's 6.3.2 is the new SBOM-inventory mandate (different scope, out of v1).
- **SOC 2 CC7.1 (Trust Services Criteria 2017 / 2022 revisions)** — system-operations control covering detection of new vulnerabilities; SAST output is generally accepted as evidence but specific format/retention requirements are not publicly fixed.
- **NIST SSDF SP 800-218, PW.7 (review human-readable code) and PW.8 (test executable code)** — implementation examples include automated static analysis in CI; retention referenced abstractly. PDF text extraction failed in research; verbatim sub-tasks are an open question.
- **License — Semgrep CE: LGPL-2.1**; Semgrep community rules: Semgrep Rules License (review for downstream use). The skill emits config + workflow files only; it does not redistribute Semgrep or its rules. No license collision foreseen for v1.
- **License — emitted artifacts**: the `.semgrep.yml`, `.github/workflows/sast.yml`, and selected rule files are authored or selected (not modified) and committed to the user's repo under their license. SLO does not impose its own license on the emitted output.

## Open questions that research did not answer

1. **`--config` vs `SEMGREP_RULES` env var on `semgrep ci`** — CLI docs say `--config` is "not supported in ci mode" but real workflows use it and it works. Empirical confirmation needed (run against a fixture, check release notes for forthcoming deprecation). Blocks workflow-template authoring at execute time.
2. **Verbatim PCI DSS v4.0.1 testing procedure 6.2.3.a/.b text** — paraphrases only from public-web search; authoritative version requires the [PCI Council document library](https://www.pcisecuritystandards.org/document_library/) (membership / direct PDF). Blocks any post-v1 coverage doc claiming "supports 6.2.3 evidence."
3. **NIST SSDF SP 800-218 PW.7 / PW.8 verbatim sub-tasks** — PDF binary extraction failed via WebFetch. Re-fetch via `chub` / direct download or pull [`nist.sp.800-218.ssdf-table.xlsx`](https://csrc.nist.gov/files/pubs/sp/800/218/final/docs/nist.sp.800-218.ssdf-table.xlsx). Blocks defensible mapping table.
4. **`metadata.cwe` coverage gaps in `semgrep-rules` per language** — empirical sweep needed (% rules with CWE field, by language, by year) for v1's stacks (Python, JS/TS, Go, Java, Rust, Ruby, PHP). Achievable via a small script during architect phase.
5. **Mapped-but-not-scanned CWE coverage claim as a documented audit-failure pattern** — plausible but unsourced after multiple search rounds. Worth a targeted search of QSA-firm postmortems (Coalfire, Schellman, A-LIGN). If null, frame the manifest's `cwes_actually_covered` field as defensive design rather than regulatory necessity.
6. **SOC 2 CC7.1 specific evidence formats and retention** — AICPA TSC primary read needed; Vanta / Drata template artifacts may surface concrete requirements. Blocks defensible retention default in workflow template.
7. **`pull_request_target` post-2025-12-08 residual exploit surface** — even with default-branch sourcing, what residual attack patterns remain? GHSecLab Part 4 covers some; verify completeness before the v2 two-workflow split.
8. **`actions/checkout` and `github/codeql-action/upload-sarif` SHA refresh cadence** — propose 90-day cadence as SLO project policy; confirm operationally during execute phase.
9. **CycloneDX TM-BOM trajectory** — re-check 2026-10 for shipping consumers in Threat Dragon / pytm. Affects whether v2 OTM-optional path swaps to TM-BOM.
10. **Semgrep SARIF + multi-CWE round-trip** — historical [issue #4673](https://github.com/semgrep/semgrep/issues/4673) broke array-typed `cwe` / `owasp` fields in SARIF. Verify on installed Semgrep ≥ 1.161 before relying on multi-CWE rules in SARIF uploads.
11. **Exact current Semgrep stable version pin** — research surfaced 1.161.0 (2026-04-22); pin via PyPI / GitHub releases at execute time.
12. **`pytorch/pytorch` self-hosted runner takeover (Oct 2024) and CVE-2025-61671** — cited at medium confidence; verify exact CVE IDs / advisory URLs before quoting in design docs.
