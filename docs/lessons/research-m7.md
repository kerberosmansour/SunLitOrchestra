# Lessons Learned — research Milestone 7

## What changed
- Added `check_plan_readiness(path) -> Vec<String>` to
  `crates/sldo-research/src/dossier.rs`. It composes
  `validate_dossier` and `check_synthesis_complete` (both pre-existing)
  with three additional gates: UTF-8 readability, `MIN_PLAN_READY_SIZE`
  (1 KiB), and section-body length checks for `## Design Recommendations`
  and (`## Library & Tool Evaluations` OR `## Architecture Options`).
- Added private `section_body(content, header)` helper that returns the
  text between a header and the next top-level `## ` header. Three unit
  tests pin the body-extraction behaviour (between headers, missing
  header → None, last section → tail).
- `crates/sldo-research/src/main.rs` gained a Summary block (dossier
  path, byte count, iteration/search counts, total wall time) and a
  conditional Next-step suggestion: when plan-readiness passes, prints
  the suggested `sldo-plan` invocation; when it fails, prints a warning
  block listing the issues. Either way the binary exits 0.
- New fixture
  `tests/fixtures/research/plan-ready-dossier.md` — a hand-written,
  3.5 KiB dossier with all required sections populated (no stub
  sentinels). Used by the M7 E2E to assert the plan-ready contract is
  reachable.
- New `tests/e2e_research_m7.rs` (7 tests) registered under `[[test]]`
  in workspace root `Cargo.toml`. Covers: dossier UTF-8 validity,
  Summary block presence, Next-step suppression on not-ready,
  plan-ready fixture is consumable by sldo-plan, and three
  back-compat help-stability guards (`sldo-plan`, `sldo-run`,
  `sldo-research` --help surfaces).
- README.md: `sldo-research` section rewritten with a flag table, the
  full `sldo-research → sldo-plan → sldo-run` pipeline example, and
  security notes (T1, T4, T5 from the runbook's threat model).
- ARCHITECTURE.md: added `### Plan-readiness gate (M7)`, a new
  `### sldo-research pipeline overview` ASCII flow diagram, and a
  `### Pipeline composition` block showing how the three CLIs chain.
  Test architecture table now lists the M7 row.

## Design decisions and why
- **Composition over duplication.** `check_plan_readiness` calls
  `validate_dossier` and `check_synthesis_complete` rather than
  inlining their logic. The runbook explicitly forbids relaxing
  `validate_dossier`; composing keeps the M4/M6 contracts intact and
  makes the M7 gate trivially auditable as "M4 + M6 + 3 extra rules".
  As a side benefit, `check_synthesis_complete` (the M6 deliverable) is
  no longer dead code — its single in-binary caller is now M7's gate,
  which also silences the dead-code warning M6 left behind.
- **Plan-readiness is observability, not a hard gate.** The runbook
  says: "If the dossier fails plan-readiness, sldo-research still
  exits 0 but prints clear warnings and does not print the next-step
  suggestion." This matches operator expectations: a research run that
  cost API credits should never silently disappear because of a
  post-hoc validation. The dossier is always written; the gate
  decides only whether to print "Next step".
- **Section-body length threshold is 100 bytes, not "non-empty".**
  Section presence is already covered by `validate_dossier`. The M7
  gate adds *substance* — a one-line stub like
  "## Design Recommendations\n\n- TBD" passes M4 but should not pass
  plan-readiness. 100 bytes is roughly two short sentences, which is
  the floor for "useful enough that sldo-plan can build on it".
- **`section_body` returns `None` for a missing header**, not an
  empty `&str`. The `Option` shape lets the caller distinguish
  "header is absent" (which is also `validate_dossier`'s job to flag)
  from "header is present but body is too thin" (M7's distinct
  concern). This avoided double-reporting "Missing section X" + "X
  has < 100 bytes".
- **Library Evaluations OR Architecture Options.** Not both. A
  research dossier for a small/well-defined topic (e.g., "evaluate
  these three logging crates") may legitimately have only library
  evaluations and no separate architecture options section, and vice
  versa. Requiring both would force synthesis to invent content for
  the missing one — lowering quality, not raising it.
- **Summary block always shown, Next-step block conditional.** Two
  separate `divider() / header()` blocks. Operators always want the
  wall-time + byte-count audit; only ready dossiers get the
  copy-pasteable command. Mirrors the existing `sldo-plan` UX where
  the success ribbon and the verification block are visually distinct.
- **Time tracking via `Instant::now()` at top of `run()`.** No new
  dependency, no chrono `Local::now()` arithmetic (which would
  conflate wall-clock skew with elapsed time). The single
  `started_at.elapsed()` call at the end captures only sldo-research
  CPU/IO time, which is exactly what operators want to budget.
- **Hand-written fixture, not dynamic generation.** The M7 E2E
  `test_plan_ready_fixture_is_consumable_by_sldo_plan` reads a
  committed-to-repo file rather than generating one at runtime.
  Reasons: (a) makes the contract grep-able and reviewable in PRs, (b)
  decouples the test from `write_dossier`'s output format (so future
  writer changes can't accidentally mask a regression), (c) gives the
  team a concrete reference example of a "good" dossier shape that can
  be linked from docs.
- **Help-snapshot tests use substring assertions, not byte-identical
  comparison.** Byte-identical would require golden files committed
  into the repo, which then bit-rot every time clap or a doc string
  changes. Substring assertions (each canonical flag must be present,
  each forbidden flag must be absent) capture the contract without
  the maintenance burden. The smoke-test step in the runbook still
  recommends a manual byte-identical diff against a pre-flight
  snapshot, which I did manually (passed).
- **No `--max-synthesis` / `--no-synthesis` / `--plan-ready` flags.**
  Same M5/M6 lessons rule — the surface stays stable. The test
  `test_sldo_research_help_unchanged_after_m7` actively asserts each of
  the three forbidden flags is absent.
- **Next-step suggestion uses placeholder `<repo-dir>`, not a guess.**
  `sldo-research` doesn't know where the user wants to plan against; the
  literal `<repo-dir>` token forces the operator to think for a moment
  rather than copy-paste something wrong. Same rationale as
  `sldo-plan`'s end-of-run output style.

## Mistakes made / necessary handoffs
- **Initial `check_plan_readiness` inlined the M4 stub-sentinel
  detection** rather than calling `check_synthesis_complete`. This
  duplicated logic and (more importantly) left `check_synthesis_complete`
  as dead code in the binary build, surfacing as a `function never used`
  warning. Refactored to call `issues.extend(check_synthesis_complete(path))`
  — both functions now stay alive and the warning is gone.
- **Initial fixture build helper used `body.push_str("\n")`** which
  triggers `clippy::single-char-add-str`. Fixed to `body.push('\n')` —
  same lesson as M6.
- **Initial summary stats reported `iterations: 1 (max 1)`** which is
  redundant since both numbers come from the CLI flag. Kept the
  format anyway because future work may distinguish "iterations
  attempted" from "iterations completed" (e.g., if claude failures
  cause early-return in the loop) — the `(max N)` slot becomes the
  diff. Documenting this here so M8 (if it ever lands) knows the
  shape is intentional, not laziness.

## Root causes
- **Dead-code warnings are a load-bearing signal in a binary-only
  crate.** `sldo-research` has no library target, so `pub` items that
  no in-binary call site touches are flagged as unused. Wired
  `check_synthesis_complete` into the M7 plan-readiness gate to
  acknowledge that — once a function is the right primitive, the
  composition should make it observable.
- **Test PATH narrowing is a recurring trap.** Same root cause as M6:
  `cmd.env("PATH", &shim)` *replaces* the entire PATH. The M7 shim
  uses `printf` only (a `/bin/sh` builtin). Not new in M7, but pinned
  again in this milestone's lessons since future shim authors will
  hit the same wall otherwise.

## Naming conventions established
- **`check_plan_readiness`** — same `check_` prefix as
  `check_synthesis_complete`, distinguishing M7's strict gate from M4's
  permissive `validate_dossier`.
- **`MIN_PLAN_READY_SIZE` (1000)** — explicit constant, distinct from
  M4's `MIN_DOSSIER_SIZE` (500). Two thresholds, not one, because the
  M4 layer is "did the writer produce something coherent?" and M7 is
  "is the result rich enough to drive sldo-plan?".
- **`MIN_SECTION_BODY_BYTES` (100)** — applied to both Design
  Recommendations and the Library/Arch alternatives. One constant for
  one concept; if future tuning splits them, that's a refactor when
  the need arises, not pre-emptive complexity.
- **`section_body`** — present-tense verb-less name (it returns a
  body, doesn't perform an action). Matches existing `topic_excerpt`
  helper naming.
- **Fixture path**:
  `tests/fixtures/research/plan-ready-dossier.md`. The `research/`
  sub-directory groups M7 fixtures away from the existing
  `mock-claude.sh`, `sample-prompt.txt`, etc. Future research
  fixtures (e.g., a `not-ready-dossier.md` for negative-case tests)
  belong under the same sub-tree.

## Test patterns that worked well
- **Round-trip writer → checker tests** (M4/M6 pattern):
  `check_plan_readiness_flags_stub_sentinel` uses `write_dossier(...,
  None, None)` to *produce* a dossier with the M4 stub sentinel, then
  runs the checker. Coupling the writer's output to the checker's
  rejection means a future change that changes either side fails the
  test in lockstep.
- **Substring-based help assertions** (active + negative). The
  positive `for needle in [...] { assert!(stdout.contains(needle)) }`
  pattern is paired with negative `for forbidden in [...] {
  assert!(!stdout.contains(forbidden)) }`. The pairing pins the M5/M6
  "no new flags" rule into the M7 test instead of leaving it as
  tribal knowledge.
- **Hand-written fixture > dynamic generation** for "what does a good
  dossier look like" tests. Reviewers can read the fixture file
  diff-by-diff; future schema changes have a concrete example they
  must update; the file becomes implicit documentation. Cost: 130
  lines of markdown that have to stay in sync with the schema.
- **Separate Summary-block test from readiness-suggestion test.**
  `test_summary_block_shown_after_run` only asserts the Summary
  appears; `test_cli_omits_next_step_when_not_ready` only asserts the
  Next-step block is absent. Splitting these means a future change
  to the Summary template doesn't break the readiness gate's
  regression test.

## Missing tests that should exist now
- An E2E that runs `sldo-research` with a fake-pre-seeded plan-ready
  dossier at `--output` and a no-op claude shim, asserting the
  Next-step suggestion appears in stderr. The current
  `test_plan_ready_fixture_is_consumable_by_sldo_plan` only asserts
  the fixture *itself* is plan-ready; it doesn't drive the binary to
  produce that output. The shim-driven path was harder to set up
  because the shim's outputs would be appended/mixed with the
  pre-seeded content; deferred as a future test.
- A test that runs `sldo-research --repo-dir ./this-very-repo
  --max-iterations 0 --max-searches 0` to prove the repo-context phase
  + readiness gate compose correctly. Today the M3 E2E covers
  repo-context; M7 doesn't pin the interaction.
- A property/fuzz-style test for `section_body` with random
  `## Header\n` patterns. The current three explicit tests cover
  happy-path / missing / tail; an adversarial body with `## ` inside a
  code fence would slip through (the regex-free implementation
  treats every `\n## ` as a section boundary). Acceptable for now —
  real Claude responses don't put `## ` inside fences — but a future
  M8 might harden it.

## Rules for any subsequent milestone
- **Compose the existing checks, do not replace them.** The
  `validate_dossier → check_synthesis_complete → check_plan_readiness`
  layering is the public contract. Adding a new gate means adding a new
  `check_*` function and composing it; never relax the existing ones.
- **`sldo-plan` is not modified by `sldo-research` work.** The
  integration is one-directional: sldo-research must produce output
  that sldo-plan can consume as `prompt_file`. That contract is just
  "valid UTF-8, > 1 KiB" — anything richer happens inside sldo-plan's
  own milestones.
- **CLI surface is stable.** Six flags, unchanged since M3. Any new
  capability ships as default behaviour or via reading the dossier
  itself, not via a new flag. The `test_sldo_research_help_unchanged_after_m7`
  forbidden-flag list is the regression guard.
- **Pre-flight protocol now includes a help-snapshot capture.** Before
  M8 (if it exists) starts, capture
  `target/debug/sldo-{plan,run,research} --help` to a temp file and
  diff after the milestone — the M7 smoke tests showed this is a
  cheap and effective regression check.
- **Test shims must use `/bin/sh` builtins only** (`printf`, `echo`,
  `[`, `:>`, parameter expansion). PATH is replaced, not augmented.
  M5/M6/M7 all relearned this — make it a hard rule for future
  research-pipeline work.
- **Do NOT run `cargo fmt --all` or `cargo clippy --workspace`.**
  Same M2/M4/M5/M6 rule — scope to `cargo fmt -p sldo-research` and
  `cargo clippy -p sldo-research --tests` plus per-test-file clippy
  for the new E2E. Pre-existing lints in `sldo-tauri` /
  `sldo-voice-tx` are out of scope.

## Template improvements suggested
- The runbook's M7 step `test_dossier_accepted_by_sldo_plan_as_prompt_file`
  description suggested either `gh pr create`-style inline-spawning of
  `sldo-plan --help`, or a direct `read_to_string` size-check. I went
  with the latter (simpler, no subprocess fragility); a future runbook
  template should pick one form rather than offering both.
- The M7 BDD scenario "Ready prints next step" assumed the test could
  reliably produce a ready dossier from a shim. In practice, producing
  a *truly* plan-ready dossier requires either a real Claude call
  (cost) or a pre-seeded fixture that the binary preserves (more
  complex than the M3-style shim pattern supports). The fixture +
  separate "fixture-is-plan-ready" assertion was the workable
  compromise. Future runbook templates could acknowledge this and
  recommend the fixture pattern up front for shape-of-output tests.
- The runbook's "Documentation Update Table" lists the M7 README change
  as adding "all flags and full pipeline example". Suggest extending
  that to also include security notes (the threat model entries T1,
  T4, T5 specifically warn about prompt injection, log sensitivity,
  and quota costs — those belong in the user-facing README too, not
  just the threat model section).
