# Lessons Learned — svl Milestone 2

## What changed
- Added a "Secure Value & Security Contract requirement" block to `skills/slo-plan/SKILL.md`, co-located after the Measurement Contract requirement: `/slo-plan` now requires §5B for value-bearing OR security-relevant milestones, defines "security-relevant" deterministically (identity, secrets, PII, payment, cloud, AI agents, public/network, CI/CD, infrastructure), names the five §5B sub-blocks to populate, requires OWASP-2024-by-name control citations, carries the inert-window note (F-ENG-3), and stays forward-looking (flag the gap; never invalidate legacy runbooks).
- Added `xtasks/sast-verify/tests/svl_m2.rs` (4 assertions).

## Results vs thesis
- N/A — not a value-bearing milestone (tooling/process).

## Design decisions and why
- **Co-locate with the Measurement Contract requirement** — same forward-looking posture, so the two read as siblings and the wording shape is reused (lowering drift risk and keeping `mloop_m3_plan` green).
- **Trigger threshold ≥6/8 in the test** — tolerant to phrasing ("PII" vs "personal data") while still proving the list is enumerated, not hand-waved.

## Mistakes made
- None. (Same per-path Read-state note as M1 applies — already internalised.)

## Root causes
- N/A.

## What was harder than expected
- Nothing; the Measurement-Contract requirement was a near-exact template.

## Naming conventions established
- Skill requirement blocks live as `## <Name> requirement` H2 sections in `SKILL.md`, co-located by theme.

## Test patterns that worked well
- A `count() >= N` threshold for an enumerated list (trigger words) — robust to synonyms while still non-vacuous.

## Missing tests that should exist now
- None for M2.

## Rules for the next milestone
- **M3**: extend BOTH the v4-template status comment (additive, byte-identical) AND `crates/sldo-common/src/runbook.rs` (`MilestoneStatus` total over the documented set; unknown→`Blocked`; fix `all_done` silent-completion) with round-trip unit tests; bump crates.io 0.1.2→0.1.3; add the Operator Readiness Gate to `/slo-execute` Global Entry (fail closed); make `/slo-resume` recognise new states; create the `operator-action-required` label. `slo-execute`/`slo-resume` are not SHA-pinned (verified in M2), but re-check before editing.

## Template improvements suggested
- None.

## filed_issues
- none — M2 surfaced no detected-work items.
