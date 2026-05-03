# SunLitOrchestrate - Codex overlay

This file is the Codex overlay for the canonical living catalog at [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md). Use it when you are working in Codex and need the Codex-specific install path, reading order, and limitations.

If you are new to the repo, start with [docs/getting-started.md](docs/getting-started.md). For the other host overlays, read [CLAUDE.md](CLAUDE.md) or [copilot-instructions.md](copilot-instructions.md).

## Read this first

1. [docs/getting-started.md](docs/getting-started.md) - first-run path
2. [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) - canonical living catalog
3. [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) - what works today and what is still host-specific

## Install into Codex

From the repo root:

```bash
cargo build -p sldo-install --release
./target/release/sldo-install --host codex
./target/release/sldo-install --host codex status
./target/release/sldo-install --host codex verify
```

Project-local install:

```bash
./target/release/sldo-install --host codex --local
```

Global installs land in `~/.codex/skills/`. Local installs land in `./.codex/skills/`.

## What works today

- The installed skill contract is the same Markdown `SKILL.md` format used by the other hosts.
- `sldo-install` can install, list, verify, and uninstall Codex-targeted skills.
- The canonical skill list stays in [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md), so this file only explains Codex-specific details.
- After installing or updating skills, reload or restart the Codex session if the current session does not pick up the new skill list.

## Current limitations

- Headless runtime automation in Codex is **not supported yet**. Do not assume there is a Codex CLI runtime equivalent to the Claude-only test harnesses.
- `/slo-research` supports host-native interactive research. The separate `sldo-research` path remains an optional Claude batch backend when a user explicitly wants batch automation.
- The live business judgment runtime harness remains Claude-only today because it shells out to `claude -p`.
- Specialist agents under [agents/](agents/) are Claude-Code-only. Codex users continue to use `/slo-critique` and `/slo-verify` directly; that is the canonical portable path and produces the same artifact formats.
- `.claude-plugin/plugin.json` is Claude Code only. Codex users install via `sldo-install --host codex`.

## First session checklist

1. Confirm the install target is Codex by running `./target/release/sldo-install --host codex status`.
2. Read [docs/getting-started.md](docs/getting-started.md) for the first-run path.
3. Use [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) to pick the skill you want.
4. Check [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) before assuming a runtime or automation surface exists.
