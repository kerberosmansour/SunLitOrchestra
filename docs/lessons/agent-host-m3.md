# Lessons Learned — agent-host Milestone 3

## What changed
- `/slo-research` is now host-native first. The installed `SKILL.md` tells the host agent to use its own research tools (web search/fetch, repo reads, file writes) and treats `sldo-research` as an optional, explicitly named Claude batch backend.
- `sldo-research` help text and module documentation now describe the binary as the optional Claude batch backend rather than the canonical research path.
- Living docs (`README.md`, `CLAUDE.md`, `copilot-instructions.md`, `docs/skill-pack-catalog.md`, `docs/ARCHITECTURE.md`, `docs/getting-started.md`, `docs/design/agent-host-capabilities.md`) align on the same split.
- New structural test `crates/sldo-install/tests/e2e_agent_host_m3.rs` guards against reintroducing a hidden `claude`/`sldo-research` requirement into the interactive path.
- New manual smoke checklist `docs/verify/agent-host-m3-smoke.md` covers both the Claude session and the GitHub Copilot session.

## Design decisions and why
- Keep `sldo-research` as a real, buildable Claude batch backend instead of deleting it — existing Claude users rely on it for non-interactive runs and the cleanup target was the implicit dependency, not the binary itself.
- Do not invent a Copilot-equivalent batch backend — there is no verified runtime for that today, and a fake parity layer would be the exact problem this milestone exists to remove.
- When the global Copilot root has unrelated conflicts, use the supported repo-local install root `./.copilot/skills/` instead of mutating `~/.copilot/skills/`. That keeps the smoke check honest without trampling unrelated user state.

## Mistakes made
- None new in this milestone closeout. The implementation slice was completed in the previous session under the same runbook contract and the only remaining outstanding work was the manual host-session smoke.

## Root causes
- The original hidden dependency was a single-line "the host agent's only tool is `sldo-research`" assumption in the installed skill plus matching wording in the living docs. Once the contract was reframed as "host-native first, batch backend optional", every dependent doc fell in line.

## What was harder than expected
- Keeping the optional-batch-backend wording consistent across six living docs without sliding into either "always required" or "never useful" framing. Both extremes would mislead users.

## Naming conventions established
- "Optional Claude batch backend" is the canonical phrase for `sldo-research` in living docs.
- "Host-native interactive research" is the canonical phrase for the default `/slo-research` path.
- Repo-local `.copilot/skills/` is a first-class validation target when the session is already running inside the repo and the global Copilot root has unrelated conflicts.

## Test patterns that worked well
- Structural tests that read installed skill copy and assert on the absence of hidden-dependency wording. They are cheap, run in-process, and fail loudly if a future edit reintroduces the implicit shell-out.
- A separate manual smoke checklist file (`docs/verify/agent-host-m<N>-smoke.md`) for things automation cannot verify, with explicit per-host sections.

## Missing tests that should exist now
- An integration smoke that programmatically drives GitHub Copilot against the installed skill — still out of scope for v1 because no verified Copilot runtime driver exists. The manual host-session checklist remains the right layer for this proof.

## Rules for the next milestone
- M4 is about honest naming of Claude-only automation. Do not introduce a generic `AgentRuntime` trait without a second real implementation. Rename modules and harness helpers, keep behavior unchanged, and preserve env-var aliases for users who already script the live runtime harness.
- M4 must keep the M3 contract intact — the interactive `/slo-research` path stays host-native; only the surrounding shared-library and harness names change.

## Template improvements suggested
- The Smoke Tests checklist template could distinguish "automatable" from "host-only manual" rows so deferred items are visibly different from skipped items in future runbooks.
