# Proactive controls vocabulary — `/slo-plan`

This file is the canonical vocabulary for two fields in every milestone's Contract Block:

1. **Data classification** — fixed four-value enum.
2. **Proactive controls in play** — stack-aware vocabulary. Rust-axum targets cite SunLitSecurityLibraries crate names; Pulumi/AWS targets cite Hulumi component names; other stacks cite OWASP Proactive Controls v3 category names directly.

Values here are Markdown-literal text. They are substituted verbatim into runbook Contract Block tables by `/slo-plan`. **They are never interpolated into shell commands, never invoked as subprocesses, never used as identifiers in generated code** — the contract is read by `/slo-execute` + `/slo-critique` + `/slo-verify` as prose, not executed as logic. This defense-in-depth rule means a malicious or unwary vocabulary value with shell metacharacters (semicolons, backticks, `$()`) cannot compromise the skill pipeline.

## Data classification (fixed enum — do not extend without a schema migration)

| Value | Definition | Examples |
|---|---|---|
| `Public` | Anyone may read; no authorization required; no restrictions on where this data travels. | Marketing copy; open-source release notes; publicly-listed service health. |
| `Internal` | Authenticated org members only; includes engineering telemetry, internal tooling UX state. Leak is embarrassing but not breach-triggering. | Skill-pack install logs; non-PII telemetry; runbook drafts. |
| `Confidential` | Business-sensitive; access controlled by role; breach has business impact (lost deals, reputation). | Source code for non-OSS projects; private threat models; customer lists. |
| `Restricted` | Regulated / PII / secrets / crypto material; access controlled by strict role + auditing; breach triggers regulatory notification. | Customer PII; passwords and tokens; encryption keys; cardholder data; PHI. |

A milestone that handles ≥ `Confidential` data MUST:

- Cite at least one control from `secure_data` (or the stack equivalent) in the Proactive controls row.
- Include at least one abuse-case BDD scenario covering the disclosure surface.

A milestone handling `Restricted` data MUST additionally:

- Set the `compliance:` frontmatter to the applicable framework list (e.g. `[soc2, asvs, hipaa]`).
- Produce an abuse-case scenario per STRIDE category — Spoofing, Tampering, Information Disclosure are all live surfaces.

## Proactive controls vocabulary — Rust-axum (SunLitSecurityLibraries)

When the target stack is Rust + axum and `security_libs_required: true`, the Proactive controls row cites crate names from SunLitSecurityLibraries. Each maps to OWASP Proactive Controls C-numbers.

| Control | OWASP C# | SunLitSecurityLibraries crate / type | Use when the milestone |
|---|---|---|---|
| **Input validation** | C5 | `secure_boundary::extract::SecureJson<T>`; `safe_types::{SafePath,SafeUrl,SafeFilename,SafeCommandArg,SqlIdentifier,SafeRedirectUrl,LdapSafeString}` | introduces any new request body, URL-from-user-input, file path, shell arg, or SQL identifier |
| **Output encoding** | C4 | `secure_output::{HtmlEncoder,UrlEncoder,JsStringEncoder,CssEncoder,XmlEncoder,ldap::{encode_dn,encode_filter},shell::encode,sanitize_uri_scheme}` | renders user data into HTML / URL / JS / CSS / XML / LDAP / shell |
| **Identity / auth** | C6 | `secure_identity` (JWT, OIDC, session, TOTP, biometric) | introduces any authentication surface |
| **Authorization** | C7 | `secure_authz::{DefaultAuthorizer,AuthzLayer,PermissionWindow,AttributeGuard}` | introduces any permission check, RBAC role, ABAC attribute, or tenant isolation |
| **Data protection** | C2 / C8 | `secure_data::{envelope::encrypt_for_storage,password::hash_password,SecretString}` | stores secrets, encrypts at rest, handles passwords, or routes data through a KMS |
| **Errors** | C10 | `secure_errors::{http::into_response_parts,panic::catch_panic_to_safe_response,kind::AppError}` | introduces any error path crossing a trust boundary |
| **Logging and monitoring** | C9 | `security_events::{SecurityEvent,DataClassification,Sink}` | records security-relevant state changes (auth, authz decisions, secret access) |
| **Security requirements** | C1 | this file + `SECURITY.md` + `docs/slo/design/<slug>-threat-model.md` | is structural (the requirement is recorded once; every milestone references it) |
| **Frameworks and libraries** | C2 | any of the above crates — avoid hand-rolled crypto / input-validation / authz | always applies when the vocabulary above is available |
| **Database access** | C3 | `sqlx` + parameterized queries only; never string-interpolate SQL | introduces database reads or writes |

## Proactive controls vocabulary — Pulumi / AWS (Hulumi)

When the target stack is Pulumi + AWS and `security_libs_required: true`, cite Hulumi component names directly.

| Control | Hulumi component | Use when the milestone |
|---|---|---|
| **Hardened bucket** | `@hulumi/baseline.aws.SecureBucket` | provisions any S3 bucket |
| **Account foundation** | `@hulumi/baseline.aws.AccountFoundation` | bootstraps a new AWS account with CIS v5 controls |
| **Hardening pack** | `HulumiHardeningPack` CrossGuard policy pack | applies account-wide policy enforcement |
| **Drift classification** | `@hulumi/drift` | validates deployed infra against declared state |
| **IaC role tagging** | `hulumi:iac-role=true` tag | any IaC execution role |

Stack-specific invariant: Pulumi/AWS milestones never commit secrets to state files. Use AWS Secrets Manager or SSM Parameter Store, referenced by ARN.

## Proactive controls vocabulary — other stacks (OWASP Proactive Controls v3)

When `security_libs_required: false` OR the stack has no equivalent library ecosystem, cite OWASP Proactive Controls v3 category names directly. Reference: https://owasp.org/www-project-proactive-controls/

- **C1 Define Security Requirements** — SECURITY.md + threat model cited above.
- **C2 Leverage Security Frameworks and Libraries** — name the language-native libraries (e.g., Go's `crypto/subtle` for constant-time comparison; Python's `cryptography` library; Node's `node:crypto`).
- **C3 Secure Database Access** — prepared statements, named ORM (e.g., `sqlx`, `sqlalchemy`, `prisma`).
- **C4 Encode and Escape Data** — name the encoding library (e.g., `html/template` in Go).
- **C5 Validate All Inputs** — allow-list schema validation (e.g., `serde` with `deny_unknown_fields`, `pydantic`, `zod`).
- **C6 Implement Digital Identity** — name the auth library (e.g., Keycloak, Auth0, Firebase Auth).
- **C7 Enforce Access Controls** — deny-by-default RBAC + ABAC.
- **C8 Protect Data Everywhere** — TLS in transit; at-rest encryption via KMS.
- **C9 Implement Security Logging and Monitoring** — structured logs, auth-event audit.
- **C10 Handle All Errors and Exceptions** — boundary catches; no detail leakage.

## Writing a Proactive-controls row in a Contract Block

**Rust-axum milestone that adds a new POST endpoint accepting user input:**

```markdown
| Proactive controls in play | `C5 secure_boundary::SecureJson` (strict deserialization, field cap); `C10 secure_errors::http::into_response_parts` (no detail leakage); `C9 security_events::SecurityEvent` (auth-event logging) |
```

**Pulumi/AWS milestone that adds an S3 bucket:**

```markdown
| Proactive controls in play | `@hulumi/baseline.aws.SecureBucket` (public-access block, encryption at rest, versioning); `HulumiHardeningPack` CrossGuard enforcement on the account |
```

**Markdown-only milestone (this runbook's M1–M4):**

```markdown
| Proactive controls in play | `C1 Define Security Requirements` — this milestone edits SKILL.md contracts; the only surface is file I/O of Markdown files, already covered by the existing `sldo-install` path-safety rules. No new trust boundaries introduced. |
```

## Writing a Data-classification row

```markdown
| Data classification | `Internal` — the milestone edits skill-pack prompt files and writes test files. No PII, no secrets, no customer data. |
```

```markdown
| Data classification | `Restricted` — the milestone handles customer authentication tokens and PII on the `/login` endpoint. SOC 2 and GDPR apply; runbook frontmatter should set `compliance: [soc2, asvs, gdpr]`. |
```

## Anti-patterns

- **Free-form data classifications.** The enum is fixed: Public / Internal / Confidential / Restricted. "Medium confidential," "critical PII," "yellow-level secrets" — all rejected. Use the enum.
- **Leaving Proactive controls blank.** Every milestone introduces some surface (even if it's just file I/O); cite at least one C-number with an explanation. If truly nothing applies, use the "milestone introduces no new surface" N/A phrasing — not blank.
- **Coining new C-numbers.** The vocabulary is fixed at C1–C10 plus SunLitSecurityLibraries crate names. Invented names drift across runbooks and defeat the point.
- **Treating this file as executable.** Proactive-controls row values are Markdown prose read by humans and by downstream skills. They are never spliced into shell, never invoked as subprocess commands, never used as code identifiers.
