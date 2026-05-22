# Verification Report — kani Milestone 2

Harness-generation + run/triage methodology references + honesty/scope gates. No UI surface (Markdown references + Rust structural assertions).

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| Naive-first rule documented | happy path | `slo_kani_naive_first_documented` | pass | `run-and-triage.md` contains "pre-fix variant must fail first" |
| Concurrency refusal | abuse (`tm-…-abuse-2`) | `slo_kani_honesty_and_concurrency_gates_present` (M1, still green) | pass | "concurrency is out of scope" in SKILL.md + fallback ref |
| Vacuity defense | abuse (`tm-…-abuse-1`) | `slo_kani_naive_first_documented` + cover! prose | pass | naive-first is the universal anti-vacuity gate; `cover!` documented |
| Unsound stub rejected | abuse (`tm-…-abuse-3`) | `slo_kani_sound_stub_rule_documented` | pass | `fallback-strategies.md` carries "sound over-approximating stubs only" |
| Verdict authority | invalid input | `slo_kani_verdict_from_tool_documented` | pass | `run-and-triage.md` carries "never from narration" |
| Timeout triage | partial failure | failure-ladder table in `run-and-triage.md` | pass | reduce bound → solver → stub/contract documented |
| Scope block mandatory | assertion violation | `verified-scope-writeup.md` rules | pass | "a green with no scope block is rejected" |
| Fail-closed parsing (ENG-2) | dependency failure | `slo_kani_parser_fails_closed_documented` | pass | `run-and-triage.md` carries "fail closed" + "non-pass", version-anchored |
| Write-path traversal (SEC-1) | abuse (`tm-…-abuse-5`) | `slo_kani_write_path_validation_documented` | pass | `harness-generation.md` carries "target-crate root" + reject `..`/absolute/`symlink` (CWE-22) |

Negative path: the 7 new M2 assertions all failed red when the references/evals were absent (confirmed during execution), then went green once authored. The sound-stub assertion additionally caught a capitalization mismatch (same class as the kani-m1 lesson) — fixed by case-insensitive comparison. Gates fail-closed; they do not pass vacuously.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none (one in-flight capitalization fix during execution; test is the guard) | `slo_kani_sound_stub_rule_documented` | resolved |

## Pass 4 — Security

| Stack | Check | Result | Evidence |
|---|---|---|---|
| Rust | `cargo audit` | pass | exit 0; no new deps in M2 |
| — | DAST | N/A | Markdown references + structural test; no smoke service |
| — | biz-public PII scan | N/A | M2 touched no `docs/biz-public/` artifacts |

Threat-model read-side: M2 turns the documented controls for `tm-kani-verification-abuse-1` (anti-vacuity), `-2` (concurrency refusal), `-3` (sound stubs), and `-5` (write-path) into asserted methodology. All active abuse rows now have asserted coverage in the skill prose.

## Pass 5 — AI tolerance

N/A — no AI runtime behavior exercised; M2 ships methodology prose + deterministic structural assertions. (The prose governs the LLM's harness-authoring; that behavior is exercised in M4.)

## Environment
- macOS; `cargo test -p sast-verify`; `cargo audit` (advisory DB reachable).

## Coverage gaps
- The methodology's *behavioral* efficacy (does the agent actually fail-close, refuse concurrency, reject unsound stubs when driving real Kani?) is exercised end-to-end in M4's demo and the adversarial eval. M2 verifies the gates are documented and asserted to exist.

## Verdict
All M2 BDD scenarios pass at runtime; Pass 4 clean; verified.
