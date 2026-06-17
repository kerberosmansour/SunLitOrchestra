# Agent Operating Contract

This is the shared always-on contract for AI coding agents working in SunLit Orchestra. Keep it small. Detailed workflows live in `skills/<name>/SKILL.md`; host-specific install and runtime limits live in `CLAUDE.md`, `AGENTS.md`, `copilot-instructions.md`, and `docs/slo/design/agent-host-capabilities.md`.

## Ask When Ambiguous

Ask a concise question when the next edit depends on information that cannot be discovered from the repo or current sources. If the risk is low, make a conservative assumption and say what you assumed.

## Smallest Safe Change

Prefer the smallest change that satisfies the active SLO contract. Do not broaden scope, rename public interfaces, add dependencies, or refactor adjacent code unless the runbook, ticket, or user explicitly allows it.

## Respect The Allow-List

When an SLO runbook or ticket names files allowed to change, treat that list as binding. If the fix needs another file, stop and surface the contract gap before editing it.

## Verify Before Claiming

Evidence beats confidence. Run the relevant formatter, build, lint, structural tests, runtime checks, or manual smoke steps before saying work is done. If a check cannot run, record why.

## Outcome First Engineering

Code completion alone is insufficient. For a value-bearing milestone, "done" means the **promised user outcome exists AND existing important outcomes still exist** — proven, not asserted. Treat the milestone's Outcome Scenarios and Critical User Journeys (the runbook §5C Outcome Validation Contract) as the **primary Definition of Done**, exercised front-to-end at runtime; a failing outcome, journey, or required regression-matrix row blocks completion no matter how many unit tests pass. Internal refactor / docs-only / test-only work is exempt.

## Keep Host Boundaries Honest

Do not invent parity between Claude Code, Codex, and GitHub Copilot. Before claiming a host supports skills, instructions, agents, plugins, or runtime automation, check `docs/slo/design/agent-host-capabilities.md`.

## Keep Instructions Layered

Always-on files should orient agents, not carry whole workflows. Link to the canonical catalog at `docs/skill-pack-catalog.md`, then use the relevant installed SLO skill for detailed procedure.

## Preserve User Work

Treat uncommitted changes as user work unless proven otherwise. Never reset, discard, or overwrite unrelated changes to make a task easier.
