---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M3
completed: 2026-05-04
status: done
---

# Completion - eng-imp M3

**Goal**: Decompose `/slo-tla` into a thin SKILL.md plus four methodology
references, while preserving the suitability gate and pinning Apalache with a
source-verified SHA-256.

## Shipped

| File | Change |
|---|---|
| `skills/slo-tla/SKILL.md` | Reduced from 333 lines to 150-line dispatcher with prereq cascade, suitability gate, method links, and compatibility sentinels. |
| `skills/slo-tla/references/methodology-elicitation.md` | Added elicitation and staged spec-drafting methodology. |
| `skills/slo-tla/references/methodology-abstraction.md` | Added abstraction-balance and state-explosion methodology. |
| `skills/slo-tla/references/methodology-counterexample.md` | Added counterexample translation methodology. |
| `skills/slo-tla/references/methodology-verified-design.md` | Added verified-design document shape, refusal gates, anti-patterns, and handoff detail. |
| `skills/slo-tla/tools.toml` | Updated `[apalache]` to source-verified `0.57.0` with `download_url` and SHA-256; kept existing TLC pin unchanged. |
| `crates/sldo-install/tests/e2e_eng_imp_m3.rs` | Added structural test for decomposition, line cap, methodology files, preserved TLA+ disciplines, and Apalache pin format/value. |
| `docs/slo/lessons/eng-imp-m3.md` | Added cut plan, Apalache verification evidence, and M4 rules. |
| `docs/ARCHITECTURE.md` | Documented `/slo-tla` as a user of the skill-local references pattern. |

## Validation

| Command / Check | Result |
|---|---|
| `cargo test --workspace` before edits | Passed. |
| `cargo test -p sldo-install --test e2e_eng_imp_m3` before implementation | Failed for expected red-first reasons. |
| `cargo test -p sldo-install --test e2e_eng_imp_m3` after implementation | Passed. |
| `cargo test -p sldo-install --test e2e_slo_sp_m5` | Passed. |
| `cargo test -p sldo-install` | Passed. |
| `cargo test --workspace` | Passed. |
| `cargo build --workspace` | Passed with pre-existing `sast-verify` warnings. |
| `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m3.rs` | Passed. |
| `git diff --check` | Passed. |
| `cargo fmt --check -p sldo-install` | Still blocked by pre-existing unrelated formatting drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| `wc -l skills/slo-tla/SKILL.md` | 150 lines. |
| Apalache SHA-256 capture | Upstream `sha256sum.txt` and local `shasum -a 256` both returned `cb805df9a68e2f278c45e751522aab119b57a454e3e0e96f5d974b969fe52b5d`. |

## Carry Forward

M4 should decompose `/slo-plan` with the same pattern, then land the soft
line-cap structural test. The test should pass `/slo-sast` and `/slo-tla`
without exceptions and force any remaining oversized skill to carry an explicit
reason.
