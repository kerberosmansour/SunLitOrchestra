# Host capability matrix — SunLitOrchestra

> **Purpose**: Document what each supported host (Claude Code, GitHub Copilot, Codex) can install + invoke + run. Drives decisions in M4 (plugin packaging), M5 (host-native agents), and future host additions.
> **Authored**: 2026-05-01 during /slo-execute M4. Retrieval-date for upstream docs cited inline.
> **Status**: ACTIVE — consulted by M5 gate and any future runbook touching host-specific install / invocation paths. Refreshed for agent-operating-contract M2 on 2026-05-19.

## Matrix

| Capability | Claude Code | GitHub Copilot | Codex | Notes |
|---|---|---|---|---|
| Install Markdown skills via `sldo-install` compatibility root | YES (default, `.claude/skills/`) | YES (`.copilot/skills`) | YES (`.codex/skills`) | Existing SLO behavior preserved; these are compatibility roots, not necessarily each host's current official project-skill root |
| Project-local install via `./.<host>/skills/` compatibility root | YES | YES (`./.copilot/skills`) | YES (`./.codex/skills`) | M2 documents the mismatch; it does not migrate installer behavior |
| Official host-native project skill root | `.claude/skills` in the existing SLO contract | `.github/skills`, `.claude/skills`, or `.agents/skills` | `.agents/skills` | Use this row when designing future host-native install support |
| Repository instruction file | `CLAUDE.md` | `.github/copilot-instructions.md` and agent `AGENTS.md` | `AGENTS.md` | Root `copilot-instructions.md` remains a SunLit overlay but `.github/copilot-instructions.md` is Copilot's repo-wide entrypoint |
| Install via plugin packaging (`.claude-plugin/plugin.json`) | YES (org-install supported) | NO (no equivalent format) | NO (no equivalent format) | Claude-only path |
| Install host-native agents | EXPERIMENTAL - existing SLO files under `agents/<name>.md` | PREVIEW - SLO ships `.github/agents/slo-runbook-review-lead.agent.md`, `.github/agents/slo-security-reviewer.agent.md`, `.github/agents/slo-design-reviewer.agent.md`, `.github/agents/slo-verification-lead.agent.md` | Codex has no shipped SLO host-native custom-agent equivalent | M3 profile files exist; still do not treat this as Copilot/Codex runtime harness parity |
| Invoke skills via slash commands (`/<name>`) | YES | YES | YES | All hosts consume the portable Markdown `SKILL.md` contract interactively |
| Run SLO multi-agent dispatch (lead -> specialists) | EXPERIMENTAL | Not shipped | Not shipped | No Copilot or Codex runtime harness is shipped today |
| Maintain shared install manifest (`~/.sldo/install.toml`) | YES | YES | YES | Manifest records per-host entries |

## Sources (retrieval-date)

- Claude Code plugin format: Anthropic public docs (snapshot 2026-05-01).
- Anthropic Claude Agent SDK: public docs (snapshot 2026-05-01).
- GitHub Copilot repository custom instructions: GitHub docs (snapshot 2026-05-19).
- GitHub Copilot agent skills: GitHub docs (snapshot 2026-05-19) — project roots `.github/skills`, `.claude/skills`, `.agents/skills`; personal roots `~/.copilot/skills`, `~/.agents/skills`.
- GitHub Copilot custom agents: GitHub docs (snapshot 2026-05-19) — repository profiles under `.github/agents/*.agent.md`.
- Codex `AGENTS.md` and skills: OpenAI docs (snapshot 2026-05-19) — project instructions via `AGENTS.md`, repo skills under `.agents/skills`.
- SLO compatibility roots: local `sldo-install` contract in this repo (`~/.copilot/skills`, `./.copilot/skills`, `~/.codex/skills`, `./.codex/skills`) as of 2026-05-19.

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

**Decision**: ship 4 agent files under `agents/`. Each file declares `copilot-fallback: /slo-critique persona rotation` (or equivalent) so non-Claude users are not second-class. This decision originally shipped an additive Claude-oriented enhancement; agent-operating-contract M3 later adds Copilot custom-agent profile files without changing the canonical portable critique flow.

**Reasoning**:

1. Anthropic's Claude Agent SDK supports declarative agent files at `agents/<name>.md`.
2. At the time of M5, GitHub Copilot and Codex had no host-native agent equivalent in this repo. Per the matrix, both could run `/slo-critique` directly, which performs four-persona rotation in-skill. Agent-operating-contract M3 adds Copilot custom-agent profiles; Codex still uses the portable fallback.
3. The structural-contract test asserts each agent file declares `copilot-fallback:` — so the portable fallback story is *enforced*, not just *promised*.
4. Output paths constrained to `{docs/slo/critique/, docs/slo/verify/}` (per F-SEC-6 critique resolution).

**Gate**: M5 may proceed if and only if this matrix says agents are installable on at least one host AND non-Claude hosts have a documented fallback. Both conditions hold here.

### M2 compatibility-root decision — `green-lit`

**Decision**: preserve existing `sldo-install` compatibility roots for M2 and document current official host-native roots separately. Do not silently migrate Copilot local installs from `./.copilot/skills` to `.github/skills`, and do not silently migrate Codex local installs from `./.codex/skills` to `.agents/skills`.

**Reasoning**:

1. Existing tests, docs, and user workflows already depend on `./.copilot/skills` and `./.codex/skills`.
2. Current host docs now expose official project-skill roots that differ from those compatibility roots.
3. A migration would require explicit installer behavior, manifest, uninstall, smoke-test, and rollback design. That is larger than M2's docs/test-only contract.
4. Future work may add an explicit `--scope project-native` or host-root override, but it must preserve manifest safety and uninstall boundaries.

### M3 Copilot custom-agent profiles — `green-lit, preview convenience`

**Decision**: ship four GitHub Copilot custom-agent profiles under `.github/agents/*.agent.md`, matching the existing SLO review/verification roles:

- `.github/agents/slo-runbook-review-lead.agent.md`
- `.github/agents/slo-security-reviewer.agent.md`
- `.github/agents/slo-design-reviewer.agent.md`
- `.github/agents/slo-verification-lead.agent.md`

**Reasoning**:

1. GitHub's current custom-agent docs support repository agent profiles in `.github/agents/` with `.agent.md` filenames, required `description`, optional `tools`, and optional `target: github-copilot`.
2. The existing Claude-oriented `agents/*.md` roles are already bounded by output-path and fallback tests; the Copilot profiles mirror those role boundaries rather than inventing new behavior.
3. Codex has no shipped SLO host-native custom-agent equivalent. Codex users keep using `/slo-critique` and `/slo-verify` directly.
4. These profiles are not a SLO headless runtime harness. No Copilot or Codex runtime harness is shipped today.

**Guardrails**:

- Lead profile may read/search/edit and invoke other custom agents; specialist security/design profiles are read/search only.
- Verification profile may execute the milestone's declared checks and write only bounded verification reports.
- No profile edits `skills/`, installer code, or host-root behavior.

## When to revisit

- New host emerges (e.g., Cursor, Cody) — add a column to the matrix and re-run M4 + M5 gates.
- Anthropic plugin format changes — update Sources retrieval date and re-verify the plugin.json shape.
- GitHub Copilot or Codex adds stronger host-native plugin/agent install — promote fallback rows from "documented fallback" to "first-class" only after profile files, tests, and runtime boundaries are updated.
- Stale retrieval-date (> 12 months from authoring) — re-fetch upstream docs and update the matrix.
