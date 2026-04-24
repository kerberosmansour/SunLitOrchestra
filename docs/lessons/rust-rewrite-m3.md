# Lessons Learned — Milestone 3: sldo-plan Binary — Runbook Generation

## Design Decisions

- **All logic in main.rs**: Template reading, prompt construction, validation, and the planning loop all live in `crates/sldo-plan/src/main.rs`. Per M2 lessons, template reading is plan-specific and doesn't belong in `sldo-common`. This keeps the binary self-contained while delegating cross-cutting concerns (colour output, logging, preflight, copilot invocation, tool flags) to `sldo-common`.
- **Functions made `pub` for testability**: `read_template`, `build_planning_prompt`, and `validate_runbook` are `pub` functions so they can be tested from unit tests within the same file. They follow the BDD scenario tables closely.
- **Separate `run()` function**: The `main()` function delegates to `run() -> Result<()>` for structured error handling, matching the `anyhow` pattern from the design principles. Errors are reported via `fail()` to stderr and exit code 1.
- **`resolve_output_path` utility**: Output path resolution (default, relative, absolute) is extracted into a testable function. The Bash script handled this inline; the Rust version makes it explicit and tested.
- **Regex for milestone counting in validation**: Uses `regex::Regex` for milestone row detection in `validate_runbook`, matching the approach in `sldo-common::runbook::parse_tracker`. Added `regex` to `sldo-plan`'s dependencies.
- **Fallback template as const string**: The fallback template is a `const &str` in the binary, matching the Bash script's heredoc approach. It mirrors the structure of `docs/runbook-template.md`.

## What Was Harder Than Expected

- **M1 E2E test compatibility**: The M1 test `plan_binary_runs` expected `sldo-plan` with no args to exit 0 and print a message (the placeholder behavior). With clap, missing required args exits non-zero. Updated the M1 test to use `--help` instead, which still proves "the binary was built and executes" (the M1 goal) while being compatible with clap.
- **macOS quarantine**: Direct execution of `./target/debug/sldo-plan` triggers macOS permission dialogs in some environments (as M1 lessons warned). Smoke tests used `cargo run -p sldo-plan` as a workaround. E2E tests use `env!("CARGO_MANIFEST_DIR")` path resolution which works reliably.
- **Prompt construction fidelity**: Porting the Bash heredoc prompt to Rust required careful attention to maintain the exact same structure. The prompt contains embedded template/requirements content using Rust format strings rather than Bash variable expansion.

## Naming Conventions Established

- **Binary functions**: `read_template`, `build_planning_prompt`, `validate_runbook`, `resolve_output_path` — match the Bash function names where applicable
- **Constants**: `COOLDOWN_SECS`, `DEFAULT_OUTPUT_SUBPATH`, `FALLBACK_TEMPLATE`, `REQUIRED_SECTIONS`, `PLACEHOLDER_PATTERNS` — uppercase snake_case for module-level constants
- **CLI struct**: `Cli` (PascalCase per clap convention) with `clap::Parser` derive
- **Test names**: snake_case BDD-style (e.g., `first_iteration_prompt`, `missing_file_returns_issue`, `valid_runbook_returns_empty_issues`)

## Test Patterns That Worked Well

- **BDD Given/When/Then comments** — continued from M1/M2, works well.
- **Temp directory isolation** for validation tests — each test creates a unique temp dir with specific runbook content and cleans up afterward.
- **clap's `try_parse_from`** for argument parsing tests — allows testing CLI parsing without spawning a process.
- **Exhaustive validation tests** — each validation check (missing file, small file, missing section, placeholders, done milestones) has its own isolated test.
- **E2E tests via process execution** — `plan_help_flag` and `plan_missing_args_exits_nonzero` test the actual binary behavior end-to-end.

## What the Next Milestone Should Do Differently

- Milestone 4 (`sldo-run` binary) follows the same pattern: clap args, main loop, invocation. The M1 E2E test `run_binary_runs` also expects exit 0 with no args (placeholder behavior), and will need the same update to use `--help` when M4 adds clap.
- The `CopilotInvocation` struct reads stdout/stderr sequentially. For long-running copilot invocations in `sldo-run`, this may cause buffering issues. Consider spawning threads for concurrent stream reading if needed.
- The `verify_commands` function in M4 will need to run shell commands and capture output. Use `std::process::Command` with `sh -c` for complex commands or split on whitespace for simple ones.
- M4's `build_execution_prompt` is different from M3's `build_planning_prompt` — it includes build/test command lists and retry context rather than template/requirements.

## BDD Scenarios to Retroactively Add

- None needed for earlier milestones. The M1/M2 scenarios remain complete.
- The M1 E2E test update (using `--help` instead of no-args) is a natural progression, not a missing scenario.
