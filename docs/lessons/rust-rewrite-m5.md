# Lessons Learned — Milestone 5: Integration Tests, Documentation & Migration

## Design Decisions

- **Mock copilot as a shell script on PATH**: Rather than injecting a trait or mock at the Rust level, the integration tests place a `mock-copilot.sh` script (renamed to `copilot`) at the front of `PATH`. This matches the runbook's design and tests the real `std::process::Command` invocation path end-to-end.
- **Unique temp directories per test**: Each E2E test creates its own uniquely-named temp directories for both the mock copilot binary and the git repo. This avoids race conditions when tests run in parallel (as Rust tests do by default).
- **CLI parity via flag checking, not string matching**: The M4 lessons correctly predicted that `--help` output format differs between Bash (custom heredoc) and Rust (clap auto-generated). The parity tests check that each expected flag name appears in the help text rather than doing exact string comparison.
- **README restructured around Rust CLI**: The README now leads with Rust installation and usage, with Bash scripts moved to a "Legacy" section. This reflects the project's intended direction while preserving documentation for both.
- **MIGRATION.md as a standalone doc**: A separate migration guide keeps the README focused on usage while providing detailed flag mapping and behavioral comparison for users transitioning from Bash.

## What Was Harder Than Expected

- **Parallel test race conditions**: The initial implementation used a single shared temp directory (`sldo_e2e_m5_mock_bin`) for the mock copilot binary. Since Rust runs tests in parallel, multiple tests would race to `remove_dir_all` and `create_dir_all` the same path, causing sporadic "No such file or directory" errors. Fixed by passing a unique suffix to each `setup_mock_copilot_dir` call.
- **Mock copilot prompt parsing**: Extracting the output path from the planning prompt required careful regex patterns since the prompt structure includes the path in backtick-quoted markdown. The mock script handles both the "Write the completed runbook" and "runbook output to" patterns.

## Naming Conventions Established

- **Test fixtures directory**: `tests/fixtures/` for shared test data (mock scripts, sample files)
- **E2E test file**: `tests/e2e_integration_m5.rs` following the `e2e_<prefix>_m<N>.rs` pattern
- **Helper functions in tests**: `workspace_root()`, `binary_path()`, `create_temp_git_repo()`, `setup_mock_copilot_dir()`, `path_with_mock()` — descriptive names with clear purpose
- **Test names**: snake_case BDD-style matching the runbook scenario names (e.g., `plan_end_to_end_with_mock`, `cli_flag_parity_plan`)

## Test Patterns That Worked Well

- **Process-based E2E tests**: Running the actual binaries with `Command::new(binary_path(...))` and injecting `PATH` via `.env()` provides high-fidelity testing of the real execution flow.
- **Temp git repo helper**: `create_temp_git_repo()` encapsulates all the boilerplate for `git init`, user config, initial commit, and branch creation. Each test gets an isolated repo.
- **Mock copilot that writes real files**: The mock script parses the prompt to find the output path and writes a structurally valid runbook. This means `sldo-plan`'s validation logic runs against real content.
- **Separated cleanup**: Each test cleans up its own temp directories at the end, with `let _ = fs::remove_dir_all(...)` to ignore cleanup errors.

## What the Next Milestone Should Do Differently

- All 5 milestones are now complete. No further milestones planned.
- If a future milestone were added (e.g., for CI/CD integration or performance benchmarks), it should consider:
  - Adding a `tests/fixtures/` README explaining the purpose of each fixture file
  - Consider using the `tempfile` crate instead of manual temp directory management for cleaner test isolation
  - The mock copilot approach could be extended to simulate failure modes (exit non-zero, write invalid runbooks) for negative path testing

## BDD Scenarios to Retroactively Add

- None needed for earlier milestones. The existing M1-M4 scenarios remain complete.
- The M5 integration tests provide the cross-cutting coverage that validates all previous milestones work together.
