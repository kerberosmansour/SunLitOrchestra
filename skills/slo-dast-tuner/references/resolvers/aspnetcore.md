---
name: resolver-aspnetcore
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: ASP.NET Core (C#)

**Detection.** `Microsoft.AspNetCore.*` in `*.csproj`; `Program.cs`/`Startup.cs`;
`*Controller.cs` or minimal-API `app.Map*`.

**Entry-point declaration model.** Two styles:
- Attribute-routed controllers: `[ApiController] [Route("api/[controller]")]` on class +
  `[HttpGet("{id}")]`, `[HttpPost]`, `[HttpPut("{id}")]`, `[HttpDelete("{id}")]` on actions.
  `[controller]` token = class name minus "Controller".
- Minimal APIs: `app.MapGet("/users/{id}", handler)`, `app.MapPost(...)`,
  `var g = app.MapGroup("/api").RequireAuthorization();`.
Inputs: `[FromRoute]`, `[FromQuery]`, `[FromBody] dto`, `[FromHeader]`.

**Path/selector template syntax.** `{param}`, `{id:int}` constraints, `{*slug}` catch-all,
`[controller]`/`[action]` tokens. Address = group/route-prefix + class route + action route.

**Auth-marker vocabulary.** `[Authorize]` / `[Authorize(Roles="Admin")]` /
`[Authorize(Policy="...")]` on class or action; `[AllowAnonymous]` opts out; minimal-API
`.RequireAuthorization()` on endpoint/group; global fallback policy in `Program.cs`
(`app.UseAuthentication/UseAuthorization`, `AddAuthentication(JwtBearer|Cookies)`) ⇒ `auth_kind`.

**Sink→entry-point resolution.**
1. Sink in a service/repository → find the controller action / minimal-API delegate calling it.
2. Compose `[Route]`/`MapGroup` prefix + action attribute; verb from `[HttpVerb]`/`MapVerb`.
3. Auth = nearest `[Authorize]`/`.RequireAuthorization()` (action overrides class/group)
   minus `[AllowAnonymous]`, else the global fallback policy; role from `Roles=`.
4. `Program.cs`/DI/config sinks ⇒ `unresolved`.

**Worked example.**
`Services/AccountSvc.cs:61` (`cmd.CommandText="… '"+dto.Email+"'"`, `sqli`) called by
`[ApiController][Route("api/[controller]")] AccountController` →
`[HttpPost("login")] Login([FromBody] LoginDto dto)` with `[AllowAnonymous]` ⇒
`{http, POST, /api/account/login, requires_auth:false, vuln_class:sqli, param:Email}`.

**Validation status.** `spec-only:from-framework-docs`.
