# Completion Summary — scanner-orch Milestone 3

## Goal completed

The workflow safety contract is locked via a static template at `references/sast/scanner-orch-workflow-template.yml` (with `{{CHECKOUT_SHA}}` / `{{UPLOAD_SARIF_SHA}}` placeholders pending first real bump). The action SHAs reference doc enforces 40-char-SHA-only on both required third-party actions (`actions/checkout`, `github/codeql-action/upload-sarif`). 20 structural-contract tests assert every documented safety property (no `pull_request_target`; `permissions: {}` at workflow scope; per-job permissions minimal; SHA-pinned actions; `fetch-depth: 0`; `SEMGREP_RULES` env var; no `secrets.*`; no `--autofix`; no `--config`). SKILL.md's Emission section documents the symlink-traversal defense (SEC-1) and CWE-list independence of the workflow YAML (SEC-4 implication).

## Files changed

- `references/sast/scanner-orch-workflow-template.yml` — NEW (~70 lines, static template + rationale comments)
- `references/sast/scanner-orch-action-shas.md` — NEW (~60 lines, pinned-SHA + bump procedure)
- `skills/slo-sast/SKILL.md` — extended (Method (M3) section added; M1 + M2 sections unchanged)
- `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` — NEW (~270 lines, 20 tests)
- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — modified (M3 asks ENG-1/SEC-1/SEC-4 applied; Milestone Tracker M3 → done)
- `docs/lessons/scanner-orch-m3.md` — NEW
- `docs/completion/scanner-orch-m3.md` — NEW (this file)

## Tests added

20 structural-contract tests in `e2e_scanner_orch_m3.rs`:
- Workflow-template safety properties (9 individual tests, one per asserted property)
- Action-SHAs reference doc (5 tests: existence, both required actions listed, SHA enforcement, refresh cadence, placeholder state)
- SKILL.md M3 documentation (4 tests: emission method documented, safety contract documented, symlink defense documented, CWE-list independence documented)
- Prior-milestone regression (2 tests: M1+M2 sections still present, references/sast/ existing files unmodified)

## Runtime validations added

- N/A at the auto-running-test layer.
- Real-Semgrep dry-run smoke step (ENG-1) added to the runbook's smoke test sequence; deferred to `/slo-verify` runtime validation.

## Compatibility checks performed

- M1 + M2 + M3 E2E suites all green (21 + 22 + 20 = 63 tests passing).
- `cargo check --workspace` green.
- `references/sast/` existing files (M1 parser-contract, M2 pinned-rules-sha, M2 stack-detection-contract, sast-rulegen pre-existing) byte-identical (asserted by `existing_references_sast_unmodified_by_m3`).

## Documentation updated

- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — Milestone Tracker row 3 → `done`.

## .gitignore changes

None.

## Test artifact cleanup verified

`git status` shows only intended new files. No untracked test outputs.

## Deferred follow-ups

- **Real action-SHA bump.** All-zero placeholders are intentional; first real bump-PR happens at M5 dogfood prep alongside the rules-SHA bump.
- **Runtime workflow YAML validity test (`actionlint`).** Defer to `/slo-verify`.
- **Real-Semgrep dry-run smoke.** Documented in runbook smoke tests; deferred to `/slo-verify`.

## Known non-blocking limitations

- **Action SHAs are placeholders.** The emitted workflow won't run successfully in real CI until the bump happens. Acceptable for the M3 milestone (which lands the contract and tests).
- **Structural-contract tests assert template properties, not emission properties.** Same caveat as M1/M2; the architectural defense (skill emits template verbatim with only SHA substitution) is what bridges template-correctness to emission-correctness.
