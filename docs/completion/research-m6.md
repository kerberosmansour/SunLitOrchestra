# Completion Summary — research Milestone 6

**Milestone**: Multi-source synthesis pass
**Status**: done
**Started / Completed**: 2026-04-19 / 2026-04-19

## What shipped

1. **`crates/sldo-research/src/prompt.rs`** — synthesis prompt builder.
   - `build_synthesis_prompt(prompt: &str, all_findings: &str, repo_context: Option<&str>) -> String`
     — embeds `dossier::REQUIRED_SECTIONS` verbatim, asks for
     `(confidence: high|medium|low)` tags on recommendations, requires
     URL extraction into `## References` as `- [Title](URL)` bullets,
     truncates raw findings at 100 KiB (tail-preserving).
   - Private constants `SYNTHESIS_FINDINGS_MAX_BYTES` (100 KiB) and
     `SYNTHESIS_TRUNCATION_MARKER`.
   - 8 inline BDD unit tests covering header inclusion, confidence
     levels, dedup instruction, raw-findings embedding, oversized-
     truncation, repo-context inclusion/omission, and the explicit
     "do not emit M4 stub sentinel" rule.
   - One additional `builders_are_pure_no_panic_on_empty_inputs` case
     extended to cover the synthesis builder.

2. **`crates/sldo-research/src/research.rs`** — synthesis phase added.
   - `ResearchFindings` gained `synthesised: Option<String>` (purely
     additive; existing `raw` and `repo_context` unchanged).
   - `research_loop` runs one final synthesis Claude invocation after
     deepening. Log file `.sldo-logs/research-synthesis.log`. Spawn
     errors / non-zero exit / empty output / non-well-formed output all
     resolve `synthesised` to `None`. Empty `raw` short-circuits the
     phase entirely.
   - Private helper `synth_output_well_formed(out) -> bool` —
     structurally validates the synth response by requiring every entry
     in `REQUIRED_SECTIONS` to appear. Three new unit tests cover full
     headers / missing one header / unstructured-blob branches.
   - Two new `ResearchFindings` construction unit tests pin the public
     `synthesised` field.

3. **`crates/sldo-research/src/dossier.rs`** — synthesis-aware writer.
   - `write_dossier(path, prompt, findings, repo_context, synthesised)` —
     5th `synthesised: Option<&str>` parameter added. `Some(text)` with
     non-empty `text` embeds the synthesised body verbatim in place of
     the M4 stub skeleton; otherwise the M4 layout is unchanged.
   - New public `check_synthesis_complete(path: &Path) -> Vec<String>` —
     stricter post-M6 readiness check that returns issues if the
     dossier still contains the `M4_STUB_SENTINEL`. Designed for M7's
     plan-readiness gate. Intentionally separate from `validate_dossier`
     (which still tolerates the sentinel for M4 compatibility).
   - 6 new inline unit tests: synthesised body replaces stubs,
     empty synthesised falls back to M4 layout, repo-context preserved
     when synthesised is Some, sentinel detected,
     check returns empty for clean dossier, missing file flagged.
   - All 9 prior `write_dossier(...)` call sites updated to the
     5-arg signature (`, None` added uniformly — they exercise the M4
     fallback).

4. **`crates/sldo-research/src/main.rs`** — wiring.
   - Two new `info(...)` lines distinguish synth-success vs synth-
     fallback paths. The existing `Research accumulated N bytes of
     findings` line is preserved verbatim (the M3 E2E asserts on it).
   - `findings.synthesised.as_deref()` passed as the 5th `write_dossier`
     argument.

5. **`tests/e2e_research_m6.rs`** — 4 subprocess tests:
   - `test_synthesis_log_created_when_claude_ok` — synthesis log
     exists after a successful run.
   - `test_synthesis_fallback_still_writes_dossier` — pipeline exits 0
     and dossier is written even when synthesis output is rejected by
     the well-formedness gate; raw findings appear in the dossier and
     the M4 stub sentinel is retained.
   - `test_successful_synthesis_replaces_stub_sentinel` — when the
     shim emits a fully-formed dossier body (every required section
     header present), the synth body is embedded and the stub
     sentinel is absent.
   - `test_help_flag_unchanged_after_m6` — pins the M1–M5 CLI surface
     and *actively asserts* `--max-synthesis` is absent (per M5
     lessons rule).

6. **Workspace root `Cargo.toml`** — new `[[test]]` entry for
   `e2e_research_m6`.

7. **`docs/ARCHITECTURE.md`** — new "### Synthesis pass (M6)"
   subsection describing the prompt-driven flow, the well-formedness
   gate, the writer signature change, and the
   `check_synthesis_complete` helper. Test architecture table updated
   with the new E2E file (4 tests).

8. **`docs/lessons/research-m6.md`** — design decisions, mistakes
   (raw-string delimiter, shim PATH narrowing, M4 regression and the
   well-formedness gate fix), naming conventions, test patterns, and
   rules for M7.

## Smoke tests passed

- `target/debug/sldo-research --prompt "test topic" --max-iterations 1 --max-searches 0`
  produces `.sldo-logs/research-synthesis.log` and writes a dossier
  (verified with a `printf` shim under `/tmp/sldo_m6_smoke/shim`). With
  the test shim's unstructured output the well-formedness gate kicks in
  and the M4 fallback is used (8 stub sentinels in the dossier — exactly
  one per non-Key-Findings required section). Real Claude responses
  satisfy the gate by construction, so operators see 0 sentinels.
- `cargo test -p sldo-research` — 74 unit tests pass.
- `cargo test --test e2e_research_m1 --test e2e_research_m2 --test e2e_research_m3 --test e2e_research_m4 --test e2e_research_m5 --test e2e_research_m6 -- --test-threads=1`
  — all 32 research E2E tests pass.
- `cargo clippy -p sldo-research --tests -- -D warnings` clean.
- `cargo clippy --test e2e_research_m6 -- -D warnings` clean.
- `cargo fmt -p sldo-research` produced no diff after running.

## Pre-existing tests

All ~290 pre-existing tests remain green under the lessons-scoped test
harness. The pre-existing `frontend_dist_exists_after_build` failure in
`e2e_tauri_m1` (esbuild/Vite arm64 platform issue) is unrelated to
research milestones — same baseline as M3, M4, and M5.

## Known deferrals (intentional)

- M7's `check_plan_readiness` will compose `check_synthesis_complete`
  with additional gates (file size, encoding, etc.) — the M6 helper
  is the building block, not the final gate.
- No `--max-synthesis` flag — the runbook + M5 lessons + M6 lessons
  all commit to single-pass synthesis.
- No new dependencies — the phase is prompt-driven; Claude Code does
  the synthesis, the Rust pipeline structurally validates the result.
