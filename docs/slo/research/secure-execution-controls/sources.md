---
name: secure-execution-controls-sources
researched: 2026-05-17
---

# Sources — secure execution controls

## Repo-local authority files

| Source | Claim supported |
|---|---|
| `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` | Existing SLO security embedding covers ideate / architect / plan / critique / verify, but not a secure-construction gate inside `/slo-execute`. |
| `skills/slo-execute/SKILL.md` | Current `/slo-execute` loop is BDD-first + allow-list + evidence-log disciplined. |
| `skills/slo-plan/references/proactive-controls-vocabulary.md` | Current stack-aware control vocabulary maps Rust/axum to SunLitSecurityLibraries and Pulumi/AWS to Hulumi. |
| `skills/slo-sec-libs/SKILL.md` | Current `/slo-sec-libs` reads CycloneDX 1.6 declarations, matches controls, and files capability gaps. |
| `skills/slo-sast/SKILL.md` | Current `/slo-sast` wires threat-model-driven Semgrep and SARIF output. |
| `skills/slo-dast-tuner/SKILL.md` | Current `/slo-dast-tuner` routes DAST through zaprun only and treats auth failure as coverage failure. |
| `skills/slo-cloud-threat-model/SKILL.md` | Current `/slo-cloud-threat-model` covers AWS / GitHub / Cloudflare scenarios and Hulumi components by ID. |
| `docs/LOOPS-ENGINEERING.md` | Current loop vocabulary includes security-tuning and library-feedback loops. |

## External sources

| Source | URL | Retrieved | Tier | Claim supported |
|---|---|---:|---:|---|
| CISA Secure by Design | https://www.cisa.gov/resources-tools/resources/secure-by-design | 2026-05-17 | 3 | Security should be designed into products early, not shifted to customers or late-stage checks. |
| NIST SP 800-218 SSDF | https://csrc.nist.gov/pubs/sp/800/218/final | 2026-05-17 | 3 | Secure software development should include practices across the SDLC. |
| OWASP ASVS | https://owasp.org/www-project-application-security-verification-standard/ | 2026-05-17 | 3 | ASVS provides technical security requirements and verification basis for web applications and services. |
| OWASP Proactive Controls 2024 | https://top10proactive.owasp.org/the-top-10/ | 2026-05-17 | 3 | Proactive Controls are developer-facing defensive categories to include in projects. |
| OWASP Input Validation Cheat Sheet | https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html | 2026-05-17 | 3 | Input validation should use robust allow-listing and must not be treated as the only defense for output-context issues. |
| CycloneDX 1.6 JSON spec | https://cyclonedx.org/docs/1.6/json/ | 2026-05-17 | 1 | CycloneDX 1.6 is the spec surface used for declarations. |
| Pulumi Policy as Code / CrossGuard | https://www.pulumi.com/docs/iac/using-pulumi/crossguard/ | 2026-05-17 | 1 | Pulumi policy packs can enforce guardrails for infrastructure. |
| Pulumi unit testing | https://www.pulumi.com/docs/iac/guides/testing/unit/ | 2026-05-17 | 1 | Pulumi programs can be tested with mocks/unit tests, including TypeScript examples. |
| GitHub Actions hardening | https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions | 2026-05-17 | 1 | GitHub recommends least privilege, careful secret handling, and pinning third-party actions to full-length commit SHAs. |
| Semgrep CLI docs | https://semgrep.dev/docs/getting-started/cli | 2026-05-17 | 1 | Semgrep CLI supports SARIF output. |
| ZAP Automation Framework authentication | https://www.zaproxy.org/docs/desktop/addons/automation-framework/authentication/ | 2026-05-17 | 1 | ZAP Automation Framework supports authentication modes, including browser-based and client-script auth through add-ons. |
| ZAP scan policy | https://www.zaproxy.org/docs/desktop/start/features/scanpolicy/ | 2026-05-17 | 1 | Active scan policies define which rules run and how they run; active scans are scoped and do not find all logical vulnerabilities. |
