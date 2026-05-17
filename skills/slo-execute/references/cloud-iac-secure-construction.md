---
name: slo-execute-cloud-iac-secure-construction
source_skill: skills/slo-execute/SKILL.md
---

# Cloud IaC secure construction

Use this lane for Pulumi TypeScript work that touches AWS, GitHub, Cloudflare, or cross-provider deployment identity.

## Selection rule

- **Hulumi explicit or detected**: when the runbook, repo, package names, imports, or user request names Hulumi, prefer Hulumi components and policy packs.
- **Sherif-owned Pulumi TypeScript with `security_libs_required: true`**: treat Hulumi as in scope unless the runbook says otherwise.
- **Generic Pulumi TypeScript**: do not force Hulumi. Use Pulumi secure patterns, Policy as Code, unit tests, and record whether Hulumi should be adopted.

## Secure defaults

| Surface | Default |
|---|---|
| S3 bucket | `@hulumi/baseline.aws.SecureBucket` or capability-gap handling. |
| Account baseline | `@hulumi/baseline.aws.AccountFoundation` plus `HulumiHardeningPack` where available. |
| GitHub Actions deployment identity | OIDC, exact repo/environment/job claims, SHA-pinned actions, explicit `permissions`, no plaintext cloud keys. |
| Cloudflare admin / preview hostname | Hulumi Cloudflare baseline when explicit; otherwise deny-by-default access, TLS, DNSSEC/proxy policy, and WAF/rate-limit checks. |
| Secrets | no plaintext secrets in state; use provider secret stores and Pulumi secret configuration. |

## Tests and evidence

- Pulumi TypeScript unit tests use `pulumi.runtime.setMocks` or the project-local equivalent.
- Policy as Code / CrossGuard checks run before merge when policy packs exist.
- `pulumi preview` output is preview evidence; sensitive values must stay redacted.
- Drift evidence is required for existing resources when the runbook touches enforcement or migration.
- GitHub workflow changes must include explicit permissions and SHA-pinned third-party actions.

## Handoff

`/slo-cloud-threat-model` names platform threats and recommended Hulumi components. `/slo-execute` uses this lane to turn that threat model into code, tests, policy checks, preview evidence, and drift evidence.
