# SunLit Orchestra Architecture

> **Reality-first orientation doc**: this file describes what is implemented at HEAD. Planned work belongs in `docs/slo/design/*.md` and in feature runbooks.

## Overview

SunLit Orchestra ships three cooperating layers:

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
| `AGENTS.md` | Codex overlay for the catalog |
| `docs/slo/design/agent-host-capabilities.md` | Capability matrix for install, interactive use, and runtime boundaries |

## Skill pack

The skill pack is the primary user-facing product. Each skill lives in `skills/<name>/SKILL.md` and is installed into a host-specific skills directory by `sldo-install`.

### Skill-pack surfaces at HEAD

| Surface | Location | What ships today |
|---|---|---|
| Sprint flow | `skills/slo-*` | Ideate → research → architect → plan → critique → execute → verify → retro → ship |
| Ticket-sized SLO flow | `skills/slo-ticket-*` | GitHub issue → compact ticket contract → execute → verify → PR handoff |
| Business advisor pack | `skills/slo-{legal,accounting,equity,fundraise}` | UK-only advisor flows with hard-block routing |
| Business generator pack | `skills/slo-{talk-to-users,gtm,product,marketing,launch,sales-funnel,pricing,metrics,cofounder,hire,founder-check}` | Artifact generators for discovery, GTM, product, finance, hiring, and founder ops |
| Security and SAST helpers | `skills/slo-{rulegen,ruleverify,sast,sec-libs}` | Semgrep rule generation, verification, SAST wiring, and CycloneDX declarations reading |
| Utilities | `skills/slo-{freeze,resume,second-opinion}` | Session control, resumption, and disagreement surfacing |
| Vendored helper | `skills/get-api-docs` | Third-party API doc fetches via `chub` |
| Examples gallery | `examples/` | Synthetic, non-normative gallery (7 files) showing what shipped SLO outputs look like — read [`examples/README.md`](../examples/README.md). Not installable; not consumed by any skill. |
| Specialist agents (optional, Claude-only) | `agents/slo-{runbook-review-lead,security-reviewer,design-reviewer,verification-lead}.md` | Host-native agent files for Claude Code that mirror `/slo-critique` persona rotation. Output paths constrained to `docs/slo/critique/` and `docs/slo/verify/`. GitHub Copilot and Codex users use `/slo-critique` directly (canonical portable path). See [`docs/slo/design/host-capability-matrix.md`](slo/design/host-capability-matrix.md). |
| Distribution channels | `sldo-install` (canonical, multi-host) + optional `.claude-plugin/plugin.json` (Claude-only, additive) | Tagged releases produce a downloadable zip via the SHA-pinned [`release-zip workflow`](../.github/workflows/release-zip.yml). |

For the full host-neutral skill inventory, read `docs/skill-pack-catalog.md`.

## Skill pack invariants (reality at HEAD)

- **Markdown-only skill contract.** The portable unit is `skills/<name>/SKILL.md`.
- **Canonical catalog plus host overlays.** `docs/skill-pack-catalog.md` is the shared catalog. `CLAUDE.md`, `copilot-instructions.md`, and `AGENTS.md` are overlays, not competing sources of truth.
- **Canonical planning artifact.** Every new feature runbook is `docs/RUNBOOK-<FEATURE>.md` and follows `docs/slo/templates/runbook-template_v_4_template.md` (v3 remains in place as the historical artifact for runbooks authored against it).
- **Ticket-sized planning artifact.** Every bite-sized GitHub issue contract lives at `docs/slo/tickets/ticket-<issue>-<slug>.md` and follows `docs/slo/templates/ticket-contract-template_v_1.md`. The template stays compact while mirroring sprint-flow reversibility, exemplar / anti-exemplar, refactoring discipline, and AI tolerance rows with N/A paths.
- **Reality-first ARCHITECTURE.md.** This file records implemented surfaces only.
- **Host-aware installer roots.** Global installs land in `~/.claude/skills/`, `~/.copilot/skills/`, or `~/.codex/skills/`. Local installs land in `./.claude/skills/`, `./.copilot/skills/`, or `./.codex/skills/`.
- **Cross-platform installer behavior.** Linux and macOS use directory symlinks. Windows tries directory symlinks first and falls back to directory junctions when symlink privileges are unavailable. Home resolution supports `HOME`, `USERPROFILE`, and `HOMEDRIVE` + `HOMEPATH`.
- **Shared manifest with explicit host ownership.** `~/.sldo/install.toml` stores install records by host so `status`, `verify`, and `uninstall` stay scoped.
- **Baseline test command.** `cargo test -p sldo-common -p sldo-install -p sldo-research`.
- **Current runtime boundary.** GitHub Copilot and Codex are interactive hosts today, not headless runtime targets.

### References subtrees

- `references/biz/` holds shared business-pack scaffolding such as gates, jurisdiction notes, templates, and regulator indexes.
- `references/security/` holds shared security finding and assessment summary templates used by review / verification skills, plus the curated CWE × OWASP × ASVS × OpenCRE table at [`references/security/standards-mapping.md`](../references/security/standards-mapping.md) (added by sap-imp M3).
- `references/sast/` holds SAST-specific references consumed by the security tooling and rule-pack work.
- `references/templates/` holds shared cross-skill discipline templates for citation hierarchy, intake, restate-and-confirm, tool safety, output frontmatter, escalation, eval cases, heuristic numbers, rate limiting, fallback handling, and version pinning.
- `skills/<skill>/references/` holds skill-local methodology files that travel with the installed skill symlink; `/slo-sast` uses this pattern for its M1-M5 operating procedures, `/slo-sec-libs` uses it for declaration-reader methodology, `/slo-tla` uses it for elicitation / abstraction / counterexample / verified-design guidance, and `/slo-plan` uses it for per-milestone authoring.
- These trees are read by skills, but they are not discovered as installable skills because `sldo-install` only walks `skills/<name>/SKILL.md`.

### Hooks and evals

- High-risk skills may carry documented expectations under `skills/<skill>/evals/*.md`. These are Markdown cases for manual checks today and for a future runtime harness later; the shared case shape lives in `references/templates/eval-cases.md`.
- The project-local Claude Code freeze hook lives in `.claude/settings.json`. It is opt-in, watches `Edit|Write|NotebookEdit`, and reads `~/.sldo/freeze-scope.txt` to block edits outside the active `/slo-freeze` scope.
- Hook setup guidance lives in `references/freeze/hook-setup.md`. The hook is a guardrail for accidental edits, not a security boundary; deleting the session-state file disables enforcement.

## Rust workspace

The current workspace has four active members:

| Member | Role |
|---|---|
| `crates/sldo-common` | Shared library used by the remaining Rust tools |
| `crates/sldo-research` | Optional Claude batch backend for sourced dossier generation |
| `crates/sldo-install` | Host-aware installer for the skill pack |
| `xtasks/sast-verify` | Deterministic Semgrep validation, coverage, and gate runner |

The root package `sunlit-orchestra-tests` hosts workspace-level integration tests in `tests/`.

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
| `host.rs` | Host descriptor table (`claude-code`, `github-copilot`, `codex`) |
| `paths.rs` | Host-specific global and local path resolution |
| `manifest.rs` | Shared install manifest with per-host ownership |
| `install.rs` | Install, verify, status, and uninstall behavior |

## SAST xtask: `xtasks/sast-verify`

`xtasks/sast-verify/` is the deterministic Semgrep verification toolchain used by the SAST skill work. It also hosts Markdown structural-contract tests added by feature runbooks.

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

### Structural-contract test families

Feature runbooks land Markdown / YAML / JSON structural-contract tests under `xtasks/sast-verify/tests/`. They walk shipped artifacts at HEAD (skills, references, examples, workflows, agents) and assert documented invariants. Each milestone of a runbook owns one test file named `<prefix>_m<N>_<feature>.rs`.

| Test family | Asserts |
|---|---|
| `sap_imp_m1_citations` | `pulldown-cmark` AST-based: every security-relevant skill cites a shared template; cited paths resolve at HEAD; `/slo-ship` security-summary section is gated by "new public surface" phrase; no shipped SKILL.md links to `examples/`. |
| `sap_imp_m2_examples` | `examples/` contains exactly 7 synthetic, non-normative artifacts; PII regex scan zero matches across email + UK NI + UK sort code + US SSN + EU IBAN; every `abbreviates:` resolves; ≤ 10 KB per file. |
| `sap_imp_m3_standards` | `references/security/standards-mapping.md` has dated rows; 4 target skills cite the mapping; threshold rule (high/critical → CWE) is documented in `/slo-critique` and `/slo-verify`; live `docs/slo/{critique,verify}/*.md` walked for the threshold rule. |
| `sap_imp_m4_workflow_pinning` | Every workflow `uses:` SHA-pinned (40-char hex) and has explicit `permissions:` block; `host-capability-matrix.md` carries a decision row; plugin.json has no path traversal; release workflow uses `git archive` + tag-trigger only. |
| `sap_imp_m5_agents` | Exactly 4 agent files; frontmatter complete; `output-paths` constrained to `{docs/slo/critique/, docs/slo/verify/}` with traversal/absolute-path rejected; `copilot-fallback` non-empty; ≤ 200 lines per agent; `skills/slo-critique/SKILL.md` SHA-256 byte-identical to pinned baseline. |

## Feedback loops

The skill pack improves itself through cyclic feedback structures that are not visible in a static dependency diagram. They are documented separately so newcomers and freshly-loaded Claude instances can answer "which loop am I in, and what do I run next?" in 90 seconds.

- [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md) — sprint loop, ticket loop, security-tuning loop, lessons loop, library-feedback loop.
- [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md) — user-interview loop, GTM loop, pricing loop, founder-check loop.

The lessons loop is the canonical example: `/slo-retro` writes `docs/slo/lessons/<prefix>-m<N>.md` at every milestone close, classifies each lesson, dedupes via `gh search`, and files tracked issues with explicit user confirmation (rules locked in [`skills/slo-retro/references/issue-filing-discipline.md`](../skills/slo-retro/references/issue-filing-discipline.md)); `/slo-execute` pre-flight Step 1.5 then queries open `retro-derived` issues for the runbook's prefix and surfaces them as scope candidates with a suggested lane (`micro | milestone | fresh-runbook`); `/slo-resume` compresses the result back to one screen.

## Current host boundaries

The current host line is simple:

- Install support is multi-host.
- The catalog and the `SKILL.md` contract are host-neutral.
- Interactive skill use is supported in Claude Code, GitHub Copilot, and Codex.
- Headless runtime automation is still Claude-specific where it exists today.
- `/slo-research` interactive use is multi-host today; `sldo-research` remains an optional Claude batch backend.
- `/slo-second-opinion` is host-neutral: it compares the current host against an external provider CLI (Codex or Gemini), and never silently falls back to asking the current host to imitate the other provider.
- `/slo-rulegen`, `/slo-sast`, and `/slo-sec-libs` are host-neutral; their subprocess discipline targets local tools such as `git`, `gh`, `python3`, and `semgrep` rather than any agent CLI.
- The live business judgment runtime harness remains a Claude-only path. The helper module (`crates/sldo-install/tests/common/claude_runtime.rs`) and its env vars (`BIZ_JUDGMENT_RUNTIME_*`) are explicitly Claude-named.

Read `docs/slo/design/agent-host-capabilities.md` before making any stronger host-compatibility promise than that.
