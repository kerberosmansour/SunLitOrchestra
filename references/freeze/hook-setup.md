---
name: freeze-hook-setup
status: guidance
created: 2026-05-04
audience: maintainers enabling project-local freeze enforcement
purpose: Document the opt-in PreToolUse hook that mirrors /slo-freeze scope into Claude Code.
---

# Freeze Hook Setup

This hook is opt-in per project. It lives in `.claude/settings.json`, not in global settings, and it reads `~/.sldo/freeze-scope.txt` when Claude Code runs `PreToolUse` for `Edit`, `Write`, or `NotebookEdit`.

Use the existing `update-config` skill when it is available so the project settings mutation is additive. Preserve any existing `PreToolUse` entries and append the freeze matcher instead of replacing the array.

The hook is a discipline-enforcer for honest mistakes, not a security boundary. If `~/.sldo/freeze-scope.txt` is missing or empty, no freeze is active and the hook exits without blocking. If the file contains a path, edits outside that path exit non-zero with `freeze: cannot edit ...`.

Recommended setup:

1. Add or extend `.claude/settings.json` in the project.
2. Keep the hook project-local; do not mutate `~/.claude/settings.json`.
3. Have `/slo-freeze <path>` write the active scope into `~/.sldo/freeze-scope.txt` when this hook is enabled.
4. Have `/slo-unfreeze` remove or empty `~/.sldo/freeze-scope.txt`.
5. Re-open `/hooks` in Claude Code to confirm the project hook is visible.

Residual risk: a user or tool can delete `~/.sldo/freeze-scope.txt`, so this is not an adversarial containment mechanism. Treat it as a guardrail layered on top of the runbook allow-list.
