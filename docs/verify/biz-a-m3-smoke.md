# Manual smoke-test checklist — Runbook A M3 (`/slo-equity` + `references/biz/hmrc-vcm-index.md`)

> Created: 2026-04-25
> Skill: [skills/slo-equity/SKILL.md](../../skills/slo-equity/SKILL.md)

## Prerequisites

- [ ] M1 + M2 smoke tests passed and recorded.
- [ ] All structural tests green: `cargo test -p sldo-install --test e2e_biz_a_m1 --test e2e_biz_a_m2 --test e2e_biz_a_m3`.
- [ ] `sldo-install --dry-run` shows 17 skills (14 pre-runbook + slo-legal + slo-accounting + slo-equity).

## Smoke fixtures

1. **Cofounder split rationale** — `/slo-equity draft cofounder-split-rationale` for two founders (technical + commercial) contributing differently. Verify: artifact at `docs/biz/equity/cofounder-split-rationale-...md`; `tier: confidential`; "LAWYER + ACCOUNTANT REVIEW RECOMMENDED" header; SEIS pre-check questions surfaced (VCM34080, Abingdon Health-line, qualifying-trade); body recommends ordinary-only shares for founders.
2. **SEIS Advance Assurance triage** — `/slo-equity triage "should I file SEIS Advance Assurance now?"`. Verify: artifact at `docs/biz-public/equity/triage-...md`; routes to accountant; cites VCM31000; recommends ≥ 6 weeks lead time before any term-sheet signature.
3. **EMI option triage** — `/slo-equity triage "I want to grant EMI options to my first 3 employees"`. Verify: routes to accountant + lawyer (EMI eligibility is HMRC-regulated AND the option grant is solicitor-drafted); flags valuation requirement (HMRC valuation agreement before grant).
4. **Preferential-rights ambiguity** — `/slo-equity draft cofounder-split-rationale` where founder mentions "I want my shares to have 2x voting rights". Verify: skill HARD-BLOCKS draft; cites Abingdon Health case; routes to lawyer for drafting review.
5. **Non-UK** — `/slo-equity draft cap-table-snapshot --jurisdiction us`. Verify: canonical "v1 supports UK only" error.

## Notes

- _<empty until smoke run>_
