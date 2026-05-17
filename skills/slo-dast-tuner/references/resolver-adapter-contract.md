---
name: slo-dast-tuner-resolver-adapter-contract
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Resolver-adapter contract + generic fallback

A resolver adapter is the **only** framework-specific part of the SAST→DAST bridge. Everything
else (entry-point-map schema, vuln taxonomy, probe library, auth/PTK discipline) is invariant.
An adapter is a *reference doc the agent reads*, not necessarily code — the agent performs the
resolution; the adapter makes it accurate and deterministic for a given framework.

## What every adapter MUST specify

Each `references/resolvers/<framework>.md` provides exactly these sections:

1. **Detection** — manifest/file signatures that select this adapter (e.g. `@nestjs/core` in
   `package.json`; `spring-boot-starter-web` in `pom.xml`/`build.gradle`; `Django` in
   `requirements.txt`; `Gemfile` has `rails`).
2. **Entry-point declaration model** — how routes/handlers are declared (decorator on method,
   central routes file/DSL, attribute, convention-based, handler-object).
3. **Path/selector template syntax** — how params are written (`:id`, `{id}`, `<int:id>`,
   `*`, `[controller]`) so concrete addresses and the injectable `param` can be built.
4. **Auth-marker vocabulary** — the tokens that mean "authenticated/authorized" (middleware,
   guard, decorator, annotation, filter, `before_action`) and how role/scope is expressed.
5. **Sink→entry-point resolution** — the concrete walk: from a SAST `file:line` up to its
   handler, then to the declaration that binds it, then to method+path+auth.
6. **Worked example** — one real source snippet → one entry-point-map target.
7. **Validation status** — `validated:<app>` (tested against a real vulnerable app) or
   `spec-only:from-framework-docs` (written from documentation, **not yet** fixture-tested).
   This label is mandatory and must be honest; an unvalidated adapter is an accelerator, not a
   guarantee, and the scan step must surface its status in the report.

## The generic fallback (no adapter, or adapter is `spec-only` and ambiguous)

The bridge must **degrade, never break**. With no matching adapter the agent applies this
language-agnostic procedure per SAST finding:

1. Open the sink file; find the smallest enclosing named function/method/handler.
2. Search the repo for where that symbol is **registered as a network entry point**: a
   route/controller table, a decorator/attribute/annotation on it, a framework registration
   call, an OpenAPI/GraphQL schema/proto referencing it, or an infra manifest (serverless.yml,
   k8s ingress, API Gateway) mapping a path to it.
3. Determine `protocol` and `selector` from whatever evidence exists (typed routes, OpenAPI,
   proto, schema). If only an OpenAPI/proto/schema exists and not the wiring, trust it.
4. Determine `requires_auth` by looking for an auth guard in the same chain/middleware/filter
   or a global security scheme in the OpenAPI/security config.
5. **If protocol or selector cannot be established from evidence → `unresolved` /
   `needs-human-input`.** Never guess a path from the CWE or the filename. This is the same
   rule `triage-sarif` enforces; the agent does not get to relax it.

The fallback is what makes the bridge universal on day one. Adapters raise accuracy and
determinism for known stacks; the fallback guarantees graceful behaviour on the unknown.

## Polyglot / multi-surface repos

A repo may have several surfaces (REST + GraphQL + a queue consumer). Run the matching adapter
per detected surface and emit one `targets[]` list spanning all of them, each entry carrying its
own `protocol`. Do not force a non-HTTP sink into an HTTP shape.

## Probe transfer caveat

`probe-library.md` payloads are mostly HTTP-level. For `graphql`/`grpc`/`event` the *class*
transfers (sqli is still sqli) but the *delivery* differs — the adapter for that surface must
point at the protocol-specific probe variant, not the REST one.

## Catalog

See [`resolvers/index.md`](resolvers/index.md) for the detection→adapter map and per-adapter
validation status.
