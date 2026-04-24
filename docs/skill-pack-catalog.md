# SunLitOrchestrate Skill Pack — Catalog (v0 spec)

> **Status**: draft spec. No skills implemented yet. This doc is the contract for the rebuild.
> **Audience**: whoever implements the skill pack (likely Claude Code itself, driven by this doc as M1 of a rebuild runbook).
> **Goal**: replace the Rust-CLI driver loop (`sldo-plan` + `sldo-run`) with a set of Claude Code skills that run a research → design → execute sprint, while preserving the rigor of the v3 runbook template as the canonical planning artifact.

---

## Design principles

1. **The v3 runbook is the output contract, not the tool.** Every planning skill eventually contributes to, or executes against, a v3 runbook. The template at [docs/runbook-template_v_3_template.md](runbook-template_v_3_template.md) is not being replaced.
2. **Skills are Markdown, not binaries.** Each skill is a `SKILL.md` in `skills/<skill-name>/`. Installer symlinks to `~/.claude/skills/slo-<skill-name>/` (or `./.claude/skills/` for repo-local).
3. **Reuse existing Rust backends where they earn it.** The `sldo-research` crate stays. The `sldo-plan` / `sldo-run` binaries become optional batch harnesses for overnight autonomy; they are no longer the primary interface.
4. **One skill, one role.** Don't blur personas. If you need CEO critique *and* eng critique, that's two skills — or one skill that invokes them as sub-phases, not one blended prompt.
5. **Skills write files, not vibes.** Every skill declares its outputs as concrete paths. Downstream skills read those paths.
6. **Interactivity at decision points only.** Ask when the answer can't be derived from the codebase or prior artifacts. Don't ask for preferences already recorded.

---

## Sprint flow

```
  /slo-ideate ─► /slo-research ─► /slo-architect ─► /slo-tla ─► /slo-plan ─► /slo-critique
                                                       │                          │
                                                       └── (skip if simple) ──────┘
                                                                                  ▼
  /slo-retro ◄── /slo-verify ◄── /slo-execute M<N> ◄────────────────────────────
                                       ▲
                                       └── loops per milestone until runbook tracker is all `done`
```

Power tools attach anywhere: `/slo-second-opinion`, `/slo-freeze`, `/slo-resume`, `/slo-ship`.

---

## Skill catalog

Each entry is the spec a `SKILL.md` file will encode. Format: **Persona** (who the agent becomes), **When** (trigger), **Reads**, **Writes**, **Methodology** (the non-obvious part), **Done when**.

---

### `/slo-ideate`

**Persona**: YC partner running office hours.
**When**: user has a raw idea and nothing else. Always the entry point for a new project.
**Reads**: user's natural-language pitch; optionally `docs/RUNBOOK-*.md` to check if this overlaps existing work.
**Writes**:
- `docs/idea/<slug>.md` — reframed problem, 3 implementation approaches with effort estimates, 5 capabilities the user didn't realize they were describing, recommended wedge.

**Methodology**: six forcing questions (pain-specific-not-hypothetical, who-hurts-today, what-gets-thrown-away-if-this-works, smallest-wedge, business-model-implications, what-if-this-is-a-feature-of-something-bigger). Push back on framing. Never let "an app for X" stand — interrogate until the pain is concrete.

**Done when**: the idea doc names a specific user, a specific pain moment, a specific wedge, and three candidate approaches ranked by 1-week-shippability.

---

### `/slo-research`

**Persona**: senior analyst with the existing `sldo-research` pipeline as a tool.
**When**: after `/slo-ideate`. Also standalone when the user asks "is this idea even viable."
**Reads**: `docs/idea/<slug>.md`; external web via `sldo-research` crate (which handles rate-limiting, sourcing, synthesis).
**Writes**:
- `docs/research/<slug>/dossier.md` — market, competitors, technical prior art, legal/regulatory constraints, open questions.
- `docs/research/<slug>/sources.md` — cited URLs with access dates.
- `docs/research/<slug>/synthesis.md` — "what this means for the design."

**Methodology**: do *not* re-implement research in the skill. Shell out to `sldo-research` (which already has M6/M7 synthesis + plan-readiness). The skill's job is framing the research prompt and gating the output. Flag claims without sources. Flag vibes without data.

**Done when**: dossier has ≥3 sourced competitor comparisons, ≥1 technical prior-art reference, and the synthesis ends with "the design must handle X, Y, Z because [source]."

---

### `/slo-architect`

**Persona**: staff engineer making stack choices.
**When**: after research, before any runbook-writing.
**Reads**: idea doc, research dossier, target repo (`git ls-files` to detect existing stack if this is brownfield).
**Writes**:
- `ARCHITECTURE.md` (or updates it) — component diagram, data flow, persistence boundaries, trust boundaries, external integrations.
- `docs/design/stack-decision.md` — chosen stack with explicit "we rejected X because Y" rationale.
- `docs/design/interfaces.md` — public APIs, commands, events, persisted shapes that downstream milestones must keep stable.

**Methodology**: force-decide the uncertain things now so the runbook can enforce compatibility later. If the system has concurrency, distributed state, ordering guarantees, resource ownership, or failure recovery, set a flag `tla_required: true` in the frontmatter of the design doc — `/slo-tla` checks this next.

**Done when**: ARCHITECTURE.md has a diagram the user can read at a glance, and every component has a one-line responsibility.

---

### `/slo-tla` — **formal verification (new)**

**Persona**: formal methods engineer who translates designs into TLA+ and runs TLC until invariants hold.
**When**: after `/slo-architect` when `tla_required: true`. Also runnable manually on any design doc. Also runnable against an existing runbook's TLA+ section to fill it in with a real, model-checked spec.
**Reads**: `ARCHITECTURE.md`, `docs/design/stack-decision.md`, the v3 runbook's TLA+ section template (for output shape), optionally an existing hand-written design.
**Writes**:
- `specs/<name>.tla` — the TLA+ spec.
- `specs/<name>.cfg` — TLC config (constants, invariants, properties, symmetry, bounds).
- `specs/<name>.trace.md` — any counterexamples TLC produced, translated into plain-English step-by-step scenarios.
- `docs/design/<name>-verified.md` — the validated design writeup, including: system goal, state, actions, safety properties, liveness assumptions, simplifications, TLC results (checked configs + bounds), open questions.
- Patches the v3 runbook's "High-Level Design for Formal Verification" section if a runbook exists.

**Methodology** (this is the non-obvious part — it's where "some work" lives):

1. **Elicit the right abstraction.** Most design docs over-specify (timestamps, UUIDs, payloads). The skill must reduce to the smallest set of states and transitions that captures real correctness risk. Ask: "What property are we trying to prove? What's the smallest state that can violate it?"
2. **Draft the spec in stages.** Start with the state variable set and the Init predicate. Get TLC running on a 2-actor, 2-step bound before adding complexity. Growing a spec incrementally is the only way to avoid state explosion.
3. **Separate safety from liveness.** Safety invariants check with TLC directly. Liveness needs fairness assumptions and temporal properties — state them explicitly, ask the user to confirm fairness (weak vs. strong on which actions).
4. **Counterexamples are the output, not the failure.** When TLC finds a violation, don't just report it — translate the trace into a plain-English scenario (`Actor A sends request → actor B crashes before ack → A retries → B recovers and processes twice`). Then propose the design fix. Then re-run. Loop until green or the user declares "acceptable, document the assumption."
5. **Declare the bounds.** Every TLC result must state the model bounds (N actors, M requests, K failures). "Verified" without bounds is a lie.
6. **Prefer Apalache for unbounded state.** If state explodes in TLC, suggest Apalache's symbolic model checking with a cited bound. The skill detects which tool is installed.

**Prereq cascade** (follows SLO's existing `preflight::check_claude_installed` pattern in [crates/sldo-common/src/preflight.rs](../crates/sldo-common/src/preflight.rs) — use `which::which(...)` and fail with a human-readable install hint; never silently fall back):

1. **JVM check.** `which::which("java")`. If missing → exit with a clear message: platform-specific install hint (`brew install openjdk` on macOS, `apt install default-jre` on Debian/Ubuntu, https://adoptium.net link otherwise). Do *not* attempt to install Java for the user.
2. **TLC jar check.** Look for `tla2tools.jar` at a known SLO-managed path (`~/.sldo/tla/tla2tools.jar`), then `$TLA_TOOLS_JAR` env var, then common system locations.
3. **If JVM present and jar missing → download and install the jar automatically.** Fetch `tla2tools.jar` from the pinned upstream release URL into `~/.sldo/tla/`, verify SHA-256 against a checksum shipped in the skill config (so we don't blindly trust the download), write a small shim script (`~/.sldo/tla/tlc`) that wraps `java -jar tla2tools.jar` with sensible default heap size, add that path to the skill's tool lookup. Record the version in `~/.sldo/tla/VERSION`. No bundling — the repo stays jar-free.
4. **Apalache (optional fallback for large state spaces).** Same cascade: check `which apalache-mc` → if missing, check JVM → if JVM present, offer to download the pinned Apalache release. Only run this cascade when the skill actually needs Apalache (state explosion detected), not at startup.
5. **Version pinning.** Upstream URLs + SHA-256 checksums live in a single `skills/slo-tla/tools.toml` so upgrades are one-line and auditable. No floating `latest` URLs.

This mirrors the `preflight` module's fail-loud discipline: `which` detection, human-readable errors, no silent fallbacks. The only addition vs. existing `preflight` is the conditional auto-download for the jar — justified because TLA+ is unique in having "JVM present but artifact missing" as a common benign state (users installed Java for other reasons, haven't heard of TLA+ before).

**Known hard parts** (the "some work" the user flagged):
- Translating a vague English design into a tractable TLA+ state machine is the core skill — it will take iteration on the prompt to get this consistent.
- Counterexample-to-English translation quality drives the whole UX; bad traces are worse than no verification.
- Managing state explosion: the skill needs heuristics for when to suggest symmetry reduction, state constraints, or switching to Apalache.
- Fairness assumptions are where users slip up; the skill must force an explicit decision rather than defaulting silently.

**Done when**: TLC (or Apalache) reports no violations on all declared safety + liveness properties at the declared bound, AND the `-verified.md` doc records every assumption, every simplification, and every property checked.

**Unique value**: no competitor skill pack (gstack, claude-code-skills, etc.) does formal verification. For systems where correctness actually matters — consensus, leader election, queue workers with idempotency, state-machine replication, distributed caches — this is a differentiator.

---

### `/slo-plan`

**Persona**: engineering manager translating a design into a milestone plan.
**When**: after `/slo-architect` (and `/slo-tla` if applicable).
**Reads**: ARCHITECTURE.md, design docs, verified TLA+ spec if any, v3 runbook template.
**Writes**:
- `docs/RUNBOOK-<feature>.md` — a fully populated v3 runbook. Milestone tracker, per-milestone contract blocks, BDD scenarios, E2E stubs, evidence log templates, definition of done.

**Methodology**: walks the user milestone-by-milestone, interactively. Does *not* generate the whole runbook in one shot — that's the failure mode of the current `sldo-plan` binary and the reason the rebuild exists. For each milestone: state goal → declare contract block → list files allowed to change → write BDD scenarios → confirm → move on. Max 5 milestones per feature runbook; if the scope needs more, that's a signal to split.

**Done when**: the runbook has every section from the template filled, no placeholders, and the user has approved the milestone tracker.

---

### `/slo-critique`

**Persona**: rotating reviewer — CEO, eng-lead, security, design — in that order.
**When**: after `/slo-plan`, before any implementation.
**Reads**: the new runbook, ARCHITECTURE.md, prior lessons files.
**Writes**:
- Inline edits / comments on the runbook.
- `docs/critique/<runbook-slug>.md` — findings summary: auto-fixed issues, ask-to-fix issues, hold-scope/reduce-scope recommendations.

**Methodology**: four sub-personas, one pass each.
- **CEO**: scope challenge. Is the 10-star product hiding inside this? Should we expand, hold, or reduce?
- **Eng lead**: architecture pokes. Hidden assumptions, missing failure modes, test gaps, orthogonal edits.
- **Security**: OWASP Top 10 + STRIDE applied to the design. Concrete exploit scenarios, not theoretical risks.
- **Design** (only if there's a UI surface): AI-slop detection, interaction gaps, empty-state handling.

Auto-fix obvious mechanical issues (missing compatibility checklist rows, wrong test naming, absent .gitignore updates). Ask before changing scope.

**Done when**: every finding is either auto-fixed, accepted with a captured rationale, or deferred with a written reason.

---

### `/slo-execute M<N>`

**Persona**: disciplined implementer driving one milestone.
**When**: user runs it per milestone. Replaces `sldo-run`'s inner loop.
**Reads**: runbook, prior lessons file (`docs/lessons/<prefix>-m<N-1>.md`), allowed files.
**Writes**:
- BDD test files first (verified to fail for the right reason).
- E2E runtime validation stubs.
- Production code.
- Evidence log entries (into the runbook itself).

**Methodology**: follows the v3 Global Entry Rules literally. Restates milestone constraints before coding. Refuses to widen scope. Refuses to touch files outside the allow-list without surfacing the conflict.

**Done when**: milestone's Definition of Done is satisfied and every Evidence Log row is filled.

---

### `/slo-verify`

**Persona**: QA lead with a real browser and real data.
**When**: after `/slo-execute` finishes a milestone, before `/slo-retro`.
**Reads**: the milestone's smoke tests, E2E contract, affected UI surfaces.
**Writes**:
- Regression tests for any bug it finds.
- `docs/verify/<prefix>-m<N>.md` — verification report: what was exercised, what passed, what didn't.

**Methodology**: Playwright-backed browser automation (reuse gstack's `/browse` approach or wrap it directly). Exercises the happy path, the empty state, and at least one partial-failure scenario. Every bug found gets a regression test before the fix.

**Done when**: every BDD scenario has been exercised at runtime, not just compiled.

---

### `/slo-retro`

**Persona**: eng manager closing out a milestone.
**When**: after `/slo-verify` passes.
**Reads**: evidence log, verification report, actual diff vs planned files.
**Writes**:
- `docs/lessons/<prefix>-m<N>.md` — applying the v3 template.
- `docs/completion/<prefix>-m<N>.md` — applying the v3 template.
- Updates the runbook's Milestone Tracker.
- Updates `ARCHITECTURE.md` per the Documentation Update Table.

**Methodology**: lessons file is the input to the *next* milestone's `/slo-execute`. Not optional. A milestone with no lessons file is not done.

**Done when**: tracker row is `done`, both templates filled, ARCHITECTURE.md reflects reality.

---

## Power tools

### `/slo-second-opinion`

Runs the current runbook / plan / diff through a different model (Codex CLI or Gemini). Not "vote" — "surface disagreement." Output is a diff of findings: what both models flagged, what only one flagged. User decides.

### `/slo-freeze <path>`

Lock edits to a directory. Prevents scope creep during implementation. Mirrors gstack's `/freeze`.

### `/slo-resume`

Pick up an interrupted runbook. Reads the tracker, finds the first non-`done` row, restores context (which skill to run next).

### `/slo-ship`

Sync main, run full suite, push branch, open PR with a runbook-aware description (summarizes completed milestones, not diff stats).

---

## What happens to the Rust crates

| Crate | Fate |
|---|---|
| `sldo-common` | Keep. Shared types + config. |
| `sldo-research` | **Keep as primary.** `/slo-research` wraps it. The research pipeline is already good; don't rewrite. |
| `sldo-plan` | Demote to optional batch mode. The interactive `/slo-plan` skill is the primary. `sldo-plan` can stay for unattended overnight runs. |
| `sldo-run` | Same as `sldo-plan` — optional batch harness. Eventually may be deleted, but not in the first pass. |
| `sldo-tauri` | **Park.** The Tauri desktop app does not fit the skill-pack model; it was trying to be a GUI for something that now lives inside Claude Code. Leave the `tauri` branch unmerged; don't invest more. Revisit only if there's a concrete user pulling for it. |

---

## Rebuild milestones (preview, to be written as a proper runbook)

1. **M1** — this catalog + skill-pack directory skeleton + installer script. No skills yet, just infrastructure.
2. **M2** — `/slo-ideate` + `/slo-retro` (smallest useful slice; end-to-end for tiny features without full research).
3. **M3** — `/slo-research` wrapper around `sldo-research`.
4. **M4** — `/slo-architect` + `/slo-plan`.
5. **M5** — `/slo-tla` (formal verification). This is the hard one. Gate it behind `tla_required: true` so normal projects don't need TLA+ installed.
6. **M6** — `/slo-critique` (four sub-personas).
7. **M7** — `/slo-execute` + `/slo-verify`.
8. **M8** — power tools (`/slo-second-opinion`, `/slo-freeze`, `/slo-resume`, `/slo-ship`).
9. **M9** — self-hosting: run the full pack on a fresh idea end-to-end, use the resulting runbook as the acceptance test. If SLO can't build SLO features, the pack isn't done.

Each milestone gets a v3 runbook section. The rebuild eats its own dogfood.

---

## Resolved decisions (as of 2026-04-23)

1. **Repo layout**: `skills/` at the repo root. Skills aren't code and don't belong under `crates/`.
2. **Namespace**: `/slo-*`. Explicit, non-conflicting with gstack, and users can install both packs side-by-side.
3. **Installer**: Rust binary (`sldo-install`). Consistent with the existing tool suite; users already have `cargo`; we get type safety and cross-platform support for free.
4. **TLA+ tool shipping**: no jar bundled in the repo. The `/slo-tla` skill checks JVM presence via `which::which("java")` (same pattern as [preflight.rs](../crates/sldo-common/src/preflight.rs)). If Java is missing → tell the user, exit. If Java is present but the jar isn't → download the pinned `tla2tools.jar` into `~/.sldo/tla/`, verify SHA-256, install a shim. Checksums + upstream URLs pinned in `skills/slo-tla/tools.toml`. See the `/slo-tla` "Prereq cascade" above for the full flow.
