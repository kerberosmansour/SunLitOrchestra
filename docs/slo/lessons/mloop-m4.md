# Lessons Learned — mloop Milestone 4

## What changed
- `skills/slo-verify/SKILL.md`: new **Pass 6. Measurement (gated)** — six checks (event presence, telemetry PII/masking, failure-path emission, replay tagging, `feature_measurement_spec` flag↔section cross-check, unfenced template-injection); tool-optional `skipped` rule; bug-found-flow reuse; threat-model rows.
- `skills/slo-retro/SKILL.md`: `## Results vs thesis` added to the lessons template.
- `xtasks/sast-verify/tests/mloop_m4_verify_retro.rs` (NEW): 9 tests incl. the non-vacuous failure-bar over a fixture pair.
- `xtasks/sast-verify/tests/fixtures/mloop_failure_bar/{bad,remediated}.md` (NEW, synthetic).

## Results vs thesis
- N/A — meta/tooling milestone (it builds the telemetry-verification + results-vs-thesis mechanism). The loop's own "thesis" — that a bad telemetry contract is caught — is proven by the non-vacuous failure-bar (bad caught on all four mechanized checks; remediated green).

## Design decisions and why
- **Pass 6, not renumbering Pass 1–5** — additive; `slo_verify_passes_not_renumbered` guards it.
- **Mechanized the checkable subset in the test, declared it "mechanically-demonstrated" in prose** — ENG-2 lockstep: the SKILL.md prose and the gated test cannot drift because the prose explicitly points at the failure-bar fixture pair and the test asserts that wording.
- **Reused the Pass 4 email regex** in the mechanized PII check — keeps the documented and demonstrated checks identical.
- **Fence-aware injection check** — `outside_fences()` strips `~~~ … ~~~` blocks, so a properly-fenced author string (remediated) passes while an unfenced one (bad) is caught — exactly the `/slo-architect` `~~~text` discipline.

## Mistakes made
- None material. (rustfmt reflowed the test once; re-ran `cargo fmt`.)

## Root causes
- N/A.

## What was harder than expected
- Designing a *non-vacuous* mechanical demo that maps cleanly to abuse-1/2/3 + missing-event without over-fitting to the fixture. Solved by four deterministic checks (regex email, flag↔section presence, fence-stripped injection token, event-token regex).

## Naming conventions established
- Failure-bar fixtures: `tests/fixtures/<prefix>_failure_bar/{bad,remediated}.md` with a `SYNTHETIC PII` header.
- Mechanized check names are the exact six-check sentinels, shared between prose and test.

## Test patterns that worked well
- One `failure_bar_is_non_vacuous` test asserting bad-fails AND remediated-passes together — the strongest single guard against a vacuous guardrail.

## Missing tests that should exist now
- None for M4. (Replay-tagging + failure-path-emission are heuristic/where-enabled, not mechanized — by design.)

## Rules for the next milestone
- M5 is pure documentation (loop docs); no new runtime surface. Reuse the loop-entry format verbatim; keep existing loop anchors stable.

## Template improvements suggested
- The fixture-pair + non-vacuity-assert pattern is reusable for any future "the guardrail catches X" milestone — worth a shared helper eventually.

## Carry-forward
- Pre-existing clippy red still open (mloop-m1); pack-wide mirrored-template byte-identity test (mloop-m3) still a candidate.
