# sldo-research — SunLitOrchestrate

> **Purpose**: Build `sldo-research`, a Rust CLI that takes a raw user prompt (from file or `--prompt` arg), runs a multi-phase Claude Code–driven research pipeline (exploration → web search → deepening → synthesis), and produces a structured research dossier consumable by `sldo-plan`.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section **and** the Pre-Milestone Protocol. After completing it, follow the Post-Milestone Protocol. Never skip ahead.
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), existing crates `crates/sldo-plan`, `crates/sldo-run`, `crates/sldo-common`.

---

## Milestone Tracker

Update this table as each milestone is completed. This is the **single source of truth** for progress.

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Crate scaffolding & CLI skeleton | `done` | 2026-04-19 | 2026-04-19 | docs/lessons/research-m1.md |
| 2 | Research prompt builder | `not_started` | | | |
| 3 | Claude Code–driven research loop | `not_started` | | | |
| 4 | Dossier format, writer & validator | `not_started` | | | |
| 5 | Web search phase integration | `not_started` | | | |
| 6 | Multi-source synthesis pass | `not_started` | | | |
| 7 | Plan-ready output & sldo-plan integration | `not_started` | | | |

<!-- Status values: not_started | in_progress | done -->
<!-- Lessons files go in docs/lessons/research-m<N>.md -->

---

## Pre-Milestone Protocol

**Do this before every milestone — no exceptions.**

1. **Read the lessons file from the previous milestone** (path in the Milestone Tracker). Apply its design corrections, naming rules, test patterns, and failure-mode coverage before writing new code.
2. **Read the current milestone section fully** — Goal, Context, Files Most Likely Touched, Step-by-Step, BDD Acceptance Scenarios, Regression Tests, E2E Runtime Validation, and Smoke Tests — before writing any code.
3. **Run the full existing test suite** and confirm it is green. Record the baseline output:
   ```
   cargo test --workspace 2>&1 | tail -20
   ```
   If anything is red, stop and fix the baseline first. Never begin a milestone on a red baseline.
4. **Read the files listed under "Files Most Likely Touched"** for the current milestone. Understand their current shape before editing.
5. **Update the Milestone Tracker** in this file: set the current milestone's Status to `in_progress` and record the Started date.
6. **Create BDD test stubs first** — scenario tests in the correct target file/module before writing production code. Tests declare the contract; implementation satisfies it.
7. **Create E2E test stubs** at `tests/e2e_research_m<N>.rs` and register them under `[[test]]` in the workspace root `Cargo.toml` before writing production code.
8. **Before running E2E tests that invoke the binary**, run `cargo build --workspace` so `target/debug/sldo-research` exists. (Lesson carried over from M1: `cargo test` does not build `[[bin]]` targets.)

---

## Post-Milestone Protocol

**Do this after every milestone — no exceptions.**

1. **Run the full test suite.** Every pre-existing test must still pass. Every new BDD scenario must pass.
   ```
   cargo test --workspace
   ```
2. **Run the E2E runtime validation tests** for this milestone:
   ```
   cargo build --workspace
   cargo test --test e2e_research_m* -- --test-threads=1
   ```
3. **Verify the workspace builds and the binary boots cleanly:**
   ```
   cargo build --workspace
   target/debug/sldo-research --help
   target/debug/sldo-plan --help
   target/debug/sldo-run --help
   ```
4. **Run the smoke tests** listed in the milestone and tick each checkbox in this file.
5. **Verify backward compatibility.** All `sldo-plan`, `sldo-run`, and `sldo-tauri` behavior must be unchanged. All ~290 existing tests must remain green.
6. **Verify test artifact cleanup.** Run `git status` and confirm no stray test output (`output/`, `.sldo-logs/`, `tmp_*`, etc.) is untracked because of tests. If a path is truly a product artifact, ensure it is covered by `.gitignore`.
7. **Review `.gitignore`.** Add new build outputs/log dirs introduced by this milestone. Remove patterns that no longer apply.
8. **Update `docs/ARCHITECTURE.md`** per the Documentation Update Table.
9. **Update `README.md`** if user-facing CLI capabilities changed.
10. **Run lint + format + audit:**
    ```
    cargo fmt --all
    cargo clippy --workspace --all-targets -- -D warnings
    cargo audit    # only if new dependencies were added in the milestone
    ```
11. **Write a lessons-learned file** at `docs/lessons/research-m<N>.md`. Include: design decisions, mistakes & root causes, naming conventions, test patterns that worked, rules for the next milestone.
12. **Write a completion summary** at `docs/completion/research-m<N>.md`.
13. **Update the Milestone Tracker** in this file: set Status to `done`, record the Completed date, fill in the Lessons File path.
14. **Re-read the next milestone** with fresh eyes; note any assumption changes in the lessons file.

---

## Background Context

### Current State

SunLitOrchestrate is a Rust workspace (`Cargo.toml`, resolver 2) with five member crates:

- `crates/sldo-common` — shared library: `copilot.rs` (`ClaudeInvocation` + `run_with_callback`), `logging.rs` (`LogFile`, `ensure_log_dir` → `.sldo-logs/`), `preflight.rs` (`check_claude_installed`, `check_file_exists`, `check_git_safety`), `toolflags.rs` (`plan_allow_flags`, `run_allow_flags`, and already-added `research_allow_flags`/`research_deny_flags`), `git.rs`, `detect.rs`, `runbook.rs`, `color.rs`.
- `crates/sldo-plan` — one binary; `main.rs` implements `Cli` (clap derive), `resolve_output_path`, `read_template`, `build_generate_prompt`, `build_review_prompt`, `validate_runbook` with `REQUIRED_SECTIONS` & `PLACEHOLDER_PATTERNS`, and a two-step generate/review loop against Claude Code CLI.
- `crates/sldo-run` — one binary; drives Claude Code through milestones one at a time using `runbook::parse_tracker` / `next_incomplete` / `all_done`, runs auto-detected or user-supplied build/test commands after each milestone, respects `--max-attempts` and `--cooldown`.
- `crates/sldo-tauri` — Tauri v2 desktop app (unchanged by this runbook).
- `crates/sldo-research` — **Milestone 1 already done**. Crate exists, `main.rs` has:
  - `Cli` (clap derive) with `prompt_file: Option<PathBuf>`, `--prompt`, `--repo-dir`, `--output` (default `output/research-dossier.md`), `--model` (default `claude-opus-4-7`), `--max-iterations` (default `3`), `--max-searches` (default `5`).
  - `run()` validates exactly-one-of (`prompt_file`, `--prompt`), runs preflight (`check_claude_installed`, `check_file_exists`, optional `check_git_safety`), prints "Research not yet implemented" and exits 0.
  - 13 unit tests inside `#[cfg(test)] mod tests`.
- Workspace root hosts integration tests: `tests/e2e_scaffold_m1.rs`, `tests/e2e_common_m2.rs`, `tests/e2e_plan_m3.rs`, `tests/e2e_run_m4.rs`, `tests/e2e_integration_m5.rs`, `tests/e2e_tauri_m[1,3-8].rs`, `tests/e2e_voice_tx_m[1,2,4,5].rs`, and `tests/e2e_research_m1.rs`. Each file is declared under `[[test]]` in the root `Cargo.toml`.
- `.gitignore` already covers `.sldo-logs/`, `.copilot-logs/`, and `output/`.

Baseline test counts (from `docs/ARCHITECTURE.md` and M1 evidence): 51 sldo-common + 24 sldo-plan + 13 sldo-run + 65 sldo-tauri + E2E tests ≈ 290 tests. A pre-existing Tauri frontend `esbuild` failure on darwin-arm64 is known and unrelated to this runbook (commit `4737b93` landed an esbuild binary fix).

### Problem

1. **No research phase between raw prompt and runbook generation.** `sldo-plan` (`crates/sldo-plan/src/main.rs`) expects `prompt_file` to already contain enough analysis for `build_generate_prompt` to produce good milestones. A terse prompt like "add OAuth2 to the API" overloads a single Claude Code session with both research and decomposition, producing vague milestones.
2. **No explicit web-documentation-gathering step.** `plan_allow_flags()` does not include `WebSearch`/`WebFetch` — the planner relies only on Claude's training data, which can be outdated for API versions and library releases.
3. **No reviewable intermediate artifact.** There is no structured dossier that a human can read, edit, and sign off on before committing to a milestone decomposition.
4. **Repo exploration and external research are mixed** during planning. Separating them produces clearer reasoning at each stage.

### Target Architecture

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         sldo-research Pipeline                           │
│                                                                          │
│ user prompt ─▶ sldo-research CLI ─▶ research-dossier.md ─▶ sldo-plan ─▶  │
│                     │                       │                  RUNBOOK.md│
│                     ▼                       │                            │
│  ┌──────────────────┴───────────────────┐   │                            │
│  │  Research loop (Claude Code CLI)     │   │                            │
│  │  1. Repo context (if --repo-dir)     │   │                            │
│  │  2. Exploration (decompose topic)    │   │                            │
│  │  3. Web search × max_searches        │   │                            │
│  │  4. Deepening × (max_iterations - 1) │   │                            │
│  │  5. Synthesis (final pass)           │   │                            │
│  └──────────────────┬───────────────────┘   │                            │
│                     ▼                       │                            │
│  .sldo-logs/research-*.log        output/research-dossier.md             │
│                                                                          │
│  Shared: sldo-common::{copilot, preflight, logging, toolflags, color}    │
│                                                                          │
│  Legend: ─── existing   - - - to be built   ▶ data flow                  │
└──────────────────────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Mirror `sldo-plan` / `sldo-run` patterns.** Reuse clap-derive CLI, `ClaudeInvocation::run_with_callback`, `LogFile`, `ensure_log_dir`, colour helpers, and preflight utilities. Do **not** invent new abstractions.
2. **Dossier is a first-class artifact.** Markdown file with a fixed section list; usable as both a human review artifact and as `prompt_file` input to `sldo-plan`.
3. **Iterative deepening.** Each phase uses a fresh Claude Code invocation whose prompt embeds the prior findings. The pattern is the same as `sldo-plan`'s resume/review loop.
4. **Web search is a Claude Code tool, not a custom HTTP client.** `research_allow_flags()` already includes `WebFetch,WebSearch`. We instruct Claude Code via the prompt; no new HTTP/SDK dependencies.
5. **Separation of concerns.** Research gathers & organises information. It does **not** produce milestones — that remains `sldo-plan`'s job. The dossier must not include a milestone tracker.
6. **Pure prompt builders.** All `build_*_prompt` functions are pure (no I/O). They take inputs and return `String`. This keeps unit tests trivial. (Lesson carried from M1.)
7. **Graceful degradation.** Any single Claude Code invocation failure (non-zero exit) is logged and the loop continues with whatever findings were captured. Only a final "no findings at all" state should produce a non-zero CLI exit.
8. **No placeholders in production paths** — no TODOs, no silent error swallowing, no mocks left behind.

### What to Keep

- `sldo-plan` CLI interface and all existing behaviour (unchanged).
- `sldo-run` CLI interface and all existing behaviour (unchanged).
- `sldo-tauri` backend, frontend, and all events/commands (unchanged).
- `sldo-common` existing public API — only **additive** changes allowed (existing `research_allow_flags`/`research_deny_flags` already exist).
- All ~290 existing tests.
- The M1-established shape of `crates/sldo-research/src/main.rs` CLI struct and `run()` preflight sequence.
- Existing runbook template files under `docs/` (untouched).

### What to Change

- `crates/sldo-research/src/main.rs` — grow from "not yet implemented" stub into a full pipeline (new `mod` declarations each milestone).
- `crates/sldo-research/src/prompt.rs` — **NEW** (M2, extended in M5/M6): pure prompt constructors.
- `crates/sldo-research/src/research.rs` — **NEW** (M3, extended in M5/M6): iteration loop, orchestration, scratch-file handling.
- `crates/sldo-research/src/dossier.rs` — **NEW** (M4, extended in M7): dossier section constants, writer, validator, plan-readiness check.
- `tests/e2e_research_m2.rs` … `tests/e2e_research_m7.rs` — **NEW** per milestone. Each registered under `[[test]]` in the root `Cargo.toml`.
- `docs/ARCHITECTURE.md` — additive: new "sldo-research" section per milestone.
- `README.md` — additive: new `sldo-research` CLI section in M7.
- `crates/sldo-common/src/toolflags.rs` — no changes; `research_allow_flags()`/`research_deny_flags()` already present and include `WebFetch,WebSearch`.

---

## BDD Practices

Every milestone follows these rules.

### Write Tests Before Production Code

1. Read the BDD acceptance table.
2. Create the test file(s) first — backend `#[cfg(test)] mod tests { … }` for unit/BDD scenarios inside the target `.rs` file; `tests/e2e_research_m<N>.rs` for runtime validation.
3. Add the new E2E file to the workspace `[[test]]` list in root `Cargo.toml` before writing production code.
4. Confirm the tests fail (they reference types/functions that don't exist yet) — but the workspace must still compile by using `#[allow(unused_imports)]` / empty stubs.
5. Write production code to make them pass.
6. Re-run tests after any refactor.

### Scenario Structure

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition]
    // When:  [action]
    // Then:  [expected outcome]
}
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Unit / BDD | `#[cfg(test)] mod tests` inside the production `.rs` | `crates/sldo-research/src/<module>.rs` |
| Workspace E2E (binary invocation) | `e2e_research_m<N>.rs` | `tests/` at workspace root |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go **beyond compilation** and verify runtime behavior:

1. Binary boots without panics.
2. Exit codes are correct (0 for success, non-zero for invalid input/unrecoverable failure).
3. Expected side effects occur (files written, log dirs created, stdout/stderr messages present).
4. Degraded states behave predictably (missing `claude`, non-zero Claude exit, empty findings).
5. All new temporary files/dirs are cleaned up — `git status` must stay clean after running the full test suite.

#### E2E Test Design Rules

- Test the full binary as a subprocess with `std::process::Command` (see `tests/e2e_research_m1.rs` for the established pattern: `binary()` helper returning `target/debug/sldo-research`).
- Use `std::env::temp_dir().join("sldo_research_e2e_m<N>_<label>")` for scratch dirs; `let _ = std::fs::remove_dir_all(&tmp);` at the end of each test.
- Tests must tolerate a missing `claude` CLI — accept either success or a clear non-empty stderr error message (M1 pattern).

---

## Milestone Plan

### Milestone 1 — Crate scaffolding & CLI skeleton

**Status: done** (2026-04-19). Lessons: `docs/lessons/research-m1.md`, Completion: `docs/completion/research-m1.md`.

Summary of what exists:

- `crates/sldo-research/Cargo.toml` with dependencies on `sldo-common`, `clap`, `anyhow`, `chrono` (all workspace deps).
- `crates/sldo-research/src/main.rs` with `Cli` (clap derive), `run()` that performs preflight and exits with an "not yet implemented" message, 13 unit tests.
- `tests/e2e_research_m1.rs` registered in root `Cargo.toml` under `[[test]]`.
- `sldo-common/toolflags.rs` exports `research_allow_flags()` (includes `WebFetch,WebSearch`) and `research_deny_flags()`.
- `.gitignore` includes `.sldo-logs/` and `output/`.

Subsequent milestones must not break the M1 CLI surface (positional `prompt_file`, `--prompt`, `--repo-dir`, `--output`, `--model`, `--max-iterations`, `--max-searches`).

---

### Milestone 2 — Research prompt builder

**Goal**: Introduce `crates/sldo-research/src/prompt.rs` with pure functions that construct the three research-phase Claude Code prompts: `build_exploration_prompt`, `build_deepening_prompt`, `build_repo_context_prompt`. Wire `main.rs` so that after preflight it prints the exploration prompt (debug/dry-run) and exits 0. No Claude invocation yet.

**Context**: `sldo-plan/src/main.rs` has `build_generate_prompt` / `build_review_prompt` as pure functions returning `String`; this is the pattern we imitate. Research prompts must instruct Claude Code to return findings in a structured-markdown shape (specific `##` headers) so the M3 loop can capture them verbatim and M6 synthesis can re-organise them reliably.

**Important design rule**: Prompt builders are pure — no file I/O, no network, no env reads. The only inputs are the function parameters. Keep the repo-dir reference in the prompt text as a `Display`-formatted path string (do not `canonicalize` inside the builder; that's the caller's job and already happens in `run()`).

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m1.md`** and apply: "Keep `prompt.rs` functions pure (no I/O)", "Write BDD tests in the source file (`#[cfg(test)] mod tests`) before production code".
3. Read these files before making changes:
   - `crates/sldo-plan/src/main.rs` — study `build_generate_prompt`, `build_review_prompt` (lines 106–262) for prompt construction style, format-literal usage, and embedded threat-model language.
   - `crates/sldo-research/src/main.rs` — verify M1 Cli surface and where prompt construction should be wired.
   - `crates/sldo-common/src/copilot.rs` — confirm `ClaudeInvocation::prompt` is just `String`; builders only need to return strings.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/prompt.rs` | NEW: `build_exploration_prompt`, `build_deepening_prompt`, `build_repo_context_prompt`, section-name consts, inline `#[cfg(test)] mod tests`. |
| `crates/sldo-research/src/main.rs` | Add `mod prompt;`. Wire: after preflight, call `prompt::build_exploration_prompt(&prompt_content, cli.repo_dir.as_deref())` and print its length + first line as an info-level summary. Still exit 0 before the full loop (that's M3). Keep the `_prompt_content` variable, make it non-underscored now. |
| `tests/e2e_research_m2.rs` | NEW: E2E test file (register under `[[test]]` in root `Cargo.toml`). |
| `Cargo.toml` (workspace root) | Add `[[test]] name = "e2e_research_m2" path = "tests/e2e_research_m2.rs"`. |

#### Step-by-Step

1. **Write BDD test stubs first** inside `crates/sldo-research/src/prompt.rs`'s `#[cfg(test)] mod tests` block (see scenarios below).
2. **Write E2E stub** at `tests/e2e_research_m2.rs` and register it in root `Cargo.toml`.
3. Create `crates/sldo-research/src/prompt.rs`:
   - Constants: `SECTION_TOPIC_DECOMPOSITION = "## Topic Decomposition"`, `SECTION_KEY_QUESTIONS`, `SECTION_REPO_CONTEXT`, `SECTION_INITIAL_FINDINGS`, `SECTION_DEEPENED_FINDINGS`, `SECTION_LIBRARY_EVAL`, `SECTION_ARCHITECTURE_OPTIONS`, `SECTION_UNANSWERED_QUESTIONS`, `SECTION_TECH_STACK`, `SECTION_PROJECT_STRUCTURE`, `SECTION_BUILD_AND_TEST`, `SECTION_EXISTING_PATTERNS`, `SECTION_CONSTRAINTS`.
   - `pub fn build_exploration_prompt(prompt_content: &str, repo_dir: Option<&std::path::Path>) -> String` — instructs Claude Code to (a) decompose the topic into 5–10 specific sub-questions, (b) if repo path is given, explore it for tech stack + existing patterns + dependencies, (c) identify key concepts/libraries/APIs/standards, (d) output under the four exploration-phase section headers.
   - `pub fn build_deepening_prompt(prompt_content: &str, previous_findings: &str, iteration: u32, repo_dir: Option<&std::path::Path>) -> String` — embeds `previous_findings` (truncated to, e.g., 32 KiB with a clear marker if truncated), asks Claude Code to answer unanswered questions, evaluate specific libraries with pros/cons, and output under the four deepening-phase headers. When `iteration >= 3`, the prompt also asks for consolidation (prepares the ground for M6 synthesis).
   - `pub fn build_repo_context_prompt(repo_dir: &std::path::Path) -> String` — instructs Claude Code to read `README*`, `Cargo.toml`/`package.json`/`pyproject.toml`/`go.mod`, top-level directory structure, identify tech stack/build/test/patterns/constraints, output under the five repo-context headers.
   - Each builder uses `format!(r#"…"#)` with explicit section names so that M4's `validate_dossier` can later check for them.
4. Add `mod prompt;` at the top of `crates/sldo-research/src/main.rs`. Import `prompt::{build_exploration_prompt, build_deepening_prompt, build_repo_context_prompt}` where needed.
5. In `main.rs::run()`, after preflight and after the `_prompt_content` read, build the exploration prompt and print its byte length and first line via `info(…)`. Keep the `info("Research not yet implemented.")` line; change text to "Research loop pending (milestone 3)".
6. Make all BDD tests pass.
7. Run `cargo test --workspace` and confirm green.
8. Run `cargo build --workspace` and `cargo test --test e2e_research_m2 -- --test-threads=1`.
9. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`.
10. Run `cargo audit` only if new dependencies were introduced (none expected for M2 — `prompt.rs` uses only `std` and `chrono` which is already a workspace dep).
11. Review `.gitignore` — no new artifact paths expected in M2 (prompt builders are pure, no files written). Confirm `output/` and `.sldo-logs/` entries from M1 are still present.

#### BDD Acceptance Scenarios

**Feature: Exploration prompt construction**

| Scenario | Given | When | Then |
|---|---|---|---|
| With repo dir includes repo reference | prompt text "add OAuth2" + `Some(Path::new("/tmp/repo"))` | `build_exploration_prompt(...)` | returned string contains the user prompt text, the literal "/tmp/repo", and all four exploration section headers |
| Without repo dir omits repo reference | prompt text + `None` | `build_exploration_prompt(...)` | returned string contains user prompt text and section headers, but does **not** contain a `Repo Context` section or `/tmp` |
| Output format instruction present | any input | `build_exploration_prompt(...)` | contains "## Topic Decomposition", "## Key Questions", "## Initial Findings" |
| Large prompt preserved | 10 KiB user text | `build_exploration_prompt(...)` | returned string contains the full 10 KiB user text verbatim |

**Feature: Deepening prompt construction**

| Scenario | Given | When | Then |
|---|---|---|---|
| References previous findings | `previous_findings = "FOUND-MARKER-123"`, iteration 2 | `build_deepening_prompt(...)` | returned string contains "FOUND-MARKER-123" and "## Deepened Findings" |
| Iteration 3 asks synthesis | iteration 3 | `build_deepening_prompt(...)` | contains the word "synthes" or "consolidat" (case-insensitive) |
| Truncates very large findings | 1 MiB findings | `build_deepening_prompt(...)` | returned string length < 100 KiB, contains a truncation marker like "[truncated" |
| Iteration 1 rejected | iteration 0 or 1 | `build_deepening_prompt(...)` passes | (iteration 1 is allowed; test that function doesn't panic for iteration 1 — the caller controls when to invoke deepening vs exploration) |

**Feature: Repo-context prompt construction**

| Scenario | Given | When | Then |
|---|---|---|---|
| Includes repo path | `Path::new("/proj/x")` | `build_repo_context_prompt(...)` | contains "/proj/x" |
| Section coverage | any repo path | `build_repo_context_prompt(...)` | contains all of "## Tech Stack", "## Project Structure", "## Build & Test", "## Existing Patterns", "## Constraints" |

#### Regression Tests

- `cargo test --workspace` — all prior tests remain green (290+).
- `target/debug/sldo-research --help` still prints usage including M1 flags.
- `target/debug/sldo-plan --help` and `target/debug/sldo-run --help` unchanged.
- All tests in `tests/e2e_research_m1.rs` still pass.

#### E2E Runtime Validation

**File**: `tests/e2e_research_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_binary_still_accepts_m1_args` | M1 CLI surface preserved | `sldo-research --prompt "x"` exits 0 (or clear error if claude missing) |
| `test_run_logs_prompt_length` | Exploration prompt is built at runtime | stderr (or stdout) contains "prompt" or byte-count hint (plain text `N bytes`) |
| `test_prompt_module_does_not_leak_files` | Pure builder — no scratch files | no new files appear under `output/` or `.sldo-logs/` beyond logs that already exist |

#### Smoke Tests

- [ ] `cargo build --workspace` succeeds with zero warnings beyond known Tauri dead-code warnings.
- [ ] `cargo test -p sldo-research` passes (new tests included).
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `target/debug/sldo-research --prompt "evaluate async runtimes"` runs to completion with the new info line present.
- [ ] `git status` is clean after the full test suite runs.

#### Post-Flight

- **ARCHITECTURE.md**: Add a subsection "### sldo-research — Research Prompt Builder (M2)" under the CLI tools section describing the three prompt builders.
- **README.md**: No user-facing change yet (tool is still not research-capable).
- **Other docs**: `docs/lessons/research-m2.md`, `docs/completion/research-m2.md`.

---

### Milestone 3 — Claude Code–driven research loop

**Goal**: Implement the core research loop in `crates/sldo-research/src/research.rs`: invoke Claude Code CLI once for repo context (if `--repo-dir` given), once for exploration, then `max_iterations - 1` deepening invocations. Capture stdout into a scratch file, feed it back into the next prompt. Return the accumulated findings to `main.rs`.

**Context**: `sldo-plan/src/main.rs` wires `ClaudeInvocation` through `toolflags::plan_allow_flags()` and uses `LogFile`; `sldo-run/src/main.rs` has the retry/attempt loop pattern. The research loop is closer to `sldo-plan`'s two-step shape than `sldo-run`'s retry loop — it's an **iterative refinement** loop, not a retry loop. Capture strategy: instruct Claude Code (via the prompt) to write each iteration's findings to `output/.research-scratch-iter-<N>.md`, then read the file back with `std::fs::read_to_string` at the end of each iteration. Fallback: if the scratch file doesn't exist, collect findings via `run_with_callback` stdout capture into an in-memory `String`.

**Important design rule**: Every Claude Code invocation gets its own `LogFile` under `.sldo-logs/` with a descriptive filename (`research-repo-context.log`, `research-exploration.log`, `research-deepen-<N>.log`). Non-zero exit codes are logged as warnings but do **not** halt the loop — the loop returns `Ok(accumulated)` as long as at least one invocation produced output.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m2.md`** and apply its guidance.
3. Read these files:
   - `crates/sldo-plan/src/main.rs` — study the `ClaudeInvocation` construction pattern and `LogFile` usage (lines 453–508).
   - `crates/sldo-run/src/main.rs` — study the main loop (lines 276–337), `build_cmd_summary`, and how `run_with_callback` is chained.
   - `crates/sldo-common/src/copilot.rs` — confirm `run_with_callback` signature.
   - `crates/sldo-common/src/logging.rs` — confirm `LogFile::new` and `ensure_log_dir`.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/research.rs` | NEW: `ResearchConfig` struct, `research_loop(&ResearchConfig) -> Result<String>`, scratch-file helpers, unit tests. |
| `crates/sldo-research/src/main.rs` | Add `mod research;`. After preflight, build a `ResearchConfig`, call `research::research_loop(&cfg)?`, print findings byte count. Still no dossier write (that's M4). |
| `tests/e2e_research_m3.rs` | NEW, registered under `[[test]]`. |
| `Cargo.toml` (workspace root) | Add `[[test]]` entry. |

#### Step-by-Step

1. Write BDD stubs inside `research.rs::tests` covering the scenarios below.
2. Write E2E stub at `tests/e2e_research_m3.rs`; register in root `Cargo.toml`.
3. Create `crates/sldo-research/src/research.rs`:
   - `pub struct ResearchConfig { pub prompt_content: String, pub repo_dir: Option<PathBuf>, pub output_path: PathBuf, pub model: String, pub max_iterations: u32, pub cooldown_secs: u64, pub log_dir: PathBuf }`.
   - `pub fn research_loop(cfg: &ResearchConfig) -> anyhow::Result<String>`:
     1. Ensure `cfg.output_path`'s parent directory exists (create if needed).
     2. Define `scratch_path(iter) = cfg.output_path.parent().unwrap().join(format!(".research-scratch-iter-{}.md", iter))`.
     3. If `cfg.repo_dir.is_some()`: run a repo-context invocation (prompt from `prompt::build_repo_context_prompt`), log to `research-repo-context.log`; capture output; store as `repo_context_text`.
     4. Iteration 1: invoke Claude Code with `build_exploration_prompt(&cfg.prompt_content, cfg.repo_dir.as_deref())`, log to `research-exploration.log`; capture findings; persist to `scratch_path(1)` for observability.
     5. Iterations 2..=`cfg.max_iterations`: build `build_deepening_prompt(&cfg.prompt_content, &accumulated, iter, cfg.repo_dir.as_deref())`; invoke; persist to `scratch_path(iter)`; append result to `accumulated`.
     6. Between iterations: `std::thread::sleep(Duration::from_secs(cfg.cooldown_secs))`.
     7. Concurrent/graceful-degradation: if a Claude invocation returns non-zero, log a warning and proceed using whatever text was captured.
   - Use `ClaudeInvocation { prompt, model: cfg.model.clone(), allow_flags: toolflags::research_allow_flags(), deny_flags: toolflags::research_deny_flags(), working_dir: cfg.repo_dir.clone().unwrap_or(std::env::current_dir()?) }`.
   - Use `run_with_callback` so that stdout is both captured into `accumulated` and printed to the user's terminal.
4. Wire into `main.rs::run()`:
   - After preflight, build `ResearchConfig` from `cli` + `prompt_content` + `log_dir = ensure_log_dir(&working_dir)?` where `working_dir` is `cli.repo_dir` if set else `std::env::current_dir()?`.
   - Call `research_loop(&cfg)?`.
   - Print `info!("Research accumulated {} bytes of findings", findings.len())`.
   - Still no dossier write — that's M4.
5. Make all BDD tests pass.
6. Run the full test suite + E2E: `cargo test --workspace` then `cargo build --workspace && cargo test --test e2e_research_m3 -- --test-threads=1`.
7. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings` (no new warnings allowed).
8. Run `cargo audit` only if new dependencies were introduced (none expected for M3 — `research.rs` uses only existing workspace deps).
9. Review `.gitignore` — M3 writes scratch files to `output/.research-scratch-iter-<N>.md` and logs to `.sldo-logs/research-*.log`; both parent dirs are already gitignored in M1, so no new entries are required. Verify with `git status` after a run.

#### BDD Acceptance Scenarios

**Feature: Research loop orchestration**

| Scenario | Given | When | Then |
|---|---|---|---|
| Config struct constructs | valid fields | `ResearchConfig { … }` | compiles; all fields publicly accessible |
| No repo dir skips context invocation | `cfg.repo_dir = None`, claude unavailable | `research_loop(&cfg)` | no repo-context log file is created; overall either returns Ok(accumulated) (possibly empty) or Err with claude-spawn message |
| With repo dir attempts context invocation | `cfg.repo_dir = Some(tmp)` where tmp is a real dir, claude unavailable | `research_loop(&cfg)` | behaviour is deterministic: either Ok(accumulated) or a spawn error — never panic |
| Iteration count respected | `max_iterations = 1` | `research_loop(&cfg)` mocked via a non-existent claude | no deepening invocation attempted (verifiable via log absence of `research-deepen-2.log`) |
| Scratch file persisted | successful exploration | `research_loop(&cfg)` | `output/.research-scratch-iter-1.md` exists and matches accumulated text |
| Non-zero exit continues | hypothetical Claude exit 1 during deepening | `research_loop(&cfg)` | returns `Ok(partial)` with partial findings; warning logged |
| Cooldown respected | `cooldown_secs = 2`, `max_iterations = 2` | time `research_loop` | ≥ 2s elapsed between first and second invocation start (verifiable via log timestamps) |

**Feature: Log file naming**

| Scenario | Given | When | Then |
|---|---|---|---|
| Exploration log | any run | `research_loop` | `.sldo-logs/research-exploration.log` exists after the call |
| Deepening logs numbered | `max_iterations = 3` | `research_loop` | `.sldo-logs/research-deepen-2.log` and `.../research-deepen-3.log` exist |
| Repo-context log only if repo_dir | `repo_dir = None` | `research_loop` | `.sldo-logs/research-repo-context.log` does **not** exist |

#### Regression Tests

- All M1 + M2 unit and E2E tests remain green.
- `cargo test --workspace` end-to-end green.
- `target/debug/sldo-research --help` still prints M1 flags; none removed or renamed.

#### E2E Runtime Validation

**File**: `tests/e2e_research_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_log_directory_created` | `.sldo-logs/` is created even when claude is absent | directory exists after running `sldo-research --prompt "x"` with claude on a throwaway PATH, or test skips gracefully if claude is available |
| `test_single_iteration` | `--max-iterations 1` runs a single exploration pass and exits | binary exits 0 or with a clear claude-spawn error; no `research-deepen-2.log` |
| `test_research_config_struct` | struct is exported and constructible from the library side | unit-style compile check |

#### Smoke Tests

- [ ] `cargo build --workspace` clean.
- [ ] `target/debug/sldo-research --prompt "topic" --max-iterations 1` writes `.sldo-logs/research-exploration.log` (if claude installed) or prints a clear spawn error.
- [ ] `git status` clean — scratch files live under `output/` which is gitignored; logs under `.sldo-logs/` which is gitignored.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo test --workspace` is green (all prior ~290 tests plus new M3 tests).

#### Post-Flight

- **ARCHITECTURE.md**: Add "### Research loop (M3)" subsection describing the five-phase pipeline and log naming.
- **README.md**: No user-facing change yet.
- **Lessons**: `docs/lessons/research-m3.md`, `docs/completion/research-m3.md`.

---

### Milestone 4 — Dossier format, writer & validator

**Goal**: Define the research-dossier markdown format in `crates/sldo-research/src/dossier.rs`. Implement `write_dossier`, `validate_dossier`, and section constants. After the research loop returns, write an initial dossier from the raw findings (no synthesis yet — synthesis is M6), run `validate_dossier`, and print any issues as warnings without failing the CLI.

**Context**: `sldo-plan/src/main.rs` has `REQUIRED_SECTIONS` + `PLACEHOLDER_PATTERNS` + `validate_runbook` as the reference pattern (lines 267–372). The dossier format must be designed so that the file produced here can later (M7) be fed directly to `sldo-plan` as its `prompt_file`. Required dossier sections: "Executive Summary", "Topic Decomposition", "Key Findings", "Library & Tool Evaluations", "Architecture Options", "API & SDK Documentation", "Repository Context" (optional — only when `--repo-dir` was given), "Design Recommendations", "Risks & Open Questions", "References".

**Important design rule**: At M4 the dossier is a straight concatenation of: header frontmatter (topic, generated-on date, source-prompt excerpt), then the raw findings. M6 will replace the body with a synthesised version; at M4 we just wrap the raw text in the section skeleton. `validate_dossier` returns `Vec<String>` issues following the `validate_runbook` convention — never panics, never returns `Result`.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m3.md`**.
3. Read:
   - `crates/sldo-plan/src/main.rs` — `validate_runbook` (lines 290–372), `REQUIRED_SECTIONS`, `PLACEHOLDER_PATTERNS`.
   - `docs/runbook-template.md` (or `docs/runbook-template_v_3_template.md`) — reference for how section headers are written.
   - `crates/sldo-research/src/research.rs` (built in M3) — understand the return shape feeding the writer.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/dossier.rs` | NEW: section constants, `write_dossier`, `validate_dossier`, unit tests. |
| `crates/sldo-research/src/main.rs` | Add `mod dossier;`. After `research_loop`, call `dossier::write_dossier(&cli.output, &prompt_content, &findings, repo_context.as_deref())?`, then `validate_dossier(&cli.output)` and log issues as warnings. |
| `crates/sldo-research/src/research.rs` | Return a struct instead of `String`: `pub struct ResearchFindings { pub raw: String, pub repo_context: Option<String> }`. Minimal local refactor. |
| `tests/e2e_research_m4.rs` | NEW, registered. |
| `Cargo.toml` (workspace root) | Add `[[test]]` entry. |

#### Step-by-Step

1. Write BDD stubs inside `dossier.rs::tests` and E2E stub at `tests/e2e_research_m4.rs`.
2. Create `crates/sldo-research/src/dossier.rs`:
   - `pub const REQUIRED_SECTIONS: &[&str] = &["## Executive Summary", "## Topic Decomposition", "## Key Findings", "## Library & Tool Evaluations", "## Architecture Options", "## API & SDK Documentation", "## Design Recommendations", "## Risks & Open Questions", "## References"];` (Repository Context is conditionally required — handled separately).
   - `pub const PLACEHOLDER_PATTERNS: &[&str] = &["[TBD]", "[description]", "[findings]", "[to be filled]", "TODO:"];`
   - `pub fn write_dossier(path: &Path, prompt: &str, findings: &str, repo_context: Option<&str>) -> Result<()>`:
     - Create parent directories (`std::fs::create_dir_all`).
     - Write a markdown document beginning with a YAML-ish frontmatter block showing topic (first 200 chars of prompt), generated date from `chrono::Local::now()`, and a summary line including `prompt.len()`.
     - Embed all required sections. If `repo_context` is `Some`, insert a "## Repository Context" section with the raw repo-context text; otherwise omit that section entirely.
     - Body: wrap findings inside the "## Key Findings" section so validation can find content there; stub out "## Executive Summary", "## Design Recommendations", etc. with a short "To be synthesised in M6" note (note: this is a deliberate M4→M6 handoff; M6 replaces these notes with real content, and M7 re-validates that the handoff is complete).
   - `pub fn validate_dossier(path: &Path) -> Vec<String>`:
     - Mirror `validate_runbook` structure (file exists, size ≥ 500 bytes, required sections present, placeholder count, but do **not** reject if "To be synthesised in M6" markers are present — those are expected at M4).
     - **At M4**: the validator allows the "placeholder-ish" stub sentinel; M6's synthesis step removes it and M7's `check_plan_readiness` asserts its absence.
     - Returns `Vec<String>`.
3. Change `research::research_loop` return type to `Result<ResearchFindings>`. Update callers.
4. Wire in `main.rs::run()`:
   - Call `research::research_loop(&cfg)?` → `findings`.
   - `dossier::write_dossier(&cli.output, &prompt_content, &findings.raw, findings.repo_context.as_deref())?`.
   - `let issues = dossier::validate_dossier(&cli.output); for i in &issues { warn(i); }`.
   - Print a success line with dossier path and byte count.
5. Make tests green.
6. Run the full suite and E2E: `cargo test --workspace` then `cargo build --workspace && cargo test --test e2e_research_m4 -- --test-threads=1`.
7. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`.
8. Run `cargo audit` only if new dependencies were introduced (none expected for M4 — `dossier.rs` uses only `std` and the already-workspace-declared `chrono`).
9. Review `.gitignore` — M4 writes `output/research-dossier.md` (and optionally nested output paths under user-supplied `--output`); `output/` is already gitignored from M1. Confirm `git status` is clean after running the default-path smoke test.

#### BDD Acceptance Scenarios

**Feature: Dossier writer**

| Scenario | Given | When | Then |
|---|---|---|---|
| Writes file at path | temp dir + valid inputs | `write_dossier(path, prompt, findings, None)` | file exists; size ≥ 500 bytes; contains "## Executive Summary" and all other required sections |
| Creates parent directory | path `tmp/nested/dir/dossier.md` | `write_dossier(...)` | nested dirs created; file written |
| Repo context included | `Some("Tech: Rust")` | `write_dossier(..., Some("Tech: Rust"))` | file contains "## Repository Context" and the string "Tech: Rust" |
| Repo context omitted | `None` | `write_dossier(..., None)` | file does not contain "## Repository Context" header |
| Frontmatter includes date | any inputs | `write_dossier(...)` | file contains current year as a 4-digit string |
| Findings embedded under Key Findings | findings text "UNIQUE-MARKER-XYZ" | `write_dossier(..., "UNIQUE-MARKER-XYZ", ...)` | file contains "UNIQUE-MARKER-XYZ" and it appears after "## Key Findings" |

**Feature: Dossier validator**

| Scenario | Given | When | Then |
|---|---|---|---|
| Complete dossier passes | file written by `write_dossier` with 2 KiB findings | `validate_dossier(path)` | returns empty vec |
| Missing file | nonexistent path | `validate_dossier(path)` | returns non-empty vec mentioning "not created" or "not found" |
| Too small | 100-byte file | `validate_dossier(path)` | returns vec containing a size-related issue |
| Missing section | hand-written dossier missing "## Key Findings" | `validate_dossier(path)` | returns vec containing "Missing section: ## Key Findings" |
| Placeholder detected | dossier contains "[TBD]" | `validate_dossier(path)` | returns vec containing a placeholder-related issue |

#### Regression Tests

- `validate_runbook` in `sldo-plan` is unchanged (no shared code with `validate_dossier`).
- `cargo test -p sldo-plan` still reports the prior 24 tests.
- All M1/M2/M3 tests still green.

#### E2E Runtime Validation

**File**: `tests/e2e_research_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_dossier_created_when_claude_missing` | Pipeline still writes a (minimal) dossier when findings are empty | file exists at `--output` path; validator reports size warning but pipeline exits 0 |
| `test_default_output_path_respected` | Default `output/research-dossier.md` works | after run, that path exists under a tempdir CWD (set via `current_dir` in Command) |
| `test_custom_output_path_respected` | `--output custom.md` works | file at custom path exists |
| `test_nested_output_dir_created` | writer creates missing parent dirs | `--output deep/new/dir/x.md` yields the full path |

#### Smoke Tests

- [ ] `target/debug/sldo-research --prompt "topic" --max-iterations 1 --output /tmp/m4-dossier.md` yields a file at that path containing the required section headers.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] All existing E2E files still pass: `cargo test --test e2e_research_m1 --test e2e_research_m2 --test e2e_research_m3`.
- [ ] `git status` clean.

#### Post-Flight

- **ARCHITECTURE.md**: Add "### Dossier format (M4)" subsection listing required sections.
- **README.md**: Brief mention of dossier output path (still preliminary — full docs land in M7).
- **Lessons**: `docs/lessons/research-m4.md`, `docs/completion/research-m4.md`.

---

### Milestone 5 — Web search phase integration

**Goal**: Add a dedicated web-search phase to the research loop. Extend `prompt.rs` with `build_websearch_prompt(topic, questions, search_index)`. Modify `research::research_loop` to run up to `cfg.max_searches` web-search invocations between exploration and deepening. Failures during web search never halt the loop.

**Context**: `toolflags::research_allow_flags()` already includes `WebFetch,WebSearch` (M1 shipped this). Claude Code CLI with that flag can use its built-in web tools. The new prompt must instruct Claude Code to search for current documentation, library versions, and community best-practice articles, and return findings under dedicated section headers.

**Important design rule**: The web-search phase is driven entirely by prompt engineering — no new Rust HTTP client, no reqwest dependency, no API integration. `--max-searches 0` must cleanly skip the phase.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m4.md`**.
3. Read:
   - `crates/sldo-research/src/prompt.rs` (M2) and `research.rs` (M3) for extension shape.
   - `crates/sldo-common/src/toolflags.rs` — confirm web flags already present.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/prompt.rs` | Add `build_websearch_prompt(topic: &str, questions: &str, search_index: u32) -> String` + unit tests. |
| `crates/sldo-research/src/research.rs` | Insert web-search phase between exploration (iteration 1) and deepening (iterations 2..=max_iterations). Loop 1..=`cfg.max_searches` invocations. Log files named `research-websearch-<N>.log`. |
| `crates/sldo-research/src/main.rs` | Pass `max_searches` through `ResearchConfig`. (Likely already present from M3 — just verify.) |
| `tests/e2e_research_m5.rs` | NEW, registered. |
| `Cargo.toml` (workspace root) | Add `[[test]]` entry. |

#### Step-by-Step

1. Write BDD stubs for `build_websearch_prompt` inside `prompt.rs::tests`.
2. Write E2E stub `tests/e2e_research_m5.rs`.
3. Implement `build_websearch_prompt`:
   - Accept a topic summary and the list of sub-questions from the exploration output.
   - Partition questions across `search_index` calls (for `search_index == 1` focus on the top-priority questions; each successive index focuses on the next slice).
   - Require Claude Code to output under three headers: `## Web Search Results`, `## Documentation Found`, `## Library Versions`.
   - Explicitly instruct Claude to list URLs with their titles so M6 can extract references.
4. In `research.rs`:
   - Add the loop between exploration and deepening: `for n in 1..=cfg.max_searches { … invoke `build_websearch_prompt` … append to accumulated … LogFile::new(&cfg.log_dir, &format!("research-websearch-{}.log", n))? … }`.
   - Handle non-zero Claude exit like the other phases (log warning, continue).
   - When `max_searches == 0`, skip the entire phase (no log files, no invocations).
5. Make all tests green.
6. Run the full suite and E2E: `cargo test --workspace` then `cargo build --workspace && cargo test --test e2e_research_m5 -- --test-threads=1`.
7. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`.
8. Run `cargo audit` only if new dependencies were introduced (none expected for M5 — web search is prompt-driven; no new Rust crates).
9. Review `.gitignore` — M5 writes `.sldo-logs/research-websearch-<N>.log`; `.sldo-logs/` is already gitignored. Confirm `git status` is clean after a run with `--max-searches 2`.

#### BDD Acceptance Scenarios

**Feature: Web search prompt**

| Scenario | Given | When | Then |
|---|---|---|---|
| Contains output-format headers | any inputs | `build_websearch_prompt(...)` | string contains "## Web Search Results", "## Documentation Found", "## Library Versions" |
| Search index varies prompt | same topic+questions, index 1 vs 2 | `build_websearch_prompt` called twice | strings differ (different question slice) |
| Empty questions fallback | `questions = ""` | `build_websearch_prompt("topic", "", 1)` | string contains "topic" and a fallback instruction like "research broadly" |

**Feature: Web search phase**

| Scenario | Given | When | Then |
|---|---|---|---|
| `max_searches = 0` skips phase | cfg with max_searches 0 | `research_loop` | no `research-websearch-*.log` files created |
| `max_searches = 3` creates 3 logs | cfg with max_searches 3, claude available | `research_loop` | three log files named `research-websearch-1.log` … `research-websearch-3.log` |
| Web failure doesn't halt | one web invocation returns non-zero | `research_loop` | deepening still runs; returns Ok(findings) |

**Feature: Tool flag preservation**

| Scenario | Given | When | Then |
|---|---|---|---|
| Research flags include Web | call `research_allow_flags()` | inspect output | contains both "WebFetch" and "WebSearch" |
| Plan flags unchanged | call `plan_allow_flags()` | inspect output | does **not** contain "WebSearch" (regression guard) |

#### Regression Tests

- `cargo test -p sldo-common` — all toolflag tests still pass.
- `cargo test -p sldo-plan` — unchanged.
- `cargo test -p sldo-run` — unchanged.
- M1–M4 E2E tests remain green.

#### E2E Runtime Validation

**File**: `tests/e2e_research_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_max_searches_zero_accepted` | CLI accepts `--max-searches 0` | exit 0 (or claude-spawn error) |
| `test_max_searches_zero_skips_phase` | No web log files when count is zero | after run, no file matches `.sldo-logs/research-websearch-*.log` |
| `test_websearch_log_files_named_correctly` | Log naming consistent | after a run with `--max-searches 2` and claude missing, either 0 logs or spawn-error; filename pattern regex passes |

#### Smoke Tests

- [ ] `target/debug/sldo-research --prompt "compare web frameworks" --max-searches 0 --max-iterations 1` runs and produces a dossier without web phase.
- [ ] `target/debug/sldo-research --prompt "compare web frameworks" --max-searches 2 --max-iterations 1` (with claude installed) produces `.sldo-logs/research-websearch-1.log` and `.sldo-logs/research-websearch-2.log`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo test --workspace` is green (all prior tests plus new M5 tests).
- [ ] `git status` clean.

#### Post-Flight

- **ARCHITECTURE.md**: Add "### Web search phase (M5)" describing the prompt-driven approach and tool flags.
- **README.md**: No user-facing change yet.
- **Lessons**: `docs/lessons/research-m5.md`, `docs/completion/research-m5.md`.

---

### Milestone 6 — Multi-source synthesis pass

**Goal**: Add a final synthesis pass that takes all accumulated raw findings (exploration + web search + deepening) and produces a coherent, deduplicated, well-organised dossier body conforming exactly to the M4 section structure. Replace the M4 "To be synthesised in M6" stub sentinels with real synthesised content.

**Context**: After M3–M5, `research::research_loop` returns concatenated raw text from multiple Claude Code invocations. That text is likely repetitive or contradictory. This milestone adds one final Claude Code invocation whose prompt instructs it to read the raw text and emit the full dossier body in the exact M4 format, with confidence levels (high/medium/low) on recommendations and explicit "Open Questions" flagging.

**Important design rule**: The synthesis prompt must embed the M4 section list verbatim so Claude Code cannot invent new section names. On synthesis-Claude-failure, fall back to the raw concatenation (the pre-M6 behaviour) — the dossier must always be written.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m5.md`**.
3. Read:
   - `crates/sldo-research/src/dossier.rs` (M4) — know the exact section constants to embed in the synthesis prompt.
   - `crates/sldo-research/src/research.rs` (M3/M5) — know where to hook the synthesis step.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/prompt.rs` | Add `build_synthesis_prompt(prompt: &str, all_findings: &str, repo_context: Option<&str>) -> String` + tests. |
| `crates/sldo-research/src/research.rs` | After deepening, run a synthesis invocation. Update return type to include `synthesised: Option<String>` alongside `raw` (minimal local refactor). |
| `crates/sldo-research/src/dossier.rs` | Extend `write_dossier` to accept `synthesised: Option<&str>`: if `Some`, replace the M4 stub body with the synthesised content; if `None`, keep raw findings wrapped as before. Keep `validate_dossier` signature unchanged. |
| `crates/sldo-research/src/main.rs` | Pass the synthesised text (if any) to `write_dossier`. |
| `tests/e2e_research_m6.rs` | NEW, registered. |
| `Cargo.toml` (workspace root) | Add `[[test]]` entry. |

#### Step-by-Step

1. Write BDD stubs.
2. Write E2E stub `tests/e2e_research_m6.rs`.
3. Implement `build_synthesis_prompt`:
   - Embed `dossier::REQUIRED_SECTIONS` verbatim.
   - Instruct Claude Code to: deduplicate, resolve contradictions preferring more recent/authoritative sources, rank recommendations by confidence (high/medium/low), list open questions under "## Risks & Open Questions", and extract all URLs into "## References".
   - Include the raw findings (truncated at ≈100 KiB) and optional repo_context.
4. Update `research_loop` to invoke Claude Code once more at the end:
   - Log file `research-synthesis.log`.
   - Capture output into `synthesised: Option<String>`. If invocation fails or output is empty, set `None` and log a warning.
5. Update `write_dossier` to accept and use `synthesised`.
6. Ensure `validate_dossier` now rejects the M4 stub sentinel ("To be synthesised in M6") when a synthesis pass was attempted — add a separate `check_synthesis_complete(path: &Path) -> Vec<String>` helper for M7's plan-readiness to use; don't mutate `validate_dossier` itself (it still succeeds for an M4-style dossier).
7. Make tests green; run fmt, clippy, audit (only if deps changed — none expected).

#### BDD Acceptance Scenarios

**Feature: Synthesis prompt**

| Scenario | Given | When | Then |
|---|---|---|---|
| Includes all section headers | any input | `build_synthesis_prompt(...)` | string contains every entry in `dossier::REQUIRED_SECTIONS` |
| Requests confidence levels | any input | `build_synthesis_prompt(...)` | string contains "confidence" or "high/medium/low" |
| Requests deduplication | any input | `build_synthesis_prompt(...)` | string contains "deduplicate" or "merge duplicates" |
| Embeds raw findings | findings = "X-UNIQUE-Y" | `build_synthesis_prompt(..., "X-UNIQUE-Y", ...)` | contains "X-UNIQUE-Y" |
| Truncates over 100KiB | findings = 1MiB | prompt length | < 150KiB and contains truncation marker |

**Feature: Synthesis integration**

| Scenario | Given | When | Then |
|---|---|---|---|
| Success → synthesised body in dossier | synthesis Claude returns new text "SYNTH-MARKER" | `research_loop` then `write_dossier` | file contains "SYNTH-MARKER" and not the M4 stub sentinel |
| Failure → raw fallback | synthesis invocation returns non-zero | pipeline | dossier still written; contains raw findings marker; warning logged |
| Synthesis log exists | any successful run | `.sldo-logs/` | `research-synthesis.log` exists |

**Feature: `check_synthesis_complete`**

| Scenario | Given | When | Then |
|---|---|---|---|
| Detects M4 stub sentinel | dossier still contains "To be synthesised in M6" | `check_synthesis_complete(path)` | returns non-empty vec |
| Clean synthesised dossier | no stub sentinel present | `check_synthesis_complete(path)` | returns empty vec |

#### Regression Tests

- M1–M5 tests all green.
- `validate_runbook` in sldo-plan unaffected.
- Existing `validate_dossier` behaviour unchanged (new check is additive).

#### E2E Runtime Validation

**File**: `tests/e2e_research_m6.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_synthesis_log_created_when_claude_ok` | Pipeline attempts synthesis | `.sldo-logs/research-synthesis.log` exists OR test skips with "claude not installed" |
| `test_synthesis_fallback_still_writes_dossier` | Fallback path works | dossier file exists even when synthesis fails; dossier contains raw findings |
| `test_check_synthesis_complete_pure` | Pure checker | using a fixture file, returns expected vec |

#### Smoke Tests

- [ ] `target/debug/sldo-research --prompt "test topic" --max-iterations 1 --max-searches 0` produces a dossier. If claude is installed, `grep -c "To be synthesised" output/research-dossier.md` → 0.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] All earlier E2E tests still pass.
- [ ] `git status` clean.

#### Post-Flight

- **ARCHITECTURE.md**: Add "### Synthesis pass (M6)" describing the final phase and its section structure.
- **README.md**: No user-facing change yet.
- **Lessons**: `docs/lessons/research-m6.md`, `docs/completion/research-m6.md`.

---

### Milestone 7 — Plan-ready output & sldo-plan integration

**Goal**: Add `check_plan_readiness()` to `dossier.rs` that layers additional constraints on top of `validate_dossier` to confirm the dossier is suitable as input to `sldo-plan`. At the end of a successful run, print a next-step command suggestion. Document the full pipeline in `README.md` and `docs/ARCHITECTURE.md`. Prove the integration end-to-end by a test that parses the dossier with `sldo-plan`'s input expectations.

**Context**: `sldo-plan/src/main.rs` requires `prompt_file` to be a readable text file — it calls `std::fs::read_to_string` and embeds the content in `build_generate_prompt`. A plan-ready dossier must be valid UTF-8, > 1 KiB, and contain enough substance that the M4 section stubs are gone and "## Design Recommendations" + (at least one of "## Library & Tool Evaluations" / "## Architecture Options") are non-trivial. `sldo-plan` is **not** modified.

**Important design rule**: `check_plan_readiness` is additive — it does **not** relax or modify `validate_dossier`; it adds stricter checks used only at the end of `sldo-research`'s run. `sldo-plan` remains untouched. If the dossier fails plan-readiness, `sldo-research` still exits 0 but prints clear warnings and does **not** print the next-step suggestion.

#### Pre-Flight

1. Complete the Pre-Milestone Protocol.
2. **Read `docs/lessons/research-m6.md`**.
3. Read:
   - `crates/sldo-plan/src/main.rs` — confirm `prompt_file` handling (lines 380–428) and `run()` signature are stable.
   - `crates/sldo-research/src/dossier.rs` (M6) — know the section constants.

#### Files Most Likely Touched

| File | Change |
|---|---|
| `crates/sldo-research/src/dossier.rs` | Add `pub fn check_plan_readiness(path: &Path) -> Vec<String>`. Reuse `validate_dossier` internally. |
| `crates/sldo-research/src/main.rs` | After `validate_dossier`, call `check_plan_readiness`. If empty, print success + suggested `sldo-plan` command; else print warnings. Print summary stats (elapsed time, iterations completed, searches performed, dossier bytes). |
| `tests/e2e_research_m7.rs` | NEW, registered. |
| `Cargo.toml` (workspace root) | Add `[[test]]` entry. |
| `docs/ARCHITECTURE.md` | Add full "sldo-research pipeline" section and update the CLI tool overview/table. |
| `README.md` | Add `### sldo-research — Generate a Research Dossier` section with full flag docs and pipeline example (`sldo-research → sldo-plan → sldo-run`). |

#### Step-by-Step

1. Write BDD stubs in `dossier.rs::tests` and E2E stub `tests/e2e_research_m7.rs`.
2. Implement `check_plan_readiness(path: &Path) -> Vec<String>`:
   - Run `validate_dossier`; merge its issues.
   - Assert file size > 1000 bytes.
   - Assert body contains "## Design Recommendations" with > 100 bytes of content after the header.
   - Assert body contains at least one of "## Library & Tool Evaluations" or "## Architecture Options" with > 100 bytes of content.
   - Assert the M4 stub sentinel ("To be synthesised in M6") is absent.
   - Assert content is valid UTF-8 (`std::fs::read_to_string` success).
3. Wire `main.rs::run()`:
   - After `validate_dossier`, call `check_plan_readiness(&cli.output)`.
   - If empty: print a success block and the suggested command:
     ```
     ✔ Research dossier is ready for planning.

     Next step — generate a runbook:
       sldo-plan <dossier-path> <repo-dir> [-o docs/RUNBOOK.md]
     ```
   - Else: print warnings, do **not** print the suggestion.
   - Print summary stats: dossier byte count, elapsed wall time (same style as `sldo-plan`'s "Total wall time").
4. Update `docs/ARCHITECTURE.md`:
   - Add `sldo-research` to the top-level "Workspace Structure" tree.
   - Add a "CLI tools" section covering `sldo-plan`, `sldo-run`, `sldo-research`.
   - Add "## sldo-research Pipeline" subsection covering the five-phase research loop and dossier format.
   - Update "Test Architecture" tables with M2–M7 E2E files.
5. Update `README.md`:
   - Add `### sldo-research — Generate a Research Dossier` after the `sldo-plan` section.
   - Document every flag (`prompt_file`, `--prompt`, `--repo-dir`, `--output`, `--model`, `--max-iterations`, `--max-searches`).
   - Show the full pipeline example `sldo-research → sldo-plan → sldo-run`.
6. Make all tests green; run fmt, clippy, audit (no deps change expected).

#### BDD Acceptance Scenarios

**Feature: Plan-readiness check**

| Scenario | Given | When | Then |
|---|---|---|---|
| Complete dossier passes | synthesised dossier, 2 KiB, all required sections with content | `check_plan_readiness(path)` | returns empty vec |
| Too small flagged | 800-byte dossier | `check_plan_readiness(path)` | vec contains size-related issue |
| Missing Design Recs flagged | dossier without "## Design Recommendations" | `check_plan_readiness(path)` | vec contains missing-section issue |
| Still contains M4 stub flagged | dossier body contains "To be synthesised in M6" | `check_plan_readiness(path)` | vec contains stub-sentinel issue |
| No lib-eval OR arch-options flagged | dossier missing both | `check_plan_readiness(path)` | vec contains "evaluation or architecture options" issue |
| Non-UTF-8 flagged | raw bytes `[0xFF, 0xFE]` written to path | `check_plan_readiness(path)` | vec contains UTF-8 related issue |

**Feature: CLI end-of-run output**

| Scenario | Given | When | Then |
|---|---|---|---|
| Ready prints next step | dossier passes readiness | subprocess stderr/stdout | contains "sldo-plan" and the dossier path |
| Not ready skips suggestion | dossier fails readiness | subprocess stderr/stdout | does **not** contain "Next step" suggestion |
| Summary stats shown | any successful run | output | contains "Total wall time:" style line |

**Feature: Backward compatibility**

| Scenario | Given | When | Then |
|---|---|---|---|
| sldo-plan unchanged | built workspace | `target/debug/sldo-plan --help` | exits 0; usage matches pre-M7 snapshot |
| sldo-run unchanged | built workspace | `target/debug/sldo-run --help` | exits 0; usage matches pre-M7 snapshot |

#### Regression Tests

- All ~290 pre-existing tests remain green.
- M1–M6 E2E tests all remain green.
- `cargo test -p sldo-plan` — all 24 tests green.
- `cargo test -p sldo-run` — all 13 tests green.
- `cargo test -p sldo-common` — all 48+ tests green.

#### E2E Runtime Validation

**File**: `tests/e2e_research_m7.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_dossier_is_valid_utf8_text` | Dossier file is UTF-8 | `std::fs::read_to_string` succeeds after a run |
| `test_cli_prints_next_step_when_ready` | Success suggestion shows | for a synthetic "good" dossier pre-seeded at `--output` path using `--max-iterations 0 --max-searches 0` fast path (see note), stderr contains "sldo-plan" and the dossier path |
| `test_cli_omits_next_step_when_not_ready` | Suggestion suppressed on failure | with a pre-seeded too-small dossier and an early exit path, stderr does **not** contain "Next step" |
| `test_dossier_accepted_by_sldo_plan_as_prompt_file` | Integration: file opens as a prompt file for sldo-plan | spawn `target/debug/sldo-plan <dossier-path> <tmp-repo> --help` (note: `--help` short-circuits before any Claude call, but resolve_output_path still works; adjust to spawn with a non-existent file flag that validates path resolution) — or directly assert `std::fs::read_to_string(&dossier_path).is_ok()` + size > 1 KiB |
| `test_plan_run_help_unchanged` | Back-compat | `target/debug/sldo-plan --help` and `target/debug/sldo-run --help` both exit 0 |

> Note: For tests that need a "ready" dossier without a live Claude Code CLI, provide a fixture file under `tests/fixtures/research/` containing a hand-written valid dossier, and assert `dossier::check_plan_readiness(fixture)` returns empty vec.

#### Smoke Tests

- [ ] `target/debug/sldo-research --prompt "add feature-flag system" --repo-dir . --max-iterations 2 --max-searches 1` runs end-to-end (with claude installed) and writes a plan-ready dossier.
- [ ] `target/debug/sldo-plan output/research-dossier.md . --help` shows usage (sanity check that sldo-plan still accepts file paths).
- [ ] `cargo test --workspace` is green.
- [ ] `cargo build --workspace` is green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo audit` passes (no new deps expected — if any were added, the Pre-Flight for that milestone would have flagged it).
- [ ] `target/debug/sldo-plan --help` and `target/debug/sldo-run --help` outputs are byte-identical to pre-M7 snapshots (captured at baseline).
- [ ] `git status` clean after test suite.
- [ ] `README.md` contains a runnable `sldo-research` example.
- [ ] `docs/ARCHITECTURE.md` has a `sldo-research` section with dossier format and pipeline diagram.

#### Post-Flight

- **ARCHITECTURE.md**: Major additive update — full pipeline + test table refresh.
- **README.md**: New `sldo-research` CLI section; pipeline workflow `sldo-research → sldo-plan → sldo-run`.
- **Lessons**: `docs/lessons/research-m7.md`, `docs/completion/research-m7.md`.

---

## Threat Model

Covers security-relevant risks to the **application features** being built in this runbook. CI/CD, cloud, and deployment are explicitly out of scope.

| # | Threat | Severity | Description | Mitigation (milestone) |
|---|---|---|---|---|
| T1 | Prompt injection via user-supplied prompt file | High | A hostile prompt file could contain Claude-Code-targeted instructions telling the agent to exfiltrate environment variables, modify source files outside the repo, or run arbitrary shell commands. | M2/M3: the Claude Code invocation runs with a tightly scoped `--allowedTools` list (no `Execute`/`Kill`; `Bash` is allow-listed — acknowledge this risk). Add documentation in M7 README cautioning users not to pass untrusted prompt files. Keep `working_dir` pinned to `--repo-dir` so file writes cannot escape the repo. Do **not** add an auto-approved `--dangerously-skip-permissions` flag anywhere in the code. |
| T2 | Prompt injection via web search content | High | Claude Code with `WebFetch`/`WebSearch` could ingest a malicious page that rewrites its own instructions. | M5: the web-search prompt explicitly tells Claude to treat fetched content as untrusted data and to summarise rather than execute instructions contained in retrieved pages. Log every `research-websearch-*.log` so the user can audit what was fetched. |
| T3 | Path traversal via `--output` | Medium | A user passing `--output ../../etc/passwd` could write outside the intended directory. | M4: keep the existing behaviour that `--output` is resolved from CWD (no auto-join to repo_dir) but document the risk in M7 README. Add a test (M4) that nested-directory creation works only under the CWD-relative path the user supplied. Do not call `canonicalize` before creating the parent — creating arbitrary paths is the user's responsibility, not an attack vector against the tool itself. |
| T4 | Sensitive content leaking into scratch/log files | Medium | The prompt, repo context, and Claude output are written to `.sldo-logs/` and `output/`. A user researching with a private-codebase repo could inadvertently share these logs. | M3/M7: ensure `.sldo-logs/` and `output/` are gitignored (already done in M1). Document clearly in M7 README that logs may contain proprietary source excerpts. Do not send logs to any network destination. |
| T5 | Denial of wallet via unbounded Claude Code invocations | Medium | A misconfigured run with `--max-iterations 1000 --max-searches 1000` could consume unreasonable API quota. | M3/M5: clap default values stay bounded (3 iterations, 5 searches). M7 README documents that each invocation consumes Claude API credits. Consider (documented only, not implemented) a future `--max-total-invocations` cap if operators request it. |
| T6 | Supply-chain attack via new crate | Low | New Rust dependencies could introduce malicious or vulnerable code. | **No new dependencies** are introduced in this runbook — all milestones reuse `sldo-common`, `clap`, `anyhow`, `chrono` (all already workspace members). If any milestone ever needs a new crate, its Pre-Flight must run `cargo audit` and justify the choice. |
| T7 | UTF-8 / binary content in prompt file | Low | A non-UTF-8 prompt file could cause `std::fs::read_to_string` to error at an unexpected point in the pipeline. | M1 already handles this with `with_context(...)`. No further work needed. |
| T8 | Git safety bypass | Medium | Running research on the `main` branch could mix research scratch files into a stable branch by accident. | M1's preflight calls `check_git_safety` when `--repo-dir` is given. M3 must not bypass this when it constructs `ResearchConfig`. Confirm in M3 E2E that running on `main` exits non-zero when `--repo-dir` is set. |
| T9 | Race between concurrent `sldo-research` runs | Low | Two concurrent runs writing to the same `--output` path could interleave content. | M3/M4: document that the tool is single-run per output path. Do not implement a lock file. If parallel use emerges as a pattern, address it then. |
| T10 | Secrets in `--prompt` inline argument | Medium | A user passing `--prompt "my API key is sk-…"` would write it into `.sldo-logs/` permanently. | M7 README documents this; no implementation change. Scrubbing arbitrary secrets is out of scope — trust the user to avoid inline secrets. |

Severity rubric: **High** = confidentiality/integrity compromise with plausible attacker path; **Medium** = operational footgun or privilege misuse; **Low** = defensive depth.

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update |
|---|---|---|
| 1 (done) | Added `sldo-research` to CLI tools section; documented M1 scaffold. | Added brief `sldo-research` placeholder. |
| 2 | Add "### sldo-research — Research Prompt Builder (M2)" subsection listing the three builders and output section headers. | No change. |
| 3 | Add "### Research loop (M3)" subsection: five-phase pipeline, log naming, `ResearchConfig`/`ResearchFindings`. | No change. |
| 4 | Add "### Dossier format (M4)" with `REQUIRED_SECTIONS` table. | Brief mention of dossier output path. |
| 5 | Add "### Web search phase (M5)" noting `research_allow_flags()` web tools and prompt-driven approach. | No change. |
| 6 | Add "### Synthesis pass (M6)" covering `build_synthesis_prompt` and fallback path. | No change. |
| 7 | Full pipeline section: `sldo-research → sldo-plan → sldo-run`; update Test Architecture table with M2–M7 counts. | New `### sldo-research — Generate a Research Dossier` CLI section with all flags and full pipeline example. |
