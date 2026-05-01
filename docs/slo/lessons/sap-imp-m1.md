# Lessons Learned — sap-imp Milestone 1

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M1 — Shared security-reporting integration extended to `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-ship`
> **Date**: 2026-05-01

## What changed

- 4 SKILL.md files added template citations:
  - `skills/slo-sast/SKILL.md` — new "Coverage-gap reporting" subsection citing `references/security/security-assessment-summary-template.md` for coverage-gap summaries and `references/security/security-finding-template.md` for individual high-severity gaps.
  - `skills/slo-rulegen/SKILL.md` — new "Reporting suspect rules" subsection citing the finding template for `gate`-failed rules.
  - `skills/slo-ruleverify/SKILL.md` — new "Expanded failure findings" subsection citing the finding template for clean-tree / coverage-gate failures.
  - `skills/slo-ship/SKILL.md` — new "Optional security-summary section (gated)" subsection citing the assessment-summary template, gated on the phrase "introduced new public surface".
- 1 new structural-contract test at `xtasks/sast-verify/tests/sap_imp_m1_citations.rs` with 5 test functions:
  - `every_security_skill_cites_a_template` — asserts ≥ 1 citation per skill in the 6-skill citing set.
  - `cited_template_paths_resolve` — asserts every cited canonical-template path exists at HEAD.
  - `slo_ship_security_summary_is_gated` — asserts gate phrase within 200 chars of slo-ship's citation.
  - `no_skill_links_to_examples` — anticipates M2's F-ENG-3 invariant (no shipped SKILL.md links to `examples/`).
  - `ast_parser_excludes_code_block_content` — verifies `pulldown-cmark` parser excludes fenced-code-block link-like syntax.
- 1 new dependency: `pulldown-cmark = "0.10"` as `[workspace.dependencies]` and `[dev-dependencies]` in `xtasks/sast-verify/Cargo.toml`. **Allow-list extension granted by user** during `/slo-execute M1` per F-ENG-1 critique resolution.

## Design decisions and why

- **`pulldown-cmark` AST-based parser, not regex** — F-ENG-1 critique resolution. AST parsing correctly excludes link-like syntax inside fenced code blocks (verified by `ast_parser_excludes_code_block_content`). Regex would have false-positives on code blocks demonstrating Markdown link syntax. Cost: 1 new dev-dep; benefit: structural correctness against the entire class of "is-this-a-real-citation?" edge cases.
- **`pulldown-cmark` 0.10 struct-variant `Tag::Link`** — version 0.10 changed the API from tuple to struct variants (`Tag::Link { link_type, dest_url, title, id }`). Pinned the version to ensure the test compiles deterministically.
- **Citation count rule = ≥ 1 per skill, not 2** — reviewed during /slo-plan, confirmed in /slo-critique. Some skills only naturally cite the summary template (slo-sast); others only cite the finding template (slo-rulegen, slo-ruleverify); only slo-ship cites both. Forcing both citations per skill would create stilted prose; ≥1 is the minimum useful invariant.
- **`/slo-ship` gate phrase = "introduced new public surface"** — drawn from the design doc and the M1 BDD scenario. Initial slo-ship draft placed the citation 220+ chars from the gate phrase due to a long parenthetical; tightening (commit 7626cbe) compressed the prose so the gate phrase precedes the citation by < 80 chars. The 200-char window in the test is the contract bound.

## Assumptions verified

- **`pulldown-cmark` 0.10 is the latest stable** — `cargo build` resolved without conflict. Verified MIT/Apache-2.0 license.
- **`xtasks/sast-verify` test target auto-discovery** — placing `sap_imp_m1_citations.rs` under `tests/` made it auto-discoverable; no `Cargo.toml` `[[test]]` registration needed.
- **AST parser excludes code-fence content** — verified by `ast_parser_excludes_code_block_content` test with a sample containing real link + code-block "fake" links; AST yields 1 link (the real one).

## Assumptions still unresolved

- **Reference-style Markdown links** (`[text][ref]` followed by `[ref]: url`) — pulldown-cmark resolves these to the same `Tag::Link` event, so they should count, but no test fixture exercises this. If a future skill rewrite uses reference-style links exclusively, the citation invariant should still hold; if it doesn't, add a fixture in M3 or later.
- **Non-canonical alias paths** — F-ENG-1 critique calls out "no alias paths" but the test currently checks if the destination "ends with" or "contains" the canonical template paths. A symlink-style alias (e.g., `references/security-finding.md` symlinked to the canonical path) would currently match. M3's standards-mapping work may tighten this.

## Mistakes made

- **Initial slo-ship draft put the gate phrase too far from the citation.** The first version of the "Optional security-summary section" had a long parenthetical enumerating "new public surface" types between the gate phrase and the citation, pushing them ~220 chars apart. The structural-contract test's 200-char window caught it; tightening the prose (commit 7626cbe) was the fix.
- **Didn't pin `pulldown-cmark` API version awareness.** The test was first written using the 0.9 tuple-variant `Tag::Link(_link_type, dest, _title)` syntax. 0.10 changed to struct-variant. Compile error surfaced the API change quickly; minor friction.

## Root causes

- **Long gate-phrase distance**: writing the section with reader-friendly prose (long enumeration of what counts as "new public surface") came at the cost of citation-distance. The structural-contract test's 200-char window is a useful forcing function — it pushes the gate phrase to be a *terse* trigger rather than a long disclosure.
- **API version drift**: pulldown-cmark 0.10 was the workspace-pinned version; my mental model was 0.9 from a prior project. Always read the version's API docs before writing match arms.

## What was harder than expected

- **The `cargo clippy --workspace --all-targets -- -D warnings` gate fails on pre-existing dead-code in `sast-verify/src/{tier_detect,yaml_schema}.rs`.** This is not an M1 issue but it surfaces a baseline-state question: should `/slo-execute` close M1 with the workspace-wide clippy gate red? Honest call: M1 ran the runbook-declared baseline (`cargo test -p sldo-common -p sldo-install -p sldo-research`) which is green, plus M1-scoped clippy (`cargo clippy -p sast-verify --test sap_imp_m1_citations`) which is clean. Workspace-wide clippy is a separate baseline-debt issue that pre-dates M1.

## Invariants/assertions added or strengthened

- (a) Every skill in `{slo-sast, slo-rulegen, slo-ruleverify, slo-ship, slo-critique, slo-verify}` contains ≥ 1 link to either canonical template path. (Encoded in `every_security_skill_cites_a_template`.)
- (b) Every cited canonical-template path resolves to an existing file at HEAD. (Encoded in `cited_template_paths_resolve`.)
- (c) `/slo-ship`'s security-summary citation is preceded by the gate phrase within 200 chars. (Encoded in `slo_ship_security_summary_is_gated`.)
- (d) No shipped SKILL.md links to `examples/` (early-landing of M2's F-ENG-3 invariant). (Encoded in `no_skill_links_to_examples`.)
- (e) Markdown citation counting uses `pulldown-cmark` AST events, not regex. (Encoded structurally in `extract_link_destinations`; verified behaviorally in `ast_parser_excludes_code_block_content`.)

## Resource bounds established or verified

- **NEW-citation count = 4 skills × ≥ 1 citation = 4–6 NEW citations added** (slo-sast adds 2, slo-rulegen 1, slo-ruleverify 1, slo-ship 2). Floor-1 per skill is the structural invariant; the runbook's "exactly 4 NEW citations" target referred to skills not links.
- **6-skill citing set** is hardcoded in `CITING_SKILL_SET` constant. Adding a 7th skill to the set requires editing the constant + a runbook amendment.
- **200-char gate phrase window** for `/slo-ship` is encoded in the test.

## Debugging / inspection notes

- `pulldown-cmark` 0.10's `Tag::Link` is a struct variant. Initial test compile failed with `error[E0164]: expected tuple struct or tuple variant, found struct variant`. The compiler's `help: the struct variant's fields are being ignored` suggested the exact pattern: `Tag::Link { link_type: _, dest_url: _, title: _, id: _ }`.
- The `slo-ship` gate-phrase failure was diagnosed by reading the panic message + the line in question — no debugger needed for Markdown structural drift.

## Naming conventions established

- **Test file naming**: `xtasks/sast-verify/tests/sap_imp_m<N>_<feature>.rs` — matches the runbook prefix `sap-imp` and milestone number, followed by a lowercase-snake feature name.
- **Test function naming**: `<invariant_being_tested>` — descriptive, no `test_` prefix (Rust idiom).

## Test patterns that worked well

- **Failure-message prose includes the asserted invariant** — when a test fails, the message says "expected ≥ 1 link to references/security/security-{finding,assessment-summary}-template.md in skills/<name>/SKILL.md, found 0 (links seen: [...])". The reader sees what was expected, what was found, and the actual link list.
- **Vacuous-pass for absent-fixture tests** — `slo_ship_security_summary_is_gated` returns early if `/slo-ship` SKILL.md doesn't cite the assessment-summary template at all. The "did the gate hold" check is only meaningful when the citation IS present; the citation invariant test is what enforces presence.

## Missing tests that should exist now

- A fixture-based test using a synthetic skill SKILL.md (under `tests/fixtures/`) demonstrating the failure path with controlled inputs. Currently the test only walks real shipped skills, so the failure path is exercised only when a real skill is broken. If we want to test the test itself, we need a fixture-based test. **Lane**: `micro` — could be folded into M2 or a future engineering-skill-improvements milestone.

## Rules for the next milestone

- **Read pulldown-cmark 0.10 API docs first** if M2 needs further AST walking (it does — F-ENG-3 reuses the AST walk).
- **Keep gate phrases tight in skill prose** — the 200-char window is real; long parentheticals defeat it.
- **Don't run workspace-wide clippy as a hard gate** until baseline-debt in `sast-verify/src/{tier_detect,yaml_schema}.rs` is resolved. Use crate-scoped clippy for milestone-specific quality gates.
- **For M2's PII regex test**, use the `regex` crate (already in workspace.dependencies). No new dep needed.

## Template improvements suggested

- **/slo-execute could explicitly call out that the runbook's "Full tests: cargo test --workspace" Evidence Log row may not match the runbook-declared baseline** when the workspace contains pre-existing parked or red tests. The skill currently treats Full tests as required-green; in practice, milestone-scoped tests + runbook-declared-baseline-green is the actual contract.
- **The v4 template could include an "Allow-list extension" row format** — when a milestone needs an out-of-scope change, the user can grant an extension and the contract should record it. M1 added this format ad-hoc; folding it into the v4 template would standardize.
