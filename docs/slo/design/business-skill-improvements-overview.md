---
name: business-skill-improvements
created: 2026-04-27
status: design lock-in
tla_required: false
security_libs_required: false
ai_component: false
compliance: [gdpr, soc2]
---

# Design overview — business skill improvements

## System goal

Close the predicate-evaluation hallucination surface across the five advisor skills (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`, `/slo-hire`) by introducing **conversational intake contracts** that the skill drives via dialogue (modeled on [`/slo-ideate`'s seven forcing questions](../../skills/slo-ideate/SKILL.md)). Move every UK statute / regulator citation from inlined SKILL.md prose into authority files (`uk-regulator-enumeration.md`, `uk-employment-statute-anchors.md`, `uk-consumer-statute-anchors.md`, `uk-marketing-statute-anchors.md`) consulted at use time. Verify SAFE / cap-table / pricing arithmetic before emission. Centralize numeric heuristics for the seven biz generators into KPI baseline files with source attribution + refresh cadence.

## Stack decision

Existing biz-pack stack — no changes:

- Markdown for all SKILL.md, intake contracts, statute / KPI authority files
- Rust (`crates/sldo-install/tests/`) for structural-contract tests
- `WebFetch` / `WebSearch` denied at the SLO-CLI invocation layer per `tm-biz-abuse-1` (no change)
- `legislation.gov.uk` / HMRC Manual / ICO / public KPI sources are captured at runbook-author time only (citations, not runtime fetches)

No new runtime dependencies. No schema migrations. No changes to the four hard-block predicate IDs (immutability locked by `crates/sldo-install/tests/e2e_biz_a_m2.rs::triage_gate_predicate_set_unchanged_from_m1`).

## Components

| Component | Responsibility | Milestone introduced/changed | Key interfaces |
|---|---|---|---|
| `references/biz/uk-regulator-enumeration.md` | Closed enum of UK regulators with `id`, `display_name`, `domain`, `statutory_basis`, `default_route_to`, `cited_by`, `last_reviewed` | M1 | Consulted by `gate-1-regulated` evaluation in every advisor skill |
| `references/biz/uk-employment-statute-anchors.md` | Verbatim quotes of IANA 2006, ERA 1996, Pensions Act 2008, Equality Act 2010, IR35 (ITEPA 2003 Ch 10) sections | M1 | Cited by `/slo-hire`, `/slo-legal` |
| `references/biz/uk-consumer-statute-anchors.md` | Verbatim quotes of CRA 2015, Consumer Contracts Regulations 2013, DMCC 2024 | M1 | Cited by `/slo-legal`, `/slo-marketing`, `/slo-pricing` |
| `references/biz/uk-marketing-statute-anchors.md` | ASA + CAP Code (12th ed), PECR, DUAA 2025 anchors | M1 | Cited by `/slo-marketing`, `/slo-launch`, `/slo-sales-funnel` |
| `references/biz/hmrc-vcm-index.md` (refresh) | Verbatim quotes of VCM34080, VCM3000, VCM31000, Abingdon Health line | M1 | Cited by `/slo-equity`, `/slo-fundraise` |
| `references/biz/{legal,accounting,equity,fundraise,hire}-intake-contract.md` | Conversational intake contracts (rename from `*-intake-form.md`) | M2 | Consumed by advisor skills before `draft` mode |
| Five advisor SKILL.md updates | Mandate intake contract, restate-and-confirm, refusal-on-ambiguity, citation discipline | M2 | SKILL.md prose only; no behavior change beyond intake |
| Numeric verification pass | SAFE math, cap-table totals, pricing value-equation re-derived | M3 | `/slo-fundraise`, `/slo-equity`, `/slo-pricing` |
| `references/biz/saas-kpi-targets-baseline.md` (refresh starter) | Source-verified KPI bands with per-row `last_checked:` | M4 | Cited by `/slo-metrics` (primary) + 6 generators |
| `references/biz/outbound-conversion-baselines.md` | Funnel-stage rates with sources | M4 | Cited by `/slo-sales-funnel`, `/slo-metrics` |
| `references/biz/product-prioritization-frameworks.md` | RICE, Kano definitions with sources | M4 | Cited by `/slo-product` |
| `references/biz/value-equation-pricing.md` | "25-33% of value" claim sourced (Hormozi *$100M Offers*); "30-50% below market" reframed | M4 | Cited by `/slo-pricing` |
| `references/biz/mom-test-canonical-questions.md` | Fitzpatrick *The Mom Test* 2013 question scaffolding | M4 | Cited by `/slo-talk-to-users`, `/slo-product`, `/slo-launch` |
| `references/biz/launch-success-thresholds.md` | "set your own threshold" framing (most prior numbers are unsourceable) | M4 | Cited by `/slo-launch` |
| `references/biz/artifact-schema.md` (refresh) | Adds `baseline_ref:` field for generator outputs that cite numeric targets | M5 | Read by `/slo-verify` Pass 4 schema check |
| Structural-contract tests | Cross-skill citation tests; closed-enum immutability tests | M5 | `cargo test --workspace` baseline |
| `/loop @annually` agent (deferred) | Re-fetches public sources, opens refresh PR | M5 (configured), runs annually | Reuses `/schedule` skill |

## Data flow

```
Founder invokes /slo-legal draft <doc-type>
       │
       ▼
┌──────────────────────────────────────────┐
│ Skill begins CONVERSATIONAL elicitation  │ ← references/templates/intake-checklist.md (R2 M1)
│   F1 Jurisdiction (one question)         │ ← references/biz/legal-intake-contract.md
│   F2 Counterparty representation         │
│   F3 Deal value (ex-VAT, total contract) │
│   F4 GDPR scope                          │
│   F5 Regulated sector                    │ ← references/biz/uk-regulator-enumeration.md
│   F6 Doc-type + counterparty             │
└────────────────────┬─────────────────────┘
                     │ on every vague answer: PUSH BACK
                     │ on insufficient info: REFUSE (third state)
                     ▼
        ┌────────────────────────────┐
        │ Restate in 3-5 sentences   │
        │ Founder confirms / corrects │
        └────────────┬───────────────┘
                     │
                     ▼
        ┌────────────────────────────┐
        │ Evaluate 4 gates against    │ ← references/biz/triage-gate.md
        │ structured intake_summary   │ ← references/biz/uk-regulator-enumeration.md
        └────────────┬───────────────┘
                     │
            ┌────────┴────────┐
            ▼                 ▼
   gates_fired:[]        gates_fired:[gate-X]
            │                 │
            ▼                 ▼
   ┌────────────────┐  ┌─────────────────┐
   │ DRAFT artifact │  │ TRIAGE artifact  │
   │ docs/biz/legal/│  │ docs/biz-public/│
   │ <slug>-<date>  │  │ legal/triage-…  │
   └────────────────┘  └─────────────────┘
```

## Trust boundaries

- `docs/biz/` is `.gitignore`-controlled; confidential artifacts never leave the founder's local machine.
- `docs/biz-public/` is `git`-tracked; placeholder-only by tier convention; `/slo-verify` Pass 4 PII scan is the second-line defense.
- All external regulator anchors (`legislation.gov.uk`, `gov.uk` HMRC, `ico.org.uk`, `www.jpplaw.co.uk`) are emitted as **citations** the founder follows manually — never fetched at runtime by any biz skill (per `tm-biz-abuse-1`).

## Interfaces locked

| Interface | Stability | Notes |
|---|---|---|
| `references/biz/uk-regulator-enumeration.md` per-row schema (`id`, `display_name`, `domain`, `statutory_basis`, `default_route_to`, `cited_by`, `last_reviewed`) | `stable-interface` | Append-only without `/slo-architect` re-pass; no removals or `id` renames |
| `references/biz/<skill>-intake-contract.md` field IDs (F1-F6) | `stable-interface` | Sister contracts must reuse F1 / F4 / F5 verbatim |
| `intake_summary:` + `gates_evaluation:` artifact frontmatter blocks | `stable-interface` | Read by `/slo-verify` Pass 4 + future audit / replay |
| Statute anchor file row schema (citation, statute name, section, retrieval date) | `stable-interface` | Cross-skill citation chain |
| `baseline_ref:` artifact frontmatter field | `stable-interface` | Added in M5 to `artifact-schema.md` |
| `/slo-verify` Pass 4 PII pattern scan + override mechanism (`pii_scan_override:`, `tier_override_reason:`) | `stable-interface` | Existing; not changed |
| Four hard-block predicate IDs in `triage-gate.md` | `immutable` | Locked by structural-contract test; reversal requires `/slo-architect` re-pass |

## TLA+ section

Not required (`tla_required: false`). No concurrent actors, no distributed state. Conversational intake is sequential founder-skill turns; gate evaluation is deterministic over the structured `intake_summary:` block.

## STRIDE sweep (per Step 3.5)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | EoP |
|---|---|---|---|---|---|---|
| Conversational intake elicitation | N/A | mitigated — `intake_summary:` is the structured frontmatter; founder corrections require explicit re-confirmation | mitigated — `intake_summary:` carries `restated_at:` timestamp + `restated_and_confirmed: true` | residual — confidential intake content goes to `docs/biz/`; mitigated by tier convention + write-time `.gitignore` warning + Pass 4 PII scan | N/A | N/A |
| Statute / regulator authority files | N/A | mitigated — git-tracked + per-row `last_checked:` date | N/A | N/A — public regulatory content | N/A | N/A |
| Numeric verification pass (SAFE / cap-table / pricing) | N/A | mitigated — re-derivation pass before write; structural-contract test asserts math consistency | mitigated — verification result captured in artifact frontmatter | N/A | N/A | N/A |
| KPI baseline files | N/A | mitigated — git-tracked + per-row `last_checked:` + annual refresh `/loop` | N/A | N/A | N/A | N/A |

New abuse cases:

- `tm-biz-skill-improvements-abuse-1: founder games the conversational intake to bypass a gate` (e.g., quotes deal value as monthly when total triggers gate-2). Mitigated by F3's structured `deal_value_basis:` enum + skill computing total contract value; `unknown` confidence is a refusal.
- `tm-biz-skill-improvements-abuse-2: stale statute citation propagates to founder artifact` (statute amended after `last_checked:`). Mitigated by `last_checked:` field + skill's stale-citation warning at +12 months. Reversal: refuse to draft until refreshed if > 24 months stale.
- `tm-biz-skill-improvements-abuse-3: KPI baseline drift (Bessemer / OpenView publishes new annual report; SLO file lags)`. Mitigated by `/loop @annually` agent + per-row `last_checked:` warning.

## Compatibility commitments

- The four hard-block predicate IDs in `triage-gate.md` remain unchanged (immutability test).
- Existing artifacts produced before M2 (without `intake_summary:` frontmatter) remain valid; the new field is optional for backward compat.
- The `cost_baseline_ref:` frontmatter field stays for advisor outputs; the new `baseline_ref:` is for generator outputs only.
- Existing `docs/biz/` and `docs/biz-public/` directory conventions unchanged.
- Existing `/slo-verify` Pass 4 PII scan logic unchanged; only the override mechanism's documentation is extended.

## Out-of-scope

- US / EU jurisdiction extensions (v2 architectural pivot per `docs/slo/design/biz-skill-pack-overview.md`).
- New advisor doc-types or new generator skills.
- Substantive changes to the biz-pack threat model beyond the three new abuse cases above.
- Building the executable eval-runner (deferred from R2 / [issue #4](https://github.com/kerberosmansour/SunLitOrchestra/issues/4)); evals in this runbook are documented expectations.
- Cross-skill SAFE-template generation (operative documents remain solicitor-drafted).

## Research-validation discipline (load-bearing)

Every UK statute citation in this runbook must be source-verified against `legislation.gov.uk` at retrieval-stamped dates. Every HMRC manual citation against `https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual`. Every KPI baseline against the source listed in [`references/biz/saas-kpi-targets-baseline.md`](../../references/biz/saas-kpi-targets-baseline.md) per-row.

**Bright-line**: unverifiable claims removed, not weakened. The bar is set in `references/templates/citation-discipline.md` (R2 M1) and applies to this runbook's M1 (statute / regulator) and M4 (KPI baselines) outputs.
