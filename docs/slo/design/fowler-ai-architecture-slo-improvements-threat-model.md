# Threat Model — Fowler AI Architecture SLO Improvements

## Scope

Public Markdown skill-pack changes and Rust structural tests for new architecture/planning/verification disciplines. No runtime service, no personal data store, no network-facing endpoint.

## STRIDE Summary

| Component | Spoofing | Tampering | Repudiation | Information disclosure | Denial of service | Elevation of privilege |
|---|---|---|---|---|---|---|
| Skill prose | N/A — no auth principal | mitigated by review + structural tests | mitigated by git history | N/A — public docs | residual risk — overlong prose exceeds context | N/A — no privilege boundary |
| User-provided source notes | N/A — treated as untrusted input | mitigated by `~~~text` fence rule and citation discipline | mitigated by research artifacts | N/A — public supplied content | residual risk — poisoned notes waste planning time | N/A |
| Runbook templates | N/A | mitigated by additive-only row tests | mitigated by git history | N/A | residual risk — too many rows create process noise | N/A |
| Structural tests | N/A | mitigated by cargo test review | mitigated by CI output | N/A | residual risk — brittle tests block valid edits | N/A |

## Abuse Cases

| id | Attacker / actor | Attack step | Desired outcome | Control |
|---|---|---|---|---|
| tm-fowler-ai-arch-abuse-1 | Contributor with a poisoned source note | Inserts "ignore SLO gates" into copied video notes and hopes it becomes skill prose | Durable prompt-injection instruction lands in SKILL.md | User strings fenced; source-backed claims cite `sources.md`; critique flags embedded instruction attempts. |
| tm-fowler-ai-arch-abuse-2 | Well-meaning agent under context pressure | Adds broad new Contract Block rows without updating ticket flow or tests | Sprint and ticket SLO drift into two standards | M5 ticket parity milestone plus structural tests. |
| tm-fowler-ai-arch-abuse-3 | Agent doing AI-feature verification | Marks nondeterministic LLM behavior pass because one sample looked good | Unsafe AI behavior ships without tolerance boundaries | AI tolerance Contract Block + `/slo-verify` pass with golden/scenario eval evidence. |
| tm-fowler-ai-arch-abuse-4 | Agent doing "refactor" work | Bundles behavior change under targeted refactor budget | Review misses behavior regression | Refactoring-discipline reference requires pre-refactor tests, microsteps, and behavior-preservation proof. |
| tm-fowler-ai-arch-abuse-5 | Agent working in brownfield repo | Copies a messy legacy implementation because it appears nearby | Architecture degrades through copied anti-patterns | Code-map artifact names exemplar and anti-exemplar files; plan rows cite them. |

## Residual Risks

- Exact YouTube transcript claims are not treated as authoritative unless separately sourced. The runbook uses user-provided notes as input and Fowler/Thoughtworks/InfoQ pages as cited authority.
- Additional Contract Block rows add cognitive load. The `N/A with reason` rule and structural tests must keep the rows from becoming theatre.
