# Completion Summary — agent-operating-contract Milestone 2

## Goal completed
- The living host capability docs now distinguish official host-native instruction, skill, and agent paths from SunLit's existing `sldo-install` compatibility roots.

## Files changed
- `docs/slo/design/agent-host-capabilities.md`
- `docs/slo/design/host-capability-matrix.md`
- `README.md`
- `docs/getting-started.md`
- `skills/README.md`
- `crates/sldo-install/README.md`
- `docs/ARCHITECTURE.md`
- `docs/skill-pack-catalog.md`
- `crates/sldo-install/tests/e2e_agent_operating_contract.rs`
- `docs/RUNBOOK-AGENT-OPERATING-CONTRACT.md`
- `docs/slo/verify/agent-operating-contract-m2.md`
- `docs/slo/lessons/agent-operating-contract-m2.md`
- `docs/slo/completion/agent-operating-contract-m2.md`

## Tests added
- Added M2 structural assertions to `crates/sldo-install/tests/e2e_agent_operating_contract.rs`.

## Runtime validations added
- `docs/slo/verify/agent-operating-contract-m2.md` records the docs-only verification pass and static evidence.

## Compatibility checks performed
- Confirmed `.copilot/skills` and `.codex/skills` remain documented as SLO installer compatibility roots.
- Confirmed `.github/skills`, `.agents/skills`, and `.github/agents/*.agent.md` are recorded as host-native paths or future inputs, not as a silent installer migration.
- Confirmed `crates/sldo-install/src/host.rs` and `crates/sldo-install/src/paths.rs` were not changed.

## Documentation updated
- Host capability matrix and onboarding docs now carry the compatibility-root distinction.
- Architecture and catalog docs now preserve that distinction as a project invariant.

## .gitignore changes
- None.

## Test artifact cleanup verified
- `git status --short` shows intended source/doc changes and new SLO process artifacts only; no generated test artifacts were left behind.

## Deferred follow-ups
- M3 still needs to decide whether to add Copilot custom-agent profiles under `.github/agents/*.agent.md` or keep `/slo-critique` as the canonical portable fallback.
- Any migration from compatibility roots to official host-native skill roots needs a separate explicit runbook.

## Known non-blocking limitations
- No live Copilot or Codex host session was invoked in M2.
- `cargo fmt --all -- --check` remains blocked by pre-existing unrelated rustfmt drift outside the M2 allow-list.
