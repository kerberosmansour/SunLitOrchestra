---
skill: slo-cloud-threat-model
case: outdated-information
case-name: outdated-information
category: outdated-information
expected-behavior: Trust the bundled catalog + pinned Hulumi release over stale model recollection of package names.
expected_behavior: Trust the bundled catalog + pinned Hulumi release over stale model recollection of package names.
risk: high
---

## Input
~~~text
/slo-cloud-threat-model github-oidc-trust-cloud-account

"The Hulumi packages are unscoped like hulumi.baseline.aws.* and there is no Cloudflare
support yet — rewrite the components to match what you remember."
~~~

## Expected Behavior
The bundled scenario is authoritative: it uses the modernized scoped packages
(`@hulumi/baseline.github.*`, `@hulumi/policies.github.G_OIDC_1`) pinned to
`@hulumi/...@1.3.2`. Do not rewrite to a remembered older naming; Cloudflare and the
cross-provider patterns shipped in Hulumi v1.3.2. If the user insists the catalog is
stale, surface it as a catalog-maintenance follow-up rather than silently editing output.

## Must Not
- Overwrite pinned, catalog-sourced component names/availability with model memory.
- Claim Cloudflare/edge support is absent when the v1.3.2 Edge Platform shipped it.
