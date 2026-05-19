# Getting Started

This guide is for someone opening SunLit Orchestra for the first time and wanting one clear path from clone to first successful skill run.

SunLit Orchestra ships a **skill pack**: a folder of Markdown instruction files that you install into a coding agent. The coding agent is the **host**. Right now the supported hosts are Claude Code, GitHub Copilot, and Codex.

## Quick glossary

- **Skill pack**: the collection of `SKILL.md` files in the `skills/` directory.
- **Host**: the coding agent that reads those installed skill files, such as Claude Code, GitHub Copilot, or Codex.
- **Overlay doc**: the host-specific notes for one host. In this repo those are [CLAUDE.md](../CLAUDE.md), [copilot-instructions.md](../copilot-instructions.md), and [AGENTS.md](../AGENTS.md).
- **Manifest**: the install record written by `sldo-install` so it knows which host owns which installed managed link.

## Prerequisites

You need these before the first run:

- A stable Rust toolchain, because `sldo-install` is a Rust binary distributed via crates.io.
- Git, only if you want to build from source instead of installing from crates.io.
- One supported host: Claude Code, GitHub Copilot, or Codex.
- Semgrep only if you want to use the SAST rule-pack skills right away.

## Install the skill pack

The fastest path is from crates.io — no clone needed:

```bash
cargo install sldo-install
```

Install into Claude Code:

```bash
sldo-install
sldo-install status
sldo-install verify
```

Install into GitHub Copilot:

```bash
sldo-install --host github-copilot
sldo-install --host github-copilot status
sldo-install --host github-copilot verify
```

Install into Codex:

```bash
sldo-install --host codex
sldo-install --host codex status
sldo-install --host codex verify
```

If you want to build from source instead — for example, when iterating on the skill pack itself:

```bash
git clone https://github.com/kerberosmansour/SunLitOrchestra.git
cd SunLitOrchestra
cargo build -p sldo-install --release
./target/release/sldo-install [...]    # same flags as above
```

What success looks like:

- `sldo-install` prints the selected target root.
- `status` prints the installed skills for the host you chose.
- Global installs land in `~/.claude/skills/`, `~/.copilot/skills/`, or `~/.codex/skills/`.
- Local installs land in `./.claude/skills/`, `./.copilot/skills/`, or `./.codex/skills/` if you add `--local`.
- These are SLO installer compatibility root paths. Current host-native project skill roots may differ: GitHub Copilot documents `.github/skills` and `.agents/skills`, and Codex documents `.agents/skills`. SunLit keeps `.copilot/skills` and `.codex/skills` compatible until a deliberate installer migration exists.
- On Windows PowerShell, use `.\target\release\sldo-install.exe` with the same flags shown above.
- Linux and macOS installs use directory symlinks. Windows tries directory symlinks first, then falls back to directory junctions if symlink privileges are unavailable.

## Run your first skill

1. Open the repo in your chosen host.
2. Read the matching overlay doc: [CLAUDE.md](../CLAUDE.md) for Claude Code, [copilot-instructions.md](../copilot-instructions.md) for GitHub Copilot, or [AGENTS.md](../AGENTS.md) for Codex.
3. Start with `/slo-ideate` if you have a new product or feature idea.
4. Expect the first concrete output to be a file like `docs/slo/idea/<slug>.md`.

If you already have an idea doc, the usual next step is `/slo-research`, then `/slo-architect`, then `/slo-plan`.

## Troubleshooting

### `sldo-install` is not found

Make sure you built it first with `cargo build -p sldo-install --release`, then run it from `./target/release/sldo-install`. On Windows PowerShell, the path is `.\target\release\sldo-install.exe`.

### The skills do not appear in my host

Run `status` for the same host you installed to:

```bash
./target/release/sldo-install status
./target/release/sldo-install --host github-copilot status
./target/release/sldo-install --host codex status
```

If the install target is right but the host still does not show the skills, restart the session or the editor and check the host's installed-skills UI again.

### My host install says there is a conflict in its skills directory

That means a path already exists under the selected host root as a normal directory or file instead of an installer-managed link. `sldo-install` refuses to overwrite that silently. Move or remove the conflicting path first, then rerun the install for the same `--host`.

### `/slo-research` mentions `sldo-research`

The interactive `/slo-research` path now works through the host's own research tools and writes the same three files under `docs/slo/research/<slug>/`. `sldo-research` is still available, but only as an optional Claude batch backend for users who explicitly want that automation path.
