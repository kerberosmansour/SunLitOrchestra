# Completion Summary — sap-imp Milestone 1

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M1 — Shared security-reporting integration extended to `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship`
> **Started**: 2026-05-01
> **Completed**: 2026-05-01

## Goal completed

Every security-relevant skill that emits findings or coverage gaps now cites the shared `references/security/security-finding-template.md` or `references/security/security-assessment-summary-template.md` for expanded reporting, AND a structural-contract test enforces the citation invariant via `pulldown-cmark` AST-based parsing (per F-ENG-1 critique resolution).

## Files changed

- `Cargo.toml` (root) — added `pulldown-cmark = { version = "0.10", default-features = false }` to `[workspace.dependencies]`.
- `xtasks/sast-verify/Cargo.toml` — added `pulldown-cmark = { workspace = true }` to `[dev-dependencies]`.
- `skills/slo-sast/SKILL.md` — added "Coverage-gap reporting" subsection with assessment-summary + finding template citations.
- `skills/slo-rulegen/SKILL.md` — added "Reporting suspect rules" subsection with finding template citation.
- `skills/slo-ruleverify/SKILL.md` — added "Expanded failure findings" subsection with finding template citation.
- `skills/slo-ship/SKILL.md` — added "Optional security-summary section (gated)" subsection with assessment-summary template citation, gated on "introduced new public surface".
- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated; M1 Contract Block updated with allow-list extension; Evidence Log filled.

## Tests added

- `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` (NEW) with 5 test functions:
  - `every_security_skill_cites_a_template`
  - `cited_template_paths_resolve`
  - `slo_ship_security_summary_is_gated`
  - `no_skill_links_to_examples` (anticipates M2's F-ENG-3)
  - `ast_parser_excludes_code_block_content`

## Runtime validations added

- The 5-test structural-contract test family runs against the actual `skills/<name>/SKILL.md` files at HEAD (not fixtures) and the actual canonical template paths at HEAD.
- E2E command: `cargo test -p sast-verify --test sap_imp_m1_citations` — green: `5 passed; 0 failed`.

## Static analysis and formatter evidence

- `cargo fmt --all` — applied; clean.
- `cargo build --workspace` — builds (with pre-existing unused-field warnings in `sast-verify/src/{tier_detect,yaml_schema}.rs` that pre-date M1).
- `cargo clippy -p sast-verify --test sap_imp_m1_citations -- -D warnings` (M1-scoped) — clean.
- Workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` has pre-existing dead-code errors out of M1's scope; recorded in lessons file.

## Compatibility checks performed

All 10 Compatibility Checklist rows verified:
- All 4 SKILL.md install paths unchanged.
- Frontmatter `name` and `description` unchanged for all four edited skills.
- `references/security/security-{finding,assessment-summary}-template.md` table-row shapes unchanged.
- `cargo test -p sldo-install` — green.
- `sldo-install --dry-run` — resolves all 32 shipped skills.
- `/slo-critique` and `/slo-verify` SKILL.md files untouched.

## Invariants/assertions added

5 invariants encoded in the structural-contract test:
1. ≥1 canonical-template citation per skill in the 6-skill citing set.
2. Every cited canonical-template path resolves at HEAD.
3. `/slo-ship` gate phrase ("introduced new public surface") within 200 chars of the assessment-summary citation.
4. No shipped SKILL.md links to `examples/` (anticipates M2 F-ENG-3).
5. Citation counting uses `pulldown-cmark` AST (excludes code-fence content).

## Resource bounds added or verified

- 6-skill citing set hardcoded in `CITING_SKILL_SET` constant. Adding a 7th requires constant edit + runbook amendment.
- ≥1 citation floor per skill (no upper cap; multi-citations allowed).
- 200-char window for `/slo-ship` gate-phrase check.

## Documentation updated

- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker + Evidence Log filled + Contract Block allow-list extension recorded.
- `docs/ARCHITECTURE.md` — pending one-line note about `xtasks/sast-verify/tests/sap_imp_m*` family (post-flight task; deferred until all 5 milestones close to avoid churn).

## .gitignore changes

None — M1 produced no new generated files.

## Test artifact cleanup verified

`git status` shows working tree clean after all M1 work was committed during execution by the user. No untracked test artifacts.

## Deferred follow-ups

- **`docs/ARCHITECTURE.md` "Test Architecture" note** — the runbook's Documentation Update Table says M1 should add one line about the `sap_imp_m*` test family. Deferring to runbook close-out so the note can describe the full family (M1 + M2 + M3 + M4 + M5 tests) in one edit.
- **Fixture-based test for the test itself** — currently the test only exercises real shipped SKILL.md files. A future engineering-skill-improvements milestone could add `tests/fixtures/` with synthetic SKILL.md files for testing the failure paths in isolation. Lane: `micro`.
- **Workspace-wide clippy baseline cleanup** — pre-existing dead-code errors in `sast-verify/src/{tier_detect,yaml_schema}.rs` block `cargo clippy --workspace --all-targets -- -D warnings`. Out of scope for M1; suggest a separate small runbook or fold into the engineering-skill-improvements runbook. Lane: `micro`.

## Known non-blocking limitations

- **Reference-style Markdown links** are not exercised by the test fixture suite (only inline `[text](url)` form is). pulldown-cmark resolves both to `Tag::Link` events, so the invariant should hold; if a future skill adopts reference-style exclusively, M3 or later should add a fixture.
- **Alias/symlink path matching** — the test uses `ends_with` / `contains` for canonical-template path matching, which would accept a hypothetical alias path. F-ENG-1's "no alias paths" rule is documented in the runbook prose but not yet structurally enforced.
