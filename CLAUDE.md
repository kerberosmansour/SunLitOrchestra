# SunLitOrchestrate — Claude Code overlay

This file is the Claude Code overlay for the canonical living catalog at [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md). Use it when you are working in Claude Code and need Claude-specific session notes. For the host-neutral list of shipped skills, read the catalog first. For GitHub Copilot-specific notes, read [copilot-instructions.md](copilot-instructions.md).

## Skill pack — first-party `/slo-*` skills

Sprint flow: Think → Plan → Build → Review → Test → Ship → Reflect.

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `/slo-ideate` | YC-style product interrogation before any code |
| Research | `/slo-research` | Host-native research first; optional Claude batch backend via `sldo-research` |
| Architect | `/slo-architect` | Stack + ARCHITECTURE.md + interfaces lock-in + `tla_required` flag |
| Verify design | `/slo-tla` | TLC model-check the design (when `tla_required: true`) |
| Plan | `/slo-plan` | Interactive v3 runbook authoring, one milestone at a time |
| Critique | `/slo-critique` | Four-persona adversarial review (CEO, eng, security, design) |
| Execute | `/slo-execute M<N>` | Per-milestone driver with allow-list enforcement |
| Verify | `/slo-verify M<N>` | Runtime QA with Playwright for UI surfaces |
| Close | `/slo-retro M<N>` | Lessons + completion + tracker update |
| Ship | `/slo-ship` | Open PR with runbook-aware description |

Power tools:

| Skill | Purpose |
|---|---|
| `/slo-second-opinion` | Cross-model disagreement surfacer (Codex / Gemini) |
| `/slo-freeze <path>` | Lock edits to one directory for the session |
| `/slo-resume` | Read current runbook's tracker, suggest next step |

## Biz skill pack — first-party advisor + generator skills

UK-only (v1). Advisor skills (4) operate as `draft | translate | triage | prepare` modes with hard-block gates from [references/biz/triage-gate.md](references/biz/triage-gate.md) (regulated / >£5,000 / counterparty-with-lawyer / GDPR). Generator skills (11; shipped in Runbooks B1, B2, C — out of Runbook A scope) produce one artifact each.

| Skill | Archetype | Domain | Cites |
|---|---|---|---|
| `/slo-legal` | advisor | UK legal — NDA, contractor SOW, IP assignment, T&Cs | [references/biz/templates/onenda-uk.md](references/biz/templates/onenda-uk.md) (CC BY-ND 4.0 verbatim), [references/biz/cost-baseline-jpp-law-2026.md](references/biz/cost-baseline-jpp-law-2026.md) |
| `/slo-accounting` | advisor | UK accounting — bookkeeping, VAT, R&D credit, MTD | HMRC route default = accountant (per [references/biz/jurisdiction-uk.md](references/biz/jurisdiction-uk.md) UK regulator index) |
| `/slo-equity` | advisor | UK equity — cofounder split, vesting, cap-table snapshot | [references/biz/hmrc-vcm-index.md](references/biz/hmrc-vcm-index.md) (VCM34080 / VCM3000 / VCM31000 + Abingdon Health line) — runs SEIS / EIS pre-check on every draft |
| `/slo-fundraise` | advisor | UK fundraise — SAFE math, pitch narrative, term-sheet redline brief | HMRC VCM index + [references/biz/ir35-cest-factors.md](references/biz/ir35-cest-factors.md) — runs Advance Assurance pre-check on every interaction; refuses term-sheet drafting without AA ≥ 6 weeks ahead |
| `/slo-talk-to-users` | generator | UK user-interview prep + post-interview extraction (Mom Test discipline); single-mode with `mode_arg: pre-interview \| post-interview`; output `docs/biz/users/<date>-<name>.md` (confidential) | Lands `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/` (Runbook B1 M1) |
| `/slo-gtm` | generator | UK GTM strategy — ICP / segmentation (3-segment cap) / motion choice (PLG \| sales-led \| community-led \| hybrid) / channel strategy / KPI alignment; output `docs/biz-public/gtm/strategy.md` | Routes direct-marketing channels to `/slo-legal triage` for PECR considerations |
| `/slo-product` | generator | UK PM artifacts (mode_arg `roadmap \| metrics \| okrs`); PM-side metrics only (DAU / activation / retention / feature adoption); 3-objective OKR cap | Redirects financial KPIs (CAC / LTV / NDR / burn multiple) to `/slo-metrics` (Runbook B2) |
| `/slo-marketing` | generator | UK marketing tactics (mode_arg `b2b \| b2c`); brand voice / content calendar / channel mix / demand gen / paid acq | Routes ALL direct-marketing implementation to `/slo-legal triage` for DUAA 2025 PECR (£17.5M ceiling); B2C flags ASA / CAP Code disclosure + CRA 2015 |
| `/slo-launch` | generator | UK launch sequence — pitch validator + 4-stage launch (silent → F&F → communities → press) with kill / delay rules per stage; output `docs/biz-public/launch-<slug>.md` | Routes any direct-marketing implementation in stage 4 to `/slo-legal triage` for PECR; readiness checklist gates broader-press launch |
| `/slo-sales-funnel` | generator | UK outbound funnel math + cold-email template (7 outbound principles) + deal structure (paid trial → recurring → opt-out); output `docs/biz-public/sales/funnel-<segment>.md` | Routes cold email to `/slo-legal triage` for PECR (gate-4-gdpr-document fires under DUAA Stage 3) |
| `/slo-pricing` | generator | UK pricing strategy — value equation (price = 25-33% of value delivered) + 3-tier-max model + canonical "increase price by 50%" experiment framing; output `docs/biz-public/pricing.md` | Routes SEIS/EIS revenue-mix considerations to `/slo-fundraise triage` for VCM3000 qualifying-trade check |
| `/slo-metrics` | generator | UK financial KPI dashboard (mode_arg `consumer \| b2b`); CAC / LTV / NDR / MoM revenue / burn multiple / gross margin / runway / ARR; output `docs/biz-public/metrics.md` | Redirects PM-side metrics (DAU / activation / retention / feature-adoption) to `/slo-product metrics` (Runbook B1 M3) |
| `/slo-cofounder` | generator | UK cofounder evaluation + 4-week paid trial framing + monthly 1:1 agenda; output `docs/biz/cofounder/<name>.md` (confidential — real persons) | Routes equity-split to `/slo-equity` (A M3); routes active disputes to a mediator + `/slo-legal triage` |
| `/slo-hire` | generator | UK hiring artifact with mode_arg = role-shape `swe \| ae \| designer \| ops`; sourcing + interview rubric + offer cadence + onboarding; output `docs/biz/hires/<role>-<name>.md` (confidential) | MANDATORY IR35 triage gate per `references/biz/ir35-cest-factors.md` — runs the seven hard-block triggers BEFORE offer; rejects "call them a contractor for tax efficiency" framing |
| `/slo-founder-check` | generator | UK founder self-assessment — 12-question check + worst-case-runway worksheet (4 cost-cut tiers) + optional YC application prep; output `docs/biz/founder-check.md` (confidential — highly personal) | Routes mental-health to professional support; routes personal-finance to StepChange / IFA; routes wind-down questions to `/slo-legal triage` |

Shared scaffolding lives at `references/biz/` at the repo root (NOT under `skills/` — `crates/sldo-install/src/install.rs:44-71`'s `discover_skills()` ignores it). Two-tier output: `docs/biz/` (gitignored, confidential drafts with real PII / deal terms) and `docs/biz-public/` (git-tracked, placeholder / decision artifacts). See [docs/slo/design/biz-skill-pack-overview.md](docs/slo/design/biz-skill-pack-overview.md) for the full design.

GDPR posture: **broad hard-block on `draft`** for all GDPR documents (privacy notice, ROPA, DPA, internal policies). Locked 2026-04-25. Reversal requires fresh `/slo-architect` pass. Cost baseline: JPP Law fixed-fee public pricing, locked 2026-04-25.

PII discipline: every biz skill writing to `docs/biz/` issues a write-time warning when the target dir is git-tracked AND a remote exists AND `tier: confidential`. Second-line defense: `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/` (added Runbook B1 M1) — flags email / UK NI / sort code / capitalised-bigram patterns with `pii_scan_override: true` + `tier_override_reason: <one-liner>` frontmatter override mechanism.

## Third-party skills vendored

| Skill | Purpose | Prereq |
|---|---|---|
| `/get-api-docs` | Fetch current third-party API docs via `chub` CLI | `npm install -g @aisuite/chub` |

See [skills/get-api-docs/UPSTREAM.md](skills/get-api-docs/UPSTREAM.md) for attribution.

## Specialist agents (optional, Claude-only — sap-imp M5)

Four host-native specialist agents under [agents/](agents/) provide an additive Claude-Code-only critique flow: `slo-runbook-review-lead`, `slo-security-reviewer`, `slo-design-reviewer`, `slo-verification-lead`. Output paths are constrained to `docs/slo/critique/` and `docs/slo/verify/` (same paths `/slo-critique` and `/slo-verify` write to). The structural-contract test in `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` enforces frontmatter, output-path safety, and `/slo-critique` SKILL.md SHA-256 byte-identical baseline.

`/slo-critique` persona rotation remains the canonical portable critique path. Agents are an optional enhancement; the canonical flow is preserved on every host.

## Examples gallery

Synthetic, non-normative gallery at [examples/](examples/). Read these to see what shipped SLO outputs look like before running a skill.

## Distribution channels

- `sldo-install` (canonical) — installs into `~/.claude/skills/` (or `--local`).
- `.claude-plugin/plugin.json` (optional, additive) — Claude Code organizational installs may prefer a one-zip distribution. Tagged releases produce a downloadable zip via the SHA-pinned [release-zip workflow](.github/workflows/release-zip.yml).

## Canonical planning artifact

Every new feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and follows [docs/slo/templates/runbook-template_v_4_template.md](docs/slo/templates/runbook-template_v_4_template.md). This is the v4 template — the output contract of `/slo-plan` — which adds Carmack-style reliability controls (debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, "make invalid states unrepresentable") on top of v3's SunLit-specific structure (carry-forward from prior retros, abuse-acceptance scenarios, threat-model integration). Do not bypass it for batch CLI shortcuts when interactive planning is an option. [docs/slo/templates/runbook-template_v_3_template.md](docs/slo/templates/runbook-template_v_3_template.md) remains in place as the historical artifact for runbooks already authored against v3.

## Baseline test command (this repo)

```bash
cargo test -p sldo-common -p sldo-install -p sldo-research
```

The workspace contains four crates: `sldo-common` (shared library), `sldo-research` (optional Claude batch backend for `/slo-research`), `sldo-install` (skill installer), and `xtasks/sast-verify` (Semgrep rule gate driven by `/slo-rulegen` + `/slo-ruleverify`). All other Rust code (the legacy `sldo-plan` / `sldo-run` CLIs, the parked `sldo-tauri` desktop UI, the `sldo-tla-sha` maintenance utility) was removed in the 2026-04 cleanup — the skills are the canonical interface now.

## Installing the pack on this machine

From the repo root:

```bash
cargo build -p sldo-install --release
./target/release/sldo-install              # global: ~/.claude/skills/
./target/release/sldo-install --dry-run    # preview
./target/release/sldo-install uninstall    # reverse
```

Manifest: `~/.sldo/install.toml`.

If you are using GitHub Copilot instead of Claude Code, use [copilot-instructions.md](copilot-instructions.md) for the matching host overlay.

If you are completely new to the repo, start with [docs/getting-started.md](docs/getting-started.md) before using this overlay.
