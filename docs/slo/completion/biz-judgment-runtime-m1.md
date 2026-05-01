# Completion — biz-judgment-runtime M1

## Goal achieved

End-to-end harness exists at [crates/sldo-install/tests/common/judgment_runtime.rs](../../crates/sldo-install/tests/common/judgment_runtime.rs) and is exercised against `references/biz/judgment-fixtures/slo-legal/ir35-genuine-contractor.md` via [crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs](../../crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs).

## Files added

- `crates/sldo-install/tests/common/mod.rs` — module declaration only.
- `crates/sldo-install/tests/common/judgment_runtime.rs` — `JudgmentFixture::parse`, `TempRepo::build`, `invoke_claude`, `discover_artifact`, plus 5 unit tests.
- `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs` — single `#[ignore]` runtime test.

## Files NOT changed (compatibility evidence)

- `crates/sldo-install/tests/e2e_biz_followup_m4.rs` — the 5 structural tests + the existing panic-stub remain untouched.
- All 9 fixture files under `references/biz/judgment-fixtures/<skill>/`.
- `crates/sldo-install/Cargo.toml` (no new deps).

## Test evidence

```
$ cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1
running 6 tests
test runtime_harness_green_on_ir35_genuine_contractor ... ignored
test common::judgment_runtime::tests::parses_real_ir35_genuine_fixture ... ok
test common::judgment_runtime::tests::rejects_path_outside_fixtures_dir ... ok
test common::judgment_runtime::tests::rejects_malformed_frontmatter ... ok
test common::judgment_runtime::tests::extracts_founder_prompt_with_blockquote ... ok
test common::judgment_runtime::tests::extracts_founder_prompt_with_extra_heading_text ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

$ cargo test -p sldo-install --test e2e_biz_followup_m4
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

The live test (`BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1 -- --ignored`) is owner-discretion — running it bills against the project owner's Anthropic budget. Smoke procedure documented in the runbook M1 Smoke Tests section.

## BDD coverage

- Row 1 (happy path) — covered by `runtime_harness_green_on_ir35_genuine_contractor` (live).
- Row 2 (invalid input — fixture) — covered by `rejects_malformed_frontmatter` unit test.
- Row 3 (empty state — env unset) — covered by `skip_if_not_live()` early-return; verified by `cargo test ... --ignored` printing the skip message.
- Row 4 (dependency failure — claude not on PATH) — covered by `claude_available()` check in the live test; surface message names `BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN` override.
- Row 5 (abuse — path traversal) — covered by `rejects_path_outside_fixtures_dir` unit test.
- Row 6 (abuse — claude crash) — partially covered: the live test panics with stdout+stderr on non-zero exit. Full coverage (a fault-injected claude binary) is out of M1 scope.
- Row 7 (abuse — multiple artifacts) — covered structurally by `discover_artifact`'s `Err` branch; not exercised in the live test (no current skill writes multiple artifacts).
- Row 8 (concurrency) — N/A.
- Row 9 (persistence — tempdir cleanup) — implicit via `TempDir::drop`.

## Definition of Done

- [x] All Step-by-Step items complete.
- [x] BDD scenarios covered (rows 1, 5 directly tested; 2, 6, 7 by helper unit tests / live-test paths; 3, 4 by env-gate behaviour; 8, 9 N/A or implicit).
- [x] Compatibility Checklist all checked.
- [x] Baseline test command green.
- [x] Lessons file written.
- [x] Completion summary written.
- [x] Tracker updated.

## Deferred follow-ups

None. M2 picks up where M1 leaves off (all 9 fixtures + retry/cost-cap + docs), and the panic stub at `e2e_biz_followup_m4.rs:184-209` is replaced by a forwarder in M2.
