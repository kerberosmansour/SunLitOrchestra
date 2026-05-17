---
name: slo-sast-custom-rule-shape
source_skill: skills/slo-sast/SKILL.md
status: stable-reference
---

# /slo-sast Custom Rule Shape — empirical guidance

`/slo-sast` selects registry rules by CWE. When the threat model needs coverage the registry
does not provide, project-specific rules are authored (hand-written, or via `/slo-rulegen` for
Rust). This reference captures what an adversarial NodeGoat + Juice Shop tuning exercise proved
about rule shape — recall roughly doubled (33% → 63% on NodeGoat) and the same unmodified pack
hit 8 exact Juice Shop ground-truth sinks. Apply this shape to any hand-authored web pack.

## 1. Default to taint mode, not literal-sink patterns

The dominant false-negative driver for web rules is matching a literal request access *at the
sink* (`eval(req.body.x)`). Real apps extract `req.*` in the route then reach the sink through a
local variable, **object destructuring**, or another module (route → DAO/service). A literal-sink
rule misses every such flow: NoSQL `$where`, SSRF, ReDoS, log injection, stored-XSS-via-autoescape
were all missed this way.

Mandatory shape for request-data-flow classes:

- `mode: taint`
- `pattern-sources`: include **both** the member forms (`req.body.$X`, `req.query.$X`,
  `req.params.$X`) **and the whole objects** (`req.body`, `req.query`, `req.params`). The
  whole-object source is what makes the propagator below fire.
- `pattern-sinks`: the dangerous call only.
- `pattern-sanitizers`: the real neutralizers (`String(...)`, `parseInt`, `Number`, `ObjectId`,
  `path.basename`, `encodeURIComponent`, framework escapers).

## 2. Always add the destructuring propagator

`const { userName } = req.body` is the dominant Express input-extraction idiom. Without a
propagator, taint dies at the destructure. Add to every taint rule:

```yaml
pattern-propagators:
  - pattern: "const { $TO } = $FROM"
    from: $FROM
    to: $TO
  - pattern: "let { $TO } = $FROM"
    from: $FROM
    to: $TO
  - pattern: "var { $TO } = $FROM"
    from: $FROM
    to: $TO
```

In the exercise this single addition flipped log-injection and IDOR from miss to hit.

### Per-language input-propagator (the generic form)

The *technique* (taint + a framework-input propagator) is language-agnostic; only the
**propagator pattern** is per-stack. Author the equivalent for the detected language:

| Stack | Request source | Propagator idiom to add |
|---|---|---|
| Express/Koa (JS/TS) | `req.body/query/params`, `ctx.request.body` | `const { $T } = $F` destructure |
| NestJS/Spring/.NET | DTO bound param (`@Body`/`@RequestBody`/`[FromBody]`) | param → field access; treat the DTO param as a source |
| Django/DRF | `request.GET/POST/data`, `serializer.validated_data` | `$T = request.data.get(...)` / `$T = serializer.validated_data[...]` |
| Flask/FastAPI | `request.args/form/json`, path/query func args | function-arg source; `$T = request.json[...]` |
| Rails/Laravel | `params[:x]`, `$request->input('x')` | `$T = params[$K]` / `$T = $request->$M(...)` |
| Go | `r.URL.Query().Get`, `chi.URLParam`, `c.Param` | `$T := r.URL.Query().Get($K)` source |

Same rule shape, swapped propagator. The class spine is shared — see
[`../../../references/security/vuln-class-taxonomy.md`](../../../references/security/vuln-class-taxonomy.md);
every rule's `metadata` MUST use a `vuln_class` from it so the SAST→DAST bridge can consume it.

## 3. Keep intrinsic-sink structural rules for flow-free classes

Some classes are dangerous regardless of flow and need no taint: Mongo `$where` with any
non-constant value, catastrophic-backtracking regex literal `\([^)]*[+*]\)[+*]`, template engine
`autoescape:false`, `helmet` absent from an `express()` file, MD5/SHA1 password hashing,
hardcoded private key / HMAC secret. Author these as plain `pattern`/`patterns` rules.

## 4. Split DB-query sinks by driver and emit the matching CWE

A generic `$C.find($SINK)` sink matches Mongo *and* Sequelize/Knex/pg. On Juice Shop this
labelled Sequelize SQLi as NoSQL — right risk class, wrong CWE, ~36 partial false positives.
Author **separate** sinks per driver family (Mongo/Mars → CWE-943; Sequelize/Knex/pg/raw SQL →
CWE-89) so the finding's CWE is correct and audit-defensible.

## 5. Constrain heuristic sinks by receiver name

Broad sinks over-fire. Example: an IDOR rule whose sink is `$DAO.$M(reqId, ...)` matched
`needle.get(url)`. Constrain with `metavariable-regex` on the receiver
(`(?i).*(dao|model|repo|repository|collection|db|store|service)`). This removed the false
positive without losing the true IDOR. Model authorization both ways: exclude handlers that
reference `req.session`/`req.user` **and** (follow-up) JWT/`isAuthorized()` middleware.

## 6. Default security-profile excludes (false-positive control)

Context-blind file matching was the dominant baseline false-positive driver (`plaintext-http-link`
on teaching HTML, `detected-bcrypt-hash` on seed fixtures). The emitted `.semgrep.yml` /
scan invocation SHOULD exclude by default: `**/views/**`, `**/templates/**`, `**/tutorial/**`,
`**/docs/**`, `**/artifacts/**`, `**/fixtures/**`, `**/*fixture*`, `**/seed*/**`, `**/test/**`,
`**/build/**`, `**/dist/**`, `**/node_modules/**`. Document deviations in the M4 manifest.

## 7. State the coverage caveats explicitly

- **Intra-file taint only** (Semgrep OSS): route→DAO cross-file flows (plaintext password / PII
  persisted in a different module) stay false-negative. Recommend interprocedural taint or a
  manual review note for CWE-256/312 when source and sink are in different files.
- **No SCA**: the vulnerable-and-outdated-components class (CWE-1035 / CWE-937, e.g.
  `marked@0.3.5`) is unreachable by Semgrep patterns. `/slo-sast` MUST recommend a paired SCA
  step and not imply dependency coverage.

## Pack layout (additive per language)

Organize as `.semgrep/<lang>/<class>/` (mirrors `semgrep/semgrep-rules` and eases upstreaming):
e.g. `.semgrep/node/injection.yaml`, `.semgrep/python/injection.yaml`. New language packs are
additive — same rule shape, per-language propagator, shared `vuln_class` taxonomy — never a
skill rewrite. This mirrors the DAST side's per-framework resolver-adapter catalog: one
invariant contract, pluggable language/framework parts.

## Reference pack

A vetted generic Node/Express + Angular starter pack built to this shape (taint + propagator,
driver-split intent, intrinsic-sink rules, receiver-constrained IDOR) is the recommended
starting point; see the SAST/DAST adversarial tuning exercise deliverables.
