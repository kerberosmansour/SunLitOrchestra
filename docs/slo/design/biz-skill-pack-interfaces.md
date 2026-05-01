# Interfaces — biz-skill-pack

Interfaces that downstream milestones must not rename or reshape without an explicit migration entry. Stability levels:

- **`stable`** — frozen for the life of this feature and beyond; breaking change requires a migration entry in the runbook.
- **`evolving`** — may change with a migration entry; not renamed casually.
- **`internal`** — fair game for refactor.

## Skill invocation verbs (15 new)

| Skill | Stability | Archetype | Notes |
|---|---|---|---|
| `/slo-legal` | `stable` (Runbook A M1) | advisor | Modes: `draft <doc-type>`, `translate <file>`, `triage <situation>`, `prepare <situation>`. v1 doc types: `nda`, `contractor-sow`, `ip-assignment`, `terms-and-conditions`. |
| `/slo-accounting` | `stable` (Runbook A M2) | advisor | Same four modes. v1 surfaces: bookkeeping triage, R&D tax credit prep, VAT registration timing, MTD compliance. |
| `/slo-equity` | `stable` (Runbook A M3) | advisor | Same four modes. v1 surfaces: cofounder split rationale, 4yr/1yr-cliff schedule, EMI option triage, dilution math. |
| `/slo-fundraise` | `stable` (Runbook A M4) | advisor | Same four modes. v1 surfaces: SAFE / cap-and-discount math, SEIS/EIS Advance Assurance triage (cite VCM34080 / VCM3000 / VCM31000), pitch narrative, term-sheet redline prep. |
| `/slo-talk-to-users` | `stable` (Runbook B1 M1) | generator | Output: `docs/biz/users/<date>-<name>.md` interview script + post-call extraction template. |
| `/slo-gtm` | `stable` (Runbook B1 M2) | generator | Output: `docs/biz-public/gtm/strategy.md`. Strategy-tier: ICP definition, segmentation, GTM motion choice (PLG \| sales-led \| community-led \| hybrid), channel strategy. Distinct from `/slo-marketing` which carries tactics. |
| `/slo-product` | `stable` (Runbook B1 M3) | generator | Output: `docs/biz-public/product/<artifact>.md`. Mode arg: `roadmap` \| `metrics` \| `okrs`. PM-side: north-star metric, activation funnel, retention curves, OKRs, roadmap framework, feature prioritization (RICE / Kano). Distinct from `/slo-metrics` which carries financial / business KPIs. |
| `/slo-marketing` | `stable` (Runbook B1 M4) | generator | Output: `docs/biz-public/marketing/<segment>-plan.md`. Mode arg: `b2b` \| `b2c`. Tactics-tier: brand voice, content calendar, channel mix, demand-gen, paid acquisition. Distinct from `/slo-launch` (one-shot event) and `/slo-sales-funnel` (one channel — outbound). |
| `/slo-launch` | `stable` (Runbook B2 M1) | generator | Output: `docs/biz-public/launch-<slug>.md` one-sentence pitch validator + launch sequence (silent → F&F → communities → press). |
| `/slo-sales-funnel` | `stable` (Runbook B2 M2) | generator | Output: `docs/biz-public/sales/funnel-<segment>.md` cold-email template + funnel math worksheet. |
| `/slo-pricing` | `stable` (Runbook B2 M3) | generator | Output: `docs/biz-public/pricing.md` value-equation calculator + tier model. |
| `/slo-metrics` | `stable` (Runbook B2 M4) | generator | Output: `docs/biz-public/metrics.md` financial / business KPI dashboard scaffolder (mode arg: `consumer` \| `b2b`). Carries CAC, LTV, NDR, MoM growth, burn multiple, gross margin. Distinct from `/slo-product metrics` which carries product KPIs. |
| `/slo-cofounder` | `stable` (Runbook C M1) | generator | Output: `docs/biz/cofounder/<name>.md` (gitignored — contains real persons). Eval checklist + trial-project framing + monthly 1:1 agenda. |
| `/slo-hire` | `stable` (Runbook C M2) | generator | Output: `docs/biz/hires/<role>-<name>.md` (gitignored). Sourcing playbook (mode arg: `swe` \| `ae` etc.) + IR35 triage gate (cites `references/biz/ir35-cest-factors.md`). |
| `/slo-founder-check` | `stable` (Runbook C M3) | generator | Output: `docs/biz/founder-check.md` (gitignored — self-assessment). Worst-case-runway worksheet + YC application prep. |

The advisor / generator distinction is itself an interface: an advisor skill MUST accept the four modes; a generator skill MUST NOT take a mode arg unless it's a domain-specific variant flag (e.g., `/slo-metrics consumer` vs `b2b`).

## Shared scaffolding — `references/biz/`

| Path | Stability | Owner | Ships in | Notes |
|---|---|---|---|---|
| `references/biz/triage-gate.md` | `stable` | M1 | Runbook A M1 | Single source of truth for the four hard-blocks. Cited by all four advisor skills. |
| `references/biz/cost-baseline-jpp-law-2026.md` | `evolving` | M1 | Runbook A M1 | Retrieval-date stamped (annual refresh expected). Cited by `/slo-legal` ROI block. |
| `references/biz/artifact-schema.md` | `stable` | M2 | Runbook A M2 | Frontmatter contract for `docs/biz/<area>/<artifact>.md` and `docs/biz-public/<area>/<artifact>.md`. Defines which artifact categories are confidential-tier vs public-tier. |
| `references/biz/jurisdiction-uk.md` | `stable` | M2 | Runbook A M2 | UK-only prose anchors for advisor skills. |
| `references/biz/ico-duaa-index.md` | `evolving` | M2 | Runbook A M2 | DUAA 2025 commencement dates, complaints-procedure duty, lawful-basis examples. |
| `references/biz/ico-enforcement-reality.md` | `evolving` | M2 | Runbook A M2 | PECR-vs-Article-13 enforcement pattern; cited so the broad GDPR hard-block has documented context, not just the gate. |
| `references/biz/open-template-anchors.md` | `stable` | M2 | Runbook A M2 | oneNDA + oneSaaS + Kindrik notes; license obligations (CC BY-ND 4.0 verbatim-render rule for oneNDA). |
| `references/biz/hmrc-vcm-index.md` | `evolving` | M3 | Runbook A M3 | VCM34080, VCM3000, VCM31000 with retrieval dates; cited by `/slo-equity` and `/slo-fundraise`. |
| `references/biz/ir35-cest-factors.md` | `evolving` | M4 | Runbook A M4 | Three-factor list (substitution, MOO, control) + CEST April 2025 refresh notes + PGMOL v HMRC commentary. Cited by `/slo-legal triage` (contractor vs employee), `/slo-fundraise` (status determination context), `/slo-hire` (Runbook C). |
| `references/biz/templates/onenda-uk.md` | `stable` (verbatim) | M1 | Runbook A M1 | Canonical oneNDA bytes. Hash check enforced in `sldo-install` test. |

## Output artifact paths and tier convention

| Path | Tier | Stability | Notes |
|---|---|---|---|
| `docs/biz/<area>/<artifact>.md` | confidential (gitignored) | `stable` | Default for advisor `draft` outputs (real names + values), `/slo-talk-to-users` interview notes, `/slo-cofounder`, `/slo-hire`, `/slo-founder-check`. |
| `docs/biz-public/<area>/<artifact>.md` | public (git-tracked) | `stable` | `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`. Also: triage / prepare outputs from advisor skills (decisions and lawyer-brief prep, no real personal data). |
| `.gitignore` entry: `docs/biz/` | — | `stable` | M1 adds `docs/biz/` to the project's `.gitignore` template (the founder's repo, not this SLO repo). |

The two-tier rule is enforced in two places: (1) the artifact-schema reference doc declares each artifact category's tier; (2) a `/slo-verify` test (M1+) scans `docs/biz-public/` for known PII patterns (email regex, UK NI numbers, sort-codes) and fails the milestone if a confidential-looking artifact is in the public tier.

## Advisor mode contract

Every advisor skill MUST accept these four modes and refuse unknown modes with a clear error:

| Mode | Input | Output | Hard-block triggers |
|---|---|---|---|
| `draft <doc-type>` | doc-type from skill's v1 menu | `docs/biz/<area>/<doc>-<counterparty>.md` | regulated domain / >£5,000 / counterparty-with-lawyer / GDPR doc → routes to `triage` |
| `translate <file>` | path to a legal/contractual doc the founder received | `docs/biz-public/<area>/translate-<file>.md` plain-English summary + risk callouts + redline questions | none — translate is always permitted |
| `triage <situation>` | free-text situation description | `docs/biz-public/<area>/triage-<slug>.md` decision + "lawyer/accountant/DPO required because X + here's what to brief them on" | none — triage is always permitted |
| `prepare <situation>` | "I have a [lawyer / accountant / advisor] call about X" | `docs/biz-public/<area>/prepare-<slug>.md` question checklist + key-terms glossary + "what good looks like" framing | none — prepare is always permitted |

Hard-block conditions are identical across all four advisor skills and live in `references/biz/triage-gate.md`. A skill author cannot inline a different gate set.

## Frontmatter contract for biz artifacts

Every artifact written by a biz skill MUST include this frontmatter:

| Key | Type | Required | Example |
|---|---|---|---|
| `name` | string (kebab-slug) | yes | `nda-acme-contractor-2026-04` |
| `created` | date (YYYY-MM-DD) | yes | `2026-04-25` |
| `tier` | enum: `confidential` \| `public` | yes | `confidential` |
| `skill` | string (which skill produced it) | yes | `slo-legal` |
| `mode` | enum: `draft` \| `translate` \| `triage` \| `prepare` (advisor only) | yes for advisor outputs | `draft` |
| `jurisdiction` | enum: `uk` (v1) | yes | `uk` |
| `cost_baseline_ref` | string (path to cost-baseline ref + version) | for advisor `draft` outputs | `references/biz/cost-baseline-jpp-law-2026.md@2026-04-25` |
| `triage_gate_passed` | bool | for advisor `draft` outputs | `true` |
| `lawyer_review_recommended` | bool | always (default `false` for translate/triage; `true` for `draft` regardless of gate) | `true` |
| `expires_or_review_by` | date | for `draft` outputs | `2027-04-25` |

The schema lives in `references/biz/artifact-schema.md` (M2). Validation runs at `/slo-verify` time per milestone.

## Triage-gate predicate contract (load-bearing)

The four hard-block predicates in `references/biz/triage-gate.md` are the single source of truth. Each predicate has:

| Field | Type | Example |
|---|---|---|
| `id` | string (stable) | `gate-1-regulated` |
| `name` | string | `Regulated domain` |
| `predicate` | one-line natural-language test | `Is the matter touching FCA, MHRA, ICO, healthcare, or financial services?` |
| `if_true` | enum: `route_to_triage` \| `block_outright` | `route_to_triage` |
| `route_to` | enum: `lawyer` \| `accountant` \| `dpo` \| `accountant_and_lawyer` | `lawyer` |
| `rationale_doc` | path to a deeper-context doc | `references/biz/ico-enforcement-reality.md` |

The four predicate ids are stable: `gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`. Adding a fifth gate is a `/slo-architect` decision, not a per-milestone option.

## What is explicitly NOT an interface

- **Skill prompt body wording.** The `SKILL.md` body text can be rewritten; only the frontmatter (`name`, `description`), the verb, and the mode contract are interface.
- **Specific phrasing inside `references/biz/*.md`.** The schema (frontmatter + predicate fields above) is the interface; the prose under each predicate is not.
- **Specific filenames inside `docs/biz/<area>/`.** The directory layout (`docs/biz/`, `docs/biz-public/`) is interface; per-area subdirectory choices (e.g., `docs/biz/legal/nda/` vs `docs/biz/legal/`) are evolving.
- **The exact text of the JPP Law cost-baseline rows.** The retrieval-date stamp + the schema (line items with prices) is interface; copying the prose verbatim is not (and would risk JPP Law's content rights).
- **Internal helper phrases like "consider getting a second opinion".** Not interface; soften / restructure as needed during `/slo-execute`.
