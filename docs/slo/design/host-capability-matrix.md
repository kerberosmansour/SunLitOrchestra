# Host capability matrix — SunLitOrchestrate

> **Purpose**: Document what each supported host (Claude Code, GitHub Copilot, Codex) can install + invoke + run. Drives decisions in M4 (plugin packaging), M5 (host-native agents), and future host additions.
> **Authored**: 2026-05-01 during /slo-execute M4. Retrieval-date for upstream docs cited inline.
> **Status**: ACTIVE — consulted by M5 gate and any future runbook touching host-specific install / invocation paths.

## Matrix

| Capability | Claude Code | GitHub Copilot | Codex | Notes |
|---|---|---|---|---|
| Install Markdown skills via `~/<host>/skills/<name>/SKILL.md` | YES (default, `.claude/skills/`) | YES (`.copilot/skills/`) | YES (`.codex/skills/`) | All three supported by `sldo-install` today |
| Project-local install via `./.<host>/skills/` | YES | YES | YES | `--local` selects the host-specific project root |
| Install via plugin packaging (`.claude-plugin/plugin.json`) | YES (org-install supported) | NO (no equivalent format) | NO (no equivalent format) | Claude-only path |
| Install host-native agents (`agents/<name>.md`) | EXPERIMENTAL - Claude Code agent SDK supports declarative agent files | NO (no equivalent format) | NO (no equivalent format) | M5 gate input |
| Invoke skills via slash commands (`/<name>`) | YES | YES | YES | All hosts consume the portable Markdown `SKILL.md` contract interactively |
| Run multi-agent dispatch (lead -> specialists) | EXPERIMENTAL | NO | NO | Out of scope for M5 unless an explicit feature-flag fallback is documented |
| Maintain shared install manifest (`~/.sldo/install.toml`) | YES | YES | YES | Manifest records per-host entries |

## Sources (retrieval-date)

- Claude Code plugin format: Anthropic public docs (snapshot 2026-05-01).
- Anthropic Claude Agent SDK: public docs (snapshot 2026-05-01).
- GitHub Copilot extensibility: GitHub docs (snapshot 2026-05-01).
- Codex skill-root behavior: local Codex runtime contract in this repo (`~/.codex/skills/`, `./.codex/skills/`) as of 2026-05-03.

## Decisions

### M4 plugin packaging — `green-lit`

**Decision**: ship `.claude-plugin/plugin.json` + a SHA-pinned release-zip workflow. Additive to `sldo-install`; does not replace it.

**Reasoning**:

1. Claude Code organizational installs benefit from a one-zip artifact compared to cloning the repo + running `sldo-install`. Real friction for non-developer Claude Code users.
2. The plugin manifest points at the existing `skills/` tree (no duplication). Source-of-truth remains `docs/skill-pack-catalog.md`.
3. `sldo-install` remains canonical for multi-host (Claude Code + GitHub Copilot + Codex). README wording made explicit.
4. Per F-SEC-3 / F-SEC-4 / F-SEC-5, structural-contract tests enforce SHA-pinning, no-duplication, no-path-traversal, minimum-privilege `permissions:`, `git archive` HEAD-only emission. The hardening is structural, not procedural.

**Out-of-scope reaffirmed**:

- No Copilot or Codex plugin path - neither host has an equivalent format in this repo. README does NOT downgrade `sldo-install`.
- No registry auto-publish.
- No artifact signing in M4. A future runbook can add sigstore/cosign once the basic packaging path exercises in production.

### M5 host-native agents — `green-lit, with portable fallback documented`

**Decision**: ship 4 agent files under `agents/`. Each file declares `copilot-fallback: /slo-critique persona rotation` (or equivalent) so non-Claude users are not second-class. The agent files are an additive Claude-only enhancement; the canonical portable critique flow stays `/slo-critique` persona rotation.

**Reasoning**:

1. Anthropic's Claude Agent SDK supports declarative agent files at `agents/<name>.md`.
2. GitHub Copilot and Codex have no host-native agent equivalent in this repo. Per the matrix, both can run `/slo-critique` directly, which performs four-persona rotation in-skill. That IS the fallback.
3. The structural-contract test asserts each agent file declares `copilot-fallback:` — so the portable fallback story is *enforced*, not just *promised*.
4. Output paths constrained to `{docs/slo/critique/, docs/slo/verify/}` (per F-SEC-6 critique resolution).

**Gate**: M5 may proceed if and only if this matrix says agents are installable on at least one host AND non-Claude hosts have a documented fallback. Both conditions hold here.

## When to revisit

- New host emerges (e.g., Cursor, Cody) — add a column to the matrix and re-run M4 + M5 gates.
- Anthropic plugin format changes — update Sources retrieval date and re-verify the plugin.json shape.
- GitHub Copilot or Codex adds host-native plugin/agent install — promote fallback rows from "documented fallback" to "first-class" and update agent files.
- Stale retrieval-date (> 12 months from authoring) — re-fetch upstream docs and update the matrix.
