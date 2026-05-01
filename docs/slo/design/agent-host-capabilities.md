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
| Optional Claude batch backend (`sldo-research`) | Supported | Not supported yet | The interactive `/slo-research` path is host-neutral; only the optional batch backend is Claude-only |
| Headless runtime automation | Supported on specific Claude-only paths | Not supported yet | No Copilot runtime harness is shipped today |
| Live business judgment runtime harness | Supported as opt-in Claude-only automation | Not supported yet | Current harness depends on `claude -p`; the helper module is `crates/sldo-install/tests/common/claude_runtime.rs` |

## Per-skill notes

This list covers skills whose host story is non-obvious. Skills not listed here are uniformly host-neutral at the `SKILL.md` layer.

| Skill | Host story | Why it matters |
|---|---|---|
| `/slo-second-opinion` | Host-neutral logic. The skill compares whatever the current host produced against an external provider CLI (Codex or Gemini). It does not require Claude. | The skill's "current host said" column captures the agent running this skill, not Claude specifically. The skill must never silently fall back to asking the current host to imitate the other provider. |
| `/slo-research` | Host-neutral interactive path. Optional Claude batch backend (`sldo-research`) is explicitly Claude-only. | A Copilot user can use `/slo-research` interactively without installing Claude; only the batch backend requires `claude`. |
| `/slo-rulegen` | Host-neutral. The bug-summary input can come from any agent-driven workflow. | Earlier copy implied "Claude-found bug summary"; that is no longer accurate. |
| `/slo-sast` | Host-neutral. M1 is parser-only; later milestones shell out to `git`, `gh`, and `semgrep` — none of which are agent CLIs. | The `claude` mention in the M1 anti-pattern list is honest: M1 must not shell out to any agent CLI. |
| Live business judgment runtime harness | Claude-only by design. | There is no host-neutral abstraction; the runbook explicitly forbids inventing one without a second real implementation. |

## Important boundaries

- The installer is multi-host now. The runtime story is not.
- GitHub Copilot should be treated as an interactive host today, not a headless automation target.
- `/slo-research` no longer hides a second-agent dependency in the interactive path. Only the optional batch backend remains Claude-specific.

## What to read next

- [../skill-pack-catalog.md](../skill-pack-catalog.md) for the host-neutral list of shipped skills.
- [../../CLAUDE.md](../../CLAUDE.md) for the Claude Code overlay.
- [../../copilot-instructions.md](../../copilot-instructions.md) for the GitHub Copilot overlay.
- [../getting-started.md](../getting-started.md) for the first-run path.
