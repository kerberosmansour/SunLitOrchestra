# Lessons Learned — agent-host Milestone 5

## What changed
- `/slo-second-opinion/SKILL.md` rewrote the diff-of-findings table column from "Claude said" to "current host said" so the host-neutral half of the comparison is named honestly. The silent-fallback rule and the anti-pattern list also dropped Claude-as-default-current-host wording.
- `/slo-rulegen/SKILL.md` description changed "Claude-found bug summary" to "agent-found bug summary" — the bug summary's origin does not need to be Claude.
- `docs/design/agent-host-capabilities.md` added a per-skill notes section covering `/slo-second-opinion`, `/slo-research`, `/slo-rulegen`, `/slo-sast`, and the live business judgment runtime. Capability matrix renamed the batch-backend row to "Optional Claude batch backend" and noted the helper module path explicitly for the live runtime row.
- `docs/skill-pack-catalog.md` "Power tools" table gained a "Host story" column so a reader sees the host-neutral / Claude-only split without leaving the catalog.
- `docs/ARCHITECTURE.md` "Current host boundaries" recorded the per-skill steady state.
- `crates/sldo-install/tests/e2e_agent_host_m5.rs` (new) and one new assertion in `e2e_slo_sp_m8.rs` guard against reintroducing Claude-coupled wording into host-neutral surfaces.

## Design decisions and why
- Did not run a repo-wide "replace Claude everywhere" sweep. The runbook explicitly forbids it; the capability matrix is the source of truth for what is genuinely Claude-only and the cleanup only touched skills the matrix says are host-neutral.
- Left `/slo-sast` alone. Its sole `claude` mention is in an anti-pattern subprocess list (`Running `claude` / `git` / `gh` / `semgrep` subprocesses`) which is honest about what M1 must not shell out to. Generalising it to "agent CLI" would have hidden the original concrete intent.
- Used "agent-found bug summary" rather than "host-found" or "current-host-found" in `/slo-rulegen`. The phrase is more natural in user copy and the capability matrix already pins down what "agent" means in context.
- Added a "Host story" column to the catalog rather than a separate footnote. A reader scanning power tools sees the boundary in the same row as the skill name, which prevents future drift.

## Mistakes made
- Initially considered loosening `e2e_slo_sp_m8.rs::second_opinion_handles_missing_providers` because the `assert!` looked for "asking Claude" wording. On re-read, the assertion already accepted "defeats the purpose" as a fallback, so the existing test continued to pass after the wording edit. No loosening needed.

## Root causes
- The `/slo-second-opinion` table column was an artifact of the early Claude-first repo. Now that the installer supports both Claude Code and GitHub Copilot, the column was the most visible piece of "Claude is the current host" baked into a host-neutral skill.
- `/slo-rulegen`'s "Claude-found" wording was descriptive when the only real bug-finder was Claude; once the install pack went multi-host, the description became less accurate than necessary.

## What was harder than expected
- Resisting scope creep into skills the runbook did not list. Several skills mention "Claude" in passing (CLAUDE.md cross-links, capability matrix references) — the temptation to "fix them all" had to be checked against the milestone's allow-list and the explicit "do not do a repo-wide replace" red line.

## Naming conventions established
- "Host story" is the canonical label in the catalog for whether a skill is host-neutral, Claude-only, or has a host-specific automation tail.
- "Per-skill notes" is the canonical name for the section of the capability matrix that records non-obvious host coupling on a per-skill basis.
- "Current host said" is the canonical wording for the half of `/slo-second-opinion` that captures whatever agent runs the skill.

## Test patterns that worked well
- Two structural assertions per BDD scenario: a positive assertion ("must contain X") and a negative assertion ("must not contain Y"). Together they catch both "we forgot to add the host-neutral wording" and "we added the wording but left the old wording in place".
- Testing the capability matrix string for a per-skill mention is a cheap drift guard. Adding the assertion took one line; the matrix is now load-bearing for any future "is this skill host-neutral?" question.

## Missing tests that should exist now
- A drift guard that scans every `skills/<name>/SKILL.md` for ungated mentions of "Claude". Today the cleanup is enforced only on the listed skills; a future skill could reintroduce the wording without tripping any assertion. Out of scope for M5 — would need a new design pass to define what counts as "ungated".

## Rules for the next milestone
- N/A — this is the final milestone of the agent-host runbook. The next runbook should bias toward keeping the capability matrix as the source of truth and adding new per-skill notes whenever a skill's host story is non-obvious.

## Template improvements suggested
- The "Files Allowed To Change" table for cleanup-style milestones could include an "Edit budget per file" hint (e.g., "header rename + 1 anti-pattern line") so a later reader can see how narrow the change was meant to be without re-reading the whole milestone.
