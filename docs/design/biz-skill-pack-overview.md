---
name: biz-skill-pack
created: 2026-04-25
status: design-locked
tla_required: false
tla_reason: >
  Markdown skill authoring + reference-doc reads + per-skill artifact writes.
  No concurrent actors sharing state, no distributed consensus, no ordering
  guarantees across processes, no resource leases or locks. Every skill runs
  sequentially in one Claude Code session as file I/O.
security_libs_required: false
security_libs_reason: >
  This feature is prose-shaped Markdown skills, not a runtime system. There
  is no axum service, no Pulumi infra, no multi-tenant auth surface where
  Hulumi or SunLitSecureLibraries components would be candidates. The
  security work here is template-injection defense, output-artifact
  containment (gitignore discipline), and oneNDA byte-verbatim verification —
  none of which map to the upstream security libraries.
ai_component: true
ai_component_reason: >
  Every advisor skill (/slo-legal, /slo-accounting, /slo-equity, /slo-fundraise)
  embeds Claude reasoning to drive draft/translate/triage/prepare modes.
  Founder-supplied prose (deal terms, counterparty names, contract scope)
  flows through the LLM context. The MITRE ATLAS + OWASP LLM Top 10 + NIST
  AI RMF triad applies. Triage decisions are LLM judgments with hard-coded
  refusal gates layered on top.
compliance: [soc2, asvs, gdpr]
compliance_reason: >
  SOC 2 + OWASP ASVS 5.0 are project defaults. GDPR is added because the
  pack itself ingests founder-supplied personal data via the chat prompt
  (counterparty names, contractor identifiers, real persons in NDAs and
  contractor SOWs) and writes it to disk under docs/biz/. /slo-legal also
  triages user GDPR documents (privacy notice, ROPA, DPA) — the broad
  hard-block-on-draft posture (decision locked: 2026-04-25) is what keeps
  GDPR a triage column rather than a controller-side compliance burden.
---

# Design — Business-side skill pack for SLO

## System goal

Add 15 Markdown skills that mirror the engineering pipeline (Think → Plan → Build → Review → Test → Ship → Reflect) for the *company-around-the-product* — legal, accounting, equity, fundraising; customer-facing strategy (talk-to-users, gtm, product, marketing); execution + measurement (launch, sales-funnel, pricing, metrics); and team (cofounder, hire, founder check). The advisor cluster (4 skills) operates with `draft` + `translate` + `triage` + `prepare` modes and refuses to draft when matters cross hard-coded gates (regulated / >£5,000 / counterparty-with-lawyer / ALL GDPR documents). The remaining 11 skills are one-shot generators. Shared scaffolding lives at `references/biz/` outside `skills/` so `sldo-install` ignores it (confirmed by `crates/sldo-install/src/install.rs:44-71`). Total work splits across **4 runbooks** to respect `/slo-plan`'s 5-milestone-per-runbook cap: Runbook A (advisor cluster, 4M), B1 (discovery → strategy → definition, 4M), B2 (execution → optimization, 4M), C (team, 3M).

## Pre-ship decisions (locked 2026-04-25)

- **GDPR rule calibration: broad hard-block on all GDPR drafts.** Privacy notice, ROPA, DPA, internal data-protection policies → translate/triage only, never `draft`. Defensible on professional-negligence + upside-asymmetry grounds; the £17.5M PECR ceiling under DUAA 2025 (Stage 3 commenced 5 February 2026) makes the tail unbounded even though sub-£1M-turnover-private-company enforcement clusters at PECR direct marketing rather than Article 13.
- **Cost baseline: JPP Law fixed-fee public pricing** (https://www.jpplaw.co.uk/sectors/fixed-fee-startup/). Russell Cooke 2026-27 list is not publicly retrievable, so it cannot be cited in a public skill. ROI claims in `/slo-legal` output are anchored to JPP Law with retrieval-date stamps.

## Planned architecture (this feature)

Solid lines exist in the SLO repo today. Dashed lines are added by Runbook A M1 (the wedge: `/slo-legal` v1 + the two shared reference files). Dotted lines are added by Runbook A M2–M4 and Runbooks B + C.

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                              FOUNDER (user)                                   │
│      (UK seed-stage technical founder running SLO on local machine)           │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ /slo-legal | /slo-accounting | /slo-equity
                                   │ /slo-fundraise | /slo-talk-to-users
                                   │ /slo-launch | /slo-sales-funnel | /slo-pricing
                                   │ /slo-metrics | /slo-cofounder | /slo-hire
                                   │ /slo-founder-check
                                   ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│                        Claude Code (skill loader, solid)                      │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ reads SKILL.md + cited reference files
                                   ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│  Biz skill pack  (dashed = M1, dotted = M2+ of Runbook A and all of B + C)    │
│                                                                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Advisor cluster — Runbook A (M1 dashed, M2–M4 dotted)                   │  │
│  │   /slo-legal      mode: draft | translate | triage | prepare  [M1]      │  │
│  │   /slo-accounting mode: draft | translate | triage | prepare  [M2]      │  │
│  │   /slo-equity     mode: draft | translate | triage | prepare  [M3]      │  │
│  │   /slo-fundraise  mode: draft | translate | triage | prepare  [M4]      │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Discovery → Strategy → Definition generators — Runbook B1 (dotted)      │  │
│  │   /slo-talk-to-users  /slo-gtm  /slo-product  /slo-marketing            │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Execution → Optimization generators — Runbook B2 (dotted)               │  │
│  │   /slo-launch  /slo-sales-funnel  /slo-pricing  /slo-metrics            │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Team — Runbook C (dotted)                                               │  │
│  │   /slo-cofounder  /slo-hire  /slo-founder-check                         │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
└────┬──────────────────────────────┬──────────────────────────────────────────┘
     │ reads (M1+)                  │ writes outputs (M1+)
     ▼                              ▼
┌──────────────────────────────┐  ┌──────────────────────────────────────────┐
│ Shared scaffolding (NEW)     │  │ Founder's repo (target of artifacts)     │
│ references/biz/              │  │                                          │
│   triage-gate.md      [M1]   │  │ docs/biz/<area>/<artifact>.md            │
│   cost-baseline-jpp-  [M1]   │  │   (gitignored when confidential          │
│     law-2026.md              │  │    drafts contain real counterparty      │
│   artifact-schema.md  [M2]   │  │    or personal data)                     │
│   jurisdiction-uk.md  [M2]   │  │                                          │
│   hmrc-vcm-index.md   [M3]   │  │ docs/biz-public/<area>/<artifact>.md     │
│   ico-duaa-index.md   [M2]   │  │   (committed; templates only,            │
│   ir35-cest-factors.md[M4]   │  │    placeholders not filled)              │
│   ico-enforcement-    [M2]   │  │                                          │
│     reality.md               │  │                                          │
│   open-template-      [M2]   │  │                                          │
│     anchors.md               │  │                                          │
└──────────────────────────────┘  └──────────────────────────────────────────┘
     │ cited (NOT fetched at runtime)
     ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│ External reference anchors (URLs cited; the pack does NOT enable WebFetch)   │
│   onenda.org                       (CC BY-ND 4.0 NDA — verbatim render only) │
│   gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm34080       │
│   gov.uk/guidance/check-employment-status-for-tax       (CEST, IR35)         │
│   ico.org.uk/.../duaa-summary-of-the-changes/           (DUAA 2025)          │
│   legislation.gov.uk/ukpga/1996/18/section/86           (ERA 1996 s86)       │
│   legislation.gov.uk/ukpga/2025/18                      (DUAA 2025)          │
│   jpplaw.co.uk/sectors/fixed-fee-startup/               (cost baseline)      │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Trust boundaries

- **Founder ↔ Claude Code.** Founder pastes deal-sensitive prose (counterparty names, contract values, contractor IP descriptions, named persons in NDAs / SOWs / privacy contexts). The agent treats this as untrusted-but-private founder data: never echoed to remote services beyond Anthropic's API; never written to a path outside the founder's repo; never written to a path that is git-tracked unless the artifact-schema reference declares the artifact safe-to-track.
- **Claude Code ↔ Skill pack.** Skills read SKILL.md + cited reference files only. Skills do **not** enable WebFetch / WebSearch — external anchors are URLs the skill emits as citations for the founder to follow manually.
- **Skill ↔ Founder's repo (filesystem).** Two-tier output convention: `docs/biz/` (gitignored by default, holds confidential drafts with real data); `docs/biz-public/` (git-tracked, holds placeholder-only artifacts the founder explicitly chose to commit). The artifact-schema reference (M2) defines which mode each artifact category lands in.
- **`/slo-legal draft nda` ↔ oneNDA canonical bytes.** The CC BY-ND 4.0 license forbids derivative works. The skill renders the oneNDA template body verbatim and emits company/counterparty/cover fields as a separately-rendered artifact that wraps but does not edit the canonical text. `/slo-verify` runs a regression test that the rendered NDA body bytes match the canonical oneNDA bytes (hash check).
- **Triage gate ↔ skill behavior.** The `references/biz/triage-gate.md` file is the single source of truth for the four hard-blocks. The four advisor skills cite this file in their SKILL.md prose; downstream `/slo-critique` reviews assume any advisor skill that drafts in a hard-block context is a P1 finding.

## Non-negotiables (downstream cannot change these without migration)

- **Markdown-only.** This feature ships zero Rust code. No new crate. The full pack — 15 skills + ~10 shared reference files — is prose under `skills/<name>/SKILL.md` and `references/biz/`.
- **Reality-first `docs/ARCHITECTURE.md`.** The diagram above lives in this design overview, not in `docs/ARCHITECTURE.md`. ARCHITECTURE.md is updated only when the biz pack actually ships (per MEMORY.md, ARCHITECTURE.md = implemented code at HEAD).
- **`references/biz/` location.** Shared scaffolding lives at `references/biz/` at the repo root. **Not** under `skills/_biz-shared/` — `crates/sldo-install/src/install.rs:44-71` confirms the installer would silently include any `skills/<name>/` containing a `SKILL.md`, making the underscore-prefixed convention fragile.
- **Advisor mode interface.** The four advisor skills accept exactly four modes: `draft`, `translate`, `triage`, `prepare`. Adding new modes is a `/slo-architect` decision, not a per-milestone option. Generator skills (8 of the 12) take no mode arg.
- **Hard-block gates.** Four hard-coded gates in advisor `draft` mode: (1) regulated domain (FCA, MHRA, ICO, healthcare, financial services); (2) deal value > £5,000; (3) counterparty has a lawyer OR founder is being asked to sign their paper; (4) ALL GDPR-related documents (privacy notice, ROPA, DPA, internal data-protection policies). Hard-block routes to `triage` mode with a "see a lawyer/accountant/DPO + here's what to brief them on" output.
- **UK-only in v1.** No `--jurisdiction us` / `--jurisdiction eu` flag. If a user passes a non-UK jurisdiction, the skill emits a clear "v1 supports UK only; US/EU is a v2 architectural pivot" error rather than degrading silently. `references/biz/jurisdiction-uk.md` is the only jurisdiction reference in M1; jurisdiction-us.md / jurisdiction-eu.md are not pre-emptively stubbed.
- **oneNDA verbatim rendering.** `/slo-legal draft nda` MUST render the canonical oneNDA template body byte-for-byte. Company-specific fields, counterparty details, and cover page are emitted in a separate file that wraps the canonical body without editing it. CC BY-ND 4.0 compliance is not optional.
- **Output-tier convention.** Confidential drafts (real names + values) land at `docs/biz/` (gitignored). Placeholder-only templates / decision memos / public-facing analyses land at `docs/biz-public/` (git-tracked). The two-tier rule is enforced by the skill prose and by a `/slo-verify` test that scans biz artifacts for known PII patterns before allowing a tracked commit.
- **No WebFetch / WebSearch in biz skills.** External legal/regulatory anchors are emitted as citations for the founder to follow. The biz pack does not enable model-driven web fetching at runtime — both because the legal/regulatory landscape moves slowly enough that a citation works, and because enabling WebFetch from a context that holds founder personal data would create an exfiltration surface.

## Residual risks carried into /slo-plan

Per the research synthesis, three findings stay as explicit risks the runbook must cite:

1. **GDPR-rule calibration could be wrong in either direction.** Locked broad hard-block; if Sherif (or another founder using SLO) wants to reverse to a narrower direct-marketing/PECR-only block in v2, the change requires a fresh `/slo-architect` pass against new ICO enforcement evidence and is gated by re-running `/slo-critique`'s security persona. The decision is reversible but the migration path is documented, not implicit.
2. **JPP Law fixed-fee pricing changes annually.** `references/biz/cost-baseline-jpp-law-2026.md` carries an explicit retrieval-date stamp and a "valid through" suggestion. A `/loop` schedule annual refresh is the recommended cadence (offered to user at end of M1).
3. **CEST April 2025 refresh stricter substitution criterion.** IR35 triage prose in `/slo-fundraise` and `/slo-hire` (Runbook C) must reflect "unrestricted and genuinely exercisable" substitution and the documented MOO blind spot (HMRC's reading of *PGMOL v HMRC* [2024] UKSC 29 — Bird & Bird "Spot the Difference" April 2025). Triage prose review at M4 (Runbook A) and again before Runbook C ships.
