# TLA+ ↔ Kani Pairing — the refinement map

When a design has **both** `tla_required: true` and `kani_required: true`, the two
formal-methods skills compose by **refinement**: TLA+ proves the *protocol* (the
choreography of actions across actors), treating each action as atomic; Kani
proves that the *Rust kernel* implementing each atomic action is panic-free and
invariant-preserving. This doc is the template + a worked example for connecting
them so the two proofs reinforce instead of drifting apart.

## The boundary invariant

> **Kani never claims what TLA+ owns.**

TLA+ owns timing, ordering, and interleavings (concurrency). Kani owns the
single-action, single-thread footwork (no panic / no UB / invariant preserved at
stated bounds). Kani **cannot** reason about interleavings, `await`, atomics, or
data races — those are out of scope for Kani and are TLA+'s job. Neither skill
claims the other's guarantee. Without this rule the two proofs "drift": a
beautiful TLA+ proof built on an atomic action whose real Rust code doesn't
actually behave atomically/correctly.

## The refinement map

One row per TLA+ atomic action. The map makes explicit that the action TLA+
trusted is implemented by a specific Rust fn, which a specific Kani harness has
proved behaves as assumed.

| TLA+ action (abstract, atomic) | Rust fn (the real footwork) | Kani harness (proof of the footwork) | Bound |
|---|---|---|---|
| `[Action(args)]` | `[fn(...)]` | `[check_fn]` | `[unwind / sizes / contract]` |

Reading a row: *"the atomic `Action` TLA+ assumed is implemented by `fn`, and
`check_fn` has proved `fn` is panic-free / invariant-preserving within the
stated bound."* Together: the dance is safe **and** each step's footwork is real.

## Worked example (using the M4 demo kernels)

Imagine a TLA+ spec for a "reduce a fraction to lowest terms" service where
each request is an atomic `Reduce(a, b)` action, and the spec assumes that
action always terminates and never divides by zero.

| TLA+ action | Rust fn | Kani harness | What it discharges |
|---|---|---|---|
| `Reduce(a, b)` (atomic, assumed total) | `reduce_fraction(a, b)` → `gcd(a, b)` | `check_gcd_contract` (contract) + `check_reduce_fraction` (`stub_verified`) | the assumed-atomic action never panics / divides by zero, given `a != 0 && b != 0` — see [kani-verification-kani.md](../verify/kani-verification-kani.md) |
| `Bound(len)` (atomic buffer write) | `zero_prefix(len, &mut [u8; 8])` | `check_zero_prefix` (`unwind(9)`) | the atomic write never goes out of bounds for `len <= 8` |

TLA+ would prove the *protocol* (e.g. no two `Reduce` actions corrupt shared
state, requests are eventually served). Kani proves the *kernels* those actions
reduce to are sound. The `gcd` row is the canonical case: TLA+ assumes `Reduce`
is total; `check_gcd_contract` proves the `requires(a!=0 && b!=0)` precondition
eliminates the div-by-zero class — so the abstract assumption is backed by code.

## When only one fires

- `kani_required` only: no protocol-level concurrency risk; verify kernels with
  Kani, skip TLA+.
- `tla_required` only: the design is concurrency-heavy but the per-action Rust
  kernels are trivial; model-check the protocol, skip Kani.
- Both: build this map so each atomic action has a named Kani harness. An action
  with no corresponding harness is a gap; a harness claiming concurrency is a
  boundary-invariant violation.
