# Harness Generation — turning a candidate into a proof

A Kani proof is a parameterless `#[kani::proof]` function, gated behind
`#[cfg(kani)]` so it never compiles into release/test builds. This reference
covers the four patterns and the write-path safety rule.

## Write-path validation (by construction, not by prose)

Harnesses are written ONLY under the **target-crate root** `src/`. Before any
write, validate the path **by construction**: canonicalize the candidate path,
confirm it stays under the validated target-crate root, and **reject** any path
that contains a `..` parent segment, is absolute, or traverses a `symlink`
component. A hostile or typo'd target module path (e.g. `../../.claude/skills/x`)
must never produce an out-of-tree write (SEC-1 / CWE-22,
`tm-kani-verification-abuse-5`). The verified-scope report under
`docs/slo/verify/` is the only other write target.

## Start small, start with the smallest useful target

Encode only the preconditions the API contract actually needs. Prefer proving a
meaningful postcondition/invariant over "does not panic", but use panic/UB
checks as a first pass while exploring. Keep bounds small initially (arrays
8–16, vecs 0–2, `unwind` = iterations + 1).

## Pattern 1 — unsafe wrapper / raw pointer

```rust
#[cfg(kani)]
#[kani::proof]
fn check_read_byte() {
    let xs: [u8; 4] = kani::any();
    let idx: usize = kani::any();
    let result = read_byte(&xs, idx);          // safe wrapper over unsafe
    assert_eq!(result.is_some(), idx < xs.len());
}
```

## Pattern 2 — boundary / arithmetic

```rust
#[cfg(kani)]
#[kani::proof]
fn check_accumulate() {
    let a: u32 = kani::any();
    let b: u32 = kani::any();
    let y = accumulate(a, b);
    assert!(y >= a && y >= b);                  // saturating postcondition
}
```

## Pattern 3 — representation invariant on a constructor/mutator

```rust
#[cfg(kani)]
#[kani::proof]
fn check_mutate_preserves_invariant() {
    let mut obj = any_valid_obj();
    assert!(obj.is_valid());
    obj.mutate(kani::any());
    assert!(obj.is_valid());                    // invariant preserved
}
```

## Pattern 4 — bounded loop with explicit unwind

```rust
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(9)]
fn check_zero_prefix() {
    let length: usize = kani::any();
    kani::assume(length <= 8);
    let mut buffer = [1u8; 8];
    zero_prefix(length, &mut buffer);
}
```

## Symbolic custom types

Prefer a small `#[cfg(kani)]` constructor helper or a guarded `kani::Arbitrary`
impl over symbolic heap growth. Keep collection sizes tiny; escalate only when
runtime stays in seconds.

## Anti-vacuity at authoring time

If a harness needs many `kani::assume(...)` calls, add `kani::cover!(...)`
reachability checks for the intended corner cases so the proof is not vacuous.
The universal anti-vacuity gate is the run/triage rule: the pre-fix variant must
fail first (see `run-and-triage.md`).
