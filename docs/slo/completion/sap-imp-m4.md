# Completion Summary — sap-imp Milestone 4

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M4 — Optional Claude plugin packaging assessment + (green-lit) packaging artifacts
> **Decision**: GREEN-LIT — both plugin.json and release-zip workflow ship.
> **Started**: 2026-05-01
> **Completed**: 2026-05-01

## Goal completed

A host-capability matrix decision doc names what each host (Claude Code, GitHub Copilot) supports for plugin install, agent install, and runtime invocation. M4's decision is **green-lit**: a `.claude-plugin/plugin.json` and a SHA-pinned release-zip workflow ship as additive distribution channels. `sldo-install` remains canonical; GitHub Copilot is not made second-class.

## Files changed

- `docs/slo/design/host-capability-matrix.md` (NEW) — capability matrix + green-lit decisions for both M4 and M5.
- `.claude-plugin/plugin.json` (NEW) — minimal manifest pointing at `skills/`.
- `.github/workflows/release-zip.yml` (NEW) — SHA-pinned release-zip workflow, tag-triggered, `git archive` HEAD-only, `permissions: contents: write`.
- `.github/workflows/semgrep.yml` — SHA-pinned `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2`; added `permissions: contents: read`.
- `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` (NEW) — 6-test structural-contract test.
- `README.md` — "Install via Claude plugin (optional, additive)" section + Examples section (M2 carry-over). Canonical-installer wording preserved.
- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated to `done (green-lit)`.

## Tests added

`xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` (NEW) with 6 test functions:

- `every_workflow_uses_is_sha_pinned`
- `every_workflow_has_explicit_permissions_block`
- `host_capability_matrix_exists_with_decision`
- `plugin_json_does_not_duplicate_skills_or_traverse_paths`
- `release_workflow_trigger_in_acceptable_set`
- `release_workflow_uses_git_archive`

## Runtime validations added

- Test command: `cargo test -p sast-verify --test sap_imp_m4_workflow_pinning` — green: `6 passed; 0 failed`.
- Tests run against actual `.github/workflows/*.yml` and `.claude-plugin/plugin.json` and `docs/slo/design/host-capability-matrix.md` at HEAD.

## Static analysis and formatter evidence

- `cargo fmt --all` — clean.
- `cargo build --workspace` — clean.
- `cargo test -p sast-verify --tests` — `27 passed; 0 failed` (gate_e2e + M1 + M2 + M3 + M4 = 41 with baseline 14).
- `cargo test -p sldo-common -p sldo-install -p sldo-research` (runbook baseline) — green.

## Compatibility checks performed

- All M1 + M2 + M3 structural-contract tests still pass.
- Existing `.github/workflows/semgrep.yml` after SHA-pin update still parses; pinning + permissions invariants now hold.
- `sldo-install` CLI surface unchanged; `~/.sldo/install.toml` schema unchanged; install paths unchanged.
- README wording explicitly preserves "Rust installer remains canonical" — no downgrade.
- `docs/skill-pack-catalog.md` unchanged (canonical inventory).

## Invariants/assertions added

6 invariants encoded:

1. SHA-pin format on every `uses:` across all workflow + composite-action YAML files.
2. Every workflow has top-level `permissions:` block (composite actions skipped via `runs:` heuristic).
3. Host capability matrix exists with green-lit / not-green-lit / deferred decision row.
4. Plugin.json contains no path traversal segments, no absolute paths, no `.claude-plugin/skills/` duplication.
5. Release workflow triggers from acceptable set {tags, release, workflow_dispatch, schedule}; forbidden `pull_request:` rejected.
6. Release workflow uses `git archive` (no `tar -czf .` / `cp -r .` / `zip -r . .`).

## Resource bounds added or verified

- 0 unpinned `uses:` references across all workflows (was 2 before M4).
- 0 workflows lacking `permissions:` block (was 1 before M4).
- 1 plugin.json (green-lit branch).
- 1 release workflow.
- Release zip emits only `git ls-files HEAD` content (no leakage from runner working dir).

## Documentation updated

- `docs/slo/design/host-capability-matrix.md` — NEW.
- `README.md` — Install-via-plugin + Examples sections.
- `docs/ARCHITECTURE.md` "Distribution channels" subsection — pending close-out.

## .gitignore changes

`dist/` will be created by the release workflow at runtime; should be added to `.gitignore` to prevent accidental local-artifact commits. **Pending**: deferred to runbook close-out for one consolidated `.gitignore` edit covering M2 (none) + M4 (dist/, *.zip).

## Test artifact cleanup verified

`git status` clean after M4 work.

## Deferred follow-ups

- **`.gitignore` `dist/` + `*.zip` patterns** — close-out task.
- **`docs/ARCHITECTURE.md` "Distribution channels" subsection** — close-out task.
- **YAML-parse-based `on:` block inspection** instead of substring matching for trigger-acceptable-set. Lane: `micro`.
- **Local dry-run of `gh release create --draft`** to actually exercise the release workflow end-to-end. Not exercised in M4. Lane: `micro`. Add to a future quarterly maintenance check.
- **softprops/action-gh-release SHA freshness** — captured 2026-05-01. Re-pin on tag movement; document in a workflow-SHA freshness ref. Lane: `micro` (recurring).

## Known non-blocking limitations

- **Composite action permissions skip is heuristic** (`runs:` not `runs-on:`). If a future workflow uses `runs:` in an unusual context, the heuristic could falsely skip the permissions check. Mitigation: convention is to keep composite actions in `.github/actions/<name>/action.yml`; the directory-walk pattern naturally separates them.
- **No structural validation of plugin.json schema fields** beyond path safety. The minimum manifest is intentionally narrow (name, version, description, author, license, skills_dir); future additions to the schema should preserve path-safety properties.
