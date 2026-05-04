---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M3
created: 2026-05-04
status: done
---

# Lessons - eng-imp M3

## Cut Plan

M3 reused the dispatcher-plus-methodology cut for `/slo-tla`.

| Destination | Content moved or retained |
|---|---|
| `skills/slo-tla/SKILL.md` | Kept frontmatter, role, shared discipline links, inputs/outputs, prereq cascade, suitability gate, method-dispatch table, common refusal gates, anti-patterns, and handoff. Final size: 150 lines. |
| `skills/slo-tla/references/methodology-elicitation.md` | Moved Q1-Q6 elicitation prompts and staged spec drafting. |
| `skills/slo-tla/references/methodology-abstraction.md` | Moved abstraction balance, state-space budget, and state-explosion triage. |
| `skills/slo-tla/references/methodology-counterexample.md` | Moved counterexample translation procedure and trace markdown shape. |
| `skills/slo-tla/references/methodology-verified-design.md` | Moved verified-design doc shape, refusal gates, anti-patterns, and handoff detail. |

## Apalache Source Verification

| Field | Captured value |
|---|---|
| Retrieval date | 2026-05-04 |
| Release page | `https://github.com/apalache-mc/apalache/releases/tag/v0.57.0` |
| Release tag | `v0.57.0` |
| Published at | 2026-04-24T13:58:09Z |
| Asset | `apalache.tgz` |
| Download URL | `https://github.com/apalache-mc/apalache/releases/download/v0.57.0/apalache.tgz` |
| Upstream checksum file value | `cb805df9a68e2f278c45e751522aab119b57a454e3e0e96f5d974b969fe52b5d` |
| Locally computed SHA-256 | `cb805df9a68e2f278c45e751522aab119b57a454e3e0e96f5d974b969fe52b5d` |

`tools.toml` keeps the existing TLC pin unchanged and updates `[apalache]` with
`version`, `url`, `download_url`, and `sha256`. The extra `url` key preserves
older structural tests while `download_url` satisfies the M3 contract.

## Compatibility Notes

- Existing `e2e_slo_sp_m5` tests still read `skills/slo-tla/SKILL.md` for JVM, checksum, bounds, fairness, Apalache, and cache-location sentinels. The dispatcher keeps those compactly.
- The suitability gate remains in `SKILL.md` because it decides whether the skill should run at all.
- The new M3 structural test enforces line cap, four methodology files, frontmatter, skill-local `references/` location, preserved TLA+ disciplines, TLC pin stability, and the Apalache SHA-256 pin.

## Evidence

| Check | Actual Result |
|---|---|
| Repo hygiene | Branch before edits: `slo/eng-imp-m3`; branch after edits: `slo/eng-imp-m3`; dirty tree before edits: clean; remediation needed: none. |
| Baseline before M3 edits | `cargo test --workspace` passed on branch `slo/eng-imp-m3` before decomposition edits. |
| Apalache release capture | GitHub release API and release page reported `v0.57.0`; upstream `sha256sum.txt` and local `shasum -a 256` matched for `apalache.tgz`. |
| Red-first M3 test | `cargo test -p sldo-install --test e2e_eng_imp_m3` failed for expected reasons: 333-line SKILL.md, missing methodology files, missing dispatch links, and missing `download_url` pin. |
| M3 structural test after implementation | `cargo test -p sldo-install --test e2e_eng_imp_m3` passed: 5 passed, 0 failed. |
| Legacy `/slo-tla` compatibility test | `cargo test -p sldo-install --test e2e_slo_sp_m5` passed: 11 passed, 0 failed. |
| Package test suite | `cargo test -p sldo-install` passed. |
| Workspace test suite | `cargo test --workspace` passed after implementation and closeout docs. |
| Workspace build | `cargo build --workspace` passed with pre-existing `sast-verify` warnings. |
| Formatting | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m3.rs` passed; `cargo fmt --check -p sldo-install` remains blocked by pre-existing unrelated drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| Diff hygiene | `git diff --check` passed. |
| Line count | `skills/slo-tla/SKILL.md` is 150 lines. |
| SAST authority docs untouched | `references/sast/` remained unchanged. |

## Rules For The Next Milestone

- In M4, apply the same thin-dispatcher pattern to `/slo-plan`, but preserve the interactive one-milestone-at-a-time authoring contract in a skill-local methodology file.
- Reuse the M2/M3 structural-test style: hard line cap, local reference files, dispatch links, and focused sentinel preservation.
- The soft line-cap test in M4 must account for already-decomposed `/slo-sast` and `/slo-tla` without requiring soft-cap exceptions.
- Scan existing `/slo-plan` tests before cutting; keep compatibility sentinels in the dispatcher where old tests expect them.

## Allow-List Note

The M3 Contract Block allowed the TLA skill, four skill-local methodology files,
`tools.toml`, and the structural test. The runbook Definition of Done also
requires tracker, lessons, completion, and an ARCHITECTURE post-flight note.
Those closeout/doc-index edits are recorded as milestone evidence rather than
product-surface changes.
