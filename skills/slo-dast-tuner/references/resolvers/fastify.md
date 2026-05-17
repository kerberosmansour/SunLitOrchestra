---
name: resolver-fastify
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Fastify (Node JS/TS)

**Detection.** `fastify` in `package.json`; `fastify.route(...)` / `fastify.get(...)`;
plugins registered via `fastify.register(plugin, { prefix: '/api' })`.

**Entry-point declaration model.** (a) shorthand `fastify.get('/x', opts, handler)`;
(b) full `fastify.route({ method, url, preHandler, handler, schema })`; (c) plugin
encapsulation — a plugin registered with `{ prefix }` and its internal routes ⇒ path =
prefix + route url. `schema` often carries the JSON-schema for body/query (use it for `param`).

**Path/selector template syntax.** `:param` (`/users/:id`), wildcard `*`. Concrete address =
register-prefix(es) + route url.

**Auth-marker vocabulary.** `preHandler: fastify.authenticate` / `[fastify.verifyJWT]`;
`onRequest: fastify.auth([...])`; `@fastify/jwt`, `@fastify/auth`, `fastify.addHook('onRequest', authHook)`
(global ⇒ all routes). Role via a `preHandler` checking `request.user.role`.

**Sink→entry-point resolution.**
1. Sink in a service/handler → identify the handler function value.
2. Find the `fastify.route`/shorthand whose `handler` is that function; collect `method`,`url`.
3. Walk plugin `register(..., {prefix})` nesting to compose the full path.
4. Auth = `preHandler`/`onRequest` referencing an auth decorator, or a global `onRequest` hook.
5. Decorators/plugins without a route ⇒ `unresolved`.

**Worked example.**
`handlers/order.js:30` (`db.query(\`…${req.body.id}\`)`, `sqli`), wired
`fastify.route({method:'POST', url:'/orders', preHandler:[fastify.authenticate], handler:placeOrder})`,
plugin prefix `/v2` ⇒ `{http, POST, /v2/orders, requires_auth:true, auth_kind:bearer-jwt, vuln_class:sqli, param:id}`.

**Validation status.** `spec-only:from-framework-docs`.
