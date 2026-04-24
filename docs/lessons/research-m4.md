# Lessons Learned — research Milestone 4

## What changed
- Added `crates/sldo-research/src/dossier.rs` with `REQUIRED_SECTIONS`,
  `PLACEHOLDER_PATTERNS`, `M4_STUB_SENTINEL`, `write_dossier`, and
  `validate_dossier`. 13 inline BDD unit tests cover the writer, validator,
  and `topic_excerpt` helper.
- Changed `research::research_loop` return type from `Result<String>` to
  `Result<ResearchFindings>` where `ResearchFindings { raw, repo_context }`
  splits the optional repo-context capture from the raw findings.
- Wired dossier write + validation into `main.rs::run()` after the research
  loop, behind a new `Dossier` header. The existing
  `Research accumulated N bytes of findings` info line is preserved (the M3
  E2E test asserts on it); we now read `findings.raw.len()` for the count.
- Added `tests/e2e_research_m4.rs` (6 tests) and registered it under
  `[[test]]` in the workspace root `Cargo.toml`.

## Design decisions and why
- **`M4_STUB_SENTINEL` is a first-class public constant**, not a literal
  scattered across the writer. M6 will grep for this exact string to detect
  stubs to replace, and M7's `check_plan_readiness` will flag its presence
  as a readiness failure. Making it a named constant keeps those future
  checks trivially greppable.
- **`validate_dossier` tolerates `M4_STUB_SENTINEL`.** The runbook's
  "What to Change" for M4 explicitly calls this out: "do not reject if
  'To be synthesised in M6' markers are present — those are expected at
  M4". M6 adds a separate `check_synthesis_complete` helper; M7 adds
  `check_plan_readiness`. Layering these keeps each milestone's validator
  cleanly scoped.
- **Writer embeds findings verbatim under `## Key Findings`** and stubs
  every other required section with the sentinel. This matches the
  runbook's described M4→M6 handoff: synthesis replaces stubs, not the
  raw-findings body.
- **`write_dossier` creates missing parent dirs.** Same as
  `sldo-plan`'s runbook writer. Users passing `--output deep/new/path.md`
  should not have to pre-create directories.
- **Topic excerpt flattens newlines** and truncates to 200 chars with an
  ellipsis. The frontmatter is a YAML-ish block — a raw newline in the
  `topic:` value would break parsing if anyone ever treats the frontmatter
  as YAML. Single-line excerpt is the safest default.
- **`ResearchFindings` is a plain struct with public fields.** Callers
  (`main.rs`) read both fields once. A builder or accessor methods would
  be ceremony for a single call site, matching the `ResearchConfig`
  precedent.
- **`repo_context` is returned separately from `raw`.** At M3 the
  repo-context output was appended to the accumulated findings; M4 pulls
  it out so the dossier writer can emit a dedicated `## Repository
  Context` section. This is the only behavioural change to
  `research_loop`; every other phase continues to accumulate into `raw`.
- **Main-side `warn` import is new.** Prior M3 main only imported
  `info/success/header/divider`. Adding `warn` lets the dossier-issue
  branch surface validator output in the operator's standard colour
  scheme.

## Mistakes made / necessary handoffs
- **Initial build failed** because `main.rs::run()` moved `prompt_content`
  into `ResearchConfig` and then tried to borrow it again for the dossier
  writer. Fix: clone into the config (`prompt_content: prompt_content.clone()`)
  so the original is available for the later `write_dossier` call. The
  clone is cheap and centralised at one call site.
- **No M3-like E2E harness compromise needed.** Because M4 only adds
  *post-loop* I/O (dossier write under `--output`), the existing M3 E2E
  tests continue to pass. They already use per-test tempdir CWDs. The
  new dossier file lands under those tempdirs and is cleaned up by the
  test's existing `cleanup()` call. No prior-milestone E2E tests were
  touched.

## Root causes
- Passing owned values into a struct literal without cloning them is a
  predictable borrow-checker issue but easy to miss when you're focused on
  the new code. Standard fix (clone at call site) keeps the struct's
  public surface simple.

## Naming conventions established
- **Dossier section constants** use the full header string with the `##`
  prefix (`"## Executive Summary"`) rather than bare names
  (`"Executive Summary"`). This matches how the writer emits them and how
  the validator scans for them — no string concatenation or trimming
  needed in either direction.
- **`M4_STUB_SENTINEL`** lives in `dossier.rs` because it is a dossier
  schema concern, not a research-loop concern. M6 will `use
  crate::dossier::M4_STUB_SENTINEL;` from the synthesis prompt builder.
- **E2E file name** follows the established `e2e_research_m<N>.rs`
  pattern and lives at the workspace root under `tests/`.
- **Test helper names** (`binary`, `unique_tmp`, `shim_dir_with_claude`,
  `shimmed_cmd`, `cleanup`) are copied verbatim from M3's file so future
  milestones can cross-reference. Duplicating these helpers per-file is
  accepted cost — Rust's integration tests each compile as their own
  crate, so sharing via a library would mean publishing a tiny helper
  crate, which violates the "no new abstractions" principle from M3.

## Test patterns that worked well
- **Unit tests for the writer assert on substring presence and
  frontmatter year**, not on exact byte layout. That keeps the tests
  stable across trivial wording changes while still catching missing
  sections.
- **`validate_dossier_tolerates_m4_stub_sentinel`** uses the writer to
  produce the fixture, then validates it. This is a round-trip property
  test — if a future change breaks the stub-sentinel tolerance, both the
  writer and the validator would need to be updated in lockstep, and the
  test forces that coupling visible.
- **E2E tests reuse the M3 PATH-shim pattern** to keep the suite offline
  and free of Claude API costs. The shim's printed marker line is the
  exact string that should end up under `## Key Findings` — so a single
  marker assertion proves both "the dossier was written" and "the
  capture-to-write path preserves content verbatim".

## Missing tests that should exist now
- A test that exercises `write_dossier` with a repo_context containing
  its own `##` headers (nested section names). The current writer just
  calls `ctx.trim_end()` and emits verbatim — edge cases around duplicated
  section headers aren't covered. M5 or M6 will likely add a test here
  once synthesis starts consuming the section.

## Rules for the next milestone (M5)
- **Do NOT run `cargo fmt --all`** — only `cargo fmt -p <crate>` for
  crates you touched and `rustfmt path/to/file.rs` for new test files.
  The workspace still has un-formatted files in non-research crates.
- **Do NOT run workspace-wide `cargo clippy --workspace`** — the Tauri
  and voice-tx crates have pre-existing clippy errors outside M5's
  scope. Run `cargo clippy -p sldo-research --tests` and
  `cargo clippy --test e2e_research_m5` scoped checks only.
- **Do NOT modify `research::run_phase` without updating its callers.**
  The web-search phase loop in M5 must use the same signature; if you
  need to add a new parameter, add an overload or a new helper rather
  than changing the existing one — M3 and M4 callers depend on the
  current shape.
- **Dossier schema is the contract for M5 references.** The M5
  web-search prompt should instruct Claude to emit URL+title pairs that
  M6/M7 will stash under `## References`. Don't change dossier section
  names in M5 — any schema change belongs in M4's file.
- **`ResearchFindings` is the integration seam between the loop and the
  dossier.** If M5 needs to surface web-search text separately (e.g., for
  a dedicated dossier section), add a field to `ResearchFindings` rather
  than threading parallel returns through `research_loop`.
- **Scratch files now coexist with the dossier in the same directory.**
  When writing E2E tests, assert on `.research-scratch-iter-N.md` vs
  `research-dossier.md` by filename — they both live under
  `<output_parent>/`. Don't glob-scan the directory; test specific paths.

## Template improvements suggested
- The M4 runbook step-by-step doesn't mention that `write_dossier`
  needs to cope with empty findings (the `.trim().is_empty()` branch).
  A future runbook template could include an "edge cases" sub-table per
  function to cover empty input, oversized input, and path edge cases
  explicitly.
- The runbook's dossier-writer description says "wrap findings inside
  the '## Key Findings' section" but does not specify what goes in the
  *other* required sections at M4. The implemented answer is "the stub
  sentinel" — future runbooks should call this out so implementers don't
  accidentally emit empty sections.
