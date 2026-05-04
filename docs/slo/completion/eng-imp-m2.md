---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M2
completed: 2026-05-04
status: done
---

# Completion - eng-imp M2

**Goal**: Decompose `/slo-sast` into a thin SKILL.md plus five stage-specific methodology references without losing the security-sensitive rules from the monolithic skill.

## Shipped

| File | Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Reduced from 312 lines to 71-line dispatcher with method links and compatibility sentinels. |
| `skills/slo-sast/references/methodology-m1-parser.md` | Added parser scaffold methodology. |
| `skills/slo-sast/references/methodology-m2-stack-detect.md` | Added stack-detection, registry-fetch, and rule-filter methodology. |
| `skills/slo-sast/references/methodology-m3-emission.md` | Added emission and workflow safety methodology. |
| `skills/slo-sast/references/methodology-m4-manifest.md` | Added manifest and preview-mode methodology. |
| `skills/slo-sast/references/methodology-m5-pr-creation.md` | Added re-derivation and PR-creation methodology. |
| `crates/sldo-install/tests/e2e_eng_imp_m2.rs` | Added structural test for decomposition, line cap, method files, and preserved security disciplines. |
| `docs/slo/lessons/eng-imp-m2.md` | Added cut plan, evidence, and M3 rules. |
| `docs/ARCHITECTURE.md` | Documented the skill-local references decomposition pattern. |

## Validation

| Command / Check | Result |
|---|---|
| `cargo test --workspace` before edits | Passed. |
| `cargo test -p sldo-install --test e2e_eng_imp_m2` before implementation | Failed for expected red-first reasons. |
| `cargo test -p sldo-install --test e2e_eng_imp_m2` after implementation | Passed. |
| `cargo test -p sldo-install --test e2e_scanner_orch_m1 --test e2e_scanner_orch_m2 --test e2e_scanner_orch_m3 --test e2e_scanner_orch_m4 --test e2e_scanner_orch_m5` | Passed. |
| `cargo test -p sldo-install` | Passed. |
| `cargo test --workspace` | Passed. |
| `cargo build --workspace` | Passed with pre-existing `sast-verify` warnings. |
| `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m2.rs` | Passed. |
| `git diff --check` | Passed. |
| `cargo fmt --check -p sldo-install` | Still blocked by pre-existing unrelated formatting drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| `wc -l skills/slo-sast/SKILL.md` | 71 lines. |
| `git diff --name-status -- references/sast` | No changes. |

## Carry Forward

M3 should reuse this cut pattern for `/slo-tla`: keep cross-stage gates in the thin SKILL.md, move stage-local methodology into `skills/slo-tla/references/`, source-verify the Apalache SHA-256 pin, and preserve existing test sentinels with compact dispatcher markers.
