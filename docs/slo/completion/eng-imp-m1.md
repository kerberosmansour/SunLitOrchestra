# Completion Summary - eng-imp Milestone 1

**Status**: `done` (2026-05-04)
**Goal**: Seed the shared engineering `references/templates/` library, lock citation/source hierarchy discipline, wire five security-engineering-facing skills to cite the templates, and document the source-verification spike.
**Runbook**: [docs/slo/future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md](../future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md) Milestone 1.

## What Shipped

| Path | Purpose |
|---|---|
| `references/templates/citation-discipline.md` | Locked six-tier source hierarchy and remove-not-weaken rule. |
| `references/templates/intake-checklist.md` | Shared conversational intake pattern. |
| `references/templates/restate-and-confirm.md` | Shared correction-loop pattern. |
| `references/templates/tool-safety-section.md` | Shared tool version/help/dry-run/stdout-stderr-exit-code discipline. |
| `references/templates/output-frontmatter.md` | Shared artifact provenance frontmatter base. |
| `references/templates/escalation.md` | Shared refusal, routing, and false-positive triage shape. |
| `references/templates/eval-cases.md` | Shared eval-case frontmatter and body shape. |
| `references/templates/heuristic-numbers-discipline.md` | Shared numeric-claim provenance rule. |
| `references/templates/rate-limiting-discipline.md` | Shared remote-call cap/backoff/spillover rule. |
| `references/templates/fallback-discipline.md` | Shared graceful-degradation evidence row. |
| `references/templates/version-pinning-discipline.md` | Shared SHA/checksum/cache-integrity discipline. |
| `skills/slo-sast/SKILL.md` | Cites shared citation, tool-safety, and version-pinning templates. |
| `skills/slo-tla/SKILL.md` | Cites shared citation, tool-safety, and version-pinning templates. |
| `skills/slo-rulegen/SKILL.md` | Cites shared citation, tool-safety, and version-pinning templates. |
| `skills/slo-verify/SKILL.md` | Cites shared citation/tool/escalation templates and documents Pass 4 capitalised-bigram false-positive triage. |
| `skills/slo-research/SKILL.md` | Cites shared citation/tool/fallback templates and requires `sldo-research --help` capture before optional batch dispatch. |
| `crates/sldo-install/tests/e2e_eng_imp_m1.rs` | M1 structural-contract tests. |
| `docs/slo/lessons/eng-imp-m1.md` | Source-verification spike and M2 carry-forward rules. |
| `README.md`, `docs/ARCHITECTURE.md` | Post-flight docs pointers to `references/templates/`. |

## Validation

- `cargo test --workspace`: passed as the baseline before M1 edits.
- `cargo test -p sldo-install --test e2e_eng_imp_m1`: failed first for missing templates/citations/lessons, then passed 6/6 after implementation.

## Compatibility

- Existing SKILL.md paths and frontmatter names are preserved.
- `references/biz/` and `references/sast/` authority files were not modified.
- No new dependencies, schema migrations, or runtime skill behavior were added.
- The M1 test asserts all 11 shared templates have frontmatter and that the five target skills cite `references/templates/`.

## Allow-List Note

The runbook M1 Contract Block listed the templates, five SKILL.md files, and structural test. Its Definition of Done and Post-Flight also required tracker, lessons, completion, README, and ARCHITECTURE updates. Those closeout/doc-index edits were made as milestone evidence and documented in `docs/slo/lessons/eng-imp-m1.md`.

## Next Milestone

M2: decompose `/slo-sast` into a thin SKILL.md plus `skills/slo-sast/references/methodology-m1..m5.md`, using `references/templates/citation-discipline.md` as the source hierarchy authority.
