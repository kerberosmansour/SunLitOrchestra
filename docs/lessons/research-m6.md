# Lessons Learned — research Milestone 6

## What changed
- Added `build_synthesis_prompt(prompt, all_findings, repo_context)` to
  `crates/sldo-research/src/prompt.rs`. The new prompt embeds
  `dossier::REQUIRED_SECTIONS` verbatim, requests `(confidence: high|
  medium|low)` tags, asks for explicit URL extraction into
  `## References`, and truncates raw findings at 100 KiB
  (`SYNTHESIS_FINDINGS_MAX_BYTES`).
- Extended `ResearchFindings` with `synthesised: Option<String>` (purely
  additive — `raw` and `repo_context` unchanged).
- `research::research_loop` now runs one final synthesis Claude
  invocation after the deepening loop. Log file
  `.sldo-logs/research-synthesis.log`. On spawn error / non-zero exit /
  empty output / non-well-formed output, `synthesised` stays `None` and
  the dossier writer falls back to the M4 layout.
- Added private `synth_output_well_formed(out) -> bool` helper in
  `research.rs`: returns true only when every entry in `REQUIRED_SECTIONS`
  is present in the captured synthesis output. Three new unit tests pin
  the accept / reject / unstructured-blob behaviour.
- Extended `dossier::write_dossier` to accept `synthesised: Option<&str>`.
  When `Some(text)` and `text` is non-empty, the synthesised body is
  embedded verbatim in place of the M4 stub skeleton; otherwise the M4
  layout is used unchanged.
- Added public `dossier::check_synthesis_complete(path) -> Vec<String>`
  (the stricter post-M6 readiness check that detects the
  `M4_STUB_SENTINEL` — designed for M7's plan-readiness gate).
- `main.rs` passes `findings.synthesised.as_deref()` to `write_dossier`
  and emits two `info(...)` lines distinguishing the synth-success vs
  synth-fallback path. The existing `Research accumulated N bytes of
  findings` line is preserved (the M3 E2E asserts on it).
- `tests/e2e_research_m6.rs` ships 4 E2E tests; registered under
  `[[test]]` in the workspace root `Cargo.toml`.
- ARCHITECTURE.md gained a `### Synthesis pass (M6)` subsection (placed
  before the M5 web-search subsection so the document reads M3 → M4 →
  M6 → M5; left M5 alone to avoid touching prior milestone content); the
  E2E test table now lists the M6 row.

## Design decisions and why
- **Synthesis runs unconditionally once `raw` is non-empty.** No
  `--max-synthesis` flag, no opt-out. The runbook explicitly forbids a
  per-synthesis-iteration count — synthesis is "one Claude invocation,
  not a loop". Per the M5 lessons rule, I checked the CLI surface stays
  stable and the M6 E2E `test_help_flag_unchanged_after_m6` actively
  asserts `--max-synthesis` is *absent*.
- **`synth_output_well_formed` gates the synthesis embedding.** Without
  this gate, the M4 E2E `test_findings_appear_under_key_findings` would
  break: the M4 shim emits a single marker line, which would otherwise be
  stuffed into the dossier as the "synthesised body" (replacing the
  section structure entirely). Validating that all 9 required headers
  appear before treating the output as a dossier body keeps M4 tests
  green and adds defence-in-depth against truncated / off-spec real
  Claude responses.
- **`Option<String>` field on `ResearchFindings`, not a separate struct
  field on the dossier writer signature.** The synthesis output is
  conceptually part of the research-loop result, and threading it
  through `ResearchFindings` keeps the writer signature uniform: every
  caller passes the same five args. The M5 lessons file flagged this
  exact pattern as the right approach.
- **`check_synthesis_complete` is separate from `validate_dossier`.**
  Per the runbook step 6: "don't mutate `validate_dossier` itself (it
  still succeeds for an M4-style dossier)". Keeping the two checks
  independent means M7's plan-readiness gate can call
  `check_synthesis_complete` while operators running `sldo-research` at
  the M4 level (synthesis disabled) still see a clean validator pass.
- **Tail-preserving 100 KiB truncation of raw findings.** Mirrors the
  deepening prompt's strategy. Most recent additions are usually the most
  relevant — keeping the tail prefers web-search and deepening output
  over the initial exploration when truncation kicks in.
- **No new dependencies.** Same rule as M5: synthesis is prompt-driven.
  No JSON parsing of synthesis output, no markdown AST library — Rust
  just `contains()`-checks for the required headers.
- **Synthesis prompt explicitly forbids the `M4_STUB_SENTINEL` literal.**
  The synthesis-prompt body says "Do NOT emit the literal string 'To be
  synthesised in M6'" and a unit test pins both that the sentinel
  string is referenced (so prompt engineers can grep for it) and that
  the prompt explicitly forbids emitting it.
- **Empty raw findings short-circuit synthesis.** When `raw.trim()` is
  empty, the synthesis Claude call is skipped entirely (no log file
  created, no spawn). This is symmetric with the writer's empty-findings
  branch and avoids feeding Claude a meaningless prompt.

## Mistakes made / necessary handoffs
- **First attempt used `cat <<EOF` in the well-formed-shim test
  helper.** The test sets `PATH` to *only* the shim directory, so
  `cat` (an external utility) was not available and the shim exited
  with `cat: command not found` (exit 127). Fix: rewrite the shim using
  `printf` (a `/bin/sh` builtin) so the script needs nothing on PATH.
  Same lesson applies to the failing-synth shim — replaced
  `cat "$counter"` with marker-file enumeration via shell `[ -f ]` and
  `:>` builtins.
- **Initial raw string used `r#"..."#` with `"##` substrings inside.**
  Rust's raw-string close-delimiter for `r#"..."#` is `"#`, so a body
  containing `"##` (any `"` immediately followed by `#`) terminates the
  literal early. Fix: bumped to `r###"..."###`. Same trap caught me in
  the synthesis prompt where I quoted section names like
  `"## References"`.
- **Required adding `pub` to `REQUIRED_SECTIONS` import in `prompt.rs`.**
  The synthesis prompt builder needs the same constant the dossier
  writer uses; I imported it via `use crate::dossier::REQUIRED_SECTIONS`
  rather than duplicating the literal. The dependency direction
  (`prompt → dossier`) is one-way; `dossier` does not import from
  `prompt`, so there's no cycle.
- **The runbook listed `test_check_synthesis_complete_pure` as an E2E
  test.** I implemented it as a `dossier::tests` unit test instead (3
  unit tests cover the helper). The E2E file would need to either
  expose the binary's library (which is currently a binary-only crate)
  or invoke `check_synthesis_complete` through some other surface; both
  are larger architectural changes than the milestone justifies. The
  unit-test split preserves the contract without restructuring the
  crate.
- **Initial M4 E2E regression** — a shim that printed a single marker
  caused the dossier to be written as the marker (synth was treated as
  Some). Fixed via `synth_output_well_formed` gating; documented above.

## Root causes
- **Test sandbox PATH narrowing.** Setting `cmd.env("PATH", &shim)`
  replaces the entire PATH — children inherit nothing else. Any shim
  that needs an external utility (`cat`, `grep`, `awk`, `cp`, `mv`)
  will fail. Default to shell builtins (`printf`, `echo`, `[`, `:>`,
  `read`, parameter expansion) when writing test shims.
- **Rust raw-string delimiter collisions.** When a literal contains
  `"#`, `r#"..."#` is wrong — escalate to `r##"..."##` or higher. When
  in doubt, use the next level up: `r###"..."###` is rarely wrong.
- **Tight coupling between test shim output and production-side
  validation.** The first M4 regression revealed that "shim succeeds →
  raw populated → synth runs → synth output is the marker → marker
  becomes dossier body" is a fragile chain. The well-formedness gate
  decouples the chain: shim output that *looks* like a dossier body
  triggers the embed path; anything else falls back. Real Claude
  responses are always well-formed (the prompt pins the format), so this
  gate is invisible in production.

## Naming conventions established
- **`research-synthesis.log`** — no index suffix. There is exactly one
  synthesis pass per run, by design (per M5 lessons rule "synthesis pass
  is one Claude invocation, not a loop"). M6/M7 must not create a
  numbered series.
- **`synth_output_well_formed`** — private, lives in `research.rs`. The
  function is a *runtime quality gate on synthesis output*, not a
  prompt-construction concern. Keeping it next to `research_loop`
  (its only caller) avoids exposing implementation detail in the public
  module surface.
- **`SYNTHESIS_FINDINGS_MAX_BYTES`, `SYNTHESIS_TRUNCATION_MARKER`** —
  symmetric with `DEEPENING_FINDINGS_MAX_BYTES` /
  `DEEPENING_TRUNCATION_MARKER`. Both are private constants; future
  prompt builders should follow the same `<PHASE>_FINDINGS_MAX_BYTES`
  convention.
- **`check_synthesis_complete`** — verb-prefixed `check_` (not
  `validate_`) to mirror M7's planned `check_plan_readiness` and
  distinguish from the broader M4 `validate_dossier`. Returns
  `Vec<String>` for issues; never panics; no `Result`.
- **E2E shim helper names** — `shim_dir_with_well_formed_synth_claude`
  and `shim_dir_with_failing_synth_claude` are descriptive of the
  *behaviour* they simulate, not the implementation (printf vs marker
  files). Helps future maintainers scan the file for "what does this
  test simulate".

## Test patterns that worked well
- **Round-trip `write_dossier` → `check_synthesis_complete` tests.**
  `check_synthesis_complete_returns_empty_for_clean_synth_dossier`
  uses the writer to produce the fixture, then runs the checker — same
  coupling pattern as M4's
  `validate_dossier_tolerates_m4_stub_sentinel` test. If a future
  change weakens the sentinel detection, both writer and checker would
  need to update in lockstep, and the test forces that coupling
  visible.
- **`assert!(!content.contains("To be synthesised in M6"))`** as the
  primary post-synthesis assertion, rather than asserting on the synth
  marker presence alone. The negative assertion catches any regression
  that re-introduces the M4 layout when synthesis succeeds.
- **`test_help_flag_unchanged_after_m6` actively asserts a missing
  flag** (`--max-synthesis`), not just the presence of pre-existing
  flags. This pins the M5 lessons rule "do not add a per-synthesis
  iteration count" into a regression guard.
- **Counter-based shim using marker files** instead of a running
  counter file. Simpler shell logic, no race conditions if the shim is
  invoked in parallel (each invocation finds its own free `call-N`
  slot). Future fault-injection shims should follow this pattern.
- **Three structural-validity unit tests for `synth_output_well_formed`**
  (full headers, missing one header, unstructured blob) cover the
  three branches of the predicate. Cheaper than running the full
  pipeline through E2E to exercise the gate.

## Missing tests that should exist now
- A test that exercises synthesis with a `repo_context` populated (the
  M3 repo-context phase running) and asserts the repo-context section
  appears *before* the synth body in the dossier. The synth-with-
  repo-ctx unit test in `dossier.rs` covers the writer side; an E2E
  with `--repo-dir` would cover the integration.
- A test that runs synthesis with an oversized raw findings (~150 KiB)
  and asserts the truncation marker appears in the synthesis prompt. The
  unit test on `build_synthesis_prompt` covers the prompt builder;
  there's no integration test that proves the truncation actually fires
  in the loop. Probably overkill — the unit test is enough.
- An E2E test for the `cooldown_secs` interaction with synthesis: does
  the loop sleep before the synthesis call? Currently no — the
  cooldown only applies between deepening iterations. Worth pinning if
  M7 changes the cooldown semantics.

## Rules for the next milestone (M7)
- **Use `dossier::check_synthesis_complete` from the plan-readiness
  gate.** It already exists, returns `Vec<String>` issues, and pins
  the `M4_STUB_SENTINEL` detection. Do not duplicate the check inside
  `sldo-plan` — re-export the function or import the dossier module.
- **Do NOT touch `validate_dossier`.** The runbook explicitly says it
  must continue to succeed on M4-style dossiers (those with the
  sentinel). M7's `check_plan_readiness` is the strict layer; that is
  where any sentinel-detection logic for plan-readiness must live.
- **The `synth_output_well_formed` gate is the contract for "synth
  succeeded".** If M7 needs a programmatic "did synthesis run
  successfully?" signal, plumb the `Option<String>` through, do *not*
  re-validate the dossier file with a different ruleset. The gate
  already encodes the structural invariant.
- **Do NOT add a `--no-synthesis` or `--max-synthesis` flag.** Two
  milestones in a row (M5 lessons + M6 implementation) have committed
  to "synthesis is unconditional". Reversing that breaks
  `test_help_flag_unchanged_after_m6`.
- **`research-synthesis.log` is a single-file contract.** M7 must not
  introduce numbered synthesis logs (`research-synthesis-1.log`).
  Operators expect at most one per run.
- **Test shims must use shell builtins.** PATH is replaced (not
  augmented) in E2E tests. `printf`, `echo`, `[`, `:>`, `read`, and
  parameter expansion are safe; `cat`, `grep`, `awk`, `cp`, `mv` are
  not. Future shims should follow the M6 patterns.
- **Do NOT run `cargo fmt --all` or `cargo clippy --workspace`.** Same
  M2/M4/M5 rule — scope to `cargo fmt -p sldo-research` and
  `cargo clippy -p sldo-research --tests`. Pre-existing lints in
  `sldo-tauri` / `voice-tx` are out of scope.

## Template improvements suggested
- The M6 runbook listed `test_check_synthesis_complete_pure` as an E2E
  test, but a pure-Rust function call is naturally a unit test (the
  binary crate has no library surface to import from `tests/`). A
  future runbook template could distinguish "E2E (subprocess) tests"
  from "unit tests (in-tree)" rather than lumping both under one
  heading.
- The runbook's BDD scenario "Failure → raw fallback" implies the
  synthesis Claude call fails. With the well-formedness gate added in
  M6, that scenario is also reachable when synth succeeds but produces
  off-spec output. A future runbook section on "synthesis output
  validation" would call out both failure modes.
- The runbook's smoke test says `grep -c "To be synthesised" → 0` when
  claude is installed. With my well-formedness gate, this depends on
  the real Claude response containing every required section header —
  which it should, given the prompt explicitly demands it. But the
  smoke test is a good operator-level check that the prompt is doing
  what it claims.
