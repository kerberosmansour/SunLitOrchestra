# Completion Summary — agent-host Milestone 3

## Goal completed
- `/slo-research` is now usable from skill-capable hosts without a hidden requirement that the current host shell out to `sldo-research`. The Claude batch backend remains explicitly optional and continues to build for users who deliberately want that path.

## Files changed
- `skills/slo-research/SKILL.md` — rewritten around interactive host-native research with an explicit optional Claude batch backend section.
- `crates/sldo-research/src/main.rs` — help text now describes the binary as the optional Claude batch backend.
- `crates/sldo-research/src/research.rs` — module docs/log messages updated for backend honesty.
- `README.md`, `CLAUDE.md`, `copilot-instructions.md`, `docs/skill-pack-catalog.md`, `docs/ARCHITECTURE.md`, `docs/getting-started.md`, `docs/design/agent-host-capabilities.md` — aligned on host-native interactive default and optional Claude batch backend.

## Tests added
- `crates/sldo-install/tests/e2e_agent_host_m3.rs` — structural guards that the interactive path does not require `sldo-research` or `claude`, and that batch guidance is documented as optional Claude-specific.

## Runtime validations added
- `crates/sldo-install/tests/e2e_agent_host_m3.rs` runs as part of `cargo test -p sldo-install` and is the runtime gate for this milestone's contract.

## Compatibility checks performed
- `docs/research/<slug>/` artifact paths unchanged.
- `incomplete: true` remains the explicit missing-coverage signal in the dossier shape.
- `sldo-research --help` exits 0 and continues to build for explicit Claude batch users.
- No living doc tells Copilot users to install Claude in order to use `/slo-research` interactively.

## Documentation updated
- `README.md` — `/slo-research` usage guidance.
- `CLAUDE.md`, `copilot-instructions.md` — host overlays match the new split.
- `docs/skill-pack-catalog.md` — catalog row updated for the host-native interactive default.
- `docs/ARCHITECTURE.md` — recorded the split between interactive skill and optional batch backend.
- `docs/getting-started.md` — first-run guidance no longer implies that interactive `/slo-research` requires `sldo-research`.
- `docs/design/agent-host-capabilities.md` — capability matrix marks the interactive path as host-neutral and the batch backend as Claude-only.
- `docs/verify/agent-host-m3-smoke.md` — manual smoke checklist for both the Claude and GitHub Copilot sessions.

## .gitignore changes
- None. Existing `.claude/`, `.copilot/`, `.sldo-logs/`, `.copilot-logs/`, and `output/` entries already cover the surfaces this milestone touches.

## Test artifact cleanup verified
- `git status` is clean on close. The only tracked changes for this milestone close-out are the runbook tracker update, smoke-checklist edits, this completion summary, and the lessons file.

## Deferred follow-ups
- None for the M3 contract itself. The GitHub Copilot session smoke was completed on 2026-04-30 against the supported repo-local install root `./.copilot/skills/`.

## Known non-blocking limitations
- The global Copilot root `~/.copilot/skills/` was not used for the session smoke because `~/.copilot/skills/get-api-docs` is a pre-existing non-symlink conflict. This does not affect the milestone result because Milestone 1 explicitly supports the repo-local install root `./.copilot/skills/`, which was installed and verified successfully before the Copilot smoke was run.
