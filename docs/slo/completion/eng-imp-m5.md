---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M5
completed: 2026-05-04
status: done
---

# Completion - eng-imp M5

**Goal**: Add high-risk skill eval cases, an opt-in `/slo-freeze` PreToolUse
hook, and the named cross-skill polish items.

## Shipped

| File / Surface | Change |
|---|---|
| `skills/{...}/evals/happy-path.md` | Added one synthetic happy-path eval for each of the 16 high-risk skills in the M5 contract. |
| `.claude/settings.json` | Added project-local `PreToolUse` hook for `Edit`, `Write`, and `NotebookEdit`; hook reads `~/.sldo/freeze-scope.txt`. |
| `references/freeze/hook-setup.md` | Added opt-in setup guidance, additive-update rule, and non-security-boundary residual risk. |
| `references/biz/consent-script-uk.md` | Added UK GDPR legitimate-interest consent language for founder interviews. |
| `skills/slo-freeze/SKILL.md` | Documented optional hook behavior, missing-scope fallback, and "not a security boundary" caveat. |
| `skills/slo-second-opinion/SKILL.md` | Added minimum CLI version check and "neither response is verified" guardrail. |
| `skills/get-api-docs/SKILL.md` | Added `chub search`/`chub get` failure handling and no-training-memory fallback rule. |
| `skills/slo-talk-to-users/SKILL.md` | Made git checks explicit and linked the consent script. |
| `skills/slo-verify/SKILL.md` | Added `pass/fail/skipped/N/A` Pass 4 result vocabulary. |
| `crates/sldo-install/tests/e2e_eng_imp_m5.rs` | Added structural-contract tests for eval dirs, frontmatter, hook JSON, hook docs, consent script, and polish text. |
| `README.md` | Added docs-index pointer for per-skill eval cases. |
| `docs/ARCHITECTURE.md` | Added "Hooks and evals" reality-at-HEAD subsection. |

## Validation

| Command / Check | Result |
|---|---|
| `cargo test --workspace` before edits | Passed. |
| `cargo test -p sldo-install --test e2e_eng_imp_m5` before implementation | Failed for expected red-first reasons. |
| `cargo test -p sldo-install --test e2e_eng_imp_m5` after implementation | Passed. |
| Hook smoke with temp `HOME` | In-scope target exited 0; out-of-scope target exited 2; missing scope file exited 0. |
| `cargo test -p sldo-install --test e2e_eng_imp_m4` | Passed. |
| `cargo test -p sldo-install --test e2e_biz_b1_m1 --test e2e_slo_sp_m5 --test e2e_eng_imp_m3` | Passed. |
| `cargo test -p sldo-install` | Passed with pre-existing unrelated `e2e_biz_followup_m5.rs` warning. |
| `cargo test --workspace` | Passed with pre-existing `sast-verify` warnings and the unrelated warning above. |
| `cargo build --workspace` | Passed with pre-existing `sast-verify` warnings. |
| `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` | Passed. |
| `git diff --check` | Passed. |
| `cargo fmt --check -p sldo-install` | Still blocked by pre-existing unrelated formatting drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |

## Carry Forward

The next engineering-improvement step should be PR closeout. M5 intentionally
implemented the 16-skill acceptance gate; the much larger all-skill eval matrix
belongs with the deferred runtime harness work.
