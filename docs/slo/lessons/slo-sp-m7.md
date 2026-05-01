# Lessons Learned — slo-sp Milestone 7

## What changed
- `/slo-execute` authored — replaces the inner loop of the legacy `sldo-run` binary.
- `/slo-verify` authored — wraps Playwright for UI, runtime exec for CLI/backend.
- 9 E2E contract tests pinning the disciplines: allow-list enforcement, BDD-first, constraint restatement, regression-test-before-fix, empty-state coverage, Playwright gating, separation of concerns.

## Design decisions and why
- **`/slo-verify` does NOT fix bugs.** Separation of concerns. When `/slo-verify` finds a bug, it writes a regression test and hands back to `/slo-execute` (or a human). Rationale: the verifier and fixer having different incentives is what keeps verifications honest.
- **Regression test BEFORE fix, committed separately.** Rationale: the test must fail today with the bug present. If you commit the fix first, the regression test gets bundled and you can't verify it would have caught the bug.
- **Allow-list enforcement is the #1 discipline.** The skill body uses "STOP coding" in caps and dedicates a section to it, because this is the single most common failure mode of AI-driven execution. Tests grep for the stop/pause/surface-the-conflict language.
- **Playwright gating on UI-surface presence.** Pure backend milestones skip the UI cascade entirely. Rationale: installing Playwright for a CLI milestone is wasted bytes and false confidence.
- **Constraint restatement in chat before coding.** Stated as a required pre-flight step. Rationale: restating forces the implementer to demonstrate understanding; this caught more bugs in my prior use of the template than any test suite.

## Mistakes made
- Accidentally passed an unknown parameter (`command-name-intentionally-left-blank`) to the Write tool on the first attempt at `slo-verify/SKILL.md`. Tool returned a validation error; retried without it. No impact on output.

## Root causes
- Fat-fingered tool call. No structural lesson.

## What was harder than expected
- Writing the allow-list enforcement in a way that distinguishes "this is a small helper and really should just be fixed" from "this is scope creep." Landed on: no exceptions, every out-of-scope edit pauses. If the edit is truly mechanical, the user can widen the allow-list in one line and resume. The friction is deliberate.

## Naming conventions established
- Verification report: `docs/slo/verify/<prefix>-m<N>.md`.
- Bug IDs: one-word slug or `b<N>` numbered within the milestone.
- Regression test commits: one bug per commit, regression test commit is distinct from fix commit.

## Test patterns that worked well
- Testing for STOP-language (stop/pause/refuse) as a proxy for the allow-list enforcement discipline. Simple but effective.
- Testing for separation-of-concerns language ("hand the bug back", "do not fix it yourself") to ensure `/slo-verify` doesn't drift into repairing.

## Missing tests that should exist now
- A scripted scenario where `/slo-execute` is told to edit a file outside the allow-list; the skill should pause. Requires a Claude-Code-non-interactive harness; deferred.
- A Playwright smoke test that drives a simple UI (e.g., the output of `sldo-plan` rendered as HTML). Probably won't build since we don't have a UI of our own; parked.

## Rules for the next milestone (M8 — power tools)
- All four power tools are small (each ~80 lines of SKILL.md). Batch the commits cleanly.
- `/slo-ship` should auto-detect the git state. Do not assume the user's pushed main branch is clean.
- `/slo-resume` reads the tracker and picks the next skill to run. Reuse the Milestone Tracker table as the source of truth; don't introduce a separate state file.
- `/slo-second-opinion` should honestly handle the "user doesn't have Codex/Gemini" case — install hint + exit, not silent fallback.

## Template improvements suggested
- Add a "Separation of concerns" block to skill-authoring template when multiple skills in a chain handle adjacent concerns (verify/execute, plan/critique). Stating what this skill DOESN'T do is as useful as stating what it does.
