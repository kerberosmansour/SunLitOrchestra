---
name: slo-dast-tuner-entry-point-map-contract
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Entry-point map contract (protocol-abstract)

The stable interface between the SAST→DAST bridge and the scan step. It is **not** HTTP-specific
— "route" is one concretization of an *entry point*. A resolver adapter produces this; the scan
step consumes it. The schema does not change per framework; only the adapter that fills it does.

## Schema

```json
{
  "schema_version": "2.0",
  "mode": "agent-resolved-guided",
  "base_url": "http://host.docker.internal:4000",
  "targets": [
    {
      "source": "app/routes/research.js:16",
      "rule": "node-ssrf-taint",
      "vuln_class": "ssrf",                 // from vuln-class-taxonomy.md
      "protocol": "http",                   // http | graphql | grpc | event | serverless
      "selector": {                         // protocol-specific addressing
        "method": "GET",                    // http verb | graphql op | grpc method | event type
        "path": "/research",                // http path-template | graphql field | service.Method | source ARN
        "param": "url"                      // injectable input name when known
      },
      "address": "http://host.docker.internal:4000/research",
      "requires_auth": true,
      "role": "user",                       // null | user | admin | scope name
      "auth_kind": "session",               // session | bearer-jwt | api-key | mtls | none
      "probe": "ssrf",                      // probe-library.md key (== vuln_class default)
      "dast_verdict": "dast-detectable"     // from vuln-class-taxonomy.md verdict vocabulary
    }
  ],
  "unresolved": [
    {"source": "server.js:15", "vuln_class": "missing-headers",
     "reason": "app-wide config, no entry point", "dast_verdict": "needs-human-input"}
  ]
}
```

## Protocol concretizations

| `protocol` | `selector.method` | `selector.path` | `address` | probe transport |
|---|---|---|---|---|
| `http` | GET/POST/… | path-template (`/u/:id`, `/u/{id}`) | base+path | HTTP request |
| `graphql` | query/mutation/subscription | `Type.field` | base + `/graphql` | GraphQL document |
| `grpc` | unary/stream | `package.Service/Method` | host:port | grpc call / grpcurl |
| `event` | event type | source (queue/topic/bucket ARN) | broker ref | publish a crafted event |
| `serverless` | trigger | function name + event source | invoke ARN/URL | trigger via its event source |

## Invariants (every adapter, every protocol)

1. **Never invent an address.** If the resolver cannot bind a sink to a real entry point, it
   goes in `unresolved` with `needs-human-input` — not a guessed path.
2. **`vuln_class` and `dast_verdict` come from `vuln-class-taxonomy.md`** — no synonyms.
3. **`requires_auth`/`auth_kind` are mandatory when true.** The scan step uses them to satisfy
   `authentication-coverage.md`; an unauthenticated scan of `requires_auth:true` targets is a
   coverage failure, not a clean result.
4. **App-wide config sinks** (`missing-headers`, `cleartext-transport`, `secret`, `weak-crypto`)
   have no single entry point → always `unresolved`, handled by the passive pass, never guessed
   onto a route.
5. The bridge **enriches targeting only**. A resolved target is a place to test; confirmation
   still requires a runtime oracle from `probe-library.md`.
