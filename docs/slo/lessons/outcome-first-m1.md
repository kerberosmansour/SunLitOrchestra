# Lessons Learned — outcome-first Milestone 1

## What changed
- Added the Outcome First Engineering contract surface to the v4 runbook template (both byte-identical copies): §5C Outcome Validation Contract, §17 sub-sections (Outcome Scenarios `oc-N`, Critical User Journeys `cuj-N`, Core Capability Regression Matrix), §11 Outcome test layer + inverted-pyramid narrative (§11.8), §6.12 "Outcome outranks unit" rule, and the `~~~text` fence rule. Added the structural-contract test `xtasks/sast-verify/tests/outcome_first_m1_template.rs` (8 assertions).

## Results vs thesis
- `N/A — methodology/tooling milestone`. The runbook is value-bearing but M1 ships template sections + a structural test, not behavioural telemetry; the §5A thesis is dogfood-measured after M5 (and the gate-fires proof is the theme-A checkpoint after M3). The leading-metric proof ("a dogfood runbook authors §5C + outcome sections") is not yet observable at M1.

## Design decisions and why
- **Edit the mirror, then `cp` onto the skill-primary copy** — guarantees the two v4 template copies stay byte-identical (`svl_m1`/`svl_m3`/`mloop_m3_plan` enforce it) without hand-applying the same diff twice and risking drift.
- **Pass 0, not "Pass 1"** — the template references "`/slo-verify`'s Outcome Validation pass" loosely; M3 will implement it as a non-renumbering Pass 0 (DW-001).
- **Per-layer Front-to-End** (theme B) — §5C steps are `applicable | not_applicable(reason)` with ≥1 real cross-layer assertion, so no-UI/library targets (this repo included) can't fake the gate or lose the backend proof.

## Mistakes made
- None blocking. The new test file needed `cargo fmt` after authoring (two long `assert!` lines) — caught by the §4.2 formatter gate before close.

## Root causes
- The fmt diff was authored-style wrapping; `cargo fmt -p sast-verify` fixed it in-place, touching only the one new file.

## What was harder than expected
- Nothing material. The §5A/§5B optional-section blockquote copied verbatim for §5C made the additive/backward-compat posture trivial to get right.

## Naming conventions established
- Test files: `xtasks/sast-verify/tests/outcome_first_m<N>_*.rs` (M1 = `outcome_first_m1_template.rs`).
- Frozen ID schemes: `oc-<slug>-N` (Outcome Scenario), `cuj-<slug>-N` (Critical User Journey) — contiguous from 1, never renumbered (same discipline as `tm-<slug>-abuse-N`).
- Outcome test layer paths: `tests/outcome/<prefix>_outcome_<journey>.<ext>` / `src/outcome/<journey>.outcome.test.tsx`.

## Test patterns that worked well
- Mirroring `mloop_m3_plan.rs`'s `workspace_root()` + `read()` helpers and its "both copies + byte-identity + no-renumber + marker" assertion shape. Asserting **positional** ordering (`§5B index < §5C index < §6 index`) proves "insertion not renumber" better than presence alone.

## Missing tests that should exist now
- None for M1. (The *gate-fires* runtime proof is intentionally the theme-A dogfood after M3, not an M1 structural assertion — recorded as a coverage gap in the verify report, by design.)

## Rules for the next milestone
- **M2 edits `slo-plan/SKILL.md` → SHA-pin it** in `outcome_first_m2_plan.rs` (founder direction); reuse §5A/§5B trigger + forward-looking posture verbatim; preserve the markers `mloop_m3_plan`/`svl_m2` already assert on slo-plan.
- Run `cargo fmt -p sast-verify` before the close-out gate every milestone — new test files need it.
- The pre-existing clippy debt (DW-002) is OUT of every allow-list; do not "fix while here."

## Template improvements suggested
- None — the v4 template's optional-section pattern absorbed §5C cleanly.

---

## Detected Work Ledger disposition (this milestone)
- **DW-001** (Pass-0 non-renumber seam) — `fix_now` scheduled in M3; not yet due.
- **DW-002** (pre-existing `cargo clippy -p sast-verify --all-targets -- -D warnings` debt: `sap_imp_m3_standards.rs:274`, `tier_detect.rs:28`, `yaml_schema.rs:20` — outside all allow-lists) — `file_github_issue` via `slo-process` lane. **Surfaced for user-confirmed filing; NOT auto-filed.** Same debt as innovation-loop's DW-001.
