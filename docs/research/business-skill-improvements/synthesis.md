---
name: business-skill-improvements
researched: 2026-04-27
incomplete: false
note: |
  Skill-pack improvement runbook — primary research input is the 2026-04-27 skill-pack
  review + issues #19 and #20 + the existing biz-pack reference files (triage-gate,
  artifact-schema, hmrc-vcm-index, ico-duaa-index, ir35-cest-factors, jurisdiction-uk,
  cost-baseline-jpp-law-2026). This file is the synthesis.
---

# Synthesis — business skill improvements

## What the design must handle (and why)

### 1. Predicate evaluation is the LLM's responsibility — and that is the failure surface

The design must handle this because [`references/biz/triage-gate.md:16`](../../../references/biz/triage-gate.md) explicitly says: *"Predicate evaluation is the LLM's responsibility — these are natural-language tests..."* The five-skill cluster (legal/accounting/equity/fundraise/hire) all rely on this pattern. Closing the surface = (a) structured intake before the LLM evaluates, and (b) refusal-on-ambiguity as the third state of gate evaluation.

### 2. Conversational discipline is the UX, not a form

The design must handle conversational elicitation because the project owner's explicit framing: *"this is a chatbot. So it isn't gonna process, like, hiring people. But giving someone advice."* The structured `intake_summary:` block is the **output of the conversation**, not a UI for the founder. Pattern source: [`/slo-ideate`'s seven forcing questions](../../../skills/slo-ideate/SKILL.md) — *"Ask one at a time. Do not accept hypotheticals. Push back on vague answers."* Same shape, different domain.

### 3. UK statute citations must come from `legislation.gov.uk` at retrieval-stamped dates

The design must handle this because the project owner's research-validation discipline applies: *"the references of the technical knowledge is valid and the correct one. Especially on security engineering recommendations."* For UK statute, the analogous primary source is `legislation.gov.uk` (the official statute repository) — not paraphrased commentary. `legislation.gov.uk` has stable section URLs (e.g., https://www.legislation.gov.uk/ukpga/1996/18/section/86 for ERA 1996 s86) that can be cited verbatim.

### 4. HMRC manual content must be quoted, not paraphrased

The design must handle this because [`references/biz/hmrc-vcm-index.md`](../../../references/biz/hmrc-vcm-index.md) currently paraphrases VCM34080, VCM3000, VCM31000, and the Abingdon Health Limited v HMRC line. The HMRC Venture Capital Schemes Manual is a public document at `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` — verbatim quotes with paragraph IDs are the correct citation form. Paraphrasing is the failure mode this runbook closes.

### 5. Closed-enum regulator list must replace open-ended LLM enumeration

The design must handle this because `gate-1-regulated` currently enumerates from training memory ("FCA, MHRA, ICO, healthcare, financial services, or any other regulator with statutory enforcement powers"). The new [`references/biz/uk-regulator-enumeration.md`](../../../references/biz/uk-regulator-enumeration.md) is a closed enum — naming a regulator NOT in the enum is a refusal pattern (either probe further or flag the enum gap as a follow-up). 29-regulator starter list seeded; runbook M1 source-verifies every row's `statutory_basis:` against `legislation.gov.uk`.

### 6. Numeric arithmetic in advisor outputs must be re-derived before write

The design must handle this because [`/slo-fundraise`'s SAFE worksheet and `/slo-equity`'s cap-table snapshot](../../../skills/slo-fundraise/SKILL.md) currently rely on LLM-computed cells. The fix: emit math as a runnable Python snippet OR a verification pass that re-derives every cell. A 5-percentage-point dilution-table error that goes to investors is the failure mode this milestone closes.

### 7. Heuristic numbers need source attribution AND refresh cadence

The design must handle both for the seven biz generator skills (`/slo-metrics`, `/slo-pricing`, `/slo-sales-funnel`, `/slo-product`, `/slo-launch`, `/slo-marketing`, `/slo-talk-to-users`). Sources verified at the file-creation date; per-row `last_checked:` field; SKILL.md emits a stale-baseline warning at +12 months (matches the existing cost-baseline staleness pattern in [`/slo-legal`](../../../skills/slo-legal/SKILL.md)).

### 8. `baseline_ref:` field must be added to the artifact schema

The design must handle this because every artifact emitted by a generator skill that consults a baseline file MUST carry the `baseline_ref:` provenance pointer (matches the existing `cost_baseline_ref:` pattern in advisor outputs). Adding to [`references/biz/artifact-schema.md`](../../../references/biz/artifact-schema.md) is M5 of this runbook.

### 9. Sister intake contracts must share F1 / F4 / F5 fields verbatim

The design must handle this because cross-skill drift on jurisdiction, GDPR scope, and regulator scope evaluation would defeat the consistency the four-gate predicates rely on. The legal-intake-contract.md is the canonical shape; accounting / equity / fundraise / hire extend it without redefining F1 / F4 / F5.

### 10. R3 depends on R2's `references/templates/intake-checklist.md` landing first

The design must handle the dependency by ordering R2 → R3. If R3 starts before R2's M1, R3's M2 (intake contracts) has to inline-author the conversational discipline pattern that R2 will later generalize. Avoidable rework.

## Open questions that research did not answer

1. **Verbatim text of HMRC VCM paragraphs** — must be fetched from `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` at runbook-author time. Each quoted paragraph carries `last_checked:` date.
2. **DUAA 2025 Stage 3 commencement details** — official: `https://www.legislation.gov.uk/ukpga/2025/18`; ICO summary: `https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/`. Refresh both at M1 of this runbook.
3. **Public KPI sources to source-verify before M4**:
   - Bessemer State of the Cloud (https://www.bvp.com/atlas) — burn multiple framework
   - OpenView SaaS Benchmarks (https://openviewpartners.com/saas-benchmarks/) — CAC payback / NDR / gross margin
   - Sequoia Retention by the Numbers (https://articles.sequoiacap.com/retention) + Andrew Chen retention curves (https://andrewchen.com/retention-is-king/)
   - Bain NPS framework (https://www.netpromotersystem.com/about/)
   - Bridge Group SaaS Sales Development Report (https://blog.bridgegroupinc.com/sdr-metrics-and-compensation-report)
   - RAIN Group sales conversion benchmarks (https://www.rainsalestraining.com/blog/sales-statistics)
   - Paul Graham *Startup = Growth* (https://paulgraham.com/growth.html)
   - Fitzpatrick *The Mom Test* (2013, ISBN 978-1492180746) — quote-permitted snippets only
4. **JPP Law fixed-fee public pricing as of 2026-04** — already captured at [`references/biz/cost-baseline-jpp-law-2026.md`](../../../references/biz/cost-baseline-jpp-law-2026.md) with `retrieved: 2026-04-25`. M5 of this runbook sets up the `/loop @annually` agent that re-fetches public sources and opens a refresh PR.
5. **AA lead-time "4-6 weeks realistic" claim in `/slo-fundraise`** — verify against HMRC VCM index public commentary at runbook-author time. Currently recited; needs source attribution.

## Source pointers

- 2026-04-27 skill-pack review (in-conversation artifact)
- [Issue #19](https://github.com/kerberosmansour/SunLitOrchestrate/issues/19) — advisor predicate hardening + conversational intake discipline
- [Issue #20](https://github.com/kerberosmansour/SunLitOrchestrate/issues/20) — KPI baseline + heuristic-number authority files
- [`references/biz/triage-gate.md`](../../../references/biz/triage-gate.md) — the four hard-block predicates
- [`references/biz/artifact-schema.md`](../../../references/biz/artifact-schema.md) — frontmatter contract
- [`references/biz/jurisdiction-uk.md`](../../../references/biz/jurisdiction-uk.md), [`references/biz/uk-regulator-enumeration.md`](../../../references/biz/uk-regulator-enumeration.md) (starter)
- [`references/biz/legal-intake-form.md`](../../../references/biz/legal-intake-form.md) (starter; rename to `legal-intake-contract.md` in M2)
- [`references/biz/saas-kpi-targets-baseline.md`](../../../references/biz/saas-kpi-targets-baseline.md) (starter)
- [`references/biz/hmrc-vcm-index.md`](../../../references/biz/hmrc-vcm-index.md), [`references/biz/ico-duaa-index.md`](../../../references/biz/ico-duaa-index.md), [`references/biz/cost-baseline-jpp-law-2026.md`](../../../references/biz/cost-baseline-jpp-law-2026.md), [`references/biz/ir35-cest-factors.md`](../../../references/biz/ir35-cest-factors.md)

## Note on chub / get-api-docs

Not applicable for this runbook — biz pack is `WebFetch`/`WebSearch` denied per [`SECURITY.md`](../../../SECURITY.md) and per the threat-model row `tm-biz-abuse-1`. External regulatory anchors (legislation.gov.uk, ICO, HMRC, JPP Law) are emitted as **citations** the founder follows manually, never fetched at runtime. Source-verification by the runbook author happens at runbook-author time only — captured as `last_checked:` dates in the reference files.
