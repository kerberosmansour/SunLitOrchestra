# Scanner-orchestration re-derivation triggers

> The four trigger predicates `/slo-sast` evaluates at every invocation to detect drift between the recorded manifest state and the current target-repo state. When any trigger fires, the skill surfaces the change as a `gh pr create` with a structured diff body. Locked in M5 of the [scanner-orchestration runbook](../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md).

## Predicates

| # | Trigger | What it compares | Fires when |
|---|---|---|---|
| 1 | **Threat-model SHA changed** | `git ls-files -s docs/slo/design/<slug>-threat-model.md` (current blob SHA) vs `manifest.threat_model_sha` (recorded) | The threat-model file's content has been edited since the manifest was written. |
| 2 | **`semgrep_rules_sha` pin bumped** | `references/sast/scanner-orch-pinned-rules-sha.md` (current pinned value) vs `manifest.semgrep_rules_sha` (recorded) | A bump-PR landed that updates the pinned upstream-rules SHA. |
| 3 | **Stack added** | Manifest files present in target repo (`Cargo.toml`, `package.json`, etc.) vs `manifest.detected_stack` (recorded) | A new stack appeared (e.g., new `package.json` alongside existing `Cargo.toml`) that wasn't present at last manifest write. |
| 4 | **CWEs claimed changed** | Deduplicated `\bCWE-(\d+)\b` parse output vs `manifest.cwes_claimed` (recorded) | The threat model's CWE list has changed (additions, removals, or both) since manifest was written. |

If `manifest.json` does NOT exist, the skill is on the M4 first-install path and runs preview-mode instead — re-derivation triggers don't apply.

## Compound triggers

If multiple triggers fire on the same invocation, they coalesce into a SINGLE `gh pr create` with a PR body containing one section per trigger. Per `multi_trigger_combined_pr` BDD scenario.

## Rate-limit cap (ENG-4)

**Maximum 1 PR per skill invocation.** The skill is single-invocation by design — drift evaluation runs once per `/slo-sast` call. Even with multiple triggers firing, exactly one `gh pr create` is invoked.

**Cross-invocation rate is the user's responsibility.** If a runaway loop somehow re-invokes `/slo-sast` repeatedly (e.g., a misconfigured CI job that runs the skill on every git push), each invocation would fire its own PR. The defense is external: CI throttling, careful threat-model edit cadence, repo branch protection. The skill itself does not persist state across invocations.

## PR title format

```
[scanner-orch] re-derive: <one-line trigger summary>
```

Examples:

- `[scanner-orch] re-derive: threat-model SHA changed`
- `[scanner-orch] re-derive: semgrep-rules SHA bump (abc123 → def456)`
- `[scanner-orch] re-derive: stack added (added: javascript)`
- `[scanner-orch] re-derive: CWEs added (CWE-22, CWE-79)`
- `[scanner-orch] re-derive: 3 triggers fired` (compound case)

Length cap: 70 chars per project convention. Trim with `…` if a SHA is the dominant content.

## PR body template

The PR body is a Markdown structure with one section per fired trigger:

```markdown
## scanner-orch re-derivation

Triggered by:

### Threat-model SHA changed
- Old SHA: `abc123…`
- New SHA: `def456…`
- Affected CWE list: ...

### semgrep-rules SHA bumped
- Old SHA: `abc123…`
- New SHA: `def456…`
- Rule-set delta: ... (added rules, removed rules, changed rules)

### Stack added
- Previously detected: ["rust"]
- Now detected: ["rust", "javascript"]
- New rules pulled in: ...

### CWEs claimed changed
- Added: ["CWE-22"]
- Removed: ["CWE-77"]
- Affected rules: ...

---

**Manifest field diffs:**

| Field | Before | After |
|---|---|---|
| `threat_model_sha` | abc | def |
| `semgrep_rules_sha` | abc | def |
| `detected_stack` | ["rust"] | ["rust", "javascript"] |
| `cwes_claimed` | [...] | [...] |
| `cwes_actually_covered` | [...] | [...] |

Auto-merge is NOT enabled. Review the rule-set delta and merge when ready.
```

**No content from threat-model prose flows into the PR body.** Only manifest-derived values (CWE codes, SHAs, rule paths) — same template-skeleton discipline as the workflow YAML in M3.

## `gh pr create` invocation discipline

- **argv-list only** (`gh`, `pr`, `create`, `--title`, `<title>`, `--body`, `<body>` as separate args). No shell-string interpolation. SEC-6.
- **NO `--repo` flag.** Rely on `gh`'s default origin-based resolution. Defends against confused-deputy attack via tampered `.git/config` (SEC-8).
- **NO merge flags.** No `--auto`, `--squash`, `--rebase`, `--admin`, `--merge`, or `gh pr merge` invocation.
- **NO `gh auth login`** from the skill. Use the user's existing auth or fail with a clear error.
- **NO swallowing of `gh` errors.** Forward stderr; exit non-zero on `gh` failure.

## Stability

This contract is `stable` per [`docs/slo/design/scanner-orchestration-interfaces.md` §8](../../docs/slo/design/scanner-orchestration-interfaces.md). The four trigger predicates form a closed enumeration; adding a fifth requires fresh `/slo-architect`. The PR title format and rate-limit cap are also `stable`.
