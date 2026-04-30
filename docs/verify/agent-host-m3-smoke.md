# Agent-Host Milestone 3 Smoke Checklist

> Runbook: [docs/RUNBOOK-AGENT-HOST-COMPATIBILITY.md](../RUNBOOK-AGENT-HOST-COMPATIBILITY.md) Milestone 3
> Goal: verify that `/slo-research` is host-native for interactive use and that the Claude batch backend is explicit and optional.
> Local validation was completed from the terminal on 2026-04-29. The Claude Code session was completed in-host on 2026-04-30 by reading the installed `~/.claude/skills/slo-research/SKILL.md` and confirming each contract line. The GitHub Copilot session was completed in-host on 2026-04-30 against the repo-local install at `./.copilot/skills/slo-research/SKILL.md` after `cargo run -p sldo-install -- --host github-copilot --local install` and `verify`. The global Copilot root was not mutated because `~/.copilot/skills/get-api-docs` is a pre-existing non-symlink conflict.

## Claude session

- [x] Open the installed `/slo-research` skill in Claude Code. — verified against `~/.claude/skills/slo-research/SKILL.md` on 2026-04-30
- [x] Confirm the interactive path tells the agent to use host-native research tools and file writes, not only `sldo-research`. — `SKILL.md` body lines 15–20: "Your default path is host-native research tools" and "`sldo-research`… is a separate path and never the only way to run this skill"
- [x] Confirm missing `docs/idea/<slug>.md` still produces a clear refusal that points to `/slo-ideate`. — `SKILL.md` Inputs and Pre-flight steps both refuse and route to `/slo-ideate`
- [x] Confirm the skill still writes to `docs/research/<slug>/dossier.md`, `sources.md`, and `synthesis.md`. — `SKILL.md` Outputs and Method step 3 list those exact filenames
- [x] Confirm any batch guidance is labeled as an optional Claude batch backend, not the default path. — `SKILL.md` "Optional path — optional Claude batch backend" section gates the batch path on explicit user request

## GitHub Copilot session

- [x] Open the installed `/slo-research` skill in GitHub Copilot. — verified on 2026-04-30 against the repo-local install at `./.copilot/skills/slo-research/SKILL.md`
- [x] Confirm the interactive path is usable without installing Claude or `sldo-research`. — `SKILL.md` body lines 15–20 describe host-native research as the default path and the anti-patterns section explicitly forbids telling GitHub Copilot users to install Claude just to use `/slo-research` interactively
- [x] Confirm the skill still requires sourced output and visible `incomplete: true` handling for missing coverage. — Method step 4 requires every claim to trace to a source URL or repo-local artifact and says to set `incomplete: true` if any required bar is missing
- [x] Confirm the skill still points to the same `docs/research/<slug>/` artifact paths. — `SKILL.md` Outputs lists `dossier.md`, `sources.md`, and `synthesis.md` under `docs/research/<slug>/`
- [x] Confirm the optional batch guidance is clearly separated from the interactive Copilot path. — `SKILL.md` has a separate `Optional path — optional Claude batch backend` section gated on explicit user request

## Local validation

- [x] `cargo test -p sldo-install --test e2e_agent_host_m3`
- [x] `cargo test -p sldo-research`
- [x] `cargo build -p sldo-research`
- [x] `git status --short` shows no untracked test artifacts
