# sldo-research — SunLitOrchestrate (AI-First Runbook v3)

> **Purpose**: Build `sldo-research`, a CLI tool that takes a raw prompt (via file or CLI arg), performs deep research using AI agent skills and web search, and produces a structured research dossier — a set of resource material and instructions sufficient for `sldo-plan` to generate a high-quality runbook.  
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.  
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.  
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [README.md](../README.md), existing crates `sldo-plan`, `sldo-run`, `sldo-common`

---

## Runbook Metadata

- **Runbook ID**: `research`
- **Prefix for test files and lessons files**: `research`
- **Primary stack**: `Rust`
- **Primary package/app names**: `sldo-research`, `sldo-common`
- **Default test commands**:
  - Backend: `cargo test --workspace`
  - Frontend: n/a
  - E2E backend: `cargo test --test e2e_research_m* -- --test-threads=1`
  - E2E frontend: n/a
  - Build/boot: `cargo build --workspace`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `sldo-plan` CLI interface and behavior
  - `sldo-run` CLI interface and behavior
  - `sldo-common` public API (existing functions)
  - All existing Tauri commands and events

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Crate scaffolding & CLI skeleton | `not_started` | | | | |
| 2 | Research prompt builder | `not_started` | | | | |
| 3 | Claude Code-driven research loop | `not_started` | | | | |
| 4 | Dossier validation & output | `not_started` | | | | |
| 5 | Web search integration | `not_started` | | | | |
| 6 | Multi-source synthesis pass | `not_started` | | | | |
| 7 | Plan-ready output & sldo-plan integration | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/research-m<N>.md -->
<!-- Completion summaries go in docs/completion/research-m<N>.md -->

---

## End-to-End Architecture Diagram

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        sldo-research Pipeline                               │
│                                                                             │
│  ┌──────────┐    ┌──────────────────┐    ┌───────────────────┐              │
│  │  User     │───▶│  sldo-research   │───▶│  Research Dossier │             │
│  │  Prompt   │    │  CLI             │    │  (.md file)       │             │
│  │ (file/arg)│    └──────────────────┘    └───────────────────┘             │
│  └──────────┘           │    │    │              │                          │
│                         │    │    │              ▼                          │
│                         ▼    ▼    ▼       ┌───────────────┐                │
│              ┌────────┐ ┌────┐ ┌─────┐    │  sldo-plan    │                │
│              │Claude  │ │Web │ │Repo │    │  (existing)   │                │
│              │Code    │ │Srch│ │Scan │    └───────────────┘                │
│              │Research│ │API │ │     │           │                          │
│              └────────┘ └────┘ └─────┘           ▼                         │
│                  │         │      │        ┌───────────────┐               │
│                  ▼         ▼      ▼        │  Runbook.md   │               │
│              ┌───────────────────────┐     └───────────────┘               │
│              │  .claude-logs/        │                                      │
│              │  (research logs)      │                                      │
│              └───────────────────────┘                                      │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────┐               │
│  │                    sldo-common (shared)                   │              │
│  │  copilot.rs │ detect.rs │ git.rs │ logging.rs │ color.rs │              │
│  │  preflight.rs │ toolflags.rs │ runbook.rs               │              │
│  └──────────────────────────────────────────────────────────┘               │
│                                                                             │
│  Legend:                                                                    │
│  ─── existing    - - - new    ═══ external    ▶ data flow                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `sldo-research` CLI | Parse prompt, orchestrate research, produce dossier | M1 (scaffold), M2-M7 (features) | CLI args, dossier output file |
| Research prompt builder | Construct Claude Code prompts for research phases | M2 | `build_research_prompt()` |
| Claude Code research loop | Invoke Claude Code CLI iteratively to gather information | M3 | `ClaudeInvocation` from `sldo-common` |
| Web search integration | Search the web for API docs, libraries, best practices | M5 | `build_websearch_prompt()` |
| Dossier validator | Check dossier completeness and structure | M4 | `validate_dossier()` |
| Synthesis pass | Merge multi-source findings into coherent dossier | M6 | `build_synthesis_prompt()` |
| Plan integration | Output format compatible with `sldo-plan` input | M7 | Dossier file format |
| `sldo-common` | Shared copilot invocation, logging, color, preflight | Existing (minor additions in M1, M5) | `toolflags::research_allow_flags()` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| User prompt | User (file/CLI) | `sldo-research` | File read / CLI arg | M1 |
| Research prompt | `sldo-research` | Claude Code CLI | `ClaudeInvocation` | M2, M3 |
| Claude Code output | Claude Code CLI | Research collector | stdout line-by-line | M3 |
| Web search prompt | `sldo-research` | Claude Code CLI | `ClaudeInvocation` with web tools | M5 |
| Raw findings | Research collector | Synthesis pass | In-memory / temp file | M6 |
| Dossier output | Synthesis pass | Dossier file (.md) | File write | M4, M6 |
| Dossier → plan | Dossier file | `sldo-plan` (prompt_file arg) | File path | M7 |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — This system is a sequential CLI pipeline with no concurrency, distributed state, or resource ownership concerns. Each phase completes before the next begins. The Claude Code CLI invocations are sequential and blocking. No formal verification is warranted.

---

## Global Execution Rules

These rules apply to every milestone without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone unless a listed step explicitly requires one additional file.
- Do not refactor unrelated code.
- Do not rename public APIs, commands, routes, events, persisted state shapes, or config keys unless the milestone explicitly says so.
- Do not introduce a new dependency unless the milestone explicitly allows it.
- Do not change database schema, file formats, or migration behavior unless the milestone explicitly includes migration work and migration tests.

### 2) Tests define the contract

- Write BDD tests before production code.
- Write E2E runtime validation stubs before production code.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when code compiles. It is done when the declared contract is satisfied and evidence is recorded.

### 3) No placeholders in production paths

The following are not allowed unless explicitly permitted in the milestone:

- TODO or placeholder logic in production code
- silent fallbacks that hide errors
- swallowed errors without structured logging or user-visible handling
- fake implementations left in place after tests pass
- commented-out dead code
- temporary mocks in production paths
- hard-coded secrets, test keys, or unsafe defaults

### 4) Preserve backwards compatibility

Every milestone must explicitly verify that previously working user flows, commands, routes, persisted state, and public interfaces still work unless the milestone explicitly replaces them.

### 5) Prefer smallest safe change

- Prefer narrow, local modifications over broad rewrites.
- Prefer extending existing patterns over inventing new abstractions.
- Prefer deleting complexity over adding new layers.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

All meaningful checks must be recorded in the milestone Evidence Log:

- command run
- relevant file or test
- expected result
- actual result
- pass/fail
- notes

### 7) Keep .gitignore current and clean up test artifacts

- If a milestone introduces new build outputs, generated files, test fixtures, scratch directories, or tool-specific caches, add matching patterns to `.gitignore` before committing.
- Review `.gitignore` at the end of every milestone for staleness — remove patterns that no longer apply.
- Never commit test output data, temporary fixtures, scratch files, or generated artifacts to source control.
- Every test that creates files on disk must clean up after itself (use `tempdir`, `tempfile`, `afterEach` cleanup, or equivalent). Tests must not leave residual data in the working tree.
- Record the `.gitignore` review in the Evidence Log.

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, test strategy improvements, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   cargo test --workspace
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create BDD test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   cargo test --test e2e_research_m* -- --test-threads=1
   ```
3. Verify the app builds and boots to a usable state.
   ```
   cargo build --workspace
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. **Clean up test artifacts**: Verify no test output files, temporary fixtures, or generated data remain in the working tree. Run `git status` and confirm no untracked test artifacts exist.
8. **Review .gitignore**: Ensure any new build outputs, generated files, or tool caches introduced in this milestone have matching `.gitignore` patterns. Remove stale patterns that no longer apply.
9. Update ARCHITECTURE.md following the Documentation Update Table.
10. Update README.md if user-facing capabilities changed.
11. Write a lessons-learned file at `docs/lessons/research-m<N>.md`.
12. Write a completion summary at `docs/completion/research-m<N>.md`.
13. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
14. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

SunLitOrchestrate has two CLI tools forming a pipeline:

1. **`sldo-plan`** (`crates/sldo-plan/src/main.rs`) — Takes a prompt file + repo dir, invokes Claude Code CLI to explore the repo and generate a runbook. Uses iterative refinement (default 3 passes). Validates the output against required sections and placeholder patterns.

2. **`sldo-run`** (`crates/sldo-run/src/main.rs`) — Takes a runbook + repo dir, drives Claude Code through milestones one at a time. Verifies build/tests after each. Loops until all milestones are `done`.

3. **`sldo-common`** (`crates/sldo-common/src/`) — Shared library providing `ClaudeInvocation`, logging, color output, git safety, preflight checks, build/test command detection, and tool permission flags.

4. **`run-milestones.sh`** (`src/run-milestones.sh`) — Bash predecessor of `sldo-run`, still functional.

The current `sldo-plan` takes a raw user prompt and expects it to be detailed enough for Claude Code to produce a good runbook. There is no intermediate research step.

### Problem

1. **Raw prompts lack research depth**: When a user writes "add OAuth2 to the API", `sldo-plan` must figure out OAuth2 best practices, library choices, and integration patterns all during planning. This overloads a single planning session and produces vague milestones.

2. **No web/API documentation gathering**: The planning agent has no explicit phase for fetching current API docs, library documentation, or community best practices. It relies entirely on its training data which may be outdated.

3. **No structured research output**: There is no intermediate artifact between "user idea" and "runbook" that captures research findings, library evaluations, architecture options, and design decisions. This makes it hard to review the research before committing to a plan.

4. **Repo context is gathered during planning, not before**: The planner explores the repo and researches external topics simultaneously. Separating these concerns would produce better plans.

### Target Architecture

```
User Prompt ──▶ sldo-research ──▶ Research Dossier ──▶ sldo-plan ──▶ Runbook ──▶ sldo-run
                     │                    │
                     ├── Claude Code CLI      ├── Topic summary
                     ├── Web search       ├── Library evaluations
                     └── Repo scan        ├── Architecture options
                                          ├── API/SDK docs excerpts
                                          ├── Design recommendations
                                          ├── Repo context summary
                                          └── Constraints & risks
```

### Key Design Principles

1. **Follow existing patterns**: `sldo-research` must follow the exact same patterns as `sldo-plan` and `sldo-run` — same CLI structure (clap derive), same use of `sldo-common`, same logging, same preflight checks, same Claude Code invocation pattern.

2. **Dossier is a first-class artifact**: The research dossier is a structured markdown file that can be (a) reviewed by a human, (b) fed directly to `sldo-plan` as a prompt file, or (c) stored for reference. It is not a throwaway intermediate.

3. **Iterative deepening**: Like `sldo-plan`'s refinement passes, research should use multiple Claude Code invocations — first for broad topic exploration, then for specific deep-dives, then for synthesis. Each pass builds on the previous output.

4. **Web search is a Claude Code tool, not a custom integration**: Rather than implementing web search directly, we instruct Claude Code CLI to use its built-in `WebFetch` and `WebSearch` tools via `--allowedTools`. This keeps the implementation simple and leverages Claude Code's existing capabilities.

5. **Separation of concerns**: Research is about *gathering and organizing information*. Planning is about *decomposing work into milestones*. Research output should be information-rich but NOT contain milestone plans.

### What to Keep

- `sldo-plan` CLI interface and all behavior — unchanged
- `sldo-run` CLI interface and all behavior — unchanged
- `sldo-common` existing public API — only additive changes
- All existing tests (290+ tests)
- All Tauri desktop app functionality
- All existing runbooks and templates

### What to Change

- **`Cargo.toml` (workspace)** — Add `crates/sldo-research` to workspace members
- **`crates/sldo-research/`** — NEW: entire crate
- **`crates/sldo-common/src/toolflags.rs`** — Add `research_allow_flags()` and `research_deny_flags()`
- **`docs/ARCHITECTURE.md`** — Document the new crate and pipeline
- **`README.md`** — Add `sldo-research` CLI documentation
- **`Makefile`** — No changes needed (workspace build/test already covers new crate)

### Global Red Lines

These are forbidden unless explicitly overridden inside a milestone.

- No unrelated refactors
- No new dependencies (beyond what milestones explicitly allow)
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control
- No test output data committed to source control
- No changes to `sldo-plan`, `sldo-run`, or `sldo-tauri` behavior

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first.
3. Confirm the tests fail for the expected reason.
4. Write production code to make the tests pass.
5. Re-run tests after any refactor.

### Required Test Coverage Categories

Every milestone must explicitly cover the categories that apply:

- happy path
- invalid input
- empty state / first-run state
- dependency failure / partial failure
- retry or rollback behavior if relevant
- concurrency or race behavior if relevant
- persistence / restore behavior if relevant
- backward compatibility behavior

If a category does not apply, state why.

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
| Backend unit tests | `#[cfg(test)] mod tests` inside the source file | Same file as production code |
| Backend integration/BDD tests | inline `#[cfg(test)] mod tests` or `tests/` | `crates/sldo-research/src/` or `tests/` |
| E2E runtime validation | `tests/e2e_research_m<N>.rs` | Workspace root `tests/` |

### Test Artifact Cleanup Rules

Every test that creates files, directories, or temporary data on disk must follow these rules:

1. **Use temporary directories**: Prefer `tempdir()`, `tempfile::TempDir`, or OS-provided temp locations. Never write test output into the source tree.
2. **Clean up on completion and failure**: Use RAII patterns (Rust `Drop`) to ensure cleanup runs even when tests fail.
3. **No residual state**: After the full test suite runs, `git status` must show no untracked files from test execution.
4. **Dedicated output directories**: If a test must write to a project-relative path (e.g., `output/`), that directory must be in `.gitignore` and tests must clean it between runs.
5. **CI parity**: Test cleanup behavior must be identical locally and in CI.

### End-to-End Runtime Validation

Every milestone must include E2E tests that go beyond compilation and verify that the system works correctly at runtime. These tests prove:

1. the binary boots without errors
2. runtime contracts are met
3. BDD scenarios work at runtime, not just in isolation
4. there are no runtime panics or silent failures
5. degraded states behave safely and visibly

### E2E Test Design Rules

1. Test runtime behavior, not just types.
2. Test the full binary where possible (invoke `sldo-research` as a subprocess).
3. Test degraded and failure states, not just the happy path.
4. Assert against observable behavior (exit codes, output file existence, file content patterns).

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

A new dependency is allowed only if the milestone explicitly includes:

- package/crate name
- why existing dependencies are insufficient
- security and maintenance rationale
- build/runtime cost rationale
- tests covering the new integration

### Migration policy

No migrations are expected in this runbook.

### Refactor budget

Stated per-milestone below.

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_research_m*` | green | | | |
| Build/boot | `cargo build --workspace` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current, no stale entries | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces and compatibility requirements?
- Did I add tests for failure modes, not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update documentation to match the implementation?
- Is every assumption either verified or explicitly documented as unresolved?
- Do all tests clean up their output artifacts? Does `git status` show a clean working tree?
- Is `.gitignore` up to date with any new generated files or build outputs?
- Is the milestone truly done according to its Definition of Done?

If any answer is "no", the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/lessons/research-m<N>.md`

```md
# Lessons Learned — research Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Naming conventions established
- [types, files, tests, events, commands]

## Test patterns that worked well
- [pattern]

## Missing tests that should exist now
- [test]

## Rules for the next milestone
- [rule]

## Template improvements suggested
- [improvement]
```

---

## Completion Summary Template

Path: `docs/completion/research-m<N>.md`

```md
# Completion Summary — research Milestone <N>

## Goal completed
- [what capability now exists]

## Files changed
- [file]

## Tests added
- [test file]

## Runtime validations added
- [e2e file]

## Compatibility checks performed
- [check]

## Documentation updated
- [doc and section]

## .gitignore changes
- [patterns added or removed]

## Test artifact cleanup verified
- [confirmation that git status is clean after test run]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## Milestone Plan

---

### Milestone 1 — Crate Scaffolding & CLI Skeleton

**Goal**: Create the `sldo-research` crate with a working CLI that parses arguments, performs preflight checks, and exits cleanly — no research logic yet.

**Context**: The workspace already has `sldo-plan` and `sldo-run` as reference implementations. This milestone establishes the crate structure, adds it to the workspace, defines the CLI interface, and wires up shared infrastructure (`sldo-common`). The CLI should accept a prompt (via file path or inline `--prompt` arg), an optional repo dir, an output path for the dossier, model selection, and iteration count. It should perform the same preflight checks as `sldo-plan` (copilot installed, git safety, file existence) and exit with a success message.

**Important design rule**: Follow `sldo-plan`'s CLI and main() structure exactly. Use clap derive, same error handling pattern, same preflight sequence.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Prompt file path or `--prompt` inline string, optional `--repo-dir`, optional `--output`, `--model`, `--max-iterations`, `--max-searches` |
| Outputs | Clean exit with preflight summary printed to stderr |
| Interfaces touched | New binary `sldo-research`, new toolflags function |
| Files allowed to change | `Cargo.toml` (workspace), `crates/sldo-common/src/toolflags.rs`, `crates/sldo-common/src/lib.rs` (only if toolflags needs re-export) |
| Files to read before changing anything | `crates/sldo-plan/src/main.rs`, `crates/sldo-plan/Cargo.toml`, `crates/sldo-run/src/main.rs`, `crates/sldo-common/src/toolflags.rs`, `Cargo.toml` |
| New files allowed | `crates/sldo-research/Cargo.toml`, `crates/sldo-research/src/main.rs`, `tests/e2e_research_m1.rs` |
| New dependencies allowed | `none` (reuse workspace dependencies only) |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. `sldo-plan` and `sldo-run` behavior unchanged. |
| Forbidden shortcuts | No mocks, no TODOs, no placeholder research logic |

#### Out of Scope / Must Not Do

- Do NOT implement any research logic or Claude Code invocation
- Do NOT implement dossier generation or output writing
- Do NOT modify `sldo-plan` or `sldo-run`
- Do NOT add web search functionality

#### Pre-Flight

1. Complete the Global Entry Rules.
2. No previous milestone lessons to read.
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `Cargo.toml` (workspace root) | Add `crates/sldo-research` to workspace members list |
| `crates/sldo-research/Cargo.toml` | NEW: Crate manifest with dependencies on `sldo-common`, `clap`, `anyhow`, `chrono` (all workspace deps) |
| `crates/sldo-research/src/main.rs` | NEW: CLI definition, arg parsing, preflight checks, clean exit |
| `crates/sldo-common/src/toolflags.rs` | Add `research_allow_flags()` and `research_deny_flags()` functions |
| `tests/e2e_research_m1.rs` | NEW: E2E tests for CLI skeleton |
| `.gitignore` | Review for any needed additions |

#### Step-by-Step

1. Write BDD test stubs first — unit tests in `main.rs` (`#[cfg(test)] mod tests`) for CLI parsing and arg validation.
2. Write E2E test stubs at `tests/e2e_research_m1.rs` — test binary invocation with `--help`, missing args, valid args with no copilot installed fallback.
3. Create `crates/sldo-research/Cargo.toml` following `sldo-plan/Cargo.toml` pattern.
4. Add `"crates/sldo-research"` to workspace members in root `Cargo.toml`.
5. Create `crates/sldo-research/src/main.rs` with:
   - `Cli` struct with clap derive:
     - `prompt_file: Option<PathBuf>` — path to a file containing the research prompt
     - `--prompt <text>` — inline prompt string (alternative to file)
     - `--repo-dir <path>` — optional target repository to research in context of
     - `--output <path>` — output dossier path (default: `output/research-dossier.md`)
     - `--model <model>` — Claude model (default: `claude-opus-4-7`)
     - `--max-iterations <N>` — max research deepening iterations (default: `3`)
     - `--max-searches <N>` — max web search invocations (default: `5`)
   - `run()` function following `sldo-plan`'s pattern:
     - Parse CLI
     - Resolve prompt (from file or `--prompt`, require exactly one)
     - Resolve output path
     - Preflight: check copilot installed, check prompt source, check repo dir if given, check git safety if repo dir given
     - Print summary header
     - Print "Research not yet implemented" info message
     - Exit 0
   - `main()` calling `run()` with `process::exit(1)` on error
6. Add `research_allow_flags()` to `toolflags.rs` — same as `plan_allow_flags()` plus web browsing tools (`--allow-tool=shell(curl:*)`, `--allow-tool=shell(wget:*)`).
7. Add `research_deny_flags()` to `toolflags.rs` — same as `plan_deny_flags()`.
8. Make all BDD tests pass.
9. Run the full test suite: `cargo test --workspace`.
10. Run E2E: `cargo test --test e2e_research_m1 -- --test-threads=1`.
11. Verify `cargo build --workspace` succeeds.
12. Verify `target/debug/sldo-research --help` works.
13. **Verify test artifact cleanup**: Run `git status` and confirm no untracked test output remains.
14. **Update .gitignore**: Add patterns for any new generated files or build outputs. Remove stale patterns.
15. Run smoke tests.
16. Complete the Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: CLI argument parsing**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Help flag prints usage | happy path | binary is built | `sldo-research --help` | exit 0, output contains "research" and usage text |
| Prompt file accepted | happy path | a file `prompt.txt` exists with content | `sldo-research prompt.txt` | preflight runs, prints summary, exits 0 |
| Inline prompt accepted | happy path | no file needed | `sldo-research --prompt "add OAuth2"` | preflight runs, prints summary, exits 0 |
| Both prompt sources rejected | invalid input | a file and `--prompt` both given | `sldo-research prompt.txt --prompt "text"` | exit non-zero, error about conflicting sources |
| No prompt source rejected | invalid input | neither file nor `--prompt` given | `sldo-research` | exit non-zero, error about missing prompt |
| Missing prompt file | invalid input | file does not exist | `sldo-research nonexistent.txt` | exit non-zero, error about file not found |
| Default output path | happy path | no `--output` given | parse args | output resolves to `output/research-dossier.md` |
| Custom output path | happy path | `--output custom.md` given | parse args | output resolves to `custom.md` |
| Default model | happy path | no `--model` given | parse args | model is `claude-opus-4.6` |
| Repo dir validated | happy path | valid repo dir given | `sldo-research --prompt "test" --repo-dir .` | preflight checks repo, prints branch |
| Invalid repo dir | invalid input | bad path given | `sldo-research --prompt "test" --repo-dir /nonexistent` | exit non-zero, error about dir |

#### Regression Tests

- All existing workspace tests: `cargo test --workspace`
- `sldo-plan --help` still works
- `sldo-run --help` still works

#### Compatibility Checklist

- [ ] `sldo-plan` builds and runs with `--help`
- [ ] `sldo-run` builds and runs with `--help`
- [ ] All existing E2E tests pass
- [ ] `cargo build --workspace` succeeds

#### E2E Runtime Validation

**File**: `tests/e2e_research_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_help_flag` | Binary runs and prints help | exit 0, stdout contains "research" |
| `test_missing_prompt_fails` | Binary rejects no-prompt invocation | exit non-zero |
| `test_prompt_file_accepted` | Binary accepts prompt file and runs preflight | exit 0 (or expected "not implemented" exit) |
| `test_inline_prompt_accepted` | Binary accepts `--prompt` arg | exit 0 (or expected exit) |
| `test_invalid_repo_dir_fails` | Binary rejects bad repo path | exit non-zero |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `target/debug/sldo-research --help` prints usage info
- [ ] `target/debug/sldo-research --prompt "test topic"` runs preflight and exits
- [ ] `target/debug/sldo-plan --help` still works (backward compat)
- [ ] `target/debug/sldo-run --help` still works (backward compat)
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` covers all new generated files and build outputs

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | `crates/sldo-research/src/main.rs` (test mod) | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_research_m1.rs` | fail for expected reason | | | |
| Implementation | CLI skeleton + preflight | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_research_m1` | green | | | |
| Build/boot | `cargo build --workspace` | builds cleanly | | | |
| Smoke tests | see list above | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current | | | |
| Compatibility checks | `sldo-plan --help`, `sldo-run --help` | no regressions | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- `sldo-research` crate exists in workspace and compiles
- CLI accepts prompt file, `--prompt`, `--repo-dir`, `--output`, `--model`, `--max-iterations`, `--max-searches`
- Preflight checks run (copilot installed, file existence, git safety)
- All BDD scenarios pass
- All E2E tests pass
- Full existing test suite remains green
- Smoke tests are checked off
- Compatibility checklist is complete
- No forbidden shortcuts remain in production code
- `git status` is clean after test run
- `.gitignore` is up to date
- Docs updated, lessons file written, completion summary written, tracker updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Add `sldo-research` to the CLI tools section, describe its role in the pipeline
- **README.md**: Add `sldo-research` CLI usage section
- **Other docs**: None

#### Notes

- Concurrency/retry categories do not apply — this is a CLI skeleton with no async behavior.
- Persistence category does not apply — no state is persisted yet.

---

### Milestone 2 — Research Prompt Builder

**Goal**: Implement the prompt construction logic that generates structured Claude Code prompts for the research phase — including topic decomposition, question generation, and repo-context prompts.

**Context**: `sldo-plan` has `build_planning_prompt()` which constructs prompts for planning. We need analogous functions for research. The research prompt builder must produce prompts that instruct Claude Code to: (a) break down the topic into researchable sub-questions, (b) explore the target repo for relevant context, and (c) format findings in a structured way. The prompt output is a string fed to `ClaudeInvocation`. This milestone does NOT invoke Claude Code — it only builds the prompts.

**Important design rule**: Research prompts must instruct Claude Code to output findings in a structured markdown format (sections with headers) so that later milestones can parse and validate the output.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | User prompt text, optional repo dir path, iteration number, previous findings (for deepening) |
| Outputs | Formatted prompt strings ready for `ClaudeInvocation` |
| Interfaces touched | New functions in `sldo-research` |
| Files allowed to change | `crates/sldo-research/src/main.rs` |
| Files to read before changing anything | `crates/sldo-plan/src/main.rs` (specifically `build_planning_prompt()`), `crates/sldo-common/src/copilot.rs` |
| New files allowed | `crates/sldo-research/src/prompt.rs`, `tests/e2e_research_m2.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. CLI interface from M1 unchanged. |
| Forbidden shortcuts | No hardcoded topic-specific content in prompts, no TODOs |

#### Out of Scope / Must Not Do

- Do NOT invoke Claude Code CLI (that's M3)
- Do NOT implement dossier file writing
- Do NOT implement web search prompts (that's M5)
- Do NOT modify any existing crate

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/research-m1.md` and apply relevant corrections.
3. Read the allowed files before editing.
4. Copy the Evidence Log template.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/main.rs` | Add `mod prompt;` declaration, wire prompt building into run() |
| `crates/sldo-research/src/prompt.rs` | NEW: Research prompt construction functions |
| `tests/e2e_research_m2.rs` | NEW: E2E tests for prompt construction |
| `.gitignore` | Review |

#### Step-by-Step

1. Write BDD tests first in `crates/sldo-research/src/prompt.rs` (`#[cfg(test)] mod tests`).
2. Write E2E stubs at `tests/e2e_research_m2.rs`.
3. Create `crates/sldo-research/src/prompt.rs` with:
   - `build_exploration_prompt(prompt_content: &str, repo_dir: Option<&Path>) -> String` — First-pass prompt that instructs Claude Code to:
     - Decompose the topic into 5-10 specific research questions
     - If repo_dir is provided: explore the repo structure, tech stack, existing patterns, dependencies
     - Identify key concepts, libraries, APIs, and standards relevant to the topic
     - Output findings as structured markdown with headers: `## Topic Decomposition`, `## Key Questions`, `## Repo Context` (if applicable), `## Initial Findings`
   - `build_deepening_prompt(prompt_content: &str, previous_findings: &str, iteration: u32, repo_dir: Option<&Path>) -> String` — Subsequent-pass prompt that:
     - References the previous findings
     - Instructs Claude Code to answer the unanswered questions
     - Explore deeper on each sub-topic
     - Evaluate specific libraries/tools with pros/cons
     - Output as structured markdown: `## Deepened Findings`, `## Library Evaluations`, `## Architecture Options`, `## Unanswered Questions`
   - `build_repo_context_prompt(repo_dir: &Path) -> String` — Focused repo exploration prompt:
     - Read README, config files, directory structure
     - Identify tech stack, build system, test framework
     - Identify existing patterns and conventions
     - Output as: `## Tech Stack`, `## Project Structure`, `## Build & Test`, `## Existing Patterns`, `## Constraints`
   - Constants for the dossier output format sections
4. Add `mod prompt;` to `main.rs`.
5. Wire: in `run()`, after preflight, build and print the exploration prompt (to verify it works), then exit.
6. Make all BDD tests pass.
7. Run full test suite.
8. Run E2E tests.
9. Verify build.
10. Complete Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Research prompt construction**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Exploration prompt with repo | happy path | prompt text + repo dir | `build_exploration_prompt(text, Some(dir))` | prompt contains topic, references repo, has structured output instructions |
| Exploration prompt without repo | happy path | prompt text only | `build_exploration_prompt(text, None)` | prompt contains topic, no repo references, has output instructions |
| Deepening prompt builds on findings | happy path | prompt + previous findings + iteration 2 | `build_deepening_prompt(...)` | prompt contains previous findings summary, asks for deeper research |
| Repo context prompt | happy path | valid repo dir | `build_repo_context_prompt(dir)` | prompt instructs repo exploration with specific sections |
| Empty prompt rejected | invalid input | empty string | `build_exploration_prompt("", None)` | returns error or prompt clearly states no topic |
| Large prompt handled | happy path | 10KB prompt text | `build_exploration_prompt(large, None)` | prompt is well-formed, doesn't truncate user content |
| Deepening iteration 3 asks synthesis | happy path | iteration=3, findings exist | `build_deepening_prompt(...)` | prompt asks for synthesis and consolidation |

#### Regression Tests

- All existing workspace tests: `cargo test --workspace`
- M1 E2E tests still pass
- CLI `--help` still works

#### Compatibility Checklist

- [ ] `sldo-research --help` still works
- [ ] `sldo-plan --help` still works
- [ ] All existing tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_research_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_prompt_functions_accessible` | Prompt module compiles and functions are callable | test compiles and runs |
| `test_exploration_prompt_format` | Exploration prompt has expected structure | contains key instruction markers |
| `test_deepening_prompt_references_findings` | Deepening prompt includes prior findings | contains findings text |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test -p sldo-research` — all unit tests pass
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` is current

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | `crates/sldo-research/src/prompt.rs` | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_research_m2.rs` | fail for expected reason | | | |
| Implementation | prompt builder functions | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_research_m2` | green | | | |
| Build/boot | `cargo build --workspace` | builds cleanly | | | |
| Smoke tests | see list above | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current | | | |
| Compatibility checks | `sldo-research --help` | no regressions | | | |

#### Definition of Done

- All prompt builder functions exist and produce well-structured prompts
- All BDD scenarios pass
- All E2E tests pass
- Full test suite green
- No placeholders in production code
- Lessons file written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Document the prompt builder module and dossier format
- **README.md**: No change yet
- **Other docs**: None

#### Notes

- Retry/rollback categories do not apply — prompt building is pure function logic.

---

### Milestone 3 — Claude Code-Driven Research Loop

**Goal**: Implement the core research loop that invokes Claude Code CLI iteratively — first for exploration, then for deepening — collecting findings into an in-memory buffer that accumulates across iterations.

**Context**: `sldo-run` has a main loop that invokes Copilot and checks milestone status. `sldo-plan` has an iteration loop for refinement. `sldo-research` needs a similar loop but for research: invoke Claude Code with the exploration prompt, capture output, invoke again with deepening prompt referencing prior output, repeat for `max_iterations`. The key challenge is capturing Copilot's output (which goes to a file via the tool-write permission) and feeding it back into subsequent prompts. Use `ClaudeInvocation::run_with_callback()` for streaming output capture.

**Important design rule**: The research loop must instruct Claude Code to append findings to a scratch file (in the output dir), then read that file back after each iteration. This mirrors how `sldo-plan` writes to the output file and validates it.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Prompt text, repo dir (optional), model, max_iterations, output path |
| Outputs | Accumulated research findings written to a scratch file after each iteration |
| Interfaces touched | New `research_loop()` function, uses `ClaudeInvocation` from `sldo-common` |
| Files allowed to change | `crates/sldo-research/src/main.rs` |
| Files to read before changing anything | `crates/sldo-run/src/main.rs` (main loop pattern), `crates/sldo-plan/src/main.rs` (iteration pattern), `crates/sldo-common/src/copilot.rs` |
| New files allowed | `crates/sldo-research/src/research.rs`, `tests/e2e_research_m3.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. CLI interface unchanged. |
| Forbidden shortcuts | No skipping iterations, no swallowing Claude Code errors, no fake output |

#### Out of Scope / Must Not Do

- Do NOT implement dossier validation (that's M4)
- Do NOT implement web search (that's M5)
- Do NOT implement synthesis (that's M6)
- Do NOT modify `sldo-common` in this milestone

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/lessons/research-m2.md`.
3. Read allowed files.
4. Copy Evidence Log template.
5. Re-state constraints.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/main.rs` | Add `mod research;`, wire research loop into `run()`, replace placeholder exit |
| `crates/sldo-research/src/research.rs` | NEW: Research loop implementation |
| `tests/e2e_research_m3.rs` | NEW: E2E tests for research loop |
| `.gitignore` | Review |

#### Step-by-Step

1. Write BDD tests in `crates/sldo-research/src/research.rs`.
2. Write E2E stubs at `tests/e2e_research_m3.rs`.
3. Create `crates/sldo-research/src/research.rs` with:
   - `research_loop(config: &ResearchConfig) -> Result<String>` where `ResearchConfig` contains:
     - `prompt_content: String`
     - `repo_dir: Option<PathBuf>`
     - `output_path: PathBuf`
     - `model: String`
     - `max_iterations: u32`
     - `cooldown_secs: u64`
   - Loop logic:
     1. If repo_dir provided, invoke Claude Code with `build_repo_context_prompt()` first, save output
     2. Iteration 1: invoke Claude Code with `build_exploration_prompt()`, capture output to scratch file
     3. Iterations 2..N: read scratch file, invoke Claude Code with `build_deepening_prompt()`, append/overwrite scratch
     4. After each iteration: read back the scratch file content, log progress
     5. Return final accumulated findings as String
   - Use `ClaudeInvocation` from `sldo-common` for each invocation
   - Use `LogFile` for logging each iteration
   - Respect cooldown between iterations
4. Add `mod research;` to `main.rs`.
5. Wire: in `run()`, call `research_loop()` after preflight. Print completion summary.
6. Make all tests pass.
7. Full test suite.
8. Build verification.
9. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Research loop execution**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Single iteration completes | happy path | valid prompt, max_iterations=1 | `research_loop(config)` | Claude Code invoked once, findings returned |
| Multiple iterations deepen | happy path | valid prompt, max_iterations=3 | `research_loop(config)` | Claude Code invoked 3 times, each referencing prior findings |
| Repo context gathered first | happy path | prompt + repo_dir | `research_loop(config)` | Repo context prompt runs before exploration |
| No repo dir skips context | happy path | prompt only, no repo_dir | `research_loop(config)` | Starts directly with exploration prompt |
| Claude Code failure handled | partial failure | Claude Code exits non-zero | `research_loop(config)` | Logs warning, continues to next iteration or returns partial findings |
| Log files created | happy path | any valid config | `research_loop(config)` | `.claude-logs/research-iteration-N.log` files exist |
| Cooldown respected | happy path | cooldown=2, iterations=2 | `research_loop(config)` | At least 2 seconds between invocations |

#### Regression Tests

- All existing workspace tests
- M1 and M2 E2E tests still pass
- CLI `--help` still works

#### Compatibility Checklist

- [ ] `sldo-research --help` still works
- [ ] `sldo-plan` and `sldo-run` unchanged
- [ ] All existing tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_research_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_research_binary_runs_without_copilot` | Binary exits with clear error when copilot not found | exit non-zero, error mentions copilot |
| `test_research_config_struct` | ResearchConfig can be constructed | compiles and fields accessible |
| `test_log_directory_created` | Research creates log directory | `.copilot-logs/` exists after run attempt |

Note: Full integration tests requiring Claude Code CLI are gated by availability — tests should skip gracefully if `copilot` is not on PATH.

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `target/debug/sldo-research --prompt "test topic"` attempts research (may fail if copilot not installed, but should fail with clear error)
- [ ] `target/debug/sldo-research --prompt "test topic" --max-iterations 1` runs single pass
- [ ] `git status` shows no untracked test artifacts
- [ ] `.gitignore` covers log dir

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all green | | | |
| BDD tests created | `research.rs` tests | fail for expected reason | | | |
| E2E stubs created | `tests/e2e_research_m3.rs` | fail for expected reason | | | |
| Implementation | research loop | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test --test e2e_research_m3` | green | | | |
| Build/boot | `cargo build --workspace` | builds cleanly | | | |
| Smoke tests | see above | all checked | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | `.gitignore` | current | | | |
| Compatibility checks | existing tools | no regressions | | | |

#### Definition of Done

- Research loop invokes Claude Code iteratively with appropriate prompts
- Findings accumulate across iterations
- Logs written for each iteration
- Claude Code failures handled gracefully
- All tests pass, smoke tests checked
- Lessons file written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Document research loop and iteration strategy
- **README.md**: No change yet (tool not fully functional)
- **Other docs**: None

---

### Milestone 4 — Dossier Validation & Output

**Goal**: Implement the dossier output format — writing accumulated research findings into a structured markdown file — and a validation function that checks the dossier has the required sections and sufficient content.

**Context**: `sldo-plan` has `validate_runbook()` which checks for required sections and placeholders. `sldo-research` needs analogous validation for the research dossier. The dossier is a structured markdown document with defined sections. After the research loop completes, the raw findings must be organized into the dossier format and validated before being declared complete.

**Important design rule**: The dossier format must be designed so that it can be fed directly to `sldo-plan` as a prompt file. It should contain enough structured information that `sldo-plan` can generate a high-quality runbook without needing to repeat the research.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Raw research findings (String), user prompt, repo context |
| Outputs | Structured dossier markdown file, validation result |
| Interfaces touched | New `dossier` module in `sldo-research` |
| Files allowed to change | `crates/sldo-research/src/main.rs`, `crates/sldo-research/src/research.rs` |
| Files to read before changing anything | `crates/sldo-plan/src/main.rs` (`validate_runbook()`), `docs/runbook-template_v_3_template.md` |
| New files allowed | `crates/sldo-research/src/dossier.rs`, `tests/e2e_research_m4.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. Dossier file works as `sldo-plan` prompt input. |
| Forbidden shortcuts | No placeholder sections in dossier, no skipping validation |

#### Out of Scope / Must Not Do

- Do NOT modify the dossier format to include milestone plans (that's `sldo-plan`'s job)
- Do NOT implement web search
- Do NOT modify `sldo-plan` or `sldo-run`

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/main.rs` | Add `mod dossier;`, wire dossier writing and validation into `run()` |
| `crates/sldo-research/src/research.rs` | Return structured findings to caller |
| `crates/sldo-research/src/dossier.rs` | NEW: Dossier format, writing, and validation |
| `tests/e2e_research_m4.rs` | NEW: E2E tests for dossier output |
| `.gitignore` | Review |

#### Step-by-Step

1. Write BDD tests first in `dossier.rs`.
2. Write E2E stubs at `tests/e2e_research_m4.rs`.
3. Create `crates/sldo-research/src/dossier.rs` with:
   - **Dossier format definition** — the output markdown structure:
     ```
     # Research Dossier — [Topic]
     > Generated by sldo-research on [date]
     > Source prompt: [first 200 chars of prompt]

     ## Executive Summary
     [2-3 paragraph overview of findings]

     ## Topic Decomposition
     [Numbered list of sub-topics identified]

     ## Key Findings
     ### [Sub-topic 1]
     [Findings with sources/references]
     ### [Sub-topic 2]
     ...

     ## Library & Tool Evaluations
     | Library/Tool | Purpose | Pros | Cons | Recommendation |
     |---|---|---|---|---|

     ## Architecture Options
     ### Option A — [name]
     [Description, trade-offs]
     ### Option B — [name]
     ...

     ## API & SDK Documentation
     [Relevant API excerpts, endpoints, data models]

     ## Repository Context
     [Tech stack, existing patterns, constraints — if repo was scanned]

     ## Design Recommendations
     [Concrete recommendations based on findings]

     ## Risks & Open Questions
     [Unresolved items, risks, areas needing human judgment]

     ## References
     [URLs, documentation links, source citations]
     ```
   - `write_dossier(path: &Path, prompt: &str, findings: &str, repo_context: Option<&str>) -> Result<()>` — Writes the dossier file
   - `validate_dossier(path: &Path) -> Vec<String>` — Validates:
     - File exists
     - Size >= 500 bytes
     - Contains required sections (Executive Summary, Key Findings, Design Recommendations)
     - No template placeholders remaining
   - `REQUIRED_SECTIONS: &[&str]` constant
4. Add `mod dossier;` to `main.rs`.
5. Wire: after `research_loop()`, call `write_dossier()`, then `validate_dossier()`. Print results.
6. If validation fails, log warnings (don't fail — partial dossier is still useful).
7. Make all tests pass.
8. Run full test suite and E2E.
9. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Dossier output and validation**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Dossier written successfully | happy path | valid findings | `write_dossier(path, prompt, findings, None)` | file created with all sections |
| Dossier with repo context | happy path | findings + repo context | `write_dossier(path, prompt, findings, Some(ctx))` | file includes Repository Context section |
| Dossier validation passes | happy path | complete dossier file | `validate_dossier(path)` | empty issues list |
| Missing section detected | invalid input | dossier missing "Key Findings" | `validate_dossier(path)` | issues list contains missing section |
| Too small dossier | invalid input | dossier < 500 bytes | `validate_dossier(path)` | issues list mentions size |
| Nonexistent file | invalid input | path doesn't exist | `validate_dossier(path)` | issues list mentions file not found |
| Output directory created | happy path | output path in non-existent subdir | `write_dossier(nested/path, ...)` | parent directories created |

#### Regression Tests

- All existing workspace tests
- M1, M2, M3 E2E tests
- CLI interface unchanged

#### Compatibility Checklist

- [ ] `sldo-research --help` still works
- [ ] All existing tools unaffected
- [ ] Output file can be read by `sldo-plan` as prompt (validated in M7)

#### E2E Runtime Validation

**File**: `tests/e2e_research_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_dossier_write_creates_file` | Dossier writing works at runtime | file exists after call |
| `test_dossier_validation_complete` | Validation passes for good dossier | no issues returned |
| `test_dossier_validation_incomplete` | Validation catches missing sections | issues list non-empty |
| `test_dossier_output_directory_creation` | Parent dirs created for output | nested path works |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test -p sldo-research` passes
- [ ] `git status` clean
- [ ] `.gitignore` current

#### Evidence Log

(Copy from template)

#### Definition of Done

- Dossier format defined with all sections
- `write_dossier()` creates well-structured output file
- `validate_dossier()` checks required sections and size
- Integration wired into `run()`
- All tests pass
- Lessons file written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Document dossier format and validation
- **README.md**: No change yet
- **Other docs**: None

---

### Milestone 5 — Web Search Integration

**Goal**: Add web search capability to the research loop by constructing prompts that explicitly instruct Claude Code to use its web browsing / search tools to find current documentation, library comparisons, and best practices.

**Context**: Claude Code CLI can be granted web browsing capabilities through tool permission flags. Rather than implementing a custom web search API, we instruct Claude Code to search the web as part of its research. This requires (a) appropriate tool flags in `sldo-common/toolflags.rs`, (b) web-search-specific prompts that guide Claude Code to find and summarize relevant URLs, and (c) integration into the research loop as a dedicated web research phase.

**Important design rule**: Web search is a *phase* in the research loop, not a separate tool. After the initial exploration pass, the research loop should perform dedicated web search iterations before deepening passes.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Research topic, sub-questions from exploration, max_searches config |
| Outputs | Web search findings appended to research accumulator |
| Interfaces touched | `prompt.rs` (new web prompt), `research.rs` (web phase), `toolflags.rs` (web flags) |
| Files allowed to change | `crates/sldo-research/src/prompt.rs`, `crates/sldo-research/src/research.rs`, `crates/sldo-research/src/main.rs`, `crates/sldo-common/src/toolflags.rs` |
| Files to read before changing anything | `crates/sldo-common/src/toolflags.rs`, `crates/sldo-common/src/copilot.rs` |
| New files allowed | `tests/e2e_research_m5.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. Existing tool flags unchanged. |
| Forbidden shortcuts | No hardcoded URLs, no bypassing Claude Code for web access |

#### Out of Scope / Must Not Do

- Do NOT implement direct HTTP requests or web scraping
- Do NOT add new external dependencies for web search
- Do NOT modify the dossier format
- Do NOT modify `sldo-plan` or `sldo-run`

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/prompt.rs` | Add `build_websearch_prompt()` function |
| `crates/sldo-research/src/research.rs` | Add web search phase to research loop |
| `crates/sldo-research/src/main.rs` | Wire `--max-searches` into research config |
| `crates/sldo-common/src/toolflags.rs` | Extend `research_allow_flags()` with web/browse tools if not already present |
| `tests/e2e_research_m5.rs` | NEW: E2E tests for web search integration |
| `.gitignore` | Review |

#### Step-by-Step

1. Write BDD tests for `build_websearch_prompt()` in `prompt.rs`.
2. Write E2E stubs at `tests/e2e_research_m5.rs`.
3. Add `build_websearch_prompt(topic: &str, questions: &str, search_index: u32) -> String` to `prompt.rs`:
   - Instructs Claude Code to search the web for specific sub-questions
   - Asks for current documentation links, library versions, API references
   - Requests structured output: `## Web Search Results`, `## Documentation Found`, `## Library Versions`
   - Each call focuses on a subset of questions (controlled by `search_index`)
4. Update `research_allow_flags()` in `toolflags.rs` to include web tool permissions:
   - `--allowedTools=Read,Write,Edit,Bash,Glob,Grep,WebFetch,WebSearch`
5. Update research loop in `research.rs`:
   - After exploration phase (iteration 1), run web search phase:
     - For `i` in `1..=max_searches`: invoke Claude Code with `build_websearch_prompt(topic, questions, i)`
     - Accumulate web findings
   - Then proceed to deepening iterations with web findings included
6. Wire `max_searches` from CLI config through to research loop.
7. Make all tests pass.
8. Full test suite + E2E.
9. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Web search prompts and integration**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Web search prompt constructed | happy path | topic + questions | `build_websearch_prompt(topic, questions, 1)` | prompt instructs web search with output format |
| Web search prompt varies by index | happy path | same topic, different index | two calls with index 1 and 2 | prompts differ in focus area |
| Web phase runs in loop | happy path | max_searches=3 | research loop runs | 3 web search invocations occur |
| Zero searches skips web phase | happy path | max_searches=0 | research loop runs | no web search invocations |
| Research flags include web tools | happy path | call `research_allow_flags()` | flags returned | contains web browsing permission |
| Web search failure doesn't halt research | partial failure | web search Claude Code call fails | research loop continues | deepening phase still runs with available findings |

#### Regression Tests

- All existing workspace tests
- M1-M4 E2E tests
- `plan_allow_flags()` and `run_allow_flags()` unchanged

#### Compatibility Checklist

- [ ] `sldo-research --help` still works
- [ ] Existing `plan_allow_flags()` unchanged
- [ ] Existing `run_allow_flags()` and `run_deny_flags()` unchanged
- [ ] All existing tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_research_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_websearch_prompt_format` | Web prompt is well-structured | contains expected instruction markers |
| `test_research_flags_include_web` | Tool flags include web permissions | `research_allow_flags()` contains web tool |
| `test_zero_searches_accepted` | CLI accepts `--max-searches 0` | runs without error |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `target/debug/sldo-research --prompt "test" --max-searches 0` runs (no web phase)
- [ ] `cargo test -p sldo-research` passes
- [ ] `git status` clean
- [ ] `.gitignore` current

#### Evidence Log

(Copy from template)

#### Definition of Done

- Web search prompt builder exists and produces structured prompts
- Research loop includes web search phase controlled by `--max-searches`
- Tool flags include web browsing permissions
- Web search failures don't halt the research loop
- All tests pass
- Lessons file written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Document web search phase in research pipeline
- **README.md**: No change yet
- **Other docs**: None

---

### Milestone 6 — Multi-Source Synthesis Pass

**Goal**: Add a final synthesis pass to the research loop that takes all accumulated findings (exploration, web search, deepening) and produces a coherent, deduplicated, well-organized dossier — resolving contradictions and surfacing confidence levels.

**Context**: After M3-M5, the research loop produces raw accumulated findings from multiple Claude Code invocations. These findings may be repetitive, contradictory, or poorly organized. This milestone adds a synthesis phase: one final Claude Code invocation that reads all raw findings and produces a clean, coherent dossier in the defined format from M4.

**Important design rule**: The synthesis prompt must instruct Claude Code to organize findings into the dossier format (from M4), resolve contradictions, rank recommendations by confidence, and explicitly flag areas of uncertainty.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | All accumulated raw findings, user prompt, repo context |
| Outputs | Synthesized dossier content in the defined format |
| Interfaces touched | `prompt.rs` (synthesis prompt), `research.rs` (synthesis phase), `dossier.rs` (write synthesized content) |
| Files allowed to change | `crates/sldo-research/src/prompt.rs`, `crates/sldo-research/src/research.rs`, `crates/sldo-research/src/dossier.rs`, `crates/sldo-research/src/main.rs` |
| Files to read before changing anything | All `sldo-research/src/` files |
| New files allowed | `tests/e2e_research_m6.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing tests pass. Dossier format unchanged. |
| Forbidden shortcuts | No skipping synthesis, no returning raw unsynthesized findings as final output |

#### Out of Scope / Must Not Do

- Do NOT modify the dossier section structure (defined in M4)
- Do NOT add new research phases
- Do NOT modify other crates

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/prompt.rs` | Add `build_synthesis_prompt()` |
| `crates/sldo-research/src/research.rs` | Add synthesis phase after all research iterations |
| `crates/sldo-research/src/dossier.rs` | Adjust `write_dossier()` to accept synthesized content |
| `crates/sldo-research/src/main.rs` | Minor wiring if needed |
| `tests/e2e_research_m6.rs` | NEW: E2E tests |
| `.gitignore` | Review |

#### Step-by-Step

1. Write BDD tests for `build_synthesis_prompt()`.
2. Write E2E stubs.
3. Add `build_synthesis_prompt(prompt: &str, all_findings: &str, repo_context: Option<&str>) -> String` to `prompt.rs`:
   - Instructs Claude Code to read all raw findings
   - Organize into the dossier sections (Executive Summary, Key Findings, etc.)
   - Resolve contradictions — prefer more recent/authoritative sources
   - Rank recommendations by confidence (high/medium/low)
   - Flag open questions explicitly
   - Output the complete dossier content
4. Update research loop in `research.rs`:
   - After all iterations (exploration + web + deepening), run synthesis:
     - Concatenate all findings
     - Invoke Claude Code with `build_synthesis_prompt()`
     - The synthesis output becomes the dossier content
   - Pass synthesized content to `write_dossier()`
5. Make all tests pass.
6. Full test suite + E2E.
7. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Synthesis pass**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Synthesis prompt includes all findings | happy path | raw findings from 3 phases | `build_synthesis_prompt(...)` | prompt contains all findings and format instructions |
| Synthesis prompt requests dedup | happy path | any input | `build_synthesis_prompt(...)` | prompt instructs deduplication |
| Synthesis prompt requests confidence levels | happy path | any input | `build_synthesis_prompt(...)` | prompt asks for confidence ranking |
| Synthesis runs as final phase | happy path | research loop completes | loop execution | synthesis invocation happens last |
| Synthesis failure returns raw findings | partial failure | synthesis Claude Code call fails | research loop | raw findings still written to dossier (fallback) |

#### Regression Tests

- All existing workspace tests
- M1-M5 E2E tests

#### Compatibility Checklist

- [ ] Dossier format sections match M4 definition
- [ ] CLI interface unchanged
- [ ] All existing tests pass

#### E2E Runtime Validation

**File**: `tests/e2e_research_m6.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_synthesis_prompt_format` | Synthesis prompt has expected structure | contains dossier section names |
| `test_synthesis_prompt_includes_findings` | Prompt embeds raw findings | contains findings text |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test -p sldo-research` passes
- [ ] `git status` clean

#### Evidence Log

(Copy from template)

#### Definition of Done

- Synthesis prompt builder exists
- Research loop runs synthesis as final phase
- Synthesized output written to dossier
- Fallback to raw findings if synthesis fails
- All tests pass
- Lessons file written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Document synthesis phase
- **README.md**: No change yet
- **Other docs**: None

---

### Milestone 7 — Plan-Ready Output & sldo-plan Integration

**Goal**: Ensure the dossier output is directly usable as input to `sldo-plan`, add end-to-end pipeline documentation, and verify the full `sldo-research → sldo-plan` flow works.

**Context**: The dossier file produced by `sldo-research` should work as the `prompt_file` argument to `sldo-plan`. This milestone verifies that integration, adds helpful CLI output (suggesting the next `sldo-plan` command to run), updates all documentation, and performs final polish.

**Important design rule**: `sldo-plan` must NOT be modified. The dossier format must be compatible with `sldo-plan` as-is. If anything in the dossier confuses `sldo-plan`, adjust the dossier format, not `sldo-plan`.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Complete dossier from M6 |
| Outputs | CLI prints next-step command, dossier validated for plan compatibility |
| Interfaces touched | `main.rs` (completion output), `dossier.rs` (plan-readiness check) |
| Files allowed to change | `crates/sldo-research/src/main.rs`, `crates/sldo-research/src/dossier.rs` |
| Files to read before changing anything | `crates/sldo-plan/src/main.rs` (understand what it expects as prompt input) |
| New files allowed | `tests/e2e_research_m7.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `sldo-plan` accepts dossier as prompt file without changes. All existing tests pass. |
| Forbidden shortcuts | No modifying `sldo-plan`, no hardcoded paths |

#### Out of Scope / Must Not Do

- Do NOT modify `sldo-plan` or `sldo-run`
- Do NOT add new research phases
- Do NOT change the dossier section structure

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `crates/sldo-research/src/main.rs` | Add completion message with suggested `sldo-plan` command |
| `crates/sldo-research/src/dossier.rs` | Add `check_plan_readiness()` function |
| `tests/e2e_research_m7.rs` | NEW: E2E tests for plan integration |
| `docs/ARCHITECTURE.md` | Document full pipeline |
| `README.md` | Add `sldo-research` documentation and pipeline usage |
| `.gitignore` | Final review |

#### Step-by-Step

1. Write BDD tests for `check_plan_readiness()` in `dossier.rs`.
2. Write E2E stubs at `tests/e2e_research_m7.rs`.
3. Add `check_plan_readiness(path: &Path) -> Vec<String>` to `dossier.rs`:
   - All `validate_dossier()` checks pass
   - Content is > 1000 bytes (substantive enough for planning)
   - Contains at least 3 sections with content
   - Contains "Design Recommendations" section (critical for planning)
   - Contains either "Library & Tool Evaluations" or "Architecture Options"
4. Update `run()` in `main.rs`:
   - After dossier is written and validated, run `check_plan_readiness()`
   - If plan-ready: print success and suggested command:
     ```
     ✔ Research dossier is ready for planning.
     
     Next step — generate a runbook:
       sldo-plan <dossier-path> <repo-dir> [--output docs/RUNBOOK.md]
     ```
   - If not plan-ready: print warnings about what's missing
   - Print summary stats: iterations completed, web searches performed, dossier size, time elapsed
5. Update `docs/ARCHITECTURE.md`:
   - Add `sldo-research` to CLI tools section
   - Document the research pipeline (exploration → web search → deepening → synthesis → dossier)
   - Document dossier format
   - Document the full pipeline: `sldo-research → sldo-plan → sldo-run`
6. Update `README.md`:
   - Add `sldo-research` CLI usage section with examples
   - Document the full pipeline workflow
   - Add dossier format description
7. Make all tests pass.
8. Run full test suite + all E2E tests.
9. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: Plan-ready output**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Plan-ready dossier accepted | happy path | complete dossier > 1000 bytes with all sections | `check_plan_readiness(path)` | empty issues list |
| Too small dossier flagged | invalid input | dossier < 1000 bytes | `check_plan_readiness(path)` | issues mention size |
| Missing recommendations flagged | invalid input | dossier without Design Recommendations | `check_plan_readiness(path)` | issues mention missing section |
| Suggested command printed | happy path | research completes successfully | CLI output | stderr contains `sldo-plan` command suggestion |
| Summary stats printed | happy path | research completes | CLI output | stderr contains iteration count, time elapsed |
| Dossier works as sldo-plan input | happy path | dossier file exists | `sldo-plan <dossier> <repo>` would accept it | file is valid text that sldo-plan can read |

#### Regression Tests

- All existing workspace tests (290+)
- All M1-M6 E2E tests
- `sldo-plan --help` and `sldo-run --help` unchanged

#### Compatibility Checklist

- [ ] `sldo-plan` accepts dossier as prompt file (file is readable text)
- [ ] `sldo-plan --help` unchanged
- [ ] `sldo-run --help` unchanged
- [ ] All existing tests pass
- [ ] No new workspace-level dependencies

#### E2E Runtime Validation

**File**: `tests/e2e_research_m7.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_plan_readiness_complete` | Plan readiness passes for good dossier | no issues |
| `test_plan_readiness_incomplete` | Plan readiness catches problems | issues list non-empty |
| `test_dossier_is_valid_text_file` | Dossier is UTF-8 text readable by any tool | file parses as valid UTF-8 |
| `test_cli_prints_next_step` | Binary suggests sldo-plan command | output contains "sldo-plan" |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds
- [ ] `target/debug/sldo-research --help` prints full usage
- [ ] `target/debug/sldo-plan --help` still works
- [ ] `target/debug/sldo-run --help` still works
- [ ] Full pipeline documented in README
- [ ] `git status` clean
- [ ] `.gitignore` final review complete

#### Evidence Log

(Copy from template)

#### Definition of Done

- `check_plan_readiness()` validates dossier for planning use
- CLI prints suggested `sldo-plan` command on success
- CLI prints summary statistics
- `ARCHITECTURE.md` documents `sldo-research` and full pipeline
- `README.md` documents `sldo-research` CLI usage
- Dossier file is valid input for `sldo-plan`
- All tests pass (existing + new)
- All smoke tests checked
- Lessons file written, completion summary written, tracker updated

#### Post-Flight

- **ARCHITECTURE.md**: Full pipeline documentation, `sldo-research` component
- **README.md**: `sldo-research` CLI usage, pipeline workflow, examples
- **Other docs**: None

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | Add `sldo-research` to CLI tools | Add `sldo-research` basic entry | Review for new build artifacts | None |
| 2 | Document prompt builder module | No change | Review | None |
| 3 | Document research loop | No change | Ensure `.copilot-logs/` covered | None |
| 4 | Document dossier format | No change | Ensure `output/` covered | None |
| 5 | Document web search phase | No change | Review | None |
| 6 | Document synthesis phase | No change | Review | None |
| 7 | Full pipeline documentation | Full `sldo-research` docs + pipeline | Final review | None |

---

## Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.
