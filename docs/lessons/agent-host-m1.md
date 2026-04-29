# Lessons Learned — agent-host Milestone 1

## What changed
- Added a host descriptor table so `sldo-install` can target both `claude-code` and `github-copilot`.
- Upgraded the global manifest to schema v2 with per-entry host ownership and backward-compatible loading of v1 manifests.
- Added host-specific integration coverage for Copilot global/local installs, host-aware status/verify output, legacy manifests, and hostile manifest targets.
- Updated installer-facing docs and `.gitignore` for the new `./.copilot/` local state.

## Design decisions and why
- Kept one shared global manifest at `~/.sldo/install.toml` and added a `host` field per entry instead of splitting global manifests. This preserved the existing path while making uninstall/status/verify host-specific.
- Kept local manifests under the selected host directory (`./.claude/` or `./.copilot/`) so repo-local state stays isolated by host.
- Rejected manifest entries whose targets fall outside the selected host root before verify/uninstall touches the filesystem. That closes the destructive-manifest gap without introducing a larger abstraction.

## Mistakes made
- The first broad dry-run smoke used the real `HOME`, which collided with an existing non-symlink Copilot skill directory.

## Root causes
- Installer smoke checks that validate path selection should not assume the developer's real host state is clean.

## What was harder than expected
- The real bug was not just path selection. The shared manifest also needed host-scoped ownership semantics so the same skill name could be installed for two hosts safely.

## Naming conventions established
- Host ids are `claude-code` and `github-copilot`.
- New milestone-specific installer coverage lives in `crates/sldo-install/tests/e2e_agent_host_m1.rs`.

## Test patterns that worked well
- Execute the compiled binary against a temporary `HOME` and mutate the on-disk manifest to simulate legacy or hostile states.
- Validate safety behavior with `verify` when possible so tests prove rejection without removing anything.

## Missing tests that should exist now
- A follow-up test for `status` and `verify` on local Copilot manifests would tighten the local-path story further.

## Rules for the next milestone
- Keep host capability claims scoped to what the code really supports today.
- Use a temporary `HOME` for installer smoke checks unless the goal is specifically to inspect conflict handling against real user state.
- When a shared file or manifest is reused across hosts, make ownership explicit in the data model before widening docs.

## Template improvements suggested
- Milestone 1's allow-list should explicitly mention `docs/ARCHITECTURE.md`, because its post-flight section requires an architecture note.
- Milestone smoke tests that inspect install roots should call out temporary `HOME` usage so conflict-guard behavior is not mistaken for a regression.