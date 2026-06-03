# Lessons Learned — svl Milestone 4

## What changed
- `/slo-execute` gained "The Detected Work Ledger — never leave a finding 'observed'" section: open the §5B ledger at start, every finding gets one of five dispositions, refuse `done` while any row is undisposed, dispositions route to existing `/slo-retro` lanes (no new lane verb).
- `/slo-retro` gained Step 0 (re-read the Detected Work Ledger and reconcile dispositions onto the existing lane vocabulary) before the existing classify/file flow — explicitly no fourth taxonomy.
- `/slo-verify` Pass 4 gained the Bundle A–F evidence-row table (selection inputs to existing surface detection, never blank vocabulary), with read-side contract phrases preserved.
- Added `xtasks/sast-verify/tests/svl_m4.rs` (5 assertions, incl. the F-SEC-1 "no new lane verb" invariant and a read-side-phrase regression guard).

## Results vs thesis
- N/A — not a value-bearing milestone (tooling/process).

## Design decisions and why
- **Reconcile, don't fork (F-SEC-1)**: the enforceable invariant is "no new `/slo-retro` lane verb", because a structural test can verify lane-verb stability but cannot verify the soft claim "no new taxonomy". The five dispositions are a thin routing vocabulary; the test pins that the three lane verbs remain and the reconciliation is stated.
- **Ledger discipline in `/slo-execute`, reconciliation in `/slo-retro`**: execution captures findings; retro disposes/files them — matches the existing separation (execute writes, retro files with dedupe+cap). Reused the filing discipline verbatim.
- **Bundles as selection inputs, not a runner**: avoided a parallel test engine; the Bundle table maps to existing Pass 4/5 checks.
- **`/slo-verify` edit kept additive**: confirmed `slo_tm_m2_consumers.rs` content-checks the read-side phrases (it SHA-pins `/slo-critique`, NOT `/slo-verify`), so no SHA baseline needed — only phrase preservation, guarded by a new regression test.

## Mistakes made
- M3's lessons mis-noted that `slo_tm_m2_consumers.rs` SHA-pins `/slo-verify` and would need a baseline update. On reading the test, it pins `/slo-critique` (untouched) and only **content**-checks `/slo-verify`. Corrected here; no baseline change was needed. Verifying the actual pin before editing (not trusting the prior note) caught it.

## Root causes
- The M3 note conflated "test references slo-verify" with "test SHA-pins slo-verify". Reading the pin mechanism (`feng6_sha_constant_in_lockstep` targets `CRITIQUE_SKILL_SHA256`) disambiguated.

## What was harder than expected
- Asserting "no new taxonomy" structurally — impossible directly. Resolved by pinning the enforceable proxy (lane-verb stability + explicit reconciliation statement).

## Naming conventions established
- Detected Work Ledger row IDs `DW-NNN`; disposition column uses the five-value vocabulary verbatim.

## Test patterns that worked well
- A regression test that re-asserts another test's content invariant (`verify_read_side_contract_phrases_survive`) when editing a shared file — cheaper than relying on the other test alone and self-documents the coupling.

## Missing tests that should exist now
- None for M4.

## Rules for the next milestone
- **M5**: LOOPS-ENGINEERING (per-stage security output) + LOOPS-BUSINESS cross-ref + `/slo-ship` secure-release checklist (`ship_state`, SBOM/provenance when-applicable) + **dogfood** the §5B contract on a small real surface. None of LOOPS docs / `/slo-ship` are SHA-pinned (verify before editing). SBOM/provenance must stay a `when applicable` row, never a hard gate for markdown.
- **Confirm DW-001 filing** (pre-existing `cargo deny` licenses failure) with the user at ship time.

## Template improvements suggested
- None.

## filed_issues
- none auto-filed (confirmation-gated). DW-001 (cargo deny licenses) recorded in the M4 verify report ledger, disposition `file_github_issue`, pending user confirmation at M5/ship.
