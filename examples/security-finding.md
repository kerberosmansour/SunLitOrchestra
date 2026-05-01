---
synthetic: true
non-normative: true
abbreviates: references/security/security-finding-template.md
---

# Synthetic Security Finding — Cursor cross-session reuse

> **Synthetic, non-normative.** This file abbreviates the canonical security-finding template. Real findings live in `docs/slo/critique/<slug>.md` or `docs/slo/verify/<slug>.md`.

### [HIGH] V3 IDOR — cursor signature does not bind to session

| Field | Value |
|---|---|
| ID | `wgt-sec-1` |
| Source | `/slo-critique` security persona / `/slo-verify` Pass 4 (semgrep `cursor-sign-must-bind-session`) |
| Status | `open` |
| Confidence | `medium` |
| Location | `widget-svc/src/cursor.rs:18` |
| Affected surface | `GET /widgets?cursor=<opaque>` (paginated list endpoint) |
| Data classification | `Internal` |
| Threat-model row | `tm-widget-abuse-1` |
| Bug class / CWE | V3 Broken Access Control / `CWE-639` (Authorization Bypass Through User-Controlled Key) |
| Standards mapping | OWASP Top-10 A01; ASVS V4.1.3 |

#### Concrete scenario

Given session A authenticated as user-a, and a cursor signed-and-leaked from session B (user-b) — leakage path: server access logs include the full URL with `?cursor=<B-cursor>`; an attacker reads the log; OR a browser extension exfiltrates the URL — when session A submits `GET /widgets?cursor=<B-cursor>`, then the server's cursor verifier checks the signature against the shared signing key, the signature is valid (the cursor *was* signed by the server), and the handler decodes it and emits widgets with `owner_id == user-b`. Session A sees user-b's widgets; the per-handler `OwnedWidgetRef` boundary check is bypassed because the cursor-decoded widget IDs were already considered "trusted" by the time `OwnedWidgetRef::new` ran.

#### Evidence

- `widget-svc/src/cursor.rs:18` — `Cursor::sign(payload, key)` signs `(payload, key)` only; no `session_id` mixed in.
- Semgrep rule `cursor-sign-must-bind-session` (synthetic) caught the pattern.
- Reproduction: see `tests/api/widgets_cross_session_cursor.rs` (proposed).

#### Impact

Authenticated user can list any other user's widgets when a cursor leak occurs. Privacy violation (widget metadata may include customer-confidential fields). Compliance: arguable GDPR Art. 32 inadequate-controls finding.

#### Remediation

Modify `Cursor::sign(payload, key)` → `Cursor::sign(payload, key, session_id)`. Derive the per-cursor key from `HKDF(server_secret, session_id)` so cursors signed for one session fail verification when presented by another.

#### Verification

- Pre-fix unit test: `cursor::tests::cross_session_cursor_rejected` — expect FAIL (regression test).
- Post-fix unit test: same test — expect PASS.
- Integration: `e2e_widget_list_cross_session_cursor_rejected` returns 403.
- Scanner re-run: `cargo run -p sast-verify -- gate` reports `0 findings of severity high` in `cursor.rs`.
