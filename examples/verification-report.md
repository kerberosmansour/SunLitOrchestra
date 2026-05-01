---
synthetic: true
non-normative: true
abbreviates: slo-verify
---

# Synthetic Verification Report — Widget Index Page M1

> **Synthetic, non-normative.** Pass results are invented. Read the canonical `/slo-verify` SKILL.md for the real contract.

## Pass 1 — Build / boot

| Check | Command | Result |
|---|---|---|
| Build | `cargo build --workspace` | clean |
| Boot | `widget-svc --config dev.toml` | listening on :8080 in 312ms |

## Pass 2 — BDD acceptance

| Scenario | Result |
|---|---|
| Owner lists own widgets | PASS (3 widgets returned) |
| Cross-tenant peek attempt | PASS (403 returned; cursor signature invalid) |
| Limit clamped above ceiling | PASS (50 widgets returned for `?limit=500`) |
| Cursor expiry (added per F-ENG-1 critique) | PASS (410 returned for 25h-old cursor) |

## Pass 3 — E2E runtime

| Test | Pass criteria | Actual |
|---|---|---|
| `e2e_widget_list_owner_only` | session A sees only A's widgets across 100 list iterations | 100/100 OK |
| `e2e_widget_list_load` | 100 concurrent listers; p95 < 80ms | p95 = 64ms |

## Pass 4 — Scanner findings

| Finding | Severity | Source | Action |
|---|---|---|---|
| `cwe-639-cursor-cross-session` | high | semgrep `cursor-sign-must-bind-session` | OPEN — see expanded below |
| `cwe-117-log-injection` (vendor lib) | low | semgrep | accepted-risk; vendor patch tracked |

### [HIGH] cwe-639-cursor-cross-session

Semgrep rule `cursor-sign-must-bind-session` flagged `widget-svc/src/cursor.rs:18` — `Cursor::sign(payload, key)` does not include `session_id` in the signing input.

#### Concrete scenario

(See [critique-report.md](critique-report.md) F-SEC-1.) Cross-session cursor reuse — same finding pattern.

#### Remediation

Pass `session_id` as additional signing input. Update `Cursor::sign(payload, key, session_id)`.

#### Verification

- Pre-fix: scanner row present with severity `high`.
- Post-fix: re-run `cargo run -p sast-verify -- gate`; expected output `0 findings of severity high`.

## Conclusion

All 4 passes green. M1 ships pending the cursor-bind fix (one-line change; tracked as inline-fix in M1 close-out).
