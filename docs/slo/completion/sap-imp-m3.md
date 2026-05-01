# Completion Summary — sap-imp Milestone 3

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M3 — Standards traceability matrix wired into security outputs
> **Started**: 2026-05-01
> **Completed**: 2026-05-01

## Goal completed

Every security-relevant SLO output now has a clear required-vs-optional standards mapping. A curated `references/security/standards-mapping.md` carries 19 CWE × OWASP × ASVS × OpenCRE rows with retrieval dates; 4 target skills cite it; the high/critical mandatory-CWE threshold rule is structurally enforced against both live critique/verify directories and the M2 fixture.

## Files changed

- `references/security/standards-mapping.md` (NEW) — curated table + tier matrix + Forbidden Shortcuts.
- `skills/slo-critique/SKILL.md` — added standards-mapping citation + threshold-rule paragraph.
- `skills/slo-verify/SKILL.md` — added standards-mapping citation + tier-matrix note + threshold-rule.
- `skills/slo-sast/SKILL.md` — added standards-mapping citation in coverage-gap section.
- `skills/slo-rulegen/SKILL.md` — added standards-mapping citation in suspect-rules section.
- `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` (NEW) — 7-test structural-contract test.
- `xtasks/sast-verify/Cargo.toml` — added `chrono` to `[dev-dependencies]`.
- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated.

## Tests added

- `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` (NEW) with 7 test functions:
  - `standards_mapping_file_exists_with_preamble`
  - `every_row_has_retrieval_date`
  - `stale_rows_warned`
  - `four_skills_cite_standards_mapping`
  - `threshold_rule_phrasing_in_critique_and_verify`
  - `live_critique_and_verify_findings_have_cwe` (per F-ENG-4)
  - `examples_high_severity_findings_have_cwe` (fixture leg)

## Runtime validations added

- Test command: `cargo test -p sast-verify --test sap_imp_m3_standards` — green: `7 passed; 0 failed`.
- Tests run against actual files at HEAD: 4 SKILL.md files, the new mapping file, the M2 fixture, and live `docs/slo/critique/*.md` / `docs/slo/verify/*.md` (vacuous-pass when empty of structured high/critical findings).

## Static analysis and formatter evidence

- `cargo fmt --all` — clean.
- `cargo build --workspace` — clean.
- `cargo test -p sast-verify --tests` — `41 passed; 0 failed` across all sap-imp tests + gate_e2e.
- `cargo test -p sldo-common -p sldo-install -p sldo-research` (runbook baseline) — green.

## Compatibility checks performed

- M1's `sap_imp_m1_citations` test still passes.
- M2's `sap_imp_m2_examples` test still passes.
- `references/security/security-{finding,assessment-summary}-template.md` shapes unchanged.
- All shipped `skills/<name>/SKILL.md` install paths and frontmatter unchanged.
- `sldo-install --dry-run` resolves all skills.

## Invariants/assertions added

7 invariants encoded:
1. Mapping file exists with required preamble sections.
2. Every CWE row has retrieval-date in YYYY-MM-DD format.
3. Stale rows (> 12 months) emit warnings (informational, not failure).
4. 4 target skills cite the mapping.
5. `/slo-critique` and `/slo-verify` document the threshold rule with required phrases within 200-char windows.
6. Live `docs/slo/{critique,verify}/*.md` high/critical structured findings have CWE within 400-char window.
7. M2's `examples/security-finding.md` fixture follows the rule.

## Resource bounds added or verified

- 19 starting CWE rows in mapping (no upper cap; curated growth).
- 12-month freshness window before stale warning.
- 4 target skills (hardcoded constant).
- 400-char threshold-rule window.

## Documentation updated

- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated.
- `docs/ARCHITECTURE.md` "References subtrees" note — pending (deferred to runbook close-out).

## .gitignore changes

None — no new generated files.

## Test artifact cleanup verified

`git status` clean after M3 work (assuming user commits incrementally as before).

## Deferred follow-ups

- **ARCHITECTURE.md note** about `references/security/standards-mapping.md` — deferred to close-out.
- **OpenCRE id schema verification** — current OpenCRE columns use observed patterns; some are `n/a`. Future maintenance: re-fetch quarterly and update retrieval-dates. Lane: `micro` (recurring).
- **Test for the test** (malformed retrieval-date fixture, OpenCRE column shape change). Lane: `micro`.

## Known non-blocking limitations

- **Threshold-rule marker set** is fixed at 6 line-anchored variants. Future critique writers using different conventions (e.g., HTML-style `<strong>HIGH</strong>` headers) wouldn't be caught. Mitigation: prefer the `### [HIGH]` convention.
- **OpenCRE column not validated** beyond presence — a malformed `OpenCRE-NNN` value is not caught. M3's contract is that retrieval-date is the freshness audit trail; OpenCRE accuracy is reviewer responsibility.
