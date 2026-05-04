---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M2
created: 2026-05-04
status: done
---

# Lessons - eng-imp M2

## Cut Plan

M2 used a dispatcher-plus-methodology cut for `/slo-sast`.

| Destination | Content moved or retained |
|---|---|
| `skills/slo-sast/SKILL.md` | Kept frontmatter, one-line role, shared discipline links, inputs/output envelope, pre-flight, method-dispatch table, common anti-patterns, and see-also links. Final size: 71 lines. |
| `skills/slo-sast/references/methodology-m1-parser.md` | Parser scaffold, threat-model parser scope rule, process, empty-list behavior, output-format note, and M1-specific anti-patterns. |
| `skills/slo-sast/references/methodology-m2-stack-detect.md` | M2 output envelope, coverage-gap reporting, stack detection, registry fetch, rule filter, and M2-specific anti-patterns. |
| `skills/slo-sast/references/methodology-m3-emission.md` | Emission flow, symlink-traversal defense, workflow safety contract, CWE-list independence, and M3-specific anti-patterns. |
| `skills/slo-sast/references/methodology-m4-manifest.md` | Manifest schema v1.0, preview-mode UX, rollback contract, and M4-specific anti-patterns. |
| `skills/slo-sast/references/methodology-m5-pr-creation.md` | Re-derivation triggers, PR creation, `gh pr create` discipline, dogfood isolation, and M5-specific anti-patterns. |

## Compatibility Notes

- Existing `e2e_scanner_orch_m1..m5` tests still read `skills/slo-sast/SKILL.md` for sentinel phrases. The thin dispatcher intentionally keeps compact `Method (M<N>)` markers and high-risk phrases while moving detailed execution prose into methodology files.
- The new M2 test enforces the new shape directly: line cap, five method files, frontmatter, skill-local `references/` location, preserved security disciplines, and verbatim survival for load-bearing MUST / MUST NOT rules.
- `references/sast/` authority files stayed untouched. This matters because those files remain the source of truth; the new `skills/slo-sast/references/` files are orchestration scaffolds, not replacement authorities.

## Evidence

| Check | Actual Result |
|---|---|
| Baseline before M2 edits | `cargo test --workspace` passed on branch `slo/eng-imp-m2` before decomposition edits. |
| Red-first M2 test | `cargo test -p sldo-install --test e2e_eng_imp_m2` failed for the expected reasons: 312-line SKILL.md, missing methodology files, missing dispatch links. |
| M2 structural test after implementation | `cargo test -p sldo-install --test e2e_eng_imp_m2` passed: 6 passed, 0 failed. |
| Scanner-orch compatibility tests | `cargo test -p sldo-install --test e2e_scanner_orch_m1 --test e2e_scanner_orch_m2 --test e2e_scanner_orch_m3 --test e2e_scanner_orch_m4 --test e2e_scanner_orch_m5` passed. |
| Package test suite | `cargo test -p sldo-install` passed. |
| Workspace test suite | `cargo test --workspace` passed after the standards-mapping link was restored in the thin dispatcher. |
| Workspace build | `cargo build --workspace` passed with pre-existing warnings in `xtasks/sast-verify`. |
| Formatting | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m2.rs` passed; `cargo fmt --check -p sldo-install` remains blocked by pre-existing unrelated drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| Line count | `skills/slo-sast/SKILL.md` is 71 lines. |
| Authority docs untouched | `git diff --name-status -- references/sast` returned no changes. |

## Rules For The Next Milestone

- In M3, use the same dispatcher-plus-methodology pattern for `/slo-tla`, but keep the suitability gate in `skills/slo-tla/SKILL.md`; that gate is cross-stage discipline, not stage-local detail.
- Before cutting `/slo-tla`, scan the existing TLA tests for sentinel phrases. Prefer compact dispatcher markers over widening the allow-list to rewrite old tests.
- Preserve source-backed rules verbatim where they mention bounds, fairness, checksum verification, JVM/TLC/Apalache prerequisites, state-explosion triage, and counterexample translation.
- The Apalache pin must be source-verified and recorded in M3 lessons with version, download URL, SHA-256, and retrieval date.
- Keep method files under `skills/slo-tla/references/` so they travel with the installed skill symlink.

## Allow-List Note

The M2 Contract Block allowed the SAST skill, five skill-local methodology files, and the structural test. The runbook Definition of Done also requires tracker, lessons, completion, and an ARCHITECTURE post-flight note. Those closeout/doc-index edits are recorded as milestone evidence rather than product-surface changes.
