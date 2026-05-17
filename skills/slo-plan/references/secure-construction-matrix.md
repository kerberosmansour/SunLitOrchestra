---
name: slo-plan-secure-construction-matrix
source_skill: skills/slo-plan/SKILL.md
---

# Secure-construction matrix

Use this when filling the `Proactive controls in play` row. The row should tell `/slo-execute` what to build with, not merely name a control family.

| Touched surface | Secure-construction default | Tests expected |
|---|---|---|
| Rust request body / boundary | SunLitSecurityLibraries `secure_boundary::SecureJson<T>` or matched equivalent. | Accepted/rejected payload tests, malformed JSON, size/depth/unknown-field abuse. |
| Rust path / URL / redirect / shell / SQL identifier | `safe_types::{SafePath,SafeUrl,SafeFilename,SafeCommandArg,SqlIdentifier,SafeRedirectUrl}`; parameterized SQL. | Traversal, SSRF, open-redirect, command-injection, SQL-identifier abuse. |
| Authn / authz / session | `secure_identity` and `secure_authz`. | Expired/replayed credentials, role and tenant isolation. |
| Secrets / passwords / crypto | `secure_data` and provider KMS/secret-store APIs. | Secret redaction, KDF/envelope API tests, no plaintext fixture/state leakage. |
| HTTP API route | Framework route inventory plus OpenAPI when possible. | Abuse BDD, SAST, and `/slo-dast-tuner`/`zaprun` when a smoke service exists. |
| SPA / DOM behavior | Framework-safe rendering/encoding and CSP-compatible patterns. | Browser E2E, DOM-XSS probes when tainted data reaches DOM. |
| Pulumi TypeScript cloud resource | Hulumi when explicit/detected; otherwise Pulumi secure pattern with policy as code. | Pulumi unit tests, preview evidence, policy checks, drift evidence when touching existing resources. |
| AWS S3 bucket | `@hulumi/baseline.aws.SecureBucket` when Hulumi is in scope. | Unit test expected public-access block/encryption/versioning; policy pack pass. |
| GitHub deployment identity | OIDC, exact trust claims, explicit workflow permissions, SHA-pinned actions. | Workflow structural tests and no plaintext cloud keys. |
| Cloudflare admin / preview hostname | Hulumi Cloudflare baseline when in scope; otherwise protected access, TLS, WAF/rate-limit checks. | Policy/preview evidence and access-deny abuse case. |
| GitHub Actions | explicit `permissions`, full-length SHA action pins, no `pull_request_target` for untrusted code. | Workflow structural tests. |
| Capability gap | No silent local hand-roll. | `/slo-sec-libs --match`, gap record, optional upstream issue/fix. |

For TypeScript and Java application code, use OWASP ASVS and official framework docs as the starting point. Do not invent library capability claims without a local declaration catalog or current official docs.
