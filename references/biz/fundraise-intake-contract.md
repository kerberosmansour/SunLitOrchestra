---
name: fundraise-intake-contract
created: 2026-05-03
status: active
audience: /slo-fundraise advisor skill
purpose: |
  Structured-data CONTRACT that the skill MUST gather conversationally before
  entering `draft` mode. The four hard-block gates in
  `references/biz/triage-gate.md` are evaluated against this structured input,
  not against free-form founder prose.
---

# /slo-fundraise — pre-draft conversational intake contract

**Conversation is the UX.** This file is a structured destination contract for
the skill, not a founder-facing form. The founder experiences one question at a
time, with pushback on vague answers and an explicit restate-and-confirm loop
before the skill evaluates any hard-block gate.

The skill MUST gather every field below through conversation before entering
`draft` mode. Empty / "I don't know" / hypothetical answers are refusal signals,
not permission to assume.

## Conversation discipline

- Ask one question at a time.
- Push on vague answers and ladder down when the founder is unsure.
- Convert answers into the structured `intake_summary:` block.
- Restate the situation in 3-5 sentences and ask for confirmation before gate evaluation.
- On correction, re-ask the affected field and re-evaluate the gates.

## Required fields (the contract)

### F1. Jurisdiction

> Where will this contract be performed, and what governing law will it use?

- `jurisdiction_self`: enum — `uk-england-wales` | `uk-scotland` | `uk-northern-ireland` | `non-uk`
- `jurisdiction_counterparty`: enum — same set
- `governing_law_specified`: bool (is there a clause in the doc / situation specifying governing law?)
- `governing_law_value`: free-text (only required if `governing_law_specified: true`)

**Refusal**: any `non-uk` value → canonical "v1 supports UK only" error from `references/biz/jurisdiction-uk.md`. Do not draft. Do not produce a "for reference only" US/EU output.

### F2. Round and investor context

> What fundraise artifact are we preparing, and what is the investor context?

- `fundraise_doc_type`: enum — `safe-worksheet` | `pitch-narrative` | `investor-update` | `term-sheet-redline-brief`
- `round_size_gbp`: number or `unknown`
- `planned_signature_date`: date or `unknown`
- `lead_investor_identity`: free-text or `none-yet`
- `investor_counsel_involved`: enum — `yes` | `no` | `unknown`

**Refusal-on-ambiguity**: unknown investor counsel status does not fall through
to `no`; ask who sent the document and whether counsel is cc'd.

### F3. SEIS/EIS and qualifying-trade pre-check

> Have you applied for SEIS/EIS Advance Assurance, and does the round depend on that relief?

- `seis_eis_relevant`: bool
- `advance_assurance_status`: enum — `not-applied` | `applied` | `received` | `not-applicable` | `unknown`
- `advance_assurance_application_date`: date or `not-applicable` or `unknown`
- `qualifying_trade_vcm3000_audit_date`: date or `unknown`
- `signature_at_least_six_weeks_after_aa_application`: enum — `yes` | `no` | `not-applicable` | `unknown`

**Refusal-on-ambiguity**: if SEIS/EIS is relevant and AA status is unknown, the
skill may prepare a triage memo but must not draft a term-sheet-adjacent artifact.

### F4. GDPR scope

> Does the document being requested relate to processing of personal data: privacy notice, ROPA, DPA, internal data-protection policy, lawful-basis statement, DPIA, DSAR procedure, breach-notification template, cookie policy, or anything else the ICO would expect to see in a controller's accountability file?

- `gdpr_document_requested`: bool
- `gdpr_document_type`: free-text (only required if `gdpr_document_requested: true`)
- `gdpr_data_in_scope`: bool (is personal data otherwise in scope, even if the document itself isn't a GDPR doc — e.g., a contractor SOW that includes shared-data processing terms?)

**Predicate firing**: `gate-4-gdpr-document` fires on `gdpr_document_requested: true` — **unconditional refusal of `draft` mode** for ANY GDPR-related document, locked decision 2026-04-25. Routes to triage with `route_to: dpo (or lawyer + dpo if no DPO)`. Reversal requires a fresh `/slo-architect` pass with new ICO enforcement evidence.

If `gdpr_data_in_scope: true` but `gdpr_document_requested: false` (e.g., a contractor SOW with data-handling clauses but the request is the SOW, not a DPA), the skill drafts the SOW but flags the data-handling clauses as "needs DPO review for adequacy" in the body.

### F5. Regulated sector

> Does the matter touch any regulator with statutory enforcement powers — see `references/biz/uk-regulator-enumeration.md`?

- `regulator_in_scope`: bool
- `regulator_id`: enum (closed list from `uk-regulator-enumeration.md`; only required if `regulator_in_scope: true`)
- `regulator_relationship`: enum — `regulated-business` | `regulated-counterparty` | `regulator-as-counterparty` | `incidental`

**Predicate firing**: `gate-1-regulated` fires on `regulator_in_scope: true` AND `regulator_relationship` ≠ `incidental`. The skill consults `references/biz/uk-regulator-enumeration.md` for routing — the default is `lawyer` but specific regulators (HMRC, ICO, Companies House, Pensions Regulator) override per the per-skill routing table.

**The skill does NOT enumerate regulators from training memory**. If the founder names a regulator not in the enumeration file, refuse: "Regulator <X> is not in the enumeration. Either (a) confirm it's not the body you mean and pick from the file, or (b) flag the gap as a follow-up so the enumeration can be extended via `/slo-architect` review (do not bypass)."

### F6. Artifact metadata

These fields are not used for gate evaluation but must be captured for the artifact frontmatter:

- `doc_type`: enum — `safe-worksheet` | `pitch-narrative` | `investor-update` | `term-sheet-redline-brief`
- `round_kebab_slug`: free-text
- `company_legal_name`: free-text
- `reviewer_roles`: enum list — `lawyer` | `accountant` | `tax-adviser`

## Restate-and-confirm step

After F1-F6 are gathered, restate the round size, AA timing, investor-counsel
status, qualifying-trade assumption, and gate evaluation in 3-5 sentences. Ask:
**"Did I get that right?"** On correction, re-ask the affected field and
re-evaluate all four gates before drafting.

## Frontmatter contract (per artifact)

The intake values land in artifact frontmatter as `intake_summary:`,
`gates_evaluation:`, `restated_and_confirmed: true`, and `restated_at:`.
