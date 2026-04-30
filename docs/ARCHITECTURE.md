# SunLitOrchestrate Architecture

> **Reality-first orientation doc**: this file describes what is implemented at HEAD. Planned work belongs in `docs/design/*.md` and in feature runbooks.

## Overview

SunLitOrchestrate ships three cooperating layers:

1. A Markdown skill pack under `skills/`.
2. A host-aware installer in `crates/sldo-install/`.
3. Supporting Rust tooling in `crates/sldo-common/`, `crates/sldo-research/`, and `xtasks/sast-verify/`.

The repo no longer ships `sldo-plan`, `sldo-run`, or `sldo-tauri` as active workspace members. The living surfaces at HEAD are the skill pack, the installer, and the remaining support crates listed above.

## Living docs

| Document | Role |
|---|---|
| `README.md` | Repo orientation for humans browsing the project |
| `docs/getting-started.md` | First-run guide with exact install and first-use steps |
| `docs/skill-pack-catalog.md` | Canonical living catalog of shipped skills |
| `CLAUDE.md` | Claude Code overlay for the catalog |
| `copilot-instructions.md` | GitHub Copilot overlay for the catalog |
| `docs/design/agent-host-capabilities.md` | Capability matrix for install, interactive use, and runtime boundaries |

## Skill pack

The skill pack is the primary user-facing product. Each skill lives in `skills/<name>/SKILL.md` and is installed into a host-specific skills directory by `sldo-install`.

### Skill-pack surfaces at HEAD

| Surface | Location | What ships today |
|---|---|---|
| Sprint flow | `skills/slo-*` | Ideate → research → architect → plan → critique → execute → verify → retro → ship |
| Business advisor pack | `skills/slo-{legal,accounting,equity,fundraise}` | UK-only advisor flows with hard-block routing |
| Business generator pack | `skills/slo-{talk-to-users,gtm,product,marketing,launch,sales-funnel,pricing,metrics,cofounder,hire,founder-check}` | Artifact generators for discovery, GTM, product, finance, hiring, and founder ops |
| Security and SAST helpers | `skills/slo-{rulegen,ruleverify,sast}` | Semgrep rule generation, verification, and SAST wiring |
| Utilities | `skills/slo-{freeze,resume,second-opinion}` | Session control, resumption, and disagreement surfacing |
| Vendored helper | `skills/get-api-docs` | Third-party API doc fetches via `chub` |

For the full host-neutral skill inventory, read `docs/skill-pack-catalog.md`.

## Skill pack invariants (reality at HEAD)

- **Markdown-only skill contract.** The portable unit is `skills/<name>/SKILL.md`.
- **Canonical catalog plus host overlays.** `docs/skill-pack-catalog.md` is the shared catalog. `CLAUDE.md` and `copilot-instructions.md` are overlays, not competing sources of truth.
- **Canonical planning artifact.** Every feature runbook is `docs/RUNBOOK-<FEATURE>.md` and follows `docs/runbook-template_v_3_template.md`.
- **Reality-first ARCHITECTURE.md.** This file records implemented surfaces only.
- **Host-aware installer roots.** Global installs land in `~/.claude/skills/` or `~/.copilot/skills/`. Local installs land in `./.claude/skills/` or `./.copilot/skills/`.
- **Shared manifest with explicit host ownership.** `~/.sldo/install.toml` stores install records by host so `status`, `verify`, and `uninstall` stay scoped.
- **Baseline test command.** `cargo test -p sldo-common -p sldo-install -p sldo-research`.
- **Current runtime boundary.** GitHub Copilot is an interactive host today, not a headless runtime target.

### References subtrees

- `references/biz/` holds shared business-pack scaffolding such as gates, jurisdiction notes, templates, and regulator indexes.
- `references/sast/` holds SAST-specific references consumed by the security tooling and rule-pack work.
- These trees are read by skills, but they are not discovered as installable skills because `sldo-install` only walks `skills/<name>/SKILL.md`.

## Rust workspace

The current workspace has four active members:

| Member | Role |
|---|---|
| `crates/sldo-common` | Shared library used by the remaining Rust tools |
| `crates/sldo-research` | Optional Claude batch backend for sourced dossier generation |
| `crates/sldo-install` | Host-aware installer for the skill pack |
| `xtasks/sast-verify` | Deterministic Semgrep validation, coverage, and gate runner |

The root package `sunlit-orchestrate-tests` hosts workspace-level integration tests in `tests/`.

## Shared library: `sldo-common`

`crates/sldo-common/src/lib.rs` currently exports these modules:

| Module | Responsibility |
|---|---|
| `claude_cli` | Claude-CLI invocation helper used by the optional Claude batch backend in `sldo-research`. Explicitly Claude-only — there is no host-neutral runtime abstraction. |
| `color` | Small color and output helpers |
| `detect` | Environment and tool detection helpers |
| `git` | Git inspection helpers |
| `logging` | Logging setup and formatting |
| `preflight` | Pre-run checks for required binaries and environment state, including the Claude-CLI presence check used by `sldo-research`. |
| `runbook` | Shared runbook parsing and validation helpers |
| `toolflags` | Shared allow-flag definitions and related helpers |

## Research backend: `sldo-research`

`crates/sldo-research/` is the Rust backend for the `/slo-research` skill. Its source is organized into:

| File | Responsibility |
|---|---|
| `main.rs` | CLI entrypoint |
| `research.rs` | Research orchestration |
| `prompt.rs` | Prompt construction |
| `dossier.rs` | Dossier assembly and related helpers |

The installed skill is host-neutral for interactive use. `sldo-research` is the optional Claude batch backend for users who explicitly want automated dossier generation from a Claude-backed CLI flow.

## Installer: `sldo-install`

`crates/sldo-install/` is the bridge between the repo and host-specific skill directories.

| File | Responsibility |
|---|---|
| `main.rs` | CLI parsing and command dispatch |
| `host.rs` | Host descriptor table (`claude-code`, `github-copilot`) |
| `paths.rs` | Host-specific global and local path resolution |
| `manifest.rs` | Shared install manifest with per-host ownership |
| `install.rs` | Install, verify, status, and uninstall behavior |

## SAST xtask: `xtasks/sast-verify`

`xtasks/sast-verify/` is the deterministic Semgrep verification toolchain used by the SAST skill work.

| File | Responsibility |
|---|---|
| `main.rs` | Subcommand entrypoint |
| `validate.rs` | `semgrep --validate` wrapper |
| `test_cmd.rs` | `semgrep --test` wrapper |
| `check_coverage.rs` | Coverage checks |
| `check_clean.rs` | Clean-tree checks for generated fixtures |
| `gate.rs` | Composes the full deterministic gate |
| `tier_detect.rs` | Rule-tier detection |
| `semgrep_runner.rs` | Shared Semgrep invocation plumbing |
| `validate_file_paths.rs` | Input path checks |
| `yaml_schema.rs` | YAML/schema helpers |

## Feedback loops

The skill pack improves itself through cyclic feedback structures that are not visible in a static dependency diagram. They are documented separately so newcomers and freshly-loaded Claude instances can answer "which loop am I in, and what do I run next?" in 90 seconds.

- [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md) — sprint loop, security-tuning loop, lessons loop, library-feedback loop.
- [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md) — user-interview loop, GTM loop, pricing loop, founder-check loop.

The lessons loop is the canonical example: `/slo-retro` writes `docs/lessons/<prefix>-m<N>.md` at every milestone close, classifies each lesson, dedupes via `gh search`, and files tracked issues with explicit user confirmation (rules locked in [`skills/slo-retro/references/issue-filing-discipline.md`](../skills/slo-retro/references/issue-filing-discipline.md)); `/slo-execute` pre-flight Step 1.5 then queries open `retro-derived` issues for the runbook's prefix and surfaces them as scope candidates with a suggested lane (`micro | milestone | fresh-runbook`); `/slo-resume` compresses the result back to one screen.

## Current host boundaries

The current host line is simple:

- Install support is multi-host.
- The catalog and the `SKILL.md` contract are host-neutral.
- Interactive skill use is supported in Claude Code and GitHub Copilot.
- Headless runtime automation is still Claude-specific where it exists today.
- `/slo-research` interactive use is multi-host today; `sldo-research` remains an optional Claude batch backend.
- `/slo-second-opinion` is host-neutral: it compares the current host against an external provider CLI (Codex or Gemini), and never silently falls back to asking the current host to imitate the other provider.
- `/slo-rulegen` and `/slo-sast` are host-neutral; their subprocess discipline targets `git`, `gh`, and `semgrep` rather than any agent CLI.
- The live business judgment runtime harness remains a Claude-only path. The helper module (`crates/sldo-install/tests/common/claude_runtime.rs`) and its env vars (`BIZ_JUDGMENT_RUNTIME_*`) are explicitly Claude-named.

Read `docs/design/agent-host-capabilities.md` before making any stronger host-compatibility promise than that.
