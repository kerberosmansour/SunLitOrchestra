# `semgrep-rules` pinned SHA

> The 40-character commit SHA of `github.com/semgrep/semgrep-rules` that `/slo-sast` clones into `~/.cache/sldo/semgrep-rules/<SHA>/`. Locked in M2 of the [scanner-orchestration runbook](../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md). Bumping this value requires a PR with reviewer attention to the upstream diff.

## Pinned SHA

```
0000000000000000000000000000000000000000
```

**Status**: PLACEHOLDER pending first real pin. The placeholder SHA is the all-zero blob, which is intentionally invalid — `/slo-sast` must refuse to operate against it. The first pin-bump PR (M2 close + first wedge-validation run) replaces this with a real `git rev-parse HEAD` result from the upstream repo. Until then, the skill exits non-zero with a clear "rules SHA is the placeholder; bump per `references/sast/scanner-orch-pinned-rules-sha.md`" message.

**Pinned date**: TBD (set on first real pin).
**Upstream**: `https://github.com/semgrep/semgrep-rules`
**Pinned by**: TBD

## SHA-only enforcement

The pinned value MUST be a 40-character hex SHA matching regex `^[0-9a-f]{40}$`. The skill MUST refuse to operate against:

- A tag reference (`v1.0`, `release-2026.04`, etc.) — defeats the trust window (tag-rewriting attacks).
- A branch reference (`develop`, `main`, etc.) — same — branches move; pinned points must be immutable.
- A short SHA (Git's default 7-char prefix) — collision-prone.
- An unset / empty / whitespace value — error path.
- The all-zero placeholder above — wedge validation hasn't happened yet.

This is the load-bearing defense against `tm-scanner-orchestration-abuse-2` (compromised semgrep-rules upstream). Tag-rewriting attacks (CVE-2025-30066 `tj-actions/changed-files`, Shai Hulud v2) are bounded to "user explicitly bumps pin" events, which surface as a diff PR for human review.

## Bump procedure

To update the pinned SHA:

1. **Run wedge-validation** (per CEO-1 in the critique). Invoke `/slo-sast` against three representative threat-model fixtures using the proposed-new SHA's cache contents; confirm the CWE × stack intersection still produces meaningful rule sets.
2. **Diff the upstream commits.** `git log <old-SHA>..<new-SHA>` against the `semgrep-rules` repo. Reviewer scans the diff for unusual content size (defends against billion-laughs-style YAML — `tm-scanner-orchestration-abuse-2 / SEC-2`), unexplained metadata changes, or autofix additions that introduce backdoors.
3. **Open a PR** that updates the SHA value above + the Pinned date + Pinned by. Title format: `chore(scanner-orch): bump semgrep-rules pin <old-7> → <new-7>`.
4. **Merge after review.** No auto-merge. The PR title-and-diff is the human-review surface that the discipline depends on.

**Recommended cadence**: 90 days (mirrors the action-SHA refresh cadence in `references/sast/scanner-orch-action-shas.md`). Bumps can be more frequent if upstream ships a critical CWE-coverage improvement (e.g., the 2026-Q1 OWASP-2025 re-mapping), but each bump pays the human-review cost.

## Cache layout

After clone, the cache lives at:

```
~/.cache/sldo/semgrep-rules/<full-40-char-SHA>/
```

(or `$XDG_CACHE_HOME/sldo/semgrep-rules/<SHA>/` if the env var is set — supports the test-harness override + non-default home directories.)

Each pinned SHA gets its own directory; bumping the pin writes a sibling. **Old caches are NEVER overwritten in place.** Cache pruning is the user's responsibility for v1 (residual risk #3 in the threat model — known accepted gap).

## Stability

This pin file is `evolving` per the cadence above. The 40-char-SHA-only constraint and the `~/.cache/sldo/semgrep-rules/<SHA>/` layout are `stable` per [`docs/slo/design/scanner-orchestration-interfaces.md` §7](../../docs/slo/design/scanner-orchestration-interfaces.md).
