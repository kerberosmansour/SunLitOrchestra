---
name: hire-intake-contract
created: 2026-05-03
status: active
audience: /slo-hire advisor skill
purpose: |
  Structured-data CONTRACT that the skill MUST gather conversationally before
  emitting a hiring artifact. The four hard-block gates in
  `references/biz/triage-gate.md` and the IR35 triggers in
  `references/biz/ir35-cest-factors.md` are evaluated against this structured
  input, not against free-form founder prose.
---

# /slo-hire — pre-artifact conversational intake contract

**Conversation is the UX.** This file is a structured destination contract for
the skill, not a founder-facing form. The founder experiences one question at a
time, with pushback on vague answers and an explicit restate-and-confirm loop
before the skill evaluates any hard-block gate.

The skill MUST gather every field below through conversation before writing a
hiring artifact. Empty / "I don't know" / hypothetical answers are refusal
signals, not permission to assume.

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

### F2. Role and engagement shape

> What role are we hiring for, and what engagement shape is being considered?

- `role_shape`: enum — `swe` | `ae` | `designer` | `ops`
- `candidate_or_role_slug`: free-text
- `engagement_type`: enum — `employee` | `contractor` | `unknown`
- `full_time_or_part_time`: enum — `full-time` | `part-time` | `unknown`
- `expected_duration_months`: number or `unknown`

**Refusal-on-ambiguity**: unknown employee/contractor status requires IR35
triage before the offer cadence can be finalized.

### F3. IR35 and offer-risk facts

> Which IR35 factors are known before the offer is made?

- `exclusivity_expected`: enum — `yes` | `no` | `unknown`
- `substitution_right`: enum — `genuine-unrestricted` | `limited` | `none` | `unknown`
- `engager_equipment_or_premises`: enum — `yes` | `no` | `mixed` | `unknown`
- `direction_or_professional_discretion`: enum — `direction` | `professional-discretion` | `mixed` | `unknown`
- `cest_output`: enum — `employed` | `self-employed` | `unable-to-determine` | `not-run`
- `cest_output_text_captured`: bool

**Refusal-on-ambiguity**: artifact must include the exact CEST output text or
`CEST not run` with explicit risk acknowledgement. Do not let the founder choose
contractor status for tax efficiency.

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

- `mode_arg`: enum — `swe` | `ae` | `designer` | `ops`
- `candidate_or_role_kebab_slug`: free-text
- `company_legal_name`: free-text
- `offer_stage`: enum — `planning` | `screening` | `finalist` | `offer-ready`

## Restate-and-confirm step

After F1-F6 are gathered, restate the role, engagement shape, IR35 facts, CEST
status, and gate evaluation in 3-5 sentences. Ask: **"Did I get that right?"**
On correction, re-ask the affected field and re-evaluate all gates before
writing the hiring artifact.

## Frontmatter contract (per artifact)

The intake values land in artifact frontmatter as `intake_summary:`,
`gates_evaluation:`, `restated_and_confirmed: true`, and `restated_at:`.
