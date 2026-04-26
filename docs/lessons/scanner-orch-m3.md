# Lessons Learned — scanner-orch Milestone 3

## What changed

- New file `references/sast/scanner-orch-workflow-template.yml` — the static safe-template skeleton with `{{CHECKOUT_SHA}}` and `{{UPLOAD_SARIF_SHA}}` placeholders only, plus per-line rationale comments documenting each safety property.
- New reference `references/sast/scanner-orch-action-shas.md` — pinned 40-char SHAs for the two required actions (`actions/checkout`, `github/codeql-action/upload-sarif`), currently the all-zero placeholder pending first real bump-PR; bump procedure + 90-day refresh cadence documented.
- Extended `skills/slo-sast/SKILL.md` — added "Method (M3 — emit ...)" section with emission flow, workflow safety contract, CWE-list independence statement, M3-specific anti-patterns. SEC-1 symlink-traversal defense documented at the emission step.
- New test `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` — 20 structural-contract tests asserting workflow-template safety properties (each property as a separate test for clear failure surfacing) + action-SHAs reference doc + SKILL.md M3 additions + prior-milestone regression.
- Asks applied: ENG-1 (real-Semgrep dry-run smoke step in runbook), SEC-1 (symlink-traversal BDD scenarios + step-by-step), SEC-4 (extend byte-compare to `.semgrep.yml`).

## Design decisions and why

- **Workflow-template-as-frozen-fixture.** The runbook's M2 lessons-file rule was: "M3's structural-contract test asserts the workflow TEMPLATE satisfies all safety properties (because the skill emits it verbatim with only SHA substitution). Template-as-frozen-fixture is the cleanest approach." Followed verbatim. The 9 individual safety-property tests parse the static template at `references/sast/scanner-orch-workflow-template.yml` and assert each property in isolation — clearer failure messages than a single composite assertion.
- **Per-property-per-test.** Each safety property gets its own `#[test]` function. When a test fails, the failure message names the violated property exactly. This is more verbose than a single assertion but makes M3's safety claim auditable line-by-line.
- **Comment-tolerance in safety assertions.** Several tests filter out lines starting with `#` before asserting (e.g., `pull_request_target` may appear in a comment explaining why it's banned without invalidating the test). The failure message includes only non-comment offenders.

## Mistakes made

- Initially considered asserting on emitted runtime YAML (post-skill-invocation), but realized the structural-contract pattern doesn't have access to runtime emission. Pivoted to template-as-fixture per the M2 lessons-file rule.

## Root causes

- Same as M1 + M2 — the structural-contract test pattern verifies documentation correctness, not runtime correctness. M3's workflow safety claim is uniquely well-suited to this pattern because the workflow YAML IS the static template.

## What was harder than expected

- Nothing surprising. The template-as-fixture pattern made M3 feel almost mechanical compared to M2 (which had to author both the contract and the schema across 2 reference docs).

## Naming conventions established

- Workflow templates: `references/sast/scanner-orch-<name>-template.yml` (`yml` extension because GitHub Actions uses `.yml` not `.yaml`).
- Pinned-SHA reference docs: one per scope (rules SHA, action SHAs); not bundled to keep bump-PR diffs focused.
- Per-safety-property tests: `workflow_template_<property-name>` (e.g., `workflow_template_uses_pull_request_not_pull_request_target`).

## Test patterns that worked well

- **Comment-tolerant assertions.** `lines().filter(|l| !l.trim_start().starts_with('#'))` lets the template have explanatory comments mentioning forbidden constructs without invalidating the safety check.
- **Forbidden-pattern arrays** with non-comment filtering — declarative way to enforce "this string MUST NOT appear in any non-comment line."
- **`workflow-scope vs job-scope position check`** — using `find()` indices to assert structural ordering (e.g., `permissions: {}` appears before `jobs:` to confirm it's at workflow scope).

## Missing tests that should exist now

- **Runtime YAML validity test.** The structural-contract tests assert content properties but don't run a YAML parser over the template (GitHub Actions YAML parsers can be picky). Defer to smoke testing or add `serde_yaml_ng::from_str` parse assertion in a future hardening.
- **`actionlint` integration.** A real-world workflow safety check would run `actionlint` (the GitHub Actions linter) against the rendered workflow. Defer to smoke + `/slo-verify`.
- **Real-Semgrep dry-run.** ENG-1 added this to smoke tests; not exercised at the auto-running-test layer (would require Semgrep in CI). Defer to `/slo-verify`.

## Rules for the next milestone

- **M4 must extend SKILL.md with Manifest + Preview-Mode sections.** Same incremental-extend pattern.
- **M4's manifest schema test** can use the same template-as-fixture approach — author a golden manifest fixture under `references/sast/` and assert the schema doc matches it.
- **Symlink-traversal defense extends to M4 manifest writes.** Per SEC-1, every file write site requires the symlink-rejection check. Add to M4 BDD.
- **`cargo test -p sldo-install --test e2e_scanner_orch_m<N>`** continues to be the iteration loop. `cargo check --workspace` for compile validation. Reserve full `cargo test --workspace` for `/slo-verify`.

## Template improvements suggested

- The runbook v3 template's "Files Allowed To Change" could include a column for "stability marker" — `stable` for files whose contract is locked, `evolving` for files whose contents may shift. M3's template + action-SHAs files are paradigmatic — `stable` for safety properties, `evolving` for SHA values.
- Consider adding a "Frozen-fixture pattern" note to the BDD section. M3's structural-contract test approach (parse a static fixture, assert properties one-at-a-time) is broadly applicable to skill-pack milestones that emit deterministic artifacts.
