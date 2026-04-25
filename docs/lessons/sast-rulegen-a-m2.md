# Lessons Learned — sast-rulegen-a Milestone 2

## What changed

- `references/sast/prompts/extend.md` — filled the skeleton from M1 with the full extend-mode prompt body. Documents: input contract, render-as-untrusted `~~~text` fence rule, validation procedure for `--file-paths`, CWE identification, variation enumeration, atomic-write contract via `tempfile::TempDir` + `fs::rename`, tier discipline, idempotency-on-collision contract, never-do list.
- `skills/slo-rulegen/SKILL.md` — added an "Extend-mode contract" section that summarizes the 7-step procedure and points to `references/sast/prompts/extend.md` for the full body. Reaffirms the SKILL.md `## Tools you MUST NOT use` section per `/slo-critique` sec-5.
- `tests/fixtures/extend_mode/good_bug/{bug-summary.md,fix.diff}` — clean fixture: panic-on-Result-fn at `src/api/users.rs:42`, fix changes `.unwrap()` → `.map_err(...)?`. Used by future end-to-end extend-mode runs and by `tests/e2e_sast_rulegen_a_m2.rs` to assert fixture presence.
- `tests/fixtures/extend_mode/malicious_bug/{bug-summary.md,fix.diff}` — adversarial fixture deliberately containing prompt-injection attempts (`ignore prior instructions`, exfil URL, commit-message-shaped injection in the diff). Used to assert prompt-injection resistance posture in code review and in future runtime BDD.
- `tests/e2e_sast_rulegen_a_m2.rs` — workspace-level E2E with 7 tests asserting structural properties of the M2-shipped artifacts (extend.md no-longer-skeleton, ~~~text fence presence, atomic-write contract documented, tm-abuse-1 cited, fixtures present, SKILL.md extend-mode contract section + Tools-MUST-NOT-use section).
- `Cargo.toml` workspace `[[test]]` registration for the M2 E2E.

## Design decisions and why

- **Extend-mode is prompt-driven, not Rust-coded.** The skill is markdown; the prompt body lives in `references/sast/prompts/extend.md`. The atomic-write contract is described in the prompt; the skill orchestrates `tempfile::TempDir` + `fs::rename` via Bash shell-out at runtime. This keeps the xtask focused on the gate (the deterministic per-rule check) and the skill focused on the orchestration. Net cost: the atomic-write logic isn't enforced in Rust, so it depends on the skill following the prompt. Net benefit: when M2.5 lifts the helper into the xtask (as documented in the "Deferred to M2.5 polish" section of extend.md), the prompt body shrinks but the contract is unchanged.
- **Tests assert structural properties of disk artifacts.** End-to-end extend-mode invocation requires the Claude Code slash-command runtime, which `cargo test` cannot invoke. The 7 M2 tests assert what's verifiable from disk: prompt content sentinels, fixture presence, SKILL.md section structure. The actual prompt-injection-resistance behaviour (the rule that gets generated when extend-mode is run on malicious_bug fixture) is a runtime BDD that lives in `/slo-verify` Pass 4, not in the unit/integration suite.
- **Malicious fixture deliberately contains injection attempts.** The fixture's purpose is to be the input to a future BDD scenario `prompt_injection_in_bug_summary_does_not_emit_url_in_pattern` (M2.5 / `/slo-verify`). Its content is by design — a runner that processes the fixture as code rather than data violates the `~~~text` fence rule.
- **Defer to M2.5: Rust-side `--file-paths` validator.** The runbook M2 contract calls for `Path::canonicalize()` + repo-root prefix assert. Implementing that as a `cargo xtask sast-verify validate-file-paths` subcommand was within reach but adds another subcommand to the xtask without a clear test for it. Documented as M2.5 deferred. The skill's prompt body still requires the validation; if the skill skips it, that's a skill-side bug, caught by `/slo-verify` Pass 4 BDD.

## Mistakes made

- **Initial extend.md kept the M1 skeleton sentinel `NOT YET IMPLEMENTED`.** Caught immediately by the M2 E2E test `extend_md_is_no_longer_a_skeleton`. Fix: re-frame the section as "Deferred to M2.5 polish". Time cost: ~2 min.

## Root causes

- The skeleton section was left from M1 to mark "M2 will fill this." When I replaced the skeleton body with the full extend-mode prompt, I forgot to rename the deferred-work section header. Lesson: the skeleton sentinels in M1-shipped files should be unique and easy-to-grep-for so M2's BDD can catch them.

## What was harder than expected

- **Authoring an extend.md body that resists prompt injection in its OWN body.** The malicious fixture's content is deliberately injection-shaped; the prompt body has to make it CLEAR to the LLM that the fixture content is data and not instructions. Solution: the `~~~text` fence rule is described AND the prompt template literally shows the fence around the placeholder substitution. The skill renders user content inside fences when invoking Claude — this is the load-bearing contract.

## Naming conventions established

- Test fixtures for extend-mode live at `tests/fixtures/extend_mode/<scenario>/{bug-summary.md, fix.diff}`. Scenario name describes the semantics: `good_bug`, `malicious_bug`, `proprietary_diff` (M2.5 if needed).
- E2E test naming: `tests/e2e_sast_rulegen_a_m<N>.rs` per the runbook E2E convention.

## Test patterns that worked well

- **Sentinel-based "skeleton no longer present" assertion.** `extend_md_is_no_longer_a_skeleton` greps for two sentinels (`M2 fills this in`, `NOT YET IMPLEMENTED`) AND for two body markers (`Render-as-untrusted contract`, `~~~text`). Catches both regression-to-skeleton AND drift-from-contract.
- **Threat-model-row citation tests.** `extend_md_cites_threat_model_row_for_prompt_injection` asserts the prompt body explicitly cites `tm-sast-rulegen-skill-pack-abuse-1`. This forces the prompt author to keep the threat-model traceability.

## Missing tests that should exist now

- **End-to-end extend-mode runtime BDD.** The 7 disk-asserting tests in `e2e_sast_rulegen_a_m2.rs` cover structure but NOT runtime behaviour. A `/slo-verify` Pass 4 scenario should: (a) invoke `/slo-rulegen --extend` against `tests/fixtures/extend_mode/malicious_bug/`, (b) assert the resulting rule pack contains zero URLs in pattern bodies, (c) assert the deny-list of WebFetch/WebSearch was honoured during the run. Out of scope for M2; lands in `/slo-verify` Pass 4.
- **Atomic-write Rust unit test.** The skill orchestrates the temp-dir-then-rename in markdown. A Rust test asserting "running the orchestration with a deliberately-failing rule leaves zero rules in `.semgrep/<lang>/`" would validate the contract concretely. M2.5 if the helper moves into the xtask.

## Rules for the next milestone (M3)

- **Workflow YAML BDD must split into TWO assertions** per `/slo-critique` sec-4 reframe: `workflow_does_not_invoke_extend_or_rulegen_paths` (forbids `--extend`, `slo-rulegen`) AND `workflow_invokes_ruleverify_for_admission_control` (REQUIRES `cargo xtask sast-verify gate` step before `semgrep ci`).
- **`returntocorp/semgrep-action` must be SHA-pinned**, not tag-pinned. Per the runbook M3 BDD `workflow_invokes_pinned_semgrep_action`.
- **`.pre-commit-config.yaml` must work under both `pre-commit` (Python) and `prek` (Rust drop-in)** — same YAML, dual-runner verified per `/slo-critique`/synthesis.
- **LICENSE is Apache-2.0 OR MIT, NEVER AGPL or GPL** per the SECURITY.md LICENSE addendum.

## Template improvements suggested

- The runbook M2 BDD scenarios mention features (`extend_mode_atomic_via_tempdir_rename`, `extend_mode_no_partial_writes_after_interrupt`) that genuinely require runtime invocation. Suggest the v3 template add a "BDD execution tier" — `unit` (cargo test), `integration` (xtask shell-out), `runtime` (slash-command via Claude Code; deferred to /slo-verify Pass 4). Currently the runbook's BDD table doesn't distinguish, leading to ambiguity about which tests must pass at milestone-close vs. ship-close.
