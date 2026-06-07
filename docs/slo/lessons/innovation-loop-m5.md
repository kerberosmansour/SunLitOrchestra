# Lessons Learned — innovation-loop Milestone 5

## What changed
- Closed the loop: `skills/slo-curate/SKILL.md` (§8 convergent — exactly one disposition per candidate, cite evidence, Curation Definition-of-Learned) + `skills/slo-demo/SKILL.md` (§9/§10 communication — typed promotion handoff, 4 destinations + next-artifact paths, suggestion-only, no auto-invoke). Synthetic gallery example Book at `docs/slo/experiments/example-context-validator/EXPERIMENT.md` (closes `promote_to_idea`, PII-clean). Test `innovation_loop_m5_close.rs` (5 assertions). Catalog 47→49 (all 8 skills shipped). ARCHITECTURE + LOOPS de-flagged from "planned" → shipped.

## Design decisions and why
- **Promotion is a suggestion, never an auto-invoke (tm-...-abuse-6).** `/slo-demo` fills the matching seed table and names the next skill, but the human runs it. The test asserts "suggestion"/"never auto-invoke" language.
- **The example Book is the leading-metric proof.** It demonstrates an experiment reaching a terminal exit state end-to-end — the §5A leading metric ("≥1 Experiment Book reaches a terminal exit state"). It is synthetic (no real PII), so it doubles as the gallery calibration artifact AND passes the M1 + M5 PII scans.

## Assumptions verified
- The example Book passes both the M5 exit-state/PII test and the M1 secret/PII scan over `docs/slo/experiments/`.
- discover_skills() lists all 8 innovation-loop skills; both count tests at 49; catalog reconciles.

## Rules for the next milestone
- N/A — final milestone. The loop is usable end-to-end. Follow-ups: file DW-001 (clippy debt) at retro; optional README one-line pointer; optional CI grep guarding the `"Shipped skills at HEAD: N"` literal (would have auto-caught DW-002).

## Invariants/assertions added
- `/slo-curate` convergent + exactly-one-disposition + cite-evidence; `/slo-demo` communication + 4 frozen destinations + next-artifact paths + suggestion-only; example Book closes with exactly one of 8 exit states + PII-clean; both target §8/§9/§10.
