---
name: artifact-schema
created: 2026-04-25
status: stable-interface
audience: every advisor skill in the biz pack + future /slo-verify Pass 4 PII scan
purpose: Frontmatter contract for every artifact produced by a biz skill, plus the per-category default-tier mapping that drives docs/biz/ vs docs/biz-public/ placement.
---

# Biz artifact frontmatter contract

Every artifact written by a biz skill MUST carry the frontmatter schema below. The schema is **stable interface** — adding new keys is a `/slo-architect` decision; renaming or removing keys breaks downstream consumers (the deferred `/slo-verify` PII scan, future tooling that reads biz artifacts).

## Frontmatter keys

| Key | Type | Required | Example | Notes |
|---|---|---|---|---|
| `name` | string (kebab-slug) | yes | `nda-acme-contractor-2026-04` | Slug derived from doc-type + counterparty + date |
| `created` | date (YYYY-MM-DD) | yes | `2026-04-25` | Date the artifact was generated |
| `tier` | enum | yes | `confidential` | Exactly two permitted values: `confidential` \| `public`. No free-form. |
| `archetype` | enum | yes | `advisor` | Exactly two permitted values: `advisor` \| `generator`. Distinguishes the four advisor cluster (legal / accounting / equity / fundraise) from the eleven generators across Runbooks B1, B2, C. Stable interface — added in Runbook B1 M1. |
| `skill` | string (skill name without `/slo-` prefix) | yes | `slo-legal` | Provenance — which skill produced this artifact |
| `mode` | enum | yes for advisor outputs | `draft` | Exactly four permitted values: `draft` \| `translate` \| `triage` \| `prepare`. Generators may omit this key. |
| `mode_arg` | string | yes for generators with a domain-specific variant arg | `roadmap` | E.g., `/slo-product` accepts `roadmap`\|`metrics`\|`okrs`; `/slo-marketing` accepts `b2b`\|`b2c`; `/slo-metrics` accepts `consumer`\|`b2b` (Runbook B2); `/slo-hire` accepts role-shape arg (Runbook C). Absent for skills with a single variant. |
| `pii_scan_override` | bool | optional | `true` | Set when a `tier: public` artifact intentionally contains content that would match the PII-pattern scan (e.g., anonymised pseudonyms used in a publicly-shared case study). Pairs with `tier_override_reason`. Read by `/slo-verify` Pass 4 PII scan. |
| `tier_override_reason` | string | required when `pii_scan_override: true` | `anonymised pseudonyms — Alice / Bob / Carol — used in case study; no real persons` | One-line rationale. |
| `jurisdiction` | enum | yes | `uk` | Exactly one permitted value in v1: `uk`. Non-UK requests are rejected before artifact write. |
| `cost_baseline_ref` | string (path + retrieval-date stamp) | yes for advisor `draft` outputs | `references/biz/cost-baseline-jpp-law-2026.md@2026-04-25` | Auditable provenance for ROI claims |
| `triage_gate_passed` | bool | yes for advisor outputs | `true` | False when any predicate in `references/biz/triage-gate.md` fired during evaluation |
| `gates_fired` | list of predicate-ids | yes when `triage_gate_passed: false` | `[gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper]` | Names every predicate that fired. Empty / absent when `triage_gate_passed: true`. |
| `lawyer_review_recommended` | bool | yes for any advisor `draft` output | `true` | Always `true` for draft mode regardless of gate status — drafted docs are first-cut, never final. May be `true` or `false` for translate / triage / prepare based on situation specifics. |
| `expires_or_review_by` | date (YYYY-MM-DD) | yes for `draft` outputs | `2027-04-25` | Drives annual refresh cadence; default 12 months from `created` |
| `template_source` | string (URL) | yes only for nda artifacts using oneNDA | `https://www.onenda.org/` | License-required citation; absent for non-templated artifacts |
| `template_license` | string (SPDX) | yes only for nda artifacts using oneNDA | `CC-BY-ND-4.0` | License obligation marker |

## Per-category default tier mapping

Every artifact category produced by a biz skill maps to a default tier. Skills MAY override the default in specific cases (e.g., a `/slo-legal translate` artifact whose source legal doc happens to contain real personal data → bumped from `public` to `confidential`).

| Skill | Mode | Doc-type / situation | Default tier | Output dir |
|---|---|---|---|---|
| `slo-legal` | `draft` | any v1 doc-type (`nda`, `contractor-sow`, `ip-assignment`, `terms-and-conditions`) | `confidential` | `docs/biz/legal/` |
| `slo-legal` | `translate` | counterparty-supplied legal doc | `public` | `docs/biz-public/legal/` (override to `confidential` if source contains real PII) |
| `slo-legal` | `triage` | situation memo | `public` | `docs/biz-public/legal/` |
| `slo-legal` | `prepare` | lawyer-call brief | `public` | `docs/biz-public/legal/` |
| `slo-accounting` | `draft` | `brief-the-accountant`, `r-and-d-claim-narrative` | `confidential` | `docs/biz/accounting/` |
| `slo-accounting` | `translate` / `triage` / `prepare` | (as per `slo-legal`) | `public` | `docs/biz-public/accounting/` |
| `slo-equity` | `draft` | `cofounder-split-rationale`, `cap-table-snapshot`, `vesting-schedule` | `confidential` | `docs/biz/equity/` |
| `slo-equity` | other modes | per pattern | `public` | `docs/biz-public/equity/` |
| `slo-fundraise` | `draft` | `safe-template`, `pitch-narrative`, `investor-update`, `term-sheet-redline-prep` | `confidential` | `docs/biz/fundraise/` |
| `slo-fundraise` | other modes | per pattern | `public` | `docs/biz-public/fundraise/` |
| `slo-talk-to-users` | (generator — no mode) | interview script, post-call extraction | `confidential` | `docs/biz/users/` (Runbook B1 M1) |
| `slo-launch`, `slo-sales-funnel`, `slo-pricing`, `slo-metrics`, `slo-gtm`, `slo-product`, `slo-marketing` | (generator — no mode) | strategy / pricing / metrics docs | `public` (no real PII expected) | `docs/biz-public/<area>/` |
| `slo-cofounder`, `slo-hire`, `slo-founder-check` | (generator — no mode) | per-person notes (cofounder eval, hire-pipeline, self-assessment) | `confidential` | `docs/biz/<area>/` (Runbook C) |

## Founder-repo `.gitignore` requirement

The biz pack assumes the founder's repo `.gitignore` excludes `docs/biz/`. Skills MUST warn at write-time when the target dir is git-tracked AND a remote exists AND `tier: confidential`. The warning text:

> WARNING: `docs/biz/` should be in your `.gitignore`. Add `docs/biz/` to `.gitignore` before you commit. Confidential artifacts in this directory contain real counterparty names, deal values, IP scope, or personal data — pushing them to a public remote leaks deal-sensitive information to anyone running GitHub code search. See `SECURITY.md` (root) "Biz skill pack — additional rules" → "Founder personal data — handling discipline" for the full discipline.

## Deferred — `/slo-verify` PII-pattern scan

The `/slo-verify` Pass 4 (security) scan over `docs/biz-public/` for known PII patterns (email regex, UK NI numbers, sort codes, named-person heuristics) is **deferred from this M2 to Runbook B1 M1** (`/slo-talk-to-users`). Reason: the first generator skills are where PII-shaped artifacts naturally land, providing real fixtures rather than empty-state placeholders. This file's per-category default-tier mapping is the schema the deferred scan will read.

## Adding a new artifact category

When a future milestone adds a new artifact category:

1. Append a row to the per-category default-tier table above with the skill, mode, doc-type, default tier, and output dir.
2. If the new category warrants a new tier value (NOT in the `confidential` | `public` enum), the change is a `/slo-architect` re-pass — do NOT extend the enum unilaterally.
3. Update the deferred PII-scan plan to cover the new category.
