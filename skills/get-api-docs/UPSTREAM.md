# Upstream attribution — get-api-docs

This skill is vendored verbatim from the Context Hub project by Andrew Ng / aisuite.

## Source

- **Repository**: https://github.com/andrewyng/context-hub
- **Path**: `cli/skills/get-api-docs/SKILL.md`
- **License**: MIT (see upstream `LICENSE` file)

## Fetched

- **Commit**: `596506ebb4d53cfbc6ae458b731e0b1a18790f9e`
- **Date**: 2026-03-21T00:44:31Z
- **Fetched on**: 2026-04-24

## Vendoring policy

We do not modify the upstream body. When upstream updates the skill body and we
want the changes, we re-vendor wholesale: replace `SKILL.md`, bump the commit
hash in this file, record the fetch date.

If local customizations are ever needed, they live in a separate SKILL.md file
under a different skill name — never inside `get-api-docs/SKILL.md`.

## Runtime prerequisite

This skill calls the `chub` CLI. Install it with:

```bash
npm install -g @aisuite/chub
```

If `chub` is not on PATH when the skill runs, the skill body instructs the agent
to print an install hint and exit.

## Verifying this copy matches upstream

```bash
# From the repo root:
curl -s https://raw.githubusercontent.com/andrewyng/context-hub/596506ebb4d53cfbc6ae458b731e0b1a18790f9e/cli/skills/get-api-docs/SKILL.md \
  | diff - skills/get-api-docs/SKILL.md
```

No diff → vendored copy is faithful.
