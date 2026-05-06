# Lessons Learned — sap-imp Milestone 4

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M4 — Optional Claude plugin packaging assessment + green-lit packaging artifacts
> **Date**: 2026-05-01

## What changed

- New `docs/slo/design/host-capability-matrix.md` — capability matrix for Claude Code + GitHub Copilot, with green-lit decisions for both M4 (plugin packaging) and M5 (agent files with Copilot fallback documented).
- New `.claude-plugin/plugin.json` — minimal manifest pointing at the canonical `skills/` tree. No skill duplication; no path traversal; no absolute paths.
- New `.github/workflows/release-zip.yml` — SHA-pinned release-zip workflow triggered on `v*` tag push only. Generates the artifact via `git archive --format=zip --prefix=sunlit-orchestra-${TAG}/ HEAD` (no runner working-dir leakage). Explicit `permissions: contents: write` block — no other scope.
- Updated `.github/workflows/semgrep.yml` (allow-list extension): SHA-pinned `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2`; added explicit `permissions: contents: read` block.
- New `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` — 6-test structural-contract test enforcing SHA-pin format, permissions block presence, decision-doc structure, plugin.json path safety, release workflow trigger discipline, and `git archive` requirement.
- Updated `README.md` with optional Claude-plugin install section + Examples section (M2 carry-over). README explicitly preserves "Rust installer remains canonical" wording.

## Design decisions and why

- **Green-lit decision for M4** based on the host-capability matrix:
  - Claude Code supports `.claude-plugin/plugin.json` (real friction reduction for org-installs).
  - GitHub Copilot has no equivalent format → it stays on `sldo-install` (no second-class treatment).
  - Both paths point at the same `skills/` tree (no source duplication).
- **Allow-list extension** for `.github/workflows/semgrep.yml` recorded inline in M4 lessons (rather than runbook Contract Block) because the discovery happened during execution: existing workflow had 2 unpinned `uses:` references and no `permissions:` block. Per M4 step-by-step note, "fix the existing pinning state first ... still M4 work, but call out as a discovery".
- **Decision doc as ALWAYS-shipped artifact** — even on the not-green-lit branch, the matrix doc would have shipped. M4's structural-contract test asserts the matrix exists with a decision row regardless of branch.
- **`git archive` mandate** for release-zip generation per F-SEC-5 — emits only tracked files at HEAD. `tar -czf .` and `cp -r .` patterns explicitly forbidden in the test.
- **`permissions: contents: write` on release workflow, `contents: read` on semgrep workflow** — minimum-privilege per F-SEC-4. Read-only for analysis workflows; write only what's needed (release artifact upload).

## Assumptions verified

- `gh api repos/<owner>/<repo>/branches/<ref>` returns the SHA for a branch tip; used for SHA-pin lookup of `dtolnay/rust-toolchain@stable`.
- `gh api repos/<owner>/<repo>/git/refs/tags/<tag>` returns SHA for a tag (for `Swatinem/rust-cache`).
- `softprops/action-gh-release` master SHA captured for the release workflow's upload step.
- The structural-contract test correctly identifies `runs:` as a composite-action marker (skipping permissions check on those — composite actions don't have permissions blocks).

## Assumptions still unresolved

- **`fail_on_unmatched_files: true`** — `softprops/action-gh-release` flag prevents silent zero-artifact uploads. Verified via the action's README; not exercised in this milestone (no actual release-tag pushed during M4).
- **`gh api` SHA freshness** — captured SHAs are point-in-time. The 12-month freshness mechanism for `references/security/standards-mapping.md` doesn't apply to workflow SHAs (different doc); a future runbook could add a workflow-SHA freshness check.

## Mistakes made

- **Initial release workflow used a fabricated SHA for `softprops/action-gh-release`** (`c95fe1...`). Looked up the real master SHA via `gh api` and replaced. Lesson: always verify SHAs via `gh api` before committing them — don't fabricate even in stub form.
- **Discovery delay on semgrep.yml issues** — the existing workflow's 2 unpinned `uses:` and missing `permissions:` block weren't surfaced until the structural-contract test ran. The runbook anticipated this case but I didn't audit the existing workflow as part of pre-flight.

## Root causes

- **SHA fabrication risk**: convenience > correctness when stubbing. Mitigation: always run the SHA-lookup command before pasting.
- **Pre-flight gap**: M4's pre-flight should have included "audit existing `.github/workflows/*.yml` against the SHA-pin invariant" as an explicit step. The structural-contract test caught the gap, but earlier audit would have surfaced it without a CI fail.

## What was harder than expected

- **Composite action detection** — `.github/actions/<name>/action.yml` files don't have `permissions:` blocks (they use `runs:` blocks instead). The structural-contract test now skips files containing top-level `runs:` (heuristic: `runs:` not preceded by `runs-on:`).
- **Trigger-acceptable-set string-matching** — the test uses substring checks (`tags:`, `release:`, `workflow_dispatch:`, `schedule:`) rather than YAML-parsed `on:` block inspection. False-positive risk: a workflow could have `tags: <something>` mentioned in a non-`on:` context. Acceptable because workflow YAML rarely uses those keys outside `on:`. A YAML-parsing version is a future micro-improvement.

## Invariants/assertions added or strengthened

- (a) Every `uses:` reference in `.github/{workflows,actions}/**/*.{yml,yaml}` matches `^[a-f0-9]{40}$`.
- (b) Every workflow has a top-level `permissions:` block (composite actions skipped).
- (c) `docs/slo/design/host-capability-matrix.md` exists with capability matrix + decision row.
- (d) `.claude-plugin/plugin.json` (if exists) contains no string with `..` segments, no absolute paths, no `.claude-plugin/skills/` duplication.
- (e) `release-zip.yml` triggers on at least one acceptable trigger {tags, release, workflow_dispatch, schedule} and never on `pull_request:`.
- (f) `release-zip.yml` body contains `git archive` and never `tar -czf .` / `cp -r .` / `zip -r . .`.

## Resource bounds established or verified

- 0 unpinned `uses:` references across all workflows.
- 0 workflow files lacking `permissions:` block (composite-action exclusion applied).
- 1 plugin.json (green-lit branch).
- 1 release workflow.

## Debugging / inspection notes

- `gh api repos/<owner>/<repo>/branches/<branch> --jq '.commit.sha'` returns the branch-tip SHA; alternative `git/refs/tags/<tag>` works for tag refs.
- The composite-action heuristic (`runs:` not `runs-on:`) was needed to prevent false positives on `.github/actions/<name>/action.yml`. Verified by `rg -n 'runs:' .github/` in the existing tree (no composite actions present yet, but the heuristic is future-proof).

## Naming conventions established

- `dist/` for build outputs (zip artifact lands here; `.gitignore` will need to include it).
- `sunlit-orchestra-${TAG}` as the release zip prefix (matches the project name + tag, idiomatic for `git archive --prefix`).

## Test patterns that worked well

- **Recursive directory walk for YAML files** — the test walks both `.github/workflows/` and `.github/actions/**` recursively, picking up future composite actions automatically.
- **Forbidden-pattern assertions** — `assert!(!content.contains(pat))` for each of the 4 forbidden zip-generation patterns. Cheap, comprehensive, forward-compatible.
- **Vacuous-pass for conditional artifacts** — plugin.json and release workflow tests return early if the file doesn't exist (deferred / not-green-lit branch). The test always runs in CI; branches don't need separate test infrastructure.

## Missing tests that should exist now

- **Full YAML parse of release-zip.yml** to assert the `on:` block has only acceptable triggers via structured YAML, not substring matching. Lane: `micro`. Defer.
- **Unzipped-content fixture test** — locally generate a release zip via `git archive`, unzip, assert content ⊆ `git ls-files`. Currently the test only structurally checks for `git archive` presence; doesn't actually run the workflow. Could be a later integration test.

## Rules for the next milestone

- **M5 reads the host-capability matrix** at pre-flight Step 3. Decision was green-lit-with-Copilot-fallback-documented — proceed with 4 agent files.
- **Output paths for agents** must canonicalize via `Path::components()` rejecting `..` per F-SEC-6. Same pattern as M4's plugin.json path traversal check.
- **`/slo-critique` SKILL.md SHA-256 baseline pin** must be captured at runbook-authoring-time + recorded as a const in `sap_imp_m5_agents.rs`. Use `sha256sum skills/slo-critique/SKILL.md`.

## Template improvements suggested

- **Pre-flight should include "audit existing artifacts against this milestone's invariants"** — the v4 template's Pre-flight section could add a step "verify existing artifacts that the structural-contract test will check (workflows, plugin manifests, agent files) and surface allow-list extensions BEFORE writing the test".
- **Allow-list extensions** could have a standard log location — currently they live inline in the Contract Block (M1) and the lessons file (M4). A consistent location helps future readers.
