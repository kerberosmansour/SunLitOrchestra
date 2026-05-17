---
name: resolver-koa
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Koa (Node JS/TS)

**Detection.** `koa` + `@koa/router`/`koa-router` in `package.json`; `new Router()`;
`app.use(router.routes())`.

**Entry-point declaration model.** `router.get('/x', mw, async ctx => {…})`;
`router.post('/x', authMw, ctrl.fn)`. Sub-routers combined via
`router.use('/api', sub.routes())` or `new Router({ prefix: '/api' })`. Input via
`ctx.params`, `ctx.query`, `ctx.request.body` (koa-bodyparser).

**Path/selector template syntax.** `:param` (`/users/:id`), `(.*)` regex, `*`. Address =
router prefix(es) + path.

**Auth-marker vocabulary.** Middleware in the chain: `koa-jwt` (`jwt({secret})`),
`koa-passport` `passport.authenticate(...)`, custom `requireAuth`/`ensureLoggedIn`;
`app.use(jwt(...))` before `router.routes()` ⇒ global auth. Role checks read `ctx.state.user.role`.

**Sink→entry-point resolution.**
1. Sink → enclosing async handler / `ctrl.fn`.
2. Find `router.METHOD('path', mw…, handler)` binding it; collect method, path.
3. Compose prefixes from `Router({prefix})` / `router.use(prefix, sub)`.
4. Auth = jwt/passport/custom mw in this route's chain or a global `app.use` before routes.
5. Pure middleware/config ⇒ `unresolved`.

**Worked example.**
`controllers/user.js:21` (`coll.findOne(ctx.query)`, `nosqli`) bound
`router.get('/users', jwt({secret}), userCtrl.list)`, app-level prefix `/api` ⇒
`{http, GET, /api/users, requires_auth:true, auth_kind:bearer-jwt, vuln_class:nosqli}`.

**Validation status.** `spec-only:from-framework-docs`.
