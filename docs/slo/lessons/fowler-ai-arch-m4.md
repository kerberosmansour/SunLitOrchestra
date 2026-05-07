# Lessons Learned — fowler-ai-arch Milestone 4

## What changed

- `/slo-critique` now explicitly describes the engineering architecture-coherence pass.
- The engineering persona now compares milestone contracts against the code-map, four-object summary, reversibility rows, exemplar / anti-exemplar rows, and AI tolerance rows.
- Critique evals now cover exemplar mismatch, missing reversibility, AI tolerance gaps, and rejected vague architecture concerns.
- Legacy hash guards were updated where M4 intentionally changed `eng.md` and `skills/slo-critique/SKILL.md`.
- `crates/sldo-install/tests/e2e_fowler_ai_arch_m4.rs` locks the M4 contract.

## Design decisions and why

- Keep architecture coherence inside the engineering persona. The existing critique rotation stays intact, and design/security personas keep their current responsibilities.
- Require artifact-row drift to name a concrete actor, action, and bad outcome. That keeps architecture critique useful instead of becoming taste-driven review.
- Update legacy pins rather than delete them. They now guard the post-M4 critique baseline and still protect the parts M4 did not authorize.

## Mistakes made

- The initial allow-list did not include legacy tests that pin the files M4 intentionally changes. The runbook was amended to include the two compatibility-test updates before closeout.

## Root causes

- Older milestone tests sometimes encode "this file did not change during that milestone" as a continuing byte-level invariant. When a later milestone intentionally changes the file, the compatibility guard must be repinned or rephrased.

## What was harder than expected

- The critique skill had two independent legacy guards: one in `sldo-install` for persona byte identity and one in `sast-verify` for the critique skill hash. Both needed to stay meaningful after the authorized change.

## Naming conventions established

- Review concept: architecture coherence pass.
- Legacy marker: `eng_persona_architecture_coherence_allowed_after_m4`.
- M4 verification report: `docs/slo/verify/fowler-ai-arch-m4.md`.

## Test patterns that worked well

- Red-first structural tests caught the exact missing terms before any prose edits.
- Keeping a focused legacy-guard test alongside the M4 test made the compatibility repin explicit.

## Missing tests that should exist now

- A future live critique harness should feed a runbook with a deliberate code-map/exemplar mismatch and assert the generated critique row contains actor, action, bad outcome, and a row-level recommendation.

## Rules for the next milestone

- M5 should mirror the sprint-flow discipline in the ticket flow without copying the full v4 runbook template.
- Ticket parity tests should include N/A paths so simple issue-sized tickets stay compact.

## Template improvements suggested

- Consider a shared "legacy hash guard repin" note for future runbooks that intentionally change files protected by older byte-invariant tests.
