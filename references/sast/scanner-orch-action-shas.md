# Scanner-orchestration action SHAs

> Pinned 40-character commit SHAs for the GitHub Actions used in the workflow template at [`references/sast/scanner-orch-workflow-template.yml`](scanner-orch-workflow-template.yml). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md). Bumping any SHA below requires a PR with reviewer attention.

## Pinned SHAs

| Action | Pinned SHA (40-char) | Approximate version | Pinned date | Pinned by |
|---|---|---|---|---|
| `actions/checkout` | `0000000000000000000000000000000000000000` | v4.2.x (TBD on first real pin) | TBD | TBD |
| `github/codeql-action/upload-sarif` | `0000000000000000000000000000000000000000` | v3.x (TBD on first real pin) | TBD | TBD |

**Status**: PLACEHOLDER pending first real pin. The all-zero SHAs are intentionally invalid — when the workflow template is rendered with these placeholders, `actions/checkout@0000...0` does not resolve, and the workflow fails at runtime in CI. This forces the wedge-validation step (CEO-1) to perform a real pin-bump before the skill can be exercised against a real target repo.

## SHA-only enforcement

The same constraint as the upstream-rules pin: only 40-character hex SHAs accepted. No tag references. No branch references. No short SHAs. The skill MUST refuse to emit a workflow if either placeholder is in effect.

## Bump procedure

1. Identify the latest stable release SHA from the action's GitHub releases page.
2. Verify the SHA against the action's release advisory or GitHub Security Lab guidance.
3. Update both the SHA and the version annotation in the table above.
4. Open a PR titled `chore(scanner-orch): bump <action> pin <old-7> → <new-7>`.
5. Reviewer checks the upstream diff and the release notes for any breaking changes.
6. Merge after review. No auto-merge.

## Refresh cadence

**Recommended cadence: 90 days.** Rationale: balances supply-chain freshness against bump-ceremony cost. More frequent bumps are acceptable when:

- A CVE advisory targets the pinned action (e.g., CVE-2025-30066 `tj-actions/changed-files` — the canonical tag-rewriting failure case that motivated SHA pinning in the first place).
- A material safety improvement lands upstream (e.g., a hardened `permissions` default).

Less frequent bumps are acceptable when:

- The action surface is stable and no advisories are pending.
- The bump-PR overhead is cost-prohibitive for the maintenance team.

## Why these two actions

- **`actions/checkout`** — required to clone the target repo into the runner; mandated `fetch-depth: 0` for `semgrep ci` diff-aware scans. No substitute.
- **`github/codeql-action/upload-sarif`** — required to upload Semgrep SARIF output to GitHub Code Scanning, which surfaces findings as PR review comments without Semgrep AppSec Platform. No substitute (Code Scanning is GitHub-native; alternative paths require Semgrep AppSec Platform).

**Other actions** are NOT permitted in the emitted workflow. Adding a third SHA to this table requires a fresh `/slo-architect` decision — the surface this skill emits is intentionally narrow.

## Stability

This file is `evolving` per the cadence above. The 40-char-SHA-only constraint and the closed-enumeration of two actions are `stable` per [`docs/design/scanner-orchestration-interfaces.md` §6](../../docs/design/scanner-orchestration-interfaces.md).
