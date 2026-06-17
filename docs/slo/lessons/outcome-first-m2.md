# Lessons Learned — outcome-first Milestone 2

## What changed
- `/slo-plan` now requires the §5C Outcome Validation Contract + §17 outcome sub-sections for value-bearing milestones (peer to §5A/§5B), and its specificity gate refuses vacuous/single-`And`/mock-only outcome scenarios + monolithic Front-to-End + blank Regression-Matrix rows. Added `references/outcome-validation-contract.md` and the SHA-pinned `outcome_first_m2_plan.rs`.

## Results vs thesis
- `N/A — methodology/tooling milestone`. M2 ships enforcement prose + a structural test; no telemetry.

## Design decisions and why
- **Reused the §5A/§5B trigger + forward-looking posture verbatim** — no new "value-bearing" definition (the `mloop_m3_plan::slo_plan_defines_value_bearing` test guards the existing definition; a divergent one would have been confusing double-gating).
- **New reference file** keeps the SKILL.md edit small and gives `/slo-critique`/authors a single how-to.
- **SHA-pin via post-edit hash** (founder direction) — computed `shasum -a 256` after the edit, set the const in lockstep.

## Mistakes made
- One M2 test (`plan_forward_looking_not_retroactive`) passed *before* implementation because `slo-plan` already contains "flag"+"legacy" from §5A/§5B. Not wrong (the invariant holds) but it doesn't isolate the M2 change — the other 4 tests carry the M2-specific weight. Noted for future: when an invariant is pre-satisfied, lean on the change-specific assertions.

## Root causes
- "flag"/"legacy" are generic enough to pre-exist. Acceptable; the marker set as a whole is M2-specific.

## What was harder than expected
- Nothing. The SHA-pin idiom copied cleanly from `sap_imp_m5_agents.rs`.

## Naming conventions established
- Reference files for a skill live at `skills/<skill>/references/<topic>.md` with the frontmatter `name/source_skill/description` (matched `methodology-milestone-authoring.md`).

## Test patterns that worked well
- Marker-substring assertions + a SHA-256 byte-pin in the same test file: the markers explain *what* changed (readable failure), the SHA guarantees *nothing else* changed.

## Missing tests that should exist now
- None for M2.

## Rules for the next milestone
- **M3 inserts Pass 0 into `slo-verify/SKILL.md` (non-renumbering, DW-001)** — enumerate every `Pass [0-9]` citation first; SHA-pin `slo-verify/SKILL.md`; preserve markers read by `svl_m4`/`kani_m3`/`mloop_m4`/`slo_tm_m2_consumers`/`sap_imp_m3_standards`.
- After M3, run the **theme-A mid-stream dogfood checkpoint** before M4.
- `cargo fmt -p sast-verify` before every close-out.

## Template improvements suggested
- None.

---

## Detected Work Ledger disposition (this milestone)
- **DW-002** (pre-existing clippy debt) — unchanged; still `file_github_issue` (slo-process), surfaced for user-confirmed filing at ship. No new findings in M2.
