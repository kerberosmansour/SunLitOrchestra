---
name: fallback-discipline
status: stable-template
created: 2026-05-04
audience: skills that can continue in degraded mode when a remote, tool, or dependency is unavailable
purpose: Shared graceful-degradation and fail-loud pattern.
---

# Fallback Discipline Template

Fallbacks preserve the user-visible discipline without pretending the primary path ran. This template cites `references/templates/citation-discipline.md` because fallback claims need clear source status.

## Fallback Row

When the primary path fails, record:

- `primary_path:` tool, remote, source, or file that failed.
- `failure:` exit code, HTTP status, missing file, or unavailable command.
- `fallback_path:` local equivalent, cached source, manual mode, or no fallback.
- `discipline_preserved:` what guarantee remains true.
- `discipline_lost:` what guarantee is weaker or unavailable.

## Rules

- Prefer local cached evidence over model memory.
- If no fallback preserves the core guarantee, stop and say so.
- Do not label fallback output as verified primary output.
- When a cache is used, record cache path and freshness.

## Examples

- Advisory DB unavailable -> skipped/tool-error row, not a vulnerability finding.
- GitHub rate-limited -> spill issue body to a local draft file, do not silently drop.
- Research source unreachable -> mark dossier incomplete and list the missing source.
