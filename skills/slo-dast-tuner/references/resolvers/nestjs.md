---
name: resolver-nestjs
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: NestJS (TypeScript)

**Detection.** `@nestjs/core` in `package.json`; `*.controller.ts` files; `@Controller()` decorators.

**Entry-point declaration model.** Decorator-on-method. `@Controller('users')` sets the path
prefix; method decorators `@Get(':id')`, `@Post()`, `@Put(':id')`, `@Delete(':id')`,
`@Patch()` add the sub-path. Full path = controller prefix + method path. Global prefix from
`app.setGlobalPrefix('api')` in `main.ts`. Input params via `@Param('id')`, `@Query('q')`,
`@Body()`/DTO, `@Headers()`.

**Path/selector template syntax.** `:param` (`@Get(':id')`), wildcards `*`. Concrete address
= `globalPrefix + controllerPath + methodPath`.

**Auth-marker vocabulary.** `@UseGuards(AuthGuard('jwt'))`, `@UseGuards(JwtAuthGuard)` on
controller or method; `@Roles('admin')` + `RolesGuard` ⇒ `role`; a global guard registered
via `APP_GUARD` provider in a module ⇒ all routes `requires_auth` unless `@Public()`.

**Sink→entry-point resolution.**
1. Sink in `*.service.ts` → find the service method; find the controller method that calls it
   (constructor-injected service, same feature module).
2. Read the controller method's HTTP decorator + the `@Controller()` prefix + global prefix.
3. Auth = nearest `@UseGuards`/`@Roles` (method overrides controller) or an `APP_GUARD`.
4. Provider/config sinks (no controller path) → `unresolved`.

**Worked example.**
`cats.service.ts:42` (raw query from `dto.name`, `sqli`) called by
`@Controller('cats')` → `@Post() create(@Body() dto)` with `@UseGuards(JwtAuthGuard)`,
global prefix `api` ⇒ `{http, POST, /api/cats, requires_auth:true, auth_kind:bearer-jwt, vuln_class:sqli, param:name}`.

**Validation status.** `spec-only:from-framework-docs` — not yet fixture-tested; resolution is
a strong hint, label targets `adapter:spec-only`. Promote after a real NestJS vulnerable-app run.
