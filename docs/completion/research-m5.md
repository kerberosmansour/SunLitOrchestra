# Completion Summary — research Milestone 5

**Milestone**: Web search phase integration
**Status**: done
**Started / Completed**: 2026-04-19 / 2026-04-19

## What shipped

1. **`crates/sldo-research/src/prompt.rs`** — new web-search prompt builder.
   - `build_websearch_prompt(topic: &str, questions: &str, search_index: u32) -> String`
     — pure constructor; partitions the question list across invocations
     and emits a Claude Code prompt that pins the three required output
     headers.
   - `SECTION_WEB_SEARCH_RESULTS`, `SECTION_DOCUMENTATION_FOUND`,
     `SECTION_LIBRARY_VERSIONS` — public header constants for the M6
     synthesis pass to grep against.
   - `WEBSEARCH_QUESTIONS_PER_INVOCATION` — private partition constant.
   - 6 inline BDD unit tests covering output-format headers,
     index-varies-prompt property, empty-questions fallback,
     URL+title instruction, hard rules, and an empty-topic edge case.

2. **`crates/sldo-research/src/research.rs`** — web-search phase added.
   - `research_loop` now runs `for n in 1..=cfg.max_searches` between
     exploration and deepening. Each invocation gets its own log file
     named `.sldo-logs/research-websearch-<N>.log`.
   - Per-search Claude failures log a warning and the loop continues —
     web phase failures never halt the pipeline.
   - `extract_key_questions(exploration_output: &str) -> String` —
     private helper that pulls the body of the `## Key Questions`
     section out of the exploration dump so the web-search prompt can
     partition it.
   - No cooldown is inserted between web-search invocations (web phases
     are lighter than deepening passes).
   - 5 new inline unit tests: `extract_key_questions` body extraction,
     missing-header fallback, last-section tail, plus the two tool-flag
     regression guards (`research_allow_flags_include_web_tools` and
     `plan_flags_do_not_include_web_search`).

3. **`crates/sldo-research/src/main.rs`** — wiring (no API changes).
   - `cfg.max_searches` already plumbed through from the M3 CLI.
   - Verified the M3 `info("Research accumulated N bytes…")` line is
     still emitted (the M3 E2E asserts on it).

4. **`tests/e2e_research_m5.rs`** — 4 subprocess tests covering:
   - `--max-searches 0` is accepted by the CLI (exit 0).
   - `--max-searches 0` skips the phase entirely (no
     `research-websearch-*.log` files).
   - `--max-searches 2` produces exactly two log files named
     `research-websearch-1.log` and `research-websearch-2.log`.
   - `--help` still documents `--max-searches` after M5.

5. **Workspace root `Cargo.toml`** — new `[[test]]` entry for
   `e2e_research_m5`.

6. **`docs/ARCHITECTURE.md`** — new "### Web search phase (M5)"
   subsection under the dossier format section, describing the
   prompt-driven approach, the three required output headers, the
   skip-phase semantics, and the no-cooldown asymmetry vs deepening.
   Test architecture table updated with the new E2E file (4 tests).

7. **`docs/lessons/research-m5.md`** — design decisions, mistakes
   (cooldown removed in fc0d3c2), naming conventions, and rules for M6.

## Smoke tests passed

- `target/debug/sldo-research --prompt "compare web frameworks" --max-searches 0 --max-iterations 1`
  produces a dossier and zero `research-websearch-*.log` files (verified
  with a `claude` shim under `/tmp/sldo_m5_smoke/shim`).
- `target/debug/sldo-research --prompt "compare web frameworks" --max-searches 2 --max-iterations 1`
  produces `.sldo-logs/research-websearch-1.log` and
  `.sldo-logs/research-websearch-2.log` plus the exploration log.
- `cargo test -p sldo-research` — 55 unit tests pass.
- `cargo test --test e2e_research_m1 --test e2e_research_m2 --test e2e_research_m3 --test e2e_research_m4 --test e2e_research_m5 -- --test-threads=1`
  — all 28 research E2E tests pass.
- `cargo clippy -p sldo-research --tests -- -D warnings` clean.
- `cargo clippy --test e2e_research_m5 -- -D warnings` clean.
- `cargo fmt -p sldo-research` produced minor reformatting only.

## Pre-existing tests

All ~290 pre-existing tests remain green under the M3 lessons-scoped
test harness. The pre-existing `frontend_dist_exists_after_build`
failure in `e2e_tauri_m1` (esbuild/Vite arm64 platform issue) is
unrelated to research milestones — same baseline as M3 and M4.

## Known deferrals (intentional)

- Synthesis of the multi-source raw findings is M6's job.
- Reference extraction (turning the URL+title pairs into the dossier's
  `## References` section) is also M6's job — M5 only ensures the raw
  findings carry the URLs in a recognisable format.
- No new dependencies were added — the phase is prompt-driven; Claude
  Code's built-in `WebFetch` and `WebSearch` tools handle all network
  I/O.
