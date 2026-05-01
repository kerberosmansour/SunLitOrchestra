# Completion — sast-rulegen-a M1.6

Closes the deferred-from-M1.5 item: the final 4 bootstrap rules. Rule-pack now hits 10/10 against the Runbook A target.

## Rules added

| Rule | CWE | Arms | Confidence | Sink shapes |
|---|---|---|---|---|
| `.semgrep/rust/cwe-787-out-of-bounds-write.{yaml,rs}` | CWE-787 | 4 | MEDIUM | `Vec::set_len`, `ptr::copy_nonoverlapping` (qualified + unqualified), `slice::get_unchecked_mut` |
| `.semgrep/rust/cwe-125-out-of-bounds-read.{yaml,rs}` | CWE-125 | 7 | MEDIUM | `slice::get_unchecked`, `slice::from_raw_parts` (qualified + unqualified), `u{32,64}::from_{le,be}_bytes($BUF[..].try_into().unwrap())` parser-DoS shape |
| `.semgrep/rust/cwe-416-use-after-free.{yaml,rs}` | CWE-416 | 4 | MEDIUM | unsafe-deref-in-fn-with-`drop`, vec-set_len-then-push-with-pre-stored-mut-ptr, FFI-callback-borrowed-local-cast |
| `.semgrep/rust/cwe-672-operation-after-expiration.{yaml,rs}` | CWE-672 | 6 | MEDIUM | libc-call-on-fd-after-`drop`(File), unsafe-deref-after-`drop`(RwLock-guard) (read + write variants), `if expires_at < now { warn }` no-return |

All 4 rules pass `cargo run -p sast-verify -- gate <rule>` (validate + test + check-coverage + check-clean).

## Why structural patterns instead of the originally-recommended taint mode

The M1.5 deferral memo advised taint mode because the variations involve flow ("attacker-controlled length flows into `from_raw_parts`"). In practice every variation reduces to a *syntactic shape with a same-function precondition* that the existing `pattern + pattern-inside fn-body-with-precondition` form expresses cleanly. Pattern-inside acts as a free flow constraint over the function body — no taint sources/sinks needed. Lessons file documents the construction.

For CWE-787 and CWE-125, the unsafe primitive itself (`set_len`, `from_raw_parts`, `get_unchecked*`) is dangerous enough to flag at MEDIUM confidence with no precondition. Single-pattern arms; the rule message guides reviewers.

## Test evidence

```
$ cargo run -p sast-verify -- gate .semgrep/rust/cwe-787-out-of-bounds-write.yaml
gate: PASS — validate + test + check-coverage + check-clean all green

$ cargo run -p sast-verify -- gate .semgrep/rust/cwe-125-out-of-bounds-read.yaml
gate: PASS

$ cargo run -p sast-verify -- gate .semgrep/rust/cwe-416-use-after-free.yaml
gate: PASS

$ cargo run -p sast-verify -- gate .semgrep/rust/cwe-672-operation-after-expiration.yaml
gate: PASS

$ cargo test -p sast-verify
test gate_passes_for_all_authored_rules ... ok (in 70.58s — 10 rules × ~7s each)
all 16 sast-verify tests green; 0 failed; 0 ignored

$ cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify
539 passed; 0 failed; 12 ignored (the 12 ignored are biz-pack runtime-harness tests)
```

## Files added

- `.semgrep/rust/cwe-787-out-of-bounds-write.{yaml,rs}`
- `.semgrep/rust/cwe-125-out-of-bounds-read.{yaml,rs}`
- `.semgrep/rust/cwe-416-use-after-free.{yaml,rs}`
- `.semgrep/rust/cwe-672-operation-after-expiration.{yaml,rs}`
- `docs/slo/lessons/sast-rulegen-a-m1.6.md`
- `docs/slo/completion/sast-rulegen-a-m1.6.md` (this file)

## Files edited

- `docs/slo/completed/RUNBOOK-SAST-RULEGEN-A.md` — M1 tracker row updated from "3/10" → "10/10" with M1.5 + M1.6 attribution.
- `CLAUDE.md` — bumped `# of rules` reference if/where present (none found in this revision; tracker is authoritative).

## Files NOT changed

- `references/sast/variations/cwe-{125,416,672,787}.md` — variation templates remain authoritative; rule arms cite the templates via `metadata.sldo-variation-template`.
- The 6 prior rules under `.semgrep/rust/` — untouched.
- `xtasks/sast-verify/` — no logic changes; the existing `gate_passes_for_all_authored_rules` integration test scales by directory iteration and now exercises 10 rules instead of 6.

## Definition of Done

- [x] All 4 rules + paired fixtures authored.
- [x] All 4 rules pass `cargo run -p sast-verify -- gate`.
- [x] `cargo test -p sast-verify` green (16 passed, integration test exercises all 10 rules).
- [x] Full baseline `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify` green (539 passed; 0 failed).
- [x] Tracker updated to reflect 10/10.
- [x] Lessons file written.
- [x] Completion summary written (this file).

## Deferred follow-ups

- **CWE-672 warn-only-no-return tightening** — current arm fires on `if expires < now { warn }` regardless of whether the next sibling statement is a `return`. A `metavariable-comparison` could narrow this; left as follow-up if reviewer-FP rate warrants.
- **Real-world FP shakedown** — `gate`'s `check-clean` exercises against `xtasks/sast-verify/tests/fixtures/clean_subset/` (small curated corpus). Running these 4 new rules against a larger Rust codebase (e.g., this workspace's own `crates/`) would surface real-world FP rates. Owner-discretion.
- **M2.5 / xtask `validate-file-paths` subcommand** — still deferred from M2; unrelated to M1.6.
