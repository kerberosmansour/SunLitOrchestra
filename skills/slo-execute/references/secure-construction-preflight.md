---
name: slo-execute-secure-construction-preflight
source_skill: skills/slo-execute/SKILL.md
---

# Secure-construction pre-flight

Run this before BDD tests or implementation. The output is a short surface map recorded in the milestone notes or Evidence Log.

## Build the surface map

Read the milestone Contract Block, `SECURITY.md`, and `docs/slo/design/<slug>-threat-model.md`. Mark every touched surface:

| Surface | Secure-construction default |
|---|---|
| Rust request body / boundary | Prefer SunLitSecurityLibraries through `/slo-sec-libs`; typical capability is `secure_boundary::SecureJson<T>`. |
| Rust path / URL / redirect / shell / SQL identifier | Prefer `safe_types` or the advertised equivalent; no raw path, string-built SQL, or shell argument construction without explicit justification. |
| Authn / authz / session | Prefer `secure_identity` and `secure_authz`; hand-rolled auth needs a residual-risk row. |
| Secrets / passwords / crypto | Prefer `secure_data`; hand-rolled crypto is forbidden unless the secure library has a documented capability gap. |
| HTTP route / API | Add abuse tests and route-aware SAST; use `/slo-dast-tuner` only when a smoke service and authorization exist. |
| Pulumi TypeScript / cloud platform | Use [`cloud-iac-secure-construction.md`](cloud-iac-secure-construction.md). |
| TypeScript / Java app code | Use OWASP ASVS / Proactive Controls plus current official framework docs; do not invent library capability claims. |

## Capability lookup

When `security_libs_required: true` and the stack is Rust/axum, Hulumi, or another declared catalog source:

1. Match the proactive-control row with `/slo-sec-libs --match <runbook.md> --catalog <catalog.json>`.
2. Use only matched capabilities that cite a `catalog_bom_ref`.
3. Treat close-but-not-exact matches as a capability gap, not a recommendation.
4. If the user owns the upstream project and explicitly authorizes upstream work, file the issue and implement the upstream fix before relying on a local workaround.

## What to avoid

- hand-rolled crypto, hand-rolled auth, raw path parsing, raw URL parsing, string-built SQL, manual output encoding, shell argument string assembly, plaintext secret handling, and bespoke tenant checks when a declared secure abstraction exists.
- model-memory recommendations for SunLitSecurityLibraries, Hulumi, TypeScript, or Java libraries.
- unauthenticated DAST evidence for authenticated routes.
- scanner-only "security" with no secure-construction choice.

## Gap and residual-risk branch

If no declared capability fits:

1. Record `capability gap: <short-id>` in the Evidence Log.
2. File through `/slo-sec-libs` intake, or upstream when the user explicitly confirms the owned repo.
3. Either fix upstream first, or write `residual risk: <reason>, expiry, follow-up issue` before using a local workaround.

If no security-relevant surface is touched, write:

```text
N/A - no new security-relevant surface
```

and include the reason.

