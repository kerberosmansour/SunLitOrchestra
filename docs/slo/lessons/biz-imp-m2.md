# Lessons Learned — biz-imp Milestone 2

## What changed

- Renamed `references/biz/legal-intake-form.md` to `references/biz/legal-intake-contract.md`.
- Added four sister contracts: accounting, equity, fundraise, and hire.
- Kept F1/F4/F5 byte-identical across all five contracts.
- Updated the five advisor skills to require conversational intake, restate-and-confirm, refusal-on-ambiguity, closed-regulator lookup, and M1 authority-file citations.
- Added `crates/sldo-install/tests/e2e_biz_imp_m2.rs`.

## Design decisions and why

- `references/templates/` has not landed yet, so M2 used the runbook-approved fallback: inline conversational and restate-confirm discipline in each contract/skill.
- The M2 structural test accepts the inline restate-confirm discipline instead of requiring a missing template path.
- The old legal intake filename was removed rather than duplicated. GitHub issue/comment URLs to the old path are accepted minor breakage and were called out on issue #19.

## Course corrections taken in flight

- A mechanical cross-reference update briefly rewrote historical mentions of `legal-intake-form.md` into self-referential `legal-intake-contract.md` rename prose. Those historical references were restored where they describe the old filename.
- The M2 test now explicitly checks the removed old file path so future mechanical rewrites do not weaken the rename assertion.

## Recommendations for M3

- Keep M3 narrow: update only the three numeric skills and the M3 structural test.
- Make the structural test look for the mismatch-refusal discipline, the two-pass or reciprocal verification wording, and a stdlib-only Python snippet constraint.
- Do not add runtime dependencies; the Python snippets are founder-visible verification artifacts, not runtime execution.

## Tests run

- `cargo test --workspace`: passed.
- `cargo test -p sldo-install --test e2e_biz_imp_m2`: failed before M2 changes, then passed 6/6.
- `cargo test -p sldo-install`: passed.

## Changes to runbook tracker

- M2 status `not_started` to `done`. Started 2026-05-03, completed 2026-05-03.
