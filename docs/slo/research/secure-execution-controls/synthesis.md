---
name: secure-execution-controls
researched: 2026-05-17
incomplete: false
---

# Synthesis — secure execution controls

Companion source register: [`sources.md`](sources.md).

## Research basis

Repo-local basis:

- `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` already made `/slo-ideate`, `/slo-architect`, `/slo-plan`, `/slo-critique`, and `/slo-verify` security-aware.
- `skills/slo-plan/references/proactive-controls-vocabulary.md` already names SunLitSecurityLibraries for Rust/axum and Hulumi for Pulumi/AWS proactive-control rows.
- `skills/slo-sec-libs/SKILL.md` already reads CycloneDX 1.6 declarations from Hulumi and SunLitSecurityLibraries, matches proactive controls to advertised capabilities, and files confirmed capability gaps.
- `skills/slo-dast-tuner/SKILL.md` already routes DAST through `zaprun` and treats SAST SARIF as evidence, not proof.
- `docs/LOOPS-ENGINEERING.md` already names the security-tuning loop and library-feedback loop.

External basis, checked 2026-05-17:

- CISA Secure by Design: https://www.cisa.gov/resources-tools/resources/secure-by-design
- NIST SSDF SP 800-218: https://csrc.nist.gov/pubs/sp/800/218/final
- OWASP ASVS: https://owasp.org/www-project-application-security-verification-standard/
- OWASP Proactive Controls 2024: https://top10proactive.owasp.org/the-top-10/
- OWASP Input Validation Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html
- CycloneDX 1.6 JSON schema/spec: https://cyclonedx.org/docs/1.6/json/
- Pulumi Policy as Code / CrossGuard: https://www.pulumi.com/docs/iac/using-pulumi/crossguard/
- Pulumi unit testing: https://www.pulumi.com/docs/iac/guides/testing/unit/
- GitHub Actions hardening: https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions
- Semgrep CLI SARIF output: https://semgrep.dev/docs/getting-started/cli
- ZAP Automation Framework authentication: https://www.zaproxy.org/docs/desktop/addons/automation-framework/authentication/
- ZAP scan policy: https://www.zaproxy.org/docs/desktop/start/features/scanpolicy/

## What the design must handle

### 1. `/slo-execute` needs a secure-construction gate, not only secure verification

Phase 1 security embedding pushes security into ideation, architecture, planning, critique, and verification, but `/slo-execute` still mostly enforces BDD-first, allow-list, baseline tests, and evidence rows. That means an agent can still implement a milestone with ad hoc crypto, raw paths, hand-rolled auth, string-built SQL, unsafe shell arguments, or weak IaC and only discover the problem later in `/slo-verify` or `/slo-critique`. The design must handle secure construction inside `/slo-execute` because CISA Secure by Design and NIST SSDF both treat security as an early lifecycle responsibility, not a scanner-afterwards activity.

### 2. Proactive controls should become implementation constraints

`/slo-plan` already requires `Data classification`, `Proactive controls in play`, and `Abuse acceptance scenarios`. The missing step is making `/slo-execute` translate those rows into implementation choices before writing tests or code. The design must handle proactive-control rows as executable intent because OWASP ASVS provides concrete verification requirements, OWASP Proactive Controls names defensive categories developers should include, and the repo-local vocabulary already maps those categories to SunLitSecurityLibraries and Hulumi.

### 3. Library recommendations must come from declarations or pinned docs, not model memory

`/slo-sec-libs` already exists because model-memory recommendations drift as Hulumi and SunLitSecurityLibraries evolve. CycloneDX declarations are the local capability-advertising contract; `/slo-sec-libs` validates and matches against them. The design must handle library selection as catalog-grounded because CycloneDX 1.6 declarations are the repo's declared machine-readable source of truth and `/slo-sec-libs` explicitly forbids inventing capability claims.

### 4. Rust-first execution should prefer SunLitSecurityLibraries by class

For Sherif's dominant Rust path, `/slo-execute` should prefer secure abstractions from SunLitSecurityLibraries for boundary parsing, safe path/URL/shell/SQL types, output encoding, authn/authz, secret handling, error boundaries, and security events. It should not write raw alternatives unless the library has no advertised capability or the runbook explicitly records a justified residual risk. The design must handle Rust secure defaults this way because the repo-local proactive-controls vocabulary already names `secure_boundary`, `safe_types`, `secure_output`, `secure_identity`, `secure_authz`, `secure_data`, `secure_errors`, and `security_events` as the preferred Rust/axum controls.

### 5. Cloud and platform resources need a Hulumi-aware but not Hulumi-for-everyone rule

Sherif's cloud work is Pulumi TypeScript and often Hulumi. The public SLO skill pack should not force Hulumi onto every generic Pulumi user, but it should strongly prefer Hulumi when the target repo, runbook, or user explicitly says Hulumi, or when `security_libs_required: true` and the target stack is Sherif-owned Pulumi TypeScript. The design must handle this distinction because Pulumi's own docs support Policy as Code and unit testing as secure IaC practices, while Hulumi is the project-specific hardened component library for AWS, GitHub, and Cloudflare patterns.

### 6. Security test choice should be derived from threat model plus touched surface

The execution flow needs a selector, not a blanket "run all scanners" rule. New Rust boundary code needs unit/property/abuse tests plus SAST. New HTTP surfaces need route-aware SAST and, when a smoke service exists, DAST through `zaprun`. Auth-gated surfaces need authenticated DAST or an explicit coverage failure. New Pulumi/Hulumi resources need Pulumi unit tests, policy-pack checks, drift/preview evidence, and GitHub Actions hardening checks when workflows are touched. The design must handle a test-selection matrix because Semgrep emits SARIF, ZAP's Automation Framework supports authentication, and ZAP scan policies make clear that active scans are scoped tools that do not find all logical vulnerabilities.

### 7. Upstream capability gaps need a first-class branch, not a local hack

If SunLitSecurityLibraries or Hulumi lacks a needed secure primitive, the agent should not silently hand-roll the feature in the downstream app. The default should be: capture the gap, file SLO intake, optionally file upstream with confirmation, and either fix upstream first or record a time-boxed residual-risk workaround. The design must handle this branch because `/slo-sec-libs` already has structured capability-gap records, an upstream filing gate, and a 40-issues/hr session cap.

### 8. TypeScript and Java should start from controls, then grow catalogs

For non-Rust application code, the safe default is not to hallucinate "equivalent" libraries. `/slo-execute` should fall back to OWASP Proactive Controls / ASVS rows, then use current official docs for the actual framework in the repo. A future extension can add declaration catalogs for TypeScript and Java secure libraries, but the first runbook should only create the interface and fallback discipline. The design must handle TypeScript/Java as control-first because this repo currently has authoritative declarations only for Hulumi and SunLitSecurityLibraries.

## Security test selection matrix

| Touched surface | Secure-construction default | Tests to add before or during `/slo-execute` | Verification lane |
|---|---|---|---|
| Rust request body / boundary | `secure_boundary::SecureJson<T>` or equivalent advertised capability | unit tests for accepted/rejected payloads; abuse BDD for oversized/unknown/malformed fields | `/slo-verify` Pass 1-3 + Pass 4 SAST |
| Rust path / URL / redirect / shell arg / SQL identifier | `safe_types::*`; parameterized SQL | negative tests for traversal, SSRF, open redirect, command injection, SQL identifier abuse | SAST/variant check; no DAST unless reachable via HTTP |
| Authn/authz/session | `secure_identity`, `secure_authz` | role/tenant isolation tests; replay/expired token tests | authenticated E2E; DAST only after logged-in state is proven |
| Secrets/passwords/crypto | `secure_data`; no raw algorithms | KDF/envelope API tests; secret redaction tests | SCA/audit; manual crypto review if new primitive |
| HTTP API route | route-aware framework code + OpenAPI when possible | abuse BDD; route inventory; SARIF endpoint/method if available | `/slo-sast` + `/slo-dast-tuner`/`zaprun` when smoke service exists |
| SPA / DOM behavior | framework-safe encoding and CSP-compatible patterns | browser E2E; DOM-XSS probes for tainted data paths | `zaprun ptk` only with PTK-capable image and explicit evidence |
| Pulumi TypeScript cloud resource | Hulumi component when explicit/detected; otherwise Pulumi secure pattern + policy | Pulumi unit tests; policy-pack checks; no plaintext secrets in state | `/slo-cloud-threat-model`; preview/drift evidence |
| GitHub Actions / platform workflow | explicit `permissions`, SHA-pinned actions, OIDC for cloud deploys | structural workflow tests | Pass 4 workflow hardening checks |
| Capability gap | no local silent hand-roll | `/slo-sec-libs --match`; gap record; optional upstream issue/fix | library-feedback loop |

## What to avoid

- Hand-rolled crypto, password hashing, auth/session, authorization, path/URL parsing, shell escaping, SQL identifiers, output encoding, or secret handling when a declared secure abstraction exists.
- Treating a scanner green result as proof the design is secure.
- Reporting an unauthenticated scan of an authenticated app as clean.
- Running DAST without authorization, scope, auth state, rate limits, and a disposable/staging target.
- Committing app-specific DAST rules to SunLitOrchestra or `zaprun`.
- Writing raw Pulumi resources for common hardened patterns when Hulumi is explicitly in scope.
- Putting secrets in Pulumi state, GitHub workflow env, test fixtures, SARIF, DAST auth artifacts, or issue bodies.
- Recommending TypeScript/Java security libraries from memory when no local capability catalog or current official docs were consulted.

## Handoff plan

The next artifact should be a future runbook, `docs/slo/future/RUNBOOK-SECURE-EXECUTION-CONTROLS.md`, with five milestones:

1. Add `/slo-execute` secure-construction pre-flight.
2. Add a secure-construction matrix and clarify `/slo-plan` Contract Block expectations.
3. Reconcile `/slo-verify` Pass 4 with `/slo-sast`, `/slo-dast-tuner`, and `zaprun`.
4. Add a Pulumi TypeScript / Hulumi cloud-IaC lane.
5. Dogfood the full loop against a Rust + Pulumi/Hulumi-style fixture and route any gaps through `/slo-sec-libs`.
