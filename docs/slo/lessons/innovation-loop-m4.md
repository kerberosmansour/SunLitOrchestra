# Lessons Learned — innovation-loop Milestone 4

## What changed
- Shipped the only code phase: `skills/slo-spike/SKILL.md` (§7 evidence mode — bounded proof artifacts under `experiments/<slug>/<spike-id>/`, mandatory budget, evidence-derived verdict, delete-or-promote, no production promotion). Test `innovation_loop_m4_spike.rs` (6 assertions). Catalog 46→47.

## Design decisions and why
- **The skill's "Hard rules" block carries the AI-tolerance headline** (critique M4): scratch-only confinement, no-production-promotion, mandatory budget + stop-at-limit, synthetic-data-default, and verdict-from-evidence-not-narration. The test asserts each as a presence check (`tm-...-abuse-2/-3/-5`).
- **Scratch root reuses the M1-anchored `/experiments/`** (.gitignore) — verified spike scratch is ignored, Books still tracked. No second scratch location (critique E3).

## Assumptions verified
- `git check-ignore experiments/<slug>/<spike-id>` returns the path (scratch ignored); discover_skills() lists spike; both count tests re-pointed 46→47.

## Rules for the next milestone
- M5 closes the loop: `/slo-curate` (exactly one disposition per candidate) + `/slo-demo` (5 promotion-seed tables, suggestion-only, no auto-invoke) + the synthetic example Book + LOOPS diagram completion. Keep re-pointing both count tests (47→49).

## Invariants/assertions added
- `/slo-spike` evidence mode + mandatory budget + delete-or-promote + scratch confinement (`experiments/<slug>/`) + no-production-promotion + evidence-derived verdict + §7 target; output-path safety.
