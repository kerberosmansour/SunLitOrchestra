# SunLitOrchestrate Skill Pack

Source of truth for every `/slo-*` skill plus any third-party skills vendored into this pack. Each subdirectory here is one skill; each must contain a `SKILL.md` with YAML frontmatter (`name`, `description`) followed by the skill body.

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
./target/release/sldo-install          # global install: ~/.claude/skills/<name>/
./target/release/sldo-install --local  # project-local: ./.claude/skills/<name>/
```

Running the installer twice is a no-op. Pass `--force` to replace an existing symlink that points somewhere else. Pass `--dry-run` to see what would change.

## Uninstall

```bash
./target/release/sldo-install uninstall
```

Only removes symlinks the installer created (recorded in `~/.sldo/install.toml`). Manual customizations are preserved.

## Status and verification

```bash
./target/release/sldo-install status   # list installed skills
./target/release/sldo-install verify   # check symlinks still match manifest
```

## Adding a new skill

1. Create `skills/<name>/SKILL.md` with frontmatter declaring `name` and `description`.
2. Re-run `sldo-install` — it picks up the new skill with zero code changes.
3. Update the catalog at [../docs/skill-pack-catalog.md](../docs/skill-pack-catalog.md).
4. If the skill replaces or follows another in the sprint, update the runbook at [../docs/RUNBOOK-SKILL-PACK.md](../docs/RUNBOOK-SKILL-PACK.md).
