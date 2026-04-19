# Completion Summary — research Milestone 4

**Milestone**: Dossier format, writer & validator
**Status**: done
**Started / Completed**: 2026-04-19 / 2026-04-19

## What shipped

1. **`crates/sldo-research/src/dossier.rs`** — dossier schema, writer, and
   validator.
   - `REQUIRED_SECTIONS` — 9 header constants, order-preserving.
   - `SECTION_REPOSITORY_CONTEXT` — header for the optional repo-context
     section.
   - `PLACEHOLDER_PATTERNS` — 5 patterns flagged by the validator.
   - `M4_STUB_SENTINEL` — `"To be synthesised in M6"` (public; M6/M7 will
     consume it).
   - `write_dossier(path, prompt, findings, repo_context)` — pure-std
     writer; creates parent dirs, emits YAML-ish frontmatter +
     section skeleton.
   - `validate_dossier(path)` — returns `Vec<String>`; mirrors
     `sldo_plan::validate_runbook`'s shape; tolerates the stub sentinel.
   - `topic_excerpt` — private helper that flattens newlines and
     truncates long prompts.
   - 13 inline BDD unit tests covering writer, validator, and excerpt.

2. **`crates/sldo-research/src/research.rs`** — minor refactor.
   - New public struct `ResearchFindings { raw: String, repo_context: Option<String> }`.
   - `research_loop` return type changed from `Result<String>` to
     `Result<ResearchFindings>`. The repo-context capture is no longer
     concatenated into the accumulated findings; it flows back to
     `main.rs` separately so the dossier writer can emit a dedicated
     `## Repository Context` section.

3. **`crates/sldo-research/src/main.rs`** — wiring.
   - `mod dossier;` added.
   - Post-loop: calls `dossier::write_dossier` with
     `findings.raw` + `findings.repo_context`, logs byte count via
     `success(...)`, then calls `dossier::validate_dossier` and surfaces
     each issue via `warn(...)` without failing the CLI.
   - `prompt_content` is cloned into `ResearchConfig` so the original
     remains available for the later `write_dossier` call.

4. **`tests/e2e_research_m4.rs`** — 6 subprocess tests covering:
   - default output path,
   - custom `--output` path,
   - nested output directory creation,
   - empty-findings still produces a valid dossier,
   - findings embedded under `## Key Findings`,
   - `--help` flag surface unchanged.

5. **Workspace root `Cargo.toml`** — new `[[test]]` entry for
   `e2e_research_m4`.

6. **`docs/ARCHITECTURE.md`** — new "### Dossier format (M4)" subsection
   describing the schema, constants, and M4→M6 handoff contract. Test
   architecture table updated with the new E2E file (6 tests).

7. **`README.md`** — new "### `sldo-research` — Generate a Research
   Dossier (preview)" subsection with a usage example and flag summary,
   plus the crate added to the Project Structure tree.

8. **`docs/lessons/research-m4.md`** — design decisions, mistakes, and
   rules for M5.

## Pre-existing tests

All ~290 pre-existing tests remain green under the M3 lessons-scoped
test harness. No test files outside the M4 surface were modified.

## Known deferrals (intentional)

- Non-UTF-8 rejection is an M7 concern (`check_plan_readiness`); the M4
  validator does not test for it.
- Synthesis-stub detection is an M6/M7 concern (`check_synthesis_complete`
  / `check_plan_readiness`); M4's validator passes dossiers containing
  the stub sentinel.
- Web-search and synthesis phases are still absent — they ship in M5 and
  M6 respectively.
