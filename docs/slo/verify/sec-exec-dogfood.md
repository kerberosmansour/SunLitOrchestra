# Secure Execution Controls Dogfood

Date: 2026-05-17

## Fixture

The dogfood fixture is `tests/fixtures/secure-execution-controls/`.

## Surface Map

| Surface | Secure-construction decision | Test selector |
|---|---|---|
| Rust boundary accepting untrusted Markdown for agent prompts | matched: `component:secure_boundary` / `secure_boundary` with `agent-prompt-injection-boundary` | unit + variant-analysis schema checks |
| Rust variant-analysis evidence | matched: `component:security_core` with `variant-analysis-result-schema` | unit + serialization checks |
| Hulumi Pulumi TypeScript bucket | matched: `@hulumi/baseline.aws.SecureBucket` | Pulumi mocks, policy-as-code, preview evidence |
| DAST | N/A - no smoke service | no HTTP service is launched by this fixture |

## Capability Matches

- matched: `secure_boundary` now advertises `agent-prompt-injection-boundary`.
- matched: `security_core` now advertises `variant-analysis-result-schema`.
- matched: `@hulumi/baseline.aws.SecureBucket` remains the hardened S3 bucket default.
- matched: Hulumi declaration now covers Cloudflare and platform patterns in v1.3.2.

## Gaps And Upstream Flow

- gap: prompt-boundary helper was missing from `kerberosmansour/SunLitSecurityLibraries`.
  - upstream issue: `kerberosmansour/SunLitSecurityLibraries#49`
  - upstream change status: applied on local branch `slo/secure-exec-capability-gaps`
- gap: typed variant-analysis result schema was missing from `kerberosmansour/SunLitSecurityLibraries`.
  - upstream issue: `kerberosmansour/SunLitSecurityLibraries#50`
  - upstream change status: applied on local branch `slo/secure-exec-capability-gaps`
- gap: Hulumi declarations were stale and did not advertise Cloudflare/platform patterns in `kerberosmansour/hulumi`.
  - upstream issue: `kerberosmansour/hulumi#171`
  - upstream change status: applied on local branch `slo/secure-exec-declarations`

## Residual Risks

- No live service or cloud account was provisioned for this dogfood. Runtime DAST,
  Pulumi preview, and drift evidence are therefore `N/A - no smoke service`.
- The local branches must still be reviewed and merged upstream before downstream
  runbooks can rely on released crate/package versions.
