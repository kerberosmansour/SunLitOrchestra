---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M4
created: 2026-05-04
status: done
---

# Lessons - eng-imp M4

## Cut Plan

M4 moved the milestone-authoring sub-procedure out of `/slo-plan` and added the
soft-cap structural guard.

| Destination | Content moved or retained |
|---|---|
| `skills/slo-plan/SKILL.md` | Kept frontmatter, inputs/output, the one-shot refusal rule, method summary, security Contract Block sentinels, gates, anti-patterns, and handoff. Final size: 63 lines. |
| `skills/slo-plan/references/methodology-milestone-authoring.md` | Moved the per-milestone authoring procedure, including the 15 required section elements and the three security Contract Block rows. |
| `crates/sldo-install/tests/e2e_eng_imp_m4.rs` | Added soft-cap tests for every `SKILL.md`, methodology files, and shared templates. |

## Soft-Cap Exceptions

The first full inventory after M3 found two skills at 202 lines. M4 added
frontmatter pragmas with explicit reasons:

| Skill | Reason |
|---|---|
| `skills/slo-execute/SKILL.md` | Carries execution safety gates plus GitHub carry-forward preflight. |
| `skills/slo-retro/SKILL.md` | Carries milestone closeout, lessons, and issue-filing discipline. |

The new test treats those as review-visible exceptions rather than silent
growth. `/slo-sast`, `/slo-tla`, and `/slo-plan` now pass the cap directly.

## Evidence

| Check | Actual Result |
|---|---|
| Repo hygiene | Branch before edits: `slo/eng-imp-m4`; branch after edits: `slo/eng-imp-m4`; dirty tree before edits: clean; remediation needed: none. |
| Prior-retro carry-forward | `gh issue list --label retro-derived --search "eng-imp" --state open --json number,title,body,url` returned `[]`. |
| Baseline before M4 edits | `cargo test --workspace` passed on branch `slo/eng-imp-m4` before decomposition edits. |
| Red-first M4 test | `cargo test -p sldo-install --test e2e_eng_imp_m4` failed for expected reasons: 143-line `/slo-plan`, missing methodology file, missing soft-cap exception, and methodology-count threshold not yet met. |
| M4 structural test after implementation | `cargo test -p sldo-install --test e2e_eng_imp_m4` passed: 6 passed, 0 failed. |
| Compatibility sentinels | `cargo test -p sldo-install --test e2e_slo_sp_m4 --test e2e_slo_sec_m2 --test e2e_v4_template --test e2e_slo_sp_m7 --test e2e_loops_m3 --test e2e_loops_m4` passed. |
| Package test suite | `cargo test -p sldo-install` passed. |
| Workspace test suite | `cargo test --workspace` passed after implementation and closeout docs. |
| Workspace build | `cargo build --workspace` passed with pre-existing `sast-verify` warnings. |
| Formatting | `rustfmt --edition 2021 crates/sldo-install/tests/e2e_eng_imp_m4.rs` and `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m4.rs` passed; `cargo fmt --check -p sldo-install` remains blocked by pre-existing unrelated drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| Diff hygiene | `git diff --check` passed. |
| Line counts | `skills/slo-plan/SKILL.md` is 63 lines; two soft-cap exceptions are explicit and reasoned. |

## Rules For The Next Milestone

- M5 touches broader surfaces: per-skill evals, `/slo-freeze` hook wiring, and cross-skill polish. Re-read the allow-list carefully before editing.
- The new soft-cap test will reject any newly oversized SKILL.md without a reasoned pragma; keep M5 prose tight or put detail in skill-local references.
- Preserve the two M4 soft-cap exceptions as review-visible markers unless M5 intentionally decomposes those skills.
- If M5 adds eval files, keep them small and synthetic; do not introduce real founder/user PII.

## Allow-List Note

The M4 Contract Block allowed `/slo-plan`, its new methodology file, and the new
structural test. The milestone's own Step 5 required adding soft-cap pragmas
where existing skills exceeded the cap, which affected `slo-execute` and
`slo-retro`. Those edits are pragma-only, behavior-neutral, and recorded as the
soft-cap inventory result.
