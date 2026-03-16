# Rust Rewrite — SunLitOrchestrate

> **Purpose**: Rewrite `plan-milestones.sh` and `run-milestones.sh` from Bash into Rust, producing two CLI binaries (`sldo-plan` and `sldo-run`) in a Cargo workspace with shared infrastructure.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section **and** the Pre-Milestone Protocol. After completing it, follow the Post-Milestone Protocol. Never skip ahead.  
> **Prerequisite reading**: [README.md](../README.md), [runbook-template.md](runbook-template.md)

---

## Milestone Tracker

Update this table as each milestone is completed. This is the **single source of truth** for progress.

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Cargo workspace scaffolding | `done` | 2026-03-16 | 2026-03-16 | `docs/lessons/rust-rewrite-m1.md` |
| 2 | Shared library — CLI parsing & common infra | `done` | 2026-03-16 | 2026-03-16 | `docs/lessons/rust-rewrite-m2.md` |
| 3 | sldo-plan binary — runbook generation | `done` | 2026-03-16 | 2026-03-16 | `docs/lessons/rust-rewrite-m3.md` |
| 4 | sldo-run binary — milestone execution | `done` | 2026-03-16 | 2026-03-16 | `docs/lessons/rust-rewrite-m4.md` |
| 5 | Integration tests, docs & migration | `not_started` | | | |

<!-- Status values: not_started | in_progress | done -->
<!-- Lessons files go in docs/lessons/rust-rewrite-m<N>.md -->

---

## Pre-Milestone Protocol

**Do this before every milestone — no exceptions.**

1. **Read the lessons file from the previous milestone** (if one exists). Its path is in the Milestone Tracker table. Apply any design corrections, naming changes, or test strategy improvements it calls for before writing new code.
2. **Read the current milestone section fully** — goal, context, change set, BDD scenarios, regression tests, and smoke tests — before writing any code.
3. **Run the full existing test suite** and confirm it passes. Record the baseline:
   ```
   cargo test --workspace 2>&1 | tail -5
   ```
   If any tests fail before you start, **stop and fix them first**. Do not begin a milestone on a red baseline.
4. **Read the files listed in "Files Most Likely Touched"** for the current milestone. Understand their current shape before changing them.
5. **Update the Milestone Tracker** in this file: set the current milestone's Status to `in_progress` and record the Started date.
6. **Create BDD test files first** — write the scenario tests from the acceptance table **before** writing production code. Tests declare the contract, then implementation satisfies it.
7. **Create E2E test stubs** — write the end-to-end runtime validation tests from the milestone's "E2E Runtime Validation" section as stubs before writing production code.

---

## Post-Milestone Protocol

**Do this after every milestone — no exceptions.**

1. **Run the full test suite**. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   ```
2. **Run the E2E runtime validation tests** for this milestone:
   ```
   cargo test --workspace --test 'e2e_*'
   ```
3. **Verify the project builds**:
   ```
   cargo build --workspace
   ```
4. **Run the smoke tests** listed in the milestone. Check off each item in this runbook file.
5. **Verify backward compatibility**: The original Bash scripts (`src/plan-milestones.sh`, `src/run-milestones.sh`) must remain functional and unmodified throughout the rewrite.
6. **Update README.md** if user-facing capabilities changed, following the Documentation Update table.
7. **Write a lessons-learned file** at `docs/lessons/rust-rewrite-m<N>.md` containing:
   - What design decisions were made and why
   - What was harder than expected
   - What naming conventions were established (type names, file names, test patterns)
   - What test patterns worked well or didn't
   - What the next milestone should do differently based on what was learned
   - Any BDD scenarios that should be retroactively added to earlier milestones
8. **Update the Milestone Tracker** in this file: set Status to `done`, record the Completed date, and fill in the Lessons File path.
9. **Re-read the next milestone's section** with fresh eyes, and note in the lessons file whether any of its assumptions need to change.

---

## Background Context

### Current State

SunLitOrchestrate consists of two Bash scripts and a runbook template:

- **`src/plan-milestones.sh`** (~310 lines) — Generates a milestone-based runbook by invoking `copilot` CLI with a carefully constructed prompt. Features: argument parsing, template reading, iterative refinement (up to N passes), runbook validation (section checks, placeholder detection, milestone counting), coloured terminal output, logging.
- **`src/run-milestones.sh`** (~370 lines) — Executes runbook milestones by invoking `copilot` CLI in a loop. Features: argument parsing, auto-detection of build/test commands (Cargo, npm/pnpm/yarn, Python, Go, Make), milestone tracker state parsing, build/test verification, retry with context, coloured output, logging.
- **`docs/runbook-template.md`** — The markdown template structure both scripts reference.
- **`.gitignore`** already includes Rust/Cargo patterns (`target/`, `debug/`, `*.rs.bk`).

Both scripts share significant common functionality:
- Coloured terminal output helpers (`info`, `success`, `warn`, `fail`, `header`, `divider`)
- Timestamp formatting (`ts`)
- Argument parsing with `--help`
- Pre-flight checks (copilot CLI existence, git branch safety, file existence)
- Copilot CLI invocation with `--allow-tool` / `--deny-tool` flags
- Log directory management
- Prompt construction from templates

### Problem

1. **Fragile string manipulation**: Bash scripts rely on `grep`, `sed`, `awk`, and regex to parse milestone tracker tables. Rust's type system and proper markdown parsing would be more robust.
2. **No type safety**: Argument parsing, path resolution, and command assembly are all untyped strings prone to quoting bugs and edge cases.
3. **Limited error handling**: Bash `set -euo pipefail` is a blunt instrument. Rust's `Result` type provides granular, composable error handling.
4. **Difficult to test**: The Bash scripts have zero tests. Decomposing into Rust functions with unit tests will increase reliability.
5. **Platform portability**: Bash scripts depend on GNU coreutils, `tee`, `grep -E`, and other tools that behave differently across macOS and Linux. Rust compiles to a single static binary per platform.
6. **Shared code duplication**: Both scripts duplicate ~100 lines of identical helper functions. A shared library crate eliminates this.

### Target Architecture

```
SunLitOrchestrate/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── sldo-common/        # Shared library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── cli.rs          # Common CLI arg types
│   │       ├── color.rs        # Coloured output helpers
│   │       ├── copilot.rs      # Copilot CLI invocation
│   │       ├── git.rs          # Git branch checks
│   │       ├── logging.rs      # Timestamped logging
│   │       ├── preflight.rs    # Pre-flight validation
│   │       ├── runbook.rs      # Runbook parsing & milestone tracking
│   │       └── toolflags.rs    # --allow-tool / --deny-tool flag sets
│   ├── sldo-plan/          # Binary: runbook generation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── sldo-run/           # Binary: milestone execution
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── src/                    # Original Bash scripts (kept)
│   ├── plan-milestones.sh
│   └── run-milestones.sh
├── docs/
│   ├── runbook-template.md
│   └── lessons/
└── tests/                  # Workspace-level integration tests
    ├── e2e_scaffold_m1.rs
    ├── e2e_common_m2.rs
    ├── e2e_plan_m3.rs
    ├── e2e_run_m4.rs
    └── e2e_integration_m5.rs
```

### Key Design Principles

1. **Feature parity with Bash**: Every user-visible behavior of the Bash scripts must be replicated. Same CLI flags, same defaults, same output format, same safety checks.
2. **Shared library, separate binaries**: Common code lives in `sldo-common`. Each binary is thin — it parses its specific args and delegates to library functions.
3. **Structured error handling**: Use `anyhow` for application errors and `thiserror` for library error types. No `unwrap()` in production code.
4. **Test-first**: BDD tests written before production code for every milestone.
5. **Preserve original scripts**: The Bash scripts remain in `src/` and are not modified. Users can choose either implementation during the transition.
6. **Cross-platform**: Avoid shelling out to Unix-specific utilities. Use Rust-native path handling, file I/O, and process management.

### What to Keep

- `src/plan-milestones.sh` — original script, unmodified
- `src/run-milestones.sh` — original script, unmodified
- `docs/runbook-template.md` — template consumed by both implementations
- `README.md` — updated but not removed
- `.gitignore` — already includes Rust patterns

### What to Change

- **`Cargo.toml`** (workspace root) — NEW: workspace definition
- **`crates/sldo-common/`** — NEW: shared library crate
- **`crates/sldo-plan/`** — NEW: plan-milestones binary
- **`crates/sldo-run/`** — NEW: run-milestones binary
- **`tests/`** — NEW: workspace-level E2E tests
- **`README.md`** — updated to document both Bash and Rust usage

---

## BDD Practices

Every milestone follows these rules. Apply them consistently.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first — `#[test]` modules for unit tests, integration test files for E2E.
3. Confirm the tests fail (they reference types/functions that don't exist yet).
4. Write the production code to make the tests pass.
5. Refactor if needed, re-run tests to confirm green.

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition]
    // When: [action]
    // Then: [expected outcome]
}
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Unit tests | `#[cfg(test)] mod tests` inside the source file | Same file as production code |
| Integration/BDD tests | `tests/<crate>_<feature>.rs` | Within each crate's `tests/` or workspace `tests/` |
| E2E runtime validation | `tests/e2e_<prefix>_m<N>.rs` | Workspace root `tests/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that verify:
1. The workspace builds without errors.
2. Binaries produce expected output for basic invocations (`--help`, missing args).
3. Library functions handle real-world inputs correctly (actual markdown parsing, real git repos).
4. No panics under realistic workloads.

---

## Milestone Plan

---

### Milestone 1 — Cargo Workspace Scaffolding

**Goal**: Set up the Cargo workspace with three crates (`sldo-common`, `sldo-plan`, `sldo-run`) so that `cargo build --workspace` and `cargo test --workspace` succeed with minimal placeholder code.

**Context**: The repo currently has no Rust code. The `.gitignore` already includes `target/` and `debug/`. We need the workspace structure in place before writing any real logic. The two binaries will depend on `sldo-common`.

**Important design rule**: Keep crate names hyphenated (`sldo-common`, `sldo-plan`, `sldo-run`) matching the binary names. The library crate's Rust module name will be `sldo_common` (underscored, per Rust convention).

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. Read these files before making changes:
   - `README.md` — understand current project description
   - `.gitignore` — confirm Rust patterns are present
   - `docs/runbook-template.md` — understand the template structure the tools consume

#### Files Most Likely Touched

| File | Change |
|---|---|
| `Cargo.toml` | NEW: workspace root manifest |
| `crates/sldo-common/Cargo.toml` | NEW: library crate manifest |
| `crates/sldo-common/src/lib.rs` | NEW: library entry point with placeholder |
| `crates/sldo-plan/Cargo.toml` | NEW: binary crate manifest, depends on sldo-common |
| `crates/sldo-plan/src/main.rs` | NEW: binary entry point with placeholder |
| `crates/sldo-run/Cargo.toml` | NEW: binary crate manifest, depends on sldo-common |
| `crates/sldo-run/src/main.rs` | NEW: binary entry point with placeholder |

#### Step-by-Step

1. **Write BDD test stubs first** — create `crates/sldo-common/src/lib.rs` with a `#[cfg(test)] mod tests` block containing a placeholder test.
2. **Create the workspace root `Cargo.toml`**:
   - Define `[workspace]` with members: `crates/sldo-common`, `crates/sldo-plan`, `crates/sldo-run`.
   - Set `resolver = "2"`.
   - Define shared `[workspace.dependencies]` for: `clap` (with `derive` feature), `anyhow`, `thiserror`, `colored`, `regex`.
3. **Create `crates/sldo-common/Cargo.toml`**:
   - `[lib]` crate.
   - Depend on workspace dependencies: `anyhow`, `thiserror`, `regex`, `colored`.
4. **Create `crates/sldo-common/src/lib.rs`**:
   - Declare future modules as comments.
   - Export a `pub fn version() -> &'static str` returning the crate version.
   - Add a `#[cfg(test)]` module with a test that calls `version()`.
5. **Create `crates/sldo-plan/Cargo.toml`**:
   - `[[bin]]` crate named `sldo-plan`.
   - Depend on `sldo-common` (path) and `clap`, `anyhow` from workspace.
6. **Create `crates/sldo-plan/src/main.rs`**:
   - `fn main()` that prints `"sldo-plan: not yet implemented"` and exits 0.
7. **Create `crates/sldo-run/Cargo.toml`**:
   - `[[bin]]` crate named `sldo-run`.
   - Depend on `sldo-common` (path) and `clap`, `anyhow` from workspace.
8. **Create `crates/sldo-run/src/main.rs`**:
   - `fn main()` that prints `"sldo-run: not yet implemented"` and exits 0.
9. **Run `cargo build --workspace`** — must succeed.
10. **Run `cargo test --workspace`** — must succeed.
11. **Run `cargo clippy --workspace`** — fix any warnings.

#### BDD Acceptance Scenarios

**Feature: Workspace builds**

| Scenario | Given | When | Then |
|---|---|---|---|
| Workspace compiles | A fresh clone of the repo | `cargo build --workspace` is run | Exit code 0, `sldo-plan` and `sldo-run` binaries exist in `target/debug/` |
| Tests pass | A fresh clone of the repo | `cargo test --workspace` is run | Exit code 0, at least 1 test passes |
| Plan binary runs | Binaries are built | `./target/debug/sldo-plan` is executed | Prints placeholder message, exits 0 |
| Run binary runs | Binaries are built | `./target/debug/sldo-run` is executed | Prints placeholder message, exits 0 |

#### Regression Tests

- No pre-existing Rust tests (this is the first milestone).
- Original Bash scripts must still be present and unchanged in `src/`.

#### E2E Runtime Validation

**File**: `tests/e2e_scaffold_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `workspace_builds` | Cargo workspace compiles without errors | `cargo build --workspace` returns exit code 0 |
| `plan_binary_runs` | sldo-plan binary executes | Process exits 0 with non-empty stdout |
| `run_binary_runs` | sldo-run binary executes | Process exits 0 with non-empty stdout |
| `common_version_exists` | Library crate exports version | `sldo_common::version()` returns a non-empty string |

#### Smoke Tests

- [x] `cargo build --workspace` succeeds
- [x] `cargo test --workspace` succeeds
- [x] `./target/debug/sldo-plan` prints a message and exits 0
- [x] `./target/debug/sldo-run` prints a message and exits 0
- [x] `src/plan-milestones.sh --help` still works (original script untouched)
- [x] `src/run-milestones.sh --help` still works (original script untouched)

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Add a "Rust (in progress)" section noting the workspace structure.

---

### Milestone 2 — Shared Library: CLI Parsing & Common Infrastructure

**Goal**: Implement all shared functionality in `sldo-common`: coloured output, timestamped logging, git safety checks, copilot CLI invocation, tool-flag definitions, and runbook/milestone-tracker parsing.

**Context**: Both Bash scripts share ~100 lines of identical helper functions (colour codes, `ts()`, `info()`, `success()`, `warn()`, `fail()`, `header()`, `divider()`), plus common preflight logic (copilot exists, git branch not main/master, file exists) and copilot invocation patterns. The runbook tracker table parsing (`grep -E '^\| [0-9]+ \|'`) is also shared. All of this belongs in the library crate.

**Important design rule**: Every public function in `sldo-common` must have at least one unit test. Use `colored` crate for terminal colours. Use `std::process::Command` for subprocess invocation — no shell wrapper.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/rust-rewrite-m1.md`** — apply any corrections from Milestone 1.
3. Read these files before making changes:
   - `src/plan-milestones.sh` lines 63–93 — colour and helper functions
   - `src/plan-milestones.sh` lines 135–195 — preflight checks
   - `src/run-milestones.sh` lines 37–87 — ALLOW_FLAGS / DENY_FLAGS
   - `src/run-milestones.sh` lines 246–276 — `all_milestones_done()`, `current_milestone_number()`, `current_milestone_title()`
   - `crates/sldo-common/src/lib.rs` — current placeholder

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-common/Cargo.toml` | Add `chrono` dependency for timestamps |
| `crates/sldo-common/src/lib.rs` | Declare and re-export modules |
| `crates/sldo-common/src/color.rs` | NEW: `info()`, `success()`, `warn()`, `fail()`, `header()`, `divider()` |
| `crates/sldo-common/src/logging.rs` | NEW: timestamped logging, log-file writing |
| `crates/sldo-common/src/git.rs` | NEW: `is_git_repo()`, `current_branch()`, `is_protected_branch()` |
| `crates/sldo-common/src/copilot.rs` | NEW: `invoke_copilot()` — build and exec `copilot -p <prompt> --model <model> ...` |
| `crates/sldo-common/src/toolflags.rs` | NEW: `plan_allow_flags()`, `plan_deny_flags()`, `run_allow_flags()`, `run_deny_flags()` |
| `crates/sldo-common/src/runbook.rs` | NEW: `MilestoneStatus`, `MilestoneRow`, `parse_tracker()`, `all_done()`, `next_incomplete()` |
| `crates/sldo-common/src/preflight.rs` | NEW: `check_copilot_installed()`, `check_file_exists()`, `check_git_branch()` |
| `crates/sldo-common/src/detect.rs` | NEW: `detect_build_commands()`, `detect_test_commands()` for auto-detection |

#### Step-by-Step

1. **Write BDD test stubs first** for every module listed below. Each module gets a `#[cfg(test)] mod tests` block with stub tests that reference the types/functions to be created.
2. **Implement `color.rs`**:
   - Functions: `ts() -> String`, `info(msg)`, `success(msg)`, `warn(msg)`, `fail(msg)`, `header(msg)`, `divider()`.
   - Use the `colored` crate. Match Bash colour codes: blue=info, green=success, yellow=warn, red=fail, cyan=header/divider.
   - Each function prints to stderr (like the Bash scripts use `echo -e`).
3. **Implement `logging.rs`**:
   - `LogFile` struct wrapping a `PathBuf`.
   - `LogFile::new(dir, filename)` — creates parent dirs.
   - `LogFile::append(line)` — appends timestamped line.
   - `ensure_log_dir(project_dir) -> PathBuf` — creates `.copilot-logs/` under project dir.
4. **Implement `git.rs`**:
   - `is_git_repo(path) -> bool` — runs `git -C <path> rev-parse --is-inside-work-tree`.
   - `current_branch(path) -> Result<String>` — runs `git -C <path> rev-parse --abbrev-ref HEAD`.
   - `is_protected_branch(branch) -> bool` — returns true if branch is `main` or `master`.
5. **Implement `preflight.rs`**:
   - `check_copilot_installed() -> Result<PathBuf>` — uses `which::which("copilot")` or falls back to `command -v` equivalent.
   - `check_file_exists(path, label) -> Result<()>`.
   - `check_git_safety(repo_dir) -> Result<String>` — combines git.rs calls, returns branch name or error.
6. **Implement `toolflags.rs`**:
   - `fn plan_allow_flags() -> Vec<String>` — returns the ALLOW_FLAGS from `plan-milestones.sh`.
   - `fn plan_deny_flags() -> Vec<String>` — returns the DENY_FLAGS.
   - `fn run_allow_flags() -> Vec<String>` — returns the ALLOW_FLAGS from `run-milestones.sh`.
   - `fn run_deny_flags() -> Vec<String>` — returns the DENY_FLAGS.
7. **Implement `copilot.rs`**:
   - `CopilotInvocation` struct: `prompt: String`, `model: String`, `allow_flags: Vec<String>`, `deny_flags: Vec<String>`, `working_dir: PathBuf`.
   - `CopilotInvocation::run(&self, log_file: &LogFile) -> Result<i32>` — spawns `copilot` process, pipes stdout/stderr to both the terminal and the log file. Returns exit code.
8. **Implement `runbook.rs`**:
   - `enum MilestoneStatus { NotStarted, InProgress, Done }` with `Display` and `FromStr`.
   - `struct MilestoneRow { number: u32, title: String, status: MilestoneStatus, started: Option<String>, completed: Option<String>, lessons_file: Option<String> }`.
   - `parse_tracker(runbook_content: &str) -> Vec<MilestoneRow>` — parses the Milestone Tracker table. Matches rows like `| 1 | Title | \`not_started\` | | | |`.
   - `all_done(rows: &[MilestoneRow]) -> bool`.
   - `next_incomplete(rows: &[MilestoneRow]) -> Option<&MilestoneRow>`.
9. **Implement `detect.rs`**:
   - `detect_build_commands(project_dir: &Path) -> Vec<String>` — checks for Cargo.toml, package.json, go.mod, Makefile (mirrors Bash logic).
   - `detect_test_commands(project_dir: &Path) -> Vec<String>` — same approach.
10. **Update `lib.rs`** to declare and re-export all modules.
11. **Make all BDD tests pass.**
12. **Run `cargo test --workspace`** and `cargo clippy --workspace`.

#### BDD Acceptance Scenarios

**Feature: Coloured output**

| Scenario | Given | When | Then |
|---|---|---|---|
| Timestamp format | Current time is known | `ts()` is called | Returns string in `YYYY-MM-DD HH:MM:SS` format |
| Info prints to stderr | A message string | `info("hello")` is called | stderr contains `[timestamp] ℹ  hello` |

**Feature: Git safety**

| Scenario | Given | When | Then |
|---|---|---|---|
| Detect git repo | A directory inside a git repository | `is_git_repo(path)` | Returns `true` |
| Detect non-git dir | A `/tmp` directory with no `.git` | `is_git_repo(path)` | Returns `false` |
| Protected branch check | Branch name is `main` | `is_protected_branch("main")` | Returns `true` |
| Safe branch check | Branch name is `feature/foo` | `is_protected_branch("feature/foo")` | Returns `false` |

**Feature: Runbook parsing**

| Scenario | Given | When | Then |
|---|---|---|---|
| Parse tracker table | Markdown with 3 milestone rows, all `not_started` | `parse_tracker(content)` | Returns 3 `MilestoneRow` structs with `NotStarted` status |
| All done detection | 3 rows all with `done` status | `all_done(rows)` | Returns `true` |
| Next incomplete | Rows 1=done, 2=not_started, 3=not_started | `next_incomplete(rows)` | Returns row 2 |
| Mixed status parsing | Row with `in_progress` status | `parse_tracker(content)` | Correctly parses as `InProgress` |

**Feature: Build/test detection**

| Scenario | Given | When | Then |
|---|---|---|---|
| Detect Cargo project | Directory with `Cargo.toml` | `detect_build_commands(dir)` | Contains `"cargo build --workspace"` |
| Detect npm project | Directory with `package.json` containing `"build"` script | `detect_build_commands(dir)` | Contains `"npm run build"` |
| No build files | Empty directory | `detect_build_commands(dir)` | Returns empty Vec |

**Feature: Tool flags**

| Scenario | Given | When | Then |
|---|---|---|---|
| Plan allow flags | N/A | `plan_allow_flags()` | Contains `"--allow-tool=write"` and `"--allow-tool=shell(cat:*)"` |
| Run allow flags | N/A | `run_allow_flags()` | Contains `"--allow-tool=shell(cargo:*)"` and `"--allow-tool=shell(rm:*)"` |
| Plan deny flags | N/A | `plan_deny_flags()` | Contains `"--deny-tool=shell(rm -rf /)"` |

#### Regression Tests

- All Milestone 1 tests must still pass.
- `sldo_common::version()` still returns a non-empty string.

#### E2E Runtime Validation

**File**: `tests/e2e_common_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `parse_real_template_tracker` | Parsing works on the actual runbook template | `parse_tracker` on `docs/runbook-template.md` content returns rows |
| `git_checks_on_own_repo` | Git functions work on a real repo | `is_git_repo(".")` returns true, `current_branch(".")` returns a non-empty string |
| `detect_commands_on_own_repo` | Detection works on SunLitOrchestrate itself | `detect_build_commands(".")` returns `["cargo build --workspace"]` |

#### Smoke Tests

- [x] `cargo test --workspace` — all tests pass
- [x] `cargo clippy --workspace` — no warnings
- [x] Each module in `sldo-common` has at least one unit test
- [x] Runbook parser correctly handles the milestone table in this very file
- [x] Tool flags match the Bash scripts exactly (diff the lists)

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: No change yet (library is internal).

---

### Milestone 3 — sldo-plan Binary: Runbook Generation

**Goal**: Implement the `sldo-plan` binary with full feature parity to `plan-milestones.sh` — argument parsing, template reading, prompt construction, iterative copilot invocation, and runbook validation.

**Context**: `plan-milestones.sh` takes a prompt file and repo directory, reads the runbook template, builds a long planning prompt, invokes `copilot` CLI up to N times with refinement, and validates the output. The shared infrastructure from Milestone 2 (`copilot.rs`, `preflight.rs`, `toolflags.rs`, `color.rs`, `logging.rs`) handles most of the cross-cutting concerns. This milestone wires them together into the planning flow.

**Important design rule**: The CLI interface must accept exactly the same flags as the Bash script: positional `<prompt-file> <repo-dir>`, plus `-o/--output`, `-m/--model`, `-n/--max-iterations`, `-h/--help`. Use `clap` derive macros for argument parsing.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/rust-rewrite-m2.md`** — apply any corrections from Milestone 2.
3. Read these files before making changes:
   - `src/plan-milestones.sh` — full script, especially `build_planning_prompt()` (lines 200–280), `validate_runbook()` (lines 283–340), and `main()` (lines 343–end)
   - `crates/sldo-plan/src/main.rs` — current placeholder
   - `crates/sldo-common/src/copilot.rs` — invocation API

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-plan/Cargo.toml` | Add any additional dependencies (e.g., `chrono`) |
| `crates/sldo-plan/src/main.rs` | Full implementation: CLI parsing, planning loop, validation |
| `crates/sldo-common/src/lib.rs` | Possibly add `pub mod template` if template reading deserves its own module |

#### Step-by-Step

1. **Write BDD test stubs first** for the acceptance scenarios below.
2. **Define CLI args** using `clap` derive:
   ```rust
   #[derive(Parser)]
   struct Cli {
       prompt_file: PathBuf,
       repo_dir: PathBuf,
       #[arg(short, long, default_value = None)]
       output: Option<PathBuf>,
       #[arg(short, long, default_value = "claude-opus-4.6")]
       model: String,
       #[arg(short = 'n', long, default_value_t = 3)]
       max_iterations: u32,
   }
   ```
3. **Implement `read_template(template_path)`**:
   - Read `docs/runbook-template.md` relative to the script/binary location.
   - Fall back to a built-in template string if file not found (matching Bash `FALLBACK_TEMPLATE`).
4. **Implement `build_planning_prompt(iteration, prompt_content, template, output_path)`**:
   - Port the heredoc from `build_planning_prompt()` in the Bash script.
   - For iteration > 1, append the refinement section.
5. **Implement `validate_runbook(path) -> Result<Vec<String>>`**:
   - Check file exists and size > 500 bytes.
   - Check required sections: "Milestone Tracker", "Pre-Milestone Protocol", "Post-Milestone Protocol", "Background Context", "Current State", "BDD Acceptance Scenarios".
   - Check milestone tracker has entries.
   - Check for unfilled template placeholders.
   - Check no milestones are marked `done`.
   - Return list of issue descriptions (empty = valid).
6. **Implement `main()` planning loop**:
   - Parse args, run preflight, resolve paths.
   - Loop up to `max_iterations`:
     - Build prompt for current iteration.
     - Invoke copilot via `CopilotInvocation`.
     - Validate runbook.
     - If valid, break. If not, sleep `COOLDOWN_SECS` and retry.
   - Print summary (milestone count, tracker state, wall time).
7. **Make all BDD tests pass.**
8. **Run `cargo test --workspace`** and `cargo clippy --workspace`.

#### BDD Acceptance Scenarios

**Feature: CLI argument parsing**

| Scenario | Given | When | Then |
|---|---|---|---|
| Help flag | N/A | `sldo-plan --help` | Prints usage and exits 0 |
| Missing args | No arguments | `sldo-plan` | Prints error and exits non-zero |
| Default output | `sldo-plan prompt.txt /tmp/repo` | Args are parsed | `output` defaults to `/tmp/repo/docs/RUNBOOK.md` |
| Custom output | `sldo-plan prompt.txt /tmp/repo -o custom.md` | Args are parsed | `output` is `/tmp/repo/custom.md` |
| Custom model | `sldo-plan prompt.txt /tmp/repo -m gpt-4` | Args are parsed | `model` is `"gpt-4"` |

**Feature: Template reading**

| Scenario | Given | When | Then |
|---|---|---|---|
| Template exists | `docs/runbook-template.md` is on disk | `read_template()` | Returns file contents containing "Milestone Tracker" |
| Template missing | Template path does not exist | `read_template()` | Returns fallback template string containing "Milestone Tracker" |

**Feature: Prompt construction**

| Scenario | Given | When | Then |
|---|---|---|---|
| First iteration | iteration=1, prompt="Add search", template="..." | `build_planning_prompt(...)` | Contains "User Requirements", "Runbook Template", "YOUR TASK", does NOT contain "REFINEMENT PASS" |
| Refinement iteration | iteration=2, output="/tmp/RUNBOOK.md" | `build_planning_prompt(...)` | Contains "REFINEMENT PASS 2" and the output path |

**Feature: Runbook validation**

| Scenario | Given | When | Then |
|---|---|---|---|
| Valid runbook | File >500 bytes with all required sections, milestones all `not_started` | `validate_runbook(path)` | Returns empty issues list |
| Missing file | Path does not exist | `validate_runbook(path)` | Returns issue "file was not created" |
| Small file | File is 100 bytes | `validate_runbook(path)` | Returns issue about suspicious size |
| Missing section | File lacks "BDD Acceptance Scenarios" | `validate_runbook(path)` | Returns issue about missing section |
| Unfilled placeholders | File contains `[description]` | `validate_runbook(path)` | Returns issue about placeholders |

#### Regression Tests

- All Milestone 1 and 2 tests must still pass.
- `sldo-run` binary still executes its placeholder.

#### E2E Runtime Validation

**File**: `tests/e2e_plan_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `plan_help_flag` | `--help` works | Process exits 0, stdout contains "Usage" |
| `plan_missing_args_exits_nonzero` | Missing args handled | Process exits non-zero, stderr contains error |
| `plan_reads_real_template` | Template reading works at runtime | Template content contains "Milestone Tracker" |
| `plan_validates_own_runbook` | Validation runs against a real file | Validation of this runbook file returns issues list (since it may have some) |

#### Smoke Tests

- [x] `cargo build --workspace` succeeds
- [x] `./target/debug/sldo-plan --help` prints usage resembling `plan-milestones.sh --help`
- [x] `./target/debug/sldo-plan` with no args exits non-zero with an error
- [x] `cargo test --workspace` — all tests pass

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Add `sldo-plan` usage section under a "Rust CLI" heading.

---

### Milestone 4 — sldo-run Binary: Milestone Execution

**Goal**: Implement the `sldo-run` binary with full feature parity to `run-milestones.sh` — argument parsing, build/test command detection, milestone-loop execution, build/test verification, retry with context, and progress reporting.

**Context**: `run-milestones.sh` loops until all milestones are `done`. Each iteration: parse the tracker to find the next incomplete milestone, build a prompt, invoke copilot, verify build+tests, sleep, repeat. It auto-detects build/test commands from project files. The shared infrastructure from Milestone 2 handles copilot invocation, runbook parsing, git checks, and output. This milestone wires them into the execution loop.

**Important design rule**: The CLI interface must accept the same flags: positional `<runbook> <repo-dir>`, plus `-m/--model`, `-a/--max-attempts`, `-c/--cooldown`, `--build-cmd` (repeatable), `--test-cmd` (repeatable), `-h/--help`.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/rust-rewrite-m3.md`** — apply any corrections from Milestone 3.
3. Read these files before making changes:
   - `src/run-milestones.sh` — full script, especially `build_prompt()` (lines 195–245), `main()` loop (lines 300–end), and `verify_commands()` (lines 290–305)
   - `crates/sldo-run/src/main.rs` — current placeholder
   - `crates/sldo-common/src/runbook.rs` — milestone parsing API
   - `crates/sldo-common/src/detect.rs` — command detection API

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-run/Cargo.toml` | Add any additional dependencies |
| `crates/sldo-run/src/main.rs` | Full implementation: CLI parsing, execution loop, verification |

#### Step-by-Step

1. **Write BDD test stubs first** for the acceptance scenarios below.
2. **Define CLI args** using `clap` derive:
   ```rust
   #[derive(Parser)]
   struct Cli {
       runbook: PathBuf,
       repo_dir: PathBuf,
       #[arg(short, long, default_value = "claude-opus-4.6")]
       model: String,
       #[arg(short = 'a', long, default_value_t = 150)]
       max_attempts: u32,
       #[arg(short = 'c', long, default_value_t = 5)]
       cooldown: u64,
       #[arg(long = "build-cmd", action = clap::ArgAction::Append)]
       build_cmds: Vec<String>,
       #[arg(long = "test-cmd", action = clap::ArgAction::Append)]
       test_cmds: Vec<String>,
   }
   ```
3. **Implement `build_execution_prompt(runbook_path, build_cmds, test_cmds, attempt)`**:
   - Port the heredoc from `build_prompt()` in the Bash script.
   - Include the build/test command lists in the prompt.
   - For attempt > 1, append the RETRY CONTEXT section.
4. **Implement `verify_commands(label, cmds, log_file) -> bool`**:
   - Run each command via `std::process::Command`.
   - Capture and log output.
   - Return true if all succeed.
5. **Implement `main()` execution loop**:
   - Parse args, run preflight, resolve runbook path.
   - Detect or use provided build/test commands.
   - Loop up to `max_attempts`:
     - Read runbook, parse tracker.
     - If `all_done()`, break with success.
     - Get `next_incomplete()` milestone.
     - Build prompt.
     - Invoke copilot.
     - Verify build + test commands.
     - Print progress (tracker state with coloured output).
     - Sleep `cooldown` seconds.
   - Print final tracker state and summary.
6. **Make all BDD tests pass.**
7. **Run `cargo test --workspace`** and `cargo clippy --workspace`.

#### BDD Acceptance Scenarios

**Feature: CLI argument parsing**

| Scenario | Given | When | Then |
|---|---|---|---|
| Help flag | N/A | `sldo-run --help` | Prints usage and exits 0 |
| Missing args | No arguments | `sldo-run` | Prints error and exits non-zero |
| Default model | `sldo-run runbook.md /tmp/repo` | Args parsed | `model` is `"claude-opus-4.6"` |
| Custom max attempts | `sldo-run rb.md . -a 50` | Args parsed | `max_attempts` is 50 |
| Multiple build cmds | `sldo-run rb.md . --build-cmd "make" --build-cmd "npm run build"` | Args parsed | `build_cmds` has 2 entries |

**Feature: Prompt construction**

| Scenario | Given | When | Then |
|---|---|---|---|
| First attempt prompt | attempt=1, runbook path="docs/RUNBOOK.md" | `build_execution_prompt(...)` | Contains "Read the runbook", build/test commands, does NOT contain "RETRY CONTEXT" |
| Retry prompt | attempt=3 | `build_execution_prompt(...)` | Contains "RETRY CONTEXT — Attempt 3" |

**Feature: Command verification**

| Scenario | Given | When | Then |
|---|---|---|---|
| Successful command | Command is `true` | `verify_commands("test", ["true"], log)` | Returns `true` |
| Failed command | Command is `false` | `verify_commands("test", ["false"], log)` | Returns `false` |
| Empty command list | No commands | `verify_commands("test", [], log)` | Returns `true` (vacuously) |

#### Regression Tests

- All Milestone 1, 2, and 3 tests must still pass.
- `sldo-plan --help` still works.

#### E2E Runtime Validation

**File**: `tests/e2e_run_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `run_help_flag` | `--help` works | Process exits 0, stdout contains "Usage" |
| `run_missing_args_exits_nonzero` | Missing args handled | Process exits non-zero |
| `run_detects_cargo_in_own_repo` | Auto-detection works at runtime | Build commands include `cargo build --workspace` |
| `run_parses_tracker_from_real_runbook` | Tracker parsing works on real input | Parses this runbook file and finds 5 milestones |

#### Smoke Tests

- [x] `cargo build --workspace` succeeds
- [x] `./target/debug/sldo-run --help` prints usage resembling `run-milestones.sh --help`
- [x] `./target/debug/sldo-run` with no args exits non-zero
- [x] `cargo test --workspace` — all tests pass
- [x] Build/test command auto-detection returns sensible results on this repo

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Add `sldo-run` usage section under the "Rust CLI" heading.

---

### Milestone 5 — Integration Tests, Documentation & Migration

**Goal**: Add end-to-end integration tests that exercise the full plan and run workflows (with a mock copilot), update all documentation, and provide a migration guide from Bash to Rust.

**Context**: Milestones 1–4 built the working binaries. This milestone ensures they work correctly end-to-end, documents the Rust CLI fully in the README, and provides guidance for transitioning from the Bash scripts. The mock copilot approach lets us test the full flow without requiring actual API access.

**Important design rule**: Integration tests must not require a real Copilot CLI or network access. Use a mock script that simulates copilot behavior (creates a runbook file, prints expected output).

#### Pre-Flight

1. Complete the Pre-Milestone Protocol above.
2. **Read `docs/lessons/rust-rewrite-m4.md`** — apply any corrections from Milestone 4.
3. Read these files before making changes:
   - `README.md` — current state
   - `crates/sldo-plan/src/main.rs` — plan binary
   - `crates/sldo-run/src/main.rs` — run binary
   - `src/plan-milestones.sh` — reference for flag parity comparison
   - `src/run-milestones.sh` — reference for flag parity comparison

#### Files Most Likely Touched

| File | Change |
|---|---|
| `tests/e2e_integration_m5.rs` | NEW: full-flow integration tests with mock copilot |
| `tests/fixtures/mock-copilot.sh` | NEW: mock copilot script for test use |
| `tests/fixtures/sample-prompt.txt` | NEW: sample prompt file for tests |
| `tests/fixtures/sample-runbook.md` | NEW: sample runbook for run-milestones tests |
| `README.md` | Major update: Rust CLI documentation, migration guide |
| `docs/MIGRATION.md` | NEW: Bash-to-Rust migration guide |

#### Step-by-Step

1. **Write BDD test stubs first** for the integration scenarios below.
2. **Create test fixtures**:
   - `tests/fixtures/mock-copilot.sh` — a script that mimics copilot: reads `-p` prompt from args, writes a minimal valid runbook to the output file mentioned in the prompt.
   - `tests/fixtures/sample-prompt.txt` — a simple requirements file.
   - `tests/fixtures/sample-runbook.md` — a runbook with 2 milestones (one `done`, one `not_started`).
3. **Write integration tests in `tests/e2e_integration_m5.rs`**:
   - Test `sldo-plan` end-to-end: set `PATH` to include mock copilot, run `sldo-plan`, verify runbook was created.
   - Test `sldo-run` end-to-end: set `PATH` to include mock copilot, run `sldo-run` against sample runbook, verify it processes milestones.
   - Test CLI flag parity: compare `--help` output structure between Rust and Bash versions.
4. **Write `docs/MIGRATION.md`**:
   - Table mapping Bash flags to Rust flags.
   - Behavioral differences (if any).
   - Installation instructions for the Rust binaries.
5. **Update `README.md`**:
   - Add a "Rust CLI" section documenting `sldo-plan` and `sldo-run`.
   - Add installation instructions (`cargo install --path crates/sldo-plan`, etc.).
   - Mark Bash scripts as "legacy" with a note that Rust is the preferred implementation.
   - Add a link to `docs/MIGRATION.md`.
6. **Make all tests pass.**
7. **Run `cargo test --workspace`** and `cargo clippy --workspace`.
8. **Final audit**: Verify every CLI flag from both Bash scripts has a corresponding Rust flag.

#### BDD Acceptance Scenarios

**Feature: End-to-end planning**

| Scenario | Given | When | Then |
|---|---|---|---|
| Plan with mock copilot | Mock copilot on PATH, prompt file exists, temp git repo on feature branch | `sldo-plan prompt.txt /tmp/repo` | Runbook file exists at `docs/RUNBOOK.md`, contains "Milestone Tracker", exit code 0 |
| Plan refuses main branch | Repo is on `main` branch | `sldo-plan prompt.txt /tmp/repo` | Exits non-zero with error about protected branch |

**Feature: End-to-end execution**

| Scenario | Given | When | Then |
|---|---|---|---|
| Run completes milestone | Mock copilot on PATH, runbook has 1 not_started milestone | `sldo-run runbook.md /tmp/repo` | Mock copilot was invoked, exit code 0 |
| Run detects all done | Runbook has all milestones `done` | `sldo-run runbook.md /tmp/repo` | Prints "All milestones" message, exits immediately |

**Feature: CLI parity**

| Scenario | Given | When | Then |
|---|---|---|---|
| Plan help matches | Both binaries available | Compare `sldo-plan --help` with `plan-milestones.sh --help` | Same options listed (flags, defaults) |
| Run help matches | Both binaries available | Compare `sldo-run --help` with `run-milestones.sh --help` | Same options listed |

#### Regression Tests

- All tests from Milestones 1–4 must still pass.
- Original Bash scripts must still be present and functional.

#### E2E Runtime Validation

**File**: `tests/e2e_integration_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `plan_end_to_end_with_mock` | Full planning flow works | Runbook file created, validation passes |
| `run_end_to_end_with_mock` | Full execution flow works | Copilot invoked, milestone processed |
| `plan_rejects_protected_branch` | Safety guard works at runtime | Exit non-zero on main branch |
| `run_rejects_protected_branch` | Safety guard works at runtime | Exit non-zero on main branch |
| `cli_flag_parity_plan` | Rust CLI matches Bash CLI | All flags present in `--help` |
| `cli_flag_parity_run` | Rust CLI matches Bash CLI | All flags present in `--help` |

#### Smoke Tests

- [ ] `cargo test --workspace` — all tests pass (including integration tests)
- [ ] `cargo clippy --workspace` — no warnings
- [ ] `README.md` documents both Bash and Rust CLIs
- [ ] `docs/MIGRATION.md` exists and covers all flags
- [ ] Original Bash scripts unchanged: `git diff src/` shows no changes
- [ ] `cargo install --path crates/sldo-plan` and `cargo install --path crates/sldo-run` succeed

#### Post-Flight

Complete the Post-Milestone Protocol above. Key documentation updates:
- **README.md**: Full Rust CLI documentation, installation, migration link.

---

## Documentation Update Table

Track which documents need updating per milestone.

| Milestone | README.md Update | Other Docs |
|---|---|---|
| 1 | Add "Rust (in progress)" note | — |
| 2 | No change (internal library) | — |
| 3 | Add `sldo-plan` usage under Rust CLI section | — |
| 4 | Add `sldo-run` usage under Rust CLI section | — |
| 5 | Full Rust CLI docs, installation, migration link | NEW: `docs/MIGRATION.md` |
