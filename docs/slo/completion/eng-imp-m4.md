---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M4
completed: 2026-05-04
status: done
---

# Completion - eng-imp M4

**Goal**: Decompose `/slo-plan`'s per-milestone authoring procedure and add a
soft line-cap structural test for future skill-pack drift.

## Shipped

| File | Change |
|---|---|
| `skills/slo-plan/SKILL.md` | Reduced from 143 lines to 63-line dispatcher with one-shot refusal, method link, security row sentinels, gates, and handoff. |
| `skills/slo-plan/references/methodology-milestone-authoring.md` | Added per-milestone authoring procedure with the 15 section elements and security Contract Block row rules. |
| `crates/sldo-install/tests/e2e_eng_imp_m4.rs` | Added hard `/slo-plan` cap, soft skill cap, methodology cap, and shared-template cap tests. |
| `skills/slo-execute/SKILL.md` | Added reasoned soft-cap exception pragma; no behavior text changed. |
| `skills/slo-retro/SKILL.md` | Added reasoned soft-cap exception pragma; no behavior text changed. |
| `docs/slo/lessons/eng-imp-m4.md` | Added cut plan, soft-cap inventory, evidence, and M5 rules. |
| `docs/ARCHITECTURE.md` | Documented `/slo-plan` as a user of the skill-local references pattern. |

## Validation

| Command / Check | Result |
|---|---|
| `cargo test --workspace` before edits | Passed. |
| `cargo test -p sldo-install --test e2e_eng_imp_m4` before implementation | Failed for expected red-first reasons. |
| `cargo test -p sldo-install --test e2e_eng_imp_m4` after implementation | Passed. |
| `cargo test -p sldo-install --test e2e_slo_sp_m4 --test e2e_slo_sec_m2 --test e2e_v4_template --test e2e_slo_sp_m7 --test e2e_loops_m3 --test e2e_loops_m4` | Passed. |
| `cargo test -p sldo-install` | Passed. |
| `cargo test --workspace` | Passed. |
| `cargo build --workspace` | Passed with pre-existing `sast-verify` warnings. |
| `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m4.rs` | Passed. |
| `git diff --check` | Passed. |
| `cargo fmt --check -p sldo-install` | Still blocked by pre-existing unrelated formatting drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| `wc -l skills/slo-plan/SKILL.md` | 63 lines. |
| `rg -n "soft-cap-exception" skills/*/SKILL.md` | Two explicit exceptions: `/slo-execute` and `/slo-retro`. |

## Carry Forward

M5 can now rely on the soft-cap guard. Keep new eval and hook documentation
concise, and use reference files rather than growing SKILL.md bodies.
