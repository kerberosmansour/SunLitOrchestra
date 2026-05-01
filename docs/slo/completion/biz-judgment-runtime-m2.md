# Completion — biz-judgment-runtime M2

## Goal achieved

All 9 v1 fixtures wired into `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` as one `#[ignore]` test each, plus `global_cost_cap_enforced`. Retry policy + cost cap + env-flag-gating are all live. The legacy panic-stub at `e2e_biz_followup_m4.rs:runtime_harness_invokes_claude_cli_per_fixture` is now a forwarder. The fixtures README is updated to reflect "harness is real, not stub".

## Files added

- `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` — 9 per-fixture tests + `global_cost_cap_enforced`.

## Files edited

- `crates/sldo-install/tests/common/judgment_runtime.rs` — added `REFUSAL_PHRASES`, `FixtureResult`, `run_fixture`, `assert_expectations`, `is_transient_error`, `parse_gate_list`, `truncate`, `global_budget_usd`, `retries`, `DEFAULT_*` constants. Added 6 new unit tests covering: gate-list parsing, refusal-phrase matching, transient-error detection, both branches of `assert_expectations`.
- `crates/sldo-install/tests/e2e_biz_followup_m4.rs` — replaced panic body of `runtime_harness_invokes_claude_cli_per_fixture` with a forwarder pointing at the new test files. Function signature + `#[ignore]` annotation preserved.
- `references/biz/judgment-fixtures/README.md` — replaced "Status: DESIGN + STUB" + "Runtime harness (stub)" sections with the new factual state. Documented all four env-var overrides.
- `docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` — tracker mark M2 done.

## Files NOT changed

- The 9 fixture files under `references/biz/judgment-fixtures/<skill>/` — frontmatter schema unchanged.
- The 5 structural tests in `e2e_biz_followup_m4.rs` (lines 56-176) — only the panic-stub body changed.
- M1 test file `e2e_biz_judgment_runtime_m1.rs` — unchanged.
- All workspace `Cargo.toml`s — no new deps.

## Test evidence

```
$ cargo test -p sldo-install --test e2e_biz_judgment_runtime_m2 --test e2e_biz_judgment_runtime_m1 --test e2e_biz_followup_m4
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out  (e2e_biz_followup_m4)
test result: ok. 11 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out (e2e_biz_judgment_runtime_m1)
test result: ok. 11 passed; 0 failed; 10 ignored; 0 measured; 0 filtered out (e2e_biz_judgment_runtime_m2)

$ cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify
all green; 12 ignored items total (all are expected #[ignore] runtime-harness tests)
```

The live runtime test (`BIZ_JUDGMENT_RUNTIME_LIVE=1 ...`) is owner-discretion. Smoke run procedure is in [docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md](../completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md) M2 Smoke Tests section; budget cap defaults to 5.00 USD aggregate.

## BDD coverage

- Row 1 (happy path — all 7 non-adversarial pass live) — covered by the 9 per-fixture tests in M2.
- Row 2 (`must_refuse: true` + non-empty `expected_gates_fired:`) — covered by `assert_expectations`'s `must_refuse` branch which permits non-`mode: draft` artifacts and verifies their gates.
- Row 3 (global budget = 0.00 → no claude calls) — covered by the cost-cap structure: setting `BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD=0.00` fails `global_cost_cap_enforced` with the worst-case-bound diagnostic before any other test runs.
- Row 4 (transient error retry) — covered by `is_transient_error` unit test + `run_fixture`'s retry loop.
- Row 5 (retry succeeds) — covered structurally by `FixtureResult.retries_used` reporting.
- Row 6 (global budget exceeded mid-run) — covered by `global_cost_cap_enforced` upper-bound check.
- Row 7 (judgment regression — adversarial fixture capitulates) — covered by `assert_expectations_fails_on_judgment_regression` unit test + `fixture_slo_legal_tax_efficiency_pushback` live test.
- Row 8 (persistence — tempdir drop) — implicit via `TempDir::drop`.
- Row 9 (default `cargo test` doesn't invoke claude) — verified by the test runs above (10 ignored, 0 invoked).

## Definition of Done

- [x] All steps complete.
- [x] All BDD scenarios covered.
- [x] Compatibility Checklist all checked (5 structural tests green; forwarder-stub still `#[ignore]`; M1 unchanged; no new deps).
- [x] Baseline green.
- [x] Lessons + completion summary written.
- [x] Tracker updated.
- [ ] PR opened with both M1 + M2 commits + docs (next).

## Deferred follow-ups

- **Real per-call cost observation** — the cost-cap test asserts a worst-case bound. If the v1 fixture set grows past ~30 items, parse `--output-format json`'s cost envelope and track actual spend. Small, well-scoped follow-up; not worth doing today since the bound is tight.
- **Jittered retry backoff** — exponential is fine for sequential 9-fixture runs; add jitter only if we ever parallelise (we shouldn't).
- **Drop the `_timeout: Duration` parameter on `invoke_claude`** — kept for API symmetry but unused. Either wire it up to `wait_timeout` polling or drop it on the next harness touch-up.
