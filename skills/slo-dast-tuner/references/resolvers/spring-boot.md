---
name: resolver-spring-boot
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Spring Boot (Java / Kotlin)

**Detection.** `spring-boot-starter-web`/`spring-webmvc`/`spring-webflux` in
`pom.xml`/`build.gradle(.kts)`; `@RestController`/`@Controller` classes.

**Entry-point declaration model.** Annotation-on-class+method. Class
`@RequestMapping("/api/users")` (or none) + method `@GetMapping("/{id}")`,
`@PostMapping`, `@PutMapping`, `@DeleteMapping`, `@RequestMapping(method=…)`. Functional
routes: `RouterFunctions.route().GET("/x", handler)`. Inputs: `@PathVariable`,
`@RequestParam`, `@RequestBody dto`, `@RequestHeader`. `server.servlet.context-path` in
`application.properties/yml` prepends all paths.

**Path/selector template syntax.** `{param}` (`/users/{id}`), `**` ant-style wildcard.
Address = context-path + class mapping + method mapping.

**Auth-marker vocabulary.** Method `@PreAuthorize("hasRole('ADMIN')")`/`@Secured`/
`@RolesAllowed`; a `SecurityFilterChain`/`WebSecurityConfigurerAdapter` with
`authorizeHttpRequests().requestMatchers("/api/**").authenticated()` ⇒ pattern-scoped auth;
`permitAll()` patterns are public. `auth_kind` from the configured mechanism
(`httpBasic`, `oauth2ResourceServer`/JWT, `formLogin`, `sessionManagement`).

**Sink→entry-point resolution.**
1. Sink in `@Service`/repository → find the `@RestController` method that calls it
   (constructor/`@Autowired` injection, same feature package).
2. Compose context-path + class `@RequestMapping` + method mapping; method = the mapping verb.
3. Auth = nearest method security annotation, else match the path against the
   `SecurityFilterChain` matchers; role from `hasRole`/`hasAuthority`.
4. `@Configuration`/`@Bean`/`application.yml` sinks ⇒ `unresolved`.

**Worked example.**
`UserService.java:88` (`jdbcTemplate.queryForList("…'"+email+"'")`, `sqli`) called by
`@RestController @RequestMapping("/api/users")` → `@PostMapping("/search") search(@RequestBody Q q)`,
filter chain requires auth on `/api/**`, JWT ⇒
`{http, POST, /api/users/search, requires_auth:true, auth_kind:bearer-jwt, vuln_class:sqli, param:email}`.

**Validation status.** `spec-only:from-framework-docs`.
