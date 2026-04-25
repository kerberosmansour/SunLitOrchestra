# SunLitOrchestrate — Claude Code project notes

This repo hosts the SunLitOrchestrate (SLO) skill pack for Claude Code. When you are working in this repo, the skills below are available via `sldo-install`.

## Skill pack — first-party `/slo-*` skills

Sprint flow: Think → Plan → Build → Review → Test → Ship → Reflect.

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `/slo-ideate` | YC-style product interrogation before any code |
| Research | `/slo-research` | Wraps `sldo-research` Rust backend for sourced dossiers |
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

Shared scaffolding lives at `references/biz/` at the repo root (NOT under `skills/` — `crates/sldo-install/src/install.rs:44-71`'s `discover_skills()` ignores it). Two-tier output: `docs/biz/` (gitignored, confidential drafts with real PII / deal terms) and `docs/biz-public/` (git-tracked, placeholder / decision artifacts). See [docs/design/biz-skill-pack-overview.md](docs/design/biz-skill-pack-overview.md) for the full design.

GDPR posture: **broad hard-block on `draft`** for all GDPR documents (privacy notice, ROPA, DPA, internal policies). Locked 2026-04-25. Reversal requires fresh `/slo-architect` pass. Cost baseline: JPP Law fixed-fee public pricing, locked 2026-04-25.

## Third-party skills vendored

| Skill | Purpose | Prereq |
|---|---|---|
| `/get-api-docs` | Fetch current third-party API docs via `chub` CLI | `npm install -g @aisuite/chub` |

See [skills/get-api-docs/UPSTREAM.md](skills/get-api-docs/UPSTREAM.md) for attribution.

## Canonical planning artifact

Every feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and follows [docs/runbook-template_v_3_template.md](docs/runbook-template_v_3_template.md). This template is the output contract of `/slo-plan`; do not bypass it for batch CLI shortcuts when interactive planning is an option.

## Baseline test command (this repo)

```bash
cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install
```

The `--workspace` baseline is NOT used because the parked `sldo-tauri` crate leaves it red on macOS arm64 (esbuild arm64 binary missing in its UI `node_modules/`). If Tauri is un-parked, restore the full-workspace baseline.

## Parked work — `crates/sldo-tauri/`

The Tauri desktop UI is parked as of 2026-04. Do NOT modify it; do NOT merge its branch into skill-pack work. Revisit only if there's a concrete user pulling for it.

## Installing the pack on this machine

From the repo root:

```bash
cargo build -p sldo-install --release
./target/release/sldo-install              # global: ~/.claude/skills/
./target/release/sldo-install --dry-run    # preview
./target/release/sldo-install uninstall    # reverse
```

Manifest: `~/.sldo/install.toml`.
