# Fallback Strategies — when a direct proof is too expensive or unsupported

Do not give up when a proof is blocked. Climb this ladder, recording every
abstraction in the scope report.

## 1. Stubs — for unsupported features, environment deps, or bad performance

Use `-Z stubbing` + `#[kani::stub(real, model)]` to replace time, randomness,
syscalls, FFI, or expensive helpers.

**Sound over-approximating stubs only.** A stub must be a behaviourally-safe
**over-approximation** of the real function for the property under verification.
An *under-approximating* (unsound) stub that silences a real failure is
forbidden — that is hiding a bug behind a fake model (`tm-kani-verification-abuse-3`).
Every stub is recorded prominently in the scope report.

## 2. Contracts — for deep call chains / recursion

Use `-Z function-contracts`:

```rust
#[kani::requires(a != 0 && b != 0)]
#[kani::ensures(|r| *r != 0 && a % *r == 0 && b % *r == 0)]
#[kani::recursion]
fn gcd(a: u8, b: u8) -> u8 { /* ... */ }
```

Verify the contract with `#[kani::proof_for_contract(gcd)]`, then let callers
reuse the verified abstraction with `#[kani::stub_verified(gcd)]` instead of
re-executing the recursion. Encode the *real* helper precondition in the
contract — never weaken the caller proof arbitrarily.

## 3. Solver switch — before declaring a target impractical

`cargo kani --solver cadical` (or per-harness `#[kani::solver(...)]`). Solver
choice can change runtime dramatically; try it after bounds and decomposition
are already sensible.

## 4. cover! — to confirm a proof is not vacuous

`kani::cover!(cond)` confirms a branch/state is reachable. `SATISFIED` = the
state is reachable; `UNSATISFIABLE` = the harness can never hit it (likely
over-constrained `assume`).

## 5. Out-of-scope routing — the honest fallback

For true concurrency / async scheduling / data-race freedom: extract a
sequential kernel, verify a per-operation invariant, or mark the target out of
scope for Kani and pair with `/slo-tla` for the protocol layer. Never claim
interleavings verified.
