# Critique — Fowler AI Architecture SLO Improvements

## Summary

Reviewed `docs/slo/future/RUNBOOK-FOWLER-AI-ARCHITECTURE-SLO-IMPROVEMENTS.md` using the `/slo-critique` rotation: CEO, engineering lead, security, and design. Design is N/A because the runbook has no UI surface.

## Findings

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| fowler-crit-1 | eng | auto-fix | Prerequisite reading | Relative links from `docs/slo/future/` to root docs were one directory too shallow. | A future agent clicks `../ARCHITECTURE.md`, lands on missing `docs/slo/ARCHITECTURE.md`, and skips architecture context before execution. | Changed root-doc links to `../../ARCHITECTURE.md`, `../../LOOPS-ENGINEERING.md`, and `../../PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`. |
| fowler-crit-2 | eng | auto-fix | M1 invariants / BDD | M1 added two `/slo-architect` outputs but did not explicitly test that stale "Five files" wording is removed. | Agent adds `*-reversibility.md` and `*-code-map.md` below an unchanged "Five files" intro, creating contradictory skill prose. | Added an M1 structural-test requirement and BDD row for output-count wording. |
| fowler-crit-3 | eng | auto-fix | M2 file allow-list | M2 updated the docs mirror v4 template but omitted `skills/slo-plan/references/runbook-template_v_4_template.md`, which `/slo-plan` prefers first. | Installed `/slo-plan` reads the skill-local template and misses exemplar/refactoring rows even though the repo mirror has them. | Added the skill-local v4 template mirror to M2 files-to-read and files-allowed-to-change. |
| fowler-crit-4 | eng | auto-fix | M3 file allow-list | M3 had the same skill-local template gap for AI tolerance rows. | `/slo-verify` expects AI tolerance evidence, but `/slo-plan` emits from the stale skill-local template. | Added `skills/slo-plan/references/runbook-template_v_4_template.md` to M3 files-allowed-to-change. |
| fowler-crit-5 | security | defer | Threat model / sources | The runbook uses user-provided YouTube/Gemini notes as input; exact transcript claims remain unsourced. | A future contributor quotes a Gemini paraphrase as Fowler's exact claim and lands a false authority statement in SKILL.md. | Defer to execution guard already present: source-backed claims must cite research sources; user notes stay untrusted unless independently sourced. |
| fowler-crit-6 | design | defer | Whole runbook | N/A — no UI surface. | No user-facing screen, component, or interaction path is introduced by this plan. | No design pass required. |

## Disposition After Revision

- Applied `fowler-crit-1` through `fowler-crit-4` directly to the runbook.
- Left `fowler-crit-5` as an execution-time guard because the runbook already has the relevant global red line and source discipline.
- Skipped design pass as N/A.
