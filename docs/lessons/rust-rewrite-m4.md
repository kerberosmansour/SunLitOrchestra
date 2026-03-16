# Lessons Learned — Milestone 4: sldo-run Binary — Milestone Execution

## Design Decisions

- **Same pattern as sldo-plan**: The `sldo-run` binary follows the same structure as `sldo-plan` — a `Cli` struct with `clap::Parser` derive, a `run() -> Result<()>` function for error handling, and `main()` that delegates to `run()` and handles errors via `fail()` + `exit(1)`.
- **Functions made `pub` for testability**: `build_execution_prompt` and `verify_commands` are `pub` functions to allow BDD-style unit testing from within the same file. This mirrors M3's approach.
- **`verify_commands` uses `sh -c`**: For running verification commands like `cargo build --workspace`, we use `Command::new("sh").arg("-c").arg(cmd)` to handle complex commands with arguments and pipes. This matches the Bash script's `eval "${cmd}"` approach.
- **Prompt construction mirrors Bash faithfully**: The `build_execution_prompt` function generates the same structured prompt as the Bash `build_prompt()`, including the retry context section for attempts > 1. The `build_cmd_summary` helper generates the same markdown list format.
- **Tracker parsing reuses sldo-common**: The loop reads the runbook file, calls `runbook::parse_tracker`, `runbook::all_done`, and `runbook::next_incomplete` — all from the shared library. No duplication.
- **Auto-detection reuses `detect` module**: Build/test command detection delegates to `sldo_common::detect::detect_build_commands` and `detect_test_commands`.

## What Was Harder Than Expected

- **macOS quarantine on direct binary execution**: As warned in M1 and M3 lessons, `./target/debug/sldo-run` triggers macOS permission dialogs. Smoke tests use `cargo run -p sldo-run` as a workaround. E2E tests use the `env!("CARGO_MANIFEST_DIR")` path which works reliably with `Command::new`.
- **M1 E2E test update**: The `run_binary_runs` test in `e2e_scaffold_m1.rs` expected exit 0 from `sldo-run` with no args (placeholder behavior). Updated to use `--help` instead, exactly as M3 lessons predicted.

## Naming Conventions Established

- **Binary functions**: `build_execution_prompt`, `verify_commands`, `build_cmd_summary`, `print_tracker_state` — descriptive names matching the Bash function names where applicable
- **CLI struct**: `Cli` (PascalCase per clap convention) with `#[derive(Parser, Debug)]`
- **Constants**: None needed in M4 — the defaults are in clap's `default_value` annotations
- **Test names**: snake_case BDD-style (e.g., `default_model`, `first_attempt_prompt`, `successful_command`, `empty_command_list`)

## Test Patterns That Worked Well

- **clap's `try_parse_from`** for argument parsing tests — allows testing CLI parsing without spawning a process. The `Debug` derive on `Cli` is required for `unwrap_err()` in the help test.
- **BDD Given/When/Then comments** — continued from M1/M2/M3, works well for clarity.
- **Temp directory isolation** for `verify_commands` tests — each test creates a unique temp dir with a log file.
- **E2E tests via process execution** — `run_help_flag` and `run_missing_args_exits_nonzero` test the actual binary end-to-end.
- **Real file tests** — `run_parses_tracker_from_real_runbook` and `run_detects_cargo_in_own_repo` test against the actual repo state, providing confidence in real-world behavior.

## What the Next Milestone Should Do Differently

- Milestone 5 (integration tests, docs & migration) will need mock copilot fixtures. The `CopilotInvocation::run()` method spawns a real process, so mocking requires putting a mock script on PATH rather than injecting a trait. This is the approach the runbook specifies.
- The `--help` output format differs slightly between Bash (`usage()` heredoc) and Rust (clap auto-generated). M5's CLI parity tests should compare flag names/defaults rather than exact string matching.
- README updates for M4 added the `sldo-run` section. M5 should expand this with installation instructions and mark Bash scripts as legacy.

## BDD Scenarios to Retroactively Add

- None needed for earlier milestones. The existing M1/M2/M3 scenarios remain complete.
- The M1 E2E test update (using `--help` for `sldo-run`) was the expected M4 change per M3 lessons.
