---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M1
created: 2026-05-04
status: in-progress
---

# Lessons - eng-imp M1

## Source-verification spike

M1 validates the `references/templates/citation-discipline.md` source hierarchy before M2-M4 perform full per-claim verification during decomposition. Spike result: the discipline is usable, but several claims need precise source selection and should remain human review visible when downstream milestones move prose.

| Claim area | Skill | Representative claim checked | Source tier | Source | Spike result | Human review note |
|---|---|---|---|---|---|---|
| Semgrep rules in CI | `/slo-sast` | `SEMGREP_RULES` can control Semgrep CE rules in CI | 1 | https://semgrep.dev/docs/deployment/oss-deployment and https://semgrep.dev/docs/semgrep-ci/ci-environment-variables | Verified enough for M1 template discipline | Human review should confirm exact wording when M2 moves `/slo-sast` prose |
| GitHub Actions event risk | `/slo-sast` | Avoid `pull_request_target` for untrusted PR analysis workflows | 1 | https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#pull_request_target | Verified enough for M1 template discipline | Human review should keep the ban phrased as workflow-safety discipline, not a blanket GitHub feature ban |
| `cargo audit` advisory purpose | `/slo-verify` | `cargo audit` audits dependencies against RustSec advisories | 2 | https://github.com/rustsec/rustsec/blob/main/cargo-audit/README.md and https://rustsec.org/ | Verified enough for M1 template discipline | Human review should verify exit-code wording before any command-reference rewrite |
| ZAP API scanning | `/slo-verify` | ZAP API scan needs an API definition / reachable service shape | 1 | https://www.zaproxy.org/docs/docker/api-scan/ | Verified enough for M1 template discipline | Human review should preserve the docs-only/library-only DAST N/A rule |
| Dastardly / Burp DAST CI scans | `/slo-verify` | DAST-style scans run against deployed/reachable web targets in CI | 1 | https://portswigger.net/burp/documentation/dastardly and https://portswigger.net/burp/documentation/dast/user-guide/ci-cd/ci-driven-scans | Verified enough for M1 template discipline | Human review should avoid claiming Dastardly covers non-web or markdown-only targets |

## Rules For The Next Milestone

- In M2, move `/slo-sast` prose into methodology files without weakening any source-backed security rule.
- Programmatically preserve `MUST` and `MUST NOT` lines where possible; do not rely only on manual grep.
- Keep `references/templates/citation-discipline.md` as the source hierarchy authority. Do not duplicate the six-tier list inside methodology files.
- Treat `/slo-sast` stack-detection, cache-integrity, Semgrep registry, workflow safety, and manifest claims as source-verification work items.

## Allow-List Note

The M1 Contract Block explicitly allowed template files, five SKILL.md files, and the structural test. The runbook Definition of Done also requires tracker, lessons, and completion artifacts. This lessons file is recorded as standard `/slo-execute` milestone evidence rather than product-surface work. README / ARCHITECTURE post-flight edits remain a separate allow-list tension to resolve before milestone close.

## Follow-Ups

- Consider a later cleanup ticket for package-level `cargo fmt --check -p sldo-install` and clippy drift if it still appears during this runbook.
