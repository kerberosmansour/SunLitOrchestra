# Candidate-Scoring Rubric — what to verify with Kani

Kani is a **targeted engine for small, bounded, deterministic kernels**, not a
whole-program verifier. The skill is selective *by design*: scan the target,
score candidate functions/modules, and prove the high-value ones. This rubric
converts Kani's documented strengths and limits into a scoring procedure.

## Procedure

1. Build a ranked list of candidate functions or modules.
2. Apply the raise-score / lower-score signals below.
3. Prefer small, high-value kernels over broad, brittle whole-module proofs.
4. For each selected candidate, hand off to `references/harness-generation.md`.

## Raise the score for

| Signal | Why it is a strong Kani target |
|---|---|
| `unsafe` blocks; safe APIs wrapping unsafe internals | UB-checking is a primary Kani use case (the sweet spot) |
| raw pointers, `NonNull`, `add`/`offset`/`as_ptr`, `transmute`, manual indexing | dangling / misaligned / one-past-the-end access is exactly what Kani finds |
| arithmetic with shifts, casts, `wrapping_*`, `checked_*`, overflow-sensitive logic | overflow / underflow / undefined-shift / div-by-zero caught quickly across symbolic ranges |
| constructors / mutators with representation invariants (`new`, `insert`, `from_*`) | "object stays well-formed after the call" is easy to assume + assert |
| state-machine steps, parser branches, transition tables, command handlers | "invalid state unreachable" / "transition preserves invariant" with `cover!` for rare states |
| public APIs, security boundaries, bounds checks, recent bug-fix hotspots | trust boundary between safe Rust and hidden machinery; bug-fix regressions |
| recursive helpers / deep call chains | candidates for `#[kani::requires]`/`#[kani::ensures]` contracts + `stub_verified` |

## Lower the score for (or route elsewhere)

| Signal | Why, and what to do instead |
|---|---|
| `std::thread`, `.await`, `tokio`, channels, `Arc<Mutex<_>>`, atomics, lock-free | **Concurrency is out of scope for Kani.** Extract a sequential kernel, or verify a per-operation invariant, or mark out-of-scope and pair with `/slo-tla`. NEVER claim interleavings verified. |
| large symbolic `Vec`/`String`, parsing over long input, nondeterministic heaps | scales poorly — even size-2 nondeterministic collections can take minutes. Start tiny (arrays 8–16, vecs 0–2); refactor or use contracts/stubs if runtime climbs. |
| heavy I/O, syscalls, real clocks, randomness, opaque FFI | model the dependency with a **sound over-approximating** stub first; only then prove. |
| unbounded loops/recursion with no practical small bound | bound it (`#[kani::unwind(N)]`) or refactor; otherwise not a Kani target. |

## Scoring output

Emit a ranked candidate list, each row: `target` · `signals matched` · `suggested
strategy (panic/UB first | invariant | contract | stub)` · `starting bound` ·
`out-of-scope notes`. The selection rationale is recorded so `/slo-verify` and
`/slo-retro` can audit *why* a kernel was (or was not) verified.

> A green Kani run is **proved within the stated harness, assumptions, and
> bounds** — never "whole system proved." Scoring decides scope; it does not
> expand the guarantee.
