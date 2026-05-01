# Getting Started

This guide is for someone opening SunLitOrchestrate for the first time and wanting one clear path from clone to first successful skill run.

SunLitOrchestrate ships a **skill pack**: a folder of Markdown instruction files that you install into a coding agent. The coding agent is the **host**. Right now the supported hosts are Claude Code and GitHub Copilot.

## Quick glossary

- **Skill pack**: the collection of `SKILL.md` files in the `skills/` directory.
- **Host**: the coding agent that reads those installed skill files, such as Claude Code or GitHub Copilot.
- **Overlay doc**: the host-specific notes for one host. In this repo those are [CLAUDE.md](../CLAUDE.md) and [copilot-instructions.md](../copilot-instructions.md).
- **Manifest**: the install record written by `sldo-install` so it knows which host owns which installed symlink.

## Prerequisites

You need these before the first run:

- Git, so you can clone the repo.
- A stable Rust toolchain, because `sldo-install` is a Rust binary you build locally.
- One supported host: Claude Code or GitHub Copilot.
- Semgrep only if you want to use the SAST rule-pack skills right away.

## Install the skill pack

Run these commands from the repo root:

```bash
git clone https://github.com/kerberosmansour/SunLitOrchestrate.git
cd SunLitOrchestrate

cargo build -p sldo-install --release
```

Install into Claude Code:

```bash
./target/release/sldo-install
./target/release/sldo-install status
```

Install into GitHub Copilot:

```bash
./target/release/sldo-install --host github-copilot
./target/release/sldo-install --host github-copilot status
```

What success looks like:

- `sldo-install` prints the selected target root.
- `status` prints the installed skills for the host you chose.
- Global installs land in `~/.claude/skills/` or `~/.copilot/skills/`.
- Local installs land in `./.claude/skills/` or `./.copilot/skills/` if you add `--local`.

## Run your first skill

1. Open the repo in your chosen host.
2. Read the matching overlay doc: [CLAUDE.md](../CLAUDE.md) for Claude Code or [copilot-instructions.md](../copilot-instructions.md) for GitHub Copilot.
3. Start with `/slo-ideate` if you have a new product or feature idea.
4. Expect the first concrete output to be a file like `docs/slo/idea/<slug>.md`.

If you already have an idea doc, the usual next step is `/slo-research`, then `/slo-architect`, then `/slo-plan`.

## Troubleshooting

### `sldo-install` is not found

Make sure you built it first with `cargo build -p sldo-install --release`, then run it from `./target/release/sldo-install`.

### The skills do not appear in my host

Run `status` for the same host you installed to:

```bash
./target/release/sldo-install status
./target/release/sldo-install --host github-copilot status
```

If the install target is right but the host still does not show the skills, restart the session or the editor and check the host's installed-skills UI again.

### Copilot install says there is a conflict in `~/.copilot/skills/`

That means a path already exists there as a normal directory or file instead of an installer-managed symlink. `sldo-install` refuses to overwrite that silently. Move or remove the conflicting path first, then rerun the install.

### `/slo-research` mentions `sldo-research`

The interactive `/slo-research` path now works through the host's own research tools and writes the same three files under `docs/slo/research/<slug>/`. `sldo-research` is still available, but only as an optional Claude batch backend for users who explicitly want that automation path.
