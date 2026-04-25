# Completion Summary — sast-rulegen-a Milestone 2

## Goal completed

`/slo-rulegen --extend` is contracted, documented, and disk-validated. The full extend-mode prompt body in `references/sast/prompts/extend.md` describes the 7-step procedure (validate inputs → render-as-untrusted → identify CWE → enumerate variations → author rule + fixture → atomic-write → idempotency on collision). The skill's `slo-rulegen/SKILL.md` has an Extend-mode contract section. Test fixtures (clean + adversarial) are committed for future runtime BDD. Workspace E2E asserts structural properties of the M2-shipped artifacts.

## Files changed

- `references/sast/prompts/extend.md` — filled the M1 skeleton with the full extend-mode prompt body
- `skills/slo-rulegen/SKILL.md` — added the "Extend-mode contract" section
- `tests/fixtures/extend_mode/good_bug/bug-summary.md` (NEW) — clean panic-on-Result-fn fixture
- `tests/fixtures/extend_mode/good_bug/fix.diff` (NEW) — clean fix diff
- `tests/fixtures/extend_mode/malicious_bug/bug-summary.md` (NEW) — adversarial fixture with prompt-injection attempts
- `tests/fixtures/extend_mode/malicious_bug/fix.diff` (NEW) — adversarial diff with commit-message-shaped injection
- `tests/e2e_sast_rulegen_a_m2.rs` (NEW) — 7 workspace-level integration tests
- `Cargo.toml` — registered the M2 E2E test
- `docs/RUNBOOK-SAST-RULEGEN-A.md` — Milestone Tracker M2 → done

## Tests added

7 in `tests/e2e_sast_rulegen_a_m2.rs`:

1. `extend_md_is_no_longer_a_skeleton`
2. `extend_md_forbids_webfetch_and_websearch_in_prose`
3. `extend_md_cites_threat_model_row_for_prompt_injection`
4. `good_bug_fixture_exists_and_has_required_files`
5. `malicious_bug_fixture_contains_prompt_injection_attempt`
6. `skill_md_documents_extend_mode_contract`
7. `skill_md_forbids_webfetch_and_websearch_in_extend_section`

All passing.

## Runtime validations added

`tests/e2e_sast_rulegen_a_m2.rs` runs to completion in < 1 second (no semgrep invocation; pure disk-content assertions). End-to-end runtime invocation of `/slo-rulegen --extend` against the malicious fixture is deferred to `/slo-verify` Pass 4 since it requires the Claude Code slash-command runtime.

## Compatibility checks performed

- M1 BDD + integration tests still pass (`cargo test -p sast-verify`)
- M1 baseline tests still pass (`cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`)
- M1's 3 bootstrap rules (CWE-755 / CWE-190 / CWE-295) still gate-clean
- The new test (`tests/e2e_sast_rulegen_a_m2.rs`) is registered in `[[test]]` per workspace convention; runs cleanly under `cargo test --test e2e_sast_rulegen_a_m2`

## Documentation updated

- `references/sast/prompts/extend.md` — full extend-mode prompt body (replaces M1 skeleton)
- `skills/slo-rulegen/SKILL.md` — extend-mode contract section
- `docs/lessons/sast-rulegen-a-m2.md` (this milestone) — written
- `docs/completion/sast-rulegen-a-m2.md` (this file) — written

## .gitignore changes

No additions in M2. The `.semgrep/.scratch/` and `xtasks/sast-verify/tests/scratch/` patterns become relevant when the atomic-write helper actually runs at runtime; the M2 ship records the contract but doesn't exercise it. Adding gitignore patterns for paths that don't exist yet would add noise. M2.5 / actual extend-mode invocation should add them.

## Test artifact cleanup verified

`git status` clean after `cargo test --test e2e_sast_rulegen_a_m2`. No untracked files.

## Deferred follow-ups

**M2.5 — runtime-mode polish:**
- Move the `--file-paths` validator into the xtask as `cargo xtask sast-verify validate-file-paths <csv>` for shell-out reuse.
- Move the atomic-write helper into the xtask as `cargo xtask sast-verify gate --temp-dir <dir>` so RAII cleanup is enforced in Rust, not in the skill prompt.
- Run `/slo-rulegen --extend` end-to-end against `tests/fixtures/extend_mode/good_bug/` and assert the produced rule pack passes M1's `gate_passes_for_all_authored_rules` integration test.
- Run `/slo-rulegen --extend` against `tests/fixtures/extend_mode/malicious_bug/` and assert the produced rules contain zero URLs in pattern bodies (prompt-injection resistance runtime BDD).

These are deferred to `/slo-verify` Pass 4 because they require the Claude Code slash-command runtime, not just `cargo test`.

## Known non-blocking limitations

- The M2 E2E covers structural properties of disk artifacts. The actual runtime behaviour (extend-mode end-to-end) is not exercised in this milestone — it lives in `/slo-verify` Pass 4 because it requires Claude Code's slash-command runtime.
- The `.semgrep/.scratch/` directory is referenced in extend.md but not created in M2. The first `--extend` runtime invocation creates it; M2.5 should add the gitignore pattern.
- The atomic-write contract is described but not Rust-enforced in M2; the skill prompt is the contract carrier. If the skill author skips a step, the contract is violated. M2.5 lifts the helper into Rust.

## Verification of /slo-critique findings applied

- **eng-5** (atomic-write tempdir+rename): documented in extend.md "Atomic-write contract" section; assert in `extend_md_is_no_longer_a_skeleton`. Rust enforcement deferred to M2.5. ✓ (contract-side)
- **sec-3** (`--file-paths` traversal): documented in extend.md "Validate inputs" section. Rust validator deferred to M2.5. ✓ (contract-side)
- **eng-4** (tier-detect URL shapes): M1's `detect_tier` returns `Confidential` always for v1. Extend-mode prompt body documents this and requires explicit `--target-tier public` for the public corpus tier. ✓
- **sec-5** (SKILL.md prose forbids WebFetch/WebSearch): both `references/sast/prompts/extend.md` AND `skills/slo-rulegen/SKILL.md` have explicit prose denials with tm-abuse-1 citations. Asserted by `extend_md_forbids_webfetch_and_websearch_in_prose` + `skill_md_forbids_webfetch_and_websearch_in_extend_section`. ✓
