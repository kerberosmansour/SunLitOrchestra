---
name: slo-dast-tuner-sast-route-bridge
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# SAST → DAST route bridge (agent-resolved guided map)

## Why this exists

`zaprun triage-sarif` correctly refuses to promote a finding to `dast-detectable` from CWE
alone, and a Semgrep SARIF carries only `file:line + ruleId + CWE` — no HTTP route or method.
So the guided map is empty and DAST cannot target SAST findings. This is not a tool defect; it
is missing information. **An AI agent can supply that information by reading the codebase** — the
deterministic tool cannot, but the agent can resolve each sink to a concrete endpoint. This is
the load-bearing middle ground between SAST and DAST.

Validated on NodeGoat: 17 SAST findings → 8 endpoints resolved with method + auth; a
bridge-guided **authenticated** scan then confirmed IDOR, open-redirect, SSRF and ReDoS — four
vulnerabilities the blind unauthenticated baseline scored **zero** on.

## The bridge procedure (agent step, between slo-sast and the scan)

For each SARIF result `(file, line, ruleId, cwe)`:

1. **Locate the sink's enclosing handler.** Read the source file; walk up from `line` to the
   nearest handler boundary (`this.<fn> = (req,res)=>`, `function <fn>(req,res)`, or an inline
   `app.METHOD(path, …, (req,res)=>{…})`).
2. **Resolve the route.** Parse the app's route-registration file(s). Build:
   - `requireMap`: `const X = require('./y')` → module `y`
   - `instanceMap`: `const xH = new XHandler(db)` → handler-object instance ↔ module
   - route table: `app.METHOD('path', mw…, instance.fn)` → method, path, middleware chain
   Match the sink's module + handler fn to the route that references it. For data-layer sinks
   (`*-dao.js`, `models/`, `services/`) map by family name to the owning route.
3. **Determine auth.** If the route's middleware chain contains an auth guard
   (`isLoggedIn`, `isAuthorized`, `passport.authenticate`, a JWT check) mark
   `requires_auth: true` and the role (`admin` if an admin guard is present).
4. **Attach the base URL.** The deployment target (`--deployment-target` / running container)
   is the base; `url = base + route.path`.
5. **Classify, do not guess.** If no route resolves to a sink (app-wide config such as
   missing-helmet, autoescape-off, http-server), emit it as `needs-human-input` /
   passive-only — never invent an endpoint. Mirrors the triage discipline.

## Guided-map schema (consumed by the scan step)

```json
{
  "schema_version": "1.0",
  "mode": "agent-resolved-guided",
  "base_url": "http://host.docker.internal:4000",
  "targets": [
    {"source": "app/routes/research.js:16", "rule": "node-ssrf-taint",
     "cwe": "CWE-918", "http_method": "GET", "url_path": "/research",
     "url": "http://host.docker.internal:4000/research",
     "requires_auth": true, "role": "user",
     "vuln_class": "ssrf", "probe": "param=internal URL; check reflection",
     "dast_verdict": "dast-detectable"}
  ],
  "unresolved": [{"source": "server.js:15", "cwe": "CWE-693",
     "reason": "app-wide config, no endpoint", "dast_verdict": "needs-human-input"}]
}
```

## How the scan step uses it

1. **Authenticate first** when any target has `requires_auth: true` — per
   `authentication-coverage.md`, an unauthenticated scan of these endpoints is a coverage
   failure. The map tells the skill *which* endpoints need auth and the role.
2. Drive a **scoped** zaprun run / targeted probes at exactly the resolved
   `method + url + param`, using the `vuln_class` to pick the payload family
   (sqli/nosqli/ssrf/redos/idor/open-redirect/path-traversal/xss).
3. Report per-target verdicts. Blind classes (eval RCE) stay `dast-partial` (need a
   timing/error oracle); server-log sinks stay `sast-only`; app-wide config goes to the
   passive header pass — never reported as confirmed without a real oracle.

## Boundaries

- The resolver is a **reusable, generic algorithm** (Express handler-object + inline + router
  mounts); it carries no app-specific literals and ports across Express apps. Frameworks with a
  different route model (NestJS decorators, Fastify, Spring `@RequestMapping`, Django urls.py)
  need a per-framework resolver of the same shape — same methodology, different parser.
- The bridge enriches targeting; it does **not** lower thresholds or assert findings. A resolved
  endpoint is a *place to test*, confirmation still requires a runtime oracle.
- Reference implementation: `report/sast_dast_bridge.py` + `report/guided_dast_probe.py` in the
  companion adversarial-tuning workspace.
