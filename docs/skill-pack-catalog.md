# Skill Pack Catalog

> **Status**: canonical living catalog of shipped SunLit Orchestra skills at HEAD.
> **Audience**: contributors, host-overlay authors, and users deciding which skill to run.

Use this file for the host-neutral list of shipped skills. Use [../CLAUDE.md](../CLAUDE.md) for the Claude Code overlay, [../copilot-instructions.md](../copilot-instructions.md) for the GitHub Copilot overlay, [../AGENTS.md](../AGENTS.md) for the Codex overlay, [getting-started.md](getting-started.md) for the first-run path, and [slo/design/agent-host-capabilities.md](slo/design/agent-host-capabilities.md) for current host support boundaries. Acronyms used here (TLA+, BDD, ICP, SEIS, IR35, ...) are defined in [GLOSSARY.md](GLOSSARY.md).

**Shipped skills at HEAD: 41** (10 sprint flow + 5 ticket flow + 10 power tools + 4 business advisor + 11 business generator + 1 vendored). Skills with mode variants (`/slo-product roadmap|metrics|okrs`, `/slo-marketing b2b|b2c`, `/slo-metrics consumer|b2b`, `/slo-hire swe|ae|designer|ops`) are one skill per row in their section, except `/slo-product` whose three modes are listed individually because the output paths differ. To reconcile against disk, run `ls skills/ | grep -v README` - should be 41 entries.

## Repo reality at HEAD

- The portable unit is the Markdown `SKILL.md` contract under `skills/<name>/SKILL.md`.
- The active Rust workspace members are `sldo-common`, `sldo-research`, `sldo-install`, and `xtasks/sast-verify`.
- The legacy `sldo-plan`, `sldo-run`, and `sldo-tauri` surfaces are not current workspace members and should not be treated as active interfaces.
- `sldo-install` can install the same skill pack into Claude Code, GitHub Copilot, or Codex.
- Headless runtime automation is still host-specific today. Check [design/agent-host-capabilities.md](design/agent-host-capabilities.md) before promising a runtime surface.

## Sprint flow

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `/slo-ideate` | YC-style product interrogation before any code |
| Research | `/slo-research` | Host-native interactive research first; optional Claude batch backend for sourced dossiers |
| Architect | `/slo-architect` | Stack + `ARCHITECTURE.md` + interfaces lock-in + `tla_required` / `kani_required` flags |
| Verify design | `/slo-tla` | TLC model-check the design when concurrency or ordering risk is real |
| Verify code | `/slo-kani` | Kani model-check small bounded Rust kernels (unsafe, arithmetic, invariants) when `kani_required`; code-level peer to `/slo-tla` |
| Plan | `/slo-plan` | Interactive v4 runbook authoring, one milestone at a time |
| Critique | `/slo-critique` | Four-persona adversarial review before execution |
| Execute | `/slo-execute M<N>` | Per-milestone driver with allow-list enforcement |
| Verify | `/slo-verify M<N>` | Runtime QA with Playwright for UI surfaces |
| Close | `/slo-retro M<N>` | Lessons + completion + tracker update |
| Ship | `/slo-ship` | Open a runbook-aware PR |

## Ticket-sized SLO flow

GitHub Issues-first path for small, reviewable work that should keep v4 rigor without a full multi-milestone runbook. The proposal and operating model live in [slo/design/ticket-sized-slo-workflow.md](slo/design/ticket-sized-slo-workflow.md). The contract template lives at [slo/templates/ticket-contract-template_v_1.md](slo/templates/ticket-contract-template_v_1.md). Ticket contracts keep compact parity with sprint contracts for reversibility, exemplar / anti-exemplar guidance, refactoring discipline, and AI tolerance, using N/A rows when a simple issue does not need them.

| Stage | Skill | Purpose |
|---|---|---|
| Pick | `/slo-ticket-pick` | Pull or claim one GitHub issue, normalize context, and create/update the issue workpad |
| Plan | `/slo-ticket-plan` | Write `docs/slo/tickets/ticket-<issue>-<slug>.md` from the v4-derived ticket contract template |
| Execute | `/slo-ticket-execute` | Implement the ticket contract BDD-first inside the exact file allow-list |
| Verify | `/slo-ticket-verify` | Run ticket-sized runtime QA, static/security checks, and regression-test-first bug handling |
| Close | `/slo-ticket-close` | Fill closure summary, open/update the PR, and move the issue to review without auto-merge |

## Power tools

| Skill | Purpose | Host story |
|---|---|---|
| `/slo-second-opinion` | Cross-model disagreement surfacer | Host-neutral. Compares the current host against an external provider (Codex / Gemini). |
| `/slo-freeze <path>` | Lock edits to one directory for the session | Host-neutral. |
| `/slo-resume` | Read the current runbook tracker and suggest the next move | Host-neutral. |
| `/slo-rulegen` | Bootstrap or extend Semgrep rule packs for Rust workspaces | Host-neutral. The bug-summary input can come from any agent-driven workflow. |
| `/slo-ruleverify` | Run the deterministic SAST gate over an existing rule pack | Host-neutral. |
| `/slo-sast` | Wire threat-model-driven SAST scanning into a target repo ([plain-language README](../skills/slo-sast/README.md)) | Host-neutral. Subprocess invocations are `git`, `gh`, and `semgrep` — never an agent CLI. |
| `/slo-dast-tuner` | Wire and tune zaprun-backed DAST for an authorized web app: SARIF-guided tuning, auth-aware coverage, PTK/DOM-XSS lane, the **SAST→DAST route bridge** + a **12-framework adapter catalog** with a generic fallback, and generic-rule boundaries ([plain-language README](../skills/slo-dast-tuner/README.md)) | Host-neutral. Operates ZAP only through `zaprun` and the latest approved digest-pinned zaprun image. |
| `/slo-sec-libs` | Read CycloneDX declarations, match proactive controls to advertised capabilities, and file confirmed capability gaps | Host-neutral. M1-M2 are read-only: offline Python `jsonschema` reader plus catalog-grounded matcher. M3 files regex-validated SLO-intake issues; M4 adds explicit upstream filing with per-issue confirmation and a 40-issues/hr session cap. |
| `/slo-nettacker` | Run authorized OWASP Nettacker assessment workflows, triage reports, and author safe custom Nettacker YAML modules | Host-neutral interactive skill. Uses local Nettacker CLI/Docker/API when available; no headless Codex/Copilot runtime harness is assumed. |
| `/slo-cloud-threat-model` | Author a scenario-driven AWS / GitHub / Cloudflare threat model (modernized SLO-native port of Hulumi's `hulumi-threat-model`, refreshed for the Hulumi v1.3.2 Edge Platform) | Host-neutral. Pure-Markdown skill; the only subprocess is an offline stdlib-only Python catalog lister/validator (argv-list, no network). Cites framework IDs + URLs only. Distinct from the `/slo-threat-model` provider tracked in #67. |

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

## Specialist agents (optional, host-native)

Additive host-native profiles that mirror `/slo-critique` and `/slo-verify` role boundaries. Claude Code uses `agents/*.md`; GitHub Copilot has bounded counterparts under `.github/agents/*.agent.md`. Output paths are constrained to `docs/slo/critique/` and `docs/slo/verify/` - same artifact paths the canonical portable path writes. Codex users continue to use `/slo-critique` and `/slo-verify` directly.

| Role | Claude profile | Copilot profile | Output paths | Portable path |
|---|---|---|---|---|
| Lead runbook review | `agents/slo-runbook-review-lead.md` | `.github/agents/slo-runbook-review-lead.agent.md` | `docs/slo/critique/` | `/slo-critique` persona rotation |
| Security review | `agents/slo-security-reviewer.md` | `.github/agents/slo-security-reviewer.agent.md` | `docs/slo/critique/` | `/slo-critique` security persona |
| Design review | `agents/slo-design-reviewer.md` | `.github/agents/slo-design-reviewer.agent.md` | `docs/slo/critique/` | `/slo-critique` design persona |
| Verification review/runtime QA | `agents/slo-verification-lead.md` | `.github/agents/slo-verification-lead.agent.md` | `docs/slo/critique/`, `docs/slo/verify/` | `/slo-verify` |

See [`docs/slo/design/host-capability-matrix.md`](slo/design/host-capability-matrix.md) for the green-lit decision and host capability rationale.

## Examples gallery

Synthetic, non-normative gallery at [`examples/`](../examples/) shows what shipped SLO outputs look like — runbook excerpts, critique reports, verification reports, security findings, SAST manifests, and biz-public artifacts. Read these to calibrate quality before running a skill. Examples are not installable; not consumed by any skill.

## Distribution channels

- `sldo-install` (canonical, multi-host: Claude Code + GitHub Copilot + Codex).
- `.claude-plugin/plugin.json` (optional, additive, Claude-only) — for organizational installs that prefer a one-zip distribution.
- Tagged releases: SHA-pinned [`release-zip workflow`](../.github/workflows/release-zip.yml) generates a `git archive`-based release zip on `v*` tag push.

## Shared invariants

- Every new feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and follows `docs/slo/templates/runbook-template_v_4_template.md` (v3 remains in place as the historical artifact for runbooks already authored against it).
- Every ticket-sized issue contract lives at `docs/slo/tickets/ticket-<issue>-<slug>.md` and follows `docs/slo/templates/ticket-contract-template_v_1.md`, which preserves the v4 Contract Block, BDD, evidence, static-analysis, assertion, and resource-bound gates in compact form.
- `references/agent/operating-contract.md` is the shared host-neutral operating contract for AI coding agents. Keep it small and route detailed procedures through installed SLO skills.
- `README.md` is the orientation doc, `docs/getting-started.md` is the first-run guide, and this file is the host-neutral skill catalog.
- Host overlays (`CLAUDE.md`, `copilot-instructions.md`, `AGENTS.md`) must stay overlays. They can add session-specific guidance, but they should point back here instead of becoming competing catalogs.
- Host install docs must distinguish SLO installer compatibility root paths (`.copilot/skills`, `.codex/skills`) from current official host-native project skill roots such as `.github/skills` and `.agents/skills`.
- Host-native agent/profile docs must keep the canonical portable fallbacks visible: `/slo-critique` and `/slo-verify` remain the cross-host paths, and Copilot profiles are not a SLO headless runtime harness.
- `references/biz/`, `references/security/`, and `references/sast/` are shared scaffolding trees. They are read by skills, but they are not skill directories.

## Current host boundaries

- Claude Code, GitHub Copilot, and Codex can consume the installed `SKILL.md` files.
- GitHub Copilot and Codex should be treated as interactive hosts today, not headless runtime targets.
- `/slo-research` interactive use is host-neutral today; `sldo-research` remains an optional Claude batch backend.
- The live business judgment runtime harness is still Claude-only today.
