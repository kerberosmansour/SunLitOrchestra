---
synthetic: true
non-normative: true
abbreviates: slo-critique
---

# Synthetic Critique Report — Widget Index Page

> **Synthetic, non-normative.** Persona findings are invented. Do not treat as a template for real critique outputs — read the canonical SKILL.md.

## Findings summary

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| F-CEO-1 | ceo | hold-scope | M1 | M1 ships an MVP listing; backlog item ICE-scored next sprint covers filters | next sprint adds `GET /widgets?status=archived`; M1 unblocks the path | Hold scope; do not bundle filter work into M1 |
| F-ENG-1 | eng | ask | M1 BDD | No BDD scenario for cursor expiry | Cursor signed at t=0; user resumes at t=24h; signature still valid; widgets in flight may have changed | Add BDD: cursor expires after 1h; expired cursor returns 410 |
| F-SEC-1 | security | ask | M1 Contract Block | V3 IDOR class is mitigated by `OwnedWidgetRef`, not eliminated — the cursor-decode path receives raw IDs from the wire | Authenticated session A submits a cursor whose decoded payload references widget owned by B; if cursor signature is valid (e.g., B's signed cursor was leaked) handler emits B's widgets | Variant analysis: `rg -n 'OwnedWidgetRef::new\(' widget-svc/src/`; expand to expanded finding (see below) |
| F-DESIGN-1 | design | n/a | — | No UI surface in M1 (API-only) | — | — |

## Expanded findings

### [HIGH] V3 IDOR — cursor-decode bypasses owner check

| Field | Value |
|---|---|
| ID | `wgt-sec-1` |
| Source | `/slo-critique` security persona |
| Status | `open` |
| Confidence | `medium` |
| Location | `widget-svc/src/handlers/list.rs:42` (proposed) |
| Affected surface | `GET /widgets?cursor=<opaque>` |
| Bug class / CWE | V3 Broken Access Control / `CWE-639` |
| Standards mapping | OWASP Top-10 A01; ASVS V4.1.3 |
| Threat-model row | `tm-widget-abuse-1` |

#### Concrete scenario

Given session A and a cursor that was originally signed for session B (e.g., leaked via a logged URL or a malicious browser extension), when session A submits `GET /widgets?cursor=<B-cursor>`, then the cursor signature verifies (because the server's signing key is shared) and the handler emits widgets B owns — bypassing the per-handler owner check that `OwnedWidgetRef` was supposed to enforce.

#### Remediation

Bind the cursor signature to the requesting session's id. Cursor signing key derives from `(server_secret, session_id)`. A B-cursor presented by session A fails verification because the derivation differs.

#### Verification

- Add BDD: "cross-session cursor reuse rejected"
- Variant scan: `rg -n 'cursor::sign\(' widget-svc/src/` — confirm all sign sites pass `session_id`
