# Lessons — biz-b2 M4 (`/slo-metrics` + Runbook B2 closure)

## What changed
- NEW skill (~140 lines, mode arg `consumer | b2b`), 12 tests including cross-skill financial-KPI citation + CLAUDE.md catalog completeness.
- CLAUDE.md catalog edit appending all 4 B2 generator rows.
- ARCHITECTURE.md rows for all 4 B2 generators batched.

## Design decisions
- Disambiguation from `/slo-product metrics` (B1 M3): financial here, PM there. Test enforces both directions.
- Cross-skill financial-KPI citation test scoped to `/slo-pricing` + `/slo-metrics` only — `/slo-fundraise` carries fundraise-process surface, NOT KPI-dashboard surface (initial test scope was over-aggressive; one iteration to scope correctly).
- B2B target NDR ≥ 110%; consumer target ≥ 15% MoM; burn multiple ≤ 2 (Bessemer Cloud Index convention).

## Recommendations for Runbook C
3 team skills: cofounder, hire, founder-check. cofounder + hire are confidential-tier (real persons). founder-check is self-assessment confidential. Pattern from B1 M1 PII discipline applies.
