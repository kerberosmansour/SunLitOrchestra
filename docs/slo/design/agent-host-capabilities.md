# Agent Host Capabilities

This document is the capability matrix for the current host story. Use it when you need to answer a simple question: "does this work in Claude Code, GitHub Copilot, Codex, or only on one host?"

## Reading rule

- If the capability is in the catalog, it is part of the pack.
- If the capability is in this matrix, it is honest about current host support.
- If a capability is missing here, do not promise it.
- Source refresh date for host-native instruction / skill / agent paths: 2026-05-19.

## Capability matrix

| Capability | Claude Code | GitHub Copilot | Codex | Notes |
|---|---|---|---|---|
| Install skills with `sldo-install` | Supported | Supported | Supported | SLO installer compatibility root behavior; not always the same as the host's current official project-skill root |
| SLO global compatibility root | `~/.claude/skills/` | `~/.copilot/skills/` | `~/.codex/skills/` | Existing `sldo-install` behavior preserved in agent-operating-contract M2 |
| SLO project-local compatibility root with `--local` | `./.claude/skills/` | `./.copilot/skills/` | `./.codex/skills/` | Existing behavior; M2 does not migrate or remove these roots |
| Current host-native project skill roots | `./.claude/skills/` through existing SLO/Claude skill contract | `.github/skills`, `.claude/skills`, or `.agents/skills` | `.agents/skills` | Official host docs now distinguish these from SLO compatibility roots |
| Repository-wide instruction file | `CLAUDE.md` | `.github/copilot-instructions.md`; `AGENTS.md` also applies to agents | `AGENTS.md` | Root `copilot-instructions.md` remains the detailed human-readable Copilot overlay for SunLit docs |
| Host-native custom agent profiles | Claude-oriented `agents/<name>.md` files currently shipped | Shipped preview profiles under `.github/agents/*.agent.md` | Codex has no shipped SLO host-native custom-agent equivalent | M3 ports the four SLO review/verification roles to Copilot while preserving portable skill fallbacks |
| Read the canonical skill catalog | Supported | Supported | Supported | Catalog is host-neutral |
| Interactive use of installed skills | Supported | Supported | Supported | Exact UX depends on the host's skill/session model |
| Host-specific overlay doc | `CLAUDE.md` | `.github/copilot-instructions.md` plus root `copilot-instructions.md` | `AGENTS.md` | All overlays point back to the same canonical catalog |
| Optional Claude batch backend (`sldo-research`) | Supported | Not supported yet | Not supported yet | The interactive `/slo-research` path is host-neutral; only the optional batch backend is Claude-only |
| Headless runtime automation (SLO harness) | Supported on specific Claude-only paths | Not supported yet | Not supported yet | No Copilot or Codex runtime harness is shipped today |
| Live business judgment runtime harness | Supported as opt-in Claude-only automation | Not supported yet | Not supported yet | Current harness depends on `claude -p`; the helper module is `crates/sldo-install/tests/common/claude_runtime.rs` |

## Sources Checked 2026-05-19

- GitHub Copilot repository custom instructions: `.github/copilot-instructions.md`, path-specific `.github/instructions/*.instructions.md`, and `AGENTS.md` for agent instructions.
- GitHub Copilot agent skills: project skills under `.github/skills`, `.claude/skills`, or `.agents/skills`; personal skills under `~/.copilot/skills` or `~/.agents/skills`.
- GitHub Copilot custom agents: repository profiles under `.github/agents/*.agent.md` in public preview. M3 ships `.github/agents/slo-runbook-review-lead.agent.md`, `.github/agents/slo-security-reviewer.agent.md`, `.github/agents/slo-design-reviewer.agent.md`, and `.github/agents/slo-verification-lead.agent.md`.
- OpenAI Codex `AGENTS.md`: Codex reads global and project `AGENTS.md` / `AGENTS.override.md` layers.
- OpenAI Codex skills: repository skills under `.agents/skills`; user skills under `~/.agents/skills`.
- Claude Code memory: `CLAUDE.md` project memory and import mechanics.

## Per-skill notes

This list covers skills whose host story is non-obvious. Skills not listed here are uniformly host-neutral at the `SKILL.md` layer.

| Skill | Host story | Why it matters |
|---|---|---|
| `/slo-second-opinion` | Host-neutral logic. The skill compares whatever the current host produced against an external provider CLI (Codex or Gemini). It does not require Claude. | The skill's "current host said" column captures the agent running this skill, not Claude specifically. The skill must never silently fall back to asking the current host to imitate the other provider. |
| `/slo-research` | Host-neutral interactive path. Optional Claude batch backend (`sldo-research`) is explicitly Claude-only. | A Copilot or Codex user can use `/slo-research` interactively without installing Claude; only the batch backend requires `claude`. |
| `/slo-rulegen` | Host-neutral. The bug-summary input can come from any agent-driven workflow. | Earlier copy implied "Claude-found bug summary"; that is no longer accurate. |
| `/slo-sast` | Host-neutral. M1 is parser-only; later milestones shell out to `git`, `gh`, and `semgrep` — none of which are agent CLIs. | The `claude` mention in the M1 anti-pattern list is honest: M1 must not shell out to any agent CLI. |
| `/slo-nettacker` | Host-neutral interactive skill. It may shell out to a local Nettacker CLI, Docker image, or API after authorization is established. | Live scanning is an external security tool workflow, not a headless agent runtime. Codex and Copilot remain interactive hosts for the skill. |
| Live business judgment runtime harness | Claude-only by design. | There is no host-neutral abstraction; the runbook explicitly forbids inventing one without a second real implementation. |

## Important boundaries

- The installer is multi-host now. The runtime story is not.
- GitHub Copilot and Codex should be treated as interactive hosts today, not headless automation targets.
- Copilot custom-agent profiles are host-native prompt/tool profiles, not a SLO headless runtime harness. No Copilot or Codex runtime harness is shipped today.
- The current `sldo-install` project-local roots for Copilot and Codex are compatibility roots (`./.copilot/skills/`, `./.codex/skills/`). Official host-native repo-skill roots now include `.github/skills` and `.agents/skills`; M2 documents this but does not migrate installer behavior.
- `/slo-research` no longer hides a second-agent dependency in the interactive path. Only the optional batch backend remains Claude-specific.

## What to read next

- [../../skill-pack-catalog.md](../../skill-pack-catalog.md) for the host-neutral list of shipped skills.
- [../../../CLAUDE.md](../../../CLAUDE.md) for the Claude Code overlay.
- [../../../copilot-instructions.md](../../../copilot-instructions.md) for the GitHub Copilot overlay.
- [../../../AGENTS.md](../../../AGENTS.md) for the Codex overlay.
- [../../getting-started.md](../../getting-started.md) for the first-run path.
