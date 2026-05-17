---
name: resolver-flask-fastapi
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Flask & FastAPI (Python decorator)

Two frameworks, one decorator-on-function model. Detect via `Flask` vs `fastapi` in
`requirements.txt`/`pyproject`.

**Entry-point declaration model.**
- Flask: `@app.route('/x', methods=['POST'])` or `@app.get('/x')`; blueprints
  `bp = Blueprint('b', __name__, url_prefix='/api')` + `app.register_blueprint(bp)` ⇒ prefix.
  Inputs: `request.args`, `request.form`, `request.json`, `view_args`.
- FastAPI: `@app.get('/x')`/`@router.post('/x')`; `APIRouter(prefix='/api')` +
  `app.include_router(router, prefix=…)`. Inputs: path/query params as function args,
  `Body`/Pydantic model, `Depends`.

**Path/selector template syntax.** Flask `<int:id>`/`<id>`; FastAPI `{id}`. Address =
blueprint/router prefix chain + route path.

**Auth-marker vocabulary.**
- Flask: `@login_required` (Flask-Login), `@jwt_required()` (Flask-JWT-Extended),
  custom `@requires_auth`; `before_request` global guard.
- FastAPI: a security `Depends(get_current_user)`/`Depends(oauth2_scheme)` in the
  signature or `dependencies=[Depends(...)]` on the router/app; `Security(scopes=[…])` ⇒ role.

**Sink→entry-point resolution.**
1. Sink → enclosing view function (or a service it calls; trace the caller view).
2. Read its route decorator (+ blueprint/router prefix) for method + path.
3. Auth = the auth decorator / security `Depends` on the function or its router; global
   `before_request`/router `dependencies` apply to all.
4. CLI/`@app.cli`/config/startup sinks ⇒ `unresolved`.

**Worked example.**
FastAPI `routers/files.py:33` `@router.get('/files/{name}')` calling
`open(f"data/{name}")` (`path-traversal`), `APIRouter(prefix='/api')`,
`dependencies=[Depends(current_user)]` ⇒
`{http, GET, /api/files/{name}, requires_auth:true, auth_kind:bearer-jwt, vuln_class:path-traversal, param:name}`.

**Validation status.** `spec-only:from-framework-docs`.
