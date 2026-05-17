---
name: slo-dast-tuner-probe-library
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Probe / oracle library (keyed by `vuln_class`)

Protocol-level payload + oracle per class. Reused across **every** framework adapter — payloads
are HTTP/wire-level, not framework-level. The resolver picks the probe from the target's
`vuln_class`; the scan step authenticates first if `requires_auth`.

Safety: probes target a local, authorized, deliberately-vulnerable app. SSRF points at the
target's own loopback (no external/metadata exfiltration). DoS-class probes (`redos`,
deserialization bombs) run **last** — a confirmed event-loop block starves later probes.

| `vuln_class` | Payload (HTTP) | Oracle |
|---|---|---|
| `sqli` | `' OR '1'='1' -- `, `1 UNION SELECT NULL-- `, boolean `1 AND 1=1` vs `1 AND 1=2` | SQL error string, row-count/response diff, UNION reflection |
| `nosqli` | param as `{"$gt":""}` / `{"$ne":null}`; string `' || '1'=='1` | result-set size change, auth bypass, 500 on broken `$where` |
| `code-injection` | arithmetic `7*7`→49; blind: `sleep(5)`/long loop | reflected computed value, or response-time delta (blind) |
| `os-command` | `; sleep 5`, `| id`, `$(sleep 5)`, OOB DNS canary | response-time delta, OOB callback, command output reflected |
| `ssti` | `${7*7}`, `{{7*7}}`, `<%= 7*7 %>`, `#{7*7}` | `49` rendered in response |
| `ssrf` | `url=http://127.0.0.1:<self>/`, `…/latest/meta-data/`, OOB host | fetched body reflected, or OOB hit; latency on internal-only host |
| `path-traversal` | `../../../../etc/passwd`, `%2e%2e%2f`, `..%00.md` | file content / 200 on non-allowlisted path |
| `open-redirect` | `to=//evil.example`, `=https://evil.example/` | 30x `Location:` to attacker host (no-follow client) |
| `xss-reflected` | `"><svg onload=…>` unique marker | marker reflected unescaped in HTML/JS context |
| `xss-stored` | unique marker in a write endpoint, then GET the render endpoint | marker executes/reflected on the read path |
| `xss-dom` | marker via fragment/param consumed by a client sink | requires **PTK / DOM lane** (see `ptk-dom-xss.md`) — not visible to web-pr |
| `xxe` | `<!DOCTYPE x [<!ENTITY e SYSTEM "file:///etc/passwd">]>` / OOB | entity content in response/error, or OOB fetch |
| `deserialization` | language-specific gadget marker; run **last** (DoS-prone) | exception class leak, sleep gadget timing, OOB |
| `idor` | enumerate the id/selector across two authenticated identities | identity-B reads identity-A's record (body differs, 200) |
| `missing-authz` | call the state-changing entry point **without** credentials | 2xx / effect without auth |
| `open passive` (`missing-headers`,`cleartext-transport`) | none | passive: inspect response headers / scheme |
| `log-injection`,`secret`,`weak-crypto`,`sensitive-store` | — | **no black-box oracle** → `sast-only`, do not probe |
| `vulnerable-dependency` | — | SCA, retire.js (frontend) → `sca-only` |

## Non-HTTP delivery (class transfers, delivery differs)

- **graphql**: injection/idor/authz via a GraphQL `query`/`mutation` document to `/graphql`;
  introspection for schema; alias-based batching for enumeration. Same classes, GraphQL body.
- **grpc**: payload in the protobuf message via `grpcurl`; sqli/idor/authz still apply; no
  reflected-HTML XSS class.
- **event/serverless**: publish a crafted event to the source (queue/topic/bucket); oracle is
  the downstream effect/log, often `dast-partial` or OOB-only.

## Discipline

- Confirmation requires the oracle to fire — a resolved target alone is not a finding.
- Blind classes stay `dast-partial`; never upgrade to confirmed without a real signal.
- `sast-only`/`sca-only`/`needs-human-input` are never "probed into" a confirmed result.
