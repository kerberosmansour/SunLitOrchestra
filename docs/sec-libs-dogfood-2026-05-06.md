---
title: "slo-sec-libs dogfood 2026-05-06"
date: 2026-05-06
target_runbook: "docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md"
target_milestone: "M3"
data_classification: "Public"
filed_issue_status: "deferred-pending-confirmation"
---

# /slo-sec-libs Dogfood - 2026-05-06

## Summary

M5 dogfooded the M1-M4 `/slo-sec-libs` pipeline against a completed SLO milestone:
`docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` M3,
the `/slo-critique` security persona rewrite.

Result:

- matched: 3 recommendations from real CycloneDX 1.6 declaration catalogs.
- unmatched: 2 capability gaps where the current catalogs are close but not exact.
- filed: 0 live issues; 2 filing candidates are deferred pending explicit per-issue confirmation.

No `gh issue create` command was run. M3 and M4 require explicit per-issue confirmation before any live filing to `kerberosmansour/slo-security-intake`,
`kerberosmansour/hulumi`, or `kerberosmansour/SunLitSecurityLibraries`.

## Candidate Shortlist

| Candidate | Evidence | Selected | Reason |
|---|---|---:|---|
| `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` M3 | `docs/slo/design/slo-security-embedding-overview.md` has `security_libs_required: true`; M3 rewrites the security persona around class elimination, threat-model citations, variant analysis, and SunLitSecurityLibraries citations. | yes | Richest security-library surface and explicitly recommended by the M5 runbook. |
| `docs/slo/completed/RUNBOOK-SAST-RULEGEN-A.md` M3 | M3 has strong proactive controls, but its Contract Block says no security library is needed and marks `security_libs_required: false`. | no | Good security surface, but not a library-recommendation target. |
| `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md` M3 | `docs/slo/design/scanner-orchestration-overview.md` has `security_libs_required: false`; M3 centers workflow hardening and fixed-template emission. | no | Useful control rows, but the overview explicitly opts out of security-library matching. |

## Inputs

| Input | Value |
|---|---|
| Target runbook | `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` |
| Target milestone | M3, `/slo-critique` security persona rewrite |
| Target overview | `docs/slo/design/slo-security-embedding-overview.md` |
| Target threat model | `docs/slo/design/slo-security-embedding-threat-model.md` |
| CycloneDX schema | `https://cyclonedx.org/schema/bom-1.6.schema.json` |
| CycloneDX schema SHA-256 | `1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f` |
| Hulumi declaration source | `kerberosmansour/hulumi@c29d75d4903838c51d35497d3e9bb78d8161c3b9` |
| Hulumi declaration file | `declarations/cyclonedx-1.6-capabilities.json` |
| Hulumi declaration SHA-256 | `c8ef1b206ff8d06fcdc373118b69084056c18b40ad8feef4b1a334b0ae857e5b` |
| SunLitSecurityLibraries declaration source | `kerberosmansour/SunLitSecurityLibraries@ac3b4ccc641cbe4f12107196de9237a1e5503ab5` |
| SunLitSecurityLibraries declaration file | `declarations/cyclonedx-1.6-capabilities.json` |
| SunLitSecurityLibraries declaration SHA-256 | `0de8f573392692e0c18c7c5462e154fff2578e8489cb4f6c8ebfb0505f12bdb9` |

## M1 Reader Result

The M1 reader was run against both public declaration files with the pinned official CycloneDX 1.6 schema.

| Source | Result | Catalog evidence |
|---|---|---|
| Hulumi | pass | 4 components, 2 claims, schema validation `strict-jsonschema` |
| SunLitSecurityLibraries | pass | 11 components, 11 claims, schema validation `strict-jsonschema` |

Safety observation: the first local smoke run used `/tmp/...` paths and the reader refused them because `/tmp` is a symlink on macOS. Rerunning through `/private/tmp/slo-sec-libs-m5/...` passed. That preserves the M1 no-symlink-path contract.

## Target Rows

The selected M3 predates the M2 Contract Block row expansion, so it has no literal `Proactive controls in play` row. This dogfood extracted control-shaped needs from the M3 goal, M3 BDD, and the already-shipped threat-model row `tm-slo-sec-abuse-3`.

| row_id | Source | Needed control |
|---|---|---|
| `m3-req-class-schema` | M3 goal and finding-acceptance gate | Canonical security finding schema: named bug class, threat-model row citation, elimination or mitigation answer, no maybe findings. |
| `m3-req-variant-analysis` | M3 goal and BDD | Variant-analysis directive with concrete search locations and no generic OWASP boilerplate. |
| `m3-abuse-prompt-boundary` | `tm-slo-sec-abuse-3` | Runbook-embedded prompt injection must not suppress security findings. |
| `m3-audit-finding-trail` | Threat-model compliance mapping and finding output | Findings should preserve traceable context without leaking untrusted prose into prompts or issue bodies. |

## Matched

matched:

| row_id | catalog_bom_ref | component | capability evidence | recommendation | confidence |
|---|---|---|---|---|---|
| `m3-req-class-schema` | `component:security_core` | SunLitSecurityLibraries `security_core` | Controls `C1`; capabilities `shared-security-types`, `identity-source-trait`, `correlation-context`; claim `claim:security_core:capabilities`. | Reuse `security_core` as the advertised shared vocabulary anchor if the critique finding schema graduates from Markdown prose to typed records. | medium |
| `m3-abuse-prompt-boundary` | `component:secure_boundary` | SunLitSecurityLibraries `secure_boundary` | Controls `C5,C8`; capabilities `input-validation`, `request-size-depth-limits`, `security-headers`; claim `claim:secure_boundary:capabilities`. | Treat untrusted runbook and threat-model text as boundary input in any future runtime verifier; apply size/depth validation before extracting rows. | medium-low |
| `m3-audit-finding-trail` | `component:security_events` | SunLitSecurityLibraries `security_events` | Control `C9`; capabilities `security-telemetry`, `hmac-sealed-events`, `classification-redaction`; claim `claim:security_events:capabilities`. | Use `security_events` if SLO later records machine-readable critique/filer events beyond the current Markdown report. | medium |

No Hulumi component was selected for this target. Hulumi advertises cloud/IaC and repository-hardening capabilities (`component:@hulumi/baseline`, `component:@hulumi/policies`, `component:@hulumi/drift`, `component:@hulumi/k8s-baseline`), but M3 is a Markdown security-persona milestone with no cloud resource, Kubernetes, AWS, or GitHub workflow surface.

## Unmatched

unmatched:

| gap_id | row_id | desired_capability | closest catalog evidence | Why unmatched | filing_status |
|---|---|---|---|---|---|
| `gap-agent-prompt-boundary` | `m3-abuse-prompt-boundary` | `agent-prompt-injection-boundary` | `component:secure_boundary` has generic input validation and size/depth limits. | The declaration does not advertise agent-prompt or Markdown instruction-boundary handling. This is the exact class in `tm-slo-sec-abuse-3`. | `deferred-pending-confirmation` |
| `gap-variant-analysis-schema` | `m3-req-variant-analysis` | `variant-analysis-result-schema` | `component:security_core` has shared types and correlation context. | The declaration does not advertise a structured variant-analysis result contract with search locations, coverage notes, and small-codebase N/A handling. | `deferred-pending-confirmation` |

## Filed

filed:

| candidate | destination | issue_url | status | reason |
|---|---|---|---|---|
| `gap-agent-prompt-boundary` | `kerberosmansour/slo-security-intake` by default; likely owner label `SunLitSecurityLibraries` | `N/A - deferred-pending-confirmation` | `deferred-pending-confirmation` | Live filing requires explicit per-issue confirmation. |
| `gap-variant-analysis-schema` | `kerberosmansour/slo-security-intake` by default; likely owner label `SunLitSecurityLibraries` | `N/A - deferred-pending-confirmation` | `deferred-pending-confirmation` | Live filing requires explicit per-issue confirmation. |

Proposed filing titles if the user confirms them later:

- `sec-libs gap: agent prompt injection boundary for Markdown skill inputs`
- `sec-libs gap: structured variant-analysis result schema`

## M2 Matcher Result

The matcher stayed catalog-grounded:

- Every matched recommendation cites an observed `catalog_bom_ref`.
- Generic control labels were treated as candidates, not sufficient proof by themselves.
- No capability was invented from model memory.
- Close-but-not-exact cases were downgraded to `unmatched`.

## M3/M4 Filing Discipline Check

- No live `gh issue create` command was run.
- Default destination remains `kerberosmansour/slo-security-intake`.
- Direct upstream filing to `kerberosmansour/SunLitSecurityLibraries` remains behind `--file-upstream` and a separate confirmation.
- Legacy secure-libraries owner spelling was not used; the canonical repository is `kerberosmansour/SunLitSecurityLibraries`.

## Outcome

M5 validates the read and match path against real public declarations and a completed SLO milestone. It also exposes a useful contract mismatch: M5 originally expected filed issue URLs, but M3/M4 correctly require explicit per-issue confirmation before live filing. This report records deferred filing candidates rather than fabricating issue URLs or filing without consent.
