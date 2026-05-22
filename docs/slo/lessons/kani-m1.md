# Lessons Learned — kani Milestone 1

## What changed
- New host-neutral skill `skills/slo-kani/` (SKILL.md + pinned `tools.toml` + `references/candidate-scoring.md`), a structural-contract test `xtasks/sast-verify/tests/kani_m1_skill_contract.rs`, a `/slo-kani` row in `docs/skill-pack-catalog.md`, and Kani artifact patterns in `.gitignore`.

## Design decisions and why
- Mirrored `skills/slo-tla/` structure (prereq cascade, suitability gate, method dispatch, common gates) — keeps the two formal-methods skills consistent for users and reuses a proven shape.
- No Rust binary to drive Kani — the skill drives `cargo kani` as a subprocess, same as `/slo-tla` drives the TLC jar. Avoids reintroducing the legacy CLI pattern removed in 2026-04 and the crates.io version-discipline burden.
- Pinned `kani-verifier` to a concrete `0.56.0` with `cargo install --locked` — the structural test rejects `latest`/floating specs (tm-kani-verification-abuse-4).

## Mistakes made
- The concurrency-refusal gate sentence ("concurrency is out of scope") was first phrased non-contiguously across the SKILL.md ("concurrency / async … (out of scope"), so the substring assertion failed on first run.

## Root causes
- Structural tests assert **exact contiguous substrings**; prose that conveys the same meaning with words interleaved does not satisfy a `contains()` check. The test is the contract, not the intent.

## What was harder than expected
- Nothing major. The fmt step reformatted the new test's closure; running `cargo fmt --all` before the `--check` gate is the right order.

## Naming conventions established
- Structural tests for this runbook: `xtasks/sast-verify/tests/kani_m<N>_<topic>.rs` (sibling files, never edits to an existing baseline test).
- Skill references: `skills/slo-kani/references/<phase>.md`, loaded per-phase via the SKILL.md method-dispatch table.
- Test helper idiom copied from `sap_imp_m5_agents.rs`: `workspace_root()` + `read()` + `extract_frontmatter()`.

## Test patterns that worked well
- BDD-first: writing `kani_m1_skill_contract.rs` before the skill files, confirming a 5/5 red for the right reason (files absent), then going green.
- A small pure helper (`pins_concrete_version`) keeps the toolchain-pin assertion robust without pulling in a TOML parser.

## Missing tests that should exist now
- M2 extends this same test file with the four honesty/scope gate sentences plus fail-closed parsing (ENG-2) and write-path validation (SEC-1) assertions — already scoped in the runbook.

## Rules for the next milestone
- **Gate sentences must be exact contiguous substrings.** When M2 adds "naive/pre-fix variant must fail first", "sound over-approximating stubs only", "verdict from tool not narration", "fail closed", and the write-path clause, author the SKILL.md/reference prose to contain the literal substrings the test asserts.
- **Scope clippy to new code.** The `sast-verify` bin has pre-existing dead-code warnings (`Public` variant, unread fields) and `sap_imp_m3_standards` has a "regex in a loop" warning. These predate this runbook; do NOT fix them (out of scope). Run `cargo clippy -p sast-verify --all-targets -- -D warnings` and confirm only that *new* files are clean.
- Keep using `cargo fmt --all` before the `--check` gate.

## Template improvements suggested
- None. The v4 contract block's "static analysis gates" row correctly anticipated the minimal-waiver path for pre-existing warnings.

## filed_issues
- none — M1 lessons are forward-rules captured in this file; `/slo-execute M2` reads them directly via Global Entry Step 1. No cross-cutting tracked-issue-worthy item. (Issue filing is additive and was intentionally skipped, not failed.)
