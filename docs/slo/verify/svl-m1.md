# Verification Report — svl Milestone 1

Milestone: Canonical doc + v4-template Secure Value & Security Contract section.
Target kind: docs + v4-template (no UI, no service). "Runtime QA" = structural-contract suite + backward-compat trace + Pass 4 security.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| §5B present in both copies + all 5 sub-blocks | happy path | `svl_m1::secure_value_section_present_in_both_template_copies` | pass | 8/8 svl_m1 green |
| Templates byte-identical | compatibility | `svl_m1::template_copies_stay_byte_identical` + `diff` empty | pass | "BYTE-IDENTICAL" |
| §5A/§6/§10/§17 not renumbered (legacy runbooks resolve) | compatibility | `svl_m1::existing_sections_not_renumbered` | pass | headings intact → legacy runbooks parse unchanged |
| Proactive-controls row cites OWASP 2024 by name | invalid input (anti-drift) | `svl_m1::proactive_controls_row_named_and_editioned` | pass | "OWASP Proactive Controls 2024" + named control |
| Canonical doc complete (rule, Bundles A–F, SSDF+OWASP 2024) | happy path | `svl_m1::canonical_doc_present_and_complete` | pass | |
| Ledger↔lane mapping + 5 statuses + unknown→blocked | happy path | `svl_m1::canonical_doc_ledger_lane_mapping_and_status_values` | pass | no new lane verb; reuses upstream-OSS/slo-process |
| Fence rule + named generation surfaces | abuse case `tm-secure-value-loop-abuse-1` | `svl_m1::canonical_doc_fence_rule_and_named_surfaces` + grep | pass | 3× `~~~text`; `/slo-ship` + `/slo-resume` named (F-SEC-3) |
| Agent one-page prompt present | happy path | `svl_m1::canonical_doc_has_agent_prompt` | pass | |
| Full suite incl. mloop byte-identity + no-renumber | regression (F-ENG-4) | `cargo test -p sast-verify` | pass | 21 test files green |

## Pass 4 — Security

Threat-model `.slo.json` present and read (abuse IDs not re-derived). M1 active abuse case in scope: `tm-secure-value-loop-abuse-1` (contract string injection).

| Check | Result | Evidence |
|---|---|---|
| `tm-secure-value-loop-abuse-1` — `~~~text` fence rule documented + scoped to named surfaces | pass | §7 of SECURE-VALUE-LOOP.md names `/slo-ship` PR body + `/slo-resume` snippets; agent prompt itself fenced |
| Secrets scan over new M1 files (`docs/SECURE-VALUE-LOOP.md`, `svl_m1.rs`) | pass | heuristic grep — only false matches on the literal word "secrets" in the Bundle-table prose; no credential material |
| `gitleaks` | skipped — not installed (`brew install gitleaks` / `go install github.com/gitleaks/gitleaks`) | tool-optional rule |
| SAST / SCA / IaC / container | N/A — no code/deps/IaC/images changed (docs + 1 Rust test only) | the lone Rust file is a structural test; covered by clippy (no new warnings) |
| DAST | N/A — markdown-only milestone, no compiled artifact / smoke service | smoke-service gate |
| Biz-pack PII scan | N/A — milestone touches no `docs/biz-public/` artifact | |

`accepted_residual` rows from the `.slo.json` (thin/dishonest contract; guidance-not-runtime; bundle drift) are knowingly accepted — not findings.

## Pass 5 — AI tolerance

N/A — no AI component (`ai_component: false`).

## Pass 6 — Measurement

N/A — milestone Contract Block Measurement deliverables row is `N/A — not value-bearing (tooling/process)`.

## Bugs found

None.

## Environment

- macOS (Darwin 25.5.0); Rust toolchain (workspace `cargo test -p sast-verify`); no Node/UI path.

## Coverage gaps

- Runtime agent adherence to the §5B prose (e.g. an agent actually fencing a user string when generating a PR body) is verified structurally as **contract-text presence**, not behaviourally — that is the documented accepted residual (F-SEC-2); the behavioural surface lands with the `/slo-ship` generation step in M5.
