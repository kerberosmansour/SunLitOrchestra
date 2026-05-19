# Lessons Learned — agent-operating-contract Milestone 3

## What changed
- Added four GitHub Copilot custom-agent profiles under `.github/agents/*.agent.md`.
- Added M3 structural tests covering profile presence, frontmatter, tool scopes, portable fallback wording, output boundaries, and no-runtime-harness wording.
- Updated capability docs, Copilot overlays, architecture, catalog, and critique notes to reflect the new profile layer.
- Added the user-requested README acknowledgement for Karpathy's four-rule CLAUDE.md framing.

## Design decisions and why
- Scoped the profiles with `target: github-copilot` because the M3 goal is GitHub Copilot profile support, not generic editor behavior.
- Kept security and design reviewers read/search-only. They return findings; the lead writes the consolidated artifact.
- Allowed execution only on the verification profile because verification may need to run milestone-declared checks.
- Kept Codex on `/slo-critique` and `/slo-verify` because there is no shipped SLO Codex custom-agent profile equivalent.

## Mistakes made
- The first M3 contract omitted `README.md`; the user's explicit acknowledgement request required a scoped allow-list amendment.

## Root causes
- Host-native profile work is adjacent to repo-facing docs, so user-visible acknowledgement/docs requests can arrive while a milestone is in flight.

## What was harder than expected
- Avoiding false parity language. Copilot custom-agent profiles are useful, but they are not the same thing as the Claude-only runtime harnesses already called out in the capability matrix.

## Naming conventions established
- Copilot custom-agent profile files use `.github/agents/<slo-role>.agent.md`.
- Copilot profiles keep the same role names as the existing Claude-oriented `agents/*.md` files.

## Test patterns that worked well
- Testing for profile files before creating them gave a clean missing-file red state.
- Testing exact tool arrays kept the profile permissions simple and auditable.

## Missing tests that should exist now
- A future live-host smoke test could verify that GitHub Copilot actually lists the profiles after merge to the default branch.
- A future installer migration runbook should test native project roots separately from custom-agent profile discovery.

## Rules for the next milestone
- Keep host-native profile files small; do not paste full skill prose into them.
- Treat custom-agent profiles as host UX, not installer behavior.
- Do not add live runtime claims without a deterministic host smoke path.

## Template improvements suggested
- SLO runbooks should have a standard way to record user-requested mid-milestone scope additions without making them look accidental.
