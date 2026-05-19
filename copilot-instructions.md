# SunLit Orchestra — GitHub Copilot overlay

This file is the GitHub Copilot overlay for the canonical living catalog at [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md). Use it when you are working in GitHub Copilot and want the Copilot-specific install path, reading order, and limitations.

If you are new to the repo, start with [docs/getting-started.md](docs/getting-started.md). If you want the Claude-specific version of this file, read [CLAUDE.md](CLAUDE.md). If you want the Codex-specific version, read [AGENTS.md](AGENTS.md).

## Shared operating contract

Follow [references/agent/operating-contract.md](references/agent/operating-contract.md) for the host-neutral agent rules: ask when ambiguous, make the smallest safe change, respect SLO allow-lists, verify before claiming, and keep host-boundary claims honest. GitHub's repo-wide Copilot custom-instructions path is [.github/copilot-instructions.md](.github/copilot-instructions.md); this root file remains the detailed Copilot overlay linked by existing SunLit docs.

## Read this first

1. [docs/getting-started.md](docs/getting-started.md) — first-run path
2. [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) — canonical living catalog
3. [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) — what works today and what is still host-specific

## Install into GitHub Copilot

Fastest path — install from crates.io:

```bash
cargo install sldo-install
sldo-install --host github-copilot
sldo-install --host github-copilot status
sldo-install --host github-copilot verify
```

From a checkout (when iterating on the skill pack itself):

```bash
cargo build -p sldo-install --release
./target/release/sldo-install --host github-copilot
./target/release/sldo-install --host github-copilot status
./target/release/sldo-install --host github-copilot verify
```

Project-local install:

```bash
sldo-install --host github-copilot --local
# or, from a checkout:
./target/release/sldo-install --host github-copilot --local
```

Global installs land in `~/.copilot/skills/`. Local installs land in `./.copilot/skills/`.

## What works today

- The installed skill contract is the same `SKILL.md` format used by Claude Code.
- `sldo-install` can install, list, verify, and uninstall Copilot-targeted skills.
- Optional Copilot custom-agent profiles for the SLO review/verification roles live under `.github/agents/*.agent.md`; they are bounded conveniences, not a SLO headless runtime harness.
- The canonical skill list stays in [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md), so this file only needs to explain Copilot-specific details.

## Current limitations

- Headless runtime automation in GitHub Copilot is **not supported yet**. Do not assume there is a Copilot CLI runtime equivalent to the Claude-only test harnesses.
- `/slo-research` now supports host-native interactive research without installing Claude. The separate `sldo-research` path remains an optional Claude batch backend when a user explicitly wants batch automation.
- The live business judgment runtime harness remains Claude-only today because it shells out to `claude -p`.
- **Specialist agents under [agents/](agents/) remain Claude-oriented**, and Copilot has bounded custom-agent profile counterparts under [.github/agents/](.github/agents/). Copilot and Codex users can always use `/slo-critique` and `/slo-verify` directly; this is the canonical portable path and produces the same `docs/slo/critique/<slug>.md` and `docs/slo/verify/<prefix>-m<N>.md` artifact formats. Codex has no shipped SLO host-native custom-agent equivalent.
- **`.claude-plugin/plugin.json`** (sap-imp M4) is Claude Code only. Copilot users install via `sldo-install --host github-copilot` (canonical, multi-host).

## First session checklist

1. Confirm the install target is Copilot by running `./target/release/sldo-install --host github-copilot status`.
2. Read [docs/getting-started.md](docs/getting-started.md) for the first-run path.
3. Use [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) to pick the skill you want.
4. Check [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) before assuming a runtime or automation surface exists.
