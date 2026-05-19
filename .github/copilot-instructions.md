# SunLit Orchestra - GitHub Copilot Instructions

This is the repository-wide GitHub Copilot custom-instructions file. For the detailed Copilot overlay, read [copilot-instructions.md](../copilot-instructions.md). For the canonical skill list, read [docs/skill-pack-catalog.md](../docs/skill-pack-catalog.md).

Follow [references/agent/operating-contract.md](../references/agent/operating-contract.md): ask when ambiguous, make the smallest safe change, respect SLO allow-lists, verify before claiming, and keep host-boundary claims honest.

Use SLO skills for process-heavy work. The portable skill contract is `skills/<name>/SKILL.md`; do not assume a Claude-only path is available in GitHub Copilot.

Optional Copilot custom-agent profiles live under `.github/agents/*.agent.md`. They are bounded conveniences for selected review/verification roles, not a replacement for `/slo-critique` or `/slo-verify` and not a SLO headless runtime harness.

Before claiming support for skills, custom instructions, custom agents, plugins, or headless runtime automation, check [docs/slo/design/agent-host-capabilities.md](../docs/slo/design/agent-host-capabilities.md).

For this repo, the normal structural test baseline is:

```bash
cargo test -p sldo-common -p sldo-install -p sldo-research
```
