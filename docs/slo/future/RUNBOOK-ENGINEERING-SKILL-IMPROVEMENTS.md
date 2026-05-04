# Engineering Skill Improvements — SunLitOrchestrate (AI-First Runbook v3)

> **Purpose**: Decompose monolithic engineering SKILL.md files (`/slo-sast`, `/slo-tla`, `/slo-plan`); seed `references/templates/` shared library; harden `/slo-freeze` with a settings.json PreToolUse hook; add per-skill `evals/`; gate every security-engineering claim through a research-validation source hierarchy.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. Every milestone closes the structural-contract test before the next begins.
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [docs/slo/design/engineering-skill-improvements-overview.md](design/engineering-skill-improvements-overview.md), [docs/slo/idea/engineering-skill-improvements.md](idea/engineering-skill-improvements.md), [docs/slo/research/engineering-skill-improvements/synthesis.md](research/engineering-skill-improvements/synthesis.md), [Issue #21](https://github.com/kerberosmansour/SunLitOrchestrate/issues/21), [Issue #22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22), 2026-04-27 skill-pack review

---

## Runbook Metadata

- **Runbook ID**: `engineering-skill-improvements`
- **Prefix for test files and lessons files**: `eng-imp`
- **Primary stack**: Markdown + Rust (`crates/sldo-install`) + `update-config` skill (settings.json mutation)
- **Primary package/app names**: `sldo-install`, `skills/slo-sast`, `skills/slo-tla`, `skills/slo-plan`, `skills/slo-freeze`, `skills/slo-execute`, `skills/slo-verify`, `references/templates/`, `references/sast/`
- **Default test commands**:
  - Workspace tests: `cargo test --workspace`
  - Specific install tests: `cargo test -p sldo-install`
  - Build: `cargo build --workspace`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable**:
  - Every existing SKILL.md is install-symlinked unchanged (path / filename preserved).
  - Existing `references/sast/` per-stage references (parser-contract, stack-detection-contract, etc.) preserved.
  - The four hard-block predicate IDs in `references/biz/triage-gate.md` (immutability locked).
  - `/slo-architect`'s `~~~text` user-string fence rule.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `references/templates/` shared library + research-validation prereq landed | `done` | 2026-05-04 | 2026-05-04 | `docs/slo/lessons/eng-imp-m1.md` | `docs/slo/completion/eng-imp-m1.md` |
| 2 | `/slo-sast` decomposition into thin SKILL.md + `methodology-m1..m5.md` | `done` | 2026-05-04 | 2026-05-04 | `docs/slo/lessons/eng-imp-m2.md` | `docs/slo/completion/eng-imp-m2.md` |
| 3 | `/slo-tla` decomposition + Apalache pin in `tools.toml` | `done` | 2026-05-04 | 2026-05-04 | `docs/slo/lessons/eng-imp-m3.md` | `docs/slo/completion/eng-imp-m3.md` |
| 4 | `/slo-plan` per-milestone authoring extracted; soft line-cap structural-contract test | `not_started` | | | | |
| 5 | Per-skill `evals/` infrastructure + `/slo-freeze` PreToolUse hook + cross-skill polish | `not_started` | | | | |

---

## End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────┐
│                  SunLitOrchestrate skill pack                        │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │  references/templates/  (NEW — M1)                          │     │
│  │   - intake-checklist.md                                     │     │
│  │   - restate-and-confirm.md                                  │     │
│  │   - citation-discipline.md  ← source-hierarchy rule         │     │
│  │   - tool-safety-section.md                                  │     │
│  │   - output-frontmatter.md                                   │     │
│  │   - escalation.md                                           │     │
│  │   - eval-cases.md                                           │     │
│  │   - heuristic-numbers-discipline.md                         │     │
│  └────────────────────────────────────────────────────────────┘     │
│                              ▲                                       │
│           ┌──────────────────┼──────────────────┐                    │
│           │                  │                  │                    │
│  ┌────────┴────────┐ ┌───────┴────────┐ ┌──────┴───────┐           │
│  │ skills/slo-sast/ │ │ skills/slo-tla/ │ │ skills/slo-  │           │
│  │  SKILL.md (lean)│ │  SKILL.md (lean)│ │  plan/        │           │
│  │  references/    │ │  references/    │ │  references/  │           │
│  │   methodology-  │ │   methodology-  │ │   methodology-│           │
│  │   m1..m5.md     │ │   *.md          │ │   milestone-  │           │
│  │   (NEW — M2)    │ │   (NEW — M3)    │ │   authoring.md│           │
│  │                 │ │   tools.toml    │ │   (NEW — M4)  │           │
│  │                 │ │   (Apalache pin)│ │               │           │
│  └─────────────────┘ └─────────────────┘ └──────────────┘           │
│                                                                      │
│  ┌─────────────────────────────────────────────────────┐            │
│  │  Per-skill evals/  (NEW — M5)                       │            │
│  │   skills/<skill>/evals/                             │            │
│  │     happy-path.md / missing-context.md / ...        │            │
│  └─────────────────────────────────────────────────────┘            │
│                                                                      │
│  ┌─────────────────────────────────────────────────────┐            │
│  │  .claude/settings.json  PreToolUse hook (M5)        │            │
│  │   - reads ~/.sldo/freeze-scope.txt                  │            │
│  │   - blocks Edit/Write/NotebookEdit outside scope    │            │
│  │   - reuses `update-config` skill for mutation       │            │
│  └─────────────────────────────────────────────────────┘            │
│                                                                      │
│  ┌─────────────────────────────────────────────────────┐            │
│  │  Soft line-cap structural-contract test (M4)        │            │
│  │   crates/sldo-install/tests/e2e_eng_imp_m4.rs       │            │
│  │   - asserts every SKILL.md ≤ 200 lines OR carries   │            │
│  │     `# soft-cap-exception: <reason>` pragma         │            │
│  └─────────────────────────────────────────────────────┘            │
│                                                                      │
│  Legend:  ─── existing    NEW (this runbook)    ▶ data flow        │
└──────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone | Key Interfaces |
|---|---|---|---|
| `references/templates/` (8 files) | Shared cross-skill discipline patterns | M1 | Cited by every SKILL.md update in M2-M5 |
| `references/templates/citation-discipline.md` | Source-hierarchy rule for security-engineering claims (research-validation prereq) | M1 | Cited by `/slo-sast`, `/slo-tla`, `/slo-rulegen`, `/slo-verify`, `/slo-research` |
| `skills/slo-sast/SKILL.md` (thin) + `references/methodology-m1..m5.md` | Decomposed sast skill | M2 | Install-symlinked unchanged path |
| `skills/slo-tla/SKILL.md` (thin) + `references/methodology-*.md` + `tools.toml` (Apalache pin) | Decomposed tla skill + Apalache version pin | M3 | Install-symlinked unchanged path |
| `skills/slo-plan/references/methodology-milestone-authoring.md` | Per-milestone authoring sub-procedure | M4 | Cited from SKILL.md |
| `crates/sldo-install/tests/e2e_eng_imp_m4.rs` | Soft line-cap structural-contract test | M4 | `cargo test --workspace` baseline |
| `skills/<skill>/evals/<case>.md` (per-skill) | Documented eval expectations (7 categories) | M5 | Future runtime harness consumes |
| `.claude/settings.json` PreToolUse hook | Hard-enforces `/slo-freeze` scope | M5 | Mutated via `update-config` skill |
| `~/.sldo/freeze-scope.txt` | Session-state file for freeze scope | M5 | Read by PreToolUse hook |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Citation consultation | SKILL.md prose | `references/templates/citation-discipline.md` | Markdown link, runtime-read-by-agent | M1 |
| Per-stage methodology load | `/slo-sast` SKILL.md | `references/methodology-m<N>.md` | Markdown link, on-demand | M2 |
| Apalache version verification | `/slo-tla` prereq cascade | `tools.toml` + SHA-256 file | Local file read | M3 |
| Soft line-cap enforcement | `cargo test` | every SKILL.md | structural-contract test | M4 |
| Eval-case enumeration | `/slo-<skill>` invocation | `skills/<skill>/evals/<case>.md` | Markdown link | M5 |
| Freeze-scope check | PreToolUse hook | `~/.sldo/freeze-scope.txt` + tool-call file path | Hook script | M5 |

---

## High-Level Design for Formal Verification (TLA+ Section)

`tla_required: false`

No concurrent actors, no distributed state, no ordering guarantees. Decomposition is textual refactoring; the PreToolUse hook is single-session, single-actor; the soft line-cap test is a deterministic markdown grep. Per `/slo-tla`'s suitability gate, this is the wrong tool here.

---

## Global Execution Rules

See [docs/slo/templates/runbook-template_v_3_template.md §"Global Execution Rules"](templates/runbook-template_v_3_template.md). Project-specific overrides:

- The four hard-block predicate IDs in `references/biz/triage-gate.md` are **immutable** by structural-contract test; this runbook does not touch them.
- Every claim in modified SKILL.md prose touching security-engineering content (Pass 4, ZAP / Dastardly / `cargo audit`, semgrep rule provenance, SAST workflow safety contract) must follow `references/templates/citation-discipline.md`'s source hierarchy. Unverifiable claims removed, not weakened.

## Global Entry / Exit Rules

See template. Specifics:

- Baseline: `cargo test --workspace`. Confirm green before each milestone.
- M1 must close before M2 starts (templates are upstream dependency).
- After M2: `wc -l skills/slo-sast/SKILL.md` ≤ ~100 (thin orchestrator).
- After M3: `wc -l skills/slo-tla/SKILL.md` ≤ ~150 (thin orchestrator + suitability gate).

---

## Background Context

### Current State

The 2026-04-27 skill-pack review identified that `/slo-sast` (296 lines), `/slo-tla` (323 lines), and `/slo-plan` (132 lines with 15-step milestone authoring sub-procedure) are monolithic SKILL.md files. Under context pressure, agents read less of long SKILL.md files and skip key gates.

Across the engineering-side skills, there is no shared `references/templates/` library. Every SKILL.md repeats its own version of intake, restate-and-confirm, citation discipline, tool-safety boilerplate, output frontmatter. When a pattern improves in one skill, the others don't inherit.

`/slo-execute`'s allow-list rule and `/slo-freeze`'s scope lock are LLM-compliance disciplines, not filesystem boundaries. A motivated agent (or one under context pressure) can edit out-of-scope files.

The project owner explicitly flagged a research-validation discipline: every claim touching security-engineering content must cite primary authoritative sources at pinned versions, not paraphrased commentary.

### Problem

1. **Monolithic SKILL.md under context pressure** — `/slo-sast` (296 lines), `/slo-tla` (323), `/slo-plan` (132). An agent invoked for `/slo-sast` M3 reads M1+M2+M4+M5 contract content competing for attention.
2. **Cross-skill drift on common patterns** — every SKILL.md re-states intake / restate / citation / tool-safety / output-frontmatter / escalation / eval-cases boilerplate.
3. **Prose-level enforcement of allow-list / freeze** — relies on LLM compliance.
4. **No per-skill evals** — structural-contract tests assert documented shape; they don't assert agent behavior under happy / missing-context / ambiguous / adversarial / outdated / tool-failure / high-risk inputs.
5. **Security-engineering claims unverifiable from training memory** — citation hierarchy + research-validation discipline missing.
6. **Tooling polish drift** — `/slo-second-opinion` shows raw provider output without disclaimer; `get-api-docs` has no `chub` failure handling; `/slo-research` doesn't capture `--help` output before dispatch; `/slo-talk-to-users` git-discovery commands implicit; `/slo-tla`'s Apalache version is a URL, not a SHA pin; `/slo-verify` Pass 4 capitalised-bigram FP triage flow is undocumented.

### Target Architecture

See "End-to-End Architecture Diagram" above.

### Key Design Principles

0. **Over-engineering for simplicity**: per [`docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`](PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md), the LLM-driven engineering pipeline can sustainably carry MORE discipline than a human-driven equivalent. R2 embodies the paradigm: 8 shared templates + 5+ methodology references + per-skill evals (7 categories) + research-validation source hierarchy + structural-contract tests at multiple layers. A human team would resist this many layered defenses; the LLM pipeline absorbs them, and the engineering output is simpler (cleaner SKILL.md, better-cited recommendations, reproducible structural-contract tests). Balance is via context-window discipline (soft line-cap 200 lines, methodology files ≤ 500 lines, lazy-load references).

1. **Templates at repo root, methodology files inside skills**: `references/templates/` is sibling to `references/biz/` and `references/sast/` (excluded from `discover_skills()`); `skills/<skill>/references/methodology-*.md` is sibling to SKILL.md (included by install symlinks).
2. **Source hierarchy is non-negotiable**: tool's own docs at pinned version → tool repo `README` / `CHANGELOG` at pinned commit → upstream advisory DB docs → conference talks / academic papers (named author + year) → vendor blog posts (secondary) → never Stack Overflow.
3. **Unverifiable claims removed, not weakened**: "approximate, please verify" is the failure mode.
4. **Soft line-cap with documented exceptions**: `# soft-cap-exception: <reason>` SKILL.md frontmatter pragma allowed; structural-contract test rejects exceptions without the reason.
5. **PreToolUse hook is project-level (`.claude/settings.json`)**, not global; uses `update-config` skill for mutation; reads `~/.sldo/freeze-scope.txt` session state.
6. **Evals are documented expectations until the runtime harness lands**: forward-compatible file shape; manual run today; harness runs them later without rewrites.
7. **Decomposition preserves install-symlink semantics**: `discover_skills()` walks `skills/<name>/SKILL.md`; per-skill `references/` subdirectories are sibling-installed.

### What to Keep

- Every existing SKILL.md install path (no renames; symlinks preserved).
- All existing `references/sast/`, `references/biz/` content (decomposition adds new methodology files; doesn't replace authority files).
- The four hard-block predicate IDs in `references/biz/triage-gate.md` (immutable).
- `/slo-architect`'s `~~~text` user-string fence rule (load-bearing for citation-discipline.md).
- All existing structural-contract tests in `crates/sldo-install/tests/`.

### What to Change

- `references/templates/` (8 NEW files) — M1.
- `skills/slo-sast/SKILL.md` (slimmed to ~100 lines) + `skills/slo-sast/references/methodology-m1..m5.md` (5 NEW) — M2.
- `skills/slo-tla/SKILL.md` (slimmed to ~150) + `skills/slo-tla/references/methodology-*.md` (4 NEW) + `tools.toml` updated with Apalache pin — M3.
- `skills/slo-plan/references/methodology-milestone-authoring.md` (NEW) + `crates/sldo-install/tests/e2e_eng_imp_m4.rs` (NEW soft line-cap test) — M4.
- Per-skill `skills/<skill>/evals/<case>.md` files for highest-risk skills — M5.
- `.claude/settings.json` PreToolUse hook for `/slo-freeze` — M5.
- `/slo-freeze`, `/slo-second-opinion`, `get-api-docs`, `/slo-research`, `/slo-talk-to-users`, `/slo-verify` SKILL.md prose updates — M5.
- `references/freeze/hook-setup.md` (NEW) — M5.

### Global Red Lines

Standard set; in addition:

- **Research-validation discipline applies to every milestone with a security-engineering claim**. Unverifiable claims removed.
- The four hard-block predicate IDs are immutable.
- No silent removal of any existing SKILL.md content during decomposition (move into methodology file with diff visible).
- The PreToolUse hook is opt-in per project; no global mutation.
- No `--no-verify` on git commits, no force-pushes, no `gh pr create --repo`.

---

## BDD and Runtime Validation Rules

See template. Project specifics:

### Required Test Coverage Categories

For each milestone:

- happy path (decomposition / template / hook / eval works as documented)
- empty state (skill invoked with no input where applicable)
- dependency failure (Apalache missing, settings.json mutation tool unavailable, etc.)
- backward compatibility (existing SKILL.md prose preserved; install symlinks unchanged)
- abuse case (tm-eng-skill-improvements-abuse-1/2/3 from design overview)

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| SKILL.md decomposition tests | `tests/e2e_eng_imp_m<N>.rs` | `crates/sldo-install/tests/` |
| Soft line-cap test | `tests/e2e_eng_imp_m4.rs` | same |
| Per-skill eval shape tests | `tests/e2e_eng_imp_evals.rs` | same |
| PreToolUse hook tests | `tests/e2e_eng_imp_freeze_hook.rs` | same |

---

## Dependency, Migration, and Refactor Policy

See template. **No new runtime dependencies in this runbook.** No schema migrations.

Refactor budget per-milestone in milestone sections.

---

## Evidence Log Template

See template.

## Self-Review Gate

See template.

## Lessons + Completion Templates

See template (`docs/slo/lessons/eng-imp-m<N>.md`, `docs/slo/completion/eng-imp-m<N>.md`).

---

## Milestone Plan

### Milestone 1 — `references/templates/` shared library + research-validation prereq

**Goal**: Seed `references/templates/` with the eight common patterns; lock the source hierarchy in `citation-discipline.md`; source-verify every existing security-engineering claim in `/slo-sast`, `/slo-tla`, `/slo-rulegen`, `/slo-verify`, `/slo-research` SKILL.md files about to be decomposed; update those SKILL.md files to cite from the templates.

**Context**: This milestone is foundational. Every downstream milestone (M2-M5) consumes from M1's templates. Skipping M1 forces inline-authoring of the patterns in M2 and creates rework when the templates eventually land.

**Important design rule**: Source hierarchy in `citation-discipline.md` is **non-negotiable**. Every other template file references it for its own discipline. Lock the hierarchy verbatim before any other template file is authored.

**Refactor budget**: `Targeted refactor permitted for replacing inlined statute / heuristic / discipline content in SKILL.md prose with template references`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | 2026-04-27 skill-pack review; existing SKILL.md prose for the 5 security-engineering-touching skills; existing `references/sast/` and `references/biz/` files |
| Outputs | 8 NEW `references/templates/*.md` files; 5 SKILL.md files updated to cite from templates; structural-contract test asserting cross-references |
| Interfaces touched | `references/templates/` is a NEW top-level reference subtree (sibling to `references/biz/`, `references/sast/`); 5 SKILL.md prose updates |
| Files allowed to change | `references/templates/{intake-checklist,restate-and-confirm,citation-discipline,tool-safety-section,output-frontmatter,escalation,eval-cases,heuristic-numbers-discipline}.md` (NEW), `skills/slo-sast/SKILL.md`, `skills/slo-tla/SKILL.md`, `skills/slo-rulegen/SKILL.md`, `skills/slo-verify/SKILL.md`, `skills/slo-research/SKILL.md`, `crates/sldo-install/tests/e2e_eng_imp_m1.rs` (NEW) |
| Files to read before changing anything | All 5 SKILL.md files; the 2026-04-27 review; existing `references/biz/artifact-schema.md` (output-frontmatter pattern source); `/slo-ideate`'s seven forcing questions (intake-checklist source); `/slo-architect`'s `~~~text` fence rule (citation-discipline source) |
| New files allowed | The 8 templates + the test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Every existing SKILL.md still installs; no behavior change in any skill (prose-only refactor) |
| Forbidden shortcuts | placeholder template content; missing source-hierarchy entries; "and similar disciplines" handwaving; removing inlined statute/heuristic content from SKILL.md without replacing with a template citation; emoji or smileys in template prose |
| **Data classification** | `Public` — templates and SKILL.md prose are public-tier |
| **Proactive controls in play** | OWASP C1 (Define security requirements — citation-discipline.md is the rule of validity for security claims); C5 (Validate inputs — intake-checklist.md is the structured-data destination); C9 (Logging — eval-cases.md is the audit pattern) |
| **Abuse acceptance scenarios** | `tm-eng-skill-improvements-abuse-4: a template's source-hierarchy rule is silently relaxed to admit unverified claims` — class eliminated by structural-contract test asserting the verbatim source-hierarchy text in `citation-discipline.md` |

#### Out of Scope / Must Not Do

- Decomposing any of the 5 SKILL.md files (M2-M4 jobs).
- Authoring the per-skill `evals/` directories (M5 job).
- Shipping any new skill behavior.
- Touching `references/biz/` or `references/sast/` files (those are authority docs; templates are discipline patterns).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read the 2026-04-27 skill-pack review carefully.
3. Read [`/slo-ideate`'s seven forcing questions section](../skills/slo-ideate/SKILL.md) (intake-checklist source pattern).
4. Read [`/slo-architect`'s `~~~text` fence rule](../skills/slo-architect/SKILL.md) (citation-discipline anchor).
5. Read [`references/biz/artifact-schema.md`](references/biz/artifact-schema.md) (output-frontmatter pattern source).
6. **Source-verification spike (validates the discipline only — full verification per-milestone)**: take 5 representative security-engineering claims from `/slo-sast` and `/slo-verify` SKILL.md prose; for each, attempt to verify against the source hierarchy. Document spike result in lessons file. **Per critique E-2**: the full verification of the ~20-30 security-engineering claims is folded into M2/M3/M4 per-milestone evidence-log work, not a single M1 batch — M1's spike validates the discipline before M2 ships at scale.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `references/templates/citation-discipline.md` | NEW: source hierarchy (6 tiers), bright-line "unverifiable claims removed not weakened" rule, retrieval-date-stamping convention, `last_checked:` discipline, `last_amended:` discipline for evolving sources, conflict-resolution rule when sources disagree |
| `references/templates/intake-checklist.md` | NEW: conversational-elicitation discipline (one question, push on vague, ladder, restate-and-confirm); cites `/slo-ideate` precedent; explicit-comprehension follow-up pattern; `--rapid-intake` opt-in compressed-mode pattern (forward-compatible even if not implemented in any skill yet) |
| `references/templates/restate-and-confirm.md` | NEW: pattern modeled on `/slo-execute` "Restate the milestone constraints in your own words"; structured restatement format; correction-loop with explicit re-elicitation, never silent re-interpretation |
| `references/templates/tool-safety-section.md` | NEW: version check + `--help` capture + dry-run + read-only-vs-mutating; argv-list discipline; cite `/slo-tla` prereq cascade + `/slo-sast` argv-list as exemplars; PATH-spoofing defense (`command -v` not `which`, absolute paths preferred); subprocess output capture (stdout / stderr / exit-code separately) |
| `references/templates/output-frontmatter.md` | NEW: canonical frontmatter base; per-skill schemas extend; agent-provenance fields (`agent_version`, `agent_session_id`, `created_at` ISO-8601 with TZ); cross-skill audit fields |
| `references/templates/escalation.md` | NEW: refusal patterns + routing + ambiguity-third-state; "fail-loud" vs "fail-graceful" decision rule; how to phrase a refusal (cite the predicate, name the gap, recommend the next action) |
| `references/templates/eval-cases.md` | NEW: 7-category shape (happy / missing-context / ambiguous / adversarial / outdated / tool-failure / high-risk); frontmatter schema for case files; per-category multi-example pattern (every category gets ≥ 2 distinct examples, not just one) |
| `references/templates/heuristic-numbers-discipline.md` | NEW: "every numeric claim cites a baseline file with retrieved-date" rule; per-row `last_checked:` field; stale-warning trigger; per-row `confidence:` enum (high/med/low based on source authority); `methodology_note:` field documenting how the source measured the claim |
| `references/templates/rate-limiting-discipline.md` | NEW (paradigm: comprehensive layer): per-session client-side cap pattern; adaptive backoff on observed rate-limit response codes; spillover-to-local-file fallback; cross-invocation rate-NOT-persisted rule; pattern shared by `/slo-retro` (R1 M3), `/slo-sec-libs` (R4 M4), and any future skill that calls a rate-limited remote |
| `references/templates/fallback-discipline.md` | NEW (paradigm: graceful degradation as a first-class pattern): when a remote / tool / dependency is unavailable, what is the local equivalent that preserves the user-visible discipline? Cites `LESSONS-BACKLOG.md` (R1) + `~/.cache/sldo/` patterns + cost-baseline-staleness fallback |
| `references/templates/version-pinning-discipline.md` | NEW (paradigm: comprehensive supply-chain defense): SHA-256 pinning vs tag pinning vs branch pinning trade-offs; `git rev-parse HEAD` cache integrity check pattern; refresh cadence + bump procedure; cross-cite `/slo-sast` action-SHA pin, `/slo-tla` `tla2tools.jar` pin, `/slo-sec-libs` (R4) declarations SHA pin |
| `skills/slo-sast/SKILL.md` | Update citation-discipline-touching prose to cite `references/templates/citation-discipline.md` |
| `skills/slo-tla/SKILL.md` | Same |
| `skills/slo-rulegen/SKILL.md` | Same |
| `skills/slo-verify/SKILL.md` | Same; also add Pass 4 capitalised-bigram FP triage flow citing `references/templates/escalation.md` |
| `skills/slo-research/SKILL.md` | Same; also add `sldo-research --help` capture requirement citing `references/templates/tool-safety-section.md` |
| `crates/sldo-install/tests/e2e_eng_imp_m1.rs` | NEW: structural-contract test asserting (a) all 8 template files exist + frontmatter valid; (b) each of the 5 updated SKILL.md cites at least one template file; (c) `citation-discipline.md` contains the verbatim 6-tier source hierarchy |

#### Step-by-Step

1. Author `citation-discipline.md` first. Lock source hierarchy. Verbatim 6-tier text.
2. Run source-verification spike: pick 5 claims, attempt verification, document.
3. Author the other 7 templates in parallel.
4. Update the 5 SKILL.md files to cite from templates (replace inlined boilerplate).
5. Test stub.
6. Verify structural-contract test passes.
7. Smoke tests.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: shared templates seeded; security-engineering claims source-verified**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| All 8 templates exist with valid frontmatter | happy path | repo at HEAD | structural-contract test runs | all 8 files present + frontmatter parses |
| `citation-discipline.md` contains 6-tier source hierarchy verbatim | happy path | file authored | test asserts text match | match found |
| Each updated SKILL.md cites ≥ 1 template | happy path | 5 SKILL.md files updated | grep each for `references/templates/` | all 5 grep hits succeed |
| Empty template file | empty state | template file accidentally empty | test runs | test FAILS with "frontmatter missing" |
| Source-hierarchy silently relaxed | abuse case (`tm-eng-abuse-4`) | someone edits `citation-discipline.md` to drop tier 6 | test runs | test FAILS with "verbatim hierarchy text mismatch" |
| Backward compat: skill not in update list | backward compatibility | other skills (e.g., `/slo-execute`) unchanged | full suite runs | unchanged skills still install + tests still pass |
| Existing `references/biz/` and `references/sast/` unaffected | backward compatibility | M1 closes | inspect those subtrees | no files changed |

#### Regression Tests

- `cargo test --workspace`.
- All existing SKILL.md install tests.
- The four-predicate immutability test.

#### Compatibility Checklist

- [ ] Every existing SKILL.md still installs.
- [ ] No removal of existing reference content.
- [ ] No skill behavior change.
- [ ] No prose loss (every removed line from a SKILL.md is replaced by a template citation that captures the same content).

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_eng_imp_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `templates_directory_has_all_eight_files` | Library is complete | all 8 file paths exist + frontmatter parses |
| `citation_discipline_source_hierarchy_verbatim` | Hierarchy is locked | exact text-match against expected 6-tier list |
| `five_skills_cite_templates` | SKILL.md updates landed | each SKILL.md grepped for ≥ 1 `references/templates/` link |
| `no_unverified_security_claim_remains` | Source-verification done | spike documentation in lessons file flagged for human review |

#### Smoke Tests

- [ ] Open `references/templates/citation-discipline.md`; verify 6-tier hierarchy renders.
- [ ] Open one updated SKILL.md (e.g., `slo-sast`); verify template cite resolves.
- [ ] `cargo test -p sldo-install` passes.
- [ ] `git status` clean.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All 11 templates exist with valid frontmatter.
- `citation-discipline.md` source hierarchy locked verbatim + tested.
- 5 SKILL.md files updated to cite from templates.
- Source-verification spike documented in lessons file.
- All BDD scenarios pass.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: add a "References subtrees" subsection mentioning `references/templates/`.
- README.md update: add a docs-index pointer to the templates library.

#### Notes

- Template authoring is high-bandwidth thinking work — resist copy-paste between templates. Each is a discrete discipline.

---

### Milestone 2 — `/slo-sast` decomposition

**Goal**: `/slo-sast/SKILL.md` reduced to ≤ 100 lines (pre-flight + mode dispatch + anti-patterns common across milestones); per-stage methodology extracted to `skills/slo-sast/references/methodology-m1-parser.md` through `methodology-m5-pr-creation.md` (5 new files). Every duplicated contract content in old SKILL.md replaced with one-line pointer.

**Context**: The current SKILL.md is 296 lines covering M1-M5 of the scanner-orchestration runbook. Future invocations of `/slo-sast` for any single stage drag in all 5 stages' content. Existing `references/sast/` files (parser-contract, stack-detection-contract, etc.) are authority docs and stay; the new `methodology-m<N>` files are orchestration scaffolds that cite the authority docs.

**Important design rule**: **Move, don't summarize**. Every line of the old SKILL.md that lands in a methodology file lands verbatim. Diff between old SKILL.md and (new SKILL.md ∪ all 5 methodology files) MUST be empty for prose content; only the structural reorganization changes.

**Refactor budget**: `Targeted refactor permitted for SKILL.md content extraction into per-stage references; prose moves verbatim`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `skills/slo-sast/SKILL.md` (296 lines); existing `references/sast/` authority docs; M1's `references/templates/citation-discipline.md` |
| Outputs | New thin SKILL.md (≤ 100 lines); 5 new `methodology-m<N>.md` files; structural-contract test asserting (a) verbatim prose preservation, (b) install symlink continuity, (c) every M<N> contract still cited from new SKILL.md |
| Interfaces touched | `skills/slo-sast/SKILL.md` content; new `skills/slo-sast/references/` subdirectory contents |
| Files allowed to change | `skills/slo-sast/SKILL.md`, `skills/slo-sast/references/methodology-m1-parser.md` (NEW), `methodology-m2-stack-detect.md` (NEW), `methodology-m3-emission.md` (NEW), `methodology-m4-manifest.md` (NEW), `methodology-m5-pr-creation.md` (NEW), `crates/sldo-install/tests/e2e_eng_imp_m2.rs` (NEW) |
| Files to read before changing anything | `skills/slo-sast/SKILL.md`; all `references/sast/` files; `references/templates/citation-discipline.md` (M1) |
| New files allowed | 5 methodology files + 1 test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All security disciplines (argv-list, no `pull_request_target`, action-SHA pin, `pull_request` only, `permissions: {}`, no `--autofix`, symlink-traversal defense, manifest schema regex-validation, `gh pr create` no-`--repo`) preserved verbatim across decomposition |
| Forbidden shortcuts | summarizing instead of moving prose; removing security disciplines; collapsing M1 + M2 contract content; mixing per-stage anti-patterns into the thin SKILL.md (only common-across-stage anti-patterns belong in SKILL.md) |
| **Data classification** | `Public` |
| **Proactive controls in play** | OWASP C1 (security requirements documented in methodology); C9 (audit trail via verbatim move + diff test) |
| **Abuse acceptance scenarios** | `tm-eng-skill-improvements-abuse-1: SKILL.md decomposition breaks install-symlink chain` — class eliminated by structural-contract test that runs `cargo test -p sldo-install` against the post-decomposition tree |

#### Out of Scope / Must Not Do

- Modifying any `references/sast/` authority doc.
- Changing any security discipline (argv-list, action SHAs, etc.).
- Decomposing `/slo-tla` or `/slo-plan` (M3 / M4 jobs).
- Adding new methodology content not present in the original SKILL.md.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1's lessons file.
3. Read current `skills/slo-sast/SKILL.md` end-to-end.
4. Read all `references/sast/` files.
5. Read `references/templates/citation-discipline.md` (M1).
6. Plan the cut: identify which lines stay in SKILL.md vs land in each methodology file. Document the cut plan in M2 lessons.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Reduce to: pre-flight (~10 lines), mode dispatch (~15 lines), anti-patterns common across stages (~10 lines), per-stage pointer table (~5 lines), See-also section (~5 lines). Total ≤ 100 lines. |
| `skills/slo-sast/references/methodology-m1-parser.md` | Threat-model parser scope rule + process + empty-list behavior — verbatim from current M1 section |
| `skills/slo-sast/references/methodology-m2-stack-detect.md` | Stack detection + registry fetch + rule filter — verbatim |
| `skills/slo-sast/references/methodology-m3-emission.md` | Symlink-traversal defense + emission flow + workflow safety contract + CWE-list independence + M3 anti-patterns — verbatim |
| `skills/slo-sast/references/methodology-m4-manifest.md` | Manifest schema v1.0 + preview-mode UX + rollback contract + M4 anti-patterns — verbatim |
| `skills/slo-sast/references/methodology-m5-pr-creation.md` | Re-derivation triggers + `gh pr create` discipline + dogfood test + M5 anti-patterns — verbatim |
| `crates/sldo-install/tests/e2e_eng_imp_m2.rs` | NEW: structural-contract test asserting prose preservation + install continuity + line-count cap |

#### Step-by-Step

1. Read the entire current SKILL.md and stage the decomposition mentally.
2. Test stub first.
3. Author the 5 methodology files (verbatim moves; diff-friendly).
4. Slim SKILL.md to ≤ 100 lines.
5. Verify prose-preservation test (concat all methodology files + new SKILL.md → must contain every line of old SKILL.md).
6. Verify install symlink test passes.
7. Smoke tests.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: `/slo-sast` decomposed without prose loss**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Decomposition produces 5 methodology files | happy path | M2 closes | inspect skill dir | 5 methodology files present + valid frontmatter |
| New SKILL.md ≤ 100 lines | happy path | M2 closes | `wc -l` SKILL.md | ≤ 100 lines |
| Prose preservation | happy path | concat new SKILL.md + 5 methodology files | grep against pre-M2 SKILL.md baseline | every paragraph from baseline appears in concat |
| Install symlinks unchanged | backward compatibility | `cargo test -p sldo-install` | runs | passes; `~/.claude/skills/slo-sast/` resolves to repo path |
| Security discipline preserved (argv-list) | abuse case (`tm-eng-abuse-1` related) | grep methodology-m2 + m5 for "argv-list" | runs | both files contain the discipline |
| Security discipline preserved (no `pull_request_target`) | abuse case | grep methodology-m3 for `pull_request_target` ban | runs | ban present |
| Security discipline preserved (action SHAs) | abuse case | grep methodology-m3 for action-SHA pin rule | runs | rule present |
| Symlink-traversal defense preserved (M3) | abuse case | grep methodology-m3 for "O_NOFOLLOW" or "symlink-traversal defense" | runs | rule present |
| Manifest schema regex-validation preserved (M4) | abuse case | grep methodology-m4 | runs | regex-validation rule present |
| `gh pr create` no-`--repo` preserved (M5) | abuse case | grep methodology-m5 | runs | rule present |
| **Programmatic MUST-rule extraction (per critique S-2 + paradigm)** | structural | extract every `MUST` / `MUST NOT` line from baseline SKILL.md | runs | every extracted rule appears verbatim in (new SKILL.md ∪ methodology files); no rule lost |
| Workflow-scope `permissions: {}` rule preserved | abuse case (paradigm: comprehensive enumeration) | grep methodology-m3 | runs | empty workflow-permissions rule present |
| `actions/checkout` `fetch-depth: 0` rule preserved | abuse case | grep methodology-m3 | runs | rule present |
| `semgrep ci` env-var-not-flag rule (`SEMGREP_RULES` not `--config`) preserved | abuse case | grep methodology-m3 | runs | rule present |
| No-`secrets.*`-in-analysis-job rule preserved | abuse case | grep methodology-m3 | runs | rule present |
| No-`--severity` rule preserved | abuse case | grep methodology-m3 | runs | rule present |
| `serde_yaml_ng` strict-parse rule preserved (M2) | abuse case | grep methodology-m2 | runs | rule present |
| 1 MiB YAML pre-parse cap preserved (M2) | abuse case | grep methodology-m2 | runs | cap present |
| `git rev-parse HEAD` cache integrity preserved (M2) | abuse case | grep methodology-m2 | runs | rule present |
| Default-fallback for empty stack detection preserved (M2) | abuse case | grep methodology-m2 | runs | rule present |
| Atomic-write tempdir + rename preserved (M3 emission) | abuse case | grep methodology-m3 | runs | rule present |
| File-content copy not symlink for dogfood fixture (M5) | abuse case | grep methodology-m5 | runs | ENG-6 rule present |
| Soft-cap exception NOT used | structural | `wc -l` SKILL.md ≤ 100 | runs | passes without `# soft-cap-exception:` pragma |

#### Regression Tests

- `cargo test --workspace`.
- All `references/sast/` files unmodified — `git diff` empty.
- `/slo-sast` install symlink test.

#### Compatibility Checklist

- [ ] `~/.claude/skills/slo-sast/SKILL.md` symlink unchanged.
- [ ] `~/.claude/skills/slo-sast/references/` subdirectory walks all 5 methodology files (sibling-install pattern).
- [ ] Every security discipline preserved.
- [ ] No behavior change in any `/slo-sast` invocation.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_eng_imp_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_sast_skill_md_at_or_under_100_lines` | Decomposition succeeded | `wc -l` ≤ 100 |
| `methodology_files_exist_with_frontmatter` | 5 methodology files present | all 5 paths + frontmatter parses |
| `prose_preservation_passes` | Every pre-M2 line is preserved somewhere | concat-and-diff against baseline |
| `install_symlinks_resolve` | Install integrity | `cargo test -p sldo-install` passes |
| `security_disciplines_preserved` | All key disciplines in their post-decomposition home | grep checks (8 specific disciplines) |

#### Smoke Tests

- [ ] Open `skills/slo-sast/SKILL.md`; confirm thin orchestrator readable in < 1 minute.
- [ ] Open `methodology-m3-emission.md`; confirm workflow safety contract + symlink-traversal defense readable.
- [ ] `cargo test -p sldo-install` passes.
- [ ] `git status` clean.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- `wc -l skills/slo-sast/SKILL.md` ≤ 100.
- Prose preservation test passes.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: note the per-skill-references decomposition pattern in the "References subtrees" section added in M1.
- README.md: not required.

#### Notes

- The cut plan documented in pre-flight is itself valuable for M3 and M4 — same shape applied to `/slo-tla` and `/slo-plan`.

---

### Milestone 3 — `/slo-tla` decomposition + Apalache pin

**Goal**: `/slo-tla/SKILL.md` reduced to ≤ 150 lines (prereq cascade + suitability gate + handoff + anti-patterns); per-stage methodology extracted to `references/methodology-elicitation.md`, `methodology-abstraction.md`, `methodology-counterexample.md`, `methodology-verified-design.md`. Apalache version + SHA-256 pinned in `tools.toml` alongside `tla2tools.jar`.

**Context**: `/slo-tla/SKILL.md` is 323 lines. Methodology IS genuinely sequential (better justification for length than `/slo-sast`), but the abstraction balance / state-explosion triage / counterexample translation / verified-design doc shape are all inlined and could split cleanly.

**Important design rule**: Same as M2 — move, don't summarize. The suitability gate ("is TLA+ the right tool here?") stays in SKILL.md; it's the cross-stage gate.

**Refactor budget**: `Targeted refactor permitted for SKILL.md content extraction; verbatim moves`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `skills/slo-tla/SKILL.md` (323 lines); existing `tools.toml` with `tla2tools.jar` pin; Apalache release page |
| Outputs | Thin SKILL.md (≤ 150 lines); 4 methodology files; updated `tools.toml` with Apalache version + SHA-256 |
| Interfaces touched | SKILL.md content; new methodology subdirectory; `tools.toml` extension |
| Files allowed to change | `skills/slo-tla/SKILL.md`, `skills/slo-tla/references/methodology-elicitation.md` (NEW), `methodology-abstraction.md` (NEW), `methodology-counterexample.md` (NEW), `methodology-verified-design.md` (NEW), `skills/slo-tla/tools.toml` (extend with Apalache pin), `crates/sldo-install/tests/e2e_eng_imp_m3.rs` (NEW) |
| Files to read before changing anything | `skills/slo-tla/SKILL.md`; current `tools.toml`; https://github.com/apalache-mc/apalache/releases at runbook-author time |
| New files allowed | 4 methodology files + 1 test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` (Apalache pin is additive to `tools.toml`, not a replacement) |
| Compatibility commitments | Suitability gate + state-explosion triage + abstraction balance prose preserved verbatim; existing `tla2tools.jar` pin unchanged |
| Forbidden shortcuts | summarizing methodology; removing the suitability gate from SKILL.md; using a release tag URL for Apalache (must be SHA-256) |
| **Data classification** | `Public` |
| **Proactive controls in play** | C1 (security requirements documented; methodology files capture "what counts as verified"); C5 (validate inputs — Apalache binary integrity) |
| **Abuse acceptance scenarios** | `tm-eng-skill-improvements-abuse-6: Apalache release-asset replaced upstream; SLO downloads tampered binary` — class eliminated by SHA-256 pin in `tools.toml` matching the runbook-author-time-captured value |

#### Out of Scope / Must Not Do

- Modifying TLA+ behavior or methodology semantics.
- Touching `crates/sldo-tla-sha` (legacy maintenance utility per CLAUDE.md).
- Decomposing `/slo-plan` (M4 job).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1 + M2 lessons.
3. Read current `skills/slo-tla/SKILL.md` end-to-end.
4. Visit https://github.com/apalache-mc/apalache/releases at runbook-author time; capture latest stable release version + SHA-256 of the `apalache.zip` asset (or equivalent). Document in lessons file.
5. Plan the cut.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-tla/SKILL.md` | Reduce to: prereq cascade (verbatim, ~50 lines), suitability gate (~30 lines, stays in SKILL.md as cross-stage discipline), per-stage pointer table (~10 lines), anti-patterns common across stages (~20 lines), handoff (~10 lines). Total ≤ 150 lines. |
| `skills/slo-tla/references/methodology-elicitation.md` | Q1-Q6 forcing questions + rejection patterns — verbatim |
| `skills/slo-tla/references/methodology-abstraction.md` | Abstraction balance + state-explosion triage + state-space budget rule — verbatim |
| `skills/slo-tla/references/methodology-counterexample.md` | Counterexample translation procedure + trace markdown shape — verbatim |
| `skills/slo-tla/references/methodology-verified-design.md` | Verified-design doc shape + gates — verbatim |
| `skills/slo-tla/tools.toml` | Extend with `[apalache]` section: `version`, `download_url`, `sha256` |
| `crates/sldo-install/tests/e2e_eng_imp_m3.rs` | NEW: structural-contract test asserting (a) line-count cap, (b) prose preservation, (c) Apalache pin format (`sha256` is 64-char hex, `version` is non-empty) |

#### Step-by-Step

1. Apalache release version + SHA-256 capture (pre-flight).
2. Test stub first.
3. Author the 4 methodology files (verbatim moves).
4. Slim SKILL.md to ≤ 150 lines.
5. Extend `tools.toml` with the Apalache pin.
6. Verify structural-contract test passes.
7. Smoke tests.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: `/slo-tla` decomposed; Apalache pinned**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| New SKILL.md ≤ 150 lines | happy path | M3 closes | `wc -l` | ≤ 150 |
| 4 methodology files exist | happy path | M3 closes | inspect dir | all 4 present + valid frontmatter |
| Prose preservation | happy path | concat all methodology + new SKILL.md | grep vs baseline | every paragraph preserved |
| Suitability gate stays in SKILL.md | happy path | inspect SKILL.md | grep for "Suitability gate" | match found |
| Apalache pin in `tools.toml` | happy path | inspect tools.toml | parse | `[apalache]` section with `version`, `download_url`, `sha256` |
| Apalache SHA-256 format | happy path | tools.toml | parse | `sha256` is 64-char hex |
| Tampered Apalache release | abuse case (`tm-eng-abuse-6`) | local Apalache asset SHA differs from pin | skill prereq cascade runs | refuse with clear stderr |
| Backward compat: `tla2tools.jar` pin unchanged | backward compatibility | M3 closes | tools.toml | `[tla2tools]` section unchanged |
| Install symlinks resolve | backward compatibility | `cargo test -p sldo-install` | runs | passes |

#### Regression Tests

- `cargo test --workspace`.
- All `references/sast/` files unmodified.
- `/slo-tla` install symlink test.
- M2's structural-contract test still passes.

#### Compatibility Checklist

- [ ] `tla2tools.jar` pin unchanged.
- [ ] Suitability gate still in SKILL.md.
- [ ] Prereq cascade order preserved.
- [ ] No behavior change in any `/slo-tla` invocation.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_eng_imp_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_tla_skill_md_at_or_under_150_lines` | Decomposition succeeded | `wc -l` ≤ 150 |
| `methodology_files_exist` | 4 methodology files present | all 4 paths + frontmatter |
| `apalache_pin_in_tools_toml` | Pin present + valid format | TOML parse succeeds + `sha256` is 64-char hex + `version` non-empty |
| `prose_preservation` | Every pre-M3 line preserved | concat-and-diff |
| `suitability_gate_in_skill_md` | Cross-stage gate stayed | grep match |
| `install_symlinks_resolve` | Install integrity | `cargo test -p sldo-install` passes |

#### Smoke Tests

- [ ] Open new `skills/slo-tla/SKILL.md`; readable in < 90 seconds.
- [ ] Open `methodology-abstraction.md`; state-explosion triage + abstraction balance preserved.
- [ ] `tools.toml` parses as valid TOML.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- `wc -l` ≤ 150.
- Apalache pin captured at runbook-author time and recorded.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" section.
- README.md: not required.

#### Notes

- Apalache version refresh cadence: re-capture annually (parallel to other authority-file refresh patterns). Not a milestone; documented in lessons file.

---

### Milestone 4 — `/slo-plan` decomposition + soft line-cap structural-contract test

**Goal**: `/slo-plan/SKILL.md` per-milestone authoring sub-procedure extracted to `references/methodology-milestone-authoring.md`. Soft line-cap structural-contract test added: every SKILL.md must be ≤ 200 lines OR carry a `# soft-cap-exception: <reason>` frontmatter pragma.

**Context**: `/slo-plan/SKILL.md` is 132 lines today, mostly because Step 2's "for each milestone, sequentially" 15-step sub-procedure is inlined. Extraction is straightforward; the structural-contract test is the discipline-anchor for future SKILL.md authors.

**Important design rule**: Soft cap is 200 lines (review's recommendation); exceptions require explicit `# soft-cap-exception: <reason>` pragma. CI flags exceptions for human review.

**Refactor budget**: `Targeted refactor permitted for SKILL.md content extraction + soft-cap test addition`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Existing `skills/slo-plan/SKILL.md`; M2 + M3 decomposition outputs |
| Outputs | Slim SKILL.md (≤ 80 lines projected); `references/methodology-milestone-authoring.md` (NEW); soft line-cap structural-contract test |
| Interfaces touched | `/slo-plan` SKILL.md content; new structural-contract test |
| Files allowed to change | `skills/slo-plan/SKILL.md`, `skills/slo-plan/references/methodology-milestone-authoring.md` (NEW), `crates/sldo-install/tests/e2e_eng_imp_m4.rs` (NEW) |
| Files to read before changing anything | `skills/slo-plan/SKILL.md`; M2 + M3 decomposition lessons |
| New files allowed | 1 methodology file + 1 test file |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Discipline rule "NEVER generate a whole runbook in one shot" stays in SKILL.md; gates + anti-patterns stay in SKILL.md; only Step 2's sub-procedure moves |
| Forbidden shortcuts | reducing SKILL.md below the discipline threshold (must keep the one-shot-refusal rule and the gates); soft-cap exception without reason; bypassing the structural-contract test |
| **Data classification** | `Public` |
| **Proactive controls in play** | C1 (security requirements documented in methodology); C9 (audit trail via structural-contract test) |
| **Abuse acceptance scenarios** | `tm-eng-skill-improvements-abuse-3: soft line-cap exception abused to bypass decomposition` — mitigated by requiring `# soft-cap-exception: <reason>` pragma + CI flagging exceptions for human review |

#### Out of Scope / Must Not Do

- Modifying `/slo-plan` behavior or the v3 runbook template (template's "Carry-forward from prior retros" addition is R1 M4's job).
- Adding new gates beyond what the soft-cap test asserts.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1, M2, M3 lessons.
3. Read current `skills/slo-plan/SKILL.md` end-to-end.
4. Plan the cut.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-plan/SKILL.md` | Slim to: discipline rule (NEVER one-shot), inputs / outputs, gates, anti-patterns, handoff. Step 2's per-milestone sub-procedure replaced with one-line cite to methodology file. Total ≤ 80 lines projected. |
| `skills/slo-plan/references/methodology-milestone-authoring.md` | NEW: 15-step per-milestone authoring procedure verbatim from current SKILL.md Step 2. Required Contract Block rows + abuse-case examples + Vocabulary references kept. |
| `crates/sldo-install/tests/e2e_eng_imp_m4.rs` | NEW: structural-contract test asserting (a) every SKILL.md ≤ 200 lines OR has `# soft-cap-exception:` pragma; (b) any pragma's reason is non-empty; (c) `methodology-milestone-authoring.md` exists with valid frontmatter; **(d, paradigm: extend the cap discipline) every methodology file under `skills/<skill>/references/` ≤ 500 lines OR has `# methodology-cap-exception:` pragma**; **(e) every reference under `references/templates/` ≤ 300 lines OR has `# template-cap-exception:` pragma** (templates should stay tight; long detail belongs in domain-specific authority files) |

#### Step-by-Step

1. Test stub first.
2. Author `methodology-milestone-authoring.md` (verbatim move).
3. Slim SKILL.md.
4. Add soft-cap structural-contract test.
5. Audit existing SKILL.md files: which exceed 200 lines? After M2 + M3, only those with legitimate reasons remain. Add `# soft-cap-exception: <reason>` pragma where needed.
6. Verify all tests pass.
7. Smoke tests.
8. Self-review.

#### BDD Acceptance Scenarios

**Feature: `/slo-plan` decomposed; soft line-cap enforced**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| New SKILL.md ≤ 80 lines | happy path | M4 closes | `wc -l skills/slo-plan/SKILL.md` | ≤ 80 |
| Methodology file exists | happy path | M4 closes | inspect | file present + valid frontmatter |
| Soft-cap test rejects > 200 line SKILL.md without pragma | happy path | a hypothetical SKILL.md with 250 lines and no pragma | test runs | test FAILS with "soft cap exceeded; add `# soft-cap-exception: <reason>`" |
| Soft-cap test accepts > 200 line SKILL.md with pragma + reason | happy path | SKILL.md with pragma + non-empty reason | test runs | test passes; CI flags for human review |
| Pragma without reason | abuse case (`tm-eng-abuse-3`) | SKILL.md with `# soft-cap-exception: ` (empty reason) | test runs | test FAILS with "exception reason required" |
| All existing SKILL.md pass | backward compatibility | post-M2/M3 tree | test runs over every SKILL.md | all pass (decomposed ones ≤ 200; un-decomposed ones either ≤ 200 or have pragma) |
| Discipline rule preserved | happy path | new SKILL.md | grep for "NEVER generate a whole runbook" | match found |

#### Regression Tests

- `cargo test --workspace`.
- M1, M2, M3 structural-contract tests.
- `/slo-plan` install symlink test.

#### Compatibility Checklist

- [ ] All existing SKILL.md install.
- [ ] Discipline rules preserved in SKILL.md.
- [ ] No skill behavior change.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_eng_imp_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_plan_skill_md_decomposed` | Decomposition done | `wc -l` ≤ 80 |
| `methodology_milestone_authoring_exists` | New methodology file present | path + frontmatter |
| `soft_line_cap_test_runs_for_every_skill_md` | Cap test is enabled | iterate `skills/*/SKILL.md`; assert ≤ 200 OR pragma |
| `pragma_reason_required` | Reason cannot be empty | parse pragma; reason non-empty |

#### Smoke Tests

- [ ] Open new `skills/slo-plan/SKILL.md`; readable in < 60 seconds.
- [ ] Run `wc -l skills/*/SKILL.md` to inventory; confirm post-M2/M3 tree compliant.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Soft line-cap test runs against every SKILL.md.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: extend "References subtrees" section.
- README.md: optional bullet about the discipline.

#### Notes

- The soft-cap test will catch future SKILL.md drift; this is the structural defense against re-monolithization.

---

### Milestone 5 — Per-skill `evals/` infrastructure + `/slo-freeze` PreToolUse hook + cross-skill polish

**Goal**: Per-skill `evals/<case>.md` files for the highest-risk skills (advisors + sast + tla + execute + verify); `.claude/settings.json` PreToolUse hook hard-enforcing `/slo-freeze` scope; cross-skill polish items from [Issue #22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22) E5.

**Context**: This milestone is the "polish + infrastructure layer". Eval cases are documented expectations until the runtime harness ships (deferred-follow-up in [Issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)). The PreToolUse hook closes the prose-level-discipline-only gap on `/slo-freeze`.

**Important design rule**: PreToolUse hook is **opt-in per-project**, not global. Project-level `.claude/settings.json`. Mutation via the existing `update-config` skill — no parallel mutation surface.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1's `references/templates/eval-cases.md`; M2 + M3 + M4 decomposed skills; `update-config` skill (existing); per-skill review eval cases (from 2026-04-27 review) |
| Outputs | `skills/<skill>/evals/<case>.md` per high-risk skill; `.claude/settings.json` PreToolUse hook + `references/freeze/hook-setup.md`; SKILL.md prose updates for `/slo-freeze`, `/slo-second-opinion`, `get-api-docs`, `/slo-research`, `/slo-talk-to-users`, `/slo-verify` |
| Interfaces touched | New `skills/<skill>/evals/` subdirectories; new `.claude/settings.json` hook; SKILL.md prose updates |
| Files allowed to change | `skills/{slo-legal,slo-accounting,slo-equity,slo-fundraise,slo-hire,slo-sast,slo-tla,slo-execute,slo-verify,slo-rulegen,slo-ruleverify,slo-research,slo-architect,slo-plan,slo-talk-to-users,slo-founder-check}/evals/<case>.md` (NEW), `.claude/settings.json` (NEW or extend), `references/freeze/hook-setup.md` (NEW), `skills/slo-freeze/SKILL.md`, `skills/slo-second-opinion/SKILL.md`, `skills/get-api-docs/SKILL.md`, `skills/slo-research/SKILL.md`, `skills/slo-talk-to-users/SKILL.md`, `skills/slo-verify/SKILL.md`, `references/biz/consent-script-uk.md` (NEW), `crates/sldo-install/tests/e2e_eng_imp_m5.rs` (NEW) |
| Files to read before changing anything | M1's `references/templates/eval-cases.md`; existing `update-config` skill; the 2026-04-27 review's "Suggested eval cases" sections per skill |
| New files allowed | per-skill eval files; `.claude/settings.json`; `references/freeze/hook-setup.md`; `references/biz/consent-script-uk.md`; the test file |
| New dependencies allowed | `none` (the PreToolUse hook is a small shell script invoked by Claude Code; no new crates) |
| Migration allowed | `no` |
| Compatibility commitments | Existing SKILL.md prose unchanged outside the named files; `update-config` skill unchanged; existing settings.json (if any) preserved |
| Forbidden shortcuts | global settings mutation; bypassing `update-config` skill; eval-case files without frontmatter; PreToolUse hook that shells out without argv-list |
| **Data classification** | `Internal` (eval cases may include adversarial inputs that look like attacker payloads) |
| **Proactive controls in play** | C1 (security requirements via citation-discipline + eval-cases); C5 (validate inputs — eval cases test input validation); C7 (access controls — PreToolUse hook is execution-control); C9 (audit trail — eval cases are documentation) |
| **Abuse acceptance scenarios** | `tm-eng-skill-improvements-abuse-2: PreToolUse hook bypassed via session-state file deletion` — residual; documented as "this is not a security boundary" disclaimer in `/slo-freeze` SKILL.md description; the hook is a discipline-enforcer, not a security primitive |

#### Out of Scope / Must Not Do

- Building the runtime Claude Code harness for executable evals (deferred-follow-up in [Issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)).
- Extending the `update-config` skill.
- Adding new skill behavior beyond the named polish items.
- Auto-extending the `/slo-execute` allow-list via PreToolUse hook (out-of-scope per [Issue #22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22) — the optional hook for execute is opt-in only).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M1-M4 lessons.
3. Read M1's `references/templates/eval-cases.md`.
4. Read 2026-04-27 review per-skill "Suggested eval cases" sections.
5. Read the existing `update-config` skill behavior.
6. Read [Issue #22](https://github.com/kerberosmansour/SunLitOrchestrate/issues/22) E5 polish items.

#### Files Allowed To Change

(See contract block above; per-skill eval files are batched.)

| File class | Planned Change |
|---|---|
| Per-skill `evals/<case>.md` (**per paradigm**: ALL 32 skills × **11 case categories** × ≥ 2 examples each ≈ 700 documented-expectation files) | NEW: documented expectations following M1's eval-cases.md shape. Categories: happy-path, missing-context, ambiguous-input, adversarial (×2: prompt-injection + predicate-gaming), outdated-information, tool-failure, high-risk-case, refusal-on-ambiguity, restate-and-confirm, citation-verification, multi-layer-defense. Per critique E-1's reduce-scope ask: NOT taken — paradigm absorbs the work. |
| `.claude/settings.json` | NEW (or extend): PreToolUse hook for `Edit`, `Write`, `NotebookEdit` consulting `~/.sldo/freeze-scope.txt` |
| `references/freeze/hook-setup.md` | NEW: opt-in setup procedure; cites `update-config` skill |
| `skills/slo-freeze/SKILL.md` | Add "this is not a security boundary" disclaimer; cite `references/freeze/hook-setup.md` |
| `skills/slo-second-opinion/SKILL.md` | Add "neither response is verified" disclaimer; minimum CLI versions documented |
| `skills/get-api-docs/SKILL.md` | Add "if `chub search` returns nothing, do NOT fall back to training memory" rule; `chub get` failure-mode guidance |
| `skills/slo-research/SKILL.md` | Require `sldo-research --help` capture before dispatch; cite `references/templates/tool-safety-section.md` |
| `skills/slo-talk-to-users/SKILL.md` | Specify exact git commands for git-tracked-+remote-+confidential check; cite `references/biz/consent-script-uk.md` |
| `skills/slo-verify/SKILL.md` | Add Pass 4 capitalised-bigram FP triage flow (cite `references/templates/escalation.md`); add "every scanned artifact appears as pass/fail/skipped/N/A" requirement |
| `references/biz/consent-script-uk.md` | NEW: GDPR-compliant interview consent script under legitimate-interest |
| `crates/sldo-install/tests/e2e_eng_imp_m5.rs` | NEW: structural-contract test asserting eval files exist, hook setup doc exists, SKILL.md prose updates landed |

#### Step-by-Step

1. Test stub first.
2. Author `references/biz/consent-script-uk.md` (small).
3. Author `references/freeze/hook-setup.md` with the opt-in procedure.
4. Update the 6 SKILL.md files (`slo-freeze`, `slo-second-opinion`, `get-api-docs`, `slo-research`, `slo-talk-to-users`, `slo-verify`).
5. Author `.claude/settings.json` PreToolUse hook (minimal shell script reading `~/.sldo/freeze-scope.txt`; argv-list discipline).
6. Author per-skill `evals/` directories. For each high-risk skill, write the 7 case shapes per M1's `eval-cases.md` template. For skills where a category genuinely doesn't apply, write a one-line rationale rather than a stub.
7. Verify structural-contract test passes.
8. Manual smoke: invoke `/slo-freeze`, attempt out-of-scope edit, verify hook blocks.
9. Self-review.

#### BDD Acceptance Scenarios

**Feature: per-skill evals + PreToolUse hook + cross-skill polish**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Per-skill evals exist for high-risk skills | happy path | M5 closes | inspect `skills/<skill>/evals/` | each high-risk skill has ≥ 1 eval case file (the 7 categories where applicable) |
| Eval files have valid frontmatter | happy path | M5 closes | parse each | frontmatter parses; `skill`, `case-name`, `category`, `expected-behavior` fields present |
| `.claude/settings.json` hook present | happy path | M5 closes | parse JSON | `PreToolUse` array contains hook for `Edit`, `Write`, `NotebookEdit` |
| Hook reads `~/.sldo/freeze-scope.txt` | happy path | hook configured | edit attempt outside scope | hook blocks with non-zero exit + clear message |
| Hook absent: skill prose-level fallback | backward compatibility | hook not yet configured | `/slo-freeze` invoked | works prose-level (existing behavior) |
| `/slo-freeze` SKILL.md describes "not a security boundary" | happy path | M5 closes | grep SKILL.md | match found |
| Pass 4 FP triage flow documented | happy path | M5 closes | grep `slo-verify/SKILL.md` for "capitalised-bigram" + "tier_override_reason" | match found with example |
| `chub` failure mode documented | happy path | M5 closes | grep `get-api-docs/SKILL.md` | "do NOT fall back to training memory" rule present |
| `sldo-research --help` capture documented | happy path | M5 closes | grep `slo-research/SKILL.md` | capture-before-dispatch rule present |
| `slo-talk-to-users` git commands explicit | happy path | M5 closes | grep `slo-talk-to-users/SKILL.md` | `git rev-parse --git-dir` and `git remote -v` present |
| Consent script exists | happy path | M5 closes | inspect `references/biz/consent-script-uk.md` | file present + GDPR legitimate-interest framing |
| Hook bypassed via session-state deletion | abuse case (`tm-eng-abuse-2`) | adversary deletes `~/.sldo/freeze-scope.txt` | hook fires | hook treats missing file as "no freeze active"; behavior reverts to unenforced; documented as residual + "not a security boundary" |
| Argv-list in hook | abuse case (per critique E-6) | hook script content | structural-contract test grep for shell-string patterns (`bash -c "..."`, `sh -c "..."`, `eval`) | no matches; hook uses argv-list / `[[ -f ... ]]` shell builtins only |
| `update-config` invocation is additive only (per critique S-4) | abuse case | settings.json modification request | inspect mutation | `PreToolUse` array extended, never overwritten; existing entries preserved |

#### Regression Tests

- `cargo test --workspace`.
- M1-M4 structural-contract tests.
- All existing SKILL.md install symlink tests.
- `update-config` skill unchanged.

#### Compatibility Checklist

- [ ] No skill behavior change beyond named polish items.
- [ ] PreToolUse hook is opt-in (existing repos without it work).
- [ ] All existing SKILL.md prose preserved (only the 6 named files updated).
- [ ] No new runtime dependencies.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_eng_imp_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `every_high_risk_skill_has_evals_dir` | Eval coverage for the 16 named skills | each path exists |
| `eval_files_have_valid_frontmatter` | Frontmatter shape | `skill`, `case-name`, `category`, `expected-behavior` parse for each |
| `claude_settings_pretooluse_hook_present` | Hook installed | JSON parse + `PreToolUse` array contains the hook |
| `freeze_hook_setup_doc_exists` | Setup doc present | path + frontmatter |
| `polish_items_landed` | 6 SKILL.md polish items applied | grep checks per item |
| `consent_script_exists` | Consent script present | path + GDPR framing in body |

#### Smoke Tests

- [ ] Manually invoke `/slo-freeze skills/slo-sast`; attempt to edit `skills/slo-tla/SKILL.md`; verify PreToolUse hook blocks.
- [ ] Manually invoke `/slo-freeze` with hook absent; observe prose-level fallback.
- [ ] Open `references/biz/consent-script-uk.md`; verify renders.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Per-skill evals exist for every high-risk skill (or rationale for non-applicable categories).
- PreToolUse hook installed, tested, documented as opt-in.
- 6 SKILL.md polish items landed.
- Tracker + lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md update: add a "Hooks and evals" subsection mentioning the PreToolUse hook + evals/ pattern.
- README.md update: add an "evals" pointer in the docs index.

#### Notes

- The runtime harness for executable evals is deferred to a separate runbook (per [Issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4) follow-ups). Eval cases here are documented expectations + manual run.

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | "References subtrees" subsection: list `references/templates/` | Optional bullet | none | 5 SKILL.md cross-references |
| 2 | "Per-skill references" pattern note | none | none | `skills/slo-sast/references/` (new files) |
| 3 | Same | none | none | `skills/slo-tla/references/` + `tools.toml` extension |
| 4 | Soft line-cap discipline note | Optional bullet | none | `skills/slo-plan/references/` (new file) |
| 5 | "Hooks and evals" subsection | Optional bullet | `.gitignore` for `~/.sldo/freeze-scope.txt` (if local-state file is created in target repos) | per-skill `evals/`; `references/freeze/hook-setup.md`; `references/biz/consent-script-uk.md` |

---

## Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## Carry-forward from prior retros

(Empty until R1 M3 ships and `/slo-retro` files lessons as issues.)

| Issue | Title | Suggested milestone | Status |
|---|---|---|---|
| (none yet) | | | |

---

## Paradigm-driven enhancements (per `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`)

This runbook applies the over-engineering-for-simplicity paradigm at the engineering-skill scale. The LLM-driven engineering pipeline absorbs more discipline than a human-driven equivalent because the agent does not pay the cognitive-load tax. Specific layers added because the LLM is the executor:

### Templates library expanded from 8 → 11 (M1)

Original M1 listed 8 shared templates. Paradigm-driven additions:

- `references/templates/rate-limiting-discipline.md` — formalizes the per-session cap pattern shared by R1 M3, R4 M4, and any future remote-calling skill. Rather than each skill re-deriving the discipline, it lives in one place.
- `references/templates/fallback-discipline.md` — graceful-degradation as a first-class pattern. Captures `LESSONS-BACKLOG.md`, `~/.cache/sldo/`, cost-baseline-staleness, and other fallback patterns that already exist scattered across the pack.
- `references/templates/version-pinning-discipline.md` — formalizes SHA-256 vs tag vs branch trade-offs across `/slo-sast` action-SHA pin, `/slo-tla` `tla2tools.jar` pin, R4's CycloneDX declarations SHA pin, R3's cost-baseline retrieval-date stamps.

A human team would resist 11 templates as bloat; the LLM pipeline composes from them at runtime with no marginal cost.

### Source-verification scope expanded (M1)

Critique E-2 surfaced that the M1 spike's "5 representative claims" was too small. Paradigm-driven correction (already applied in M1 pre-flight): full verification of all ~25 security-engineering claims is folded into M2/M3/M4 per-milestone evidence-log work. The spike validates the discipline; the comprehensive verification is across the runbook, not deferred.

### Programmatic MUST-rule extraction (M2 + M3)

Critique S-2 surfaced that the prose-preservation BDD's hand-written security-discipline list could drift. Paradigm-driven correction: M2 + M3 BDD now extracts every `MUST` / `MUST NOT` line from the baseline SKILL.md programmatically and asserts each appears verbatim in (new SKILL.md ∪ methodology files). 14 specific disciplines now enumerated in M2's BDD table (was 6); same comprehensive coverage in M3.

### Soft-cap discipline extended to references (M4)

Original M4 capped SKILL.md at 200 lines. Paradigm-driven extension: methodology files capped at 500 lines; templates capped at 300 lines (because templates should stay tight; long detail belongs in domain authority files). Each cap respects context-window practicality; combined, they prevent re-monolithization at any layer of the reference tree.

### Comprehensive eval coverage (M5)

Critique E-1 proposed reducing M5 evals to "highest-risk only (9 skills)". Paradigm-driven correction: NOT taken. ALL 32 skills × 11 case categories × ≥ 2 examples ≈ 700 documented-expectation files. Bounded by context window via on-demand loading (the runtime harness — when it ships — loads one eval at a time), not by author tedium. Plus 4 new categories beyond the original 7:

- `refusal-on-ambiguity` — catches the third state of gate evaluation
- `restate-and-confirm` — catches drift in the conversational pattern
- `citation-verification` — catches paraphrased-not-quoted statute / heuristic / KPI claims
- `multi-layer-defense` — validates that the 4 defense layers are present for the skill's primary risk class

Plus `expected_refusal_text:` field on every eval case so future runtime harness can byte-compare phrasing.

### Defense-in-depth across milestones

| Concern | Layer 1 | Layer 2 | Layer 3 | Layer 4 |
|---|---|---|---|---|
| SKILL.md drift after decomposition | Programmatic MUST-rule extraction (M2+M3 BDD) | Prose-preservation diff (M2+M3 BDD) | Install-symlink continuity (M2+M3 BDD) | Soft-cap structural-contract test (M4) catches re-monolithization |
| Citation hallucination | Source hierarchy in `citation-discipline.md` (M1) | Bright-line "unverifiable claims removed not weakened" (M1) | Per-claim `last_checked:` discipline (M1) | Cross-skill citation tests (M5) |
| Allow-list / freeze enforcement | Prose-level discipline in `/slo-execute` SKILL.md | `/slo-freeze` prose-level scope lock (existing) | PreToolUse hook (M5, opt-in) | Soft-cap-exception pragma audit (M4) catches drift |
| Eval coverage drift | Per-skill `evals/` directory (M5) | 11 case categories per skill | ≥ 2 examples per category | `expected_refusal_text:` field for byte-compare |
| Settings.json mutation safety | `update-config` skill is canonical mutation surface (M5) | Additive-only `PreToolUse` array extension (per critique S-4) | Hook-script argv-list test (per critique E-6) | Project-level scope (no global mutation) |

### Bounded by context-window

The paradigm's discipline-vs-context-window balance is enforced structurally: SKILL.md ≤ 200 lines (M4), methodology files ≤ 500 lines (M4 extension), templates ≤ 300 lines (M4 extension), eval cases small (~50-100 lines each), references in `references/sast/` and `references/biz/` can be unbounded (consulted by file:section, not read end-to-end).

### Ask items still open from critique

The paradigm doesn't auto-resolve every critique ask. Still open:
- E-3: PreToolUse hook framing — confirm "discipline-enforcer for honest mistakes, not adversarial bypass"
- E-5: include `/slo-architect` in M2 decomposition wave (line count check needed)
- S-3: hook fail-closed when session-state file unexpectedly missing during active freeze
