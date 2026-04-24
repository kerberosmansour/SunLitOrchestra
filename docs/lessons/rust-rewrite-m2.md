# Lessons Learned — Milestone 2: Shared Library — CLI Parsing & Common Infrastructure

## Design Decisions

- **Module-per-concern structure**: Each piece of shared functionality lives in its own module (`color.rs`, `logging.rs`, `git.rs`, `preflight.rs`, `toolflags.rs`, `copilot.rs`, `runbook.rs`, `detect.rs`). This matches the Bash scripts' functional grouping and makes each module independently testable.
- **`chrono` for timestamps**: Used `chrono::Local::now()` for `ts()` rather than `std::time::SystemTime` — cleaner formatting API and matches the Bash `date "+%Y-%m-%d %H:%M:%S"` output exactly.
- **`which` crate for binary lookup**: Used `which::which("copilot")` in `preflight.rs` instead of shelling out to `command -v`. This is cross-platform and avoids subprocess overhead. Added to `[workspace.dependencies]` as the M1 lessons recommended.
- **Regex-based tracker parsing**: `runbook::parse_tracker()` uses two regexes — one for row detection (`^\|\s*(\d+)\s*\|`) and one for status extraction (`` `(not_started|in_progress|done)` ``). This mirrors the Bash `grep -E '^\| [0-9]+ \|'` pattern and correctly skips non-tracker numbered tables.
- **Tool flags as Vec<String>**: Flags are returned as `Vec<String>` rather than static arrays, making them easy to extend and pass to `Command::arg()` calls. Each flag is an exact copy of the Bash script's flag value.
- **Stderr for coloured output**: All colour helpers (`info`, `success`, `warn`, `fail`, `header`, `divider`) print to stderr, matching the Bash scripts' `echo -e` behaviour. This keeps stdout clean for structured output.

## What Was Harder Than Expected

- **Clippy strictness with `lines().flatten()`**: The initial clippy suggestion to use `.flatten()` on `BufReader::lines()` triggered a second warning about infinite loops on repeated read errors. The correct pattern is `.map_while(Result::ok)` which stops on first error. Good to know for future I/O streaming.
- **Detect module testing**: Testing `detect_build_commands` and `detect_test_commands` requires creating temporary directories with specific files (package.json, Makefile, etc.). Each test carefully creates and cleans up its own temp dir to avoid flaky interactions.

## Naming Conventions Established

- **Module files**: `crates/sldo-common/src/<module>.rs` — lowercase, single word or underscored
- **Public function names**: match Bash function names where applicable (`ts`, `info`, `success`, `warn`, `fail`, `header`, `divider`, `is_git_repo`, `current_branch`, `is_protected_branch`)
- **Struct names**: PascalCase (`LogFile`, `CopilotInvocation`, `MilestoneRow`, `MilestoneStatus`)
- **Test names**: snake_case describing the BDD scenario (e.g., `detect_git_repo`, `parse_tracker_table`, `plan_allow_flags_contains_write`)

## Test Patterns That Worked Well

- **BDD Given/When/Then comments in every test** — continued from M1, works well for readability.
- **Tests that run against the real repo** — `detect_cargo_project` and `git_checks_on_own_repo` test against the actual SunLitOrchestrate repo, catching real-world issues.
- **Temp directory isolation** — each test that needs filesystem state creates a unique temp dir and cleans it up, preventing test interference.
- **Graceful handling of optional external tools** — `check_copilot_installed` and `copilot_invocation_run_handles_missing_binary` tests work regardless of whether copilot is installed.

## What the Next Milestone Should Do Differently

- Milestone 3 (`sldo-plan` binary) will need to import many modules from `sldo-common`. The public API is now: `color::*`, `logging::*`, `git::*`, `preflight::*`, `toolflags::*`, `copilot::*`, `runbook::*`, `detect::*`.
- The `CopilotInvocation` struct currently reads stdout/stderr sequentially (stdout first, then stderr). For real copilot invocations, consider threading or async to read both streams concurrently. This may surface in M3 testing.
- The `detect` module mirrors Bash detection logic but doesn't handle all edge cases (e.g., pnpm workspaces, monorepo setups). This is fine for feature parity but may need enhancement later.
- Template reading for M3 should be in `sldo-plan/src/main.rs` or a new `template.rs` module, not in `sldo-common`, since it's specific to the plan binary.

## BDD Scenarios to Retroactively Add

- None needed for earlier milestones. The M1 scenarios remain complete.
