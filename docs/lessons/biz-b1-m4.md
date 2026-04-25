# Lessons — biz-b1 M4 (`/slo-marketing` + CLAUDE.md catalog)

## What changed

- NEW `skills/slo-marketing/SKILL.md` — generator with binary mode arg `b2b | b2c`.
- NEW `crates/sldo-install/tests/e2e_biz_b1_m4.rs` — 9 tests including CLAUDE.md catalog completeness.
- NEW `docs/verify/biz-b1-m4-smoke.md` — 6 fixtures.
- EDITED `CLAUDE.md` — bundled catalog edit appending all 4 B1 generators + PII discipline note.
- ARCHITECTURE.md row added.

## Design decisions

- **Direct-marketing PECR routing is mandatory** (gate-4-gdpr-document fires on B2B + B2C). The skill OUTPUTS the marketing plan but flags any direct-marketing channel as "BLOCKED until /slo-legal triage resolves" — the founder cannot launch the channel without resolving the gate.
- **B2C mode flags ASA / CAP Code + CRA 2015** as additional UK regulator routings beyond PECR. Subscription / digital-content sales channels route through CRA 2015 (14-day cooling-off).
- **Growth-hacking is REFUSED outright** when it bypasses consent (LinkedIn scraping, email-list buying). Skill emits explicit DUAA PECR ceiling reminder (£17.5M / 4% global turnover).

## Recommendations for Runbook B2

B2 ships 4 more generators: `/slo-launch` (one-shot launch sequence), `/slo-sales-funnel` (cold-email + funnel math), `/slo-pricing` (value equation + tier model), `/slo-metrics` (financial KPIs). Pattern from B1 transfers; B2 generators are similarly compact. `/slo-metrics` will be the most complex due to mode arg + cross-skill coupling with `/slo-product metrics`.
