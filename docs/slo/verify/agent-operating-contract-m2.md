# Verification Report — agent-operating-contract Milestone 2

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `official_roots_not_erased` | happy path | Ran `cargo test -p sldo-install --test e2e_agent_operating_contract`; test reads capability docs for `.github/skills`, `.agents/skills`, `.github/agents`, and the 2026-05-19 source date. | pass | Capability docs now distinguish official host-native roots from SLO compatibility roots. |
| `compatibility_roots_not_removed` | backward compat | Ran `cargo test -p sldo-install --test e2e_agent_operating_contract`; test reads `README.md`, `docs/getting-started.md`, `skills/README.md`, and `crates/sldo-install/README.md`. | pass | Onboarding docs still name `.copilot/skills` and `.codex/skills` as compatibility roots. |
| `unknown_migration_not_silent` | invalid input | Confirmed the new M2 assertions failed before the docs update, then passed after the docs update. | pass | Initial failure reported missing `.github/skills` and missing `compatibility root` wording. |
| `cloud_agent_not_confused_with_slo_runtime_harness` | abuse case | Ran `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`. | pass | Docs keep the exact invariant that no Copilot or Codex SLO runtime harness is shipped today. |

## Pass 4 Security / Static Evidence

| Stack / tool | Command | Result | Evidence |
|---|---|---|---|
| Rust structural tests | `cargo test -p sldo-install --test e2e_agent_operating_contract` | pass | 6 tests passed. |
| Rust regression tests | `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` | pass | 8 tests passed across 3 binaries. |
| Rust lint | `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings` | pass | Changed test target is lint-clean. |
| Formatter | `cargo fmt --all -- --check` | skipped | Pre-existing unrelated rustfmt drift remains outside the M2 allow-list. |
| Dependency audit | N/A | N/A | No dependency graph changes. |
| DAST | N/A | N/A | Markdown and structural-test milestone; no smoke service or OpenAPI surface. |
| Biz PII scan | N/A | N/A | M2 does not write `docs/biz-public/` artifacts. |

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| N/A | N/A | No M2 bugs found. | N/A | N/A |

## Environment

- OS: Darwin 25.4.0 arm64.
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`.
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.
- Browser: N/A — no UI surface.

## Coverage gaps

- No live Copilot, Codex, or Claude host session was invoked; M2 is intentionally documentation and structural-test only.
- Global `cargo fmt --all -- --check` remains blocked by pre-existing unrelated formatting drift.
