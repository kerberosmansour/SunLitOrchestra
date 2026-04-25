# Smoke — biz-b1 M3 (`/slo-product`)

1. **Roadmap mode** — `/slo-product roadmap` for fixture product. Verify: artifact at `docs/biz-public/product/roadmap.md`; 90-day RICE/Kano table; "what we're NOT doing" list; redirects to `metrics` mode if north-star not defined.
2. **Metrics mode** — `/slo-product metrics`. Verify: artifact at `docs/biz-public/product/metrics.md`; ONE north-star metric (not multiple); activation funnel; retention cohorts; cross-reference to `/slo-metrics` for financial KPIs.
3. **OKRs mode** — `/slo-product okrs`. Verify: artifact at `docs/biz-public/product/okrs.md`; 3 objectives MAX; KRs measurable with baseline + target; KR-to-north-star ladder explicit.
4. **Unknown mode_arg** — `/slo-product foo`. Verify: rejection with valid options listed.
5. **Financial-KPI redirect** — `/slo-product metrics` where founder asks about CAC. Verify: redirect to `/slo-metrics`, no artifact written for the financial part.
6. **Non-UK** — canonical error.
