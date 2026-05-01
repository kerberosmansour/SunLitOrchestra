# Lessons Learned — scanner-orch Milestone 4

## What changed

- New reference `references/sast/scanner-orch-manifest-schema.md` — full schema v1.0 documentation; field-by-field table; validation rules (regex-validated values, set-algebra invariant); explicit "defensive design, not regulatory mandate" framing; PCI 6.2.3 (v4.0.1) citation discipline.
- Extended `skills/slo-sast/SKILL.md` — Method (M4) section with manifest emission + preview-mode UX (first install vs re-derivation vs mixed-pre-existing-state); rollback contract; symlink-defense at manifest write site; M4-specific anti-patterns (no silent commit, no schema field omission, no overpromising language).
- New test `crates/sldo-install/tests/e2e_scanner_orch_m4.rs` — 17 structural-contract tests asserting schema-doc completeness + framing discipline + SKILL.md M4 additions + prior-milestone regression.
- Asks applied: ENG-3 (mixed-state preview-mode trigger), SEC-1 variant (symlink defense at manifest writes).

## Design decisions and why

- **"Defensive design" framing as the load-bearing claim.** The structural-contract test asserts the schema doc explicitly disclaims regulatory framing. This is the audit-defense narrative: the manifest is internally useful but NOT framed as audit evidence, because no published QSA postmortem fixes the mapped-but-not-scanned pattern as a documented audit-failure. Every user-facing prose follows this rule.
- **Mixed-pre-existing-state trigger expands ENG-3.** The runbook BDD added two new abuse scenarios (`preview_mode_triggered_by_pre_existing_workflow`, `preview_mode_triggered_by_pre_existing_semgrep_config`). These distinguish "first install on a clean repo" from "first install on a repo with prior config" — both go through preview-mode, but with different diff context.
- **Schema-doc test for the set-algebra rule.** Asserts `cwes_uncovered = cwes_claimed \ cwes_actually_covered` is documented as a construction rule (not computed independently). Catches future schema-evolution bugs that might re-derive `cwes_uncovered` from a different basis.

## Mistakes made

- Initial `manifest_schema_uses_defensive_design_framing` test had a too-strict regex that failed on the literal wording "not regulatory mandate" — relaxed to a broader contains-check that allows variants.
- The `manifest_schema_avoids_overpromising_strings` test had to allow "❌" prefixed lines (the prohibition list) and the explicit "not regulatory" disclaimer line. Required a multi-condition filter.

## Root causes

- Same as M3 — content-keyword tests are sensitive to phrasing variations. Defaulting to `to_lowercase().contains(...)` for forgiving matches works for most intent-asserting tests; strict equality only when the exact text is the contract.

## What was harder than expected

- Threading the "defensive design" framing through both the schema doc AND the SKILL.md AND the structural-contract tests. Three places that all need to say the same thing in different shapes — easy to drift.

## Naming conventions established

- Schema-version-as-string-literal in the `schema_version` field (e.g., `"1.0"` not `1.0` numeric). Makes schema migration explicit at the JSON layer.
- Test-function naming for framing claims: `<artifact>_<framing-property>` (e.g., `manifest_schema_uses_defensive_design_framing`, `manifest_schema_pci_citation_correct`).

## Test patterns that worked well

- **Section-bounded assertions** (`let m4_section_start = skill.find("Method (M4")`). Lets a test check that a property holds within a specific milestone's section, not anywhere in the SKILL.md. Catches "M4-specific" requirements without being fooled by M2's stale references.
- **Allowed-context filtering** for forbidden-string tests. The "❌"-prefix exclusion lets a doc explicitly enumerate forbidden phrases without tripping the test.

## Missing tests that should exist now

- **Schema-version migration test.** No test asserts that `schema_version: "1.0"` is the only currently-emitted value. A future M4.5 milestone bumping to v1.1 would benefit from a parallel schema-version test that catches accidental v1.1-only fields landing in the v1.0 doc.
- **Runtime preview-mode user-decline rollback test.** Documented in SKILL.md but not exercised at the auto-running-test layer. Defer to `/slo-verify`.

## Rules for the next milestone

- **M5 must extend SKILL.md with Re-Derivation + PR Creation sections.** Same incremental-extend pattern.
- **M5 introduces `gh pr create` invocation discipline.** All forbidden flags (`--auto`, `--squash`, `--rebase`, `--merge`, `--repo`) MUST be enumerated in M5's anti-patterns AND asserted by the structural-contract test.
- **Symlink defense extends to M5's PR-body file (if any).** Each new write site requires the check.
- **Defensive-design framing applies to M5's PR body too.** The PR body describes drift; do not frame as audit evidence.

## Template improvements suggested

- The runbook v3 template's BDD table should have a "framing" tag for findings that require specific user-facing language. M4's "defensive design, not regulatory mandate" is paradigmatic — it's not a code property, it's a narrative property, and it's load-bearing.
- The Self-Review Gate should add: "Did I check that user-facing prose avoids overpromising frames? (e.g., audit-required, regulatory-mandate, PCI-compliant)."
