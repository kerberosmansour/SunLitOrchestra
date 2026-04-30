# Skill Pack Catalog

> **Status**: canonical living catalog of shipped SunLitOrchestrate skills at HEAD.
> **Audience**: contributors, host-overlay authors, and users deciding which skill to run.

Use this file for the host-neutral list of shipped skills. Use [../CLAUDE.md](../CLAUDE.md) for the Claude Code overlay, [../copilot-instructions.md](../copilot-instructions.md) for the GitHub Copilot overlay, [getting-started.md](getting-started.md) for the first-run path, and [design/agent-host-capabilities.md](design/agent-host-capabilities.md) for current host support boundaries.

## Repo reality at HEAD

- The portable unit is the Markdown `SKILL.md` contract under `skills/<name>/SKILL.md`.
- The active Rust workspace members are `sldo-common`, `sldo-research`, `sldo-install`, and `xtasks/sast-verify`.
- The legacy `sldo-plan`, `sldo-run`, and `sldo-tauri` surfaces are not current workspace members and should not be treated as active interfaces.
- `sldo-install` can install the same skill pack into Claude Code or GitHub Copilot.
- Headless runtime automation is still host-specific today. Check [design/agent-host-capabilities.md](design/agent-host-capabilities.md) before promising a runtime surface.

## Sprint flow

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `/slo-ideate` | YC-style product interrogation before any code |
| Research | `/slo-research` | Host-native interactive research first; optional Claude batch backend for sourced dossiers |
| Architect | `/slo-architect` | Stack + `ARCHITECTURE.md` + interfaces lock-in + `tla_required` flag |
| Verify design | `/slo-tla` | TLC model-check the design when concurrency or ordering risk is real |
| Plan | `/slo-plan` | Interactive v3 runbook authoring, one milestone at a time |
| Critique | `/slo-critique` | Four-persona adversarial review before execution |
| Execute | `/slo-execute M<N>` | Per-milestone driver with allow-list enforcement |
| Verify | `/slo-verify M<N>` | Runtime QA with Playwright for UI surfaces |
| Close | `/slo-retro M<N>` | Lessons + completion + tracker update |
| Ship | `/slo-ship` | Open a runbook-aware PR |

## Power tools

| Skill | Purpose | Host story |
|---|---|---|
| `/slo-second-opinion` | Cross-model disagreement surfacer | Host-neutral. Compares the current host against an external provider (Codex / Gemini). |
| `/slo-freeze <path>` | Lock edits to one directory for the session | Host-neutral. |
| `/slo-resume` | Read the current runbook tracker and suggest the next move | Host-neutral. |
| `/slo-rulegen` | Bootstrap or extend Semgrep rule packs for Rust workspaces | Host-neutral. The bug-summary input can come from any agent-driven workflow. |
| `/slo-ruleverify` | Run the deterministic SAST gate over an existing rule pack | Host-neutral. |
| `/slo-sast` | Wire threat-model-driven SAST scanning into a target repo | Host-neutral. Subprocess invocations are `git`, `gh`, and `semgrep` — never an agent CLI. |

## Business advisor skills

UK-only in v1. These skills operate in `draft`, `translate`, `triage`, or `prepare` modes and hard-block where their gate says a professional must take over.

| Skill | Domain | Notes |
|---|---|---|
| `/slo-legal` | UK legal | NDA, contractor SOW, IP assignment, T&Cs |
| `/slo-accounting` | UK accounting | Bookkeeping, VAT, R&D credit, MTD |
| `/slo-equity` | UK equity | Founder split, vesting, cap-table snapshot |
| `/slo-fundraise` | UK fundraise | SAFE math, pitch narrative, term-sheet prep |

## Business generator skills

These skills generate exactly one primary artifact each.

| Skill | Output | Purpose |
|---|---|---|
| `/slo-talk-to-users` | `docs/biz/users/<date>-<name>.md` | Interview prep and post-interview extraction |
| `/slo-gtm` | `docs/biz-public/gtm/strategy.md` | ICP, segmentation, GTM motion, channel strategy |
| `/slo-product roadmap` | `docs/biz-public/product/roadmap.md` | Product roadmap |
| `/slo-product metrics` | `docs/biz-public/product/metrics.md` | Product KPI dashboard |
| `/slo-product okrs` | `docs/biz-public/product/okrs.md` | Quarterly product OKRs |
| `/slo-marketing b2b|b2c` | `docs/biz-public/marketing/<mode>-plan.md` | Marketing tactics plan |
| `/slo-launch` | `docs/biz-public/launch-<slug>.md` | Staged launch sequence |
| `/slo-sales-funnel` | `docs/biz-public/sales/funnel-<segment>.md` | Outbound funnel math and cold-email structure |
| `/slo-pricing` | `docs/biz-public/pricing.md` | Pricing strategy and tier framing |
| `/slo-metrics consumer|b2b` | `docs/biz-public/metrics.md` | Financial and business KPI dashboard |
| `/slo-cofounder` | `docs/biz/cofounder/<name>.md` | Cofounder evaluation or trial framing |
| `/slo-hire swe|ae|designer|ops` | `docs/biz/hires/<role>-<name>.md` | Hiring artifact with IR35 gate |
| `/slo-founder-check` | `docs/biz/founder-check.md` | Founder self-assessment and runway worksheet |

## Vendored skills

| Skill | Purpose | Prerequisite |
|---|---|---|
| `/get-api-docs` | Fetch current third-party API docs via `chub` before coding against an external API | `npm install -g @aisuite/chub` |

## Shared invariants

- Every feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and follows `docs/runbook-template_v_3_template.md`.
- `README.md` is the orientation doc, `docs/getting-started.md` is the first-run guide, and this file is the host-neutral skill catalog.
- Host overlays must stay overlays. They can add session-specific guidance, but they should point back here instead of becoming competing catalogs.
- `references/biz/` and `references/sast/` are shared scaffolding trees. They are read by skills, but they are not skill directories.

## Current host boundaries

- Claude Code and GitHub Copilot can both consume the installed `SKILL.md` files.
- GitHub Copilot should be treated as an interactive host today, not a headless runtime target.
- `/slo-research` interactive use is host-neutral today; `sldo-research` remains an optional Claude batch backend.
- The live business judgment runtime harness is still Claude-only today.
