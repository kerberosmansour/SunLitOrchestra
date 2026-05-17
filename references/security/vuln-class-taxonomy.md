---
name: vuln-class-taxonomy
status: stable-reference
shared_by: [skills/slo-sast, skills/slo-dast-tuner]
---

# Vulnerability-class taxonomy (the SAST↔DAST spine)

This is the **single lingua franca** between `slo-sast` (which emits a class per finding)
and `slo-dast-tuner` (which picks a probe per class). Every custom Semgrep rule, every
resolver adapter, and every DAST probe keys off these exact `vuln_class` slugs. Do not invent
synonyms; extend this list with a PR if a class is genuinely missing.

| `vuln_class` | Primary CWE(s) | Typical SAST signal | DAST observability |
|---|---|---|---|
| `sqli` | CWE-89 | string-built SQL into a driver/ORM raw query | dast-detectable (error/UNION/boolean) |
| `nosqli` | CWE-943 / CWE-94 | `$where`/operator object from request | dast-detectable / partial |
| `code-injection` | CWE-94 / CWE-95 | `eval`/`Function`/`vm`/`ScriptEngine` on request | dast-partial (often blind) |
| `os-command` | CWE-78 | `exec`/`spawn`/`Runtime.exec`/`os.system` on request | dast-detectable (time/oob) |
| `ssti` | CWE-1336 / CWE-94 | request into template compile/render string | dast-detectable (polyglot) |
| `ssrf` | CWE-918 | request URL into outbound HTTP client | dast-detectable (reflection/oob) |
| `path-traversal` | CWE-22 / CWE-23 | request into fs path / sendFile | dast-detectable |
| `open-redirect` | CWE-601 | request into redirect/Location | dast-detectable (30x) |
| `xss-reflected` | CWE-79 | request echoed unescaped in response | dast-detectable |
| `xss-stored` | CWE-79 | request persisted then rendered unescaped | dast-partial (plant+observe) |
| `xss-dom` | CWE-79 | client sink (`innerHTML`, trustHtml bypass) | dast-detectable via DOM/PTK only |
| `xxe` | CWE-611 / CWE-776 | XML parser with entity expansion on request | dast-detectable (oob/error) |
| `deserialization` | CWE-502 | untrusted input to unsafe deserializer | dast-partial |
| `idor` | CWE-639 / CWE-284 | request id → data access w/o ownership/authz | dast-detectable (multi-user) |
| `missing-authz` | CWE-862 / CWE-285 | state-changing route w/o auth guard | dast-detectable (unauth call) |
| `log-injection` | CWE-117 | request into log sink w/o CRLF neutralization | sast-only |
| `secret` | CWE-798 / CWE-321 | hardcoded key/credential literal | sast-only |
| `weak-crypto` | CWE-327 / CWE-916 | MD5/SHA1/ECB/static-IV/weak KDF | sast-only |
| `sensitive-store` | CWE-312 / CWE-256 | PII/password persisted unhashed/unencrypted | sast-only |
| `missing-headers` | CWE-693 / CWE-1021 | no CSP/HSTS/frameguard/nosniff config | dast-detectable (passive) |
| `cleartext-transport` | CWE-319 | plaintext HTTP server / no TLS | dast-detectable (passive) |
| `vulnerable-dependency` | CWE-1035 / CWE-937 | known-vulnerable package version | SCA-only (neither pure SAST nor DAST) |

## Verdict vocabulary (used by the entry-point map)

- `dast-detectable` — a reliable black-box runtime oracle exists.
- `dast-partial` — reachable but blind/state-gated; needs a tuned oracle.
- `sast-only` — no black-box oracle (server-log, at-rest, code-literal).
- `sca-only` — needs a dependency scanner, not SAST or DAST.
- `needs-human-input` — entry point, auth, or exploitability unresolved.

## Rule

A `vuln_class` of `sast-only`/`sca-only`/`needs-human-input` must **never** be reported as a
confirmed DAST finding, and a resolver must **never** invent an entry point to make one
"testable". Honesty over coverage — this is the same discipline as `triage-sarif`.
