# Completion Summary — agent-host Milestone 5

## Goal completed
- Targeted skill copy and structural tests no longer assume Claude as the default current host where the behaviour is host-neutral. Skills the capability matrix marks as Claude-only retain explicit Claude wording; nothing in scope was softened just to look host-neutral.

## Files changed
- `skills/slo-second-opinion/SKILL.md` — diff-of-findings table column ("Claude said" → "current host said"); silent-fallback rule and anti-patterns rewritten to reference the current host rather than Claude specifically.
- `skills/slo-rulegen/SKILL.md` — description ("Claude-found bug summary" → "agent-found bug summary").
- `docs/slo/design/agent-host-capabilities.md` — capability matrix row labels updated; new "Per-skill notes" section listing `/slo-second-opinion`, `/slo-research`, `/slo-rulegen`, `/slo-sast`, and the live business judgment runtime harness.
- `docs/skill-pack-catalog.md` — Power tools table gained a "Host story" column.
- `docs/ARCHITECTURE.md` — "Current host boundaries" records the per-skill steady state after M5.
- `crates/sldo-install/tests/e2e_slo_sp_m8.rs` — added one new assertion that the `/slo-second-opinion` diff-of-findings table is host-neutral (no "Claude said" column).
- `crates/sldo-install/tests/e2e_agent_host_m5.rs` (NEW) — structural guards covering the host-neutral wording cleanup and the capability-matrix per-skill notes.

## Tests added
- `crates/sldo-install/tests/e2e_agent_host_m5.rs::test_host_neutral_skills_have_no_hidden_claude_requirement`
- `crates/sldo-install/tests/e2e_agent_host_m5.rs::test_explicit_claude_only_skills_are_marked_in_capability_matrix`
- One assertion added inline to `crates/sldo-install/tests/e2e_slo_sp_m8.rs::second_opinion_is_disagreement_finder_not_arbitrator`.

## Runtime validations added
- The two new structural tests above run as part of `cargo test -p sldo-install` and act as the runtime gate for this milestone's contract.

## Compatibility checks performed
- Skill names and outputs unchanged.
- `/slo-second-opinion` still rejects vote/arbitration framing (`second_opinion_is_disagreement_finder_not_arbitrator` continues to pass).
- `/slo-rulegen` tool restrictions (`WebFetch` and `WebSearch` deny, gate composition rules, atomic-write contract) untouched.
- Historical runbooks, lessons, and completion documents remain untouched outside the agent-host runbook itself.
- `cargo test -p sldo-common -p sldo-install -p sldo-research` is fully green.
- `cargo build -p sldo-install -p sldo-research` builds cleanly.

## Documentation updated
- `docs/slo/design/agent-host-capabilities.md` — capability matrix tightened; new per-skill notes section.
- `docs/skill-pack-catalog.md` — "Host story" column added to the Power tools table.
- `docs/ARCHITECTURE.md` — current host boundaries describe the per-skill steady state.
- `skills/slo-second-opinion/SKILL.md` and `skills/slo-rulegen/SKILL.md` — host-neutral wording where the behavior does not require Claude.

## .gitignore changes
- None. M5 introduces no generated files or build outputs.

## Test artifact cleanup verified
- `git status` is clean of untracked test artifacts. The only working-tree changes are the milestone source/doc edits and the M3+M4+M5 closeout files.

## Deferred follow-ups
- A repo-wide drift guard that scans `skills/<name>/SKILL.md` for ungated mentions of "Claude". Out of scope for M5 — adding it would need a separate design pass to define what counts as "ungated" without false-positiving on legitimate Claude-only references.

## Known non-blocking limitations
- `/slo-sast` retains its `claude` mention in the M1 anti-pattern subprocess list. That is intentional and correct: M1 forbids shelling out to any agent CLI, and the concrete name keeps the rule legible. Generalising it to "agent CLI" would have hidden the original intent.
- Spot-checking the touched skills inside an interactive host session is recommended but not required for milestone closeout. The structural tests cover the wording contract; a session-level smoke is a cheap belt-and-braces if a user is in Claude Code or Copilot anyway.
