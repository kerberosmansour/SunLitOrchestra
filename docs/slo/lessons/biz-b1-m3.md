# Lessons — biz-b1 M3 (`/slo-product`)

## What changed

- NEW `skills/slo-product/SKILL.md` — first generator with mode arg (`roadmap | metrics | okrs`), three output paths.
- NEW `crates/sldo-install/tests/e2e_biz_b1_m3.rs` — 8 tests, all green.
- NEW `docs/slo/verify/biz-b1-m3-smoke.md` — 6 fixtures.
- ARCHITECTURE.md row added.

## Design decisions

- **Cross-skill disambiguation explicit in SKILL.md prose**. `/slo-product metrics` and `/slo-metrics` (B2) both touch metrics; the skill enumerates which KPIs go where AND redirects financial-KPI requests to `/slo-metrics`. Test enforces 3+ financial KPIs are named as not-here.
- **OKR cap at 3 objectives** mirrors `/slo-gtm`'s 3-segment cap. Soft limit with override; rejection prose enforces "nothing-is-priority" anti-pattern guidance.
- **Roadmap mode requires north-star metric defined first**: redirects to `mode_arg: metrics` if north-star missing. This forces the strategic frame before the tactical roadmap.

## Recommendations for M4 (`/slo-marketing`)

Final B1 milestone. Mode arg `b2b | b2c`. Test pattern: same as M3's three-mode test, scaled down to two modes. Plus: M4 ships the single CLAUDE.md catalog edit appending B1 generator rows.
