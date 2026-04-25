# Business-Side Skill Pack, Runbook C — Team (AI-First Runbook v3)

> **Purpose**: Ship the three team-tier generator skills (`/slo-cofounder`, `/slo-hire`, `/slo-founder-check`) — closing out the 15-skill biz pack scope.
> **Prerequisite reading**: Runbook A + B1 + B2 completion summaries; design overview.

---

## Runbook Metadata

- **Runbook ID**: `biz-skill-pack-c`
- **Prefix**: `biz-c`
- **Stack**: Markdown SKILL.md + Rust structural-contract tests.
- **Public interfaces (new)**: `/slo-cofounder` (single-mode), `/slo-hire` (mode_arg = role-shape: `swe | ae | designer | ops`), `/slo-founder-check` (single-mode self-assessment).
- **All three are CONFIDENTIAL-tier outputs** (real persons / self-assessment data) — `docs/biz/cofounder/`, `docs/biz/hires/`, `docs/biz/founder-check.md`. Pass 4 PII-scan covers `docs/biz-public/`; these never land there.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-cofounder` (eval checklist + trial-project framing + monthly 1:1 agenda) | `done` | 2026-04-25 | 2026-04-25 | per-milestone files in [docs/lessons/](lessons/) and [docs/completion/](completion/) | |
| 2 | `/slo-hire` (sourcing playbook with role-shape mode arg + IR35 triage gate) | `done` | 2026-04-25 | 2026-04-25 | per-milestone files in [docs/lessons/](lessons/) and [docs/completion/](completion/) | |
| 3 | `/slo-founder-check` (self-assessment + worst-case-runway worksheet + YC application prep) | `done` | 2026-04-25 | 2026-04-25 | per-milestone files in [docs/lessons/](lessons/) and [docs/completion/](completion/) | |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — Markdown skill authoring.

---

## Global Execution Rules

Same as B1. Plus: `/slo-cofounder` and `/slo-hire` outputs land in `docs/biz/<area>/<name>.md` (confidential; gitignored). `/slo-founder-check` lands in `docs/biz/founder-check.md` (also confidential — self-assessment data).

---

## Milestone Plan

### M1 — `/slo-cofounder`

Generator. Single-mode. Output `docs/biz/cofounder/<name>.md` (confidential). Content: cofounder evaluation checklist (stress-handling > skills), trial-project framing (4-week paid trial before full equity), monthly 1:1 agenda template (relationship-first, not product-first). Routes equity-split conversations to `/slo-equity` (Runbook A M3). Routes cofounder-disagreement-mediation to "engage a mediator" — out of skill scope.

### M2 — `/slo-hire`

Generator with mode_arg covering role shape (`swe | ae | designer | ops` v1; founder may extend with documented reason). Output `docs/biz/hires/<role>-<name>.md` (confidential). Content: sourcing playbook + interview rubric + offer cadence + onboarding checklist. **Mandatory IR35 triage gate**: every hire decision invokes `references/biz/ir35-cest-factors.md` to determine contractor-vs-employee status; CEST output drives the routing. Hard-blocks "let's call them a contractor for tax efficiency" with the seven IR35 factors check.

### M3 — `/slo-founder-check`

Generator. Single-mode self-assessment. Output `docs/biz/founder-check.md` (confidential — self-assessment is highly personal data even though it's the founder writing about themselves). Content: 12-question self-assessment (stress / runway / cofounder / health / family / finances), worst-case-runway worksheet (cash + months + cut-cost levers + pivot options), optional YC application prep (10 questions YC asks every applicant). CLAUDE.md catalog edit (final — appends C generators).

---

> **Status**: Runbook drafted; M1 starts immediately.
