# Business Skill Improvements — SunLitOrchestrate (AI-First Runbook v3)

> **Purpose**: Close the predicate-evaluation hallucination surface across the five advisor skills via conversational intake contracts; move every UK statute / regulator citation from inlined SKILL.md prose to authority files; verify SAFE / cap-table / pricing arithmetic before emission; centralize numeric heuristics for the seven biz generators with source attribution.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. R3 depends on R2 M1 (`references/templates/`) landing first.
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [docs/slo/design/business-skill-improvements-overview.md](design/business-skill-improvements-overview.md), [docs/slo/idea/business-skill-improvements.md](idea/business-skill-improvements.md), [docs/slo/research/business-skill-improvements/synthesis.md](research/business-skill-improvements/synthesis.md), [Issue #19](https://github.com/kerberosmansour/SunLitOrchestrate/issues/19), [Issue #20](https://github.com/kerberosmansour/SunLitOrchestrate/issues/20), 2026-04-27 skill-pack review

---

## Runbook Metadata

- **Runbook ID**: `business-skill-improvements`
- **Prefix for test files and lessons files**: `biz-imp`
- **Primary stack**: Markdown + Rust (`crates/sldo-install`) + `WebFetch`/`WebSearch` denied (per `tm-biz-abuse-1`)
- **Primary package/app names**: `sldo-install`, advisor skills (`slo-legal`, `slo-accounting`, `slo-equity`, `slo-fundraise`, `slo-hire`), generator skills (`slo-metrics`, `slo-pricing`, `slo-sales-funnel`, `slo-product`, `slo-launch`, `slo-marketing`, `slo-talk-to-users`), `references/biz/`
- **Default test commands**:
  - Workspace tests: `cargo test --workspace`
  - Specific install tests: `cargo test -p sldo-install`
  - Build: `cargo build --workspace`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable**:
  - The four hard-block predicate IDs in `references/biz/triage-gate.md` (immutable).
  - Existing `references/biz/artifact-schema.md` frontmatter contract (additive only — adding `intake_summary:`, `gates_evaluation:`, `baseline_ref:` is non-breaking).
  - Existing `docs/biz/` / `docs/biz-public/` two-tier convention.
  - Every existing advisor + generator SKILL.md install path.
  - Existing `WebFetch`/`WebSearch` denial at the SLO-CLI invocation layer.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | UK regulator enumeration source-verified + statute anchor files | `not_started` | | | | |
| 2 | Five conversational intake contracts + advisor SKILL.md updates (rename `*-form.md` → `*-contract.md`) | `not_started` | | | | |
| 3 | Numeric verification (SAFE / cap-table / pricing math) | `not_started` | | | | |
| 4 | KPI baseline files + generator skill updates | `not_started` | | | | |
| 5 | `baseline_ref:` artifact schema field + cross-skill citation tests + annual refresh `/loop` | `not_started` | | | | |

---

## End-to-End Architecture Diagram

```
┌────────────────────────────────────────────────────────────────────────┐
│                  SunLitOrchestrate biz pack (UK only)                  │
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────┐     │
│  │  references/biz/  (existing + new)                            │     │
│  │   triage-gate.md (immutable)                                  │     │
│  │   artifact-schema.md (extended M5: + baseline_ref)            │     │
│  │   jurisdiction-uk.md (existing)                               │     │
│  │   uk-regulator-enumeration.md (M1 — closed enum, 29+ rows)    │     │
│  │   uk-employment-statute-anchors.md (M1 — IANA / ERA / etc.)   │     │
│  │   uk-consumer-statute-anchors.md (M1 — CRA / CCR / DMCC)      │     │
│  │   uk-marketing-statute-anchors.md (M1 — ASA / CAP / PECR)     │     │
│  │   hmrc-vcm-index.md (M1 — verbatim quote refresh)             │     │
│  │   ico-duaa-index.md (M1 — refresh)                            │     │
│  │   cost-baseline-jpp-law-2026.md (existing)                    │     │
│  │   ir35-cest-factors.md (existing)                             │     │
│  │   {legal,accounting,equity,fundraise,hire}-intake-            │     │
│  │     contract.md (M2 — conversational intake)                  │     │
│  │   saas-kpi-targets-baseline.md (M4 — refresh starter)         │     │
│  │   outbound-conversion-baselines.md (M4)                       │     │
│  │   product-prioritization-frameworks.md (M4)                   │     │
│  │   value-equation-pricing.md (M4)                              │     │
│  │   mom-test-canonical-questions.md (M4)                        │     │
│  │   launch-success-thresholds.md (M4)                           │     │
│  └──────────────────────────────────────────────────────────────┘     │
│                              ▲           ▲                             │
│           ┌──────────────────┼───────────┴──────────┐                  │
│           │                  │                      │                   │
│  ┌────────┴────────┐ ┌───────┴────────┐ ┌──────────┴───────┐          │
│  │ Advisor cluster │ │ Generator      │ │ /slo-talk-to-users│          │
│  │ (5 skills)      │ │ cluster         │ │ + /slo-launch    │          │
│  │                 │ │ (7 skills)      │ │                  │          │
│  │ - SKILL.md      │ │ - SKILL.md      │ │ - SKILL.md       │          │
│  │   updated M2    │ │   updated M4    │ │   updated M4     │          │
│  │ - intake        │ │ - cite KPI      │ │ - cite Mom Test  │          │
│  │   contract      │ │   baselines     │ │   + launch       │          │
│  │   conversational│ │   via baseline_ │ │   thresholds     │          │
│  │   F1-F6         │ │   ref           │ │                  │          │
│  └─────────────────┘ └─────────────────┘ └──────────────────┘          │
│           │                                                              │
│           │ at draft-time:                                               │
│           ▼                                                              │
│  ┌─────────────────────────────────────────┐                            │
│  │ Numeric verification pass (M3)          │                            │
│  │  /slo-fundraise SAFE math               │                            │
│  │  /slo-equity cap-table totals           │                            │
│  │  /slo-pricing value-equation            │                            │
│  └─────────────────────────────────────────┘                            │
│                                                                          │
│  ┌──────────────────────────────────────────┐                           │
│  │ M5 cross-skill citation structural       │                           │
│  │ contract test:                           │                           │
│  │  - every drafted artifact has            │                           │
│  │    intake_summary + gates_evaluation     │                           │
│  │  - every generated artifact citing       │                           │
│  │    numbers has baseline_ref              │                           │
│  │  - immutability of 4 predicates          │                           │
│  └──────────────────────────────────────────┘                           │
│                                                                          │
│  Legend:  existing  /  NEW (this runbook)  /  ▶ data flow              │
└────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone | Key Interfaces |
|---|---|---|---|
| `references/biz/uk-regulator-enumeration.md` | Closed enum of UK regulators (source-verified `statutory_basis:` per row) | M1 | `gate-1-regulated` evaluation in every advisor skill |
| `references/biz/uk-employment-statute-anchors.md` | Verbatim quotes of IANA 2006, ERA 1996, Pensions Act 2008, Equality Act 2010, IR35 (ITEPA 2003 Ch 10) | M1 | `/slo-hire`, `/slo-legal` |
| `references/biz/uk-consumer-statute-anchors.md` | CRA 2015, Consumer Contracts Regulations 2013, DMCC 2024 verbatim | M1 | `/slo-legal`, `/slo-marketing`, `/slo-pricing` |
| `references/biz/uk-marketing-statute-anchors.md` | ASA + CAP Code (12th ed), PECR, DUAA 2025 verbatim | M1 | `/slo-marketing`, `/slo-launch`, `/slo-sales-funnel` |
| `references/biz/hmrc-vcm-index.md` (refresh) | VCM34080 / VCM3000 / VCM31000 / Abingdon Health verbatim | M1 | `/slo-equity`, `/slo-fundraise` |
| `references/biz/{legal,accounting,equity,fundraise,hire}-intake-contract.md` | Conversational intake contracts | M2 | All 5 advisor skills |
| Five advisor SKILL.md updates | Mandate intake contract, restate-and-confirm, refusal-on-ambiguity, citation discipline | M2 | SKILL.md prose; behavior change is intake-only |
| Numeric verification | SAFE math, cap-table totals, pricing value-equation re-derived | M3 | `/slo-fundraise`, `/slo-equity`, `/slo-pricing` |
| `references/biz/saas-kpi-targets-baseline.md` (source-verified) | KPI bands with per-row `last_checked:` + source URL | M4 | `/slo-metrics` (primary) |
| `references/biz/outbound-conversion-baselines.md` | Funnel-stage rates with sources | M4 | `/slo-sales-funnel`, `/slo-metrics` |
| `references/biz/product-prioritization-frameworks.md` | RICE, Kano definitions sourced | M4 | `/slo-product` |
| `references/biz/value-equation-pricing.md` | "25-33% of value" claim sourced | M4 | `/slo-pricing` |
| `references/biz/mom-test-canonical-questions.md` | Fitzpatrick *Mom Test* 2013 question scaffolding | M4 | `/slo-talk-to-users`, `/slo-product`, `/slo-launch` |
| `references/biz/launch-success-thresholds.md` | "set your own threshold" framing for unsourceable launch numbers | M4 | `/slo-launch` |
| `references/biz/artifact-schema.md` (refresh) | Adds `baseline_ref:`, `intake_summary:`, `gates_evaluation:` fields | M5 | Read by `/slo-verify` Pass 4 schema check |
| Cross-skill citation structural-contract tests | Every advisor SKILL.md references the regulator enum + intake contract; every generator SKILL.md references its baseline file | M5 | `cargo test --workspace` |
| Annual refresh `/loop` agent | Re-fetches public sources, opens refresh PR | M5 (configured) | `/schedule` skill |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Statute citation consultation | Advisor SKILL.md prose | `references/biz/uk-*-statute-anchors.md` | Markdown link, runtime-read | M1 |
| Conversational intake → structured `intake_summary:` | Founder dialogue | Artifact frontmatter | LLM synthesis | M2 |
| Gate evaluation against structured intake | Advisor skill | `references/biz/triage-gate.md` + `uk-regulator-enumeration.md` | LLM evaluation against closed-enum | M2 |
| Numeric re-derivation | Advisor skill | Artifact body | Python snippet OR re-derive pass | M3 |
| KPI baseline consultation | Generator SKILL.md | `references/biz/saas-kpi-targets-baseline.md` (etc.) | Markdown link, runtime-read | M4 |
| Annual baseline refresh | `/schedule` agent | GitHub | Automated PR open with diffs | M5 |

---

## High-Level Design for Formal Verification (TLA+ Section)

`tla_required: false`

No concurrent actors, no distributed state. Conversational intake is sequential founder-skill turns; gate evaluation is deterministic over the structured `intake_summary:` block; numeric verification is single-pass. Per `/slo-tla`'s suitability gate, this is the wrong tool here.

---

## Global Execution Rules

See [docs/slo/templates/runbook-template_v_3_template.md §"Global Execution Rules"](templates/runbook-template_v_3_template.md). Project-specific overrides:

- The four hard-block predicate IDs in `references/biz/triage-gate.md` are **immutable** by structural-contract test (`triage_gate_predicate_set_unchanged_from_m1`); this runbook does not touch them.
- `WebFetch` / `WebSearch` denial at the SLO-CLI invocation layer is preserved across this runbook (per `tm-biz-abuse-1`). External regulatory anchors fetched at runbook-author time only and captured as `last_checked:` dates.
- Every UK statute citation must be source-verified against `legislation.gov.uk` at retrieval-stamped dates. Every HMRC manual citation against `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual`. Unverifiable claims removed, not weakened (per R2 M1's `references/templates/citation-discipline.md`).

## Global Entry / Exit Rules

See template. Specifics:

- Baseline: `cargo test --workspace`. Confirm green.
- R3 M2 depends on R2 M1's `references/templates/intake-checklist.md` landing. If R2 M1 unfinished, M2 inline-authors the conversational discipline pattern (acceptable but creates rework when R2 M1 lands; flag in lessons).
- R3 M4 depends on R2 M1's `references/templates/heuristic-numbers-discipline.md` landing.

---

## Background Context

### Current State

The biz pack has 4 advisor skills + 11 generator skills built in Runbooks A / B1 / B2 / C, with the `references/biz/` reference subtree (triage-gate, artifact-schema, jurisdiction-uk, hmrc-vcm-index, ico-duaa-index, ir35-cest-factors, cost-baseline-jpp-law-2026). The four hard-block predicates and the two-tier output convention (`docs/biz/` confidential, `docs/biz-public/` placeholder + Pass 4 PII scan) are stable.

The 2026-04-27 skill-pack review identified two structural risks:

1. **Predicate evaluation by LLM is the largest hallucination surface in the pack**: the four advisor skills (+ `/slo-hire`) rely on the LLM evaluating natural-language predicates from founder prose. `gate-2-deal-value-over-5k` evaluation against "the contract is around eight grand" (monthly? annual? VAT? what term?) is a known failure mode.
2. **The seven biz generators recite numeric heuristics from training memory without source attribution**: CAC payback, MoM growth, burn multiples, conversion rates, fee figures, success thresholds. Founders quote these numbers to investors; sourceless KPI claims are a credibility risk.

The project owner explicitly flagged a third constraint: **conversational discipline** — *"this is a chatbot. So it isn't gonna process, like, hiring people. But giving someone advice."* The structured `intake_summary:` block is the **output of the conversation**, not a UI for the founder to fill.

### Problem

1. **LLM-evaluated predicates with no structured intake**: All four advisor skills (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`) plus `/slo-hire` evaluate the four hard-block gates against natural-language founder prose. `references/biz/triage-gate.md:16` explicitly acknowledges this.
2. **Statute citations paraphrased inline**: SKILL.md prose recites CRA 2015 / ERA 1996 s86 / IANA 2006 s15 / Pensions Act 2008 / ITA07 s257HJ(1) / Consumer Contracts Regulations 2013 / ASA / CAP Code / PECR / DUAA 2025 ceiling figures / HMRC manual paragraphs (VCM34080, VCM3000, VCM31000) — all from training memory, all rendered prose-shaped rather than verbatim quotes.
3. **HMRC manual content paraphrased**: `references/biz/hmrc-vcm-index.md` paraphrases the Abingdon Health Limited v HMRC line and the VCM paragraphs.
4. **Open-ended regulator enumeration**: `gate-1-regulated` evaluation enumerates UK regulators from training memory ("FCA, MHRA, ICO, healthcare, financial services, or any other regulator with statutory enforcement powers"). Closed-enum file would replace.
5. **Numeric arithmetic is LLM-computed**: `/slo-fundraise`'s SAFE worksheet, `/slo-equity`'s cap-table snapshot, `/slo-pricing`'s value-equation calculator — all rely on LLM-computed cells. A 5-percentage-point dilution-table error that goes to investors is the failure mode.
6. **Recited numeric heuristics across 7 generator skills**: ~40+ unsourced numbers across `/slo-metrics`, `/slo-pricing`, `/slo-sales-funnel`, `/slo-product`, `/slo-launch`, `/slo-marketing`, `/slo-talk-to-users`.
7. **Form-shaped intake framing**: the original `legal-intake-form.md` starter file read as a form for the founder to fill; the chatbot's strength is conversation. Contract → `*-intake-contract.md` rename + conversational-discipline framing throughout.

### Target Architecture

See "End-to-End Architecture Diagram" above.

### Key Design Principles

0. **Over-engineering for simplicity**: per [`docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`](PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md), the LLM-driven advisor / generator pipeline can sustainably carry MORE discipline than a human-driven equivalent. R3 embodies the paradigm: 6-field conversational intake with explicit-comprehension follow-ups (per critique S-1), verbatim statute citations from `legislation.gov.uk`, closed-enum 29-row regulator list, numeric verification with re-derivation, KPI baselines with per-row source attribution + annual `/loop` refresh, structural-contract tests across cross-skill citations + immutability of 4 predicates. A human paralegal team would not sustain this much rigor across 5 advisor + 7 generator skills; the LLM pipeline does. The user (founder) experiences a 5-minute conversation that produces a lawyer-review-ready draft with full provenance.

1. **Conversational intake is the UX**: skill drives one-question-at-a-time elicitation; structured `intake_summary:` is the output. Pattern source: [`/slo-ideate`'s seven forcing questions](../skills/slo-ideate/SKILL.md). Refusal-on-ambiguity is the third state of gate evaluation.
2. **Closed-enum regulator list**: `gate-1-regulated` evaluation consults `references/biz/uk-regulator-enumeration.md`; naming a regulator NOT in the file is a refusal pattern (probe further or flag the enum gap).
3. **Verbatim statute quotes from `legislation.gov.uk`**: every cited statute carries a verbatim quote + URL + retrieval date. SKILL.md prose cites file:section, never paraphrases the text.
4. **HMRC manual content quoted, not paraphrased**: same pattern with `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` as the primary source.
5. **Numeric arithmetic re-derived before write**: emit math as runnable Python snippet OR explicit verification pass that re-derives every cell. Mismatch → refuse to draft.
6. **Heuristic numbers carry `baseline_ref:` provenance**: every artifact emitted by a generator skill that consults a baseline file MUST carry the `baseline_ref:` frontmatter pointer with retrieval-date stamp.
7. **Annual refresh cadence**: each authority file has `retrieved:` + `refresh_recommended_by: <retrieved + 12 months>`; SKILL.md emits a stale warning at +12 months and refuses at +24 months.
8. **The four hard-block predicate IDs are immutable**: locked by structural-contract test.
9. **Sister intake contracts share F1 / F4 / F5 fields verbatim**: cross-skill drift on jurisdiction / GDPR / regulator scope evaluation would defeat consistency.

### What to Keep

- All four hard-block predicate IDs in `references/biz/triage-gate.md`.
- Existing `references/biz/artifact-schema.md` frontmatter contract (additive only).
- Existing `docs/biz/` / `docs/biz-public/` two-tier convention.
- Every existing advisor + generator SKILL.md install path.
- Existing `WebFetch` / `WebSearch` denials.
- `/slo-verify` Pass 4 PII pattern scan + override mechanism.
- Cost baseline file `references/biz/cost-baseline-jpp-law-2026.md` (already source-verified at `retrieved: 2026-04-25`).

### What to Change

- `references/biz/uk-regulator-enumeration.md` (already a starter; M1 source-verifies `statutory_basis` of every row).
- `references/biz/uk-employment-statute-anchors.md` (NEW M1).
- `references/biz/uk-consumer-statute-anchors.md` (NEW M1).
- `references/biz/uk-marketing-statute-anchors.md` (NEW M1).
- `references/biz/hmrc-vcm-index.md` (refresh M1: verbatim quotes).
- `references/biz/ico-duaa-index.md` (refresh M1: source-verify DUAA 2025 commencement details).
- `references/biz/legal-intake-form.md` → rename `legal-intake-contract.md` + conversational reframing (M2; partially done in starter file).
- `references/biz/{accounting,equity,fundraise,hire}-intake-contract.md` (NEW M2).
- 5 advisor SKILL.md updates (M2).
- 3 advisor + 1 generator SKILL.md updates for numeric verification (M3).
- `references/biz/saas-kpi-targets-baseline.md` (refresh M4: source-verify each row).
- `references/biz/outbound-conversion-baselines.md` (NEW M4).
- `references/biz/product-prioritization-frameworks.md` (NEW M4).
- `references/biz/value-equation-pricing.md` (NEW M4).
- `references/biz/mom-test-canonical-questions.md` (NEW M4).
- `references/biz/launch-success-thresholds.md` (NEW M4).
- 7 generator SKILL.md updates (M4).
- `references/biz/artifact-schema.md` extended with `baseline_ref:`, `intake_summary:`, `gates_evaluation:` fields (M5).

### Global Red Lines

Standard set; in addition:

- **The four hard-block predicate IDs are immutable**.
- **Source-verification discipline applies to every milestone touching statute / regulator / KPI claims**. Unverifiable claims removed.
- **No `WebFetch` / `WebSearch` enabled in any biz skill**.
- **No conversational intake reframed as a form**. The `legal-intake-form.md` filename rename to `legal-intake-contract.md` is part of M2; downstream sister contracts use `*-intake-contract.md` from creation.
- **No US / EU jurisdiction extension** (v2 architectural pivot).
- No `--no-verify`, no force-pushes.

---

## BDD and Runtime Validation Rules

See template. Project specifics:

### Required Test Coverage Categories

For each milestone:

- happy path
- empty state (e.g., founder declines to provide a field; refusal-on-ambiguity fires)
- ambiguous input (e.g., founder rounds the deal value)
- adversarial input (e.g., founder games the intake to bypass a gate)
- backward compatibility (existing artifacts pre-this-runbook still parse)
- abuse case (per-milestone, see threat-model rows in design overview)
- outdated information (e.g., stale citation > 12 months old)

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Reference-file structural tests | `tests/e2e_biz_imp_m<N>.rs` | `crates/sldo-install/tests/` |
| Cross-skill citation tests | same | same |
| Closed-enum immutability tests | inline in M5's test file | same |

---

## Dependency, Migration, and Refactor Policy

See template. **No new runtime dependencies in this runbook.** No schema migrations.

Refactor budget per-milestone in milestone sections.

---

## Evidence Log Template

See template.

## Self-Review Gate

See template.

## Lessons + Completion Templates

See template (`docs/slo/lessons/biz-imp-m<N>.md`, `docs/slo/completion/biz-imp-m<N>.md`).

---

## Milestone Plan

### Milestone 1 — UK regulator enumeration source-verified + statute anchor files

**Goal**: Source-verify every row of [`references/biz/uk-regulator-enumeration.md`](references/biz/uk-regulator-enumeration.md) against `legislation.gov.uk`; create three new statute-anchor authority files (employment, consumer, marketing); refresh `hmrc-vcm-index.md` with verbatim quotes of VCM34080, VCM3000, VCM31000, Abingdon Health line; refresh `ico-duaa-index.md` with verbatim DUAA 2025 commencement quote.

**Context**: Starter [`references/biz/uk-regulator-enumeration.md`](references/biz/uk-regulator-enumeration.md) already lists 29+ regulators with `statutory_basis:` field — but the bases are starter-quality (one-line). M1 source-verifies each row against `legislation.gov.uk` and captures the verbatim Act title + section anchor. Three new statute-anchor files cover employment, consumer, marketing law that the advisor SKILL.md prose currently paraphrases.

**Important design rule**: **Verbatim quotes only**. SKILL.md updates in M2 cite file:section, never paraphrase the statute text.

**Refactor budget**: `Targeted refactor permitted for replacing inlined statute paraphrases with citations`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `references/biz/uk-regulator-enumeration.md` (starter); existing `hmrc-vcm-index.md`; existing `ico-duaa-index.md`; `legislation.gov.uk` URLs at runbook-author time; `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` |
| Outputs | Source-verified `uk-regulator-enumeration.md`; 3 new statute-anchor files; refreshed `hmrc-vcm-index.md`; refreshed `ico-duaa-index.md`; structural-contract test asserting verbatim-quote fields populated |
| Interfaces touched | `references/biz/` reference subtree |
| Files allowed to change | `references/biz/uk-regulator-enumeration.md`, `references/biz/uk-employment-statute-anchors.md` (NEW), `references/biz/uk-consumer-statute-anchors.md` (NEW), `references/biz/uk-marketing-statute-anchors.md` (NEW), `references/biz/hmrc-vcm-index.md` (refresh), `references/biz/ico-duaa-index.md` (refresh), `crates/sldo-install/tests/e2e_biz_imp_m1.rs` (NEW) |
| Files to read before changing anything | All existing `references/biz/` files; R2 M1's `references/templates/citation-discipline.md` (source hierarchy) |
| New files allowed | 3 new statute-anchor files + the test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing `references/biz/` file shapes preserved (additive refresh only); the four hard-block predicate IDs untouched |
| Forbidden shortcuts | paraphrasing statute text; "approximately says" framing; using vendor-blog citations as primary; using Stack Overflow as authoritative; missing per-row `last_checked:` date |
| **Data classification** | `Public` |
| **Proactive controls in play** | OWASP C1 (security requirements anchored in statute); C5 (validate inputs — closed-enum regulator list is input validation); C9 (audit trail via `last_checked:` per-row dates) |
| **Abuse acceptance scenarios** | `tm-biz-skill-improvements-abuse-2: stale statute citation propagates to founder artifact` — mitigated by `last_checked:` + skill stale-citation warning at +12 months, refuse at +24 months |

#### Out of Scope / Must Not Do

- Modifying advisor SKILL.md prose (M2 job).
- Touching the four hard-block predicate IDs.
- Adding new regulators to the enumeration without a `/slo-architect` re-pass.
- Citing US / EU statute (v2 scope).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read R2 M1's `references/templates/citation-discipline.md` for source hierarchy.
3. Visit `legislation.gov.uk` at runbook-author time. Capture the precise URL pattern for section-level anchors (e.g., `https://www.legislation.gov.uk/ukpga/1996/18/section/86`).
4. Visit `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` and locate VCM34080, VCM3000, VCM31000 paragraphs. Capture verbatim text.
5. Visit `https://www.legislation.gov.uk/ukpga/2025/18` for DUAA 2025; capture commencement details.
6. Plan the order: regulator enum first (since it's the closed enum gate-1 consults), then statute anchors, then HMRC + ICO refresh.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/biz/uk-regulator-enumeration.md` | Source-verify each `statutory_basis:` row against `legislation.gov.uk`; add per-row `statute_url:` field with anchor; **per paradigm — comprehensive temporal audit fields**: `commenced_date:` (when the cited Act took effect), `last_amended:` (most recent amendment retrieved), `last_reviewed:` (when SLO last verified), `next_review_due:` (last_reviewed + 12 months), `confidence:` (high / medium / low based on whether `legislation.gov.uk` was directly verified vs cross-cited from another authority); bump `last_reviewed:` to runbook-author date |
| `references/biz/uk-employment-statute-anchors.md` | NEW: verbatim quotes of IANA 2006 s15 (right-to-work), ERA 1996 s86 (notice periods), Pensions Act 2008 (auto-enrolment thresholds), Equality Act 2010 (protected characteristics), ITEPA 2003 Ch 10 (IR35) |
| `references/biz/uk-consumer-statute-anchors.md` | NEW: CRA 2015 (consumer rights), Consumer Contracts Regulations 2013 (14-day cooling-off), DMCC 2024 verbatim |
| `references/biz/uk-marketing-statute-anchors.md` | NEW: ASA + CAP Code (12th ed) section anchors, PECR (Privacy and Electronic Communications Regulations 2003) verbatim, DUAA 2025 PECR ceiling text |
| `references/biz/hmrc-vcm-index.md` | Refresh: replace paraphrases of VCM34080 / VCM3000 / VCM31000 with verbatim quotes; capture `last_checked:` per paragraph |
| `references/biz/ico-duaa-index.md` | Refresh: replace DUAA 2025 commencement paraphrase with verbatim text from `legislation.gov.uk/ukpga/2025/18` + ICO summary page |
| `crates/sldo-install/tests/e2e_biz_imp_m1.rs` | NEW: structural-contract test asserting (a) `uk-regulator-enumeration.md` rows have `statute_url:` and `last_reviewed:` fields, (b) the 3 new statute-anchor files exist with verbatim-quote sections, (c) HMRC + ICO refresh verbatim quotes present, (d) source URLs are `legislation.gov.uk` or `gov.uk` (no Stack Overflow / vendor blogs) |

#### Step-by-Step

1. Test stub first.
2. Source-verify regulator enum row by row against `legislation.gov.uk`. Capture URL + text per row.
3. Author the 3 new statute-anchor files.
4. Refresh `hmrc-vcm-index.md` with verbatim quotes.
5. Refresh `ico-duaa-index.md` similarly.
6. Verify structural-contract test passes.
7. Smoke tests.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: UK statute + regulator authority files source-verified**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Every regulator row has source-verified `statutory_basis` | happy path | M1 closes | parse uk-regulator-enumeration.md | every row has `statute_url:` resolvable to `legislation.gov.uk` |
| 3 new statute-anchor files exist with verbatim quotes | happy path | M1 closes | inspect each | files present + each cited section has a verbatim quote, not paraphrase |
| HMRC manual paragraphs quoted verbatim | happy path | M1 closes | inspect hmrc-vcm-index.md | VCM34080 + VCM3000 + VCM31000 + Abingdon Health line each have `quoted_text:` blocks |
| ICO DUAA 2025 commencement text verbatim | happy path | M1 closes | inspect ico-duaa-index.md | DUAA 2025 commencement quote present from `legislation.gov.uk/ukpga/2025/18` |
| Statute citation paraphrased instead of quoted | abuse case (`tm-biz-imp-abuse-1: paraphrase smuggled into authority file`) | someone tries to land a "approximately says" line | structural-contract test | test FAILS with "verbatim quote required" |
| Source from blog rather than `legislation.gov.uk` | abuse case (citation-hierarchy violation) | a row cites a vendor blog as `statute_url:` | structural-contract test | test FAILS with "source must be legislation.gov.uk or gov.uk" |
| Stale row | outdated information | a row has `last_checked:` > 12 months ago | runtime use | skill stale-citation warning fires (M2 implementation) |
| Ambiguous regulator naming | ambiguous input | founder names "ICO/Information Commissioner's Office" | M1 reading | both names resolve to same `id: ico` |
| Backward compat: existing biz pack tests | backward compatibility | M1 closes | `cargo test --workspace` | all existing biz-pack structural-contract tests pass |
| Four predicates immutable | abuse case (predicate set tampering) | someone tries to remove a predicate ID | `triage_gate_predicate_set_unchanged_from_m1` test | test FAILS |

#### Regression Tests

- `cargo test --workspace`.
- `triage_gate_predicate_set_unchanged_from_m1` test passes.
- All existing biz-pack structural-contract tests.
- Existing advisor SKILL.md install symlinks.

#### Compatibility Checklist

- [ ] Four predicate IDs unchanged.
- [ ] `references/biz/cost-baseline-jpp-law-2026.md` unchanged (its refresh is annual; out-of-scope here).
- [ ] `references/biz/ir35-cest-factors.md` unchanged.
- [ ] `references/biz/jurisdiction-uk.md` cross-references the new closed enum but otherwise unchanged.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_biz_imp_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `regulator_enum_source_verified` | Every row has source-verified `statutory_basis` + `statute_url:` | parse + URL pattern match against `legislation.gov.uk` or `gov.uk` |
| `statute_anchor_files_have_verbatim_quotes` | The 3 new files have `quoted_text:` blocks per cited section | grep + frontmatter-shape parse |
| `hmrc_vcm_index_verbatim` | VCM paragraphs quoted | grep for "VCM34080", "VCM3000", "VCM31000", "Abingdon Health" + `quoted_text:` block per |
| `ico_duaa_verbatim` | DUAA 2025 commencement quote present | grep + URL match |
| `no_unauthoritative_sources` | No vendor blogs, no Stack Overflow | grep refuses any URL not in (`legislation.gov.uk`, `gov.uk`, `ico.org.uk`, `www.netpromotersystem.com` (Bain — only for NPS context, not statute), `bvp.com` (Bessemer — only for KPI context, not statute)) — restricted to authoritative sources for statute |
| `triage_gate_predicate_set_unchanged_from_m1` | Predicate IDs immutable | exact-match assertion |

#### Smoke Tests

- [ ] Open `uk-regulator-enumeration.md`; click a `statute_url:` link — resolves to `legislation.gov.uk`.
- [ ] Open `hmrc-vcm-index.md`; verify VCM34080 verbatim quote renders.
- [ ] Open `uk-employment-statute-anchors.md`; verify ERA 1996 s86 quote.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Every regulator row source-verified.
- 3 new statute-anchor files authored with verbatim quotes.
- HMRC + ICO refreshes complete.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" with the new statute anchors.
- README.md: not required.

#### Notes

- Source-verification is high-bandwidth manual work. Time-box per row to ~5 minutes; if a row's `statutory_basis` cannot be verified within budget, mark the row `last_checked: pending-verification` and flag for next sprint — do not weaken the verification rule.

---

### Milestone 2 — Five conversational intake contracts + advisor SKILL.md updates

**Goal**: Five `*-intake-contract.md` files (legal, accounting, equity, fundraise, hire) authored with the conversational-elicitation discipline. Five advisor SKILL.md files updated to mandate the contract, restate-and-confirm step, refusal-on-ambiguity, and citation discipline. Existing `references/biz/legal-intake-form.md` renamed to `legal-intake-contract.md`.

**Context**: The starter [`references/biz/legal-intake-form.md`](references/biz/legal-intake-form.md) is partially conversational-reframed (per the project owner's feedback). M2 finishes the rename, drafts the four sister contracts, and updates the SKILL.md files to consume them.

**Important design rule**: **Conversation is the UX, not a form**. Every intake contract repeats verbatim from `references/templates/intake-checklist.md` (R2 M1) the conversational discipline section. Drift between sister contracts on F1/F4/F5 fields is a structural-contract violation.

**Refactor budget**: `Targeted refactor permitted for advisor SKILL.md prose updates that consume the intake contracts and citation files`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1's source-verified regulator enum + statute anchors; R2 M1's `references/templates/intake-checklist.md` + `restate-and-confirm.md` + `escalation.md` + `citation-discipline.md`; existing `references/biz/legal-intake-form.md` (starter); existing 5 advisor SKILL.md files |
| Outputs | 5 intake-contract files (one rename, 4 NEW); 5 SKILL.md updates; structural-contract test asserting (a) F1/F4/F5 fields verbatim across sisters; (b) every advisor SKILL.md cites its intake contract; (c) "Conversation is the UX" disclaimer present in each contract; (d) `legal-intake-form.md` rename complete |
| Interfaces touched | `references/biz/<skill>-intake-contract.md` files; 5 advisor SKILL.md files |
| Files allowed to change | `references/biz/legal-intake-form.md` → rename to `legal-intake-contract.md`, `references/biz/{accounting,equity,fundraise,hire}-intake-contract.md` (NEW), 5 advisor SKILL.md files, `crates/sldo-install/tests/e2e_biz_imp_m2.rs` (NEW) |
| Files to read before changing anything | M1 outputs; R2 M1's `references/templates/` library; existing advisor SKILL.md files; existing `legal-intake-form.md` starter; the 2026-04-27 review's "Concern 1" (form vs conversational) discussion thread |
| New files allowed | 4 sister contract files + the test file (rename of existing file is not a new file) |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing biz-pack tests pass; the four hard-block predicate IDs untouched; existing artifact frontmatter shape extended (`intake_summary:` and `gates_evaluation:` are NEW additive fields) |
| Forbidden shortcuts | "submit a form" framing in any contract file; missing F1/F4/F5 verbatim across sisters; SKILL.md prose that consumes the contract without the conversational-discipline reminder; advisor skill that drafts without the restate-and-confirm step; ambiguity that falls through to draft instead of refusing |
| **Data classification** | `Confidential` for intake conversation content (real founder PII may surface during elicitation); `Public` for the contract-discipline files themselves |
| **Proactive controls in play** | OWASP C1 (security requirements: structured intake closes the predicate-evaluation surface); C5 (validate inputs — F1-F6 schema is input validation); C7 (access controls — `intake_summary:` lands in `tier: confidential` artifacts); C9 (audit trail via `intake_summary:` + `gates_evaluation:` + `restated_at:` timestamp) |
| **Abuse acceptance scenarios** | `tm-biz-skill-improvements-abuse-1: founder games the conversational intake to bypass a gate` — mitigated by F3's structured `deal_value_basis:` enum + `unknown` confidence is refusal; `tm-biz-imp-abuse-3: SKILL.md drift causes one advisor to skip restate-and-confirm` — class eliminated by structural-contract test asserting every advisor SKILL.md cites `references/templates/restate-and-confirm.md` |

#### Out of Scope / Must Not Do

- Numeric verification (M3 job).
- KPI baseline file work (M4 job).
- Touching the four hard-block predicate IDs.
- New advisor doc-types or new skills.
- Modifying generator skills (M4 covers them).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1 lessons.
3. Read R2 M1's `references/templates/intake-checklist.md` + `restate-and-confirm.md` + `escalation.md` + `citation-discipline.md`. If R2 M1 is not yet shipped, inline-author the conversational-discipline pattern into each intake contract (and flag the duplication for cleanup once R2 M1 lands).
4. Read existing 5 advisor SKILL.md files end-to-end.
5. Read existing `references/biz/legal-intake-form.md` starter (already partially conversational-reframed).
6. **Per critique B-7**: enumerate every cross-reference to `references/biz/legal-intake-form.md` in the repo (grep `legal-intake-form.md`); update each to the new path `legal-intake-contract.md` as part of M2. **Note known minor breakage**: existing GitHub issue comment URLs (e.g., issue #19's existing comments) point to the old filename — those won't auto-update; document as accepted breakage in the M2 lessons file and add a follow-up comment to issue #19 noting the rename.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/biz/legal-intake-form.md` → `legal-intake-contract.md` | RENAME; update internal cross-references; verify "Conversation is the UX" framing throughout (already partially done in starter) |
| `references/biz/accounting-intake-contract.md` | NEW: F1/F4/F5 verbatim from legal-intake-contract; F2/F3/F6 domain-specific (HMRC matter type, period, qualifying-activity claim type, FTE counts, founder-personal vs company-side) |
| `references/biz/equity-intake-contract.md` | NEW: F1/F4/F5 verbatim; F2/F3/F6 (current cap table as structured rows, share-class breakdown, SEIS/EIS status, AA application date) |
| `references/biz/fundraise-intake-contract.md` | NEW: F1/F4/F5 verbatim; F2/F3/F6 (round size GBP, planned signature date, AA application date, investor counsel y/n, lead investor identity, qualifying-trade VCM3000 audit date) |
| `references/biz/hire-intake-contract.md` | NEW: F1/F4/F5 verbatim; F2/F3/F6 (role, full/part-time, expected duration, exclusivity, equipment/premises, substitution, CEST output capture) |
| `skills/slo-legal/SKILL.md` | Update to mandate intake contract; replace inlined statute prose with citations to M1 statute anchor files; cite `references/templates/restate-and-confirm.md` |
| `skills/slo-accounting/SKILL.md` | Same shape |
| `skills/slo-equity/SKILL.md` | Same shape |
| `skills/slo-fundraise/SKILL.md` | Same shape |
| `skills/slo-hire/SKILL.md` | Same shape; CEST output capture mandated (artifact must include CEST output text or "CEST not run" notice with explicit risk acknowledgment) |
| `crates/sldo-install/tests/e2e_biz_imp_m2.rs` | NEW: structural-contract test asserting (a) F1/F4/F5 verbatim across 5 contracts, (b) "Conversation is the UX" disclaimer in each, (c) every advisor SKILL.md cites its contract + restate-and-confirm template, (d) `legal-intake-form.md` does not exist (rename done) |

#### Step-by-Step

1. Test stub first.
2. Rename `legal-intake-form.md` → `legal-intake-contract.md`; update cross-references in any consuming file.
3. Author the 4 sister contracts. F1/F4/F5 verbatim from legal; F2/F3/F6 per-skill domain.
4. Update the 5 advisor SKILL.md files to consume contracts + cite M1 statute anchors + restate-and-confirm.
5. Verify structural-contract test passes (especially F1/F4/F5 verbatim assertion).
6. Smoke tests: invoke each advisor skill against a fixture; observe conversational elicitation.
7. Self-review.

#### BDD Acceptance Scenarios

**Feature: conversational intake contracts mandated; advisor SKILL.md updated**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All 5 contracts exist with conversational discipline | happy path | M2 closes | inspect each | each file has "Conversation is the UX" framing + F1-F6 + restate-and-confirm step |
| F1/F4/F5 verbatim across sisters | happy path | M2 closes | byte-compare F1/F4/F5 sections | match for all five contracts |
| Each advisor SKILL.md cites its contract | happy path | M2 closes | grep each | each cites `references/biz/<skill>-intake-contract.md` |
| Each advisor SKILL.md cites restate-and-confirm | happy path | M2 closes | grep each | each cites `references/templates/restate-and-confirm.md` (or inline-authored version if R2 M1 unfinished) |
| Advisor draft refuses on ambiguity | happy path | founder gives "around eight grand" deal value | skill evaluates F3 | skill asks for `deal_value_basis:` clarification; refuses to draft until concrete |
| Advisor draft proceeds on confirmed gates-pass intake | happy path | founder confirms F1-F6 + restate | skill evaluates 4 gates against `intake_summary:` | drafts artifact with `intake_summary:` and `gates_evaluation:` frontmatter |
| Founder games intake to bypass gate-2 | abuse case (`tm-biz-imp-abuse-1`) | founder quotes monthly when total triggers gate-2 | skill computes total from `deal_value_basis: monthly-recurring × deal_value_term_months` | gate-2 fires; route to triage |
| Founder names regulator NOT in enum | abuse case (regulator-enum bypass) | F5 names "Acme Regulator" | skill checks against `uk-regulator-enumeration.md` | refuses; flags content gap or asks founder to pick from enum |
| GDPR document request | abuse case (gate-4 unconditional) | F4 indicates GDPR document | skill enters `triage` mode | unconditional refusal of `draft`; routes to DPO |
| Form-style framing slipped into a contract | abuse case (`tm-biz-imp-abuse-4: contract reframes as form`) | someone edits a contract to say "founder fills in" | structural-contract test | test FAILS with "Conversation is the UX framing required" |
| Backward compat: existing biz artifact pre-M2 | backward compatibility | existing artifact without `intake_summary:` field | `cargo test --workspace` | passes; `intake_summary:` is additive optional |
| `legal-intake-form.md` doesn't exist | structural | M2 closes | `ls references/biz/legal-intake-form.md` | does not exist (rename complete) |
| Sister F1 fields drift | abuse case (`tm-biz-imp-abuse-5: drift in shared fields`) | accounting-intake-contract.md F1 changes wording | structural-contract test | test FAILS with "F1 must match legal-intake-contract.md verbatim" |

#### Regression Tests

- `cargo test --workspace`.
- `triage_gate_predicate_set_unchanged_from_m1` test.
- All existing biz-pack structural-contract tests.
- Existing advisor SKILL.md install symlinks.

#### Compatibility Checklist

- [ ] Existing `triage-gate.md` predicate IDs unchanged.
- [ ] Existing `artifact-schema.md` shape preserved (`intake_summary:` is additive optional).
- [ ] All existing advisor SKILL.md files install.
- [ ] No existing biz-pack test fails.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_biz_imp_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `five_intake_contracts_exist_with_conversational_framing` | Conversational UX framing present | each file has "Conversation is the UX" disclaimer + F1-F6 + restate step |
| `f1_f4_f5_verbatim_across_sisters` | Cross-sister consistency | byte-compare F1/F4/F5 sections from legal-intake-contract against each sister; assert match |
| `every_advisor_skill_md_cites_contract` | SKILL.md updates landed | grep all 5 advisor SKILL.md for `references/biz/<skill>-intake-contract.md` |
| `every_advisor_skill_md_cites_restate` | Restate-and-confirm cited | grep all 5 advisor SKILL.md for `references/templates/restate-and-confirm.md` (or inline equivalent) |
| `legal_intake_form_renamed` | Rename complete | `references/biz/legal-intake-form.md` does not exist; `legal-intake-contract.md` does |
| `intake_summary_field_in_artifact_schema` | Schema extension landed (M5 task; M2 confirms forward-compat) | M2 doesn't add the field; M5 does — but M2 verifies that adding it later won't break existing artifacts |

#### Smoke Tests

- [ ] Open one sister contract (e.g., `equity-intake-contract.md`); confirm F1/F4/F5 byte-identical to legal.
- [ ] Mock-invoke `/slo-legal draft contractor-sow`; observe conversational elicitation; provide ambiguous F3; observe refusal-on-ambiguity.
- [ ] Mock-invoke `/slo-fundraise draft safe-worksheet`; observe AA pre-check + conversational intake.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Five intake contracts exist with conversational framing.
- F1/F4/F5 verbatim across sisters.
- Five advisor SKILL.md files updated.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" with the 5 intake contracts.
- README.md: not required.

#### Notes

- M2 explicitly does NOT add `intake_summary:` / `gates_evaluation:` to `artifact-schema.md` — that's M5's schema-extension job. M2 ensures the SKILL.md prose consumes the contracts; M5 wires the schema.

---

### Milestone 3 — Numeric verification (SAFE / cap-table / pricing math)

**Goal**: `/slo-fundraise`'s `safe-worksheet`, `/slo-equity`'s `cap-table-snapshot`, `/slo-pricing`'s value-equation calculator emit math as runnable Python snippets OR explicit re-derivation pass before write. Mismatch → refuse.

**Context**: LLM-computed arithmetic in cap-table snapshots and SAFE worksheets propagates errors to founder-facing artifacts the founder will quote to investors. A 5-percentage-point dilution-table error is the failure mode.

**Important design rule**: **Math is computed, not narrated**. Either emit a runnable script the founder runs (transparent + verifiable), or emit the artifact with a verification block that re-derives every numeric cell at write time and refuses on mismatch.

**Refactor budget**: `Targeted refactor permitted for adding numeric verification to 3 advisor skills`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M2 outputs (intake contracts + advisor SKILL.md updates); existing `/slo-fundraise`, `/slo-equity`, `/slo-pricing` SKILL.md files |
| Outputs | 3 advisor SKILL.md updates that emit verifiable math; structural-contract test asserting verification discipline |
| Interfaces touched | `/slo-fundraise/SKILL.md`, `/slo-equity/SKILL.md`, `/slo-pricing/SKILL.md` |
| Files allowed to change | `skills/slo-fundraise/SKILL.md`, `skills/slo-equity/SKILL.md`, `skills/slo-pricing/SKILL.md`, `crates/sldo-install/tests/e2e_biz_imp_m3.rs` (NEW) |
| Files to read before changing anything | M1 + M2 lessons; existing 3 SKILL.md files |
| New files allowed | 1 test file |
| New dependencies allowed | `none` (Python is system-installed; the snippet is fence-rendered, not executed by the skill) |
| Migration allowed | `no` |
| Compatibility commitments | Existing draft modes preserved; verification is additive |
| Forbidden shortcuts | LLM-computed math without a re-derivation block; missing refusal-on-mismatch; emitting a script that depends on libraries not in stdlib |
| **Data classification** | `Confidential` for the artifact bodies (cap tables and SAFE math contain real financial figures) |
| **Proactive controls in play** | OWASP C5 (validate inputs — math re-derivation IS validation); C9 (audit trail via verification-result frontmatter field) |
| **Abuse acceptance scenarios** | `tm-biz-imp-abuse-6: LLM-computed dilution table is wrong by 5pp; founder quotes wrong number to investor` — class eliminated by re-derivation pass before write |

#### Out of Scope / Must Not Do

- Modifying advisor doc-types or modes.
- Touching intake contracts (M2 job).
- KPI baselines (M4 job).
- Adding Python as a runtime dependency (the snippet is fence-rendered for the founder to run).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M2 lessons.
3. Read existing 3 SKILL.md files.
4. Identify every numeric cell in current SAFE worksheet / cap-table-snapshot / value-equation outputs (inventory the math surface).

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-fundraise/SKILL.md` | `safe-worksheet` doc-type emits math as a fenced Python snippet (stdlib-only, MIT-licensed `# SPDX-License-Identifier: MIT` header) the founder runs; **per paradigm — two-pass verification**: (1) skill emits the snippet + an expected-results table; (2) skill re-derives every cell inline using a different computation order (e.g., sum-of-shares vs total-shares-given-cap); mismatch on either pass → refuse to write with the diff surfaced. Per critique S-5: rounding tolerance documented as **±£1 for currency cells, ±0.01% for percentages, ±1 for whole-share counts** (per-cell-type table in the SKILL.md). |
| `skills/slo-equity/SKILL.md` | `cap-table-snapshot` doc-type emits structured table where **every "Total" row is independently re-computed via TWO methods** (sum-down, weighted-product cross-check); both must agree within tolerance; mismatch → refuse. Per-cell tolerance same as above. |
| `skills/slo-pricing/SKILL.md` | Value-equation calculator emits as a runnable computation (price = round(value × ratio, -2)); 1.5× experiment math similar; **per paradigm — verify reciprocal**: skill computes price=value×0.25 AND value=price/0.25; both must agree within tolerance; documents the 25-33% range explicitly so the founder sees the band, not a single-point estimate. |
| `crates/sldo-install/tests/e2e_biz_imp_m3.rs` | NEW: structural-contract test asserting (a) each of 3 SKILL.md files describes verification block / runnable snippet, (b) refusal-on-mismatch is documented, (c) Python snippets are stdlib-only (no `import requests`, etc.) |

#### Step-by-Step

1. Test stub first.
2. Inventory math surfaces in pre-flight.
3. Update 3 SKILL.md files with verification discipline.
4. Verify structural-contract test passes.
5. Smoke tests: invoke each skill against a fixture; observe verification block; verify a tampered cell triggers refusal.
6. Self-review.

#### BDD Acceptance Scenarios

**Feature: numeric verification for SAFE / cap-table / pricing math**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| SAFE worksheet emits verifiable math | happy path | `/slo-fundraise draft safe-worksheet` | runs end-to-end | output contains either (a) fenced Python snippet, or (b) re-derivation block with totals checked |
| Cap-table totals re-computed | happy path | `/slo-equity draft cap-table-snapshot` | runs | every "Total" row is independently re-computed before emission |
| Pricing value-equation runs | happy path | `/slo-pricing` value-equation | runs | computation emitted as runnable snippet OR re-derived block |
| Mismatch in cap-table totals | abuse case (`tm-biz-imp-abuse-6`) | LLM-computed total disagrees with sum-of-rows | skill | refuses to write; surfaces the discrepancy |
| Founder asks for 5pp dilution table | happy path | small round | runs | dilution table verified |
| Python snippet has non-stdlib import | abuse case (snippet provenance) | snippet says `import requests` | structural-contract test | test FAILS with "stdlib-only required" |
| Empty SAFE worksheet | empty state | founder declines all F3 numbers | skill | refusal-on-ambiguity from M2's intake fires before reaching M3 verification |
| Backward compat: existing `/slo-pricing` artifacts pre-M3 | backward compatibility | older artifact without verification block | tests | parse cleanly; verification is additive |

#### Regression Tests

- `cargo test --workspace`.
- M1 + M2 structural-contract tests.
- Existing `/slo-fundraise`, `/slo-equity`, `/slo-pricing` install symlinks.

#### Compatibility Checklist

- [ ] Existing draft modes work.
- [ ] Existing `intake_summary:` from M2 still flows through.
- [ ] No new dependencies.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_biz_imp_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `safe_worksheet_has_verification` | SAFE math is verifiable | grep `slo-fundraise/SKILL.md` for "verification block" or "runnable Python snippet" |
| `cap_table_totals_re_derived` | Cap table re-derived | grep `slo-equity/SKILL.md` for "re-derive every Total row" |
| `value_equation_emits_computation` | Pricing math computed | grep `slo-pricing/SKILL.md` for verifiable math |
| `python_snippets_stdlib_only` | Snippet provenance | grep emitted Python templates for non-stdlib imports; assert none |
| `refusal_on_mismatch_documented` | Discipline present | grep each SKILL.md for "refuse to write" pattern |

#### Smoke Tests

- [ ] Mock-invoke `/slo-fundraise draft safe-worksheet`; observe runnable snippet.
- [ ] Mock-invoke `/slo-equity draft cap-table-snapshot`; introduce a tampered total in the agent's response; observe refusal.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- 3 SKILL.md files updated.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md: not required.
- README.md: not required.

#### Notes

- The fence-rendered Python snippet pattern means the SKILL doesn't execute Python; the founder runs it. This avoids adding Python to the runtime trust surface while still verifying the math.

---

### Milestone 4 — KPI baseline files + generator skill updates

**Goal**: Source-verify the [`references/biz/saas-kpi-targets-baseline.md`](references/biz/saas-kpi-targets-baseline.md) starter; create 5 sister KPI / framework / pricing / launch / mom-test files; update the 7 generator SKILL.md files to consume them via `baseline_ref:`.

**Context**: The starter `saas-kpi-targets-baseline.md` is partially populated. M4 source-verifies every row against the cited public sources, captures `last_checked:` per-row, and authors the 5 sister files.

**Important design rule**: **Every numeric heuristic carries provenance**. SKILL.md prose cites file:section, never inlines numbers. Generator artifacts carry `baseline_ref:` frontmatter.

**Refactor budget**: `Targeted refactor permitted for replacing inlined numeric heuristics in 7 generator SKILL.md files with citations to baseline files`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1 outputs; M2 outputs; M3 outputs; existing `references/biz/saas-kpi-targets-baseline.md` (starter); R2 M1's `references/templates/heuristic-numbers-discipline.md`; existing 7 generator SKILL.md files; public sources (Bessemer, OpenView, Sequoia, Andrew Chen, Bain, Bridge Group, RAIN Group, PG, Fitzpatrick, Hormozi) at runbook-author time |
| Outputs | Source-verified `saas-kpi-targets-baseline.md`; 5 sister KPI files; 7 generator SKILL.md updates; structural-contract test asserting cross-skill citations + `baseline_ref:` discipline |
| Interfaces touched | `references/biz/` reference subtree; 7 generator SKILL.md files |
| Files allowed to change | `references/biz/saas-kpi-targets-baseline.md` (refresh), `references/biz/{outbound-conversion-baselines,product-prioritization-frameworks,value-equation-pricing,mom-test-canonical-questions,launch-success-thresholds}.md` (5 NEW), 7 generator SKILL.md files (`slo-metrics`, `slo-pricing`, `slo-sales-funnel`, `slo-product`, `slo-launch`, `slo-marketing`, `slo-talk-to-users`), `crates/sldo-install/tests/e2e_biz_imp_m4.rs` (NEW) |
| Files to read before changing anything | M1 + M2 + M3 lessons; existing 7 generator SKILL.md files; M2 M1's `heuristic-numbers-discipline.md` |
| New files allowed | 5 sister files + 1 test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | The four hard-block predicate IDs unchanged; existing biz-pack tests pass |
| Forbidden shortcuts | recited numbers without source attribution; vendor-blog citations as primary; missing `last_checked:` per-row; copy-paste "approximate" between rows |
| **Data classification** | `Public` |
| **Proactive controls in play** | OWASP C1 (security requirements anchored in source-verified sources); C5 (validate inputs — closed-source-list per row); C9 (audit trail via per-row `last_checked:`) |
| **Abuse acceptance scenarios** | `tm-biz-skill-improvements-abuse-3: KPI baseline drift causes founder to quote stale figure to investor` — mitigated by `last_checked:` per-row + skill stale-warning at +12 months + annual `/loop` agent (M5) |

#### Out of Scope / Must Not Do

- Touching advisor SKILL.md files (M2 job, already done).
- New generator skills.
- Cross-jurisdiction benchmarks (UK only).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1 + M2 + M3 lessons.
3. Read R2 M1's `references/templates/heuristic-numbers-discipline.md`.
4. Visit each public source URL at runbook-author time:
   - Bessemer State of the Cloud (https://www.bvp.com/atlas)
   - OpenView SaaS Benchmarks (https://openviewpartners.com/saas-benchmarks/)
   - Sequoia Retention by the Numbers (https://articles.sequoiacap.com/retention)
   - Andrew Chen retention (https://andrewchen.com/retention-is-king/)
   - Bain NPS (https://www.netpromotersystem.com/about/)
   - Bridge Group SaaS Sales Development Report
   - RAIN Group sales benchmarks
   - Paul Graham *Startup = Growth* (https://paulgraham.com/growth.html)
   - Fitzpatrick *The Mom Test* 2013 (ISBN 978-1492180746) — quote-permitted snippets only
   - Hormozi *$100M Offers* (value-equation source)
5. Capture per-row `last_checked:` date and verbatim claim text where possible.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/biz/saas-kpi-targets-baseline.md` | Refresh: source-verify every row; per-row `last_checked:` populated; per-row `source_url:` populated; **per paradigm — comprehensive provenance fields**: `confidence:` (high/med/low based on whether the source is industry-canonical (Bessemer, OpenView) vs single-vendor blog), `methodology_note:` (one-line: how the source measured the claim — e.g., "Bessemer's burn-multiple is computed from cohort-level cash data; not directly comparable to monthly burn ratios"), `sample_size:` (where the source discloses), `vintage:` (the publication year of the source — distinct from `last_checked:` which is when SLO verified the URL), `applicability_caveat:` (one-line: when this number doesn't apply — e.g., "B2B targets assume product-led; sales-led ACV > £25k may differ") |
| `references/biz/outbound-conversion-baselines.md` | NEW: full funnel rates with sources |
| `references/biz/product-prioritization-frameworks.md` | NEW: RICE (Intercom 2016), Kano (Kano 1984) definitions sourced |
| `references/biz/value-equation-pricing.md` | NEW: "25-33% of value" claim sourced (Hormozi); "30-50% below market" reframed as opinion if not sourceable |
| `references/biz/mom-test-canonical-questions.md` | NEW: Fitzpatrick *The Mom Test* 2013 question scaffolding. **Per critique B-2**: attribution-only (cite ISBN 978-1492180746 + page numbers); the question SCHEMA is the discipline; question text is paraphrased with explicit attribution, no verbatim long-quotes (copyright). |
| `references/biz/launch-success-thresholds.md` | NEW: "set your own threshold" reframing for unsourceable launch numbers |
| `skills/slo-metrics/SKILL.md` | Replace inlined KPI numbers with citations to `saas-kpi-targets-baseline.md@<retrieval-date>`; require artifact `baseline_ref:` frontmatter |
| `skills/slo-pricing/SKILL.md` | Same shape; cite `value-equation-pricing.md` for "25-33% of value" |
| `skills/slo-sales-funnel/SKILL.md` | Cite `outbound-conversion-baselines.md` |
| `skills/slo-product/SKILL.md` | Cite `product-prioritization-frameworks.md` for RICE/Kano |
| `skills/slo-launch/SKILL.md` | Cite `launch-success-thresholds.md`; reframe inlined success numbers as "set your own threshold" |
| `skills/slo-marketing/SKILL.md` | Cite M1's `uk-marketing-statute-anchors.md` for ASA/CAP/PECR |
| `skills/slo-talk-to-users/SKILL.md` | Cite `mom-test-canonical-questions.md` |
| `crates/sldo-install/tests/e2e_biz_imp_m4.rs` | NEW: structural-contract test asserting (a) each of 6 baseline files has source-verified rows with `last_checked:`, (b) every generator SKILL.md cites its baseline file, (c) no inlined numbers remain in generator SKILL.md prose, (d) authoritative sources only |

#### Step-by-Step

1. Test stub first.
2. Source-verify `saas-kpi-targets-baseline.md` row by row.
3. Author the 5 sister files.
4. Update 7 generator SKILL.md files to cite from baselines.
5. Verify structural-contract test passes.
6. Smoke tests: open one generator artifact (e.g., `/slo-metrics b2b`); observe `baseline_ref:` frontmatter.
7. Self-review.

#### BDD Acceptance Scenarios

**Feature: KPI baselines source-verified; generator SKILL.md cites from baselines**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Every KPI row source-verified | happy path | M4 closes | parse `saas-kpi-targets-baseline.md` | every row has `source_url:` + `last_checked:` |
| 5 sister files exist | happy path | M4 closes | inspect | all 5 present + frontmatter valid |
| Each generator SKILL.md cites baseline | happy path | M4 closes | grep | each cites at least one `references/biz/<baseline>.md` |
| Generator artifact carries `baseline_ref:` | happy path | `/slo-metrics b2b` runs | inspect output | `baseline_ref:` frontmatter present + `@<retrieval-date>` |
| Stale baseline (> 12 months) | outdated information | `last_checked: > 12 months ago` | skill consults | skill emits stale warning |
| Stale > 24 months | outdated information | `last_checked: > 24 months` | skill consults | skill refuses; demands refresh |
| Vendor-blog primary citation | abuse case (citation-hierarchy violation) | a baseline file cites a vendor blog as primary | structural-contract test | test FAILS |
| Inlined number in SKILL.md | abuse case (`tm-biz-imp-abuse-7: number drift between SKILL.md and baseline`) | SKILL.md has "≥ 110% NDR" inline | structural-contract test | test FAILS with "cite baseline file" |
| Backward compat | backward compatibility | existing biz pack tests | `cargo test --workspace` | passes |

#### Regression Tests

- `cargo test --workspace`.
- M1 + M2 + M3 structural-contract tests.
- All existing biz-pack tests.

#### Compatibility Checklist

- [ ] All 7 generator SKILL.md install.
- [ ] No advisor SKILL.md affected.
- [ ] No predicate-set drift.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_biz_imp_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `kpi_baseline_rows_source_verified` | Per-row `source_url:` + `last_checked:` populated | parse + URL match |
| `five_sister_baseline_files_exist` | All 5 present + valid frontmatter | path + frontmatter parse |
| `each_generator_skill_md_cites_baseline` | SKILL.md updates landed | grep each generator SKILL.md |
| `no_inlined_numbers_in_generator_skill_md` | Citation discipline | grep checks for inlined numeric patterns (e.g., `\d+%` outside fenced examples) |
| `authoritative_sources_only` | No vendor blogs as primary | URL pattern match against authoritative-source list |

#### Smoke Tests

- [ ] Open `saas-kpi-targets-baseline.md`; click a `source_url:` link — resolves to a primary source.
- [ ] Mock-invoke `/slo-metrics b2b`; observe `baseline_ref:` in output frontmatter.
- [ ] Open `mom-test-canonical-questions.md`; verify Fitzpatrick attribution.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- 6 baseline files source-verified or authored.
- 7 generator SKILL.md files updated.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" with KPI baselines.
- README.md: not required.

#### Notes

- Fitzpatrick *Mom Test* quotation is constrained by copyright; use snippet-permitted excerpts only or paraphrase with attribution. Document the quote-permission decision in lessons.

---

### Milestone 5 — `baseline_ref:` artifact schema field + cross-skill citation tests + annual refresh `/loop`

**Goal**: Add `baseline_ref:`, `intake_summary:`, `gates_evaluation:` fields to [`references/biz/artifact-schema.md`](references/biz/artifact-schema.md). Add cross-skill citation structural-contract tests. Configure annual `/loop` agent that re-fetches public KPI sources and opens a refresh PR.

**Context**: M5 wires the schema and the automation. Schema extensions are additive (existing artifacts unaffected); cross-skill citation tests catch SKILL.md drift; annual loop catches baseline staleness.

**Important design rule**: **Schema extensions are additive optional**. Existing artifacts without the new fields parse cleanly.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1-M4 outputs; existing `references/biz/artifact-schema.md`; existing `/schedule` skill |
| Outputs | Schema extension; cross-skill citation tests; annual `/loop` configuration |
| Interfaces touched | `references/biz/artifact-schema.md`; `crates/sldo-install/tests/`; `/schedule` configuration |
| Files allowed to change | `references/biz/artifact-schema.md`, `crates/sldo-install/tests/e2e_biz_imp_m5.rs` (NEW), `.sldo/refresh-loop.toml` (NEW — `/schedule` config) |
| Files to read before changing anything | M1-M4 lessons; existing `artifact-schema.md`; `/schedule` skill |
| New files allowed | 1 test file + 1 schedule config |
| New dependencies allowed | `none` |
| Migration allowed | `no` (additive only) |
| Compatibility commitments | Existing artifacts parse cleanly; existing tests pass; immutability of 4 predicates preserved |
| Forbidden shortcuts | non-additive schema changes; missing immutability test; auto-merging the refresh PR |
| **Data classification** | `Public` for the schema; `Internal` for the refresh-loop config (no PII) |
| **Proactive controls in play** | C9 (audit trail via schema fields); C7 (access controls — refresh loop doesn't auto-merge) |
| **Abuse acceptance scenarios** | `tm-biz-imp-abuse-8: refresh loop auto-merges into main` — class eliminated by `/schedule` config opening PR only, never auto-merging (matches `/slo-sast` M5 PR discipline) |

#### Out of Scope / Must Not Do

- Modifying existing schema fields.
- Adding non-additive schema changes.
- Auto-merging refresh PRs.
- Touching advisor or generator SKILL.md (M2 / M4 jobs).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1-M4 lessons.
3. Read existing `artifact-schema.md`.
4. Read `/schedule` skill behavior.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/biz/artifact-schema.md` | Add **9 new optional fields per paradigm — comprehensive audit trail**: `baseline_ref:` (string, path + retrieval-date), `intake_summary:` (block, F1-F6 fields per intake contract), `gates_evaluation:` (block, per-predicate `pass / fail / insufficient-info` + `gates_fired:` list), `restated_and_confirmed:` (bool), `restated_at:` (ISO-8601 timestamp with TZ), `agent_version:` (e.g., `claude-opus-4-7`), `agent_session_id:` (opaque session identifier for cross-artifact correlation), `conversation_turn_count:` (number of founder-skill turns during intake), `intake_duration_seconds:` (elapsed elicitation time — anti-pattern detector: if < 30s for full F1-F6 intake, the conversation was bypassed; flag for review). All optional for backward compat; new generator/advisor outputs populate them. |
| `crates/sldo-install/tests/e2e_biz_imp_m5.rs` | NEW: cross-skill citation structural-contract test; immutability test re-asserted; schema-additive-only test |
| `.sldo/refresh-loop.toml` | NEW: `/schedule` configuration for annual KPI baseline refresh PR |

#### Step-by-Step

1. Test stub first.
2. Update `artifact-schema.md` with 3 new optional fields.
3. Author cross-skill citation tests.
4. Configure annual `/loop` for KPI baselines.
5. Verify structural-contract test passes.
6. Smoke tests.
7. Self-review.

#### BDD Acceptance Scenarios

**Feature: schema extended; cross-skill citations tested; refresh loop configured**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Schema has new optional fields | happy path | M5 closes | parse `artifact-schema.md` | 3 new fields documented as optional |
| Cross-skill citation test passes | happy path | M5 closes | `cargo test` | every advisor SKILL.md cites `references/templates/restate-and-confirm.md`; every generator cites a baseline file |
| Existing artifact parses cleanly | backward compatibility | pre-M5 artifact in `docs/biz/` | `/slo-verify` Pass 4 | passes; new fields are optional |
| Refresh `/loop` opens PR (not auto-merge) | abuse case (`tm-biz-imp-abuse-8`) | annual loop fires; baseline drift detected | loop runs | PR opened; no auto-merge |
| Predicate immutability re-asserted | abuse case | someone tries to remove a predicate | `triage_gate_predicate_set_unchanged_from_m1` | test FAILS |
| Schema non-additive change attempt | abuse case (schema-evolution discipline) | someone tries to remove a field | structural-contract test | test FAILS with "schema changes must be additive" |

#### Regression Tests

- `cargo test --workspace`.
- All M1-M4 structural-contract tests.
- `triage_gate_predicate_set_unchanged_from_m1`.

#### Compatibility Checklist

- [ ] All existing biz artifacts parse.
- [ ] All existing biz-pack tests pass.
- [ ] Predicate IDs immutable.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_biz_imp_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `artifact_schema_has_new_optional_fields` | Schema extension landed | parse + grep for `baseline_ref:`, `intake_summary:`, `gates_evaluation:` |
| `cross_skill_citation_test` | SKILL.md citation discipline | every advisor SKILL.md greps for restate-and-confirm; every generator greps for a baseline file |
| `triage_gate_predicate_set_unchanged_from_m1` | Predicate immutability | exact-match assertion |
| `schema_additive_only` | No schema regressions | every existing field still documented |
| `refresh_loop_opens_pr_not_auto_merge` | Loop discipline | parse `.sldo/refresh-loop.toml`; assert no auto-merge flag |

#### Smoke Tests

- [ ] Open `artifact-schema.md`; verify 3 new fields documented as optional.
- [ ] Trigger refresh loop manually; observe PR opened (not merged).
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Schema extended additively.
- Cross-skill citation tests passing.
- Annual `/loop` configured.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" with the schema extension; mention annual refresh loop.
- README.md: optional bullet about KPI baseline refresh.

#### Notes

- The annual `/loop` is the structural-defense against baseline drift. Manual refresh is also acceptable (a contributor can run the loop on-demand).

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | "References subtrees" extension: regulator enum + 3 statute-anchor files | none | none | `references/biz/uk-*-statute-anchors.md`, `hmrc-vcm-index.md` (refresh), `ico-duaa-index.md` (refresh) |
| 2 | "References subtrees" extension: 5 intake contracts | none | none | 5 advisor SKILL.md updates |
| 3 | none | none | none | 3 advisor SKILL.md updates |
| 4 | "References subtrees" extension: 6 KPI baseline files | none | none | 7 generator SKILL.md updates |
| 5 | "References subtrees" + "Annual refresh loop" subsection | optional bullet | none | `references/biz/artifact-schema.md` (additive) |

---

## Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## Carry-forward from prior retros

(Empty until R1 M3 ships and `/slo-retro` files lessons as issues.)

| Issue | Title | Suggested milestone | Status |
|---|---|---|---|
| (none yet) | | | |

---

## Paradigm-driven enhancements (per `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`)

This runbook applies the over-engineering-for-simplicity paradigm at the biz-pack scale. The advisor + generator pipeline absorbs more discipline than a human paralegal team would sustain because the LLM does not pay the cognitive-load tax. Specific layers added because the LLM is the executor:

### Comprehensive temporal audit on regulator + statute + KPI rows (M1 + M4)

Original M1 specified `last_reviewed:` per regulator row. Paradigm-driven extension: **every authority-file row carries five temporal fields**:

- `commenced_date:` — when the cited Act took effect
- `last_amended:` — most recent amendment retrieved at runbook-author time
- `last_checked:` — when SLO last verified the source URL
- `next_review_due:` — `last_checked + 12 months`
- `confidence:` — `high` (directly verified at primary source) / `medium` (cross-cited from a single secondary) / `low` (inferred)

Same shape on KPI baseline rows in M4 with `vintage:` (publication year of the source) and `methodology_note:` (one-line: how the source measured the claim) and `applicability_caveat:` (when this number doesn't apply). A human paralegal team would maintain 1-2 of these per row; the LLM pipeline maintains all five with no marginal authoring cost.

### Comprehensive artifact-schema audit fields (M5)

Original M5 added 3 optional fields. Paradigm-driven extension: **9 fields**, including `agent_version:`, `agent_session_id:`, `conversation_turn_count:`, `intake_duration_seconds:`. The duration field is itself an anti-pattern detector: a full F1-F6 intake completing in < 30s indicates the agent bypassed the conversational discipline; downstream review can flag.

### Two-pass numeric verification with per-cell tolerance (M3)

Critique S-5 surfaced that LLM-computed math needs rounding tolerance. Paradigm-driven correction: **two-pass verification** (compute via primary method + re-derive via different computation order; mismatch → refuse) with **per-cell-type tolerance table**: ±£1 for currency, ±0.01% for percentages, ±1 for whole-share counts. Pricing computes both directions (price → value → price). Cap-table totals computed sum-down AND weighted-product. A human pipeline would settle for one method; LLM absorbs the verification cost.

### Sister-contract verbatim-share with normalization (M2)

Critique B-4 surfaced whitespace-sensitivity in the byte-compare test. Paradigm-driven correction: **normalize whitespace + line-endings before byte-compare** (already applied). Plus the F1/F4/F5 verbatim share is across 5 sister contracts (legal/accounting/equity/fundraise/hire) — drift-prevention via structural-contract test, not human-review.

### Defense-in-depth across milestones

| Concern | Layer 1 | Layer 2 | Layer 3 | Layer 4 |
|---|---|---|---|---|
| Predicate-evaluation hallucination | Conversational intake (M2) | Restate-and-confirm before draft (M2) | Refusal-on-ambiguity third state (M2) | Closed-enum regulator lookup against `uk-regulator-enumeration.md` (M1) |
| Statute citation drift | Verbatim quotes from `legislation.gov.uk` (M1) | Per-row `last_checked:` + `last_amended:` (M1) | Annual `/loop` refresh PR (M5) | Stale-warning at +12 months, refusal at +24 months |
| Numeric arithmetic error | Two-pass verification (M3) | Per-cell tolerance table (M3) | Refusal-on-mismatch with diff surfaced (M3) | Founder-runnable Python snippet (transparent + independently verifiable) |
| KPI baseline drift | Per-row `last_checked:` + source URL (M4) | `confidence:` + `methodology_note:` + `applicability_caveat:` (M4) | Annual `/loop` refresh PR (M5) | `baseline_ref:` frontmatter on every generator artifact (M5) |
| PII leak via biz-public artifacts | Tier convention (existing) | `.gitignore` rule (existing) | Write-time warning when git-tracked + remote (existing) | `/slo-verify` Pass 4 PII scan (existing) |
| Conversational intake bypassed | Discipline cited in SKILL.md (M2) | Restate-and-confirm step (M2) | `intake_duration_seconds:` audit field (M5) flags < 30s as suspect | Cross-skill citation test asserts intake-contract reference present (M5) |

### Comprehensive intake — F1-F6 expanded with explicit-comprehension follow-ups

The F1-F6 intake structure is comprehensive, not minimum-viable. Per critique S-1, F3 (deal value) carries an explicit-comprehension follow-up that re-frames the value in two ways before locking. Same pattern is forward-compatible across F1 (jurisdiction — confirm "England & Wales" vs "Scotland"), F4 (GDPR — confirm scope), F5 (regulator — confirm the named regulator IS in the closed enum). The conversational delivery makes the comprehensive intake feel natural; a paper form would be tedious.

### Bounded by context-window

The paradigm's discipline-vs-context-window balance: intake contracts are ~100-150 lines each (within soft cap from R2); statute anchor files can be longer because they're consulted by section, not read end-to-end; KPI baseline files have many small rows; the 9-field artifact frontmatter is one block per artifact, not loaded across multiple artifacts.

### Ask items still open from critique

The paradigm doesn't auto-resolve every critique ask. Still open:
- B-3: add `--rapid-intake` flag for time-pressed founders (forward-compat documented in `references/templates/intake-checklist.md` per R2 M1; full implementation deferred)
- B-5: confirm MIT license header on emitted Python snippets (applied; see M3 contract)
- S-5: per-cell rounding tolerance applied as ±£1 / ±0.01% / ±1 share — confirm the bands
