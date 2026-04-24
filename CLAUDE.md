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
