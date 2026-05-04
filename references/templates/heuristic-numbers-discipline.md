---
name: heuristic-numbers-discipline
status: stable-template
created: 2026-05-04
audience: skills that use benchmarks, thresholds, prices, conversion rates, time estimates, or other numeric heuristics
purpose: Shared provenance rule for numeric claims.
---

# Heuristic Numbers Discipline Template

Numeric heuristics go stale quickly. This template cites `references/templates/citation-discipline.md` because every number needs a source tier and retrieval date.

## Rule

Every numeric claim cites a baseline file with a retrieved date. Inline numbers in SKILL.md are allowed only when they are constants owned by the skill contract, such as "ask one question at a time" or "cap milestones at five."

## Baseline Row Fields

- `claim:` the number or threshold.
- `source_url:` or `source_file:`.
- `retrieved:` `YYYY-MM-DD`.
- `last_checked:` `YYYY-MM-DD`.
- `confidence:` `high`, `medium`, or `low`.
- `methodology_note:` how the source measured or defined the number.
- `applicability_caveat:` when the number applies only to a market, jurisdiction, stack, or time period.

## Stale Warning

If `last_checked` is more than 12 months old, the skill must warn before relying on the number. If the user asks for a decision more than 24 months beyond the checked date, route to fresh research or a human source lookup.
