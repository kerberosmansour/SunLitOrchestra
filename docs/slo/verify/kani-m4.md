# Verification Report — kani Milestone 4

The failure-bar demonstration. Unlike M1–M3 (structural), M4 is a **live Kani run** against a real seeded-bug crate (`kani-verifier 0.67.0`).

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| K1 off-by-one caught | happy path | `cargo kani --harness check_zero_prefix` (buggy) | pass (red) | FAILURE: `index out of bounds` |
| K1 remediated | happy path | re-run (exclusive range) | pass (green) | SUCCESSFUL @ `unwind(9)`, `length<=8` |
| K2 one-past-end caught | abuse (`tm-…-abuse-1`) | `check_read_byte` (unsafe) | pass (red) | FAILURE: `dereference failure: pointer NULL` (non-vacuous: red first) |
| K2 remediated genuinely | invalid input | re-run (`Option`+postcondition) | pass (green) | SUCCESSFUL; fixed by real safety, not assume-tightening |
| K3 overflow caught | resource bound | `check_accumulate` (`+`) | pass (red) | FAILURE: `attempt to add with overflow` |
| K3 remediated | resource bound | re-run (`saturating_add`) | pass (green) | SUCCESSFUL across symbolic `u32×u32` |
| K4 contract caught | partial failure | `check_gcd_contract` minus `requires` | pass (red) | FAILURE: `remainder with a divisor of zero` / `division by zero` |
| K4 remediated + reused | dependency failure | `requires` added; `check_reduce_fraction` via `stub_verified` | pass (green) | both SUCCESSFUL (`-Z function-contracts [-Z stubbing]`) |
| Bound stated | assertion violation | scope-report review | pass | every green carries its bound; no whole-system claim |
| Kani toolchain absent | dependency failure | prereq cascade (documented) | pass | loud skip path documented; here Kani present + pin-matched |

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none in the SLO repo | the demo's own harnesses are the regression guard | the four *seeded* bugs are the demo's point — each caught + remediated |

## Pass 4 — Security

| Stack | Check | Result | Evidence |
|---|---|---|---|
| Rust (SLO repo) | `cargo audit` | pass | no new deps; demo is external, not a workspace member |
| Rust (SLO repo) | workspace-baseline isolation | pass | `cargo test -p sast-verify` unaffected by the external Kani toolchain |
| — | DAST / PII | N/A | no service; no `docs/biz-public/` |

Threat-model: K-series each prove non-vacuous via the mandatory red-first step (`tm-kani-verification-abuse-1`). The K2 unsafe-pointer red→green directly exercises the unsafe-wrapper abuse path. No new SLO-repo surface introduced (the demo lives in a separate repo).

## Pass 5 — AI tolerance
N/A — the harnesses + verdicts are deterministic `cargo kani` runs.

## Environment
- macOS; `cargo-kani 0.67.0` (matches `tools.toml` pin); demo crate `~/Dev/GitHub/sunlit-kani-demo` @ `c7953f6`.

## Coverage gaps
- None outstanding. The demo is published at https://github.com/kerberosmansour/sunlit-kani-demo @ `c7953f6` (the user ran the `gh` command; commit re-authored with the GitHub noreply email to clear email-privacy protection, giving the final SHA).

## Verdict
The failure bar is met: all four kernels demonstrated catch→remediate→green at stated bounds with real Kani output, naive-first throughout. Demo published. Verified.
