# API Facade Gateway — api-facade (AI-First Runbook v2)

> **Purpose**: Build a production-grade Rust web API that acts as a secure facade/proxy between internal clients and third-party APIs (e.g., OpenAI), fetching API keys from a secret store so clients never handle third-party credentials directly.  
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.  
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [README.md](../README.md)

---

## Runbook Metadata

- **Runbook ID**: `api-facade`
- **Prefix for test files and lessons files**: `api-facade`
- **Primary stack**: `Rust + Axum + Tokio + Reqwest`
- **Primary package/app names**: `api-facade-gateway`
- **Default test commands**:
  - Backend: `cargo test --workspace`
  - Frontend: `N/A`
  - E2E backend: `cargo test --test 'e2e_api_facade_*' -- --test-threads=1`
  - E2E frontend: `N/A`
  - Build/boot: `cargo build --release && ./target/release/api-facade-gateway`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `POST /v1/proxy/{provider}/{endpoint}` — primary facade route
  - `GET /health` — health check endpoint
  - `GET /ready` — readiness probe

---

## Threat Model

### System Overview

This API facade sits between trusted internal clients and untrusted third-party APIs. Clients authenticate to the facade; the facade injects the real API key from a secret store and proxies the request to the third party. The primary security goal is: **third-party API keys never leave the server, and only authorized clients can trigger API calls.**

### Trust Boundaries

```
┌─────────────────────┐
│   Internal Client    │  Trust Boundary 1: Client ↔ Facade
│  (browser, service)  │  - Client presents its own credential (JWT / API key)
└────────┬────────────┘  - TLS terminates here
         │ HTTPS (mTLS optional)
         ▼
┌─────────────────────┐
│   API Facade         │  Trust Boundary 2: Facade ↔ Secret Store
│   Gateway (Rust)     │  - Facade authenticates to secret store
│                      │  - Secrets fetched at startup + cached with TTL
│   ┌──────────────┐   │
│   │ Auth Layer   │   │  Trust Boundary 3: Facade ↔ Third-Party API
│   │ Rate Limiter │   │  - Facade injects API key into outbound request
│   │ Audit Logger │   │  - Response is sanitized before returning to client
│   │ Proxy Core   │   │
│   └──────────────┘   │
└────────┬──────┬──────┘
         │      │
         ▼      ▼
┌────────────┐ ┌──────────────────┐
│ Secret     │ │ Third-Party API  │
│ Store      │ │ (OpenAI, etc.)   │
│ (Vault /   │ └──────────────────┘
│  AWS SM /  │
│  env file) │
└────────────┘
```

### STRIDE Threat Analysis

| Threat Category | Threat | Component | Severity | Mitigation |
|---|---|---|---|---|
| **Spoofing** | Attacker impersonates a legitimate client | Auth Layer | Critical | JWT validation with asymmetric keys (RS256/ES256) or HMAC API keys with constant-time comparison. Reject unsigned/expired tokens. |
| **Spoofing** | Attacker impersonates the secret store | Secret Store connector | High | TLS certificate verification on secret store connections. Pin CA or use mTLS. |
| **Spoofing** | DNS hijack redirects third-party API calls | Outbound proxy | High | TLS with certificate verification on all outbound connections. No `danger_accept_invalid_certs`. |
| **Tampering** | Client sends malformed/oversized request body to exploit parsing | Request ingestion | High | Strict request body size limits (configurable, default 1 MB). Content-type validation. Schema validation on proxy requests before forwarding. |
| **Tampering** | Attacker modifies request in transit | Network | High | TLS 1.2+ required on all connections (inbound and outbound). HSTS header. |
| **Tampering** | Attacker injects headers to override API key or authorization | Proxy layer | Critical | Strip and rebuild outbound headers from an allowlist. Never forward `Authorization` header from client to third party. Facade injects its own. |
| **Repudiation** | Client denies making a request | Audit layer | Medium | Structured audit log with client identity, timestamp, request hash, response status. Logs are append-only and written to a durable sink. |
| **Information Disclosure** | API key leaked in logs | Logging | Critical | Never log secret values. Redact `Authorization` headers in all log output. Use `secrecy` crate for secret types that prevent `Display`/`Debug` leaks. |
| **Information Disclosure** | API key leaked in error responses | Error handling | Critical | Error responses use generic messages. Never include internal details, stack traces, or upstream error bodies that may contain auth info. |
| **Information Disclosure** | Third-party response contains data the client should not see | Response sanitization | Medium | Configurable response field allowlist/denylist per provider. Strip upstream `Set-Cookie`, `Server`, internal headers. |
| **Information Disclosure** | Timing side-channel on auth comparison | Auth Layer | Medium | Use constant-time comparison for API keys/HMAC (`subtle` crate or `ring`). |
| **Denial of Service** | Client floods the API with requests | Rate limiter | High | Per-client rate limiting (token bucket). Global rate limit as circuit breaker. Configurable limits per provider/endpoint. |
| **Denial of Service** | Slowloris/slow POST attacks | HTTP server | Medium | Request timeout (configurable, default 30s). Body read timeout. Connection idle timeout. Axum/Hyper built-in protections. |
| **Denial of Service** | Unbounded connection pool to third-party API | Outbound proxy | Medium | Bounded connection pool size per provider. Request queue depth limit. Backpressure via HTTP 429/503. |
| **Denial of Service** | Memory exhaustion from large request/response bodies | Proxy layer | High | Streaming proxy with bounded buffers. Max body size enforced on both inbound and outbound. |
| **Elevation of Privilege** | Client accesses a provider/endpoint they are not authorized for | Authorization | Critical | Per-client scope claims in JWT (e.g., `allowed_providers: ["openai"]`, `allowed_endpoints: ["chat/completions"]`). Deny by default. |
| **Elevation of Privilege** | Admin endpoint exposed without auth | Admin routes | High | Admin/config endpoints on a separate port or behind separate auth. No admin routes on the public-facing listener by default. |
| **Elevation of Privilege** | Secret store compromise exposes all API keys | Secret store | Critical | Least-privilege access policy in secret store. Rotate keys on schedule. Support for per-client or per-provider key isolation. |

### Security Requirements Derived from Threat Model

1. **SR-01**: All inbound connections must use TLS 1.2+. The server must not serve plaintext HTTP.
2. **SR-02**: Client authentication is mandatory on all proxy routes. Supported: JWT (RS256/ES256) or HMAC API keys.
3. **SR-03**: Authorization must enforce per-client scopes: allowed providers and allowed endpoints. Deny by default.
4. **SR-04**: API keys must be stored in a secret store, never in config files, environment variables in production, or source code. The `secrecy` crate must wrap all secret values to prevent accidental logging.
5. **SR-05**: Outbound requests to third-party APIs must use TLS with certificate verification. Headers are rebuilt from an allowlist; the client's `Authorization` header is never forwarded.
6. **SR-06**: Request body size is capped (default 1 MB, configurable). Response body size is capped (default 10 MB, configurable).
7. **SR-07**: Per-client rate limiting with configurable burst and sustained rates. Global rate limit as a circuit breaker.
8. **SR-08**: Structured audit logging on every proxy request: client ID, provider, endpoint, request size, response status, latency. No secret values in logs.
9. **SR-09**: Error responses are generic. No stack traces, no upstream error bodies that may leak secrets, no internal state.
10. **SR-10**: Health and readiness endpoints are unauthenticated but do not leak system information beyond `ok`/`not ready`.
11. **SR-11**: Graceful shutdown: drain in-flight requests before exiting. Configurable drain timeout.
12. **SR-12**: All cryptographic operations use audited libraries (`ring`, `jsonwebtoken`, `subtle`). No hand-rolled crypto.

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Project scaffolding, config, and health endpoints | `not_started` | | | | |
| 2 | Secret store integration | `not_started` | | | | |
| 3 | Client authentication and authorization | `not_started` | | | | |
| 4 | Facade proxy core | `not_started` | | | | |
| 5 | Rate limiting, body limits, and input validation | `not_started` | | | | |
| 6 | Audit logging, observability, and production hardening | `not_started` | | | | |
| 7 | End-to-end integration and security testing | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/api-facade-m<N>.md -->
<!-- Completion summaries go in docs/completion/api-facade-m<N>.md -->

---

## Global Execution Rules

These rules apply to every milestone without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone unless a listed step explicitly requires one additional file.
- Do not refactor unrelated code.
- Do not rename public APIs, commands, routes, events, persisted state shapes, or config keys unless the milestone explicitly says so.
- Do not introduce a new dependency unless the milestone explicitly allows it.
- Do not change database schema, file formats, or migration behavior unless the milestone explicitly includes migration work and migration tests.

### 2) Tests define the contract

- Write BDD tests before production code.
- Write E2E runtime validation stubs before production code.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when code compiles. It is done when the declared contract is satisfied and evidence is recorded.

### 3) No placeholders in production paths

The following are not allowed unless explicitly permitted in the milestone:

- TODO or placeholder logic in production code
- silent fallbacks that hide errors
- swallowed errors without structured logging or user-visible handling
- fake implementations left in place after tests pass
- commented-out dead code
- temporary mocks in production paths
- hard-coded secrets, test keys, or unsafe defaults

### 4) Preserve backwards compatibility

Every milestone must explicitly verify that previously working user flows, commands, routes, persisted state, and public interfaces still work unless the milestone explicitly replaces them.

### 5) Prefer smallest safe change

- Prefer narrow, local modifications over broad rewrites.
- Prefer extending existing patterns over inventing new abstractions.
- Prefer deleting complexity over adding new layers.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

All meaningful checks must be recorded in the milestone Evidence Log:

- command run
- relevant file or test
- expected result
- actual result
- pass/fail
- notes

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, test strategy improvements, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   cargo test --workspace
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create BDD test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   cargo test --test 'e2e_api_facade_*' -- --test-threads=1
   ```
3. Verify the app builds and boots to a usable state.
   ```
   cargo build --release && ./target/release/api-facade-gateway --config config/test.toml --dry-run
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. Update ARCHITECTURE.md following the Documentation Update Table.
8. Update README.md if user-facing capabilities changed.
9. Write a lessons-learned file at `docs/lessons/api-facade-m<N>.md`.
10. Write a completion summary at `docs/completion/api-facade-m<N>.md`.
11. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
12. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

No API facade gateway exists yet. This is a greenfield project within the existing workspace. The workspace currently contains CLI tools (`sldo-plan`, `sldo-run`) and a Tauri desktop application (`sldo-tauri`). The API facade will be a new, independent crate within the workspace.

### Problem

1. **Clients have direct access to third-party API keys**: Applications that need to call LLM APIs (OpenAI, Anthropic, etc.) currently must embed API keys. This is a critical security risk — keys are exposed in client-side code, configuration, or mobile app binaries.
2. **No centralized access control**: There is no way to enforce per-client authorization policies (which models, which endpoints, what rate limits) when clients call third-party APIs directly.
3. **No audit trail**: Direct client-to-API calls leave no centralized audit log for usage tracking, cost attribution, or abuse detection.
4. **No defense-in-depth**: Without a proxy layer, there is no place to enforce body size limits, request validation, rate limiting, or response sanitization.

### Target Architecture

```
                         ┌─────────────────────────────────────────────┐
                         │          api-facade-gateway (Rust/Axum)     │
                         │                                             │
  Client ──TLS──▶ │ Middleware Stack:                            │
  (JWT/API key)   │  ├─ TLS Termination (or behind LB)          │
                  │  ├─ Request Size Limit                       │
                  │  ├─ Authentication (JWT / API Key)           │
                  │  ├─ Authorization (scope check)              │
                  │  ├─ Rate Limiting (per-client)               │
                  │  ├─ Audit Logging                            │
                  │  └─ Proxy Handler                            │
                  │       ├─ Validate request shape              │
                  │       ├─ Fetch secret from store (cached)    │
                  │       ├─ Build outbound request              │
                  │       │   (allowlisted headers only)         │
                  │       ├─ Forward to third-party API ──TLS──▶ │ OpenAI / etc.
                  │       ├─ Validate + sanitize response        │
                  │       └─ Return to client                    │
                  │                                             │
                  │ Config: config/default.toml                 │
                  │ Secrets: Vault / AWS SM / env (dev only)    │
                  └─────────────────────────────────────────────┘

  Crate structure:
  crates/
    api-facade-gateway/
      Cargo.toml
      src/
        main.rs          — entry point, server bootstrap
        config.rs         — configuration loading (TOML + env overlay)
        error.rs          — unified error types, no secret leakage
        routes/
          mod.rs
          health.rs       — GET /health, GET /ready
          proxy.rs        — POST /v1/proxy/{provider}/{endpoint:.*}
        middleware/
          mod.rs
          auth.rs         — JWT and API key authentication
          authz.rs        — scope-based authorization
          rate_limit.rs   — per-client token bucket
          audit.rs        — structured request/response logging
          body_limit.rs   — request/response body size enforcement
        secrets/
          mod.rs
          traits.rs       — SecretStore trait
          vault.rs        — HashiCorp Vault backend
          aws.rs          — AWS Secrets Manager backend
          env.rs          — env-var backend (dev only)
          cache.rs        — TTL-based secret cache
        proxy/
          mod.rs
          client.rs       — reqwest client pool, TLS config
          provider.rs     — per-provider config (base URL, header rules)
          sanitize.rs     — response header/body sanitization
        auth/
          mod.rs
          jwt.rs          — JWT parsing and validation
          api_key.rs      — API key validation with constant-time comparison
          claims.rs       — client identity and scope types
      config/
        default.toml      — default configuration
        test.toml         — test configuration
      tests/
        e2e_api_facade_m1.rs
        e2e_api_facade_m2.rs
        ...
```

### Key Design Principles

1. **Secrets never leave the server**: The `secrecy` crate wraps all secret values. `Display` and `Debug` impls on secret types are masked. Log formatters redact known sensitive fields.
2. **Deny by default**: Clients have no access until explicitly granted via scopes. Unknown providers/endpoints are rejected. Unknown headers are stripped.
3. **Streaming over buffering**: The proxy streams request/response bodies where possible to avoid memory exhaustion. Size limits are enforced on the stream, not by buffering the entire body.
4. **Fail closed**: On auth failure, secret store unavailability, or upstream error, the response is a safe generic error. No fallback to unauthenticated mode.
5. **Configuration over code**: Provider configs (base URLs, header allowlists, rate limits) are in TOML, not hard-coded. Adding a new provider requires only config changes.
6. **Observability built-in**: Every request gets a trace ID. Structured JSON logs. Metrics exported via Prometheus endpoint. Health/readiness probes for orchestrator integration.

### What to Keep

- All existing workspace code (`sldo-common`, `sldo-plan`, `sldo-run`, `sldo-tauri`) is untouched.
- Workspace-level `Cargo.toml` structure.
- Existing test structure.

### What to Change

- **`Cargo.toml` (workspace)** — add `crates/api-facade-gateway` to workspace members
- **`crates/api-facade-gateway/`** — NEW: entire crate
- **`docs/ARCHITECTURE.md`** — add API facade section

### Global Red Lines

These are forbidden unless explicitly overridden inside a milestone.

- No unrelated refactors
- No new dependencies (beyond those listed per milestone)
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control
- No `unsafe` code unless justified and documented
- No `unwrap()` or `expect()` in production paths (use proper error propagation)
- No `danger_accept_invalid_certs` on outbound TLS
- No logging of secret values, API keys, or authorization headers

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first.
3. Confirm the tests fail for the expected reason.
4. Write production code to make the tests pass.
5. Re-run tests after any refactor.

### Required Test Coverage Categories

Every milestone must explicitly cover the categories that apply:

- happy path
- invalid input
- empty state / first-run state
- dependency failure / partial failure
- retry or rollback behavior if relevant
- concurrency or race behavior if relevant
- persistence / restore behavior if relevant
- backward compatibility behavior

If a category does not apply, state why.

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition]
    // When: [action]
    // Then: [expected outcome]
}
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Unit tests | `#[cfg(test)] mod tests` inside the source file | Same file as production code |
| Integration/BDD tests | `tests/api_facade_<feature>.rs` | `crates/api-facade-gateway/tests/` |
| E2E runtime validation | `tests/e2e_api_facade_m<N>.rs` | `crates/api-facade-gateway/tests/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go beyond compilation and verify that the system works correctly at runtime. These tests prove:

1. the server boots without errors
2. runtime contracts are met across HTTP boundaries
3. BDD scenarios work at runtime, not just in isolation
4. there are no runtime panics, unhandled rejections, or silent failures
5. degraded states behave safely and visibly

### E2E Test Design Rules

1. Test runtime behavior, not just types.
2. Test the full stack where possible.
3. Test degraded and failure states, not just the happy path.
4. Assert against observable behavior.

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

A new dependency is allowed only if the milestone explicitly includes:

- package/crate name
- why existing dependencies are insufficient
- security and maintenance rationale
- build/runtime cost rationale
- tests covering the new integration

### Migration policy

N/A — this is a greenfield project. No existing schemas or persisted state.

### Refactor budget

Each milestone must state one of the following:

- `No refactor permitted beyond direct implementation`
- `Minimal local refactor permitted in listed files only`
- `Targeted refactor permitted for [specific reason]`

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test 'e2e_api_facade_*'` | green | | | |
| Build/boot | `cargo build --release` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces and compatibility requirements?
- Did I add tests for failure modes, not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update documentation to match the implementation?
- Is every assumption either verified or explicitly documented as unresolved?
- Is the milestone truly done according to its Definition of Done?
- **Are any secret values logged, displayed, or included in error responses?** (Must be NO)
- **Are all outbound TLS connections verified?** (Must be YES)

If any answer is wrong, the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/lessons/api-facade-m<N>.md`

```md
# Lessons Learned — api-facade Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Naming conventions established
- [types, files, tests, events, commands]

## Test patterns that worked well
- [pattern]

## Missing tests that should exist now
- [test]

## Security observations
- [observation]

## Rules for the next milestone
- [rule]

## Template improvements suggested
- [improvement]
```

---

## Completion Summary Template

Path: `docs/completion/api-facade-m<N>.md`

```md
# Completion Summary — api-facade Milestone <N>

## Goal completed
- [what capability now exists]

## Files changed
- [file]

## Tests added
- [test file]

## Runtime validations added
- [e2e file]

## Compatibility checks performed
- [check]

## Documentation updated
- [doc and section]

## Security requirements addressed
- [SR-XX: description]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## Milestone Plan

---

### Milestone 1 — Project Scaffolding, Configuration, and Health Endpoints

**Goal**: Create the `api-facade-gateway` crate with Axum, configuration loading from TOML with environment variable overlay, unified error handling, and health/readiness endpoints. The server must start, listen on a configurable port, and respond to `GET /health` and `GET /ready`.

**Context**: No code exists yet. This milestone creates the foundation that every subsequent milestone builds on. Configuration must be environment-aware from day one because secrets, ports, and provider URLs differ per environment. The error type must enforce the "no secrets in responses" rule from the start.

**Important design rule**: The config system must support layered configuration: default TOML file → environment-specific TOML file → environment variable overrides. Secrets are *not* in config files; this milestone only sets up the config structure. The actual secret store comes in M2.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | TOML config file path (CLI arg or env var), environment variables |
| Outputs | Running HTTP server on configured port, health/readiness JSON responses |
| Interfaces touched | `GET /health` → `{"status": "ok"}`, `GET /ready` → `{"status": "ready"}` or `{"status": "not_ready", "reason": "..."}` |
| Files allowed to change | `Cargo.toml` (workspace root) |
| Files to read before changing anything | `Cargo.toml` (workspace root) |
| New files allowed | entire `crates/api-facade-gateway/` directory |
| New dependencies allowed | `axum ^0.7`, `tokio ^1` (full features), `serde ^1`, `serde_json ^1`, `toml ^0.8`, `tracing ^0.1`, `tracing-subscriber ^0.3`, `tower-http ^0.5` (cors, trace), `clap ^4` (derive), `thiserror ^1` |
| Migration allowed | `no` |
| Compatibility commitments | All existing workspace crates must still compile and all existing tests must still pass |
| Forbidden shortcuts | No `unwrap()` in production code, no placeholder health logic, no hard-coded port |

#### Out of Scope / Must Not Do

- No authentication or authorization (M3)
- No secret store integration (M2)
- No proxy routes (M4)
- No rate limiting (M5)
- No TLS termination (handled by reverse proxy in production; documented in M6)
- No metrics or Prometheus endpoint (M6)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. No previous lessons file exists — this is M1.
3. Read `Cargo.toml` (workspace root) to understand workspace member structure.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `Cargo.toml` (workspace root) | Add `crates/api-facade-gateway` to workspace members |
| `crates/api-facade-gateway/Cargo.toml` | NEW: crate manifest with dependencies |
| `crates/api-facade-gateway/src/main.rs` | NEW: entry point — parse CLI args, load config, start server |
| `crates/api-facade-gateway/src/config.rs` | NEW: config structs, TOML loading, env overlay |
| `crates/api-facade-gateway/src/error.rs` | NEW: `AppError` enum, `IntoResponse` impl, no secret leakage |
| `crates/api-facade-gateway/src/routes/mod.rs` | NEW: route registration |
| `crates/api-facade-gateway/src/routes/health.rs` | NEW: health and readiness handlers |
| `crates/api-facade-gateway/config/default.toml` | NEW: default configuration |
| `crates/api-facade-gateway/config/test.toml` | NEW: test configuration |
| `tests/e2e_api_facade_m1.rs` | NEW: E2E runtime validation |

#### Step-by-Step

1. Write BDD test stubs first for all scenarios below.
2. Write E2E runtime validation stubs first.
3. Create `crates/api-facade-gateway/Cargo.toml` with listed dependencies.
4. Add the crate to workspace members.
5. Implement `config.rs`: define `AppConfig`, `ServerConfig` structs; implement TOML loading with env overlay.
6. Implement `error.rs`: define `AppError` with `IntoResponse` that returns safe JSON errors.
7. Implement `routes/health.rs`: `GET /health` always returns 200, `GET /ready` checks readiness state.
8. Implement `routes/mod.rs`: register routes.
9. Implement `main.rs`: CLI arg parsing, config loading, server startup with graceful shutdown signal handler.
10. Create `config/default.toml` and `config/test.toml`.
11. Make all BDD tests pass.
12. Run the full test suite.
13. Run E2E runtime validation.
14. Run smoke tests.
15. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Configuration loading**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Load default config | happy path | A valid `default.toml` exists | The server starts with `--config config/default.toml` | Config is loaded with all default values |
| Env var overrides config | happy path | `API_FACADE_SERVER__PORT=9090` is set | Config is loaded | `server.port` is `9090` regardless of TOML value |
| Missing config file | invalid input | Config file path does not exist | The server starts | Exits with clear error message and non-zero exit code |
| Invalid TOML | invalid input | Config file contains invalid TOML | The server starts | Exits with clear error message referencing the parse error |
| Missing required fields | invalid input | Config file omits `server.port` and no env var | The server starts | Exits with error describing the missing field |

**Feature: Health and readiness endpoints**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Health check returns ok | happy path | Server is running | `GET /health` | 200, `{"status": "ok"}` |
| Readiness returns ready | happy path | Server is fully initialized | `GET /ready` | 200, `{"status": "ready"}` |
| Unknown route returns 404 | invalid input | Server is running | `GET /nonexistent` | 404, JSON error body |

**Feature: Error response safety**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Error responses are generic | invalid input | Any error occurs | Error is returned to client | Response body contains no stack trace, no file paths, no internal details |

#### Regression Tests

- All existing workspace tests still pass (`cargo test --workspace`)
- All existing crates still compile (`cargo build --workspace`)

#### Compatibility Checklist

- [ ] Existing workspace tests pass
- [ ] Existing crates compile without changes
- [ ] No changes to any existing file except workspace `Cargo.toml`

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_server_boots_and_responds_to_health` | Server starts and health endpoint works at runtime | GET /health returns 200 with `{"status": "ok"}` |
| `test_readiness_endpoint` | Readiness probe works | GET /ready returns 200 with `{"status": "ready"}` |
| `test_unknown_route_returns_404` | 404 handler works | GET /nonexistent returns 404 with JSON error |
| `test_error_responses_have_no_internal_details` | Security: errors are safe | Error responses contain no file paths, no stack traces |

#### Smoke Tests

- [ ] `cargo build --release -p api-facade-gateway` compiles
- [ ] `./target/release/api-facade-gateway --config crates/api-facade-gateway/config/default.toml` starts and prints listening address
- [ ] `curl http://localhost:<port>/health` returns `{"status": "ok"}`
- [ ] `curl http://localhost:<port>/ready` returns `{"status": "ready"}`
- [ ] `curl http://localhost:<port>/nonexistent` returns 404 JSON
- [ ] Ctrl+C triggers graceful shutdown

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | `tests/e2e_api_facade_m1.rs`, unit tests | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_api_facade_m1.rs` | fail for expected reason | | | |
| Implementation | config, error, health routes, main | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_api_facade_m1` | green | | | |
| Build/boot | `cargo build --release -p api-facade-gateway` | boots cleanly | | | |
| Smoke tests | curl commands | all checked | | | |
| Compatibility checks | existing tests | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- `GET /health` returns `{"status": "ok"}` with 200
- `GET /ready` returns `{"status": "ready"}` with 200
- Config loads from TOML with environment variable overlay
- Missing/invalid config produces clear error and non-zero exit
- Error responses contain no internal details
- All BDD scenarios pass
- All E2E runtime validations pass
- Full existing test suite remains green
- Smoke tests are checked off
- Compatibility checklist is complete
- No `unwrap()` or `expect()` in production paths
- Lessons file is written
- Completion summary is written
- Milestone Tracker is updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Add "API Facade Gateway" section describing the crate, its purpose, config system, and module layout.
- **README.md**: Add API Facade Gateway to the list of workspace crates.
- **Other docs**: None.

#### Notes

- Concurrency/race testing does not apply — this milestone has no shared mutable state.
- Persistence testing does not apply — no persistence in this milestone.
- Retry/rollback does not apply — no retryable operations.

---

### Milestone 2 — Secret Store Integration

**Goal**: Implement a `SecretStore` trait with three backends: HashiCorp Vault, AWS Secrets Manager, and environment variables (dev-only). Add a TTL-based cache layer. The server must fetch and cache third-party API keys from the configured backend on startup and on-demand.

**Context**: M1 delivered configuration loading and health endpoints. This milestone adds the secret store layer that M4 (proxy) will use to inject API keys into outbound requests. The `secrecy` crate must wrap all secret values so they cannot be accidentally logged or serialized. The cache prevents excessive round-trips to the secret store.

**Important design rule**: The `SecretStore` trait must be async and object-safe so backends are interchangeable at runtime based on configuration. The env-var backend is for development only and must log a warning on startup if used. The cache must support per-key TTL and background refresh.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Secret store backend config (type, endpoint, auth), secret key names in provider config |
| Outputs | `SecretStore` trait returning `Secret<String>` values on demand |
| Interfaces touched | New `SecretStore` trait, cache layer, readiness probe updated to check secret store connectivity |
| Files allowed to change | `crates/api-facade-gateway/src/routes/health.rs` (readiness check), `crates/api-facade-gateway/src/config.rs` (secret store config), `crates/api-facade-gateway/src/main.rs` (initialize secret store) |
| Files to read before changing anything | All M1 files |
| New files allowed | `crates/api-facade-gateway/src/secrets/mod.rs`, `traits.rs`, `vault.rs`, `aws.rs`, `env.rs`, `cache.rs` |
| New dependencies allowed | `secrecy ^0.8`, `reqwest ^0.12` (for Vault HTTP API), `aws-sdk-secretsmanager` (latest), `aws-config` (latest), `tokio ^1` (time feature for TTL) |
| Migration allowed | `no` |
| Compatibility commitments | `GET /health` still works, `GET /ready` now also checks secret store connectivity |
| Forbidden shortcuts | No hard-coded secrets, no plaintext secret logging, no `String` for secret values (must use `Secret<String>`) |

#### Out of Scope / Must Not Do

- No proxy routes (M4)
- No authentication (M3)
- No actual third-party API calls (M4)
- No rate limiting (M5)
- Do not implement the full Vault authentication protocol (support token-based auth only for now; AppRole can be a follow-up)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/api-facade-m1.md` and apply relevant corrections.
3. Read all M1 files.
4. Copy the Evidence Log template.
5. Re-state constraints.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/api-facade-gateway/src/config.rs` | Add `SecretStoreConfig` struct |
| `crates/api-facade-gateway/src/main.rs` | Initialize secret store, pass to app state |
| `crates/api-facade-gateway/src/routes/health.rs` | Readiness probe checks secret store |
| `crates/api-facade-gateway/src/secrets/mod.rs` | NEW: module root, re-exports |
| `crates/api-facade-gateway/src/secrets/traits.rs` | NEW: `SecretStore` trait definition |
| `crates/api-facade-gateway/src/secrets/vault.rs` | NEW: Vault backend implementation |
| `crates/api-facade-gateway/src/secrets/aws.rs` | NEW: AWS Secrets Manager backend |
| `crates/api-facade-gateway/src/secrets/env.rs` | NEW: env-var backend (dev only) |
| `crates/api-facade-gateway/src/secrets/cache.rs` | NEW: TTL-based caching wrapper |
| `crates/api-facade-gateway/Cargo.toml` | Add new dependencies |
| `tests/e2e_api_facade_m2.rs` | NEW: E2E runtime validation |

#### Step-by-Step

1. Write BDD test stubs for all scenarios below.
2. Write E2E runtime validation stubs.
3. Define `SecretStore` trait in `traits.rs`:
   ```rust
   #[async_trait]
   pub trait SecretStore: Send + Sync {
       async fn get_secret(&self, key: &str) -> Result<Secret<String>, SecretStoreError>;
       async fn health_check(&self) -> Result<(), SecretStoreError>;
   }
   ```
4. Implement `EnvSecretStore` — reads from env vars with a configurable prefix.
5. Implement `VaultSecretStore` — HTTP client to Vault KV v2 API with token auth.
6. Implement `AwsSecretStore` — AWS SDK client.
7. Implement `CachedSecretStore<S: SecretStore>` — wraps any backend with TTL-based caching using `tokio::sync::RwLock` and per-key expiry.
8. Update `config.rs` with secret store configuration.
9. Update `main.rs` to initialize the configured backend.
10. Update `health.rs` readiness to ping the secret store.
11. Make all BDD tests pass.
12. Run the full test suite and E2E.

#### BDD Acceptance Scenarios

**Feature: Secret store trait**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Fetch secret from env backend | happy path | `API_FACADE_SECRET_OPENAI_KEY=sk-test` is set | `get_secret("openai_key")` | Returns `Secret<String>` containing the value |
| Missing secret key | invalid input | Key does not exist in backend | `get_secret("nonexistent")` | Returns `SecretStoreError::NotFound` |
| Backend unavailable | partial failure | Vault endpoint unreachable | `get_secret("any_key")` | Returns `SecretStoreError::Unavailable` with no secret data in error |

**Feature: Secret caching**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Cache hit avoids backend call | happy path | Secret was fetched < TTL ago | `get_secret("openai_key")` again | Returns cached value without backend call |
| Cache expiry triggers refresh | happy path | Secret was fetched > TTL ago | `get_secret("openai_key")` | Fetches from backend, updates cache |
| Cache miss fetches from backend | empty state | Cache is empty | `get_secret("openai_key")` | Fetches from backend, stores in cache |

**Feature: Secret safety**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Secret Debug does not leak value | security | A `Secret<String>` is obtained | `format!("{:?}", secret)` | Output is `Secret([REDACTED])` or similar, never the actual value |
| Secret Display does not leak value | security | A `Secret<String>` is obtained | `format!("{}", secret)` | Output is redacted |
| Error messages do not contain secrets | security | A secret store error occurs | Error is formatted | No secret values in error message |

**Feature: Readiness probe with secret store**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Ready when secret store is healthy | happy path | Secret store responds to health check | `GET /ready` | 200 `{"status": "ready"}` |
| Not ready when secret store is down | partial failure | Secret store is unreachable | `GET /ready` | 503 `{"status": "not_ready", "reason": "secret store unavailable"}` |

#### Regression Tests

- `GET /health` still returns 200 with `{"status": "ok"}`
- All M1 tests pass
- Config loading still works (backward compatible)

#### Compatibility Checklist

- [ ] `GET /health` unchanged
- [ ] `GET /ready` returns 200 when healthy (superset of M1 behavior)
- [ ] Config files from M1 still load without error (new fields have defaults)
- [ ] All M1 E2E tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_env_backend_fetches_secret` | Env secret store works at runtime | Set env var, fetch returns correct secret |
| `test_cached_secret_store_caches` | Cache works at runtime | Second fetch is faster / doesn't hit backend |
| `test_readiness_reflects_secret_store_health` | Readiness probe includes secret store | Unhealthy backend → 503 on /ready |
| `test_secret_debug_is_redacted` | Secrets don't leak via Debug | Debug output is redacted |

#### Smoke Tests

- [ ] Server starts with env secret backend configured
- [ ] `GET /ready` returns 200 when env vars are set
- [ ] Server logs warning when using env backend
- [ ] No secret values visible in any log output

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | secret store tests | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_api_facade_m2.rs` | fail for expected reason | | | |
| Implementation | secret store trait, backends, cache | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_api_facade_m2` | green | | | |
| Build/boot | `cargo build --release -p api-facade-gateway` | boots cleanly | | | |
| Smoke tests | curl + log check | all checked | | | |
| Compatibility checks | M1 tests | no regressions | | | |

#### Definition of Done

- `SecretStore` trait is defined and object-safe
- Env, Vault, and AWS backends compile and pass unit tests
- Cached wrapper caches with TTL
- `Secret<String>` is used everywhere — no raw `String` for secrets
- Debug/Display on secrets is redacted
- Readiness probe checks secret store health
- All BDD and E2E tests pass
- No secret values in logs or error messages
- Lessons file and completion summary written
- Milestone Tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Add secret store subsystem, trait design, caching strategy.
- **README.md**: Document secret store backends and configuration.

#### Notes

- Vault integration will be tested with a mock HTTP server in tests, not a real Vault instance.
- AWS integration will use the SDK's mock/local testing support.
- The env backend `MUST` log `tracing::warn!("Using environment variable secret backend — NOT for production use")` on initialization.

---

### Milestone 3 — Client Authentication and Authorization

**Goal**: Implement JWT validation and API key authentication middleware. Add scope-based authorization that restricts clients to specific providers and endpoints. All proxy routes (added in M4) will be protected by this middleware. Addresses SR-02, SR-03, SR-12.

**Context**: M1 and M2 provide a running server with config and secrets. This milestone adds the auth layer. JWT support enables integration with identity providers (Auth0, Keycloak, etc.). API key support enables service-to-service auth. Both mechanisms produce a `ClientIdentity` with scopes that the authorization layer checks.

**Important design rule**: Authentication and authorization are separate middleware layers. Authentication extracts identity; authorization checks permissions. This separation allows testing each independently and enables future auth mechanisms without changing authorization logic.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `Authorization: Bearer <jwt>` or `X-API-Key: <key>` headers |
| Outputs | `ClientIdentity` in request extensions (includes client_id, allowed_providers, allowed_endpoints) |
| Interfaces touched | New auth middleware, new authz middleware, new `ClientIdentity` type |
| Files allowed to change | `crates/api-facade-gateway/src/config.rs` (auth config), `crates/api-facade-gateway/src/main.rs` (add middleware), `crates/api-facade-gateway/src/routes/mod.rs` (apply middleware to protected routes) |
| Files to read before changing anything | All M1 and M2 files |
| New files allowed | `crates/api-facade-gateway/src/auth/mod.rs`, `jwt.rs`, `api_key.rs`, `claims.rs`, `crates/api-facade-gateway/src/middleware/mod.rs`, `auth.rs`, `authz.rs` |
| New dependencies allowed | `jsonwebtoken ^9`, `subtle ^2` (constant-time comparison), `ring ^0.17` (for HMAC if needed) |
| Migration allowed | `no` |
| Compatibility commitments | Health and readiness endpoints remain unauthenticated |
| Forbidden shortcuts | No plaintext API key comparison (must be constant-time), no `HS256` default (require explicit algorithm config), no auth bypass flags |

#### Out of Scope / Must Not Do

- No proxy routes (M4)
- No rate limiting (M5)
- No OAuth2 flow (out of scope — we validate tokens, we don't issue them)
- No mTLS client authentication (future enhancement)
- No RBAC beyond provider/endpoint scopes

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/api-facade-m2.md`.
3. Read all M1 and M2 files.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/api-facade-gateway/src/config.rs` | Add `AuthConfig` (JWT issuer, audience, JWKS URL or public key, API key hashes) |
| `crates/api-facade-gateway/src/main.rs` | Add auth middleware to router |
| `crates/api-facade-gateway/src/routes/mod.rs` | Split routes into public (health) and protected groups |
| `crates/api-facade-gateway/src/auth/mod.rs` | NEW: module root |
| `crates/api-facade-gateway/src/auth/jwt.rs` | NEW: JWT decoding and validation |
| `crates/api-facade-gateway/src/auth/api_key.rs` | NEW: API key validation with constant-time comparison |
| `crates/api-facade-gateway/src/auth/claims.rs` | NEW: `ClientIdentity`, scope types |
| `crates/api-facade-gateway/src/middleware/mod.rs` | NEW: middleware module root |
| `crates/api-facade-gateway/src/middleware/auth.rs` | NEW: authentication middleware (extracts identity) |
| `crates/api-facade-gateway/src/middleware/authz.rs` | NEW: authorization middleware (checks scopes) |
| `crates/api-facade-gateway/Cargo.toml` | Add new dependencies |
| `tests/e2e_api_facade_m3.rs` | NEW: E2E runtime validation |

#### BDD Acceptance Scenarios

**Feature: JWT authentication**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Valid JWT accepted | happy path | Request has valid `Authorization: Bearer <jwt>` with correct issuer, audience, not expired | Request hits protected route | 200, `ClientIdentity` extracted with correct claims |
| Expired JWT rejected | invalid input | Request has expired JWT | Request hits protected route | 401 `{"error": "unauthorized", "message": "token expired"}` |
| Wrong issuer rejected | invalid input | JWT has unexpected issuer | Request hits protected route | 401 |
| Missing Authorization header | invalid input | No auth header present | Request hits protected route | 401 `{"error": "unauthorized", "message": "missing credentials"}` |
| Malformed JWT rejected | invalid input | Authorization header has invalid token | Request hits protected route | 401 |
| Wrong algorithm rejected | security | JWT signed with `none` or unexpected algorithm | Request hits protected route | 401 |

**Feature: API key authentication**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Valid API key accepted | happy path | Request has valid `X-API-Key` header matching a configured hash | Request hits protected route | 200, `ClientIdentity` extracted |
| Invalid API key rejected | invalid input | `X-API-Key` does not match any hash | Request hits protected route | 401 |
| Timing-safe comparison | security | Attacker sends many keys of varying lengths | Response time measured | No significant timing difference between valid-prefix and invalid-prefix keys |

**Feature: Authorization**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Allowed provider + endpoint accepted | happy path | Client has scope `openai:chat/completions` | Request to `/v1/proxy/openai/chat/completions` | 200 (request proceeds) |
| Disallowed provider rejected | invalid input | Client has scope `openai:*` only | Request to `/v1/proxy/anthropic/messages` | 403 `{"error": "forbidden", "message": "provider not allowed"}` |
| Disallowed endpoint rejected | invalid input | Client has scope `openai:chat/completions` only | Request to `/v1/proxy/openai/embeddings` | 403 `{"error": "forbidden", "message": "endpoint not allowed"}` |
| Wildcard endpoint scope | happy path | Client has scope `openai:*` | Any openai endpoint | 200 (request proceeds) |

**Feature: Health endpoints remain public**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Health without auth | backward compat | No auth header | `GET /health` | 200 (no auth required) |
| Ready without auth | backward compat | No auth header | `GET /ready` | 200 or 503 (no auth required) |

#### Regression Tests

- All M1 and M2 tests pass
- Health and readiness endpoints work without authentication
- Config loading backward compatible

#### Compatibility Checklist

- [ ] `GET /health` works without authentication
- [ ] `GET /ready` works without authentication
- [ ] All M1 and M2 E2E tests pass
- [ ] Config files from M1/M2 still load (new auth fields have defaults or are optional)

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_valid_jwt_authenticates` | JWT validation works at runtime | Valid JWT returns 200 on protected route |
| `test_expired_jwt_rejected` | Expired tokens are rejected | 401 returned |
| `test_valid_api_key_authenticates` | API key auth works | Valid key returns 200 |
| `test_invalid_api_key_rejected` | Bad keys rejected | 401 returned |
| `test_authorization_allows_scoped_access` | Authz allows correct scope | 200 on allowed route |
| `test_authorization_denies_out_of_scope` | Authz blocks wrong scope | 403 on disallowed route |
| `test_health_requires_no_auth` | Public endpoints stay public | 200 without any auth header |

#### Smoke Tests

- [ ] Server starts with auth configured
- [ ] `curl /health` without auth → 200
- [ ] `curl /ready` without auth → 200
- [ ] `curl -H "Authorization: Bearer <valid_jwt>" /v1/proxy/openai/test` → proceeds past auth (404 on route is ok — proxy not yet implemented)
- [ ] `curl /v1/proxy/openai/test` without auth → 401
- [ ] `curl -H "X-API-Key: <invalid>" /v1/proxy/openai/test` → 401

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | auth tests | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_api_facade_m3.rs` | fail for expected reason | | | |
| Implementation | auth, authz middleware | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_api_facade_m3` | green | | | |
| Build/boot | `cargo build --release -p api-facade-gateway` | boots cleanly | | | |
| Smoke tests | curl commands | all checked | | | |
| Compatibility checks | M1+M2 tests | no regressions | | | |

#### Definition of Done

- JWT validation with configurable algorithm, issuer, audience
- API key validation with constant-time comparison
- `ClientIdentity` extracted into request extensions
- Scope-based authorization checks provider and endpoint
- Health/readiness remain unauthenticated
- All BDD and E2E tests pass
- No timing side-channels in API key comparison
- Lessons and completion summary written

#### Post-Flight

- **ARCHITECTURE.md**: Add authentication and authorization sections.
- **README.md**: Document auth configuration (JWT, API keys, scopes).

---

### Milestone 4 — Facade Proxy Core

**Goal**: Implement the proxy handler that receives authenticated client requests, fetches the third-party API key from the secret store, builds a sanitized outbound request, calls the third-party API, sanitizes the response, and returns it to the client. This is the core value of the system. Addresses SR-05.

**Context**: M1–M3 provide a running server with config, secrets, and auth. This milestone implements the actual proxying. The proxy must rebuild outbound requests from an allowlist of headers (never forward client auth), inject the API key from the secret store, enforce TLS on outbound calls, and strip internal headers from responses.

**Important design rule**: The proxy must be provider-agnostic. Provider-specific behavior (base URL, auth header format, allowed endpoints) comes from configuration. Adding support for a new third-party API should require only a TOML config entry, not code changes.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `POST /v1/proxy/{provider}/{endpoint:.*}` with authenticated `ClientIdentity` |
| Outputs | Proxied response with sanitized headers and body |
| Interfaces touched | New proxy route, new provider config, secret store usage |
| Files allowed to change | `crates/api-facade-gateway/src/config.rs` (provider config), `crates/api-facade-gateway/src/main.rs` (add proxy route), `crates/api-facade-gateway/src/routes/mod.rs` (register proxy routes) |
| Files to read before changing anything | All M1–M3 files |
| New files allowed | `crates/api-facade-gateway/src/routes/proxy.rs`, `crates/api-facade-gateway/src/proxy/mod.rs`, `client.rs`, `provider.rs`, `sanitize.rs` |
| New dependencies allowed | `reqwest ^0.12` (if not already added in M2; with `rustls-tls` feature, NOT `native-tls`), `bytes ^1`, `http ^1` |
| Migration allowed | `no` |
| Compatibility commitments | All M1–M3 routes and behavior unchanged |
| Forbidden shortcuts | No forwarding of client `Authorization` header, no `danger_accept_invalid_certs`, no unbounded body buffering, no hard-coded provider URLs |

#### Out of Scope / Must Not Do

- No rate limiting (M5)
- No streaming (implement buffered proxy first; streaming is a follow-up)
- No request body transformation (forward as-is after size check)
- No response body transformation beyond header sanitization
- No retry logic on upstream failures (surface the error to client)
- No WebSocket proxying

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/api-facade-m3.md`.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/api-facade-gateway/src/config.rs` | Add `ProviderConfig` (base_url, auth_header_name, secret_key_name, allowed_endpoints, header_allowlist) |
| `crates/api-facade-gateway/src/main.rs` | Initialize reqwest client, add proxy route |
| `crates/api-facade-gateway/src/routes/mod.rs` | Register proxy routes inside auth middleware group |
| `crates/api-facade-gateway/src/routes/proxy.rs` | NEW: proxy handler |
| `crates/api-facade-gateway/src/proxy/mod.rs` | NEW: module root |
| `crates/api-facade-gateway/src/proxy/client.rs` | NEW: reqwest client with TLS, connection pooling |
| `crates/api-facade-gateway/src/proxy/provider.rs` | NEW: provider config resolution, URL building |
| `crates/api-facade-gateway/src/proxy/sanitize.rs` | NEW: response header/body sanitization |
| `crates/api-facade-gateway/config/default.toml` | Add example provider config for OpenAI |
| `tests/e2e_api_facade_m4.rs` | NEW: E2E runtime validation |

#### BDD Acceptance Scenarios

**Feature: Proxy request forwarding**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Successful proxy to third-party | happy path | Authenticated client, valid provider config, secret available | `POST /v1/proxy/openai/chat/completions` with JSON body | 200 with third-party response body |
| API key injected from secret store | happy path | Secret store has key for `openai` | Outbound request inspected | `Authorization: Bearer <secret>` header present on outbound, not the client's token |
| Client auth header not forwarded | security | Client sends `Authorization: Bearer <client_jwt>` | Outbound request inspected | Outbound request has the API key, not the client JWT |
| Unknown provider rejected | invalid input | Provider `unknown` not in config | `POST /v1/proxy/unknown/endpoint` | 404 `{"error": "not_found", "message": "provider not configured"}` |
| Secret store unavailable | partial failure | Secret store returns error | Proxy request | 502 `{"error": "bad_gateway", "message": "upstream dependency unavailable"}` |
| Third-party returns error | partial failure | Third-party API returns 429 | Proxy request | 429 forwarded to client (status preserved) |
| Third-party unreachable | partial failure | DNS or connection failure | Proxy request | 502 with generic error |

**Feature: Response sanitization**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Internal headers stripped | security | Third-party response has `Server`, `X-Request-Id`, `Set-Cookie` | Response returned to client | Those headers are removed |
| Content-Type preserved | happy path | Third-party returns `Content-Type: application/json` | Response returned to client | `Content-Type` header preserved |

**Feature: Provider configuration**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Provider base URL used | happy path | Config has `openai.base_url = "https://api.openai.com/v1"` | `POST /v1/proxy/openai/chat/completions` | Outbound request goes to `https://api.openai.com/v1/chat/completions` |
| Provider-specific auth format | happy path | Config has `openai.auth_header = "Authorization"`, `auth_prefix = "Bearer "` | Outbound request | Header is `Authorization: Bearer <key>` |
| Anthropic-style auth format | happy path | Config has `anthropic.auth_header = "x-api-key"`, `auth_prefix = ""` | Outbound request | Header is `x-api-key: <key>` |

#### Regression Tests

- All M1–M3 tests pass
- Health, readiness, auth all unchanged

#### Compatibility Checklist

- [ ] `GET /health` still works
- [ ] `GET /ready` still works
- [ ] Auth middleware still rejects unauthenticated requests
- [ ] All previous E2E tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_proxy_forwards_to_mock_upstream` | Proxy works end-to-end | Authenticated request → mock upstream receives call → response returned to client |
| `test_api_key_injected_not_client_token` | Secret injection is correct | Mock upstream sees API key from secret store, not client JWT |
| `test_unknown_provider_returns_404` | Unknown providers handled | 404 returned for unconfigured provider |
| `test_upstream_error_forwarded_safely` | Error handling is safe | Upstream 500 → client gets 502 with generic message |
| `test_response_headers_sanitized` | Headers stripped | Internal upstream headers not returned to client |
| `test_outbound_request_uses_tls` | TLS enforced | Non-HTTPS base_url in config → rejected at startup or request time |

#### Smoke Tests

- [ ] `POST /v1/proxy/openai/chat/completions` with valid auth and body returns proxied response
- [ ] Upstream error surfaces as 502 to client
- [ ] Response does not contain upstream `Server` header
- [ ] Logs show request trace with client ID but no API key

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | proxy tests | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_api_facade_m4.rs` | fail for expected reason | | | |
| Implementation | proxy handler, client, sanitizer | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_api_facade_m4` | green | | | |
| Build/boot | `cargo build --release -p api-facade-gateway` | boots cleanly | | | |
| Smoke tests | curl to proxy | all checked | | | |
| Compatibility checks | M1-M3 tests | no regressions | | | |

#### Definition of Done

- `POST /v1/proxy/{provider}/{endpoint}` works end-to-end
- API key injected from secret store, not from client
- Client auth header never forwarded to upstream
- Response headers sanitized
- Unknown providers → 404
- Upstream failures → 502 with generic error
- Provider config is TOML-driven, not hard-coded
- TLS enforced on all outbound connections
- All BDD and E2E tests pass
- Lessons and completion summary written

#### Post-Flight

- **ARCHITECTURE.md**: Add proxy subsystem, data flow diagram, provider configuration.
- **README.md**: Document how to add a new provider, example curl commands.

---

### Milestone 5 — Rate Limiting, Body Limits, and Input Validation

**Goal**: Add per-client rate limiting (token bucket), request/response body size limits, and input validation on proxy requests. Addresses SR-06, SR-07.

**Context**: M4 delivers a working proxy. This milestone adds the protective layers that prevent abuse, resource exhaustion, and malformed input from reaching the upstream API or exhausting server resources.

**Important design rule**: Rate limiting must be per-client (keyed by `client_id` from `ClientIdentity`). Limits are configurable per provider. A global rate limit acts as a circuit breaker. Body size limits are enforced before reading the entire body (using `Content-Length` header for known sizes, chunked stream counting for unknown).

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Request body, `Content-Length` header, client identity |
| Outputs | 429 when rate limited, 413 when body too large, 400 when validation fails |
| Interfaces touched | New rate limit, body limit, validation middleware |
| Files allowed to change | `crates/api-facade-gateway/src/config.rs`, `main.rs`, `routes/mod.rs`, `routes/proxy.rs` |
| Files to read before changing anything | All M1–M4 files |
| New files allowed | `middleware/rate_limit.rs`, `middleware/body_limit.rs` |
| New dependencies allowed | `governor ^0.6` (rate limiting), `dashmap ^5` (concurrent map for per-client state) |
| Migration allowed | `no` |
| Compatibility commitments | All M1–M4 routes and behavior unchanged |
| Forbidden shortcuts | No in-memory-only rate limiter without eviction (must bound memory), no disabling body limits via config to 0/unlimited |

#### Out of Scope / Must Not Do

- No distributed rate limiting (Redis-backed; this is single-instance, follow up for distributed)
- No request body transformation or schema validation beyond size and content-type
- No response body transformation

#### BDD Acceptance Scenarios

**Feature: Rate limiting**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Under rate limit | happy path | Client has not exceeded limit | Request sent | 200 (request proceeds) |
| Rate limit exceeded | happy path | Client has exceeded per-second limit | Request sent | 429 `{"error": "too_many_requests", "message": "rate limit exceeded", "retry_after": N}` |
| Different clients independent | happy path | Client A is rate-limited | Client B sends request | Client B succeeds |
| Rate limit per provider | happy path | Client rate-limited on `openai` | Same client requests `anthropic` | Anthropic request succeeds |
| Global rate limit | partial failure | Global limit exceeded | Any client sends request | 429 with `"message": "service rate limit exceeded"` |

**Feature: Body size limits**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Request within size limit | happy path | Body is 500 bytes, limit is 1 MB | Request sent | 200 (proceeds) |
| Request exceeds size limit | invalid input | Body is 2 MB, limit is 1 MB | Request sent | 413 `{"error": "payload_too_large"}` |
| Content-Length checked first | invalid input | `Content-Length: 2000000`, limit 1 MB | Before body read | 413 returned without reading body |
| No Content-Length, enforced via stream | invalid input | Chunked body exceeds limit | During body read | 413 returned, stream closed |

**Feature: Input validation**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Valid Content-Type accepted | happy path | `Content-Type: application/json` | Request to proxy | Proceeds |
| Missing Content-Type on POST | invalid input | No Content-Type header | POST to proxy | 400 `{"error": "bad_request", "message": "Content-Type required"}` |
| Request method validation | invalid input | `GET /v1/proxy/openai/completions` | Request sent | 405 `{"error": "method_not_allowed"}` (proxy only accepts POST) |

#### Regression Tests

- All M1–M4 tests pass
- Proxy still works for valid requests within limits

#### Compatibility Checklist

- [ ] All M1–M4 routes unchanged
- [ ] Valid proxy requests still succeed
- [ ] Auth behavior unchanged
- [ ] Health/readiness unchanged

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_rate_limiting_returns_429` | Rate limiter works at runtime | Burst of requests → 429 after limit |
| `test_body_size_limit_enforced` | Large body rejected | 2 MB body → 413 |
| `test_content_length_precheck` | Size checked before body read | Large Content-Length → 413 instantly |
| `test_valid_request_passes_all_checks` | Normal flow unimpeded | Valid request within all limits → 200 |

#### Smoke Tests

- [ ] Normal proxy request still works
- [ ] Rapid-fire 100 requests → eventually returns 429
- [ ] 2 MB POST body → 413
- [ ] GET to proxy route → 405

#### Definition of Done

- Per-client token bucket rate limiting with configurable limits
- Global rate limit circuit breaker
- Request body size limit enforced via Content-Length and stream
- Content-Type validation on POST
- 429, 413, 400, 405 responses with correct JSON bodies
- `Retry-After` header on 429 responses
- Memory bounded (client state eviction)
- All tests pass, lessons written

---

### Milestone 6 — Audit Logging, Observability, and Production Hardening

**Goal**: Add structured audit logging for all proxy requests, Prometheus metrics endpoint, request tracing with correlation IDs, CORS configuration, security headers, and graceful shutdown with drain timeout. Addresses SR-08, SR-09, SR-10, SR-11.

**Context**: M1–M5 deliver a functional, secure proxy. This milestone makes it production-ready with observability, operational safety, and security headers.

**Important design rule**: Audit logs are structured JSON written to stdout (for container log aggregation). They must include: timestamp, trace_id, client_id, provider, endpoint, request_size, response_status, latency_ms. They must NOT include: request/response bodies, API keys, authorization headers, or any secret value.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | All proxy requests (audit), config for CORS, metrics |
| Outputs | Structured JSON logs to stdout, `/metrics` Prometheus endpoint, security headers on all responses |
| Interfaces touched | New audit middleware, metrics endpoint, CORS config, security headers |
| Files allowed to change | `config.rs`, `main.rs`, `routes/mod.rs`, all middleware files |
| Files to read before changing anything | All M1–M5 files |
| New files allowed | `middleware/audit.rs`, `middleware/security_headers.rs`, `routes/metrics.rs` |
| New dependencies allowed | `metrics ^0.22`, `metrics-exporter-prometheus ^0.13`, `uuid ^1` (for trace IDs), `chrono ^0.4` (for timestamps) |
| Migration allowed | `no` |
| Compatibility commitments | All existing routes, auth, proxy, rate limiting unchanged |
| Forbidden shortcuts | No logging of request/response bodies, no logging of secrets, no disabling audit logging via config |

#### Out of Scope / Must Not Do

- No distributed tracing (OpenTelemetry — follow-up)
- No log shipping (handled by infrastructure)
- No alerting rules
- No dashboard definitions

#### BDD Acceptance Scenarios

**Feature: Audit logging**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Proxy request logged | happy path | Authenticated proxy request completes | Log output inspected | JSON log line with client_id, provider, endpoint, status, latency_ms |
| Failed auth logged | happy path | Unauthenticated request rejected | Log output inspected | JSON log line with `"auth": "rejected"`, no client_id |
| No secrets in logs | security | Any request | All log output inspected | No API keys, no auth tokens, no secret values |
| Trace ID in response | happy path | Any request | Response headers inspected | `X-Trace-Id` header present |

**Feature: Metrics**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Prometheus endpoint available | happy path | Server running | `GET /metrics` | 200 with Prometheus text format |
| Request count metric | happy path | 5 proxy requests made | `GET /metrics` | `api_facade_requests_total` counter ≥ 5 |
| Latency histogram | happy path | Proxy requests made | `GET /metrics` | `api_facade_request_duration_seconds` histogram present |

**Feature: Security headers**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Security headers present | happy path | Any response | Headers inspected | `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Cache-Control: no-store` |
| CORS configured | happy path | OPTIONS preflight request | Response headers | Correct `Access-Control-Allow-Origin`, methods, headers |

**Feature: Graceful shutdown**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| In-flight requests complete | happy path | 3 requests in flight, SIGTERM received | Requests complete | All 3 get responses, then server exits |
| Drain timeout exceeded | partial failure | Long-running request, SIGTERM, drain timeout 5s | After 5 seconds | Server force-exits, connection dropped |

#### Regression Tests

- All M1–M5 tests pass
- Proxy, auth, rate limiting all unchanged

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m6.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_audit_log_contains_required_fields` | Audit logging works | Log output has client_id, provider, status, latency |
| `test_no_secrets_in_logs` | No secret leakage | Log output grep for known test secret returns nothing |
| `test_metrics_endpoint` | Prometheus metrics work | `/metrics` returns text with expected counters |
| `test_security_headers_present` | Security headers applied | Response has all required headers |
| `test_trace_id_in_response` | Tracing works | `X-Trace-Id` header present and unique per request |
| `test_graceful_shutdown` | Drain works | SIGTERM → in-flight completes → server exits |

#### Definition of Done

- Structured JSON audit logs on every proxy request
- No secrets in any log output
- Prometheus metrics at `/metrics`
- `X-Trace-Id` on all responses
- Security headers on all responses
- CORS configurable via TOML
- Graceful shutdown with configurable drain timeout
- All tests pass, lessons written

---

### Milestone 7 — End-to-End Integration and Security Testing

**Goal**: Comprehensive end-to-end testing that validates the entire system works correctly as a production-grade API facade. This includes full-flow integration tests, security-focused tests (OWASP-aligned), performance baseline tests, and chaos/failure-mode tests.

**Context**: M1–M6 deliver all functional components. This milestone validates that they work together correctly and securely under realistic conditions. It does not add new features — it increases confidence in existing features.

**Important design rule**: This milestone focuses on testing and documentation only. No production code changes unless tests reveal bugs. If bugs are found, they are fixed in the affected module with a regression test.

**Refactor budget**: `No refactor permitted beyond direct implementation (bug fixes only)`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Realistic test scenarios covering the full request lifecycle |
| Outputs | Test reports, security audit documentation, performance baseline |
| Interfaces touched | None (test-only milestone) |
| Files allowed to change | Production code only for bug fixes discovered during testing |
| Files to read before changing anything | All production code |
| New files allowed | `tests/e2e_api_facade_m7.rs`, `tests/security_api_facade.rs`, `docs/SECURITY-AUDIT.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Everything still works |
| Forbidden shortcuts | No skipping tests, no `#[ignore]` without documented reason |

#### Out of Scope / Must Not Do

- No new features
- No performance optimization (only measurement)
- No infrastructure/deployment work

#### BDD Acceptance Scenarios

**Feature: Full-flow integration**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Complete happy path flow | happy path | Server running with all middleware, env secrets, mock upstream | Authenticated POST to proxy | Request proxied to upstream, response returned, audit log written, metrics updated, trace ID in response, no secrets leaked |
| Multiple providers | happy path | OpenAI and Anthropic configured | Requests to both providers | Each uses its own API key and auth format |
| Concurrent clients | concurrency | 10 concurrent authenticated clients | All send proxy requests simultaneously | All succeed or are rate-limited, no panics, no data leakage between clients |

**Feature: Security tests (OWASP-aligned)**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Header injection attempt | security | Client sends `Authorization: Bearer <malicious>\r\nX-Injected: true` | Proxy processes request | Header injection rejected, outbound request clean |
| Path traversal attempt | security | Client requests `/v1/proxy/openai/../../../etc/passwd` | Proxy processes request | Path normalized or rejected, no information disclosure |
| Oversized header attack | security | Client sends 100 KB of headers | Request processed | Rejected before processing |
| SSRF via provider config | security | Attacker tries to proxy to `http://169.254.169.254` (cloud metadata) | Outbound request | Blocked — only HTTPS to configured external hosts |
| Response splitting attempt | security | Upstream returns headers with `\r\n` | Response returned | Headers sanitized, no splitting |
| Slowloris simulation | security | Client sends request body 1 byte/second | After body timeout | Connection terminated, no resource leak |

**Feature: Failure modes**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Secret store down after startup | partial failure | Secret store goes unavailable, cache still warm | Proxy request | Serves from cache, logs warning |
| Secret store down, cache expired | partial failure | Secret store goes unavailable, cache expired | Proxy request | 502 returned, audit log records failure |
| Upstream timeout | partial failure | Upstream takes 60s to respond | Proxy request (30s timeout) | 504 `{"error": "gateway_timeout"}` |
| Config reload | operational | Config file changed (if hot reload supported) | New request | Picks up new config |

#### E2E Runtime Validation

**File**: `tests/e2e_api_facade_m7.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_complete_flow_end_to_end` | Full system works | Single request exercises auth → authz → rate limit → proxy → audit |
| `test_concurrent_clients_no_cross_leak` | No data leakage under concurrency | 10 clients, each sees only their data |
| `test_header_injection_blocked` | Header injection prevented | Malicious headers rejected |
| `test_path_traversal_blocked` | Path traversal prevented | Normalized or rejected |
| `test_ssrf_external_only` | SSRF mitigated | Private IP ranges blocked |
| `test_secret_store_failover_to_cache` | Cache provides resilience | Serves from cache when store is down |
| `test_upstream_timeout_handled` | Timeout handled gracefully | 504 returned, no hung connections |

**File**: `docs/SECURITY-AUDIT.md`

Document the results of all security tests, threat model validation, and any findings.

#### Smoke Tests

- [ ] Full end-to-end flow works with all middleware active
- [ ] No panics under any test scenario
- [ ] No secrets in any output (logs, responses, metrics)
- [ ] Memory usage stable under sustained load
- [ ] All previous milestone tests still pass

#### Definition of Done

- All integration tests pass
- All security tests pass or findings documented
- Performance baseline measured and documented
- `SECURITY-AUDIT.md` written with threat model validation results
- All previous milestone tests pass
- Final `ARCHITECTURE.md` update reflects complete system
- Final `README.md` includes setup, configuration, and running instructions
- Lessons and completion summary written
- All milestone tracker entries are `done`

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | ARCHITECTURE.md Update | README.md Update | Other Docs |
|---|---|---|---|
| 1 | Add API Facade Gateway section, crate layout | Add crate to workspace overview | None |
| 2 | Add secret store subsystem | Document secret store backends | None |
| 3 | Add authentication and authorization | Document auth configuration | None |
| 4 | Add proxy subsystem, data flow | Document provider config, example curl | None |
| 5 | Add rate limiting, body limits | Document rate limit config | None |
| 6 | Add audit logging, metrics, security headers | Document observability config | None |
| 7 | Final review and completeness check | Full setup/running instructions | `docs/SECURITY-AUDIT.md` |

---

## Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## Appendix A: Provider Configuration Example

```toml
# config/default.toml

[server]
host = "0.0.0.0"
port = 8080
drain_timeout_secs = 30

[secret_store]
backend = "env"  # "vault" | "aws" | "env"
cache_ttl_secs = 300

[secret_store.vault]
address = "https://vault.example.com:8200"
token_path = "/run/secrets/vault-token"
mount_path = "secret"

[secret_store.aws]
region = "us-east-1"

[auth]
mode = "jwt"  # "jwt" | "api_key" | "both"

[auth.jwt]
algorithm = "RS256"
issuer = "https://auth.example.com/"
audience = "api-facade"
jwks_url = "https://auth.example.com/.well-known/jwks.json"

[auth.api_keys]
# Keys are stored as SHA-256 hashes, not plaintext
hashes = [
    { hash = "sha256:abc123...", client_id = "service-a", scopes = ["openai:*"] },
    { hash = "sha256:def456...", client_id = "service-b", scopes = ["openai:chat/completions", "anthropic:messages"] },
]

[rate_limit]
default_requests_per_second = 10
default_burst = 20
global_requests_per_second = 1000
global_burst = 2000

[rate_limit.per_provider.openai]
requests_per_second = 50
burst = 100

[body_limit]
max_request_bytes = 1_048_576   # 1 MB
max_response_bytes = 10_485_760 # 10 MB

[cors]
allowed_origins = ["https://app.example.com"]
allowed_methods = ["POST", "OPTIONS"]
allowed_headers = ["Authorization", "Content-Type", "X-API-Key"]
max_age_secs = 3600

[providers.openai]
base_url = "https://api.openai.com/v1"
secret_key = "openai_api_key"        # key name in secret store
auth_header = "Authorization"
auth_prefix = "Bearer "
allowed_endpoints = ["chat/completions", "embeddings", "models"]
header_allowlist = ["Content-Type", "Accept", "OpenAI-Organization"]
timeout_secs = 30
max_retries = 0

[providers.anthropic]
base_url = "https://api.anthropic.com/v1"
secret_key = "anthropic_api_key"
auth_header = "x-api-key"
auth_prefix = ""
allowed_endpoints = ["messages"]
header_allowlist = ["Content-Type", "Accept", "anthropic-version"]
timeout_secs = 30
max_retries = 0

[logging]
format = "json"   # "json" | "pretty"
level = "info"
audit_enabled = true

[security_headers]
hsts_max_age = 31536000
```

## Appendix B: Error Response Format

All error responses follow this structure:

```json
{
    "error": "error_code",
    "message": "Human-readable description safe for clients",
    "trace_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

Error codes:
- `unauthorized` — 401
- `forbidden` — 403
- `not_found` — 404
- `method_not_allowed` — 405
- `payload_too_large` — 413
- `too_many_requests` — 429
- `bad_request` — 400
- `bad_gateway` — 502
- `gateway_timeout` — 504
- `internal_error` — 500

**Never included in error responses**: stack traces, file paths, internal module names, secret values, upstream error details that may contain auth info.

## Appendix C: Security Requirements Traceability

| SR | Description | Milestone | Verified In |
|---|---|---|---|
| SR-01 | TLS 1.2+ on all inbound connections | M6 (documented; TLS termination at LB) | M7 |
| SR-02 | Client authentication mandatory on proxy routes | M3 | M3, M7 |
| SR-03 | Per-client scopes for providers and endpoints | M3 | M3, M7 |
| SR-04 | Secrets in secret store, wrapped with `secrecy` | M2 | M2, M7 |
| SR-05 | TLS + header allowlist on outbound requests | M4 | M4, M7 |
| SR-06 | Request/response body size limits | M5 | M5, M7 |
| SR-07 | Per-client rate limiting | M5 | M5, M7 |
| SR-08 | Structured audit logging, no secrets | M6 | M6, M7 |
| SR-09 | Generic error responses | M1 | M1, M7 |
| SR-10 | Health/readiness don't leak info | M1 | M1, M7 |
| SR-11 | Graceful shutdown with drain | M6 | M6, M7 |
| SR-12 | Audited crypto libraries only | M3 | M3, M7 |
