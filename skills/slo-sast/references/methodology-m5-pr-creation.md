---
name: slo-sast-methodology-m5-pr-creation
source_skill: skills/slo-sast/SKILL.md
stage: M5
status: stable-reference
---

# /slo-sast Methodology M5 — Re-derivation Trigger Detection + Diff PR Generation

## Method (M5 — re-derivation trigger detection + diff PR generation)

### Re-derivation trigger evaluation

Per [`references/sast/scanner-orch-rederivation-triggers.md`](../../../references/sast/scanner-orch-rederivation-triggers.md), evaluate the four predicates at every invocation when `.semgrep/manifest.json` exists:

1. **Threat-model SHA changed** — current `git ls-files -s docs/slo/design/<slug>-threat-model.md` blob SHA differs from `manifest.threat_model_sha`.
2. **`semgrep_rules_sha` pin bumped** — current pinned value in `references/sast/scanner-orch-pinned-rules-sha.md` differs from `manifest.semgrep_rules_sha`.
3. **Stack added** — current manifest-file inventory yields a `detected_stack` that's a superset of `manifest.detected_stack`.
4. **CWEs claimed changed** — current parser output differs from `manifest.cwes_claimed`.

If no triggers fire, exit 0 with stderr `"scanner-orch: no drift detected"`. No PR is opened. No artifacts are touched.

### PR creation

If at least one trigger fires:

1. Re-derive the full pipeline (M2 stack detect + fetch + filter + M3 emission + M4 manifest) into a tempdir, NOT into the target repo.
2. Compute the manifest field diffs.
3. Construct PR title per the format in `references/sast/scanner-orch-rederivation-triggers.md` (`[scanner-orch] re-derive: <one-line trigger summary>`, length-capped to 70 chars).
4. Construct PR body per the template in the triggers reference doc — Markdown structure with one section per fired trigger plus a manifest field diff table.
5. Create a feature branch (`scanner-orch/re-derive/<short-SHA>` or similar) and commit the regenerated artifacts.
6. Invoke `gh pr create` with argv-list form: `gh`, `pr`, `create`, `--title`, `<title>`, `--body`, `<body>`. **NO other flags.**

### `gh pr create` invocation discipline

Read [`references/sast/scanner-orch-rederivation-triggers.md`](../../../references/sast/scanner-orch-rederivation-triggers.md) for the full rule set. Critical:

- **argv-list only** (SEC-6) — never shell-string interpolation.
- **NO `--repo` flag** (SEC-8) — rely on `gh`'s default origin-based resolution. Defends against confused-deputy via tampered `.git/config`.
- **NO merge flags** — no `--auto`, `--squash`, `--rebase`, `--admin`, `--merge`. No `gh pr merge`. Ever.
- **Max 1 PR per invocation (ENG-4)** — even with multiple triggers firing, exactly one `gh pr create` is invoked. Cross-invocation rate is the user's responsibility (CI throttling, threat-model edit cadence).
- **NO `gh auth login`** from the skill — use existing user auth or fail with clear error.
- **NO swallowing of `gh` errors** — forward stderr; exit non-zero on failure.

### Dogfood (runbook-close validation)

The runbook closes by running `/slo-sast` against this SLO repo using `docs/slo/design/scanner-orchestration-threat-model.md` as input. The dogfood test fixture mirrors the real SLO subtree via **file-content copy** (NOT symlinks — ENG-6) to a `tempfile::TempDir`. Every test step that might write into the dogfood-subtree fixture stays inside the tempdir; the real repo is never mutated by the test.

### Anti-patterns (M5 specific)

- **Auto-merge in any form.** Every re-derivation surfaces as a human-review PR.
- **Cross-repo filing.** The skill targets only the current repo's origin remote.
- **Embedding scan findings in the PR body.** The PR is about config drift, not findings — findings go through Code Scanning.
- **Embedding threat-model prose in the PR body.** Same template-skeleton discipline as M3's workflow YAML.
- **Symlinks in the dogfood subtree fixture.** ENG-6 mandates copy-only.
- **Skipping argv-list discipline for `gh` or `git`.** SEC-6 applies to every subprocess invocation.
- **Passing `--repo`** to `gh pr create`. SEC-8 — confused-deputy defense.
- **Persisting state across invocations** to "improve" rate-limiting. Single-invocation discipline; cross-invocation rate is external.
