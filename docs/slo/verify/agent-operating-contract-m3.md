# Verification Report — agent-operating-contract Milestone 3

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `copilot_profiles_are_bounded` | happy path | Ran `cargo test -p sldo-install --test e2e_agent_operating_contract`; test reads `.github/agents/*.agent.md` files for expected names, frontmatter, tools, target, line caps, and no traversal fragments. | pass | Four Copilot profiles exist with bounded tools and `target: github-copilot`. |
| `portable_fallback_not_erased` | backward compat | Same test reads profiles and capability docs for `/slo-critique`, `/slo-verify`, and canonical portable path wording. | pass | Profiles explicitly preserve the portable SLO skill paths. |
| `codex_not_promised_agent_parity` | invalid input | Same test reads capability docs for the Codex no-custom-agent-equivalent wording. | pass | Capability docs say Codex has no shipped SLO host-native custom-agent equivalent. |
| `preview_support_not_runtime_harness` | abuse case | Same test plus `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5`. | pass | Docs and profiles preserve the no-Copilot/no-Codex runtime-harness boundary. |

## Pass 4 Security / Static Evidence

| Stack / tool | Command | Result | Evidence |
|---|---|---|---|
| Rust structural tests | `cargo test -p sldo-install --test e2e_agent_operating_contract` | pass | 9 tests passed. |
| Rust regression tests | `cargo test -p sldo-install --test e2e_agent_host_m2 --test e2e_agent_host_m3 --test e2e_agent_host_m5` | pass | 8 tests passed across 3 binaries. |
| Existing Claude agent invariant tests | `cargo test -p sast-verify --test sap_imp_m5_agents` | pass | 7 tests passed; package compile emitted existing non-fatal warnings in `sast-verify`. |
| Rust lint | `cargo clippy -p sldo-install --test e2e_agent_operating_contract -- -D warnings` | pass | Changed test target is lint-clean. |
| Formatter | `cargo fmt --all -- --check` | skipped | Pre-existing unrelated rustfmt drift remains outside the M3 allow-list. |
| Dependency audit | N/A | N/A | No dependency graph changes. |
| DAST | N/A | N/A | Markdown profile/docs milestone; no smoke service or OpenAPI surface. |
| Live Copilot custom-agent execution | N/A | N/A | M3 is structural; local CI cannot deterministically run GitHub Copilot cloud-agent sessions. |

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| N/A | N/A | No M3 bugs found. | N/A | N/A |

## Environment

- OS: Darwin 25.4.0 arm64.
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`.
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.
- Browser: N/A — no UI surface.

## Coverage gaps

- No live GitHub Copilot cloud-agent session was invoked.
- Global `cargo fmt --all -- --check` remains blocked by pre-existing unrelated formatting drift.
