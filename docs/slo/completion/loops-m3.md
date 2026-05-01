# Completion Summary — loops Milestone 3

## Goal completed
- `/slo-retro` now classifies each lesson, dedupes via three-strike `gh search`, files with explicit user confirmation, and falls back to `LESSONS-BACKLOG.md` when `gh` is unavailable. The marker (label `retro-derived`) is locked so `/slo-execute` M4 can query it. Issue filing is strictly additive after the lessons file write.

## Files changed
- `skills/slo-retro/SKILL.md` — added "## Issue filing" section + updated "## Outputs" to spell out additive ordering + extended "## Anti-patterns" with two filing-specific rules.
- `skills/slo-retro/references/issue-filing-discipline.md` (new)
- `crates/sldo-install/tests/e2e_loops_m3.rs` (new)
- `docs/ARCHITECTURE.md` — Feedback-loops paragraph now cites the reference file.
- `docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` — Milestone Tracker row updated to `done`.
- `docs/slo/lessons/loops-m3.md` (new)
- `docs/slo/completion/loops-m3.md` (new)

## Tests added
- `crates/sldo-install/tests/e2e_loops_m3.rs` — 10 structural-contract tests covering: SKILL.md extension, reference file presence, argv-list discipline, NO `--repo` rule, 40-issues/hour rate-limit cap, `LESSONS-BACKLOG.md` fallback, three-strike dedupe, NFKC normalization, body_sha256 audit-row field, marker choice, and that the new reference file does NOT mint a sibling skill.

## Runtime validations added
- All 10 M3 tests pass under `cargo test -p sldo-install`. They are pure structural-contract tests asserting the documented shape; runtime behavior of the agent following the discipline will be exercised when the next milestone close-out actually files issues against this repo (dogfood smoke test).

## Compatibility checks performed
- Existing lessons-file write (`docs/slo/lessons/<prefix>-m<N>.md`) happens BEFORE any `gh` call.
- If `gh` errors, the lessons file is still on disk. (Asserted by the SKILL.md prose; test invariants enforce the documented order.)
- Existing `/slo-retro` install symlink unchanged — the new reference file lives under `skills/slo-retro/references/` (not the top-level skills tree) and is not discovered as a sibling skill.
- `/slo-execute`'s existing pre-flight Step 1 (read previous milestone's lessons) unaffected.
- M1 + M2 structural-contract tests still pass.

## Documentation updated
- `skills/slo-retro/SKILL.md` — "Outputs" + "Issue filing" + "Anti-patterns" sections.
- `skills/slo-retro/references/issue-filing-discipline.md` — new locked reference.
- `docs/ARCHITECTURE.md` — Feedback-loops section now cites the reference.

## .gitignore changes
- None required. `LESSONS-BACKLOG.md` is intended to be a tracked file in the user's target repo when `gh` is unavailable; we don't ship one in this repo.

## Test artifact cleanup verified
- `git status` shows only the M3-expected new files plus the SKILL.md edit and the runbook tracker update. No untracked test fixtures.

## Deferred follow-ups
- M4 introduces the `/slo-execute` pre-flight read of these filed issues plus the runbook template's "Carry-forward from prior retros" section.
- Real-world `gh search` reliability on `--label retro-derived` cannot be exercised here without populating issues against the repo. Dogfood: the next milestone close (M3 itself, then M4) will produce real `retro-derived` filings (with user confirmation) — that is the canonical first soak test.
- A future runtime / integration test could synthesize a homoglyph candidate and assert the three-strike dedupe behavior directly.

## Known non-blocking limitations
- The marker-choice spike was not run against a populated test repo (auto-mode, defensible-default rationale captured in lessons). If the label scheme proves brittle in real usage, the reference file is the single update point and the structural-contract tests re-run.
- The 40-issues/hour cap is per-session-per-hour by documented discipline; there is no cross-session enforcement (matches the `/slo-sec-libs` pattern this inherits from).
- The `/slo-sec-libs` skill ships in Runbook 4; until then, `upstream-OSS` lessons go through this same `/slo-retro` filing flow with the upstream classification, not through a dedicated skill.
