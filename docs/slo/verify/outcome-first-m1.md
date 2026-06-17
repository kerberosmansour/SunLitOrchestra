# Verification Report — outcome-first Milestone 1

Template contract (v4 §5C + §17 sub-sections + §11 Outcome layer + §6.12 rule). No UI surface (Markdown template + a Rust structural-contract test), so no Playwright; the structural test `xtasks/sast-verify/tests/outcome_first_m1_template.rs` is the runtime gate.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| Sections present (§5C/§17/§11/§6.12) | happy path | `template_has_outcome_first_sections` | pass | 8/8 green |
| Resolution enum never blank | happy path | `template_regression_matrix_resolution_never_blank` | pass | green |
| §5C after §5B, no renumber | backward compat | `template_5c_after_5b_no_renumber` | pass | green |
| v3 untouched | backward compat | `v3_template_untouched_no_5c` | pass | green |
| Per-layer Front-to-End + ≥1 cross-layer (theme B) | partial failure / theatre | `template_per_layer_front_to_end_with_cross_layer_assertion` | pass | green |
| Authored-string fence | abuse case `tm-outcome-first-abuse-1` | `template_carries_authored_string_fence_rule` | pass | green |
| Frozen `oc-`/`cuj-` id schemes | happy path | `template_defines_frozen_id_schemes` | pass | green |
| Both copies carry §5C | dual-copy parity | `both_template_copies_have_5c` + `diff` empty | pass | byte-identical |
| Empty/first-run (sections optional) | empty state | additive insertion; legacy runbooks still satisfy the template (no consumer test fails for absence) | pass | sections are optional-by-shape (§5A/§5B precedent) |
| No regression in template-reading tests | regression | full suite incl. `svl_m1`/`svl_m3`/`mloop_m3_plan` (byte-identity) + `mloop_m3_plan` §5A markers + `svl_m3` §5B markers | pass | 30 suites, 0 failed |

## Pass 4 — Security

| Check | Result | Note |
|---|---|---|
| Bundle A (docs/planning) — authored-string injection | pass | `tm-outcome-first-abuse-1` fence rule present + asserted by the M1 test |
| `.slo.json` read-side contract | pass | schema 0.1.0 validates; abuse-1 `status: active`; IDs not re-derived |
| SAST/SCA/secrets | not_applicable | no application code / no dependency change in M1 |
| DAST | N/A — markdown template + a Rust test; no smoke service / no compiled released artifact |
| `docs/biz-public/` PII scan | not_applicable | M1 touches no `docs/biz-public/` artifact |

## Pass 5 — AI tolerance

`N/A — no AI component` (Markdown template + a deterministic Rust test).

## Pass 6 — Measurement

`skipped — no telemetry context`: the runbook §5A is dogfood-measured with **no runtime telemetry surface**; M1 ships template sections + a structural test, not behavioural events. The "saved query" (grep for the §5C/§17 headings) resolves green.

## Bugs found

None.

## Environment

- macOS (Darwin 25.5.0); Rust toolchain via `cargo`; `sast-verify` test crate. No browser/Node needed (no UI).

## Coverage gaps

- None for M1's contract. The *executable* proof that the gate **fires** at runtime (a real Pass 0 blocking a real milestone) is deliberately deferred to the **theme-A mid-stream dogfood checkpoint after M3** — M1's structural test proves only that the contract surface is *documented*, which is M1's whole scope.

## Disposition

M1 verified — every BDD/outcome assertion passes at runtime; full suite green; no bugs. Pre-existing clippy debt recorded as DW-002 (`file_github_issue`, out of allow-list). Ready for `/slo-retro M1`.
