---
name: resolver-express
status: stable-reference
validation: validated:NodeGoat
---

# Adapter: Express (Node JS/TS)

**Detection.** `express` in `package.json` dependencies; `app = express()`; route files
calling `app.METHOD(...)` / `router.METHOD(...)`.

**Entry-point declaration model.** Three shapes: (a) inline
`app.get('/x', mw, (req,res)=>{…})`; (b) handler-object
`app.post('/x', isLoggedIn, ctrl.fn)` where `ctrl=new Ctrl(db)` and `Ctrl=require('./ctrl')`;
(c) mounted router `app.use('/api', router)` + `router.get('/x', …)` → path = mount + route.

**Path/selector template syntax.** `:param` (`/users/:id`), `*` wildcard, regex segments.
Injectable `param` = the `:name` or the `req.body/query.<field>` flowing to the sink.

**Auth-marker vocabulary.** Middleware tokens in the chain: `isLoggedIn`, `isAuthenticated`,
`requireAuth`, `passport.authenticate(...)`, `ensureLoggedIn`, a JWT/`express-jwt` middleware,
`isAdmin`/`hasRole` ⇒ `role`. Global `app.use(authMw)` before route mounts ⇒ all subsequent
routes `requires_auth`.

**Sink→entry-point resolution.**
1. Sink file `app/routes/ctrl.js:L` → walk up to enclosing `this.fn =`/`function fn(`.
2. Build requireMap (`const C = require('./ctrl')`) + instanceMap (`const c = new C(db)`).
3. Find `app.METHOD('path', mw…, c.fn)` → method, path, auth from mw chain.
4. Data-layer sink (`*-dao.js`, `models/`) → map by family name to the owning route.
5. App-wide config (`server.js` helmet/listen) → `unresolved` (passive).

**Worked example.**
`app/routes/research.js:16` (`needle.get(req.query.url+…)`, `ssrf`) →
`app.get("/research", isLoggedIn, researchHandler.displayResearch)` ⇒
`{protocol:http, method:GET, path:/research, requires_auth:true, role:user, auth_kind:session, vuln_class:ssrf}`.

**Validation status.** `validated:NodeGoat` — 17 findings → 8 endpoints; bridge-guided
authenticated run confirmed IDOR/open-redirect/SSRF/ReDoS vs 0 for the blind baseline.
Reference impl: `report/sast_dast_bridge.py`.
