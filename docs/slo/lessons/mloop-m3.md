# Lessons Learned — mloop Milestone 3

## What changed
- v4 template (BOTH copies): new `## 5A. Measurement Contract` section (10 fields: value hypothesis, review windows, primary leading/lagging metric, guardrails, telemetry deliverables, rollout, diagnosis, experiment, privacy controls) + a `Measurement deliverables` row in the §17 Contract Block. Optional-by-shape (legacy runbooks valid).
- `skills/slo-plan/SKILL.md`: Measurement Contract requirement + deterministic "value-bearing" definition + Contract Block sentinel.
- `xtasks/sast-verify/tests/mloop_m3_plan.rs` (NEW): 6 tests incl. byte-identity + kani-subblock-survival.

## Results vs. success thesis
- N/A — meta/tooling milestone (builds the planning-time measurement contract).

## Design decisions and why
- **§5A, not renumbering §6–§20** — inserting a lettered section preserves every downstream heading reference (and `kani_m3_integration.rs` pins §5's Kani sub-block). Asserted both.
- **Edited the repo mirror, then `cp` → skill-primary** — guarantees byte-identity rather than hand-replicating (which risks drift). A dedicated `template_copies_stay_byte_identical` test now guards the sync going forward.
- **Deterministic value-bearing definition** (ENG-1) — "introduces/changes user-facing capability; excludes refactor/docs-only/test-only" makes the gate reproducible across agents.

## Mistakes made
- First §5A draft used "Existing runbooks remain valid"; the test wanted the contiguous phrase "legacy runbooks". Aligned the wording (intent unchanged). Lesson: when a sentinel is an exact phrase, write the doc to that phrase or relax the sentinel — decide deliberately, don't leave them mismatched.

## Root causes
- Sentinel phrase chosen before the prose was finalized.

## What was harder than expected
- The dangerous seam: the v4 template has a skill-primary + repo-mirror pair. Caught it in pre-flight (the allow-list named only the mirror) and surfaced an allow-list extension rather than silently editing one copy.

## Invariants/assertions added or strengthened
- Measurement Contract section (10 fields) in BOTH copies; byte-identity; `Measurement deliverables` row; `/slo-plan` requirement + value-bearing definition + flag-don't-invalidate; §6/§10/§17 not renumbered; §5 Kani sub-block survives.

## Resource bounds established or verified
- One new section + one Contract Block row; review windows bounded (24h/7d/28d); no unbounded metric list.

## Debugging / inspection notes
- Pre-fix run confirmed the 3 new-content assertions failed; byte-identity passed pre-edit (both unchanged) and post-edit (cp).

## Naming conventions established
- Lettered template sections (`5A`) for additive insertions that must not renumber existing sections.

## Test patterns that worked well
- `cp` source→mirror + a byte-identity assertion is a robust way to keep mirrored docs in sync.

## Missing tests that should exist now
- Consider a generic "all mirrored templates byte-identical" test pack-wide (follow-up, not this runbook).

## Rules for the next milestone
- M4 edits `/slo-verify` + `/slo-retro` + adds fixtures; the failure-bar fixture pair must be NON-vacuous (bad fails, remediated passes, asserted together) and fixtures must use SYNTHETIC PII only.

## Template improvements suggested
- The §5A optional-section pattern could be promoted to a documented "additive lettered section" convention.

## Carry-forward
- Pre-existing clippy red still open (see mloop-m1).
- New follow-up candidate (micro): pack-wide mirrored-template byte-identity test.
