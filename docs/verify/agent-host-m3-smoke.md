# Agent-Host Milestone 3 Smoke Checklist

> Runbook: [docs/RUNBOOK-AGENT-HOST-COMPATIBILITY.md](../RUNBOOK-AGENT-HOST-COMPATIBILITY.md) Milestone 3
> Goal: verify that `/slo-research` is host-native for interactive use and that the Claude batch backend is explicit and optional.
> Local validation was completed from the terminal on 2026-04-29. The Claude Code and GitHub Copilot session items still need to be checked in those hosts directly.

## Claude session

- [ ] Open the installed `/slo-research` skill in Claude Code.
- [ ] Confirm the interactive path tells the agent to use host-native research tools and file writes, not only `sldo-research`.
- [ ] Confirm missing `docs/idea/<slug>.md` still produces a clear refusal that points to `/slo-ideate`.
- [ ] Confirm the skill still writes to `docs/research/<slug>/dossier.md`, `sources.md`, and `synthesis.md`.
- [ ] Confirm any batch guidance is labeled as an optional Claude batch backend, not the default path.

## GitHub Copilot session

- [ ] Open the installed `/slo-research` skill in GitHub Copilot.
- [ ] Confirm the interactive path is usable without installing Claude or `sldo-research`.
- [ ] Confirm the skill still requires sourced output and visible `incomplete: true` handling for missing coverage.
- [ ] Confirm the skill still points to the same `docs/research/<slug>/` artifact paths.
- [ ] Confirm the optional batch guidance is clearly separated from the interactive Copilot path.

## Local validation

- [x] `cargo test -p sldo-install --test e2e_agent_host_m3`
- [x] `cargo test -p sldo-research`
- [x] `cargo build -p sldo-research`
- [x] `git status --short` shows no untracked test artifacts
