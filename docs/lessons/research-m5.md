# Lessons Learned — research Milestone 5

## What changed
- Added `build_websearch_prompt(topic, questions, search_index)` to
  `crates/sldo-research/src/prompt.rs` plus three new public section
  constants: `SECTION_WEB_SEARCH_RESULTS`, `SECTION_DOCUMENTATION_FOUND`,
  `SECTION_LIBRARY_VERSIONS`.
- Added a `WEBSEARCH_QUESTIONS_PER_INVOCATION` partition constant so the
  research loop and the prompt builder agree on slice size without a
  parameter.
- Added a web-search phase to `research::research_loop` between exploration
  (iter 1) and deepening (iters 2..=max). Loop body invokes
  `build_websearch_prompt` once per `n in 1..=cfg.max_searches` and writes
  per-search log files named `.sldo-logs/research-websearch-<N>.log`.
- Added `extract_key_questions` helper in `research.rs` that pulls the body
  of the exploration's `## Key Questions` section so the web-search prompt
  can partition the question list across invocations.
- Added `max_searches: u32` field to `ResearchConfig` (already present from
  the M3 CLI surface — verified no rename was needed).
- Added 4 E2E tests in `tests/e2e_research_m5.rs`, registered under
  `[[test]]` in the workspace root `Cargo.toml`.
- ARCHITECTURE.md gained a `### Web search phase (M5)` subsection and the
  E2E test table now lists the M5 row.

## Design decisions and why
- **No new Rust HTTP client, no `reqwest`, no API integration.** The phase
  is driven entirely by Claude Code's built-in `WebFetch` / `WebSearch`
  tools, gated by `toolflags::research_allow_flags()` which already
  shipped them at M1. Adding a Rust-side HTTP client would have required
  a new dependency (violating the runbook's "no new deps" rule), parallel
  rate-limit logic, and a separate retry policy — none of which add value
  when Claude Code already orchestrates the searches.
- **Partition by question slice, not round-robin.** Index 1 covers the
  first `WEBSEARCH_QUESTIONS_PER_INVOCATION` questions, index 2 covers the
  next slice, etc. This is deterministic from the `(questions,
  search_index)` pair, so the unit tests don't need any randomness or
  ordering tolerance, and operators can re-run a single search index and
  reproduce its prompt exactly.
- **Empty-questions fallback.** When `extract_key_questions` returns an
  empty string (no `## Key Questions` header in the exploration output)
  or the slice for this index is empty, the prompt emits a "Research the
  topic broadly" instruction. This keeps the phase useful even when the
  exploration phase failed or returned malformed output.
- **No cooldown between web-search invocations.** Deepening invocations
  use `cfg.cooldown_secs` to smooth rate limits; web-search invocations
  do not. Originally I added a cooldown for symmetry, then removed it
  (commit fc0d3c2) because web-search invocations are lighter (small
  prompts, mostly delegated to Claude's built-in tools) and adding a 60s
  sleep × 5 searches added a minute of E2E test time for no benefit. The
  ARCHITECTURE.md note now calls this asymmetry out explicitly.
- **`max_searches == 0` is the explicit skip signal.** Not a `--no-web`
  flag. The integer matches the existing `--max-iterations` shape and
  composes cleanly: a CI run that wants to skip web for cost reasons
  passes `--max-searches 0 --max-iterations 1` to get a fast
  exploration-only dossier.
- **Per-search failures never halt the loop.** Same pattern as M3's
  exploration / deepening phases: a non-zero Claude exit logs a `warn(...)`
  and the loop moves on. Web is the most likely phase to fail
  (third-party sites can rate-limit Claude's fetcher), and the user still
  gets a dossier with whatever the other phases produced.
- **Web-search output is appended to `raw`, not split into a separate
  `ResearchFindings` field.** M6's synthesis pass reads `raw` end-to-end;
  splitting web findings out would force M6 to re-merge them. M5 keeps
  the existing `ResearchFindings { raw, repo_context }` shape — the
  repo-context split was a one-off because it lands in its own dossier
  section.

## Mistakes made / necessary handoffs
- **Initial attempt added a `cooldown_secs` sleep before each web-search
  invocation.** The runbook didn't ask for it; I added it for symmetry
  with deepening. Removed in commit fc0d3c2 once it became clear the
  pause inflated E2E test time without helping. Lesson: only mirror
  existing patterns when the underlying constraint applies — symmetry
  for its own sake is bloat.
- **Initial M5 logging said "skipping web phase" even when the phase ran
  with zero searches.** Cleaned up in fc0d3c2 — the logging now reflects
  what actually happened (zero invocations means no log lines about
  "skipping" because there is no decision point to log).
- **No M3-style E2E harness compromise needed.** The web-search phase is
  also driven by `claude -p`, so the existing PATH-shim pattern from
  M3/M4 covered M5 verbatim. The shim only needs to print one marker
  line; the loop handles all four invocations (exploration + 2 web +
  optional deepening) with the same shim.

## Root causes
- Adding a sleep without questioning whether the rate-limit it guards
  against actually applies to the new phase. Deepening uses the cooldown
  because deepening prompts are large and Claude's response is large;
  web-search prompts are small and most of the work happens inside
  Claude's tool layer (rate-limited separately). The lesson is to match
  patterns to the constraint they enforce, not to surface similarity.

## Naming conventions established
- **Web log filenames** use `research-websearch-<N>.log` (1-based,
  matching the `for n in 1..=cfg.max_searches` loop variable). Kept
  consistent with `research-deepen-<N>.log`'s 1-based naming so a
  directory listing reads as a chronological narrative.
- **`SECTION_WEB_SEARCH_RESULTS`, `SECTION_DOCUMENTATION_FOUND`,
  `SECTION_LIBRARY_VERSIONS`** are the public section header constants.
  M6 will `use` these from `prompt.rs` when grepping the raw findings to
  pull the URL+title pairs out for the dossier's `## References` section.
- **`WEBSEARCH_QUESTIONS_PER_INVOCATION`** is private to `prompt.rs`. The
  research loop intentionally does not pass a `slice_size` parameter —
  the partition is the prompt builder's concern, not the loop's.
- **`extract_key_questions`** lives in `research.rs` (next to its only
  caller) rather than `prompt.rs`. It's a parser of *exploration output*,
  not a constructor of *web-search prompts*.

## Test patterns that worked well
- **Unit tests assert on substring presence of the three required
  headers**, not on full prompt layout. Same pattern M3/M4 established —
  resilient to wording tweaks while still pinning the contract.
- **The "search_index varies prompt" test** uses `assert_ne!` on the
  output of two calls with different indexes. This is a property check
  that doesn't depend on the partition size — if M6 ever changes
  `WEBSEARCH_QUESTIONS_PER_INVOCATION`, this test still passes as long
  as different indexes produce different prompts.
- **The empty-questions test** is critical — the partition logic uses
  `saturating_sub` and `min` to avoid panicking on out-of-range slices.
  A unit test with `questions = ""` would have caught any
  off-by-one in the partition arithmetic.
- **E2E `test_max_searches_zero_skips_phase`** asserts on the *absence*
  of files matching `research-websearch-*.log`. Pairs with the
  `test_websearch_log_files_named_correctly` test that asserts on their
  *presence with exact names*. Together they pin the skip semantics and
  the naming contract.
- **Tool-flag regression guards** (`research_allow_flags_include_web_tools`
  and `plan_flags_do_not_include_web_search`) live in `research.rs::tests`
  rather than `sldo-common::toolflags::tests`. The test names encode the
  *consumer's* expectation (M5 needs both flags, plan must not have
  WebSearch), so future changes to `toolflags` that break M5 fail loudly
  in `sldo-research`'s test output.

## Missing tests that should exist now
- An E2E test that verifies a web-search failure (non-zero claude exit
  on the websearch invocation only) does not halt the loop and
  deepening still runs. Currently we cover the unit-level "warn-and-
  continue" path indirectly via the existing M3 patterns; M6 will likely
  add a fault-injection shim test once the synthesis pass needs to see
  partial findings.
- A test that pins the partition arithmetic for `search_index = 3` with
  exactly enough questions to fill the third slice. Edge case: the
  `start..end` with `end = .min(len)` should produce an exactly-full
  slice and the next index should fall through to the broad-research
  branch.

## Rules for the next milestone (M6)
- **The synthesis prompt MUST consume `findings.raw` directly.** Do not
  re-parse the per-section headers from `prompt.rs` to split web vs
  exploration vs deepening — the synthesis prompt instructs Claude to
  do that classification, and the headers in `raw` already mark the
  source phase.
- **Use the existing `M4_STUB_SENTINEL` constant from `dossier.rs`** to
  detect which sections need replacing. Do not introduce a parallel
  sentinel in `prompt.rs` — the dossier schema is the single source of
  truth for "what counts as a stub".
- **The synthesis pass is one Claude invocation, not a loop.** Do not
  add a `max_synthesis_iterations` flag. If the output is malformed,
  log a warning and emit the raw findings with a sentinel rather than
  retrying — the operator can re-run the whole research command.
- **Add `synthesised: Option<String>` to `ResearchFindings`.** That is
  the integration seam for M6 → dossier writer. Do not mutate `raw` in
  place — the existing E2E tests assert on the
  `Research accumulated N bytes of findings` info line, which counts
  `raw.len()`.
- **Do NOT run `cargo fmt --all` or `cargo clippy --workspace`.** Same
  M2/M4 rules apply: scope to `cargo fmt -p sldo-research` and
  `cargo clippy -p sldo-research --tests`. The Tauri/voice-tx crates
  have pre-existing lints outside research scope.
- **Web-search log files are now part of the directory contract.** When
  M6 writes its synthesis log, name it `research-synthesis.log` (no
  index suffix — there's only one synthesis pass) and do NOT touch any
  `research-websearch-*.log` files even to read them. The synthesis
  prompt receives findings via `raw`, not via log-file scraping.

## Template improvements suggested
- The M5 runbook says "Failures during web search never halt the loop"
  but doesn't specify whether *spawn* failures (claude not on PATH for
  this one invocation) should also be tolerated. Implementation chose
  yes — `run_phase` already wraps both the `LogFile::new` and the
  invocation in a single fallible scope and the loop's `match` arm
  treats spawn errors the same as non-zero exits. A future runbook
  template could include an explicit "failure modes" sub-table per phase.
- The runbook's `--max-searches` docs don't mention the partition slice
  size. Operators choosing a value need to know that 5 searches × 4
  questions/slice = 20 questions covered total. Adding a "tuning notes"
  section to the milestone would help.
