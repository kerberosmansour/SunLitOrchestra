# SunLitOrchestrate Skill Pack

This directory is the raw source for every `/slo-*` skill plus any third-party skills vendored into the pack. Each subdirectory here is one skill; each must contain a `SKILL.md` with YAML frontmatter (`name`, `description`) followed by the skill body.

The canonical living catalog is [../docs/skill-pack-catalog.md](../docs/skill-pack-catalog.md). Host-specific session overlays live in [../CLAUDE.md](../CLAUDE.md) and [../copilot-instructions.md](../copilot-instructions.md). The first-run guide lives in [../docs/getting-started.md](../docs/getting-started.md).

## Layout

```
skills/
  README.md                 # this file
  slo-ideate/SKILL.md       # first-party skills, /slo-<name>
  slo-retro/SKILL.md
  ...
  get-api-docs/             # third-party, vendored (see UPSTREAM.md inside)
    SKILL.md
    UPSTREAM.md
```

## Install

From the repo root:

```bash
cargo build --release --bin sldo-install
./target/release/sldo-install                          # global Claude Code install: ~/.claude/skills/<name>/
./target/release/sldo-install --host github-copilot   # global GitHub Copilot install: ~/.copilot/skills/<name>/
./target/release/sldo-install --local                  # project-local Claude Code install: ./.claude/skills/<name>/
./target/release/sldo-install --host github-copilot --local
                                                       # project-local GitHub Copilot install: ./.copilot/skills/<name>/
```

Running the installer twice is a no-op. `claude-code` is the default host if you do not pass `--host`. Pass `--force` to replace an existing symlink that points somewhere else. Pass `--dry-run` to see what would change.

## Uninstall

```bash
./target/release/sldo-install uninstall
./target/release/sldo-install --host github-copilot uninstall
```

Only removes symlinks the installer created for the selected host (recorded in `~/.sldo/install.toml` for global installs, or the host-local manifest for `--local`). Manual customizations are preserved.

## Status and verification

```bash
./target/release/sldo-install status                         # list Claude Code installs
./target/release/sldo-install verify                         # verify Claude Code installs
./target/release/sldo-install --host github-copilot status  # list GitHub Copilot installs
./target/release/sldo-install --host github-copilot verify  # verify GitHub Copilot installs
```

## Adding a new skill

1. Create `skills/<name>/SKILL.md` with frontmatter declaring `name` and `description`.
2. Re-run `sldo-install` — it picks up the new skill with zero code changes.
3. Update the catalog at [../docs/skill-pack-catalog.md](../docs/skill-pack-catalog.md).
4. If the skill replaces or follows another in the sprint, update the runbook at [../docs/slo/completed/RUNBOOK-SKILL-PACK.md](../docs/slo/completed/RUNBOOK-SKILL-PACK.md).
