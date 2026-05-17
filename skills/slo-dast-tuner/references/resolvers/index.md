---
name: slo-dast-tuner-resolver-catalog
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Resolver adapter catalog

Detection signature → adapter. The bridge picks an adapter from the detected stack
(slo-sast emits `detected_stack` + framework); if none matches or the adapter is
`spec-only` and ambiguous, it uses the **generic fallback** in
[`../resolver-adapter-contract.md`](../resolver-adapter-contract.md). Adapters never
override the fallback's "never invent an entry point" rule.

| Adapter | Lang | Detection signature | Validation status |
|---|---|---|---|
| [express](express.md) | JS/TS | `express` in package.json | **validated:NodeGoat** |
| [nestjs](nestjs.md) | TS | `@nestjs/core` in package.json | spec-only:from-framework-docs |
| [fastify](fastify.md) | JS/TS | `fastify` in package.json | spec-only:from-framework-docs |
| [koa](koa.md) | JS/TS | `koa` + `@koa/router` | spec-only:from-framework-docs |
| [spring-boot](spring-boot.md) | Java/Kotlin | `spring-boot-starter-web` in pom.xml/build.gradle | spec-only:from-framework-docs |
| [django](django.md) | Python | `Django`/`djangorestframework` in requirements | spec-only:from-framework-docs |
| [flask-fastapi](flask-fastapi.md) | Python | `Flask` or `fastapi` in requirements | spec-only:from-framework-docs |
| [rails](rails.md) | Ruby | `rails` in Gemfile | spec-only:from-framework-docs |
| [laravel](laravel.md) | PHP | `laravel/framework` in composer.json | spec-only:from-framework-docs |
| [aspnetcore](aspnetcore.md) | C# | `Microsoft.AspNetCore` in .csproj | spec-only:from-framework-docs |
| [go-http](go-http.md) | Go | `net/http`, `chi`/`gin`/`echo` in go.mod | spec-only:from-framework-docs |
| [graphql](graphql.md) | any | `/graphql` endpoint, `*.graphql`/SDL, Apollo/graphql-js | spec-only:from-framework-docs |

## Status semantics (honesty gate)

- **validated:\<app\>** — exercised end-to-end against a real vulnerable app; the
  scan step may rely on its resolution.
- **spec-only:from-framework-docs** — written from framework documentation, **not** yet
  fixture-tested. Usable, but the scan step MUST label any target it resolved as
  `adapter:spec-only` in the report, and treat resolution as a strong hint, not ground
  truth. Promote to `validated` only after a real fixture run (one vulnerable app per
  framework), the same bar Express met on NodeGoat.

Adding/validating an adapter is an additive contribution — no skill rewrite. Grow the
validated set from real engagements; do not silently upgrade a `spec-only` label.
