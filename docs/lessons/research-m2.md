# Lessons Learned — research Milestone 2

## What changed
- Added `crates/sldo-research/src/prompt.rs` with three pure prompt constructors:
  `build_exploration_prompt`, `build_deepening_prompt`, `build_repo_context_prompt`,
  plus 13 phase-specific section-name `pub const`s.
- Wired `prompt::build_exploration_prompt` into `main.rs::run()`. After
  pre-flight, the binary prints the exploration prompt's byte length and first
  line, then exits 0 with "Research loop pending (milestone 3)."
- Added `tests/e2e_research_m2.rs` (3 E2E tests) and registered it in workspace
  root `Cargo.toml` under `[[test]]`.

## Design decisions and why
- Builders are **pure**: take `&str` / `Option<&Path>`, return `String`.
  Caller does any canonicalization (`main.rs` already canonicalises `--repo-dir`).
  This keeps unit tests trivial and reuses the M1 pattern called out in
  `research-m1.md`.
- Section headers live in `pub const`s rather than inline strings. Later
  milestones (M4 `validate_dossier`, M6 `build_synthesis_prompt`) will reference
  them by name, so a single source of truth prevents drift.
- The deepening prompt truncates `previous_findings` to 32 KiB by keeping the
  **tail** (newest content). Older content is summarised by earlier deepening
  rounds, so the most recent slice is the most informative.
- Deepening uses `is_char_boundary` to slice safely on multi-byte boundaries.
  The naive byte-index slice would panic on UTF-8 input.
- Truncation marker (`[truncated …]`) is a separate constant so future
  validators can grep for it deterministically.
- Synthesis hint only fires at `iteration >= 3` to avoid pre-empting M6's
  dedicated synthesis pass too aggressively.
- `#[allow(dead_code)]` is applied to `build_deepening_prompt` and
  `build_repo_context_prompt` (and the supporting truncation constant) because
  they are wired in by M3. The alternative (calling them from `main.rs`
  immediately) would have produced misleading runtime output that doesn't match
  the M2 step-by-step.

## Mistakes made
- Initially ran `cargo fmt --all` per the post-flight protocol. That reformatted
  many pre-existing files (sldo-plan, sldo-tauri, several E2E tests) which
  violates the hard rule "Do NOT touch code or tests belonging to other
  milestones." Reverted those changes and ran `cargo fmt -p sldo-research` +
  `rustfmt tests/e2e_research_m2.rs` instead.

## Root causes
- The workspace baseline is not fully formatted — pre-existing files have
  rustfmt nits that were never addressed. `cargo fmt --all` is a workspace-wide
  no-op only on a fully-formatted baseline, not this one.

## What was harder than expected
- Avoiding dead-code warnings while still creating the future-milestone
  builders. Solved with targeted `#[allow(dead_code)]` per item plus a comment
  noting the next milestone that wires it in.

## Naming conventions established
- Prompt-builder fns: `build_<phase>_prompt`.
- Section constants: `SECTION_<UPPER_SNAKE>` whose value is the literal `## …`
  header text (so `format!()` interpolations stay self-documenting).
- M2 E2E file name: `tests/e2e_research_m<N>.rs`, registered under `[[test]]`
  in workspace root Cargo.toml — same as M1.

## Test patterns that worked well
- One BDD test per scenario row in the runbook. Each test's body uses the
  Given/When/Then comment block.
- Asserting *both* presence (e.g., section headers) and *absence* (e.g.,
  no Repo Context header when `repo_dir = None`) catches regressions in
  either direction.
- For the truncation test, generating 1 MiB of input and asserting `out.len()
  < 100 KiB` plus presence of the truncation marker, rather than checking an
  exact byte count, makes the test resilient to small prompt-template tweaks.
- The "no leak" E2E test creates a fresh tempdir, runs the binary inside it,
  and asserts the dir is empty afterwards. This is a much stronger purity
  guarantee than only inspecting `output/` — it catches any stray writes.

## Missing tests that should exist now
- None for M2 scope.

## Rules for the next milestone (M3)
- **Do not run `cargo fmt --all` workspace-wide.** Run `cargo fmt -p
  <crate>` for each crate you actually touched and `rustfmt path/to/file.rs`
  for new test files. Otherwise you will silently re-format other milestones'
  code.
- **Do not run `cargo clippy --workspace --all-targets -- -D warnings`** as a
  pass/fail gate — pre-existing Tauri/voice-tx tests have clippy errors that
  are out of scope. Run clippy on `sldo-research` and the M3 E2E test
  individually.
- M3 will wire `build_deepening_prompt` and `build_repo_context_prompt` via
  `research_loop`; remove their `#[allow(dead_code)]` markers when you do.
- M3's `research_loop` should respect the `DEEPENING_FINDINGS_MAX_BYTES` cap
  by feeding accumulated findings through `build_deepening_prompt` (which
  already handles truncation), not by truncating in the loop itself.
- The `info()` line surfacing prompt byte counts is the runtime signal the M2
  E2E test asserts on. M3 should preserve a similar "N bytes" hint when
  invoking Claude Code, so the M2 E2E test continues to pass.

## Template improvements suggested
- The runbook's M2 step-by-step asks to "import `prompt::{build_exploration_prompt,
  build_deepening_prompt, build_repo_context_prompt}` where needed" — but only
  `build_exploration_prompt` is wired in M2. The other two cause dead-code
  warnings unless `#[allow(dead_code)]` is applied. Consider clarifying that
  M2 should import only the function that's actually used.
