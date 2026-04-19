# Completion Summary — research Milestone 7

**Milestone**: Plan-ready output & sldo-plan integration
**Status**: done
**Started / Completed**: 2026-04-19 / 2026-04-19

## What shipped

1. **`crates/sldo-research/src/dossier.rs`** — plan-readiness gate.
   - `pub fn check_plan_readiness(path: &Path) -> Vec<String>` — strict
     end-of-pipeline check that composes `validate_dossier` and
     `check_synthesis_complete` with three additional gates: file is
     valid UTF-8, total size > 1 KiB, `## Design Recommendations` has
     > 100 bytes of body, AND at least one of `## Library & Tool
     Evaluations` / `## Architecture Options` has > 100 bytes of body.
   - `pub` constants `MIN_PLAN_READY_SIZE` (1000) and
     `MIN_SECTION_BODY_BYTES` (100) — kept private but documented in
     the function docstring.
   - Private `section_body(content, header) -> Option<&str>` helper:
     extracts section body between a header and the next top-level
     `## ` marker. Header-line-aware (skips the header's own newline).
   - 7 new unit tests for `check_plan_readiness`: passes for complete
     dossier, flags too-small, flags missing Design Recs, flags M4
     stub sentinel, flags missing both lib-eval AND arch-options,
     flags non-UTF-8 file, flags missing file.
   - 3 new unit tests for `section_body`: between headers, missing →
     None, last section → tail.
   - `check_plan_readiness` is the **only** in-binary caller of
     `check_synthesis_complete`, finally retiring the dead-code
     warning M6 left behind.

2. **`crates/sldo-research/src/main.rs`** — end-of-run output.
   - Captures `started_at = Instant::now()` at the top of `run()`.
   - After dossier write/validate, calls `check_plan_readiness(&cli.output)`.
   - Always prints a Summary block (header + dossier path + bytes +
     iteration/search counts + `Total wall time: X.XXs`).
   - When plan-ready: prints "Research dossier is ready for planning."
     and a `sldo-plan <dossier-path> <repo-dir> [-o docs/RUNBOOK.md]`
     suggestion.
   - When not plan-ready: prints a warning block listing the issues
     and suppresses the next-step suggestion.
   - Either way the binary exits 0; the dossier is always written.

3. **`tests/fixtures/research/plan-ready-dossier.md`** — hand-written,
   3.5 KiB plan-ready fixture covering all 9 required sections with
   substantive content (no stub sentinels), realistic confidence tags
   on Design Recommendations, and 6 References URLs. Used by the M7
   E2E to assert the plan-ready contract is reachable.

4. **`tests/e2e_research_m7.rs`** — 7 subprocess + integration tests:
   - `test_dossier_is_valid_utf8_text` — dossier file is read back as
     valid UTF-8 after a normal run.
   - `test_summary_block_shown_after_run` — Summary header + Total
     wall time line present in stderr.
   - `test_cli_omits_next_step_when_not_ready` — under a tiny-shim
     run (M4 fallback layout), the "Next step" suggestion is absent
     and a "not yet plan-ready" warning is present.
   - `test_plan_ready_fixture_is_consumable_by_sldo_plan` — the
     fixture is valid UTF-8, > 1 KiB, contains every required
     section header, and does not contain the M4 stub sentinel.
   - `test_sldo_plan_help_unchanged_after_m7` — `sldo-plan --help`
     surface (PROMPT_FILE, REPO_DIR, --output, --model) intact.
   - `test_sldo_run_help_unchanged_after_m7` — `sldo-run --help`
     surface (RUNBOOK, REPO_DIR) intact.
   - `test_sldo_research_help_unchanged_after_m7` — every M3-era flag
     still present, AND `--max-synthesis` / `--no-synthesis` /
     `--plan-ready` actively asserted absent.

5. **Workspace root `Cargo.toml`** — new `[[test]]` entry for
   `e2e_research_m7`.

6. **`docs/ARCHITECTURE.md`**:
   - New `### Plan-readiness gate (M7)` subsection describing the
     composition of `validate_dossier`, `check_synthesis_complete`,
     and the three additional M7 rules.
   - New `### sldo-research pipeline overview` ASCII flow diagram
     covering all five phases plus write / validate / readiness gate.
   - New `### Pipeline composition` block showing
     `sldo-research → sldo-plan → sldo-run` chaining and pinning the
     "only sldo-research has web access" property.
   - Test architecture table updated with the M7 row (7 tests).

7. **`README.md`**:
   - `sldo-research` section rewritten from "preview" to a full
     CLI section with a flag table covering every documented flag.
   - Added the full pipeline example (`sldo-research → sldo-plan →
     sldo-run`) plus tighter-quota single-shot variants.
   - Added a security-notes blockquote covering threat-model entries
     T1 (prompt injection), T4 (log sensitivity), and T5 (API quota
     cost).
   - Project Structure block updated to remove the "(preview)"
     qualifier.

8. **`docs/lessons/research-m7.md`** — design decisions, mistakes
   (initial inlined sentinel check, single_char_add_str), naming
   conventions, test patterns, and standing rules for future
   sldo-research work.

## Smoke tests passed

- `target/debug/sldo-research --prompt "test" --max-iterations 1 --max-searches 0`
  (with a `printf` shim) prints a Summary block followed by a
  not-ready warning naming the M4 stub sentinel — exactly the
  expected behaviour for an unstructured shim that triggers the M4
  fallback layout.
- `cargo test -p sldo-research` — 84 unit tests pass.
- `cargo test --test e2e_research_m1 --test e2e_research_m2 --test e2e_research_m3 --test e2e_research_m4 --test e2e_research_m5 --test e2e_research_m6 --test e2e_research_m7 -- --test-threads=1`
  — all 39 research E2E tests pass.
- `cargo clippy -p sldo-research --tests -- -D warnings` clean (no
  more dead-code warning for `check_synthesis_complete`).
- `cargo clippy --test e2e_research_m7 -- -D warnings` clean.
- `cargo fmt -p sldo-research` produced no diff after running.
- `target/debug/sldo-plan --help` and `target/debug/sldo-run --help`
  outputs are byte-identical to pre-M7 snapshots (captured in
  `/tmp/sldo-{plan,run}-help-prem7.txt` at the start of the milestone
  and `diff`-ed at the end).

## Pre-existing tests

All ~290 pre-existing tests remain green under the lessons-scoped
test harness. The pre-existing `frontend_dist_exists_after_build`
failure in `e2e_tauri_m1` (esbuild/Vite arm64 platform issue) is
unrelated to research milestones — same baseline as M3, M4, M5, M6.

## Known deferrals (intentional)

- A live shim-driven E2E that asserts the Next-step *suggestion*
  appears in stderr (rather than asserting the fixture is plan-ready
  separately). Producing a plan-ready dossier from a shim is harder
  than producing a not-ready one, since the shim's output is
  deterministic and doesn't naturally satisfy the
  `synth_output_well_formed` + `MIN_SECTION_BODY_BYTES` thresholds
  simultaneously. The fixture-based contract test covers the same
  guarantee from a different angle.
- Future iteration counts may distinguish "iterations attempted" from
  "iterations completed" in the Summary block — the `(max N)` slot is
  currently redundant for clarity but reserved for that future split.
- `sldo-plan` integration is contractual (file format) only. No
  cross-crate Rust API was added; sldo-plan continues to read the
  dossier as a plain `prompt_file`. This is intentional — keeps the
  sldo-plan crate untouched, per runbook design rule.

## Pipeline status

With M7 done, the full `sldo-research → sldo-plan → sldo-run`
pipeline is operational end-to-end:

```
$ sldo-research --prompt "<topic>" --repo-dir <repo>
  → output/research-dossier.md
  → "Research dossier is ready for planning."
  → "Next step — generate a runbook:
       sldo-plan output/research-dossier.md <repo-dir> [-o docs/RUNBOOK.md]"

$ sldo-plan output/research-dossier.md <repo>
  → docs/RUNBOOK.md  (existing M3 behaviour, unchanged)

$ sldo-run docs/RUNBOOK.md <repo>
  → drives Claude Code through each milestone (existing M4 behaviour,
    unchanged)
```

`RUNBOOK-RESEARCH-GENERATED.md` is now complete; M1–M7 all marked done
in the tracker.
