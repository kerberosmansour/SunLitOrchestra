# Lessons Learned — mloop Milestone 1

## What changed
- `skills/slo-ideate/SKILL.md`: Q3 reframed from "smallest wedge" to "smallest complete value slice" (folding the report's two diagnostic prompts — behaviour-that-proves-value + technical/pricing/UX/demand — into Q3 rather than adding numbered questions); new `## Success thesis` output section (leading metric, lagging metric, guardrails, review window, miss-diagnosis); a "When to stop" bullet requiring the success thesis.
- `xtasks/sast-verify/tests/mloop_m1_ideate.rs` (NEW): 5 structural-contract assertions.

## Results vs. success thesis
- N/A — this is a meta/tooling milestone (it builds the success-thesis mechanism; it has no user-facing telemetry of its own). The loop's own thesis is "future idea docs carry a measurable success thesis," verified structurally here.

## Design decisions and why
- **Slot count kept at 7** — the interfaces doc declared the forcing-question count stable; folding diagnostics into Q3 preserves that while still delivering the report's intent.
- **Exact contiguous sentinels** — reused the kani-m1 lesson that `contains()` can be satisfied by interleaved prose; phrased "smallest complete value slice" as one contiguous string so the assertion is meaningful.
- **Behaviour-not-PII note** — added to the success-thesis template (C8 / data minimisation) so the very first artifact in the loop steers away from pasting real-user identifiers; routes that risk to the /slo-verify PII scan.

## Assumptions verified
- `/slo-ideate` frontmatter untouched → `sldo-install --dry-run` still discovers the skill (verified).
- No other skill's structural baseline regressed (full `cargo test -p sast-verify` green).

## Assumptions still unresolved
- None for M1.

## Mistakes made
- Initial test array literal wasn't rustfmt-clean; `cargo fmt --all` fixed it. Minor.

## Root causes
- Hand-wrote a multi-line array without running fmt first.

## What was harder than expected
- Nothing material.

## Invariants/assertions added or strengthened
- Q3 reframing present; `## Success thesis` + 4 sub-field sentinels; behaviour-not-PII; existing 4 sections preserved; forcing-question count == 7.

## Resource bounds established or verified
- Success thesis bounded to 1 leading + 1 lagging metric + ≤3 guardrails + 1 window (no unbounded metric list).

## Debugging / inspection notes
- Pre-fix run confirmed the 3 new-content assertions fail for the right reason before editing (anti-vacuity).

## Naming conventions established
- Test files: `xtasks/sast-verify/tests/mloop_m<N>_<area>.rs`. Test fns: `slo_<skill>_<assertion>`.

## Test patterns that worked well
- `workspace_root()` + `read()` helpers copied from `kani_m1_skill_contract.rs`; exact-substring assertions.

## Missing tests that should exist now
- None for M1.

## Rules for the next milestone
- M2 also edits a SKILL.md + `references/biz/artifact-schema.md`; reuse the same helper shape. Assert the `tier` enum is byte-unchanged.
- Run `cargo fmt --all` immediately after writing any new test file.

## Template improvements suggested
- None.

## Deferred follow-up (file as retro-derived issue, `micro` lane)
- **Pre-existing clippy red in `sast-verify`** — `tests/sap_imp_m3_standards.rs` (regex-creation-in-loops), `src/tier_detect.rs` (`Public` never constructed), `src/yaml_schema.rs` (`Rule` fields never read). Out of every mloop milestone's allow-list; fixing requires its own micro-ticket. The runbook's clippy gate is "pass or documented exception" — exception documented here.
