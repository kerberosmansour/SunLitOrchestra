# Lessons — sast-rulegen-a M1.6

Closes the deferred-from-M1.5 item: 4 remaining bootstrap rules (CWE-787, CWE-125, CWE-416, CWE-672) bringing the rule pack from 6/10 to 10/10.

## What worked

- **Avoided taint mode entirely.** The M1.5 deferral memo recommended Semgrep taint mode for these classes, citing flow-analysis needs (e.g., "attacker-controlled length flowing into `from_raw_parts`"). In practice every variation reduces to a *syntactic shape with a precondition* that fits `pattern: <sink>` paired with `pattern-inside: |\n fn $F(...) { ... <precondition> ... }`. The pattern-inside acts as a free flow constraint over the function body — Semgrep's existing structural matcher already handles "precondition statement appears anywhere before sink in the same function" without needing taint sources/sinks.
- **The `pattern + pattern-inside fn-body-with-precondition` pattern.** Worked uniformly across all 4 rules:
  - CWE-416: sink `unsafe { ... *$P ... }` + precondition `drop($V)` in same fn body.
  - CWE-672: sink `libc_call($FD, ...)` + precondition `let $FD = $H.as_raw_fd(); ... drop($H);` in same fn body.
  - CWE-787 + CWE-125: degenerate case — the sink primitive (`set_len`, `get_unchecked`, `from_raw_parts`, attacker-length-byte-parse) is itself bug-shaped enough to flag at MEDIUM confidence without further preconditions.
- **Single-pattern arms when the primitive is the bug.** For CWE-787 and CWE-125, the unsafe primitive is dangerous enough on its own to flag without context (any `vec.set_len()` or `from_le_bytes(...).try_into().unwrap()`-into-slice is suspicious). MEDIUM confidence accepts the FP rate; the message guides reviewers on what to verify. Trying to add context preconditions would have lost real bugs.
- **Sidestepped the multi-statement `pattern: |` constraint** that blocked the original M1.5 attempt at CWE-672. The constraint is real (Semgrep CE rejects `pattern: | stmt1; ...; stmt2`), but only matters when both pre and sink need to live in the SAME pattern. By splitting into `pattern:` (sink) + `pattern-inside:` (precondition in fn body), each operand is single-construct and the parser accepts it.

## What I'd do differently

- **The `metavariable-comparison` directive could have made CWE-672's "warn-only-no-return" arm tighter.** The current arm flags `if expires < now { warn; }` regardless of whether the next sibling statement is a `return`. A `metavariable-comparison` checking the absence of a `return` after the warn would lower the FP rate. Skipped for v1 to keep arm shapes simple; left as a follow-up if reviewer-FP signal warrants.
- **CWE-672's `extend_rwlock_borrow` arm uses `pattern-inside: fn $F(...) { ... drop($G); ... }` which means the rule fires even if the unsafe deref is *before* the drop in the function body.** Semgrep's `pattern-inside` matches anywhere in the inside-pattern. For these particular bug shapes the early-deref case is also a bug (the deref races with the drop), so the over-fire is acceptable. Documented; future rule authors should know `pattern-inside` is order-insensitive.

## Pitfalls / things to remember

- **`pattern-inside` is order-insensitive.** It asserts the precondition appears *somewhere* in the enclosing scope, not specifically *before* the sink. For CWE-416 / CWE-672 this is fine because either ordering is bug-shaped, but a future rule author who needs strict before/after ordering will need a different approach (e.g., `pattern-regex` over a normalized dump, or runtime miri).
- **`unsafe { ... *$P ... }` matches both reads (`let _ = *p;`) and writes (`*p = 9;`).** Convenient — single arm covers both directions of pointer misuse.
- **Two arms per CWE-672 / CWE-416 sub-shape** (one for `read`, one for `write`; or one for `fn`, one for `unsafe fn`) — Semgrep doesn't unify across signature shapes, so explicitly enumerating both is required. The `pattern-either` arm count grows but each arm stays readable.

## Rule-pack state at end of M1.6

```
.semgrep/rust/
├── cwe-125-out-of-bounds-read.{yaml,rs}            (NEW M1.6)
├── cwe-190-integer-overflow-in-security-context.{yaml,rs}  (M1)
├── cwe-20-improper-input-validation.{yaml,rs}      (M1.5)
├── cwe-295-improper-cert-validation.{yaml,rs}      (M1)
├── cwe-416-use-after-free.{yaml,rs}                (NEW M1.6)
├── cwe-672-operation-after-expiration.{yaml,rs}    (NEW M1.6)
├── cwe-697-incorrect-comparison.{yaml,rs}          (M1.5)
├── cwe-755-panic-on-result-fn.{yaml,rs}            (M1)
├── cwe-787-out-of-bounds-write.{yaml,rs}           (NEW M1.6)
└── cwe-79-xss-in-rust-webapp.{yaml,rs}             (M1.5)
```

10/10 against the runbook target. `gate_passes_for_all_authored_rules` integration test in `xtasks/sast-verify/tests/gate_e2e.rs` exercises all 10 in ~70s.
