# Skill-Pack Rebuild — SunLitOrchestra (AI-First Runbook v3)

> **Purpose**: Rebuild SunLitOrchestra as a Claude Code skill pack that orchestrates research → design → execute sprints, while preserving the v3 runbook template as the canonical planning artifact.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.
> **Prerequisite reading**: [docs/skill-pack-catalog.md](skill-pack-catalog.md), [docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md), [README.md](../README.md), [crates/sldo-common/src/preflight.rs](../crates/sldo-common/src/preflight.rs).

---

## Runbook Metadata

- **Runbook ID**: `slo-skill-pack`
- **Prefix for test files and lessons files**: `slo-sp`
- **Primary stack**: Rust (workspace, installer) + Markdown (skill sources) + optional TLA+/Apalache (M5)
- **Primary package/app names**: `sldo-install` (new), `slo-tla` / `slo-ideate` / ... (skill directories)
- **Default test commands**:
  - Backend (baseline, per-crate to skip parked work): `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`
  - Frontend: `n/a` (Tauri UI parked — no frontend in this runbook)
  - E2E backend: `cargo test -p sldo-install --test '*'` (per milestone also adds its own crate)
  - E2E frontend: `n/a`
  - Build/boot: `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install --release`
- **Baseline note**: the `sunlit-orchestra-tests` root crate contains a mix of integration tests. The `e2e_tauri_*` tests fail on macOS arm64 because of a broken esbuild binary under the Tauri UI's `node_modules/` (parked work). The `e2e_voice_tx_*` tests are also part of the parked Tauri / voice-transcriber scope. We therefore baseline at the library-crate level only. If anyone later un-parks Tauri, run `npm install` in `crates/sldo-tauri/ui/` first, then the full `cargo test --workspace` should pass again.
- **Allowed new dependencies by default**: `none`. Each milestone that needs a new crate declares it inline with rationale.
- **Schema/config migration allowed by default**: `no`. Only M4/M5/M9 may touch persisted state, and they must carry migration tests.
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `sldo-plan` CLI flags (demoted to batch mode, but must keep working for users on prior scripts)
  - `sldo-run` CLI flags (same)
  - `sldo-research` CLI flags + output dossier shape
  - `sldo-common` public API (`preflight`, `detect`, `runbook`, `copilot`, `git`)

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Skill-pack skeleton + `sldo-install` | `done` | 2026-04-23 | 2026-04-23 | [slo-sp-m1.md](lessons/slo-sp-m1.md) | [slo-sp-m1.md](completion/slo-sp-m1.md) |
| 2 | `/slo-ideate` + `/slo-retro` (end-to-end slice) | `done` | 2026-04-23 | 2026-04-23 | [slo-sp-m2.md](lessons/slo-sp-m2.md) | [slo-sp-m2.md](completion/slo-sp-m2.md) |
| 3 | `/slo-research` (wraps `sldo-research`) | `done` | 2026-04-23 | 2026-04-23 | [slo-sp-m3.md](lessons/slo-sp-m3.md) | [slo-sp-m3.md](completion/slo-sp-m3.md) |
| 4 | `/slo-architect` + `/slo-plan` | `done` | 2026-04-23 | 2026-04-23 | [slo-sp-m4.md](lessons/slo-sp-m4.md) | [slo-sp-m4.md](completion/slo-sp-m4.md) |
| 5 | `/slo-tla` (formal verification) | `done` | 2026-04-23 | 2026-04-24 | [slo-sp-m5.md](lessons/slo-sp-m5.md) | [slo-sp-m5.md](completion/slo-sp-m5.md) |
| 6 | `/slo-critique` (4 sub-personas) | `done` | 2026-04-24 | 2026-04-24 | [slo-sp-m6.md](lessons/slo-sp-m6.md) | [slo-sp-m6.md](completion/slo-sp-m6.md) |
| 7 | `/slo-execute` + `/slo-verify` | `done` | 2026-04-24 | 2026-04-24 | [slo-sp-m7.md](lessons/slo-sp-m7.md) | [slo-sp-m7.md](completion/slo-sp-m7.md) |
| 8 | Power tools | `done` | 2026-04-24 | 2026-04-24 | [slo-sp-m8.md](lessons/slo-sp-m8.md) | [slo-sp-m8.md](completion/slo-sp-m8.md) |
| 9 | Self-hosting validation | `done` | 2026-04-24 | 2026-04-24 | [slo-sp-m9.md](lessons/slo-sp-m9.md) | [slo-sp-m9.md](completion/slo-sp-m9.md) |
| 10 | Context Hub integration (`chub` + `get-api-docs`) | `done` | 2026-04-24 | 2026-04-24 | [slo-sp-m10.md](lessons/slo-sp-m10.md) | [slo-sp-m10.md](completion/slo-sp-m10.md) |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/slo-sp-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/slo-sp-m<N>.md -->

---

## End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                         SunLitOrchestra Skill Pack                         │
│                                                                              │
│   User (Claude Code session)                                                 │
│        │                                                                     │
│        ▼                                                                     │
│   ┌─────────────────────────────────────────────────────────────────────┐    │
│   │  Skills (Markdown in skills/, symlinked to ~/.claude/skills/slo-*)  │    │
│   │                                                                     │    │
│   │   /slo-ideate ─► /slo-research ─► /slo-architect                    │    │
│   │                                        │                            │    │
│   │                                        ▼                            │    │
│   │   ┌────────── tla_required? ──────► /slo-tla                        │    │
│   │   │                                     │                           │    │
│   │   ▼                                     ▼                           │    │
│   │  /slo-plan ◄───────────────────────────┘                            │    │
│   │        │                                                            │    │
│   │        ▼                                                            │    │
│   │  /slo-critique                                                      │    │
│   │        │                                                            │    │
│   │        ▼                                                            │    │
│   │  /slo-execute ─► /slo-verify ─► /slo-retro (loop per milestone)     │    │
│   │                                                                     │    │
│   │  Power: /slo-second-opinion  /slo-freeze  /slo-resume  /slo-ship    │    │
│   └─────────────────────────────────────────────────────────────────────┘    │
│        │                            │                         │              │
│        │ (writes)                   │ (reads/writes)          │ (shells out) │
│        ▼                            ▼                         ▼              │
│   docs/slo/idea/, docs/slo/research/,   docs/RUNBOOK-*.md,       Rust backends:      │
│   docs/slo/design/,  specs/*.tla,   docs/slo/lessons/,           sldo-research       │
│   docs/slo/critique/, docs/slo/verify/  docs/slo/completion/         (primary, kept),    │
│                                                          sldo-plan, sldo-run │
│                                                          (batch fallback)    │
│                                                                              │
│   External tools (detected via which::which, lazy install):                  │
│   ═══ java (user-installed)  ═══ tla2tools.jar (~/.sldo/tla, auto-download)  │
│   ═══ apalache-mc (optional)  ═══ playwright (for /slo-verify)               │
│                                                                              │
│   Legend: ─── existing  - - - new  ═══ external  ▶ data flow                 │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `skills/` (repo root) | Source of truth for all skill SKILL.md files | M1 (created) | `skills/<name>/SKILL.md` |
| `sldo-install` (Rust bin) | Install/update/uninstall the skill pack; symlink skills into `~/.claude/skills/slo-*`; detect host (Claude Code, OpenClaw) | M1 (created) | CLI: `sldo-install [--global|--local] [--host <name>] [--uninstall]` |
| `/slo-ideate` | YC-style product interrogation | M2 | Reads user pitch → writes `docs/slo/idea/<slug>.md` |
| `/slo-retro` | Lessons + completion writer, tracker updater | M2 | Reads evidence log → writes `docs/slo/lessons/*`, `docs/slo/completion/*` |
| `/slo-research` | Wraps `sldo-research` | M3 | Reads idea doc → writes `docs/slo/research/<slug>/*` |
| `/slo-architect` | Stack + ARCHITECTURE.md + interfaces lock-in | M4 | Reads idea + research → writes `ARCHITECTURE.md`, `docs/slo/design/*` |
| `/slo-plan` | Interactive v3 runbook authoring | M4 | Reads architecture → writes `docs/RUNBOOK-<feat>.md` |
| `/slo-tla` | Formal verification via TLC/Apalache | M5 | Reads design → writes `specs/*.tla`, `specs/*.cfg`, `docs/slo/design/*-verified.md` |
| `~/.sldo/tla/` | User-scoped TLA+ tool cache (jar + shim) | M5 | `tla2tools.jar`, `tlc` shim, `VERSION` |
| `skills/slo-tla/tools.toml` | Pinned upstream URLs + SHA-256 checksums | M5 | TOML |
| `/slo-critique` | 4 sub-persona plan review | M6 | Reads runbook → writes `docs/slo/critique/<slug>.md`, inline edits |
| `/slo-execute` | Per-milestone implementer | M7 | Reads milestone → writes tests + production code + evidence log entries |
| `/slo-verify` | Runtime + browser QA per milestone | M7 | Reads BDD scenarios → writes `docs/slo/verify/*`, regression tests |
| Power tools | `/slo-second-opinion`, `/slo-freeze`, `/slo-resume`, `/slo-ship` | M8 | See per-skill spec in catalog |
| `get-api-docs` skill (external) | Agent-facing API doc fetcher backed by `chub` CLI ([andrewyng/context-hub](https://github.com/andrewyng/context-hub)) | M10 | Skill body at `skills/get-api-docs/SKILL.md` (vendored); installs to `~/.claude/skills/get-api-docs/`; requires `chub` on `PATH` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Idea capture | user pitch | `docs/slo/idea/<slug>.md` | skill writes file | M2 |
| Research dispatch | `/slo-research` | `sldo-research` Rust binary | shell-out, JSON on stdout | M3 |
| Runbook authoring | `/slo-plan` | `docs/RUNBOOK-<feat>.md` | skill writes file | M4 |
| TLA+ jar fetch | `/slo-tla` | `~/.sldo/tla/tla2tools.jar` | HTTPS GET + SHA-256 verify | M5 |
| TLC invocation | `/slo-tla` | `java -jar tla2tools.jar` | shell-out | M5 |
| Per-milestone execution | `/slo-execute` | repo files + evidence log | skill edits files | M7 |
| Browser QA | `/slo-verify` | Playwright (local) | shell-out | M7 |
| Cross-model review | `/slo-second-opinion` | Codex CLI / Gemini CLI | shell-out | M8 |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — this runbook delivers a developer-facing tool (skills + installer). There is no concurrency, distributed state, ordering guarantee, resource ownership, or failure recovery requirement that formal verification would meaningfully protect.

The one component that *could* merit TLA+ modeling — `/slo-tla` itself — will use its own spec as a dogfood test in M9 (self-hosting). That spec will model the "elicit → draft → check → counterexample → refine" loop, but it is not a prerequisite for shipping the skill.

---

## Global Execution Rules

These rules apply to every milestone without exception. (Summarized; full text in [docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md#global-execution-rules).)

1. **Stay inside scope** — only change files listed in the current milestone.
2. **Tests define the contract** — BDD and E2E stubs before production code.
3. **No placeholders in production paths** — no TODOs, silent fallbacks, swallowed errors, or fake implementations.
4. **Preserve backwards compatibility** — existing `sldo-plan`, `sldo-run`, `sldo-research` CLIs must keep working.
5. **Prefer smallest safe change.**
6. **Record evidence, not claims.**
7. **Keep .gitignore current and clean up test artifacts.**

---

## Global Entry Rules (Pre-Milestone Protocol)

Before every milestone:

1. Read the lessons file from the previous milestone (`docs/slo/lessons/slo-sp-m<N-1>.md`).
2. Read the current milestone fully.
3. Run baseline: `cargo test --workspace` — must be green. If red, fix first.
4. Read files in "Files Allowed To Change" and "Files To Read Before Changing Anything".
5. Update the Milestone Tracker: set current milestone to `in_progress`, record Started date.
6. Create BDD test files first.
7. Create E2E runtime validation stubs first.
8. Copy the milestone's Evidence Log template into working notes.
9. Re-state milestone constraints in your own words before coding.

---

## Global Exit Rules (Post-Milestone Protocol)

After every milestone:

1. Full test suite: `cargo test --workspace`.
2. E2E runtime: milestone-specific E2E commands.
3. Build/boot: `cargo build --workspace --release`.
4. Run smoke tests listed in the milestone.
5. Verify backward compatibility against the Compatibility Checklist.
6. Complete the Self-Review Gate.
7. `git status` — no untracked test artifacts.
8. Review `.gitignore` — patterns current.
9. Update ARCHITECTURE.md per the Documentation Update Table.
10. Update README.md if user-facing capabilities changed.
11. Write `docs/slo/lessons/slo-sp-m<N>.md`.
12. Write `docs/slo/completion/slo-sp-m<N>.md`.
13. Update the Milestone Tracker (status `done`, Completed date, paths).
14. Re-read the next milestone with fresh eyes; record assumption changes in lessons.

---

## Background Context

### Current State

SunLitOrchestra is a Rust workspace with five crates:

- [crates/sldo-common](../crates/sldo-common/) — shared types (preflight, detect, git, copilot, runbook, logging).
- [crates/sldo-plan](../crates/sldo-plan/) — CLI that calls Copilot/Claude to generate a v3 runbook from a prompt + repo.
- [crates/sldo-run](../crates/sldo-run/) — CLI that drives Copilot/Claude through an existing runbook milestone-by-milestone.
- [crates/sldo-research](../crates/sldo-research/) — research pipeline (M6 synthesis + M7 plan-readiness complete).
- [crates/sldo-tauri](../crates/sldo-tauri/) — desktop UI in progress on the `tauri` branch.

The v3 runbook template at [docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md) is the crown jewel: BDD contracts, evidence logs, TLA+ section, lessons files, self-review gates. 11 runbooks have been authored against earlier template versions.

### Problem

1. **Driver-loop pattern produces shallow plans.** `sldo-plan` generates a whole runbook in one Copilot call. The result is always syntactically valid but often strategically thin — wedge analysis, competitor research, and adversarial review are missing because nothing in the pipeline forces them.
2. **No interactivity at decision points.** The current UX is "fill prompt file, press enter, wait, review 2,000-line output." There is no surface for steering at the scope-is-wrong moment.
3. **No formal verification.** The v3 template has a TLA+ section, but no tool actually produces a model-checked spec. The section gets filled with prose, not proofs.
4. **No real QA.** `sldo-run` verifies compilation + tests pass. It does not open a browser, click a button, or check that the app boots to a usable state.
5. **Tauri GUI competing with the skill model.** The desktop app on the `tauri` branch is trying to be a front-end for something that Claude Code already is. More effort there is negative ROI.
6. **No upstream interrogation.** There is no `/office-hours` equivalent — no forcing-function that reframes the user's pitch before committing to a runbook.

### Target Architecture

See the End-to-End Architecture Diagram above. The short version: skills (Markdown) replace the driver-loop binaries as the primary interface. The `sldo-research` Rust backend stays. The Tauri app is parked. The v3 runbook template becomes the output contract of `/slo-plan` rather than the one-shot output of `sldo-plan`.

### Key Design Principles

1. **The v3 runbook is the output contract, not the tool.** Every planning skill ultimately writes, extends, or executes against a v3 runbook.
2. **Skills are Markdown, not binaries.** One skill, one `SKILL.md`. No embedded logic beyond what Claude Code executes.
3. **Reuse Rust backends where they earn it.** `sldo-research` is already good. Wrap, don't rewrite.
4. **Fail loud on missing prereqs.** Follow the `preflight` module pattern: `which::which(...)` + human-readable install hint + exit clean. Never silently fall back.
5. **Interactivity at decision points only.** Ask when the answer isn't in the codebase or prior artifacts.
6. **Declare bounds for every "verified" claim.** In `/slo-tla`, every TLC result states the model bounds (N actors, M requests, K failures). "Verified" without bounds is a lie.

### What to Keep

- `sldo-common` public API (all modules).
- `sldo-research` crate (primary research backend; wrapped by `/slo-research`).
- `sldo-plan` and `sldo-run` CLIs (demoted to optional batch mode; must keep working).
- The v3 runbook template ([docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md)).
- All existing lessons and completion files.

### What to Change

- **Add `skills/` directory at repo root** — new home for all SKILL.md files.
- **Add `crates/sldo-install`** — Rust binary that symlinks `skills/<name>/` into `~/.claude/skills/slo-<name>/` (or `./.claude/skills/slo-<name>/` for project-local).
- **Update README.md** — describe the skill-based UX as primary; `sldo-plan`/`sldo-run` demoted to "batch mode."
- **Leave the `tauri` branch alone.** Do not merge. Do not delete. Parked.

### Global Red Lines

- No unrelated refactors.
- No new dependencies unless the milestone explicitly lists them with rationale.
- No schema migrations.
- No config key renames.
- No public API/event/route renames.
- No production placeholders.
- No silent error swallowing.
- No secrets in source control.
- No test output data committed to source control.
- **No modifications to `crates/sldo-tauri/`** in any milestone of this runbook. The Tauri app is parked.

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

New dependencies proposed per milestone (subject to justification block in the contract):

| Crate | Proposed in | Why | Alternatives rejected |
|---|---|---|---|
| `reqwest` (blocking, rustls-tls, no default features) | M5 | Download `tla2tools.jar` from upstream | `ureq` — fine alternative, but `reqwest` is already common in the ecosystem and supports checksumming streams; keep surface small with `default-features = false` |
| `sha2` | M5 | Verify SHA-256 of downloaded jar | None — stdlib has no hashing |
| `tempfile` | M1 (dev-dep) | Test symlink creation without polluting home dir | `std::env::temp_dir` — no auto-cleanup |

No other new deps. Any proposed addition must follow the v3 dependency policy.

### Migration policy

Only M5 touches persisted state (`~/.sldo/tla/*`). That is user-local cache, not project schema, but it still carries:
- a versioning file (`VERSION`)
- forward-compatibility: if a newer skill version expects a new path, the old path is migrated on first run
- rollback: uninstall removes the entire `~/.sldo/tla/` directory

### Refactor budget

Per milestone:
- M1: `Minimal local refactor permitted in listed files only` (touches `Cargo.toml` workspace members).
- M2–M8: `No refactor permitted beyond direct implementation`.
- M9: `Targeted refactor permitted for issues surfaced during self-hosting`.

---

## Evidence Log Template

Copy this into each milestone section during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `[command]` | green | | | |
| Build/boot | `cargo build --workspace --release` | boots cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current, no stale entries | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

See [runbook-template_v_3_template.md → Self-Review Gate](runbook-template_v_3_template.md#self-review-gate). Every question must be answered Yes before marking a milestone done.

---

## Milestone Plan

### Milestone 1 — Skill-pack skeleton + `sldo-install`

**Goal**: Stand up the `skills/` directory and ship an `sldo-install` Rust binary that can install, update, and uninstall the (initially empty) skill pack into `~/.claude/skills/slo-*/` or `./.claude/skills/slo-*/`. End state: running `sldo-install` with no skills yet produces a clean install with a manifest and zero symlinks.

**Context**: The skill pack needs a home in the repo and a way to get onto users' machines before any individual skill is authored. Mirrors gstack's `./setup`, but written in Rust for consistency with the existing crates.

**Important design rule**: The installer must be idempotent. Running it twice in a row produces the same state as running it once. Uninstall must remove every artifact the installer created and nothing else.

**Refactor budget**: `Minimal local refactor permitted in listed files only` (workspace `Cargo.toml` gets a new member).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | CLI flags: `--global` (default) / `--local` / `--host <name>` / `--uninstall` / `--dry-run` |
| Outputs | Symlinks in `~/.claude/skills/slo-<name>/` (or `./.claude/skills/...`); manifest at `~/.sldo/install.toml` |
| Interfaces touched | New binary `sldo-install`; new `skills/` directory at repo root |
| Files allowed to change | `Cargo.toml` (workspace members), `crates/sldo-install/**` (new), `skills/README.md` (new), `.gitignore`, `README.md` (skill-pack section) |
| Files to read before changing anything | `crates/sldo-common/src/preflight.rs`, `crates/sldo-common/src/detect.rs`, `crates/sldo-common/src/logging.rs`, `docs/skill-pack-catalog.md`, `Cargo.toml` |
| New files allowed | `crates/sldo-install/Cargo.toml`, `crates/sldo-install/src/main.rs`, `crates/sldo-install/src/install.rs`, `crates/sldo-install/src/manifest.rs`, `crates/sldo-install/tests/install_e2e.rs`, `skills/README.md` |
| New dependencies allowed | `tempfile` (dev-dep, test-only). No runtime deps beyond what `sldo-common` already has. |
| Migration allowed | `no` |
| Compatibility commitments | `sldo-plan`, `sldo-run`, `sldo-research` CLIs unchanged. `sldo-common` public API unchanged. |
| Forbidden shortcuts | No mock filesystems in prod code; no hardcoded absolute paths; no silent overwrites of existing symlinks without `--force`; no touching `crates/sldo-tauri/` |

#### Out of Scope / Must Not Do

- Do not author any actual `SKILL.md` files. Skeleton only.
- Do not support hosts other than Claude Code in this milestone. The `--host` flag parses but only `claude-code` is implemented.
- Do not add auto-update. Manual `sldo-install` re-run is the update path.
- Do not touch `crates/sldo-tauri/`.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Baseline: `cargo test --workspace`.
3. Read `crates/sldo-common/src/preflight.rs` and reuse the `which`-based detection pattern.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `Cargo.toml` | Add `crates/sldo-install` to workspace members |
| `crates/sldo-install/Cargo.toml` | NEW: binary crate, deps `clap`, `anyhow`, `serde`, `toml`, `sldo-common` |
| `crates/sldo-install/src/main.rs` | NEW: clap CLI, wires subcommands |
| `crates/sldo-install/src/install.rs` | NEW: symlink logic, host detection, idempotency |
| `crates/sldo-install/src/manifest.rs` | NEW: read/write `~/.sldo/install.toml` |
| `crates/sldo-install/tests/install_e2e.rs` | NEW: E2E tests using `tempfile::TempDir` as a fake home |
| `skills/README.md` | NEW: explains skill-pack layout |
| `.gitignore` | Add `~/.sldo/` patterns? No — that's outside the repo. Add any test-output patterns `crates/sldo-install/tests/` produces (should be none if using `tempfile`). |
| `README.md` | Add a "Skill pack (experimental)" section pointing at `sldo-install` |

#### Step-by-Step

1. Write BDD test stubs for: idempotent install, uninstall removes everything, `--dry-run` writes nothing, `--local` installs into `./.claude/skills/`, missing `skills/` directory fails loud.
2. Write E2E stubs: full install-then-uninstall against a `TempDir` fake home.
3. Create the crate skeleton (`Cargo.toml`, `main.rs` with clap).
4. Implement `install::install_global(home: &Path, skills_dir: &Path) -> Result<Manifest>`.
5. Implement `install::uninstall_global(home: &Path, manifest: &Manifest) -> Result<()>`.
6. Implement manifest read/write.
7. Run `cargo test --workspace`.
8. Update README.md.
9. `git status` + `.gitignore` review.
10. Self-Review Gate.

#### BDD Acceptance Scenarios

**Feature: skill-pack install and uninstall**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| idempotent_global_install | happy path | No prior install; empty `skills/` | `sldo-install` runs twice | Both exit 0; second run reports "no changes"; manifest is identical |
| uninstall_removes_all | happy path | Prior install with 0 skills | `sldo-install --uninstall` | No symlinks in `~/.claude/skills/slo-*`; no manifest file |
| local_install_into_project | happy path | Empty project dir | `sldo-install --local` from repo | Symlinks land in `./.claude/skills/slo-*`; manifest at `./.claude/slo-install.toml` |
| dry_run_writes_nothing | happy path | No prior install | `sldo-install --dry-run` | Prints planned actions; no files or symlinks created |
| missing_skills_dir | dependency failure | `skills/` directory deleted | `sldo-install` | Exits non-zero with clear error naming the missing path |
| existing_symlink_refuses | invalid input | A dangling symlink at `~/.claude/skills/slo-foo` | `sldo-install` (no `--force`) | Exits non-zero; prints "symlink exists, use --force"; does not overwrite |
| existing_symlink_with_force | happy path | Same as above + `--force` | `sldo-install --force` | Overwrites, records in manifest |
| empty_skills_dir_ok | empty state | `skills/` exists but contains no `SKILL.md` | `sldo-install` | Exit 0; manifest lists 0 installed skills; prints "no skills to install" |
| unknown_host | invalid input | — | `sldo-install --host unicorn` | Exits non-zero with "unsupported host" and list of supported hosts |

#### Regression Tests

- `cargo test -p sldo-common` — existing tests unchanged.
- `sldo-plan --help` — exits 0 (smoke).
- `sldo-run --help` — exits 0 (smoke).
- `sldo-research --help` — exits 0 (smoke).

#### Compatibility Checklist

- [ ] `sldo-plan --help` still works
- [ ] `sldo-run --help` still works
- [ ] `sldo-research --help` still works
- [ ] `sldo-common` public API unchanged (verified by `cargo check --workspace`)
- [ ] `crates/sldo-tauri/` is untouched (`git diff --stat` shows no files in that path)

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/install_e2e.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_full_install_uninstall_cycle` | Install then uninstall leaves filesystem identical to start | `tempfile::TempDir` used as HOME; before/after file listings match |
| `test_idempotent_double_install` | Running install twice doesn't corrupt state | Manifest hash identical after runs 1 and 2 |
| `test_force_overwrites_existing_symlink` | `--force` replaces without error | Symlink points at new target after run |

#### Smoke Tests

- [ ] `cargo build --workspace --release` succeeds
- [ ] `./target/release/sldo-install --help` shows all flags
- [ ] `./target/release/sldo-install --dry-run` prints a plan without writing
- [ ] App launches without errors
- [ ] `git status` shows no untracked test artifacts

#### Evidence Log

(Copy the template above and fill during execution.)

#### Definition of Done

- All BDD scenarios pass
- All E2E runtime validations pass
- Full existing test suite remains green
- Smoke tests checked off
- Compatibility checklist complete
- `sldo-install --help` produces human-readable output that matches this contract
- Lessons file written
- Completion summary written
- Milestone Tracker updated

---

### Milestone 2 — `/slo-ideate` + `/slo-retro`

**Goal**: Ship the smallest end-to-end slice: two skills that together let a user go from raw idea to closed milestone without any of the middle skills. `/slo-ideate` produces an idea doc; `/slo-retro` writes lessons + completion files and updates a tracker. Proves the skill-authoring and install pipeline works.

**Context**: Before building the full sprint, verify the skill format compiles, installs, and executes inside Claude Code. Two skills is the smallest number that exercises a "skill writes artifact → another skill reads artifact" handoff.

**Important design rule**: Every skill's `SKILL.md` must declare inputs (files/state it reads) and outputs (files it writes) in its frontmatter so `/slo-resume` (M8) can reason about pipeline state.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | For `/slo-ideate`: user's natural-language pitch. For `/slo-retro`: evidence log + tracker row |
| Outputs | `docs/slo/idea/<slug>.md`; `docs/slo/lessons/<prefix>-m<N>.md`; `docs/slo/completion/<prefix>-m<N>.md`; updated tracker |
| Interfaces touched | New `skills/slo-ideate/SKILL.md`, `skills/slo-retro/SKILL.md` |
| Files allowed to change | `skills/slo-ideate/**`, `skills/slo-retro/**`, `crates/sldo-install/src/install.rs` (to pick up skills from the `skills/` dir; if install logic is already generic, no change here), `README.md` |
| Files to read before changing anything | `docs/skill-pack-catalog.md`, `docs/slo/templates/runbook-template_v_3_template.md`, existing lessons files (`docs/slo/lessons/research-m*.md`) as style reference |
| New files allowed | `skills/slo-ideate/SKILL.md`, `skills/slo-retro/SKILL.md`, `skills/slo-ideate/examples/` (optional), test fixtures |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `sldo-install` from M1 must install these two skills with zero code changes (generic symlink logic) |
| Forbidden shortcuts | No placeholder "TODO: implement" in the skill bodies. Skills must be fully functional on first ship. |

#### Out of Scope / Must Not Do

- Do not implement any other skill.
- Do not enforce a specific slug format beyond "lowercase-kebab".
- Do not build a UI — these are Markdown skills executed by Claude Code.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-ideate/SKILL.md` | NEW: full skill body per catalog spec |
| `skills/slo-retro/SKILL.md` | NEW: full skill body per catalog spec |
| `skills/slo-ideate/examples/briefing-app.md` | NEW: example idea doc (style reference) |
| `README.md` | Add 2 skills to the available-skills list |

#### BDD Acceptance Scenarios

**Feature: /slo-ideate**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| raw_pitch_produces_idea_doc | happy path | User provides 1-sentence pitch | `/slo-ideate` runs | `docs/slo/idea/<slug>.md` exists with reframed pain, 3 approaches, recommended wedge |
| pitch_too_vague_triggers_questions | invalid input | Pitch is "I want to build an app" | `/slo-ideate` runs | Skill asks forcing questions; does not produce file until specifics captured |
| existing_idea_detected | empty state | `docs/slo/idea/<slug>.md` already exists for this slug | `/slo-ideate` runs | Skill surfaces the overlap and asks whether to extend or start fresh |

**Feature: /slo-retro**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| writes_lessons_and_completion | happy path | A milestone's evidence log is filled | `/slo-retro` runs | Two files written at correct paths; tracker row updated to `done` |
| missing_evidence_log_refuses | invalid input | Evidence log table has blank Actual Result rows | `/slo-retro` runs | Skill refuses to proceed and lists blank rows |
| lessons_file_already_exists | dependency failure | Prior lessons file for same milestone present | `/slo-retro` runs | Skill offers to append or overwrite; does not silently overwrite |

#### E2E Runtime Validation

Because skills are Markdown executed inside Claude Code, E2E validation is manual-but-scripted: a shell test that:
1. Installs the skills via `sldo-install`.
2. Invokes Claude Code with a seed pitch.
3. Asserts the output file exists and contains the expected frontmatter + section headers.

**File**: `crates/sldo-install/tests/e2e_slo_sp_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_ideate_skill_installed_and_discoverable` | After `sldo-install`, Claude Code lists `/slo-ideate` | Symlink exists; `SKILL.md` parses as valid frontmatter + body |
| `test_retro_skill_installed_and_discoverable` | Same for `/slo-retro` | Same |

#### Smoke Tests

- [ ] `sldo-install` picks up both new skills (exit 0, manifest lists them)
- [ ] Each `SKILL.md` passes a frontmatter-validity check (manual: no YAML parse errors)
- [ ] Spot-run `/slo-ideate` in a fresh Claude Code session with a test pitch → produces the expected file
- [ ] `git status` shows no untracked test artifacts

#### Definition of Done

- BDD scenarios satisfied (verified by running skills in a real Claude Code session; record in evidence log)
- E2E tests green
- Skills install via M1's `sldo-install` without code changes
- Lessons + completion files written
- Tracker updated

---

### Milestone 3 — `/slo-research` (wraps `sldo-research`)

**Goal**: Ship `/slo-research` as a thin skill that reads an idea doc, builds a research prompt, shells out to the `sldo-research` binary, and writes the structured dossier under `docs/slo/research/<slug>/`. No reimplementation of research logic in the skill.

**Context**: `sldo-research` already handles sourcing, synthesis (M6), and plan-readiness (M7). The skill's job is framing and output gating — not research itself.

**Important design rule**: If `sldo-research` exits non-zero or returns fewer than 3 sourced competitor comparisons, the skill must surface the gap and refuse to produce a "complete" dossier. Partial research is worse than no research because it gives false confidence.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `docs/slo/idea/<slug>.md` from M2 |
| Outputs | `docs/slo/research/<slug>/dossier.md`, `sources.md`, `synthesis.md` |
| Interfaces touched | Shell-out to `sldo-research` CLI |
| Files allowed to change | `skills/slo-research/**`, `crates/sldo-research/**` ONLY if output format needs tightening for the skill to parse reliably |
| Files to read before changing anything | `crates/sldo-research/src/main.rs`, `crates/sldo-research/src/dossier.rs`, `crates/sldo-research/src/research.rs`, `docs/RUNBOOK-RESEARCH.md`, existing research lessons |
| New files allowed | `skills/slo-research/SKILL.md`, `skills/slo-research/templates/` (prompt templates) |
| New dependencies allowed | `none` |
| Migration allowed | `no` (but if `sldo-research` output format changes, carry backward compatibility for existing `docs/slo/research/*` files) |
| Compatibility commitments | `sldo-research` CLI flags unchanged; dossier schema unchanged (or versioned if it must change) |
| Forbidden shortcuts | No re-implementing research in the skill body. No hardcoded example dossiers shipped as if they were real output. |

#### Out of Scope / Must Not Do

- Do not modify the research pipeline's ranking, sourcing, or synthesis logic.
- Do not add a new output format.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| idea_doc_produces_dossier | happy path | Valid idea doc at `docs/slo/idea/foo.md` | `/slo-research` runs | All three output files exist with required sections |
| missing_idea_doc_refuses | invalid input | No idea doc for slug | `/slo-research foo` | Skill exits with "run /slo-ideate first" |
| sldo_research_binary_missing | dependency failure | `sldo-research` not on PATH | `/slo-research` runs | Skill surfaces install hint from `preflight` pattern; exits clean |
| fewer_than_3_sources | partial failure | Research returns 2 competitors | `/slo-research` runs | Dossier flagged `incomplete: true`; skill prompts to either expand scope or accept |

#### Definition of Done

- BDD scenarios pass
- Existing `sldo-research` tests remain green
- Dossier produced for a known idea doc is readable, sourced, and ends with "the design must handle X, Y, Z because [source]"

---

### Milestone 4 — `/slo-architect` + `/slo-plan`

**Goal**: Two skills that together take an idea + research and produce a fully populated v3 runbook. `/slo-architect` writes `ARCHITECTURE.md` + stack decisions + the `tla_required` flag. `/slo-plan` walks milestone-by-milestone interactively.

**Context**: This is the replacement for the `sldo-plan` Rust binary's primary role. `sldo-plan` stays callable but is demoted to batch mode.

**Important design rule**: `/slo-plan` must NEVER generate a runbook in one shot. It walks one milestone at a time, confirming each contract block with the user before moving on. Max 5 milestones per feature runbook; if scope needs more, split.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Idea doc + research dossier + (optional) existing codebase state |
| Outputs | `ARCHITECTURE.md` (or updated), `docs/slo/design/stack-decision.md`, `docs/slo/design/interfaces.md`, `docs/RUNBOOK-<feature>.md` |
| Interfaces touched | New skills only; no Rust changes |
| Files allowed to change | `skills/slo-architect/**`, `skills/slo-plan/**` |
| Files to read before changing anything | `docs/slo/templates/runbook-template_v_3_template.md`, existing runbooks for style |
| New files allowed | Two new `SKILL.md` files; prompt templates |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Produced runbooks must match the v3 template structure exactly (smoke-tested by parsing one) |
| Forbidden shortcuts | No whole-runbook-in-one-shot generation; no skipping the interactive per-milestone confirmation |

#### Out of Scope / Must Not Do

- Do not modify `sldo-plan` Rust binary.
- Do not implement `/slo-critique` — that's M6.

#### BDD Acceptance Scenarios

**Feature: /slo-architect**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| greenfield_architecture | happy path | Idea + research, no existing codebase | `/slo-architect` runs | `ARCHITECTURE.md` created with diagram, component summary, stack decisions |
| brownfield_preserves_stack | happy path | Existing repo with detected stack | `/slo-architect` runs | Stack decision doc explains why the existing stack was kept (or challenged) |
| tla_required_concurrency | happy path | Design involves concurrent actors or distributed state | `/slo-architect` runs | Design doc frontmatter sets `tla_required: true` |
| tla_not_required_crud | happy path | Simple CRUD design | `/slo-architect` runs | Frontmatter sets `tla_required: false` with one-line justification |

**Feature: /slo-plan**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| runbook_matches_v3_template | happy path | Architecture + design docs ready | `/slo-plan` runs | Generated runbook parses against v3 template structure |
| one_shot_refused | invalid input | User asks "generate the whole runbook now" | `/slo-plan` runs | Skill refuses, explains interactive-per-milestone discipline |
| scope_over_5_milestones | invalid input | Design implies 7+ milestones | `/slo-plan` runs | Skill suggests splitting into separate runbooks before proceeding |

#### Definition of Done

- Runbook produced is structurally a valid v3 instance
- `tla_required` flag is set correctly for a sample concurrent design and a sample CRUD design
- Batch fallback (`sldo-plan` binary) still runs

---

### Milestone 5 — `/slo-tla` (formal verification)

**Goal**: Ship `/slo-tla` — the hardest skill. Takes a high-level design (either from `/slo-architect` output or a hand-written doc), produces a TLA+ spec, runs TLC (or Apalache for state explosion), translates counterexamples to plain English, iterates with the user on design fixes, and outputs a verified-design doc + spec artifacts.

**Context**: No competitor skill pack does formal verification. This is SLO's differentiator. The v3 template already has a TLA+ section (lines 99–157 of [docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md)) — this skill fills that section with a model-checked spec, not prose.

**Important design rule**: **Every "verified" claim must declare its bounds.** N actors, M requests, K failures, etc. "TLC found no violations" is meaningless without bounds.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Design doc (from `/slo-architect` or standalone); optional existing spec to extend |
| Outputs | `specs/<name>.tla`, `specs/<name>.cfg`, `specs/<name>.trace.md` (if counterexamples), `docs/slo/design/<name>-verified.md`; patches to runbook's TLA+ section if present |
| Interfaces touched | New skill; new tool cache at `~/.sldo/tla/`; runtime shell-out to `java -jar` and optionally `apalache-mc` |
| Files allowed to change | `skills/slo-tla/**`, `skills/slo-tla/tools.toml` (NEW) |
| Files to read before changing anything | `crates/sldo-common/src/preflight.rs` (fail-loud pattern), `docs/slo/templates/runbook-template_v_3_template.md` (TLA+ section shape) |
| New files allowed | `skills/slo-tla/SKILL.md`, `skills/slo-tla/tools.toml`, `skills/slo-tla/templates/*.tla.tmpl`, `skills/slo-tla/counterexample-translator.md` |
| New dependencies allowed | `reqwest` (blocking, rustls-tls, default-features = false) for the jar download helper; `sha2` for checksum verification. If the skill relies on a small Rust helper binary for download+verify, it goes in a new `crates/sldo-tla-tools/` crate with this scope. Alternative: do the download inline in the skill via `curl` + `shasum` shell commands — acceptable and simpler; prefer this unless there's a concrete reason for the Rust helper. |
| Migration allowed | `yes` (for `~/.sldo/tla/` cache layout, with version file) |
| Compatibility commitments | None — this is a new surface |
| Forbidden shortcuts | No bundling the jar in the repo. No floating `latest` URLs. No silent fallback if Java is missing. No "verified" claim without bounds. No counterexample reported as raw TLC output — must be translated to English. |

#### Out of Scope / Must Not Do

- Do not build a GUI for TLA+.
- Do not auto-install Java — tell the user and exit.
- Do not model-check unbounded state without declaring the bound.
- Do not ship Apalache integration fully — M5 ships TLC only; Apalache is a stub-with-detect that emits "state explosion detected, install Apalache" hints. Full Apalache integration is a follow-up.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-tla/SKILL.md` | NEW: full skill body |
| `skills/slo-tla/tools.toml` | NEW: pinned TLC version + SHA-256 + upstream URL |
| `skills/slo-tla/templates/basic-state-machine.tla.tmpl` | NEW: starter template |
| `skills/slo-tla/counterexample-translator.md` | NEW: methodology for converting TLC traces to English |
| `.gitignore` | Add `specs/*.tla.out`, `states/` (TLC scratch dirs) |

#### Step-by-Step

1. Write BDD stubs: JVM missing, JVM present jar missing (triggers download), JVM + jar present, checksum mismatch, spec compiles and TLC runs, invariant violated triggers counterexample translation, bounds missing is rejected.
2. Write E2E stub: end-to-end against a tiny example spec (2-actor mutual exclusion, known-good).
3. Author `tools.toml` with pinned version.
4. Author `SKILL.md` — include the elicitation methodology (state vars → Init → Next → invariants → bounds), the counterexample translator, the fairness-decision forcing function.
5. Implement jar download + checksum verify (shell commands in the skill body are acceptable).
6. Smoke test against a known-good 2-actor mutex spec.
7. Deliberately break the invariant, verify the counterexample translator output is readable English.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| jvm_missing_clean_exit | dependency failure | `java` not on PATH | `/slo-tla` runs | Exits with platform-specific install hint; no partial state created |
| jvm_present_jar_missing | dependency failure | Java on PATH; `~/.sldo/tla/tla2tools.jar` absent | `/slo-tla` runs | Downloads jar, verifies SHA-256, installs shim, continues |
| checksum_mismatch_aborts | dependency failure | Downloaded jar doesn't match pinned SHA | `/slo-tla` runs | Deletes partial download; exits non-zero with security warning |
| tla_spec_passes_tlc | happy path | Valid design; TLC runs and finds no violations | `/slo-tla` runs | Writes `-verified.md` listing checked properties + bounds |
| invariant_violated_counterexample | happy path | Design has a race; TLC finds a counterexample | `/slo-tla` runs | `trace.md` contains English step-by-step violation; skill proposes design fix |
| bounds_unstated_refused | invalid input | User asks to mark design "verified" without bounds | `/slo-tla` runs | Skill refuses; requires explicit N/M/K bounds |
| state_explosion_apalache_hint | partial failure | TLC runs out of memory | `/slo-tla` runs | Emits "state explosion detected" + Apalache install suggestion; does not claim verified |
| fairness_not_declared | invalid input | Liveness property stated without fairness assumption | `/slo-tla` runs | Skill forces explicit weak/strong-on-which-action decision before proceeding |

#### E2E Runtime Validation

**File**: `skills/slo-tla/tests/e2e_mutex.sh` (shell script runnable in CI)

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_mutex_passes` | Known-good 2-actor mutex spec passes TLC | Exit 0; `-verified.md` produced |
| `test_broken_mutex_fails_with_trace` | Deliberately-broken mutex produces readable counterexample | `trace.md` contains "Actor A ... Actor B ..." English narrative |
| `test_jar_download_reproducible` | Second run doesn't re-download | First run downloads, second run uses cached jar |

#### Smoke Tests

- [ ] With Java uninstalled: skill exits clean with install hint
- [ ] With Java installed, jar absent: skill downloads, verifies, caches
- [ ] With both present: skill runs TLC in < 30s on the mutex example
- [ ] Counterexample output is readable by someone who doesn't know TLA+
- [ ] `~/.sldo/tla/VERSION` matches pinned version
- [ ] `git status` shows no untracked `.out` files or TLC state directories

#### Definition of Done

- All BDD scenarios pass
- E2E shell tests green
- Tool cascade (JVM-detect → jar-download-if-needed → shim) works on macOS, Linux, and WSL (record the tested platforms in the lessons file)
- Counterexample translator produces readable English on the broken-mutex example
- `tools.toml` contains pinned version + SHA-256

---

### Milestone 6 — `/slo-critique` (4 sub-personas)

**Goal**: Ship `/slo-critique` — runs CEO, eng-lead, security, and design review passes against a runbook. Auto-fixes mechanical issues; asks before scope changes.

**Context**: Replaces the missing "adversarial review" layer. Each sub-persona is a sub-skill; the parent orchestrates.

**Important design rule**: Each sub-persona must produce findings with concrete exploit/failure scenarios, not theoretical risks. "This might have a race" is rejected; "actor A sends X; actor B crashes before Y; on recovery B processes X twice" is accepted.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `docs/RUNBOOK-<feature>.md` |
| Outputs | Inline runbook edits; `docs/slo/critique/<slug>.md` summary |
| Interfaces touched | New skills only |
| Files allowed to change | `skills/slo-critique/**` (parent + 4 sub-skill dirs) |
| Files to read before changing anything | gstack's `/plan-ceo-review`, `/plan-eng-review`, `/cso` prompts at `~/Documents/Dev/GitHub/gstack/plan-ceo-review/`, `~/Documents/Dev/GitHub/gstack/plan-eng-review/`, `~/Documents/Dev/GitHub/gstack/cso/` for methodology reference |
| New files allowed | Five `SKILL.md` files (parent + 4 sub), shared methodology templates |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Runbooks produced by `/slo-plan` must pass through `/slo-critique` without structural damage |
| Forbidden shortcuts | No theoretical findings; no scope changes without user approval |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| mechanical_issues_auto_fixed | happy path | Runbook has missing compat checklist rows | `/slo-critique` runs | Rows filled; summary lists auto-fixes |
| scope_change_requires_approval | happy path | CEO persona proposes scope expansion | `/slo-critique` runs | Skill pauses, asks user, does not edit without approval |
| concrete_exploit_required | invalid input | Security finding states "maybe a race" | Review runs | Skill re-prompts for concrete scenario; rejects vague findings |
| no_ui_no_design_review | happy path | Runbook has zero UI surface | `/slo-critique` runs | Design persona skipped with note in summary |

#### Definition of Done

- 4 sub-personas produce ≥1 concrete finding each on a test runbook designed to have planted bugs
- Auto-fix vs. ask-for-approval behavior matches BDD scenarios
- Summary doc at `docs/slo/critique/<slug>.md` cross-references each finding to a runbook line

---

### Milestone 7 — `/slo-execute` + `/slo-verify`

**Goal**: Ship the execution loop skills. `/slo-execute M<N>` drives one milestone: restates constraints, writes BDD tests first, implements, fills evidence log. `/slo-verify` runs runtime + browser QA against the milestone's BDD scenarios.

**Context**: Replaces `sldo-run`'s inner loop with interactive execution. `/slo-verify` wraps Playwright (following gstack's `/browse` approach).

**Important design rule**: `/slo-execute` must refuse to touch files outside the milestone's Files Allowed To Change list. Violating the allow-list is the single most common failure mode of AI-driven runbook execution — hard-enforce it.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A v3 runbook with at least one milestone |
| Outputs | BDD tests, E2E stubs, production code, filled evidence log rows, regression tests generated from bugs found |
| Interfaces touched | New skills; Playwright shell-out |
| Files allowed to change | `skills/slo-execute/**`, `skills/slo-verify/**` |
| Files to read before changing anything | `docs/slo/templates/runbook-template_v_3_template.md`, gstack's `/qa` and `/ship` |
| New files allowed | Two `SKILL.md` files, Playwright config templates |
| New dependencies allowed | Playwright via `npm init playwright` when a repo first needs it (installed into target repo, not our repo) |
| Migration allowed | `no` |
| Compatibility commitments | Works against runbooks produced by `/slo-plan` (M4) and by the legacy `sldo-plan` binary |
| Forbidden shortcuts | No editing outside the milestone allow-list; no marking a milestone done before evidence log is complete; no skipping BDD-first discipline |

#### BDD Acceptance Scenarios

**Feature: /slo-execute**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| allowed_files_respected | happy path | Milestone limits changes to file A | `/slo-execute M1` runs | Only file A (and tests) edited; git diff confirms |
| out_of_scope_edit_refused | invalid input | Implementation requires editing file B not in allow-list | `/slo-execute M1` runs | Skill pauses, surfaces the conflict, asks to widen milestone or split |
| bdd_first_enforced | happy path | Milestone has BDD scenarios | `/slo-execute M1` runs | Test files exist and fail for the right reason BEFORE any production code is written |
| evidence_log_filled | happy path | Milestone complete | `/slo-execute M1` ends | Every row of evidence log has Actual Result filled |

**Feature: /slo-verify**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| happy_path_exercised | happy path | UI milestone with a form | `/slo-verify` runs | Playwright actually submits the form; screenshot recorded |
| empty_state_checked | empty state | UI with list + empty state | `/slo-verify` runs | Empty state rendered and screenshot recorded |
| bug_generates_regression_test | partial failure | QA finds a bug | `/slo-verify` runs | Regression test is added; fix is committed separately; bug reproduces without fix, passes with fix |

#### Definition of Done

- `/slo-execute` successfully drives M1 of a test runbook against a scratch repo
- `/slo-verify` catches a planted bug and generates a regression test for it
- File-allow-list enforcement blocks an attempted out-of-scope edit in the BDD scenario

---

### Milestone 8 — Power tools

**Goal**: Ship `/slo-second-opinion`, `/slo-freeze`, `/slo-resume`, `/slo-ship`.

**Context**: These are small compared to M5/M6/M7 but close the sprint loop. Each is one `SKILL.md`.

**Important design rule**: Power tools must be independent — no power tool is a prerequisite for any core skill.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Varies per tool |
| Outputs | Varies per tool |
| Files allowed to change | `skills/slo-second-opinion/**`, `skills/slo-freeze/**`, `skills/slo-resume/**`, `skills/slo-ship/**` |
| New dependencies allowed | `none` (shell-out to `codex` or `gemini` for second-opinion; already on user's PATH if they have those installed) |

#### BDD Acceptance Scenarios

| Scenario | Tool | Category | Given | When | Then |
|---|---|---|---|---|---|
| codex_missing_clean_exit | `/slo-second-opinion` | dependency failure | `codex` not on PATH | Skill runs | Clean exit with install hint |
| cross_model_diff | `/slo-second-opinion` | happy path | Both Claude and Codex reviewed | Skill runs | Output shows overlap + unique findings |
| freeze_blocks_outside_edit | `/slo-freeze` | happy path | `/slo-freeze src/auth/` active | Edit attempted in `src/payments/` | Edit blocked with clear message |
| resume_finds_next_milestone | `/slo-resume` | happy path | Runbook has M1 done, M2 not_started | `/slo-resume` | Identifies M2; suggests `/slo-execute M2` |
| ship_opens_pr | `/slo-ship` | happy path | Branch has commits ahead of main | `/slo-ship` | PR opened; description summarizes milestones, not line counts |

#### Definition of Done

- All 4 power tools installed and discoverable
- Each passes its BDD scenarios
- None required for M1–M7 to function (verified by uninstalling power tools and re-running earlier E2E tests)

---

### Milestone 9 — Self-hosting validation

**Goal**: Use the full skill pack to design and plan a new feature *for SunLitOrchestra itself*, end-to-end, producing a v3 runbook that passes `/slo-critique`. If the pack cannot build the pack, the pack is not done.

**Context**: The ultimate dogfood. Also the place where any rough edges surface — expect to need targeted fixes across earlier milestones.

**Important design rule**: Pick a real feature that actually needs shipping. Candidate: an Apalache integration for `/slo-tla` (since M5 ships TLC only). This exercises `/slo-architect` (stack decision), `/slo-tla` (self-reference), and `/slo-plan` (writing a new runbook).

**Refactor budget**: `Targeted refactor permitted for issues surfaced during self-hosting` — the whole point is to surface and fix pain.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | A real feature pitch for SLO (proposed: full Apalache integration) |
| Outputs | A new `docs/RUNBOOK-<feature>.md` produced entirely through the skill pipeline; `docs/slo/critique/<feature>.md` from `/slo-critique` |
| Files allowed to change | Any skill file touched by a surfaced rough edge; the new runbook file itself |
| Forbidden shortcuts | No hand-authoring sections that the skills should have produced; no skipping `/slo-critique` |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| end_to_end_produces_runbook | happy path | Feature pitch, empty `docs/RUNBOOK-apalache.md` | Run full pipeline `/slo-ideate` → `/slo-retro` | Complete v3 runbook produced; `/slo-critique` finds ≤ 5 concrete findings |
| pipeline_interruption_resumes | dependency failure | User quits after `/slo-architect` | `/slo-resume` next day | Correctly identifies `/slo-tla` (if `tla_required`) or `/slo-plan` as next step |
| pain_points_logged_and_fixed | happy path | Any rough edge surfaced | Fix committed | Each fix has a regression test or a lessons-file entry |

#### Definition of Done

- Full runbook produced end-to-end through the skills
- `/slo-critique` findings addressed or explicitly deferred with reason
- All surfaced rough edges have fixes or follow-up tickets
- README.md updated to recommend the skill-based UX as the primary interface
- `sldo-plan` and `sldo-run` READMEs clearly labeled "batch mode / legacy"

---

### Milestone 10 — Context Hub integration (`chub` + `get-api-docs`)

**Goal**: Vendor the `get-api-docs` SKILL.md from [andrewyng/context-hub](https://github.com/andrewyng/context-hub) into this repo under `skills/get-api-docs/`, teach `sldo-install` to handle third-party skills, and wire it into CLAUDE.md so every session can reach for curated API docs instead of guessing. Also install it onto the user's laptop at `~/.claude/skills/get-api-docs/SKILL.md` (already done as part of this runbook's first pass — preserve it).

**Context**: Context Hub (by Andrew Ng / aisuite) is a CLI (`chub`) that gives agents curated, versioned API docs with local annotations and feedback. It's complementary to SLO: where `/slo-research` does market/competitor research, `get-api-docs` does third-party library reference lookups. Installing it as a sibling skill makes the whole pack stronger.

**Important design rule**: `get-api-docs` is third-party. We vendor the SKILL.md into `skills/` for versioning and install-time discoverability, but we do not modify the skill body. If upstream updates, we re-vendor wholesale. Attribution and license preserved in a `skills/get-api-docs/UPSTREAM.md` file.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `chub` CLI on PATH; upstream SKILL.md body |
| Outputs | `skills/get-api-docs/SKILL.md` (vendored); `skills/get-api-docs/UPSTREAM.md` (attribution + source commit hash); CLAUDE.md line referencing the skill; `sldo-install` picks it up |
| Interfaces touched | `sldo-install` (generic skill pickup — should already work if M1's install logic is correct) |
| Files allowed to change | `skills/get-api-docs/**`, `CLAUDE.md` (or create it), `README.md` |
| Files to read before changing anything | Upstream `andrewyng/context-hub` repo `cli/skills/get-api-docs/SKILL.md` and `LICENSE` |
| New files allowed | `skills/get-api-docs/SKILL.md`, `skills/get-api-docs/UPSTREAM.md`, `CLAUDE.md` (if not present) |
| New dependencies allowed | Runtime: `chub` CLI (user-installed via `npm install -g @aisuite/chub`; not a Cargo dep). `sldo-install` should detect `chub` via `which` and print install hint if missing. |
| Migration allowed | `no` |
| Compatibility commitments | `sldo-install` from M1 must install `get-api-docs` with zero code changes. If it can't, that's an M1 bug — fix in M1 retrospectively, not here. |
| Forbidden shortcuts | No modifying the vendored SKILL.md body. No silent failure if `chub` is missing. No bundling the chub CLI. |

#### Out of Scope / Must Not Do

- Do not attempt to wrap `chub` as a Rust crate.
- Do not auto-install `chub` via npm — that's the user's call. Tell, don't fix.
- Do not build a cross-skill "chub is in use" state tracker.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/get-api-docs/SKILL.md` | NEW: verbatim copy of upstream (with commit hash recorded in UPSTREAM.md) |
| `skills/get-api-docs/UPSTREAM.md` | NEW: attribution, source URL, commit hash, license notice |
| `CLAUDE.md` | NEW (or append): list `get-api-docs` in the available-skills section, note it requires `chub` on PATH |
| `README.md` | Add a "Third-party skills included" section |

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| vendored_skill_installs | happy path | `sldo-install` from M1 | `sldo-install` runs | Symlink at `~/.claude/skills/get-api-docs/` points into repo |
| chub_missing_hint | dependency failure | `chub` not on PATH | `sldo-install --verify` runs | Warning printed: "`chub` CLI not found — install with `npm install -g @aisuite/chub`" |
| chub_present_no_warning | happy path | `chub --version` succeeds | `sldo-install --verify` runs | No warning; exit 0 |
| upstream_hash_recorded | happy path | Vendoring the skill | We write UPSTREAM.md | File contains source URL, commit SHA, fetched date |

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_slo_sp_m10.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `test_get_api_docs_installs_like_first_party` | Third-party vendored skill is installed by the same code path as first-party skills | Symlink exists; manifest entry present; no special-case code needed |
| `test_verify_detects_missing_chub` | The verify subcommand reports missing CLI | When `PATH` excludes chub, verify exits non-zero with install hint |

#### Compatibility Checklist

- [ ] `/slo-*` skills still install correctly alongside `get-api-docs`
- [ ] `sldo-install` doesn't special-case third-party skills (generic logic)
- [ ] Existing `~/.claude/skills/get-api-docs/SKILL.md` on the user's laptop is preserved or overwritten only on `--force`

#### Smoke Tests

- [ ] `chub --version` works
- [ ] After `sldo-install`, `ls ~/.claude/skills/get-api-docs/SKILL.md` resolves to the vendored copy
- [ ] `skills/get-api-docs/UPSTREAM.md` contains a real commit hash from upstream
- [ ] `git status` shows no untracked test artifacts

#### Definition of Done

- All BDD scenarios pass
- E2E tests green
- Vendored SKILL.md matches the hash recorded in UPSTREAM.md (verifiable by re-fetching)
- CLAUDE.md (or README) tells future sessions about `get-api-docs`
- Lessons + completion files written

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | Add `sldo-install` + `skills/` to architecture | Add "Skill pack (experimental)" section | (none expected) | — |
| 2 | Add `/slo-ideate`, `/slo-retro` to component summary | Add 2 skills to skill list | (none expected) | — |
| 3 | Add `/slo-research` + data flow to `sldo-research` | Add 1 skill | (none expected) | — |
| 4 | Add architect + plan data flows | Add 2 skills | (none expected) | — |
| 5 | Add `/slo-tla` + `~/.sldo/tla/` cache + external tool boxes | Add `/slo-tla` + TLA+ prereq section | Add `specs/*.out`, `states/` | `docs/slo/design/tla-tool-cache.md` (new) |
| 6 | Add 4 critique sub-personas | Add 1 skill (parent) | (none expected) | — |
| 7 | Add execute + verify | Add 2 skills | (none expected for ours; target-repo Playwright artifacts are target-repo concern) | — |
| 8 | Add 4 power tools | Add 4 skills | (none expected) | — |
| 9 | Finalize diagram; mark as v0.1 | Promote skill UX to primary; demote batch CLIs | (surface-dependent) | Update `docs/skill-pack-catalog.md` if decisions changed |
| 10 | Add "External skills vendored" note | Add "Third-party skills included" section w/ chub prerequisite | (none expected) | `skills/get-api-docs/UPSTREAM.md` new; `CLAUDE.md` new/updated |

---

## Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.
