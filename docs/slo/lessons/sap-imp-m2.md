# Lessons Learned — sap-imp Milestone 2

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M2 — Example output gallery under `examples/`
> **Date**: 2026-05-01

## What changed

- New `examples/` directory at repo root with 7 files: README + 6 abbreviated synthetic artifacts (runbook excerpt, critique report, verification report, security finding, SAST manifest, biz-public artifact).
- New structural-contract test `xtasks/sast-verify/tests/sap_imp_m2_examples.rs` with 6 test functions enforcing file count, frontmatter, JSON synthetic flag, PII regex (5 patterns), size cap, and `abbreviates:` resolution.
- 2 new dev-deps (already in `[workspace.dependencies]`): `regex`, `serde_yaml_ng` registered as `[dev-dependencies]` in `xtasks/sast-verify/Cargo.toml`.

## Design decisions and why

- **PII regex set: email + UK NI + UK sort code + US SSN + EU IBAN** per F-SEC-2 critique resolution. Email regex is permissive on purpose (any email is a leak risk in synthetic content). EU IBAN regex requires ≥ 15 chars to suppress false-positives on uppercase 4-letter sequences followed by digits.
- **`abbreviates:` resolution: skill-name walk OR filesystem path** per F-ENG-2 resolution. Skill-name walk lets `abbreviates: slo-critique` resolve via `skills/slo-critique/SKILL.md`; filesystem path lets `abbreviates: docs/slo/templates/runbook-template_v_4_template.md` resolve via `Path::exists()`. Both are accepted; either succeeds.
- **No `pii_scan_override:` mechanism in M2** — strict scan now per Out-of-Scope rule. A future runbook can introduce an override only when a legitimate need arises.
- **Hardcoded `EXPECTED_FILES` list** rather than dynamic discovery — encodes the M2 invariant that the gallery has *exactly* 7 specific files. Adding an 8th requires editing the constant + a runbook amendment.

## Assumptions verified

- All 6 PII regex patterns yield zero matches against the synthetic content.
- All 6 Markdown files (excluding README) parse as valid YAML frontmatter with required fields.
- `examples/sast-manifest.json` is valid JSON with `"synthetic": true` at top level.
- All 7 files are well under the 10 KB cap (largest is `security-finding.md` at 2.9 KB).

## Assumptions still unresolved

- **Reference-style Markdown links inside examples** are not exercised by the test fixture. Unlikely to matter — the M1 `no_skill_links_to_examples` test still walks shipped skills, not examples.
- **Hidden file detection** — current implementation skips entries starting with `.` (e.g., `.DS_Store` on macOS). If a file is named `.gitkeep` it would be skipped. None of these matter for M2's invariants.

## Mistakes made

- None during M2 itself. The TDD-first discipline meant the test was authored before any artifact, and every failure was an expected-failure mode.

## Root causes

- N/A.

## What was harder than expected

- **Synthetic but realistic content**. Each artifact had to look like a real SLO output (so it's calibrating) WITHOUT containing real PII (so it passes the scan). The constraint forced compact, archetypal prose. The widget-index narrative thread (used across runbook excerpt, critique, verification, security finding) provided continuity without pulling from real customer cases.

## Invariants/assertions added or strengthened

- (a) `examples/` contains exactly 7 files, matching `EXPECTED_FILES` set.
- (b) Every Markdown example (5 of 6 artifacts) carries `synthetic: true`, `non-normative: true`, and non-empty `abbreviates:` in YAML frontmatter.
- (c) `examples/sast-manifest.json` carries `"synthetic": true` at top level.
- (d) PII regex scan over `examples/**/*.{md,json}` finds zero matches across 5 locale-specific patterns.
- (e) Every `abbreviates:` value resolves via skill-name lookup OR filesystem path.
- (f) Every file in `examples/` is ≤ 10 KB.

## Resource bounds established or verified

- 7 files cap, encoded in `EXPECTED_FILE_COUNT` and `EXPECTED_FILES`.
- 10 KB per file, encoded in `SIZE_CAP_BYTES`.
- 5 PII regex patterns, encoded in `pii_regexes()`.

## Debugging / inspection notes

- The EU IBAN regex `[A-Z]{2}\d{2}[A-Z0-9]{1,30}` is permissive — it would match `CWE89A1` or `OK20BCDEFGH` (random uppercase prefix + digits). Suppression: only count matches ≥ 15 chars (an actual IBAN's minimum length range). Verified by running the test and seeing zero false-positives across the 7-file gallery.
- US SSN regex with `\b...\b` boundary correctly excludes things like `127.0.0.1-port-1234` that would otherwise look like SSN.

## Naming conventions established

- Test file: `xtasks/sast-verify/tests/sap_imp_m<N>_<feature>.rs` — same as M1.
- Test functions: descriptive snake_case, no `test_` prefix.

## Test patterns that worked well

- **YAML frontmatter parsing via `serde_yaml_ng::Value` + map-key probing** — generic enough to validate any frontmatter shape, robust to extra fields.
- **Per-finding error-collection pattern** — each test gathers `failures: Vec<String>` then asserts `failures.is_empty()` at the end. Lets one test surface ALL violations in one run, not just the first.

## Missing tests that should exist now

- **Test that the test correctly catches PII** — currently only the happy path is exercised (zero matches). A fixture-based test that injects each of the 5 PII patterns and asserts the test fails would prove the scan works. Lane: `micro`. Defer to engineering-skill-improvements runbook.

## Rules for the next milestone

- **M3 will need to walk `docs/slo/critique/*.md` and `docs/slo/verify/*.md` for the threshold-rule check** (per F-ENG-4 resolution) — vacuous-pass when those directories are empty. Pattern is already established in M1's `no_skill_links_to_examples` walking the `skills/` directory.
- **M3 should reuse the YAML frontmatter parsing helper** (`extract_frontmatter` + `serde_yaml_ng::from_str`) — could be promoted to a shared module if it appears in M3 and M5.

## Template improvements suggested

- The v4 template's BDD table doesn't explicitly call out "vacuous-pass" cases. M3, M4, M5 all have vacuous-pass branches; the template could include "vacuous-pass when X" as an explicit BDD category.
