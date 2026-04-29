---
name: business-skill-improvements
created: 2026-04-27
status: ideation (preempted — derived from 2026-04-27 skill-pack review + issues #19, #20)
tla_required: false
---

# Business skill improvements

## The pain

The 2026-04-27 skill-pack review identified the **single largest hallucination surface in the entire pack**: the four advisor skills (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`) plus `/slo-hire` rely on the LLM evaluating critical safety predicates from natural-language founder prose with no structured intake. [`references/biz/triage-gate.md:16`](../../references/biz/triage-gate.md) explicitly acknowledges this: *"Predicate evaluation is the LLM's responsibility — these are natural-language tests..."*

Concrete failure mode: a founder says "the contract is around eight grand" and the model evaluates `gate-2-deal-value-over-5k` against an unstated assumption (monthly? annual? ex-VAT? VAT-inclusive? for what term?). The four hard-block gates apply correctly to the *wrong* facts. Drafts land that should have routed to triage. The founder, who is a UK seed-stage technical founder with no prior legal exposure, never sees the gap.

A second class of failure across the seven biz generators (`/slo-metrics`, `/slo-pricing`, `/slo-sales-funnel`, `/slo-product`, `/slo-launch`, `/slo-marketing`, `/slo-talk-to-users`): they recite **dozens of numeric heuristics from training memory without source attribution**: CAC payback months, MoM growth %, burn multiples, conversion rates, fee figures, success thresholds, annual discount %. Founders quote these numbers to investors and board members; sourceless KPI claims that turn out wrong are a credibility risk and the artifact cannot be defended in a board pack.

The project owner's framing: the chatbot's strength is conversation. Intake must be **conversational, not form-filling** — modeled on [`/slo-ideate`'s seven forcing questions discipline](../../skills/slo-ideate/SKILL.md). The structured `intake_summary:` block is the **output of the conversation**, not a UI for the founder.

## Five capabilities the user described without realizing

- Five **conversational intake contracts** (`legal-intake-contract.md`, `accounting-intake-contract.md`, `equity-intake-contract.md`, `fundraise-intake-contract.md`, `hire-intake-contract.md`) — each defines the structured-data destination shape that the skill MUST gather conversationally before entering `draft` mode.
- A `references/biz/uk-regulator-enumeration.md` closed enum (29+ UK regulators with `id`, `display_name`, `domain`, `statutory_basis`, `default_route_to`, `cited_by`) — `gate-1-regulated` evaluation consults this file rather than enumerating from training memory.
- UK statute / HMRC manual / ICO authority files extended (`uk-employment-statute-anchors.md`, `uk-consumer-statute-anchors.md`, `uk-marketing-statute-anchors.md`, plus verbatim-quote refresh of `hmrc-vcm-index.md`) — every advisor SKILL.md cites file:section, never paraphrases.
- Numeric verification for SAFE worksheets, cap-table snapshots, pricing math — emitted as runnable scripts or re-derived by an explicit verification pass before write.
- KPI baseline authority files (`saas-kpi-targets-baseline.md`, `outbound-conversion-baselines.md`, `product-prioritization-frameworks.md`, `value-equation-pricing.md`, `mom-test-canonical-questions.md`, `launch-success-thresholds.md`) — every numeric heuristic in a generator-skill artifact carries `baseline_ref:` provenance.

## Top risks

- **Breach**: low — confidential founder data already lives at `docs/biz/` (gitignored) with the existing PII-pattern scan and write-time warning. This work doesn't expand the PII surface; it deepens the discipline.
- **Compliance fine**: medium — every advisor skill's output is a first-cut artifact the founder takes to a solicitor. A wrong gate evaluation that lets a draft slip through when triage was correct = founder might miss solicitor-required matter. Mitigation: refusal-on-ambiguity rule (the third state added to gate evaluation) catches "I don't know" answers that today fall through to draft.
- **Prolonged outage**: low — these are documentation + reference-file changes plus SKILL.md prose updates. No runtime dependency that can fail at scale.

## Approach A — conservative (recommended)

- **Effort**: 14 person-days (statute authority files require source-verification against `legislation.gov.uk` and HMRC Manuals; conversational intake contracts and KPI baselines compose from R2's templates).
- **Wedge**: M1 = UK regulator enumeration + statute anchor files. Once these exist, the advisor SKILL.md updates are mechanical (replace paraphrased prose with `references/biz/<file>.md@<retrieval-date>` citations). The conversational intake contracts compose from R2's `references/templates/intake-checklist.md`.
- **Risks**: source-verifying every statute citation against `legislation.gov.uk` is high-bandwidth. Rule: unverifiable claims removed, not weakened.

## Approach B — cloud / SaaS

Not applicable.

## Approach C — local / desktop

Not applicable.

## Recommendation

Approach A. 5 milestones. The first lands the authority files (regulator enum + statute anchors); the second lands the five conversational intake contracts and the SKILL.md updates that consume them; the third lands numeric verification (SAFE / cap-table / pricing); the fourth lands the KPI baseline files for the seven generator skills; the fifth lands cross-cutting polish (frontmatter `baseline_ref:` field added to artifact-schema, structural-contract tests for cross-skill citations, refresh-cadence warnings).

This runbook **depends on R2's `references/templates/intake-checklist.md` landing first**. R2 → R3 ordering is the cleanest sequence; if R3 starts before R2's M1, M2 of R3 has to inline-author the conversational discipline pattern that R2 will later generalize.

## Open questions for /slo-research

(Most resolved by reading the existing skill-pack review + issues #19/#20; flagged for completeness.)

1. Verbatim text of HMRC VCM34080, VCM3000, VCM31000, and Abingdon Health Limited v HMRC line — currently paraphrased in [`references/biz/hmrc-vcm-index.md`](../../references/biz/hmrc-vcm-index.md). Source: `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual` — fetch and quote each paragraph at runbook-author time.
2. DUAA 2025 Stage 3 commencement details and the £17.5M / 4% global turnover ceiling — currently cited at [`references/biz/ico-duaa-index.md`](../../references/biz/ico-duaa-index.md). Verify against `https://www.legislation.gov.uk/ukpga/2025/18` + the ICO's own DUAA summary page.
3. JPP Law fixed-fee public pricing as of 2026-04 — currently captured at [`references/biz/cost-baseline-jpp-law-2026.md`](../../references/biz/cost-baseline-jpp-law-2026.md) with `retrieved: 2026-04-25`. Refresh-cadence: annual. The runbook should set up the `/loop @annually` agent that re-fetches and opens a refresh PR.
4. KPI baseline source-verification pattern: per-row `last_checked:` date with public-source URL. Sources to verify before M4: Bessemer State of the Cloud (burn multiple), OpenView SaaS Benchmarks (CAC payback / NDR / gross margin), Sequoia + Andrew Chen retention curves, Bain (NPS), Bridge Group + RAIN Group (outbound funnel), Fitzpatrick *The Mom Test* 2013, Hormozi *$100M Offers* (value equation).
5. UK regulator enumeration source-of-truth: every regulator row's `statutory_basis:` field cites a primary Act on `legislation.gov.uk`. Verify each row before M1.
