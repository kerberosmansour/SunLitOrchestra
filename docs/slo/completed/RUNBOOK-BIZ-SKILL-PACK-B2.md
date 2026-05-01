# Business-Side Skill Pack, Runbook B2 — Execution → Optimization Generators (AI-First Runbook v3)

> **Purpose**: Ship the four execution-tier generator skills (`/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`) that take a UK seed-stage founder from launch sequence through outbound conversion through pricing through financial-KPI dashboarding.
> **Prerequisite reading**: [docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md](RUNBOOK-BIZ-SKILL-PACK-A.md), [docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-B1.md](RUNBOOK-BIZ-SKILL-PACK-B1.md), [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md).

---

## Runbook Metadata

- **Runbook ID**: `biz-skill-pack-b2`
- **Prefix**: `biz-b2`
- **Primary stack**: Markdown SKILL.md + Rust structural-contract tests (same as Runbook A + B1).
- **Test commands**: same baseline; per-milestone `cargo test -p sldo-install --test e2e_biz_b2_m<N>`.
- **Public interfaces (new)**: `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`. All generators. `/slo-metrics` carries mode arg `consumer | b2b`.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-launch` (one-shot launch sequence: pitch validator, silent → F&F → communities → press) | `done` | 2026-04-25 | 2026-04-25 | (per-milestone files in [docs/slo/lessons/](lessons/) and [docs/slo/completion/](completion/)) | |
| 2 | `/slo-sales-funnel` (funnel math, cold-email template, deal structure) | `done` | 2026-04-25 | 2026-04-25 | (per-milestone files in [docs/slo/lessons/](lessons/) and [docs/slo/completion/](completion/)) | |
| 3 | `/slo-pricing` (value equation, tier model, "increase by 50%" experiment framing) | `done` | 2026-04-25 | 2026-04-25 | (per-milestone files in [docs/slo/lessons/](lessons/) and [docs/slo/completion/](completion/)) | |
| 4 | `/slo-metrics` (financial KPI dashboard, mode arg consumer \| b2b) | `done` | 2026-04-25 | 2026-04-25 | (per-milestone files in [docs/slo/lessons/](lessons/) and [docs/slo/completion/](completion/)) | |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — Markdown skill authoring. No concurrency.

---

## Global Execution Rules

Same as Runbook B1. Generators don't cite advisor predicate IDs; cite `references/biz/artifact-schema.md` for frontmatter contract; UK-only error pattern; no WebFetch; two-tier output convention.

---

## Background Context (B2-specific)

### Current State (post-Runbook-A + B1 execution on this branch)

- 8 biz skills: 4 advisors (Runbook A) + 4 generators (Runbook B1).
- 74 structural-contract tests green (42 from A + 32 from B1).
- 10 shared references at `references/biz/`.
- `/slo-verify` Pass 4 PII-pattern scan integrated (B1 M1).
- CLAUDE.md catalogs all 8 skills shipped.

### Problem (B2-specific)

1. **Founders have strategy + product definition + marketing tactics from B1, but no execution scaffolding.** Launch sequence, outbound conversion, pricing, financial measurement all live OUTSIDE the pack today.
2. **`/slo-metrics` (B2 M4) is the cross-skill twin to `/slo-product metrics` (B1 M3)**. The disambiguation is documented in B1 M3's SKILL.md; B2 M4 must REINFORCE the disambiguation by enumerating the financial KPIs that belong here (CAC, LTV, NDR, MoM revenue, burn multiple, gross margin) and EXPLICITLY routing PM-side requests to `/slo-product metrics`.
3. **`/slo-pricing` interacts with `/slo-fundraise`** for SEIS/EIS qualifying-trade considerations on subscription-revenue products. The skill cross-references but does NOT replicate.

### Key Design Principles

- Same as B1.
- **`/slo-pricing`'s "increase price by 50%" experiment framing is named in the skill** as the canonical pricing-experiment. Founders default-undercharge; the prompt forces them to test 1.5× before committing to under-price.
- **`/slo-metrics burn-multiple` formula** is per Bessemer Cloud Index convention: `burn multiple = net cash burn / net new ARR`. Documented; cited; cross-checked in test.

### Global Red Lines

Same as B1.

---

## Milestone Plan

### Milestone 1 — `/slo-launch`

**Goal**: Generator producing a one-shot launch sequence at `docs/biz-public/launch-<slug>.md`.

**Content**: pitch validator (one-sentence test), launch sequence stages (silent → F&F → communities → press), per-stage success criteria, kill criteria.

**Compatibility**: Runbook A + B1 unchanged.

---

### Milestone 2 — `/slo-sales-funnel`

**Goal**: Generator producing outbound funnel math + cold-email template at `docs/biz-public/sales/funnel-<segment>.md`.

**Content**: funnel math worksheet (target customers → required outreach → conversion rates), cold-email template enforcing 7 outbound principles (subject specificity, personal-not-personalized, one ask, etc.), deal structure (paid trial → recurring → opt-out).

**PECR routing**: any cold-email implementation routes to `/slo-legal triage` for gate-4-gdpr-document.

---

### Milestone 3 — `/slo-pricing`

**Goal**: Generator producing pricing strategy at `docs/biz-public/pricing.md`.

**Content**: value-equation calculator (price = 25-33% of value delivered), tier model, "increase by 50%" experiment framing (default undercharge correction).

---

### Milestone 4 — `/slo-metrics` + cross-skill disambiguation test

**Goal**: Generator with mode arg `consumer | b2b`. Output: `docs/biz-public/metrics.md`.

**Content**: financial KPI dashboard scaffolder. Consumer mode: 15% MoM growth target, NPS, retention curves. B2B mode: NDR ≥ 110% target, CAC payback, burn multiple. Cross-references `/slo-product metrics` for PM-side and routes PM-side requests there.

**Cross-skill test**: every generator with a financial-KPI surface (`/slo-pricing`, `/slo-fundraise` from Runbook A, `/slo-metrics`) cites at least one of the canonical financial KPIs (CAC | LTV | NDR | burn multiple).

**CLAUDE.md catalog**: M4 single edit appending B2 generators (4 rows).

---

> **Status**: Runbook drafted; M1 execution starts immediately.
