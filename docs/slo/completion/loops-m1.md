# Completion Summary — loops Milestone 1

## Goal completed
- The four engineering loops (sprint, security-tuning, lessons, library-feedback) are now first-class artifacts at `docs/LOOPS-ENGINEERING.md`. ARCHITECTURE.md cross-links to them. Every cited engineering SKILL.md back-links to the loop it participates in. A newcomer asking "which loop am I in, and what do I run next?" can answer it in 90 seconds via the "Start here" table.

## Files changed
- `docs/LOOPS-ENGINEERING.md` (new)
- `docs/ARCHITECTURE.md` — added "Feedback loops" section
- `docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` — Milestone Tracker row updated to `done`
- `docs/slo/lessons/loops-m1.md` (new)
- `docs/slo/completion/loops-m1.md` (new)
- `README.md` — added LOOPS-ENGINEERING.md to docs index
- `skills/slo-ideate/SKILL.md`, `skills/slo-research/SKILL.md`, `skills/slo-architect/SKILL.md`, `skills/slo-tla/SKILL.md`, `skills/slo-plan/SKILL.md`, `skills/slo-critique/SKILL.md`, `skills/slo-execute/SKILL.md`, `skills/slo-verify/SKILL.md`, `skills/slo-retro/SKILL.md`, `skills/slo-ship/SKILL.md`, `skills/slo-sast/SKILL.md`, `skills/slo-rulegen/SKILL.md`, `skills/slo-ruleverify/SKILL.md` — appended a one-line "Loops" back-link footer to each
- `crates/sldo-install/tests/e2e_loops_m1.rs` (new)

Pre-flight maintenance fixes (out of milestone scope but blocking the green-baseline gate):
- `crates/sldo-install/tests/e2e_slo_sec_m1.rs` — re-pointed stale `RUNBOOK-SAST-RULEGEN-A.md` reference to `RUNBOOK-SLO-SEC-LIBS.md`
- `crates/sldo-install/tests/e2e_slo_sec_m2.rs` — same re-point
- `tests/e2e_research_m7.rs` — removed two backwards-compat tests for the removed `sldo-plan` / `sldo-run` binaries

## Tests added
- `crates/sldo-install/tests/e2e_loops_m1.rs` — five structural-contract tests (`loops_engineering_doc_exists_and_has_required_sections`, `loops_engineering_doc_has_start_here_orienter`, `architecture_md_cross_links_loops_engineering`, `every_cited_engineering_skill_has_cross_reference`, `library_feedback_loop_has_unshipped_footnote`).

## Runtime validations added
- All five M1 tests run as part of `cargo test --workspace` and `cargo test -p sldo-install`. They assert the document is present, opens with a "Start here" orienter, has the four loop sections with the per-loop schema markers, is cross-linked from ARCHITECTURE.md, and that every cited SKILL.md carries the back-link.

## Compatibility checks performed
- Every engineering SKILL.md still installs symlinked correctly (existing `crates/sldo-install/tests/install_e2e.rs` passes).
- ARCHITECTURE.md still renders cleanly; the new "Feedback loops" section sits between the existing "Skill pack" content and "Current host boundaries".
- No existing test failed after M1's changes — full workspace run shows 62 test result lines, all `ok`, 0 `FAILED`.

## Documentation updated
- `docs/ARCHITECTURE.md` — new "Feedback loops" section linking the engineering and business loops docs.
- `README.md` — added a docs-index bullet for `docs/LOOPS-ENGINEERING.md`.
- 13 engineering `skills/<name>/SKILL.md` files — appended a back-link footer.

## .gitignore changes
- None required.

## Test artifact cleanup verified
- `git status` shows no untracked test artifacts. The new files are the doc, the test, the lessons + completion files, and the SKILL.md edits — all expected.

## Deferred follow-ups
- M2 will add `docs/LOOPS-BUSINESS.md` and update ARCHITECTURE.md's "Feedback loops" section to point there as well; the bullet is already in place.
- A future "Library-feedback loop" runbook (`/slo-sec-libs`, R4) will replace the placeholder section here with a fully-shipped flow.
- The `LESSONS-BACKLOG.md` audit-row schema is referenced in the lessons loop but won't exist on disk until M3 wires it up.

## Known non-blocking limitations
- The "Start here" table currently has six rows; if it grows much beyond 8-10 it will stop being scannable. Any future row addition must drop a row that has become less useful — anti-process-theatre check.
- The Library-feedback loop section is a placeholder; until R4 ships, the upstream-feedback path runs through the Lessons loop's `upstream-OSS` classification.
