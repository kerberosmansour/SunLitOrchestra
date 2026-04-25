# Sources — scanner-orchestration

All URLs accessed 2026-04-26 unless otherwise noted. Grouped by research question.

## Q1 — Semgrep CI integration

- [Add Semgrep to CI/CD](https://semgrep.dev/docs/deployment/add-semgrep-to-ci) — canonical recipe.
- [CI environment variables](https://semgrep.dev/docs/semgrep-ci/ci-environment-variables) — `SEMGREP_BASELINE_REF`, `SEMGREP_BASELINE_COMMIT`, `SEMGREP_GHA_MIN_FETCH_DEPTH`, `SEMGREP_RULES`.
- [CI configuration reference](https://semgrep.dev/docs/semgrep-ci/configuration-reference/) — flag surface; `--config` "not supported in ci mode" caveat.
- [Sample CI configurations](https://semgrep.dev/docs/semgrep-ci/sample-ci-configs).
- [Findings in CI](https://semgrep.dev/docs/semgrep-ci/findings-ci) — blocking-finding semantics.
- [Configure blocking findings](https://semgrep.dev/docs/semgrep-ci/configuring-blocking-and-errors-in-ci) — verbatim "all findings block in stand-alone setup."
- [Semgrep CE in CI](https://semgrep.dev/docs/deployment/oss-deployment).
- [Semgrep CLI reference](https://semgrep.dev/docs/cli-reference).
- [Semgrep in CI — KB](https://semgrep.dev/docs/kb/semgrep-ci).
- [Semgrep in CI — overview](https://semgrep.dev/docs/semgrep-ci/overview/).
- [Failed to run a git command during a pull request or merge request scan](https://semgrep.dev/docs/kb/semgrep-ci/git-command-errors) — `fetch-depth: 1` pitfall.
- [Continuous integration | Trail of Bits Testing Handbook](https://appsec.guide/docs/static-analysis/semgrep/continuous-integration/).
- [How to Set Up Semgrep in 2026 (DEV community)](https://dev.to/rahulxsingh/how-to-set-up-semgrep-in-2026-complete-installation-and-configuration-guide-5emm).
- [DevOps Daily — Semgrep SAST guide](https://devops-daily.com/guides/sast-tools/03-semgrep).
- [How To Enable Code Scanning With Semgrep — 0xdbe](https://0xdbe.github.io/GitHub-HowToEnableCodeScanningWithSemgrep/) — OSS-only PR-feedback path via SARIF + Code Scanning.
- [Sample Semgrep CI (j3ssie)](https://github.com/j3ssie/sample-semgrep-ci).

## Q2 — Semgrep registry CWE metadata

- [Contribute rules to the Semgrep Registry](https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository) — mandatory `category: security` metadata fields.
- [Rule structure syntax](https://semgrep.dev/docs/writing-rules/rule-syntax).
- [Semgrep Registry — Tags](https://registry.semgrep.dev/tag).
- [`p/cwe-top-25` ruleset](https://semgrep.dev/p/cwe-top-25).
- [`p/owasp-top-ten` ruleset](https://semgrep.dev/p/owasp-top-ten).
- [`p/github-actions` ruleset](https://semgrep.dev/p/github-actions).
- [returntocorp/semgrep-rules — `metadata-cwe.yaml`](https://github.com/returntocorp/semgrep-rules/blob/develop/yaml/semgrep/metadata-cwe.yaml) — registry-side CWE-tag linter.
- [Semgrep Release Notes (index)](https://semgrep.dev/docs/release-notes).
- [Semgrep Release Notes — February 2025](https://semgrep.dev/docs/release-notes/february-2025).
- [Semgrep Release Notes — December 2025](https://semgrep.dev/docs/release-notes/december-2025).
- [Semgrep Release Notes — January 2026](https://semgrep.dev/docs/release-notes/january-2026).
- [Semgrep Release Notes — February 2026](https://semgrep.dev/docs/release-notes/february-2026).
- [Semgrep Release Notes — March 2026](https://semgrep.dev/docs/release-notes/march-2026).
- [OWASP Top 10 2025: What's New | Semgrep](https://semgrep.dev/blog/2026/owasp-top-10-2025-whats-new/).
- [Semgrep — Imagine zero false positive SAST (2025)](https://semgrep.dev/blog/2025/making-zero-false-positive-sast-a-reality-with-ai-powered-memory/).
- [semgrep on PyPI](https://pypi.org/project/semgrep/).
- [semgrep/semgrep on GitHub](https://github.com/semgrep/semgrep).
- [semgrep/semgrep releases (GitHub)](https://github.com/semgrep/semgrep/releases).
- [semgrep/semgrep-rules (GitHub)](https://github.com/semgrep/semgrep-rules).
- [semgrep/semgrep issue #4673](https://github.com/semgrep/semgrep/issues/4673) — historical SARIF / array-CWE bug.

## Q3 — Compliance / audit (PCI DSS, SOC 2, NIST SSDF)

- [PCI Security Standards Council — Document Library](https://www.pcisecuritystandards.org/document_library/) — primary source; testing procedure verbatim text gated.
- [VISTA InfoSec — PCI DSS Requirement 6 changes from v3.2.1 to v4.0](https://vistainfosec.com/blog/pci-dss-requirement-6-changes-from-v3-2-1-to-v4-0-explained/) — v3.2.1 → v4.0 numbering shift.
- [GuidePoint — PCI DSS 4.0 future-dated requirements](https://www.guidepointsecurity.com/blog/pci-dss-4-0-major-future-dated-requirements/).
- [Linford & Co — PCI DSS 4.0 Mandatory Requirements 2025 Guide](https://linfordco.com/blog/pci-dss-4-0-requirements-guide/).
- [Halock — PCI DSS v4.0.1 software catalog mandate](https://www.halock.com/what-is-the-new-pci-dss-v4-0-1-software-catalog-mandate/) — clarifies v4.0.1 6.3.2 = SBOM mandate.
- [Cybeats — PCI DSS 4.0 SBOMs 2025 readiness](https://www.cybeats.com/blog/pci-dss-4-0-sboms-a-2025-readiness-guide).
- [KirkpatrickPrice — PCI Requirement 6.3.2 review of custom code prior to release (video)](https://kirkpatrickprice.com/video/pci-requirement-6-3-2-review-custom-code-prior-release/) — uses v3.2.1 numbering.
- [Pen Test Partners — PCI DSS v4.0 evidence and documentation requirements checklist](https://www.pentestpartners.com/security-blog/pci-dss-v4-0-evidence-and-documentation-requirements-checklist/) — accepts "code review records: evidence of peer reviews or automated scans of application code."
- [securityreview.ai — Is your code or your pipeline the bigger PCI DSS 4.0 risk?](https://www.securityreview.ai/blog/is-your-code-or-your-pipeline-the-bigger-pci-dss-4-0-risk).
- [Strike Graph — PCI DSS vs SOC 2](https://www.strikegraph.com/blog/pci-dss-vs-soc-2).
- [SOC 2 Audit Checklist (2026)](https://soc2auditors.org/insights/soc-2-audit-checklist/).
- [Dsalta — SOC 2 compliance in 2025](https://www.dsalta.com/resources/articles/soc-2-compliance-in-2025-requirements-readiness-and-audit-success).
- [NIST SP 800-218 (CSRC)](https://csrc.nist.gov/pubs/sp/800/218/final).
- [NIST SP 800-218 SSDF table (xlsx)](https://csrc.nist.gov/files/pubs/sp/800/218/final/docs/nist.sp.800-218.ssdf-table.xlsx) — alternate fetch path for verbatim PW.7 / PW.8 tasks.
- [Aikido — NIST SSDF explained](https://www.aikido.dev/learn/compliance/compliance-frameworks/nist-ssdf).
- [OWASP Top 10 — 2025 edition](https://owasp.org/Top10/2025/).

## Q4 — `pull_request_target` security posture

- [GitHub Changelog — Actions pull_request_target and environment branch protections changes (2025-11-07)](https://github.blog/changelog/2025-11-07-actions-pull_request_target-and-environment-branch-protections-changes/) — 2025-12-08 default-branch sourcing change.
- [GitHub Changelog — Actions policy now supports blocking and SHA-pinning actions (2025-08-15)](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/).
- [GitHub Actions 2026 security roadmap](https://github.blog/news-insights/product-news/whats-coming-to-our-github-actions-2026-security-roadmap/).
- [Complete Guide to GitHub Actions 2026 Security Roadmap (dev.to)](https://dev.to/x4nent/complete-guide-to-github-actions-2026-security-roadmap-dependency-locking-native-egress-5aap).
- [GitHub Docs — Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use).
- [GitHub Security Lab — Preventing pwn requests (Part 1)](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/).
- [GitHub Security Lab — New vulnerability patterns and mitigation strategies (Part 4)](https://securitylab.github.com/resources/github-actions-new-patterns-and-mitigations/).
- [Wiz — Hardening GitHub Actions: Lessons from Recent Attacks](https://www.wiz.io/blog/github-actions-security-guide).
- [Sysdig — Insecure GitHub Actions in MITRE, Splunk, and other OSS repos](https://www.sysdig.com/blog/insecure-github-actions-found-in-mitre-splunk-and-other-open-source-repositories).
- [Orca Security — GitHub Actions security risks](https://orca.security/resources/blog/github-actions-security-risks/).
- [Orca Security — GitHub Actions Hardening](https://orca.security/resources/blog/github-actions-hardening/).
- [Orca Security — pull_request_nightmare Part 2 exploits](https://orca.security/resources/blog/pull-request-nightmare-part-2-exploits/).
- [AquilaX — GitHub Actions Security Hardening](https://aquilax.ai/blog/github-actions-security-hardening).
- [Arctiq — Top 10 GitHub Actions Security Pitfalls](https://arctiq.com/blog/top-10-github-actions-security-pitfalls-the-ultimate-guide-to-bulletproof-workflows).
- [GitGuardian — GitHub Actions Security Best Practices cheat sheet](https://blog.gitguardian.com/github-actions-security-cheat-sheet/).
- [Towards a secure-by-default GitHub Actions (community discussion #179107)](https://github.com/orgs/community/discussions/179107).
- [Improve Actions security (community discussion #157949)](https://github.com/orgs/community/discussions/157949).
- [GHSA-mrrh-fwg8-r2c3 — tj-actions/changed-files compromise (CVE-2025-30066)](https://github.com/advisories/ghsa-mrrh-fwg8-r2c3).

## Q5 — Threat-model-driven prior art

- [iriusrisk/OpenThreatModel — OTM specification repo](https://github.com/iriusrisk/OpenThreatModel).
- [iriusrisk/OpenThreatModel — `otm_schema.json`](https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json).
- [Open Threat Model (OTM) — StartLeft documentation](https://iriusrisk.github.io/startleft/Open-Threat-Model-(OTM)/).
- [OWASP/threat-dragon — TMF format wiki](https://github.com/OWASP/threat-dragon/wiki/Threat-Model-File-(TMF)-format).
- [OWASP Threat Dragon project page](https://owasp.org/www-project-threat-dragon/).
- [OWASP/threat-dragon (GitHub)](https://github.com/OWASP/threat-dragon).
- [OWASP/pytm — Pythonic threat-modeling framework](https://github.com/OWASP/pytm).
- [OWASP pytm project page](https://owasp.org/www-project-pytm/).
- [OWASP Developer Guide — pytm chapter](https://devguide.owasp.org/en/04-design/01-threat-modeling/02-pytm/).
- [OWASP Threat Model Library (TM-BOM successor)](https://owasp.org/www-project-threat-model-library/).
- [OWASP Ontology Driven Threat Modeling Framework](https://owasp.org/www-project-ontology-driven-threat-modeling-framework/).
- [OWASP Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html).
- [SecOpsTM — STRIDE-as-code framework](https://github.com/ellipse2v/SecOpsTM) — closest adjacent player.
- [A Dragon and Python walk into an OWASP card game (DragPyT)](https://threatmodeling.dev/dragpyt/).
- [Threat Modeling Handbook #5 — AppSec Untangled](https://medium.com/appsec-untangled/threat-modeling-handbook-5-convert-your-threat-model-into-an-automated-pentest-using-devsecops-84efcd138202) — manual-practice precedent.
- [Threat Modeling, Ch. 4 "Automated Threat Modeling" (O'Reilly)](https://www.oreilly.com/library/view/threat-modeling/9781492056546/ch04.html).
- [Autogrep — Automated Generation and Filtering of Semgrep Rules from Vulnerability Patches](https://lambdasec.github.io/AutoGrep-Automated-Generation-and-Filtering-of-Semgrep-Rules-from-Vulnerability-Patches/).
- [Phoenix Security — Semgrep ASPM integration](https://phoenix.security/phoenix-security-integration-semgrep/).
- [Semgrep* paper (ACM DL, EASE 2024)](https://dl.acm.org/doi/10.1145/3661167.3661262).
- [Semgrep* paper (ACM DL fullHtml)](https://dl.acm.org/doi/fullHtml/10.1145/3661167.3661262).
- [RealVuln benchmarking paper (arXiv)](https://arxiv.org/html/2604.13764).
- [GitLab — Customize SAST rulesets](https://docs.gitlab.com/user/application_security/sast/customize_rulesets/).
- [sast-semgrep agent skill (claude-plugins.dev)](https://claude-plugins.dev/skills/@AgentSecOps/SecOpsAgentKit/sast-semgrep).
- [Lobehub — claude-build-workflow-sast-semgrep](https://lobehub.com/skills/rohunj-claude-build-workflow-sast-semgrep).
- [IBM/mcp-context-forge issue #259 — Semgrep+ZAP Makefile/Actions targets](https://github.com/IBM/mcp-context-forge/issues/259).
