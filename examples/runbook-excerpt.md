---
synthetic: true
non-normative: true
abbreviates: docs/slo/templates/runbook-template_v_4_template.md
---

# Synthetic Runbook Excerpt — Widget Index Page (v4)

> **Purpose**: Illustrate one milestone of a v4 runbook end-to-end. Synthetic; not a template.

## Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `widget-index` |
| Prefix | `wgt` |
| Primary stack | Rust + axum + sqlx + Postgres |
| Default unit test | `cargo test -p widget-svc` |
| Default lint | `cargo clippy --workspace -- -D warnings` |

## Milestone 1 — `Read-only widget listing endpoint`

**Goal**: Add `GET /widgets` returning a paginated JSON list of widgets owned by the caller.

**Carmack-style reliability goal**: Make "list a widget you don't own" architecturally impossible via the typed `OwnedWidgetRef` boundary — the handler cannot construct one without the caller's session id.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

### Contract Block

| Field | Value |
|---|---|
| Inputs | Authenticated `GET /widgets?cursor=<opaque>&limit=<1..50>` |
| Outputs | `200 OK` with `{ widgets: [...], next_cursor: Option<String> }` |
| Files allowed to change | `widget-svc/src/handlers/list.rs` (NEW), `widget-svc/src/routes.rs`, `tests/api/widgets_list.rs` (NEW) |
| Resource bounds | `limit` clamped to `[1, 50]`; pagination cursor opaque, server-signed |
| Invariants | Handler can only emit widgets where `widget.owner_id == session.user_id` |
| Data classification | `Internal` |
| Proactive controls | `C5` (validate inputs), `C7` (access controls via `OwnedWidgetRef`) |
| Abuse acceptance scenarios | `tm-widget-abuse-1: caller passes another user's widget id in cursor → handler rejects with 403` |

### BDD Acceptance Scenarios

| Scenario | Given | When | Then |
|---|---|---|---|
| Owner lists own widgets | session A; 3 widgets owned by A | `GET /widgets` | 200; 3 widgets returned |
| Cross-tenant peek attempt | session A; cursor signed for session B | `GET /widgets?cursor=<B-cursor>` | 403; cursor signature invalid |
| Limit clamped above ceiling | session A | `GET /widgets?limit=500` | 200; ≤ 50 widgets returned |

### Definition of Done

- 3 BDD scenarios pass.
- `cargo clippy -- -D warnings` clean.
- `OwnedWidgetRef` constructor takes `session_id` parameter; cannot be elided.
- Lessons file written.
