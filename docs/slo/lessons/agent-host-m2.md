# Lessons Learned — agent-host Milestone 2

## What changed
- Split the living docs into clear roles: `README.md` for orientation, `docs/getting-started.md` for the first-run path, `docs/skill-pack-catalog.md` for the canonical host-neutral catalog, and `CLAUDE.md` plus `copilot-instructions.md` as host overlays.
- Added `docs/slo/design/agent-host-capabilities.md` so host support claims have one explicit place to live.
- Replaced the stale `docs/ARCHITECTURE.md` with a reality-first summary of the current workspace and host boundaries.
- Added structural test coverage in `crates/sldo-install/tests/e2e_agent_host_m2.rs` to keep the catalog/overlay split and first-run guide from drifting again.

## Design decisions and why
- Kept `CLAUDE.md` as a rich overlay instead of deleting or shrinking it aggressively. Existing sessions and tests already rely on it, so the safer move was to demote it from canonical truth to host-specific overlay while preserving the catalog content Claude users expect.
- Introduced a dedicated capability matrix instead of sprinkling host limitations across several docs. Unsupported runtime surfaces are easiest to keep truthful when there is one document responsible for them.
- Rewrote `docs/ARCHITECTURE.md` instead of layering disclaimers on top of stale sections. The old document had drifted too far from the actual workspace members.

## Mistakes made
- The first Milestone 2 test only proved the missing-file blockers, not the full scope of stale assumptions in the existing living docs.

## Root causes
- The repo had allowed `README.md`, `CLAUDE.md`, the catalog, and the architecture doc to overlap in responsibility for too long, so each one accumulated a different slice of history.

## What was harder than expected
- `docs/ARCHITECTURE.md` was stale in a deeper way than the first failing test showed. Once the missing files were added, the real work was re-establishing one reality-first architecture story without touching historical runbooks and completion docs.

## Naming conventions established
- `docs/skill-pack-catalog.md` is the canonical living catalog.
- `CLAUDE.md` and `copilot-instructions.md` are host overlays.
- `docs/slo/design/agent-host-capabilities.md` is the single capability matrix for host support claims.
- Milestone-specific living-doc coverage lives in `crates/sldo-install/tests/e2e_agent_host_m2.rs`.

## Test patterns that worked well
- Structural Markdown tests were enough to lock the role split: canonical catalog, overlays, capability matrix, and getting-started guide.
- A missing-file failure is a good first guardrail for doc milestones because it forces the concrete deliverables into existence before style or wording debates start.

## Missing tests that should exist now
- Add a regression check that the baseline test command shown in the living docs stays aligned with the actual supported in-scope command.
- Add a stronger guard that `README.md` and `docs/ARCHITECTURE.md` do not reintroduce removed crates as active current interfaces.

## Rules for the next milestone
- Keep host claims bounded to validated runtime behavior, not to what the installed `SKILL.md` files make tempting to assume.
- When a doc split creates a canonical file plus overlays, add tests for both the split itself and the cross-links.
- If a living doc is stale at the model level, replace it cleanly instead of stacking caveats on top of it.

## Template improvements suggested
- The runbook allow-list should name newly created docs explicitly, not just the pre-existing files that will be updated.
- For doc-heavy milestones, the evidence template should reserve one row for the first failing structural test and one row for the final role split, because those are distinct checkpoints.