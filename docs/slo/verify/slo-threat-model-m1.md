# Verification Report — slo-threat-model Milestone 1

Milestone: M1 — Schema reference + dogfood fixture + frozen-ID guard.
Date: 2026-05-19. Verifier: /slo-verify. Branch: `slo/slo-threat-model-m1`.

M1 is a backend/contract milestone (Markdown schema doc + JSON fixture + Rust
structural-contract test). No UI surface → Playwright cascade skipped. Runtime
verification = forcing each negative BDD scenario by mutating a copy of the
fixture and observing the guard fail, then restoring byte-identically.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `schema_doc_and_fixture_conform` | happy path | `cargo test -p sast-verify --test slo_tm_m1_schema` on the good fixture | pass | 6/6 tests green; `schema_doc_has_required_sections` + `fixture_parses_and_conforms` ok |
| `unknown_top_level_key_rejected` | invalid input | Added `evil_payload` top-level key to a fixture copy; re-ran test | pass | `deny_unknown_fields` → strict parse failed; all `load_fixture()` tests FAILED as required (proven on the file path, not only in-memory) |
| `empty_abuse_cases_rejected` | empty state | Set `abuse_cases: []` on a copy; re-ran | pass | `abuse_cases_are_non_empty` FAILED with the intended message |
| `malformed_json_is_a_hard_parse_error` | dependency failure | Replaced fixture with `{ this is not json`; re-ran | pass | `serde_json` hard parse error; never string-scanned around (ENG-2 satisfied) |
| `renumbered_abuse_id_fails` | abuse case | Renumbered `tm-slo-sec-abuse-3` → `…-99` on a copy; re-ran | pass | `frozen_id_invariant_holds` FAILED — **the wedge's core guarantee proven at runtime**: a renumbered abuse id is caught |
| `silent_drop_of_superseded_fails` | abuse case | Deleted `tm-slo-sec-abuse-4` on a copy; re-ran | pass | `frozen_id_invariant_holds` FAILED on the contiguity check (a silent drop is caught) |
| (extra) invalid `classification` enum | invalid input | Set a classification to `"secret"` on a copy; re-ran | pass | `every_abuse_and_residual_carries_classification` FAILED as required |

All "pass" rows for negative scenarios mean the guard **correctly failed** the
mutated input. Restore integrity: fixture SHA-256 after all mutations ==
pre-verification SHA-256 (`d5191539…818e2d`) — no residue.

## Pass 4 — Security (supply-chain + selector)

| Check | Stack | Result | Evidence |
|---|---|---|---|
| `cargo audit` | Rust | pass | exit 0; 118 crate deps scanned, 0 vulnerabilities; M1 added no dependency |
| `cargo deny check advisories` | Rust | pass | `advisories ok`, exit 0 (default config) |
| `semgrep` | Rust | N/A | M1 surface is a JSON fixture + Markdown schema doc + one Rust test; the `.semgrep/rust` pack targets `src/`, not test/fixture artifacts — no meaningful sink introduced |
| `ast-grep` | Rust | skipped | not on PATH (install hint: `cargo install ast-grep`) |
| DAST | — | N/A | no compiled artifact / no smoke service — contract + test milestone (anti-pattern guard: do not DAST a docs/contract target) |
| Biz-pack PII scan | — | N/A | `docs/biz-public/` not present in this repo state; nothing to scan |

Pass 4 threat-model linkage: M1's own surface is covered by the contract it
ships — `tm-slo-threat-model-abuse-2` (unknown top-level key) and
`tm-slo-threat-model-abuse-7/8` (renumber / silent-drop) were each exercised
above and the guard fired. No Pass 4 finding; no regression test required.

## Pass 5 — AI tolerance

N/A — no AI component. M1 ships a static schema doc, a static JSON fixture,
and a deterministic Rust test (Contract Block AI-tolerance row = `N/A`).
`ai_component: true` applies to the M2 consumer surface, not M1.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none | — | no bugs found across Passes 1–5 |

## Environment

- OS: macOS (Darwin 25.4.0), arm64.
- Toolchain: cargo/rustc workspace; `sast-verify` package.
- cargo-audit 0.22.1, cargo-deny 0.19.0, semgrep present, ast-grep absent.
- No browser/Node needed (no UI surface).

## Coverage gaps

- None. Every M1 BDD scenario was exercised at runtime, including all
  negative/abuse scenarios via controlled fixture mutation.
- `superseded`-status happy path (a row with `status: superseded` +
  `superseded_by` + `supersede_reason`) is asserted by the test logic but the
  dogfood model has no superseded rows, so it was exercised by the schema/test
  path, not by a live superseded fixture row. Noted for M2/producer work; not
  an M1 gap (the source model legitimately has none).
