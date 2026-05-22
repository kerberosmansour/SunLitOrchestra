# Lessons Learned — mloop Milestone 5

## What changed
- `docs/LOOPS-ENGINEERING.md`: new `## Feature-performance loop` entry (standard format + ASCII diagram) between Lessons loop and Library-feedback loop; Start-here row; See-also link to the runbook.
- `docs/LOOPS-BUSINESS.md`: cross-reference (Start-here row + See-also note) to the engineering-side loop + the post-ship `/slo-metrics` cohort touchpoint.
- `xtasks/sast-verify/tests/mloop_m5_loops.rs` (NEW): 4 tests.

## Results vs thesis
- N/A — pure-documentation milestone. The loop it documents is the very thesis of this runbook; M1–M4 built the mechanism, M5 makes it discoverable.

## Design decisions and why
- **One canonical home (engineering doc) + a cross-ref (business doc)** — the loop threads engineering skills; duplicating the full entry in both docs would drift. A `single_home_no_duplication` test enforces it.
- **Section-slice assertion** — the test extracts the `## Feature-performance loop` section and asserts the six standard sub-headings within it, so the format matches the other loops rather than just "the labels exist somewhere".

## Mistakes made
- New test file needed rustfmt (array literal reflow). Same minor lesson as M1 — run `cargo fmt --all` right after writing a test.

## Root causes
- Multi-line array written before fmt.

## What was harder than expected
- Nothing.

## Naming conventions established
- Loop entries follow the locked format: User-visible outcome / Trigger / Steps / Exit condition / Artifacts / Skills involved / diagram.

## Test patterns that worked well
- Extract-section-then-assert-sub-headings; assert single-home by the ABSENCE of the H2 in the companion doc.

## Missing tests that should exist now
- None for M5.

## Rules for the next milestone
- Runbook complete — next is `/slo-ship`. Carry the two open follow-ups (clippy red; mirrored-template byte-identity test) into the PR description as known/deferred.

## Template improvements suggested
- None.

## Carry-forward
- Pre-existing clippy red (mloop-m1); pack-wide mirrored-template byte-identity test (mloop-m3); shared fixture-pair non-vacuity helper (mloop-m4) — all `micro`/`fresh-runbook` lane follow-ups, none blocking.
