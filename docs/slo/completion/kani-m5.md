# Completion Summary — kani Milestone 5

## Goal completed
- The TLA+↔Kani pairing is explicit (refinement map: action → fn → harness, with the "Kani never claims what TLA+ owns" boundary invariant), and deep verification is a documented **local** developer workflow (quick + deep tiers, pinned toolchain, deep-before-release). No CI automation in v1 — and that deferral is mechanically enforced by a structural test. This is the final milestone: the runbook is complete.

## Files changed
- `docs/slo/design/kani-verification-kani-pairing.md` (NEW — refinement map + boundary invariant + worked example)
- `skills/slo-kani/references/local-deep-verification.md` (NEW — quick/deep tiers, pinned toolchain, deep-before-release, why-local rationale)
- `skills/slo-tla/SKILL.md` (reciprocal `/slo-kani` handoff note)
- `skills/slo-kani/SKILL.md` (local-verify dispatch row)
- `docs/LOOPS-ENGINEERING.md` (`/slo-kani` added to the sprint loop, step 4b + skills list)
- `xtasks/sast-verify/tests/kani_m5_pairing.rs` (NEW — 5 assertions)
- `docs/ARCHITECTURE.md` (planned note folded into the HEAD skill-pack table — skill now ships)

## Tests added
- `kani_m5_pairing.rs`: refinement-map + boundary-invariant; quick/deep tiers; pinned-toolchain in local-verify; no-Kani-CI guard; slo-tla reciprocal note.

## Runtime validations added
- All 5 exercised; the no-CI guard confirms the v1 decision. Report: `docs/slo/verify/kani-m5.md`.

## Compatibility checks performed
- Full `cargo test -p sast-verify` green incl. `slo_tm_m2_consumers` (the `/slo-tla` reciprocal-note edit is additive and broke nothing); no `.github/workflows/*` changed.

## Documentation updated
- LOOPS-ENGINEERING sprint loop; ARCHITECTURE HEAD table (folded planned note); pairing doc + local-verify ref (new).

## .gitignore changes
- None.

## Test artifact cleanup verified
- `git status` clean apart from intended files.

## Deferred follow-ups
- CI automation for Kani (a quick-PR job + scheduled deep job) is a documented future enhancement, intentionally deferred per the v1 decision — not a gap.

## Known non-blocking limitations
- The pairing's dual-tool behavior would be exercised end-to-end only on a target with both `tla_required` and `kani_required`; the template + boundary invariant + the M4-grounded worked example are delivered and tested.
