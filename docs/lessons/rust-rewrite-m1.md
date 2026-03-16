# Lessons Learned — Milestone 1: Cargo Workspace Scaffolding

## Design Decisions

- **Workspace root as a package**: The workspace root `Cargo.toml` includes a `[package]` section (`sunlit-orchestrate-tests`) so that workspace-level E2E tests in `tests/` can be discovered by `cargo test --workspace`. Without a package, a virtual workspace cannot own test targets. This is a common Rust pattern for workspace-level integration tests.
- **Crate naming**: Crate directories use hyphens (`sldo-common`, `sldo-plan`, `sldo-run`) matching binary names. The library crate's Rust module name is `sldo_common` (underscored, per Rust convention). This is declared explicitly via `[lib] name = "sldo_common"` in the crate's `Cargo.toml`.
- **Explicit `[[test]]` targets**: E2E tests are declared as explicit `[[test]]` targets in the root `Cargo.toml` rather than relying on auto-discovery, giving clarity about which test files belong to which milestone.

## What Was Harder Than Expected

- **Binary execution in E2E tests**: Running binaries via `cargo run --bin sldo-plan` from within E2E tests fails when the root package doesn't contain those bins. Using `env!("CARGO_MANIFEST_DIR")` + `/target/debug/sldo-plan` to locate the built binary directly is more reliable.
- **Quarantine permissions on macOS**: Direct execution of binaries under `target/debug/` can trigger macOS quarantine/permission prompts in some environments. Using `cargo run -p <crate>` is a reliable alternative for manual smoke testing.

## Naming Conventions Established

- **Crate names**: `sldo-common`, `sldo-plan`, `sldo-run`
- **Library module**: `sldo_common`
- **E2E test files**: `tests/e2e_<prefix>_m<N>.rs` (e.g., `tests/e2e_scaffold_m1.rs`)
- **Unit tests**: `#[cfg(test)] mod tests` inside source files
- **Test function names**: snake_case describing the scenario (e.g., `version_returns_non_empty_string`)

## Test Patterns That Worked Well

- BDD-style Given/When/Then comments in every test function — improves readability.
- E2E tests that shell out to `cargo build --workspace` to verify the full workspace compiles.
- Using `env!("CARGO_MANIFEST_DIR")` to locate binaries reliably in E2E tests.

## What the Next Milestone Should Do Differently

- Milestone 2 will add many modules to `sldo-common`. Each module should have BDD tests written before the implementation. The `#[cfg(test)] mod tests` pattern established here should be followed consistently.
- The `which` crate should be added as a dependency for `check_copilot_installed()` in `preflight.rs` (mentioned in the M2 step-by-step). Add it to `[workspace.dependencies]` to keep dependency versions centralized.
- The `chrono` crate (needed for M2 timestamps) should also go in `[workspace.dependencies]`.
- Consider adding `[[test]]` entries for future E2E files in the root `Cargo.toml` as they are created.

## BDD Scenarios to Retroactively Add

- None needed for M1. The current scenarios (workspace builds, tests pass, binaries run) cover the milestone completely.
