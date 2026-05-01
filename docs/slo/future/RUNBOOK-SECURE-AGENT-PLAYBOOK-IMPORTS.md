# Secure Agent Playbook Imports - SunLitOrchestrate (future runbook)

> **Purpose**: Capture reusable project-level ideas from OWASP Secure Agent Playbook and adapt them to SunLitOrchestrate without turning SLO into a generic security-assessment playbook.
> **Status**: not_started. The small shared security-reporting template layer has landed in `references/security/`; the remaining items below need normal `/slo-ideate -> /slo-plan` treatment before implementation.
> **Source reviewed**: `/Users/sherifmansour/Dev/GitHub/secure-agent-playbook` project layout, agents, skills, plays, templates, examples, plugin metadata, and release workflow.
> **Design write-up**: [docs/slo/design/secure-agent-playbook-imports-overview.md](../design/secure-agent-playbook-imports-overview.md).

## What is worth borrowing

| Idea from Secure Agent Playbook | Fit for SunLitOrchestrate | Proposed SLO adaptation |
|---|---|---|
| Standard finding and report templates | High | Landed as `references/security/security-finding-template.md` and `references/security/security-assessment-summary-template.md`; `/slo-critique` and `/slo-verify` now point to them for expanded security findings. |
| Thin skill wrappers over deeper play/procedure files | High | Continue the existing `docs/slo/future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md` plan to split long skills (`/slo-sast`, `/slo-tla`, `/slo-plan`) into lean entrypoints plus `references/` methodology files. |
| Example outputs | High | Add `examples/` with synthetic but realistic examples: a completed runbook excerpt, critique report, verification report, SAST manifest, and biz-public artifact. Keep examples anonymized and clearly non-normative. |
| Dedicated agent roles and team lead | Medium | SLO already models CEO / eng / security / design personas inside `/slo-critique`. Add true host-native `agents/` only after host support and install semantics are explicit; otherwise keep persona orchestration inside skills. |
| Standards traceability via CWE / OWASP / ASVS / OpenCRE | Medium-high | Use in security finding templates and `/slo-sast` reports. Do not require OpenCRE lookup for every runbook row until source freshness and offline behavior are designed. |
| Plugin marketplace metadata and release zip | Medium | Consider `.claude-plugin/plugin.json` + release asset packaging as an optional distribution path, but do not replace `sldo-install` or weaken the current multi-host installer story. Any workflow must obey SLO's SHA-pinning discipline. |
| Large embedded standards data under `data/` | Low-medium | Avoid vendoring large data until a specific skill consumes it. Prefer small curated references and retrieval-date stamped docs, matching current biz/SAST discipline. |

## Milestone sketch

| # | Milestone | Status | Scope |
|---|---|---|---|
| 1 | Example output gallery | `not_started` | Add `examples/README.md` plus 3-5 synthetic examples that show what good SLO artifacts look like. |
| 2 | Skill decomposition continuation | `not_started` | Continue the engineering-skill-improvements runbook: thin long SKILL.md files, move procedure detail to references. |
| 3 | Standards traceability pass | `not_started` | Decide which SLO security outputs require CWE / OWASP / ASVS / OpenCRE fields and which stay lightweight. |
| 4 | Optional plugin packaging assessment | `not_started` | Evaluate `.claude-plugin/plugin.json` and release zip generation without undermining GitHub Copilot support or the Rust installer. |
| 5 | Agent-role experiment | `not_started` | Prototype host-native agents only if current host capabilities support installation, scoping, and reviewable outputs cleanly. |

## Constraints

- Do not copy Secure Agent Playbook content wholesale. Borrow structure and presentation patterns, then re-author SLO-specific artifacts.
- Keep SLO's primary identity: runbook-driven software delivery. Security assessment remains a woven-in discipline, not the whole product.
- Any GitHub Actions workflow added for packaging must pin third-party actions by SHA, matching `SECURITY.md` and SAST workflow discipline.
- Examples must be synthetic and must not include personal data, real secrets, or confidential `docs/biz/` content.
