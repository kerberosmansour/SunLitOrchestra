# Lessons Learned — svl Milestone 3

## What changed
- Extended `crates/sldo-common/src/runbook.rs::MilestoneStatus` additively to be total over the documented status set (`blocked` + the five honest exit states), with `Display`/`FromStr` arms and an `is_complete()` helper.
- **Fixed F-ENG-1**: `parse_tracker` now gates on a backtick-wrapped status token in column 3 (instead of a fixed regex of three words), so `blocked`/`accepted_risk`/future/unknown rows are no longer silently dropped; unknown maps to `Blocked` (never `NotStarted`/`Done`). `all_done` can no longer falsely report a runbook complete.
- Added 5 Rust regression/round-trip tests (incl. the F-ENG-1 and F-ENG-2 cases) + a "non-milestone table skipped" guard.
- Bumped workspace version 0.1.2 → 0.1.3 (crates.io discipline).
- Extended the v4-template status comment (both copies, byte-identical) with the 5 new values + the unknown→blocked fail-safe note.
- Added the Operator Readiness Gate to `/slo-execute` Global Entry (pre-flight Step 4.7, fail closed); `/slo-resume` now recognises the new states read-only + the unknown→blocked fail-safe.
- Documented both GitHub labels (`operator-action-required`, `security-review-required`) with `gh label create` commands in the canonical doc.
- Added `xtasks/sast-verify/tests/svl_m3.rs` (6 contract-text assertions).

## Results vs thesis
- N/A — not a value-bearing milestone (tooling/process).

## Design decisions and why
- **Gate on backtick-wrapped token in col 3, not a word-list regex** — the original `status_re = (not_started|in_progress|done)` was the root cause of F-ENG-1: a non-matching status made the whole row invisible. Column-position gating is robust to new statuses and to unknown tokens (the closing backtick anchor also prevents prefix-collision, e.g. `blocked` vs `blocked_by_operator`).
- **Unknown → `Blocked`, not a new `Unknown` variant** — keeps the enum closed and the fail-safe simple; the contract only needs "never silently complete", which `Blocked` (a non-`Done` state) satisfies.
- **`is_complete()` returns true only for `Done`** — `accepted_risk`/`issue_filed` are terminal but not green, so `all_done` stays conservative (a runbook with an accepted-risk milestone is not "all done").
- **Bumped the workspace version, not per-crate** — `version.workspace = true`; inter-crate deps use `^0.1.2`, which `0.1.3` satisfies, so no ripple to the dependent crates' Cargo.toml — the bump stayed inside the allow-list.

## Mistakes made
- None functional. Caught (at design/edit time) that bumping the workspace version *could* ripple to dependent crates' `version =` requirements; verified `^0.1.2` semver makes `0.1.3` compatible, so no out-of-allow-list edit was needed.

## Root causes
- F-ENG-1 root cause (historical): status parsing was done by scanning the whole line for one of three literal words, so any other status silently failed the row gate. Surfaced by `/slo-critique`, not by any prior test — the original tests only used the three happy-path statuses.

## What was harder than expected
- Reasoning about whether the workspace version bump would force editing dependent crates' Cargo.toml. Resolved by semver analysis (`^0.1.2` ⊇ 0.1.3).

## Naming conventions established
- Honest-exit-state enum variants are UpperCamel of the snake_case status (`BlockedByOperator` ↔ `blocked_by_operator`).
- Fail-safe rule phrasing reused verbatim across template comment, `/slo-execute`, `/slo-resume`, and `sldo-common` docs: "unknown → `blocked`, never silently `done`".

## Test patterns that worked well
- A real behavioural regression test (`all_done_false_when_a_row_is_blocked_by_operator`) that reproduces the exact critique defect — the highest-value test in this runbook.
- Splitting the contract surface: Rust *behaviour* tested in `sldo-common`'s own `mod tests`; contract-*text* tested in `svl_m3.rs`. Each failure localises.

## Missing tests that should exist now
- None for M3. (M4: a "no new lane verb" assertion for the ledger.)

## Rules for the next milestone
- **M4**: Detected Work Ledger discipline in `/slo-execute` (refuse `done` on an undisposed row); reconcile the five dispositions to existing `/slo-retro` lanes with **no new lane verb** (assert it); `/slo-verify` records Bundle A–F as first-class evidence rows. `/slo-verify` and `/slo-retro` ARE SHA-pinned by `slo_tm_m2_consumers.rs` (and `slo-critique` by `sap_imp_m5_agents.rs` — not touched). **Update the `slo_tm_m2_consumers.rs` SHA baseline in the same milestone as the `/slo-verify` edit**, or that test goes red.
- File a Detected Work item for the pre-existing `cargo deny` licenses-policy failure (don't absorb it silently).

## Template improvements suggested
- None.

## filed_issues
- none filed this milestone. One detected-work item recorded for M4's ledger: pre-existing `cargo deny check` licenses-policy failure (not introduced here; dependency graph unchanged).
