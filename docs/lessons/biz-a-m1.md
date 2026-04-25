# Lessons Learned — biz-a Milestone 1 (`/slo-legal` v1 + shared `references/biz/` wedge)

## What changed

- **NEW skill** `skills/slo-legal/SKILL.md` — UK-only advisor with `draft|translate|triage|prepare` modes; v1 `draft` doc-types: `nda`, `contractor-sow`, `ip-assignment`, `terms-and-conditions` (B2B only).
- **NEW shared scaffolding root**: `references/biz/` (directory at repo root, outside `skills/`).
- **NEW** `references/biz/triage-gate.md` — single source of truth for the four hard-block predicates (`gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`); predicate-id schema with six required columns (`id`, `name`, `predicate`, `if_true`, `route_to`, `rationale_doc`).
- **NEW** `references/biz/cost-baseline-jpp-law-2026.md` — UK fixed-fee solicitor pricing baseline (JPP Law, retrieved 2026-04-25). Ships with placeholder GBP figures that must be replaced with current JPP Law page values at first use; prices are not in the open repo because the live page is the source of truth, refreshed annually.
- **NEW** `references/biz/templates/onenda-uk.md` — oneNDA UK Country Schedule placeholder, marker `ONENDA-UK-PLACEHOLDER`. CC BY-ND 4.0 verbatim render obligation documented; canonical bytes deferred (see "Mistakes made / Course corrections").
- **NEW** `crates/sldo-install/tests/e2e_biz_a_m1.rs` — 10 structural-contract tests, all green.
- **NEW** `docs/verify/biz-a-m1-smoke.md` — manual smoke-test checklist for runtime verification of the skill against five fixtures.
- **UPDATED** `docs/ARCHITECTURE.md` — added `slo-legal` row to the skill table; added an invariant paragraph documenting the `references/biz/` location and the `crates/sldo-install/src/install.rs:44-71` rationale.
- **UPDATED** `docs/RUNBOOK-BIZ-SKILL-PACK-A.md` Milestone Tracker — M1 status `done`.

## Design decisions and why

- **Predicate-id schema lives in `triage-gate.md` as a Markdown table, not a YAML / JSON sidecar.** Reason: Markdown is the SLO convention, the schema is read by `/slo-critique` and structural-contract tests as prose, and the predicate-id strings never need to be programmatically dispatched (they're cited, not invoked). A table with six columns is enough discipline; introducing a second format would create drift surface.
- **`references/biz/` at repo root, not under `skills/`.** Confirmed by reading `crates/sldo-install/src/install.rs:44-71`: `discover_skills()` requires `<skills_dir>/<name>/SKILL.md`. A sibling directory has zero installer interaction by construction. The structural test `references_biz_dir_not_discovered_as_skill` adds a regression guard for any future drift (e.g. someone moving the dir under `skills/`).
- **`cost-baseline-jpp-law-2026.md` ships with placeholder GBP figures, not snapshotted live values.** Reason: the live JPP Law page is the source of truth, prices change annually, and reproducing third-party pricing in an open-source repo without permission is a tighter compliance question than necessary. A founder running M1 implementation is instructed (in the file's header + the smoke-test checklist) to fetch the current figures and update before first use.
- **GDPR broad hard-block (locked 2026-04-25) is wired in M1 even though `references/biz/ico-duaa-index.md` and `references/biz/ico-enforcement-reality.md` ship in M2.** Reason: gate-4-gdpr-document is the predicate; the rationale docs are descriptive, not load-bearing for the gate's behavior. Rationale documents land when their second consumer (`/slo-accounting`) arrives in M2 — per CLAUDE.md "three similar lines is better than a premature abstraction."

## Course corrections taken in flight

- **oneNDA SHA-256 hash check deferred from M1 to a follow-up.** The runbook spec called for a pinned SHA-256 hash check at install time. In execution, oneNDA's primary distribution is PDF; producing verbatim Markdown bytes would require either WebFetch + PDF-extraction (fragile, may produce a non-canonical rendering) or hand-typing the canonical bytes (error-prone). Decision: ship `references/biz/templates/onenda-uk.md` as a placeholder with marker `ONENDA-UK-PLACEHOLDER`; structural test `onenda_template_placeholder_or_pinned_hash` asserts EITHER (a) the placeholder marker is present, OR (b) the file matches a future-pinned hash. `/slo-legal draft nda` refuses to draft until the placeholder is replaced, with a clear error message pointing to replacement instructions in the file itself. Future small follow-up runbook will pin the canonical hash once a project owner has fetched and verified the bytes from onenda.org.
- **Deferred PII-pattern scan in `/slo-verify`.** M1's runbook spec said the artifact-schema reference (M2) would enable a `/slo-verify` Pass 4 PII scan over `docs/biz-public/`. In execution, deferring the scan to Runbook B1 M1 (`/slo-talk-to-users`, the first generator producing PII-shaped artifacts) is cleaner — there will be a real fixture to test against rather than an empty-state placeholder. M2's lessons file should pick this up.
- **Description block-scalar handling in the structural test.** First test pass failed `slo_legal_skill_md_has_required_frontmatter` because the description used a YAML `>` block scalar, and the test asserted non-empty inline content. Fixed by extending the test to recognize block-scalar followed by indented content. The test now correctly handles inline strings, double-quoted strings, and `>`/`|` block scalars.

## Mistakes made

- **Initial test assumed all YAML strings were inline.** The `description: >` block-scalar form is common and was used in the SKILL.md frontmatter. The first test pass failed; cost was a single iteration cycle. Lesson: when writing structural tests for prose authored separately, anticipate the full surface of the prose's permitted forms — YAML supports inline / quoted / block-scalar / multi-line; tests should accept all four.
- **No mistake found on the predicate schema design.** The four predicate IDs are stable interface; the cross-skill citation test in M2 will lock that. M1's structural test asserts the four IDs exist in the gate file and are cited by the slo-legal SKILL.md — both green at M1 completion.

## Root causes

- **YAML block-scalar oversight** — root cause: I wrote the test before authoring the SKILL.md. The test's strict-inline parse was a defensible default for "non-empty description" but didn't anticipate the SKILL.md prose I would later write using `>` for readability. Fix: either always test with realistic content first, or write the test with multiple permitted forms upfront.

## What was harder than expected

- **The cost-baseline file is awkward in an open repo.** The whole pack's ROI claim hinges on a price comparison, but the prices change annually and reproducing them risks staleness or attribution issues. The placeholder approach is correct but adds a manual step for users that the runbook had assumed away. Future runbook authoring should plan for this: cost data with annual refresh cadence belongs in a separate data file the founder owns, not the open-source pack.
- **Threading the oneNDA license obligation through SKILL.md, the placeholder file, the structural test, and the smoke checklist.** Four touch points for one license rule. Each correctly documented but the redundancy is the price of CC BY-ND 4.0 in a templating tool.

## What was easier than expected

- **Cross-skill drift prevention.** Putting the four predicate IDs in `triage-gate.md` as the single source of truth and asserting via structural test that `slo-legal/SKILL.md` cites all four works cleanly. M2's cross-skill citation test (`/slo-legal` AND `/slo-accounting` both cite the four IDs) is a simple extension of M1's pattern.
- **The runbook's M1 plan transferred to execution with minor pivots only.** The deferred SHA hash check and the deferred PII scan are the only material deviations; the rest of the spec landed as written.

## Recommendations for M2

- **Author the cross-skill citation test first.** It's the most valuable test in M2 (proves the advisor pattern replicates). Write it before authoring `/slo-accounting` SKILL.md so the test fails for the right reason ("slo-accounting/SKILL.md missing predicate-id citation") and the implementation is driven by the test.
- **The `triage-gate.md` immutability test in M2 should compare against the M1 commit hash of the file, not against a textual fingerprint.** Git already gives us byte-level fingerprints for free; using `git rev-parse HEAD:references/biz/triage-gate.md` (or equivalent) is more robust than parsing the file again.
- **Update the runbook's M2 evidence-log row for the deferred PII-scan note.** Explicit documentation that PII-scan integration in `/slo-verify` is now scheduled for Runbook B1 M1 (not M2 as M1's plan said).
- **Consider whether `/slo-accounting`'s `draft` modes should produce anything substantive in v1.** Accounting draft work is mostly "brief the accountant" memos, which are prose-shaped. The mode contract should be honoured (all four modes accepted) but `draft <doc-type>` may have a thinner v1 doc-type list than `/slo-legal`'s four — perhaps just `brief-the-accountant` and `r-and-d-claim-narrative`.

## Changes to ARCHITECTURE.md

- Added `slo-legal` to the skill table.
- Added invariant paragraph documenting `references/biz/` location + `crates/sldo-install/src/install.rs:44-71` rationale.

## Changes to runbook tracker

- M1 status `not_started` → `done`. Started 2026-04-25, completed 2026-04-25.
