---
name: resolver-laravel
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Laravel (PHP)

**Detection.** `laravel/framework` in `composer.json`; `routes/web.php`/`routes/api.php`;
`app/Http/Controllers/*`.

**Entry-point declaration model.** Route files: `Route::get('/users/{id}', [UserController::class,'show'])`,
`Route::post('/users', 'UserController@store')`, `Route::resource('posts', PostController::class)`
(REST set), `Route::apiResource(...)`. `Route::prefix('api')->group(...)` and
`Route::middleware([...])->group(...)` compose prefix/middleware. `routes/api.php` is
auto-prefixed `/api`. Inputs: `$request->input('x')`, `$request->id`, route `{params}`,
FormRequest validated data.

**Path/selector template syntax.** `{param}`, `{param?}` optional, `{id}` + `where`
constraints. `Route::resource` ⇒ index GET `/p`, show GET `/p/{p}`, store POST `/p`,
update PUT/PATCH `/p/{p}`, destroy DELETE. Address = `/api` (api.php) + group prefixes + path.

**Auth-marker vocabulary.** `->middleware('auth')`/`'auth:sanctum'`/`'auth:api'` on route or
group; `Route::middleware(['auth'])->group`; controller constructor
`$this->middleware('auth')`; `'role:admin'`/`'can:...'`/policy `authorize()` ⇒ role/object
authz. `auth_kind` from guard (`sanctum`/`passport`=bearer-jwt, `web`=session).

**Sink→entry-point resolution.**
1. Sink in a controller/service/model → find the controller action method.
2. Grep route files for `[Controller::class,'action']`/`'Controller@action'`/`resource`;
   apply `prefix`/`middleware` group chain; api.php ⇒ `/api` prefix.
3. Auth = route/group/controller `auth*` middleware; role from `role:`/`can:`/policy.
4. Artisan commands, service providers, config ⇒ `unresolved`.

**Worked example.**
`app/Http/Controllers/SearchController.php:27` `#index`
(`DB::select("… '".$request->q."'")`, `sqli`); `routes/api.php`:
`Route::middleware('auth:sanctum')->prefix('v1')->get('/search',[SearchController::class,'index'])` ⇒
`{http, GET, /api/v1/search, requires_auth:true, auth_kind:bearer-jwt, vuln_class:sqli, param:q}`.

**Validation status.** `spec-only:from-framework-docs`.
