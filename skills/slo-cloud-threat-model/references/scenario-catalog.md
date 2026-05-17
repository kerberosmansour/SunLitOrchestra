---
name: slo-cloud-threat-model-scenario-catalog
status: stable
source_skill: skills/slo-cloud-threat-model/SKILL.md
audience: humans choosing a scenario; the agent confirming coverage
purpose: Human-readable index of the prebuilt scenarios. The machine source of truth is scripts/scenario_catalog.py + scenarios/.
---

# Scenario catalog

The stable declared order is owned by `scripts/scenario_catalog.py`
(`SCENARIO_ORDER`); the data lives in `scenarios/<id>.json`. New scenarios append
at the end so the order stays a stability contract (mirrors Hulumi's
`list-scenarios.mjs`). Component availability is pinned to the Hulumi release that
shipped the surface; Cloudflare / cross-provider surfaces shipped in **Hulumi
v1.3.2** (Edge Platform, 2026-05-15).

## AWS (ported + modernized from `hulumi-threat-model`)

| Scenario ID | Focus | Primary Hulumi surface |
|---|---|---|
| `aws-multi-account-baseline` | Day-zero account foundation: CloudTrail, Config, GuardDuty, Security Hub, IAM, KMS | `@hulumi/baseline.aws.AccountFoundation`, `@hulumi/policies.aws.*`, `@hulumi/drift` |
| `s3-public-bucket-hardening` | public-access-block, SSE-KMS, versioning, TLS-only, object-lock | `@hulumi/baseline.aws.SecureBucket`, `@hulumi/policies.aws.HulumiHardeningPack` |
| `iam-least-privilege` | password policy, role scoping, Access Analyzer, MFA, iac-role tag | `@hulumi/baseline.aws.AccountFoundation.iamBaseline`, `@hulumi/policies.aws.*` |
| `rds-encryption-at-rest` | storage/backup CMK, TLS param group, IAM auth, deletion protection | `@hulumi/baseline.aws.AccountFoundation.kmsRing`, `@hulumi/policies.aws.CisV5Pack` |
| `lambda-secrets-access` | execution-role scoping, Secrets Manager, KMS, env-var leakage | `@hulumi/baseline.aws.AccountFoundation.secretsManager`, `@hulumi/policies.aws.HulumiHardeningPack` |

## GitHub (ported + modernized)

| Scenario ID | Focus | Primary Hulumi surface |
|---|---|---|
| `github-oidc-trust-cloud-account` | three-axis `sub` claim, UNC6426 wildcard-trust mitigation | `@hulumi/baseline.github.OrgFoundation`, `@hulumi/policies` `G_OIDC_1` |
| `github-actions-supply-chain` | third-party Action ingestion, SHA-pinning, pwn-request, cache poisoning | `@hulumi/baseline.github.OrgFoundation`, `@hulumi/policies.github.HulumiGithubHardeningPack` |
| `github-app-token-exposure` | App / installation-token rotation, scope minimization, short-lived exchange | `@hulumi/baseline.github.OrgFoundation`, `@hulumi/policies.github.HulumiGithubHardeningPack` |
| `github-self-hosted-runner` | ephemeral runners, runner-image hardening, runner-group scoping | `@hulumi/baseline.github.OrgFoundation`, `@hulumi/drift.adapters.GithubWebhookFallbackAdapter` |

## Cloudflare / edge (new — Hulumi v1.3.2 Edge Platform)

| Scenario ID | Focus | Primary Hulumi surface |
|---|---|---|
| `cloudflare-zone-and-dns-foundation` | zone DNSSEC + strict SSL/TLS + proxied-by-default public DNS | `@hulumi/cloudflare-baseline.ZoneFoundation`, `.PublicHostname`, `CF_DNSSEC_1`, `CF_DNS_1` |
| `cloudflare-edge-waf-and-bot-protection` | managed/OWASP rulesets, rate-limit baseline, plan-aware bot intent, overtrust | `@hulumi/cloudflare-baseline.EdgeWafBaseline`, `.BotProtectionBaseline` |
| `cloudflare-origin-bypass-prevention` | Cloudflare→AWS origin reachability: tunnel vs allowlist+AOP, DNS-history bypass | `@hulumi/platform-patterns.CloudflareOriginIngress`, `CF_ORIGIN_1`, `X_ORIGIN_1` |
| `cloudflare-protected-admin-access` | Cloudflare Access deny-by-default in front of admin/preview hosts | `@hulumi/cloudflare-baseline.ProtectedAdminHostname` |

## Cross-provider (new)

| Scenario ID | Focus | Primary Hulumi surface |
|---|---|---|
| `github-aws-oidc-deployment-identity` | positive GitHub→AWS deployment role: exact repo+env+job_workflow_ref claims | `@hulumi/platform-patterns.GitHubAwsOidcDeploymentRole`, `G_OIDC_1` |
