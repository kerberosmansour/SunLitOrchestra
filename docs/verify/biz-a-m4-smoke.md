# Manual smoke-test checklist — Runbook A M4 (`/slo-fundraise` + `references/biz/ir35-cest-factors.md` + CLAUDE.md catalog)

> Created: 2026-04-25
> Skill: [skills/slo-fundraise/SKILL.md](../../skills/slo-fundraise/SKILL.md)

## Prerequisites

- [ ] M1 + M2 + M3 smoke tests passed.
- [ ] All four structural-contract test suites green: `cargo test -p sldo-install --test e2e_biz_a_m1 --test e2e_biz_a_m2 --test e2e_biz_a_m3 --test e2e_biz_a_m4`.
- [ ] `sldo-install --dry-run` shows 18 skills (14 pre-runbook + slo-legal + slo-accounting + slo-equity + slo-fundraise).
- [ ] CLAUDE.md catalogs all four advisor skills with their domain + cited references.

## Smoke fixtures

1. **AA-not-yet hard-block** — `/slo-fundraise draft safe-worksheet` for a £200k seed at £2M cap, founder confirms "we haven't applied for SEIS Advance Assurance yet". Verify: skill HARD-BLOCKS `draft safe-worksheet`; routes to triage; output explicitly recommends apply-for-AA-now-then-come-back; cites `hmrc-vcm-index.md` ≥ 6 weeks lead time.
2. **AA-applied draft** — `/slo-fundraise draft safe-worksheet` for the same round, founder confirms "AA applied 8 weeks ago, AA received". Verify: skill produces the SAFE math worksheet at `docs/biz/fundraise/safe-worksheet-...md`; `tier: confidential`; "LAWYER + ACCOUNTANT REVIEW RECOMMENDED" header; pre / post-money math + dilution table; ROI block citing JPP Law SHA-SEIS line.
3. **Pitch narrative** — `/slo-fundraise draft pitch-narrative` for a fictional seed-stage SaaS. Verify: artifact at `docs/biz/fundraise/pitch-narrative-...md`; structured prose covering problem / solution / wedge / traction / team / ask / SEIS-EIS investor-tax-relief mention.
4. **Term-sheet redline brief** — `/slo-fundraise draft term-sheet-redline-brief` with founder pasting an investor's term-sheet text. Verify: skill produces brief at `docs/biz/fundraise/term-sheet-redline-brief-...md`; gate-3 fires (counterparty has lawyer); routes to lawyer for redline; brief structured as standard / negotiable / red-flag analysis.
5. **Translate term sheet** — `/slo-fundraise translate <pasted-term-sheet>`. Verify: plain-English summary at `docs/biz-public/fundraise/translate-...md` (override to confidential if pasted text contains real PII / deal terms).
6. **Preferential-rights flag in cap-table** — `/slo-fundraise triage "investor wants 1.5x liquidation preference"`. Verify: skill flags Abingdon Health line; routes to lawyer + accountant; warns about SEIS / EIS qualification risk on preferential rights.
7. **Non-UK** — `/slo-fundraise draft safe-worksheet --jurisdiction us`. Verify: canonical UK-only error.

## Cross-skill consistency

- [ ] Run `/slo-equity` (M3) and `/slo-fundraise` (M4) on overlapping fact patterns (e.g., "we're raising £200k SEIS round, cofounder split is 70/30, vesting starts now"). Both skills should fire the same predicate IDs; both should cite the same VCM34080 / VCM31000 references; both should flag the preferential-rights / Abingdon Health line if any preferential rights are mentioned.
- [ ] No drift in predicate-id wording or HMRC manual paragraph references between the two outputs.

## Final pack-level verification

- [ ] CLAUDE.md "Biz skill pack" section accurately catalogs all four advisor skills with correct domain summaries.
- [ ] `references/biz/` contains 8 files: triage-gate, cost-baseline-jpp-law-2026, artifact-schema, jurisdiction-uk, ico-duaa-index, ico-enforcement-reality, open-template-anchors, hmrc-vcm-index, ir35-cest-factors, plus templates/onenda-uk.md = 9 markdown files + 1 subdirectory.
- [ ] `skills/slo-legal/`, `skills/slo-accounting/`, `skills/slo-equity/`, `skills/slo-fundraise/` each contain exactly one `SKILL.md`.

## Notes

- _<empty until smoke run>_
