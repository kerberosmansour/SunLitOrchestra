# Lessons Learned - sec-libs Milestone 4

## What changed

- Extended `skills/slo-sec-libs/SKILL.md` with the explicit `--file-upstream --upstream-dir <path>` gate.
- Extended `skills/slo-sec-libs/references/upstream-filing-discipline.md` with upstream owner mapping, cap rules, spillover format, and fallback behavior.
- Added `crates/sldo-install/tests/e2e_sec_libs_m4.rs` with 12 structural-contract tests, including an executable cap-boundary helper.
- Updated the runbook tracker/evidence and the skill catalog.

## Design decisions and why

- **Default destination remains SLO intake.** Owner names alone do not trigger upstream filing; `--file-upstream` is the opt-in gate.
- **No `--repo` exception for upstream filing.** Direct upstream filing uses a local checkout whose origin matches the mapped owner repo, so the user can inspect the destination before confirming.
- **The cap is memory-only.** M4 tracks `filed_this_session` in the current invocation and refuses to persist state across sessions. Cross-session rate discipline remains the user's responsibility.
- **Declining upstream does not lose the gap.** The flow offers a separately confirmed SLO-intake fallback when the user says no to upstream filing.

## Test patterns that worked well

- The cap-boundary helper makes the 40/41 behavior executable without creating live issues.
- Tests pin the two allowed upstream mappings and the `unknown` refusal path.
- M1-M3 compatibility remains in the M4 suite so the new upstream path cannot erase default intake behavior.

## Missing tests that should exist later

- M5 dogfood should run the full read -> match -> file flow against real target data and record actual issue URLs after explicit confirmation.
- A future standalone filer executable should add fixture tests for owner mapping, decline fallback, cap spillover, and secondary-rate-limit responses.
- If more library owners are added, the mapping table should get a fixture for each owner.

## Rules for M5

- Dogfood must exercise the default intake path before relying on upstream filing.
- Any live filing still needs per-issue confirmation.
- Record whether each filed issue went to SLO intake or a direct upstream repo.
- Keep canonical `SunLitSecurityLibraries` spelling.
