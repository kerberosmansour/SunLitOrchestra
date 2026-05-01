# Lessons Learned — slo-sp Milestone 8

## What changed
- Four power tools authored: `/slo-second-opinion`, `/slo-freeze`, `/slo-resume`, `/slo-ship`.
- 9 E2E contract tests pinning the key disciplines: cross-model disagreement framing, root-freeze refusal, orientation-not-execution for resume, main-branch refusal + no-force-push + no-skip-hooks for ship.

## Design decisions and why
- **`/slo-second-opinion` is a disagreement finder, not an arbitrator.** Rationale: if two models disagree, the user should read both, not have Claude pick a winner. Picking would bake in the wrong equilibrium ("whichever model happens to be more assertive").
- **No silent fallback when the provider CLI is missing.** Explicit install hint + exit, per `preflight` pattern. No "pretend to be Codex."
- **`/slo-freeze` is per-session state, not a file.** Rationale: frozen scope is ephemeral; serializing it adds cleanup burden. `/slo-unfreeze` or a new session clears it.
- **`/slo-resume` is read-only.** It orients; the user decides. Rationale: a resume skill that auto-starts the next milestone undermines the "pause at decision points" discipline of the pack.
- **`/slo-ship` refuses on main/master, refuses on red baseline, refuses on non-done tracker, does not force-push, does not skip hooks.** Four non-negotiables. Rationale: every one of these has been a documented failure mode in real pack usage (mostly from the original Rust-driver era).

## Mistakes made
- None that required rework.

## Root causes
- N/A.

## What was harder than expected
- Writing `/slo-ship` without making it a thin wrapper over `gh pr create`. The skill's value is the PR body composition (from completion summaries) and the refusals. The `gh` call at the end is almost an afterthought.
- `/slo-second-opinion` wanted to become a "pick the better model" tool. Resisting that and keeping it as a diff-surfacer required explicit anti-pattern language.

## Naming conventions established
- Per-persona / per-tool SKILL.md files live at `skills/<skill-name>/SKILL.md`. No nested tool directories.
- PR titles: `<prefix>: <runbook title>`.
- PR bodies: milestone-by-milestone, link to completion summaries, list deferred follow-ups.

## Test patterns that worked well
- Testing for the ABSENCE of bad patterns ("does not force-push", "never automatic", "do not silently fall back") by grepping for the negative-statement language.
- Testing for the PR-body-from-completions discipline by grepping for both "completion" and "milestone" in the same body.

## Missing tests that should exist now
- A scripted `/slo-ship` dry-run against a fake repo on a fake branch with a fake runbook — ensure the skill refuses correctly on each of the four non-negotiable gates. Blocked on harness.
- A scripted `/slo-second-opinion` that mocks `which codex` as absent and confirms the skill exits clean. Blocked on harness.

## Rules for the next milestone (M9 — self-hosting)
- M9 uses the pack end-to-end against a real SLO feature. Do NOT modify any M1-M8 artifact unless the self-hosting surfaces a specific pain. Resist the urge to polish.
- The candidate self-hosting target is "full Apalache integration for `/slo-tla`" (see M5 lessons). If that feels too large, pick something smaller like "auto-populate SHA-256 in `tools.toml` on first maintainer run."
- Every rough edge surfaced during self-hosting gets a lessons-file entry. Some will become follow-up commits on this branch; some will become new runbooks.

## Template improvements suggested
- The "Gates — refuse when" pattern is now in multiple skills. Consider codifying it as a required section alongside Method and Anti-patterns. Every power tool has one; some core skills don't, to their detriment.
