# Lessons Learned — sap-imp Milestone 3

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M3 — Standards traceability matrix wired into security outputs
> **Date**: 2026-05-01

## What changed

- New `references/security/standards-mapping.md` with curated CWE × OWASP × ASVS × OpenCRE table (19 rows for the most common bug classes), per-output-type tier matrix, threshold rule, and Forbidden Shortcuts section.
- Updated 4 SKILL.md files (`/slo-critique`, `/slo-verify`, `/slo-sast`, `/slo-rulegen`) with citations to the new mapping file + threshold-rule phrasing.
- New structural-contract test `xtasks/sast-verify/tests/sap_imp_m3_standards.rs` with 7 test functions enforcing mapping presence, retrieval-date column, freshness warning, 4-skill citation, threshold-rule phrasing, live-artifact threshold-enforcement (vacuous-pass when empty), and fixture-based threshold check.
- 1 new dev-dep: `chrono` registered in `xtasks/sast-verify/Cargo.toml` `[dev-dependencies]`.

## Design decisions and why

- **Curated rows only, no bulk vendoring** — explicit Forbidden Shortcut in the mapping file's preamble. The 19 starting rows cover the common bug classes; future runbooks can add rows as needed.
- **Retrieval-date column on every row** — encoded as `YYYY-MM-DD` regex. Stale rows (> 12 months) emit warnings only, not failures, per design-doc note.
- **Threshold-rule structural-contract enforcement uses line-anchored markers** — to avoid false positives on narrative prose mentions of "severity: high". Markers: `### [HIGH]` / `### [CRITICAL]` headers, `| Severity | high |` table cells, `severity: high` line starts. Verified against the existing critique file (which mentions the rule in prose but is correctly excluded).
- **400-char window for threshold check** (vs. 200-char in M1's gate-phrase) — more generous because findings often have intervening evidence/scenario prose between severity and CWE.

## Assumptions verified

- All 4 target skills accepted the citation insertion without prose drift.
- `examples/security-finding.md` (M2 fixture) correctly carries CWE-639 within 400 chars of `[HIGH]` marker.
- `docs/slo/critique/secure-agent-playbook-imports.md` narrative mentions of "severity: high" / "severity: critical" do NOT trigger false-positive threshold violations (line-anchored marker exclusion works).

## Assumptions still unresolved

- **Active critique outputs from future runbooks** — the threshold-rule walk runs against `docs/slo/critique/*.md` AND `docs/slo/verify/*.md`. Current state: only `secure-agent-playbook-imports.md` exists; no high/critical actual findings (only narrative). Future critique passes will populate these dirs and the rule fires.
- **OpenCRE id stability** — I used `OpenCRE-NNN-NNN` format based on observed patterns from opencre.org snapshots; some rows have `n/a`. If OpenCRE changes their id scheme, the mapping table will need a refresh. The 12-month freshness warning surfaces this.

## Mistakes made

- **Initial test logic was too greedy** — `severity: high` matched narrative prose in the critique file, not just findings. Tightened to line-anchored markers (`lt.starts_with(...)`) to exclude narrative.

## Root causes

- Naive substring matching against unstructured Markdown is brittle. Real findings live in structured contexts (table rows, frontmatter, headers); narrative prose mentions are explanatory. Line-anchored matching is the discriminator.

## What was harder than expected

- **Choosing the right threshold-rule marker set**. Different SLO outputs use different conventions:
  - Critique tables: `severity: high` in cell
  - Security-finding template: `### [HIGH]` header + `| Severity | high |` row
  - Verify Pass 4: free-form severity in row
  
  Settled on a multi-marker approach (any of those line-anchored variants triggers the check). Future writers should prefer the `### [HIGH]` header for expanded findings.

## Invariants/assertions added or strengthened

- (a) `references/security/standards-mapping.md` exists with preamble containing per-output-type tier matrix and "no bulk vendoring" Forbidden Shortcut.
- (b) Every row in the mapping table has a `retrieval-date: YYYY-MM-DD`.
- (c) Stale rows (> 12 months) emit warnings (no failure).
- (d) 4 target skills cite `references/security/standards-mapping.md`.
- (e) `/slo-critique` and `/slo-verify` SKILL.md text contains "high" + "critical" + "MUST" + "CWE" within 200-char windows.
- (f) Live `docs/slo/critique/*.md` + `docs/slo/verify/*.md` walk: any high/critical structured marker has CWE within 400 chars.
- (g) `examples/security-finding.md` fixture follows the same rule.

## Resource bounds established or verified

- 19 starting rows in `standards-mapping.md`. No upper hard cap (curated growth).
- 4 target skills cited; hardcoded in `M3_TARGET_SKILLS` constant.
- 12-month freshness window encoded in `stale_rows_warned`.
- 400-char threshold-rule window.

## Debugging / inspection notes

- The threshold-rule false-positive surfaced via the test's verbose error message — the offending byte ~4530 in the critique file pointed to the F-ENG-4 description text. Reading the file at that position confirmed it was prose, not a finding. Tightening the marker pattern (line-anchored) fixed it without changing the rule's intent.

## Naming conventions established

- `standards-mapping.md` — singular noun + dash separator, matches sibling `security-finding-template.md` / `security-assessment-summary-template.md`.
- Test function names continue snake_case + descriptive (e.g., `live_critique_and_verify_findings_have_cwe`).

## Test patterns that worked well

- **Per-source dual enforcement** (live walk + fixture walk) — `live_critique_and_verify_findings_have_cwe` walks real artifacts (vacuous-pass when empty); `examples_high_severity_findings_have_cwe` walks the M2 fixture (always present). Together they ensure the rule fires whether or not a real critique has been authored yet.
- **Line-anchored marker matching** — using `line.trim_start().starts_with(...)` rather than substring `contains(...)` excludes narrative-prose false positives.

## Missing tests that should exist now

- **Test that asserts a NEW row added with malformed retrieval-date fails** — currently the test only walks real rows. Lane: `micro`.
- **Test that an OpenCRE id update doesn't break the parse** — current test only checks retrieval-date format, not OpenCRE column shape. Lane: `micro`.

## Rules for the next milestone

- **M4 will need similar line-anchored matching for SHA-pin assertions** in workflow files. Avoid `contains("@v")` — use line-anchored or YAML-parsed checks.
- **M4's plugin.json path traversal check** needs `Path::components()` canonicalization — same pattern as M5's `output-paths` check.

## Template improvements suggested

- The v4 template's "Standards mapping" row in the security-finding-template should include a column for `retrieval-date` matching this curated table's format. Currently it's free-form.
