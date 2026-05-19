# Lessons Learned — agent-operating-contract Milestone 2

## What changed
- Refreshed the host capability docs with a 2026-05-19 source date and separated official host-native roots from SLO installer compatibility roots.
- Added M2 structural assertions to `crates/sldo-install/tests/e2e_agent_operating_contract.rs`.
- Updated onboarding docs so existing `.copilot/skills` and `.codex/skills` users are not silently told their install paths vanished.
- Recorded Copilot custom agents as M3 input, not as shipped SLO parity.

## Design decisions and why
- Kept installer behavior unchanged. M2 is a truth-doc and guardrail milestone, not a migration.
- Used the terms "official host-native root" and "SLO installer compatibility root" so docs can describe current host behavior without breaking existing users.
- Preserved the existing `Headless runtime automation` anchor phrase because older structural tests already use it as the compatibility signal.

## Mistakes made
- The first docs edit renamed the `Headless runtime automation` row too aggressively, which broke an existing regression test.
- The M2 allow-list initially omitted SLO closeout artifacts even though `/slo-verify` and `/slo-retro` require them.

## Root causes
- The repo previously mixed official host paths and installer compatibility paths in one mental bucket.
- Some structural tests intentionally depend on stable wording, so meaningful wording changes need to preserve those anchor strings.

## What was harder than expected
- Making the docs more precise without implying a migration had already happened.
- Keeping Copilot custom-agent preview support visible while still saying plainly that SLO does not ship a Copilot/Codex headless runtime harness today.

## Naming conventions established
- `official host-native root` means a path documented by the host vendor for native host behavior.
- `SLO installer compatibility root` means a path currently produced or documented by `sldo-install`.
- `.github/agents/*.agent.md` is reserved for any future Copilot custom-agent profile work.

## Test patterns that worked well
- Failing structural tests over multiple docs caught both missing official roots and missing compatibility-root wording.
- Keeping existing agent-host tests in the regression set caught the accidental loss of a stable anchor phrase.

## Missing tests that should exist now
- M3 should add structural coverage for any `.github/agents/*.agent.md` profiles before creating them.
- A later installer-migration runbook should test CLI status output if `sldo-install` ever starts writing official host-native roots.

## Rules for the next milestone
- Read the previous lessons before expanding M3.
- Do not call Copilot custom agents equivalent to SLO runtime automation.
- If M3 emits `.github/agents/*.agent.md`, write structural tests first for paths, frontmatter, allowed tools, and fallback wording.
- Keep installer compatibility-root docs separate from host-native agent-profile docs.

## Template improvements suggested
- Runbook milestones should include `/slo-verify` and `/slo-retro` artifact paths in their allow-list when the user asks to follow every SLO step.
- Doc-heavy milestones should reserve one evidence row for stable anchor-string regressions.
