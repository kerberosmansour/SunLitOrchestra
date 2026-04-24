# Bug-class catalog — `/slo-critique`

Organized by OWASP ASVS 5.0 chapter (V1–V17). Each chapter lists 2–4 named bug classes with one elimination pattern per class. Where the elimination pattern is already implemented in SunLitSecureLibraries (Rust-axum) or Hulumi (Pulumi/AWS), the crate / component is cited. Otherwise the pattern is described in language-agnostic terms with a pointer to OWASP Proactive Controls v3.

A **class** is architecturally eliminated when the pattern makes every instance impossible by construction. A **class** is mitigated when instances are bounded but possible. The security persona uses this distinction in every finding.

## V1 — Encoding and Sanitization

| Bug class | Elimination pattern | SunLitSecureLibraries / Hulumi |
|---|---|---|
| HTML injection / reflected XSS | Context-aware output encoding at rendering boundary | `secure_output::HtmlEncoder` |
| URL open-redirect | Relative-path-only redirect type | `secure_boundary::safe_types::SafeRedirectUrl` |
| LDAP injection | RFC 4515 escaping at boundary | `secure_output::ldap::encode_filter` |
| Shell metachar injection | Single-quote escape + single-arg `Command::arg` | `secure_output::shell::encode` + arg-typed `Command` |

## V2 — Session Management

| Bug class | Elimination pattern | Library |
|---|---|---|
| Session fixation | Rotate session id on auth state change | `secure_identity::session` with rotation policy |
| Session hijacking via insecure cookie | `Secure`, `HttpOnly`, `SameSite=Strict` default | `secure_boundary::headers::SecurityHeadersLayer` |
| Unbounded session lifetime | Idle + absolute timeout policy | `secure_identity::PermissionWindow` + session TTL |

## V3 — Access Control

| Bug class | Elimination pattern | Library |
|---|---|---|
| IDOR / broken object-level authz | Deny-by-default authorizer checking every action | `secure_authz::DefaultAuthorizer` + `AuthzLayer` |
| Cross-tenant access | Tenant-scoped `ResourceRef` required on every check | `secure_authz::resource::ResourceRef::with_tenant` |
| Privilege escalation via state poisoning | Temporal permission windows revalidated per request | `secure_authz::PermissionWindow` |
| Missing function-level authorization | Middleware enforces authz on every route | `AuthzLayer` wrapping the router |

## V4 — Input Validation

| Bug class | Elimination pattern | Library |
|---|---|---|
| SQL injection | `SqlIdentifier` safe type; parameterized queries only | `secure_boundary::safe_types::SqlIdentifier` |
| Path traversal | Path type that rejects `../` and symlinks | `secure_boundary::safe_types::SafePath` |
| Mass assignment | DTO-only writes with `deny_unknown_fields` | `secure_boundary::extract::SecureJson<T>` |
| Deserialization bomb | Bounded JSON nesting depth + field count | `secure_boundary::RequestLimits` |
| SSRF | Allow-list-based URL type rejecting private IPs | `secure_boundary::safe_types::SafeUrl` |
| CRLF header injection | `\r` / `\n` rejection in header-value constructors | `secure_boundary::header_sanitize::sanitize_header_value` |

## V5 — Output Encoding and Injection Prevention

(See V1 — same concerns, complementary framing.) Key additional classes:

| Bug class | Elimination pattern | Library |
|---|---|---|
| Stored XSS via user markup | HTML sanitizer with explicit allow-list | `secure_boundary::sanitize::sanitize_html` |
| JS-string injection in server-rendered templates | Context-specific JS string encoder | `secure_output::JsStringEncoder` |
| CSS expression injection | CSS hex-encoder for value positions | `secure_output::CssEncoder` |

## V6 — Cryptography

| Bug class | Elimination pattern | Library |
|---|---|---|
| Plaintext secret in `Debug`/`Display` | Secret types that redact automatically | `secure_data::secret::SecretString` |
| Insecure symmetric algorithm | Algorithm policy object + envelope encryption | `secure_data::envelope::encrypt_for_storage` |
| Weak password hashing | Argon2id with enforced minimum params | `secure_data::password::hash_password` (`password` feature) |
| Key-management sprawl | KMS-backed envelope with centralized key provider | `secure_data::kms::{VaultKeyProvider, AwsKmsKeyProvider}` |
| Static per-request nonce (AEAD) | Random nonce per call, never caller-supplied | `secure_data::envelope::*` (internal) |

## V7 — Error Handling and Logging

| Bug class | Elimination pattern | Library |
|---|---|---|
| Internal-detail error leakage at trust boundary | Three-layer error model with public/private split | `secure_errors::http::into_response_parts` |
| Panic leaking to client | Panic catch at service boundary | `secure_errors::panic::catch_panic_to_safe_response` |
| Log injection via CRLF in user strings | Structured events with classification-based redaction | `security_events::SecurityEvent` + log-injection prevention |
| Missing audit trail for security events | Per-event HMAC sealing + NDJSON sink | `security_events::Sink` |

## V8 — Data Protection

| Bug class | Elimination pattern | Library |
|---|---|---|
| PII at rest unencrypted | Envelope encryption with KMS-wrapped data keys | `secure_data::envelope::encrypt_for_storage` |
| Sensitive buffer leaked via swap | `SensitiveBuffer` with `mlock`-style pinning (mobile) | `secure_data` `mobile-storage` feature |
| Backup exfiltration of secrets | Secrets excluded from platform backup | `secure_data::BackupExclusion` |

## V9 — Communications

| Bug class | Elimination pattern | Library |
|---|---|---|
| TLS downgrade | Minimum TLS 1.2 enforced; 1.3 preferred | `secure_network::TlsPolicy` |
| Cert-pinning bypass | SPKI SHA-256 pin validation | `secure_network::cert_pin` |
| Cleartext traffic on mobile | Detection + refusal | `secure_network::cleartext` |

## V10 — Malicious Code (Supply Chain)

| Bug class | Elimination pattern | Library / tool |
|---|---|---|
| Vulnerable dependency via CVE | CI-gated `cargo audit` | `cargo-audit` (Phase 3 — see `/slo-security-test`) |
| License violation | `cargo deny check` with explicit allow-list | `cargo-deny` |
| Unvetted transitive dep | `cargo vet` audit trail | `cargo-vet` |
| Git dependency from unknown source | `cargo deny` registry allow-list | `cargo-deny` sources config |

## V11 — Business Logic

| Bug class | Elimination pattern | Library |
|---|---|---|
| Race condition in multi-step flow | Temporal `PermissionWindow` + optimistic locking | `secure_authz::PermissionWindow` + db version col |
| Workflow skip via state manipulation | State-machine-encoded status checks | language-specific (no single crate) |
| Rate-limit bypass via distributed clients | Token-bucket at API gateway | application-level (Redis bucket) |

## V12 — Files and Resources

| Bug class | Elimination pattern | Library |
|---|---|---|
| Zip-slip / archive path escape | Prefix check before `fs::write`; `SafePath` on extracted entries | `secure_boundary::safe_types::SafePath` |
| Unbounded upload size | Body size cap via `RequestLimits` | `secure_boundary::RequestLimits` |
| Arbitrary file overwrite via user path | Restrict writes to pre-declared directories | language-specific |

## V13 — API and Web Service

| Bug class | Elimination pattern | Library |
|---|---|---|
| Unrestricted CORS | Explicit origin allow-list | `secure_boundary::cors::SecureCorsBuilder` |
| Missing Fetch Metadata checks | Layer rejects cross-origin navigations | `secure_boundary::fetch_metadata::FetchMetadataLayer` |
| GraphQL query-depth DoS | Query complexity cap + depth limit | application-level + `RequestLimits` |

## V14 — Configuration

| Bug class | Elimination pattern | Library / Hulumi |
|---|---|---|
| Public S3 bucket | Hardened default with public-access block | `@hulumi/baseline.aws.SecureBucket` |
| Overly-permissive IAM role | CIS-aligned account foundation | `@hulumi/baseline.aws.AccountFoundation` |
| Missing security headers | Headers layer with HSTS/CSP/XFO defaults | `secure_boundary::headers::SecurityHeadersLayer` |
| Secret in env file committed | `.gitignore` + SECURITY.md policy | meta — operational |

## V15 — Web Service (SOAP, WebSocket, gRPC)

Narrower scope; often N/A in modern HTTP-only services. When applicable:

| Bug class | Elimination pattern | Library |
|---|---|---|
| XXE in SOAP / XML | DOCTYPE / entity declaration rejection | `secure_boundary::xml::SecureXml` |
| WebSocket origin confusion | Origin allow-list at handshake | application-level |

## V16 — SPA / Client-side

Often N/A for server-side runbooks. When applicable:

| Bug class | Elimination pattern |
|---|---|
| DOM-based XSS | Framework-enforced templating with context-aware escaping (React, Vue) |
| Client-side open-redirect | `SafeRedirectUrl` at POST-login redirect boundary |
| Session-token exposure to JS | `HttpOnly` cookie + token-binding |

## V17 — Architecture, Design, and Threat Modeling

| Bug class | Elimination pattern |
|---|---|
| Missing threat model | `/slo-architect` Step 3.5 generates `docs/design/<slug>-threat-model.md` |
| No explicit trust boundaries | ARCHITECTURE.md diagram requires labeled boundaries |
| Tight coupling between services exposes lateral movement | Segmentation at IAM layer (for AWS) or network layer |
| Missing security requirements | `SECURITY.md` at repo root (generated by `/slo-architect`) |

## Using this catalog in a finding

Every accepted finding names a class from this catalog (e.g., **V4 SQL injection**), cites the threat-model row (e.g., `tm-<slug>-abuse-3`), answers whether the class is eliminated or only mitigated, and provides a variant-analysis pointer (see [`variant-analysis-playbook.md`](variant-analysis-playbook.md)).

Example:

> **Finding**: V4 SQL injection via `/api/orders` — user-controlled `sort_by` parameter is string-interpolated into an ORDER BY clause.
>
> **Class status**: not eliminated. The pattern is possible because the query uses dynamic ORDER BY without a safe-identifier type.
>
> **Threat-model row**: tm-foo-abuse-3 (Tampering cell for `orders-service` component).
>
> **Elimination path**: replace dynamic ORDER BY with `SqlIdentifier::try_from(sort_by.as_str())?` which rejects any non-alphanumeric input.
>
> **Variant-analysis pointer**: `rg -n "format!.*ORDER BY"` across `services/*/src/**` surfaces three other sites; see playbook.

## Anti-patterns

- **Coining new bug classes outside ASVS 5.0.** If a vulnerability doesn't map to an ASVS chapter, the classification probably isn't ready — go find the existing chapter or add to V17 (Architecture) with justification.
- **Listing a class with no concrete surface.** A catalog entry is not a finding. Every finding points at a specific line/file/handler. Use the catalog to *name* the class; use the plan to show *where*.
- **"Maybe" class status.** Every finding answers eliminated / mitigated / residual. "Possibly present" is rejected — the reviewer must commit.
