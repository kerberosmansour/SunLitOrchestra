# Lessons Learned - sec-libs Milestone 2

## What changed

- Added `skills/slo-sec-libs/references/methodology-m2-matcher.md`, the read-only matcher contract for proactive-control rows against M1 catalogs.
- Extended `skills/slo-sec-libs/SKILL.md` with `--match <runbook.md> --catalog <catalog.json>` mode.
- Added `crates/sldo-install/tests/e2e_sec_libs_m2.rs` with 11 structural-contract tests.
- Updated `docs/slo/future/RUNBOOK-SLO-SEC-LIBS.md` with M2 tracker, smoke, and evidence rows.

## Design decisions and why

- **No new matcher executable in M2.** The runbook allowed a methodology file and structural tests, not a new runtime dependency. Matching remains host-driven and read-only, with a strict output contract.
- **Catalog ID is the trust anchor.** Every matched candidate and selected recommendation must cite a `components[].bom_ref` from the M1 catalog. If the ID is absent, the match is refused and downgraded to `unmatched`.
- **Conservative-by-default tiebreaker.** Critique C-6 asked for a decision on comparable parametric candidates. M2 now says to prefer the stricter value only when both candidates are valid and comparable; if comparability is unclear, surface a tie.
- **Generic controls are low confidence.** A `C5` label can make a component a candidate, but it cannot outrank concrete parametric evidence.

## Test patterns that worked well

- The M2 tests pin the methodology's key phrases so future edits cannot silently drop tiebreaker or tie behavior.
- A tiny structural guard function catches fabricated candidate IDs without needing a new JSON dependency.
- M1 compatibility is asserted by checking the reader script, M1 methodology, and existing deny-list remain present.

## Missing tests that should exist later

- A future runtime matcher script, if added, should get executable fixture tests for all BDD scenarios.
- The host-driven row extraction should be dogfooded against at least three real runbooks in M5, because Markdown table shape varies.
- Conservative parameter comparison should become executable once catalog properties settle around named parameter fields.

## Rules for M3

- Treat each `unmatched` record as one gap. Do not merge multiple controls into one issue.
- Do not allow free-text target prose to flow directly into issue bodies. M3 must validate the capability-gap schema before filing.
- Preserve the M2 output shape so M3 can consume `row_id`, `control_id`, `control_text`, and `reason`.
- Keep filing out of M2; all GitHub side effects start in M3.
