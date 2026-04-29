# Agent Host Capabilities

This document is the capability matrix for the current host story. Use it when you need to answer a simple question: "does this work in Claude Code, GitHub Copilot, both, or neither?"

## Reading rule

- If the capability is in the catalog, it is part of the pack.
- If the capability is in this matrix, it is honest about current host support.
- If a capability is missing here, do not promise it.

## Capability matrix

| Capability | Claude Code | GitHub Copilot | Notes |
|---|---|---|---|
| Install skills with `sldo-install` | Supported | Supported | Same `SKILL.md` contract; different install roots |
| Project-local install with `--local` | Supported | Supported | Writes to `./.claude/skills/` or `./.copilot/skills/` |
| Read the canonical skill catalog | Supported | Supported | Catalog is host-neutral |
| Interactive use of installed skills | Supported | Supported | Exact UX depends on the host's skill/session model |
| Host-specific overlay doc | `CLAUDE.md` | `copilot-instructions.md` | Both point back to the same canonical catalog |
| `/slo-research` automated batch backend | Supported | Not supported yet | Current batch backend shells out to `claude` via `sldo-research` |
| Headless runtime automation | Supported on specific Claude-only paths | Not supported yet | No Copilot runtime harness is shipped today |
| Live business judgment runtime harness | Supported as opt-in Claude-only automation | Not supported yet | Current harness depends on `claude -p` |

## Important boundaries

- The installer is multi-host now. The runtime story is not.
- GitHub Copilot should be treated as an interactive host today, not a headless automation target.
- `/slo-research` is the main remaining hidden host boundary. The installed skill is visible in both hosts, but the current batch backend is still Claude-specific.

## What to read next

- [../skill-pack-catalog.md](../skill-pack-catalog.md) for the host-neutral list of shipped skills.
- [../../CLAUDE.md](../../CLAUDE.md) for the Claude Code overlay.
- [../../copilot-instructions.md](../../copilot-instructions.md) for the GitHub Copilot overlay.
- [../getting-started.md](../getting-started.md) for the first-run path.