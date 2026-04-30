# Loops + Lessons Closure — SunLitOrchestrate (AI-First Runbook v3)

> **Purpose**: Make engineering loops and business loops first-class artifacts; extend `/slo-retro` to file lessons as tracked issues; close the loop at milestone start so prior-retro issues become scope candidates for the current milestone; and make "what do I do next?" a first-class answer via the existing `/slo-resume` entrypoint.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules.
> **Prerequisite reading**: [ARCHITECTURE.md](../ARCHITECTURE.md), [docs/design/loops-and-lessons-closure-overview.md](design/loops-and-lessons-closure-overview.md), [docs/idea/loops-and-lessons-closure.md](idea/loops-and-lessons-closure.md), [docs/research/loops-and-lessons-closure/synthesis.md](research/loops-and-lessons-closure/synthesis.md), [Issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16), [Issue #17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17), [Issue #18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18)

---

## Runbook Metadata

- **Runbook ID**: `loops-and-lessons-closure`
- **Prefix for test files and lessons files**: `loops`
- **Primary stack**: Markdown + Rust (`crates/sldo-install`) + `gh` CLI
- **Primary package/app names**: `sldo-install`, `skills/slo-retro`, `skills/slo-execute`, `skills/slo-resume`
- **Default test commands**:
  - Workspace tests: `cargo test --workspace`
  - Specific install tests: `cargo test -p sldo-install`
  - E2E (markdown structural-contract tests): part of `cargo test -p sldo-install`
  - Build: `cargo build --workspace`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `docs/runbook-template_v_3_template.md` section schema (additive only)
  - `docs/lessons/<prefix>-m<N>.md` template
  - `skills/slo-retro/SKILL.md` Outputs contract (additive only)
  - `skills/slo-execute/SKILL.md` Pre-flight contract (additive only)
  - `skills/slo-resume/SKILL.md` read-only orientation contract (additive only)

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | LOOPS-ENGINEERING.md authored + cross-linked | `done` | 2026-04-30 | 2026-04-30 | [docs/lessons/loops-m1.md](lessons/loops-m1.md) | [docs/completion/loops-m1.md](completion/loops-m1.md) |
| 2 | LOOPS-BUSINESS.md authored + cross-linked | `done` | 2026-04-30 | 2026-04-30 | [docs/lessons/loops-m2.md](lessons/loops-m2.md) | [docs/completion/loops-m2.md](completion/loops-m2.md) |
| 3 | `/slo-retro` extension: classify, dedupe, file lessons as issues | `done` | 2026-04-30 | 2026-04-30 | [docs/lessons/loops-m3.md](lessons/loops-m3.md) | [docs/completion/loops-m3.md](completion/loops-m3.md) |
| 4 | `/slo-execute` pre-flight loop closure + runbook template "Carry-forward from prior retros" section with suggested lane | `not_started` | | | | |
| 5 | `/slo-resume` next-step digest + lane-aware orientation | `not_started` | | | | |

---

## End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                  SunLitOrchestrate skill pack                           │
│                                                                          │
│  ┌────────────────┐         ┌──────────────────────┐                     │
│  │ ARCHITECTURE.md│ ◀─────▶ │  docs/LOOPS-          │ ◀─────────┐        │
│  │ (existing)     │         │  ENGINEERING.md       │ ─ ─ ─ ─ ─ │        │
│  └────────────────┘         │  (M1 NEW)            │           │        │
│         ▲                    └──────────────────────┘           │        │
│         │                              ▲                        │        │
│         │                              │                        │        │
│  ┌──────┴──────────┐         ┌──────────────────────┐           │        │
│  │  Engineering    │ ◀─────▶ │  docs/LOOPS-BUSINESS │           │        │
│  │  SKILL.md files │         │  .md (M2 NEW)        │           │        │
│  └─────────────────┘         └──────────────────────┘           │        │
│                                       ▲                          │        │
│                                       │                          │        │
│                              ┌────────┴──────────┐               │        │
│                              │  Biz SKILL.md     │               │        │
│                              │  files            │               │        │
│                              └───────────────────┘               │        │
│                                                                  │        │
│  ┌────────────────┐         ┌──────────────────────┐             │        │
│  │ /slo-retro M<N>│ ────▶   │  docs/lessons/       │             │        │
│  │ (M3 EXTENDED)  │         │  <prefix>-m<N>.md     │             │        │
│  └────────────────┘         └──────┬───────────────┘             │        │
│         │                          │                              │        │
│         │ classify each lesson     │                              │        │
│         ▼                          ▼                              │        │
│  ┌─────────────────────────────────────────────────────┐         │        │
│  │  gh issue create (with confirmation)                │         │        │
│  │  destinations:                                       │         │        │
│  │    - product → current repo                         │         │        │
│  │    - upstream-OSS → resolved upstream repo          │ - - - ─ ┘        │
│  │    - slo-process → kerberosmansour/SunLitOrchestrate│                  │
│  │  fallback: LESSONS-BACKLOG.md                       │                  │
│  └────────────────────────────┬────────────────────────┘                  │
│                               │                                            │
│                               ▼                                            │
│              ┌──────────────────────────────────────┐                      │
│              │ /slo-execute M<N+k> pre-flight        │                      │
│              │ (M4 EXTENDED)                         │                      │
│              │   gh issue list --label retro-derived │                      │
│              │   surface as scope candidates         │                      │
│              └──────────────┬───────────────────────┘                      │
│                             │                                              │
│                             ▼                                              │
│              ┌──────────────────────────────────────┐                      │
│              │ /slo-resume (M5 EXTENDED)           │                      │
│              │ reads tracker + carry-forward lane  │                      │
│              │ emits one next action               │                      │
│              └──────────────────────────────────────┘                      │
│                                                                            │
│  Legend:  ─── existing    - - - new (this runbook)    ▶ data flow         │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `docs/LOOPS-ENGINEERING.md` | First-class engineering-loops doc | M1 NEW | Cross-linked from ARCHITECTURE.md + each implicated engineering SKILL.md |
| `docs/LOOPS-BUSINESS.md` | First-class business-loops doc | M2 NEW | Cross-linked from each implicated biz SKILL.md |
| `skills/slo-retro/SKILL.md` | Extended: classify each lesson, dedupe via `gh search`, file with confirmation | M3 EXTENDED | Existing lessons-file output preserved; issue filing additive |
| `skills/slo-retro/references/issue-filing-discipline.md` | argv-list + dedupe + rate-limit + fallback | M3 NEW | Cited from SKILL.md |
| `skills/slo-execute/SKILL.md` | Extended: pre-flight reads open prior-retro issues for this runbook's prefix | M4 EXTENDED | Existing pre-flight steps preserved; issue read additive |
| `docs/runbook-template_v_3_template.md` | New optional "Carry-forward from prior retros" section with suggested lane | M4 EXTENDED | Read by `/slo-plan` and `/slo-resume` |
| `skills/slo-resume/SKILL.md` | Extended: reads tracker + carry-forward section, emits one next action and lane | M5 EXTENDED | Read-only orientation contract preserved; no state mutation |
| `LESSONS-BACKLOG.md` | Local fallback for repos without `gh` available | M3 NEW | Append-only rows |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Lesson classification | `/slo-retro` | LLM (the running agent) | In-skill judgment, structured output | M3 |
| Issue dedupe | `/slo-retro` | GitHub | `gh search issues` (read-only) | M3 |
| Issue creation | `/slo-retro` (with user confirmation) | GitHub OR LESSONS-BACKLOG.md | `gh issue create` argv-list OR file append | M3 |
| Pre-flight loop closure | `/slo-execute` pre-flight | GitHub | `gh issue list` argv-list (read-only) | M4 |
| Carry-forward surfacing | `/slo-execute` | Founder/operator console | Stdout prose | M4 |
| Next-step orientation | `/slo-resume` | Founder/operator console | Read-only tracker + carry-forward summary | M5 |

---

## High-Level Design for Formal Verification (TLA+ Section)

`tla_required: false`

This work has no concurrent actors, no distributed state, no ordering guarantees, no resource ownership, no failure recovery protocols. The lesson-→issue-→carry-forward flow is sequential per milestone. Per [`/slo-tla`'s suitability gate](skills/slo-tla/SKILL.md#L120), TLA+ is not the right tool here — recommend the standard structural-contract tests in `cargo test --workspace` for correctness gating.

---

## Global Execution Rules

See [docs/runbook-template_v_3_template.md §"Global Execution Rules"](runbook-template_v_3_template.md). Project-specific overrides:

- This runbook produces **only markdown changes + skill SKILL.md prose updates + structural-contract tests**. No new Rust crates, no new runtime dependencies, no schema migrations.
- Every `gh` invocation in modified skill prose MUST follow argv-list discipline (matches `/slo-sast` M5 + `/slo-sec-libs` M3).

## Global Entry Rules (Pre-Milestone Protocol)

See template. Specifics:

- Baseline test command: `cargo test --workspace`. Confirm green before starting.
- Read [`docs/design/loops-and-lessons-closure-overview.md`](design/loops-and-lessons-closure-overview.md) before M1.

## Global Exit Rules (Post-Milestone Protocol)

See template. Specifics:

- Update [`ARCHITECTURE.md`](../ARCHITECTURE.md) with the new `docs/LOOPS-*.md` cross-references when M1 / M2 close.

---

## Background Context

### Current State

The SLO skill pack has 32 SKILL.md files, 7 reference subtrees under `references/`, 5+ active runbooks under `docs/RUNBOOK-*.md`, and an established lessons-file pattern at `docs/lessons/<prefix>-m<N>.md`. [`/slo-retro`](../skills/slo-retro/SKILL.md) writes the lessons file at milestone close; [`/slo-execute`](../skills/slo-execute/SKILL.md) reads the *previous* milestone's lessons at pre-flight (a single-step look-back, not a queued backlog).

What works:

- Lessons get written every milestone.
- The previous milestone's lessons get read by `/slo-execute` Step 1.

What does not work:

- Lessons that apply to upstream tools (Semgrep, Playwright, `cargo audit`, etc.) never get filed upstream.
- Lessons that apply to SLO itself never get filed as tracked work.
- The cyclic structures of the skill pack (engineering loops, business loops) are implicit; newcomers and freshly-loaded Claude instances cannot see them.
- A milestone in M5 cannot easily surface a lesson learned at M2 — the look-back is one-step.
- `/slo-resume` only reads the milestone tracker today; it cannot see carry-forward, issue-backed follow-ups, or whether the safest next move is tiny, milestone-sized, or "fresh runbook".
- The current documentation shape risks showing mechanism before user outcome. A loops doc can be structurally complete and still fail the practical question: "Which loop am I in, and what do I run next?"

### Problem

1. **Forward dropout in lessons**: Lessons land in markdown files but rarely get re-read past the immediately-following milestone. Concrete: M3 lessons file mentioned a naming-convention drift; M5 of the same runbook re-introduced the drift because no agent re-read M3's file before M5 work began.
2. **Lateral dropout in upstream signal**: When `/slo-execute` discovers a bug in an upstream tool, the lesson sits in a markdown file. The library-feedback loop dies in a local file. Concrete: a Semgrep CLI flag deprecation was noted in `docs/lessons/slo-sec-m4.md` but never filed against the Semgrep repo.
3. **Implicit loop structure**: ARCHITECTURE.md describes static structure; SKILL.md files describe individual moves. No document shows the cyclic feedback structures (sprint loop, security-tuning loop, lessons loop, library-feedback loop on engineering side; user-interview loop, GTM loop, pricing loop, founder-check loop on business side). Newcomers cannot answer "how does this skill pack improve itself?".
4. **No fallback when `gh` is unavailable**: Any extension to `/slo-retro` that depends on `gh` becomes a hard dependency, which violates the graceful-degradation pattern observed across the rest of the pack. A `LESSONS-BACKLOG.md` local fallback closes this gap.
5. **Orientation gap after interruptions**: the current `/slo-resume` answer is too thin. It can name the first non-`done` milestone, but not the highest-value follow-up carried from prior retros, nor whether the next move should stay tiny, consume the current milestone, or become a separate runbook.
6. **Process-theatre risk**: this runbook adds real discipline, but if the user-facing surface expands into more prose, more tables, or more decisions without reducing reviewer work, the loop becomes "more process" rather than "simpler output from a stricter machine".

### Target Architecture

See "End-to-End Architecture Diagram" above.

### Key Design Principles

0. **Over-engineering for simplicity**: per [`docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`](PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md), the LLM-driven loop pipeline can sustainably carry MORE discipline than a human-driven equivalent because LLMs do not pay the cognitive-load tax. The lessons loop itself is the canonical example: humans skip post-mortems; LLMs file them as tracked work, run dedupe via `gh search`, and surface them at the next milestone's pre-flight. The user (founder / engineer) sees a working loop; the agent does the bookkeeping. The bound is still context and ambiguity: visible ceremony still costs the user, so the extra discipline must stay mostly behind the scenes.

1. **Loop docs live next to architecture**: `docs/LOOPS-ENGINEERING.md` and `docs/LOOPS-BUSINESS.md` are top-level `docs/` siblings of `ARCHITECTURE.md`. Cross-links go both ways: ARCHITECTURE.md links to LOOPS-*.md; each implicated SKILL.md links to its loop section.
2. **Issue filing requires user confirmation**: Issue creation is publicly visible; never auto-file. This is a discipline rule, not a security boundary.
3. **`gh search` is the dedupe mechanism**: Marker (title prefix vs label vs body sentinel) is decided in M3 spike step before the dedupe rule lands.
4. **Graceful degradation when `gh` is unavailable**: `LESSONS-BACKLOG.md` local fallback. The retro skill never fails because of `gh` trouble.
5. **Carry-forward at milestone start, not retro**: The retro filing produces issues but doesn't change behavior; surfacing at the next milestone's pre-flight is what closes the loop.
6. **Argv-list discipline**: Every `gh` invocation in modified skill prose follows argv-list form. Inherited from `/slo-sast` M5 + `/slo-sec-libs` M3 disciplines.
7. **Outcome-first presentation**: loop docs and read-only orientation outputs start with "where am I / what do I do next / what outcome does this loop produce?" before diagrams, rationale, or bookkeeping internals.
8. **Maximal internal discipline, minimal visible ceremony**: every new user-visible field, section, or prompt must reduce user decisions or reviewer work. If it only helps the implementation, move it into a reference file or test instead of the point-of-use surface.
9. **Strengthen existing entrypoints before minting new verbs**: this runbook improves `/slo-resume` rather than inventing a new `/slo-help` surface. The pack should converge on one canonical "what next?" orientation path.
10. **Use scope lanes to prevent silent widening**: carry-forward items must be triaged as `micro`, `milestone`, or `fresh-runbook` so small fixes stay light-weight and large follow-ups do not smuggle themselves into the current milestone.

### What to Keep

- Existing [`docs/lessons/<prefix>-m<N>.md`](lessons/) template (additive changes only).
- Existing [`/slo-retro` Outputs contract](../skills/slo-retro/SKILL.md) (additive changes only — issue filing AFTER lessons file write).
- Existing [`/slo-execute` Pre-flight steps](../skills/slo-execute/SKILL.md) (additive changes only — issue read AFTER existing pre-flight).
- Existing structural-contract tests in [`crates/sldo-install/tests/`](../crates/sldo-install/tests/).

### What to Change

- **`docs/LOOPS-ENGINEERING.md`** — NEW (M1).
- **`docs/LOOPS-BUSINESS.md`** — NEW (M2).
- **`skills/slo-retro/SKILL.md`** — extend with classify/dedupe/file flow (M3).
- **`skills/slo-retro/references/issue-filing-discipline.md`** — NEW (M3).
- **`skills/slo-execute/SKILL.md`** — extend pre-flight with issue-list read (M4).
- **`docs/runbook-template_v_3_template.md`** — add optional "Carry-forward from prior retros" section with suggested lane (M4).
- **`skills/slo-resume/SKILL.md`** — extend with carry-forward-aware, lane-aware orientation output (M5).
- **`ARCHITECTURE.md`** — add cross-links to LOOPS-*.md (M1, M2).
- **Each engineering SKILL.md and biz SKILL.md** referenced by a loop — add a one-line cross-reference into LOOPS-*.md (M1, M2).

### Global Red Lines

Standard set; in addition:

- No auto-filing of issues without user confirmation (discipline).
- No `--no-verify` on git commits.
- No `gh pr create --repo` (confused-deputy defense from `/slo-sast` M5).
- No replacing the existing one-step lessons look-back in `/slo-execute` — additive only.

---

## BDD and Runtime Validation Rules

See template. Project specifics:

### Required Test Coverage Categories

For each milestone:

- happy path (the loop produces / consumes content as documented)
- empty state (no prior-retro issues for this runbook's prefix)
- dependency failure (`gh` not on PATH, or auth missing)
- backward compatibility (existing runbooks without "Carry-forward" section still work)
- abuse case (per-milestone — see threat-model rows in [`docs/design/loops-and-lessons-closure-overview.md`](design/loops-and-lessons-closure-overview.md))
- discoverability / orientation (an interrupted user can recover one clear next action without rereading the full runbook)

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Markdown structural-contract tests | `tests/e2e_loops_m<N>.rs` | `crates/sldo-install/tests/` |
| Issue-filing discipline tests | inline in `crates/sldo-install/src/` test modules | — |

---

## Dependency, Migration, and Refactor Policy

See template. **No new runtime dependencies in this runbook.** No schema migrations. Refactor budget per-milestone in the milestone sections.

---

## Evidence Log Template

See template.

---

## Self-Review Gate

See template. Project-specific addendum:

- Can an interrupted user recover the next concrete action in one screen of output?
- Did every newly-added visible field or section reduce user decisions or reviewer work?
- Did any methodology detail that is not needed at point-of-use get moved into a reference file instead?

---

## Lessons-Learned File Template

See template (`docs/lessons/loops-m<N>.md`).

---

## Completion Summary Template

See template (`docs/completion/loops-m<N>.md`).

---

## Milestone Plan

### Milestone 1 — `docs/LOOPS-ENGINEERING.md` authored + cross-linked

**Goal**: A `docs/LOOPS-ENGINEERING.md` that documents at minimum the four engineering loops (sprint, security-tuning, lessons, library-feedback), cross-linked from `ARCHITECTURE.md` and from each implicated engineering SKILL.md, with a newcomer-friendly "Start here" orienter at the top.

**Context**: The project has no engineering-loops doc today. Issue [#17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17) is the canonical decision record for "split from business loops, separate doc, separate concerns" + "living doc, no staleness check" + "no per-runbook declaration of loops".

**Important design rule**: Match the closest existing design-doc style (likely `docs/design/scanner-orchestration-overview.md`) for ASCII vs Mermaid choice. The doc opens with a short "Start here" orienter (`question -> loop -> first skill -> expected artifact`). Per loop: user-visible outcome, trigger, steps, exit condition, artifacts, skills involved, diagram.

**Refactor budget**: `No refactor permitted beyond direct implementation` — pure new doc + cross-link additions to existing files.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Issue #17 decisions; existing engineering SKILL.md files; existing ARCHITECTURE.md |
| Outputs | `docs/LOOPS-ENGINEERING.md` (new); cross-links from ARCHITECTURE.md + each implicated SKILL.md |
| Interfaces touched | ARCHITECTURE.md cross-references; per-skill SKILL.md cross-references |
| Files allowed to change | `docs/LOOPS-ENGINEERING.md` (new), `ARCHITECTURE.md`, each engineering SKILL.md cited as part of a loop, `crates/sldo-install/tests/e2e_loops_m1.rs` (new) |
| Files to read before changing anything | All engineering SKILL.md files; `docs/design/scanner-orchestration-overview.md` (for diagram style); Issue #17 |
| New files allowed | `docs/LOOPS-ENGINEERING.md`, `crates/sldo-install/tests/e2e_loops_m1.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All existing SKILL.md prose preserved; cross-link additions only |
| Forbidden shortcuts | placeholder loop entries, hand-waved "and others"; every loop entry must name user-visible outcome / trigger / steps / exit condition / skills involved; dumping loop internals before the orienter |
| **Data classification** | `Public` — engineering loops doc is public-tier (no real PII; the SLO repo is currently private but the doc is intended for public consumption when the repo opens) |
| **Proactive controls in play** | OWASP Proactive Controls v3 — `C1 Define Security Requirements` (loops document the engineering security-tuning loop explicitly); `C9 Implement Security Logging and Monitoring` (lessons loop is the engineering audit trail) |
| **Abuse acceptance scenarios** | `tm-loops-abuse-1: prompt injection via lesson body content` — pre-existing `~~~text` user-string fence rule from `/slo-architect` SECURITY.md template applies; documented in this runbook's design overview as residual + mitigated |

#### Out of Scope / Must Not Do

- Adding business-loops content (M2's job).
- Extending the runbook template (M4's job).
- Modifying `/slo-retro` or `/slo-execute` SKILL.md (M3 / M4).
- Removing any existing content from ARCHITECTURE.md.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read [Issue #17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17) end-to-end.
3. Survey every engineering SKILL.md to identify which ones participate in which loop.
4. Pick diagram format (ASCII vs Mermaid) by inspecting `docs/design/` for the dominant style.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/LOOPS-ENGINEERING.md` | NEW: per-loop sections with name / trigger / steps / exit / artifacts / skills / diagram |
| `ARCHITECTURE.md` | Add a "## Feedback loops" section linking to both LOOPS-*.md files (one paragraph + two bullets) |
| `skills/<each-engineering-skill>/SKILL.md` | Add a one-line "See `docs/LOOPS-ENGINEERING.md#<loop-name>` for the loop this skill participates in" cross-reference |
| `crates/sldo-install/tests/e2e_loops_m1.rs` | NEW: structural-contract test asserting (a) `docs/LOOPS-ENGINEERING.md` exists; (b) it has a section per loop; (c) every cross-referenced SKILL.md actually contains the cross-reference link |
| `.gitignore` | (Likely no change; flag if M1 produces any temp artifacts.) |

#### Step-by-Step

1. Write the structural-contract test stub in `tests/e2e_loops_m1.rs` first (asserts file existence + section headers + cross-reference targets).
2. Confirm the test fails for the expected reason (file missing).
3. Author `docs/LOOPS-ENGINEERING.md` with a top-level "Start here" orienter and the four documented loops:
   - **Sprint loop**: think → plan → build → review → test → ship → reflect (skills involved: every `/slo-*`).
   - **Security-tuning loop**: threat model → SAST/DAST tuning → findings → threat-model refinement (skills: `/slo-architect`, `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-verify`, `/slo-critique`).
   - **Lessons loop**: retro → issues → priority order → next milestone (skills: `/slo-retro`, `/slo-execute`).
   - **Library-feedback loop**: `/slo-sec-libs` capability gap → upstream filing → library improvement → re-scan (skills: `/slo-sec-libs` once R4 ships).
4. Cross-link from `ARCHITECTURE.md`.
5. Add per-skill cross-references (one-line each).
6. Make the structural-contract test pass.
7. Run full suite + smoke tests.
8. Self-review gate.

#### BDD Acceptance Scenarios

**Feature: engineering loops documented and discoverable**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Engineering loops doc exists | happy path | repo at HEAD | open `docs/LOOPS-ENGINEERING.md` | document opens with at least 4 loop sections (sprint, security-tuning, lessons, library-feedback) |
| Newcomer can identify the right engineering loop quickly | discoverability / orientation | user asks "I have a repeated regression, where do I start?" | read top of `docs/LOOPS-ENGINEERING.md` | "Start here" section maps the question to the lessons loop and first skill |
| ARCHITECTURE.md cross-links to engineering loops | happy path | ARCHITECTURE.md exists | grep ARCHITECTURE.md for `LOOPS-ENGINEERING.md` | match found in a "Feedback loops" section |
| Each engineering SKILL.md cited in a loop has a cross-reference | happy path | SKILL.md cited under "Sprint loop" section | grep SKILL.md for `LOOPS-ENGINEERING.md` | match found |
| Loop cited in doc has missing cross-reference | invalid input / structural | LOOPS-ENGINEERING.md cites SKILL.md X under loop Y | grep SKILL.md X for `LOOPS-ENGINEERING.md#<loop-y>` | structural-contract test FAILS until cross-reference is added |
| Empty loops file | empty state | LOOPS-ENGINEERING.md is empty | structural-contract test runs | test FAILS with "no loop sections found" |
| Library-feedback loop placeholder while R4 unshipped | partial | `/slo-sec-libs` does not yet exist | inspect Library-feedback loop section | section is present with "ships in Runbook 4" footnote, NOT removed silently |
| Prompt-injection abuse case | abuse case (`tm-loops-abuse-1`) | a lesson body contains `<script>alert(1)</script>` | LOOPS-ENGINEERING.md template uses `~~~text` fence for any quoted lesson body | Markdown renders the script as literal text, not executable |

#### Regression Tests

- `cargo test --workspace` — full suite green.
- Existing `crates/sldo-install/tests/` SKILL.md installation tests still pass after cross-references added.
- Each modified SKILL.md still installs symlinked correctly.

#### Compatibility Checklist

- [ ] Every existing SKILL.md still installs.
- [ ] ARCHITECTURE.md still renders cleanly (no broken links).
- [ ] No existing test fails.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_loops_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `loops_engineering_doc_exists_and_has_required_sections` | Document is present with required loop sections | All four loop sections present; each has name + trigger + steps + exit-condition + skills + diagram |
| `loops_engineering_doc_has_start_here_orienter` | Outcome-first orientation exists | grep finds "Start here" plus at least one `question -> loop -> first skill` mapping |
| `architecture_md_cross_links_loops_engineering` | Bidirectional discoverability | ARCHITECTURE.md grep for `LOOPS-ENGINEERING.md` returns ≥ 1 match in a "Feedback loops" or equivalent section |
| `every_cited_skill_has_cross_reference` | Cross-reference invariant | For every SKILL.md cited under a loop, that SKILL.md contains the corresponding `LOOPS-ENGINEERING.md#<loop>` link |

#### Smoke Tests

- [ ] Open `docs/LOOPS-ENGINEERING.md` — diagrams render in GitHub web view.
- [ ] Click ARCHITECTURE.md → LOOPS-ENGINEERING.md — link works.
- [ ] Open one cross-referenced SKILL.md — back-link to LOOPS-ENGINEERING.md works.
- [ ] `cargo test -p sldo-install` passes.
- [ ] `git status` shows no untracked test artifacts.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline | `cargo test --workspace` | green | | | |
| Test stub created | `tests/e2e_loops_m1.rs` | fails with "file missing" | | | |
| LOOPS-ENGINEERING.md authored | inspect file | 4 loop sections + diagrams | | | |
| ARCHITECTURE cross-link | grep for `LOOPS-ENGINEERING.md` | ≥ 1 match | | | |
| Per-skill cross-references | grep all engineering SKILL.md | every cited skill linked | | | |
| Full tests | `cargo test --workspace` | green | | | |
| `.gitignore` review | | no new patterns needed | | | |

#### Definition of Done

- All listed BDD scenarios pass.
- Structural-contract test passes.
- ARCHITECTURE.md links to LOOPS-ENGINEERING.md.
- Every engineering SKILL.md cited under a loop has the cross-reference.
- `cargo test --workspace` is green.
- Lessons file written.
- Completion summary written.
- Tracker updated.

#### Post-Flight

- ARCHITECTURE.md update: confirm "Feedback loops" section landed and renders.
- README.md update: add a "Loops" bullet in the docs index if the README has one.

#### Notes

- The Library-feedback loop section is a placeholder until R4 ships — explicit footnote; do not omit silently.
- The runbook template change (M4) does not block M1.

---

### Milestone 2 — `docs/LOOPS-BUSINESS.md` authored + cross-linked

**Goal**: A `docs/LOOPS-BUSINESS.md` document covering at minimum the four business loops (user-interview, GTM, pricing, founder-check), cross-linked from each implicated biz SKILL.md, with a newcomer-friendly "Start here" orienter at the top.

**Context**: Issue [#18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) is the canonical decision record. Same shape as M1; biz domain. Possible additional loops to inventory before authoring: fundraise loop, cofounder loop, hiring loop, legal-triage loop (Issue #18 Q1 leaves these as open additions).

**Important design rule**: Match M1's diagram style for consistency. The doc opens with a short "Start here" orienter (`question -> loop -> first skill -> expected artifact`). Per loop: user-visible outcome, trigger, steps, exit condition, artifacts, skills involved, diagram.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Issue #18 decisions; existing biz SKILL.md files; M1's LOOPS-ENGINEERING.md as style anchor |
| Outputs | `docs/LOOPS-BUSINESS.md` (new); per-biz-skill cross-references |
| Interfaces touched | per-biz-skill SKILL.md cross-references; possibly ARCHITECTURE.md if a biz-side architectural sketch is added |
| Files allowed to change | `docs/LOOPS-BUSINESS.md` (new), each biz SKILL.md cited as part of a loop, `crates/sldo-install/tests/e2e_loops_m2.rs` (new) |
| Files to read before changing anything | All biz SKILL.md files; M1's LOOPS-ENGINEERING.md; Issue #18 thread |
| New files allowed | `docs/LOOPS-BUSINESS.md`, `crates/sldo-install/tests/e2e_loops_m2.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | M1 work unchanged; biz SKILL.md prose preserved |
| Forbidden shortcuts | "see M1" placeholder for biz loop steps; every biz loop must be authored explicitly; dumping loop internals before the orienter |
| **Data classification** | `Public` — biz loops doc is structural; no real founder PII |
| **Proactive controls in play** | OWASP C1 (security requirements anchored in user-interview loop's PII handling); C7 (Enforce Access Controls — biz docs `tier:` discipline anchored in user-interview loop's PII flow) |
| **Abuse acceptance scenarios** | `tm-loops-abuse-1: prompt injection via lesson body content` (same as M1); `tm-loops-abuse-2: biz-loop documentation leaks PII via interview-quote example` — eliminated by structural rule "no real interview quotes in LOOPS-BUSINESS.md; all examples use Alice / Bob pseudonyms" |

#### Out of Scope / Must Not Do

- Adding engineering-loops content (M1's job).
- Modifying any biz SKILL.md beyond the cross-reference link.
- Authoring new biz loops not anchored in existing skill invocations (the loop has to map to a real skill flow).

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read [Issue #18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) end-to-end including thread comments for any inventory additions.
3. Survey every biz SKILL.md to identify loop participation.
4. Decide: are there 4 loops (Issue #18 inventory) or 5+ (added during runbook execution)? Confirm with project owner if more than 4.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `docs/LOOPS-BUSINESS.md` | NEW: per-loop sections matching M1 style |
| `skills/<each-biz-skill>/SKILL.md` | Add one-line cross-reference |
| `crates/sldo-install/tests/e2e_loops_m2.rs` | NEW: same shape as M1's test |

#### Step-by-Step

1. Test stub first.
2. Confirm fails for expected reason.
3. Author `docs/LOOPS-BUSINESS.md` with a top-level "Start here" orienter and the documented loops (user-interview, GTM, pricing, founder-check; plus any inventory additions confirmed in pre-flight).
4. Add per-skill cross-references.
5. Make tests pass.
6. Smoke tests.
7. Self-review gate.

#### BDD Acceptance Scenarios

**Feature: business loops documented and discoverable**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Business loops doc exists | happy path | repo at HEAD | open `docs/LOOPS-BUSINESS.md` | doc opens with ≥ 4 loop sections |
| Newcomer can identify the right business loop quickly | discoverability / orientation | founder asks "we're not learning from user calls, where do I start?" | read top of `docs/LOOPS-BUSINESS.md` | "Start here" section maps the question to the user-interview loop and first skill |
| Per-biz-skill cross-reference present | happy path | SKILL.md cited under a loop | grep for `LOOPS-BUSINESS.md` | match found |
| Real-PII example smuggled into doc | abuse case (`tm-loops-abuse-2`) | dev tries to commit LOOPS-BUSINESS.md with a real-name example | `/slo-verify` Pass 4 PII scan over docs/biz-public/ | scan flags; commit blocked until anonymized |
| Backward compat with M1's LOOPS-ENGINEERING.md | backward compatibility | M1's doc + ARCHITECTURE cross-link | both docs present | both render; both cross-linked |
| Empty business loops file | empty state | doc empty | structural-contract test runs | test fails |

#### Regression Tests

- `cargo test --workspace`.
- M1's structural-contract test still passes.
- Each modified biz SKILL.md still installs.

#### Compatibility Checklist

- [ ] M1's LOOPS-ENGINEERING.md unchanged.
- [ ] All biz SKILL.md files still install.
- [ ] No biz-pack structural-contract test regressed.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_loops_m2.rs` — same shape as M1's, asserting business doc + cross-references.

#### Smoke Tests

- [ ] Diagrams render in GitHub web view.
- [ ] Cross-links work both directions.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

(Same shape as M1; copy at execution time.)

#### Definition of Done

- All listed BDD scenarios pass.
- Cross-references in every biz SKILL.md cited under a loop.
- `cargo test --workspace` green.
- Lessons + completion files written.

#### Post-Flight

- ARCHITECTURE.md "Feedback loops" section updated to link both docs.

#### Notes

- If Issue #18 inventory adds loops (e.g., fundraise loop), confirm with project owner before authoring (skill prose may not yet support a true fundraise loop).

---

### Milestone 3 — `/slo-retro` extension: classify, dedupe, file lessons as issues

**Goal**: `/slo-retro` extends to classify each lesson (product / upstream-OSS / slo-process), dedupe via `gh search`, and file as an issue with user confirmation. Fallback to `LESSONS-BACKLOG.md` if `gh` is unavailable. The lessons file is still written first; issue filing is additive after.

**Context**: [Issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) is the canonical decision record. Decisions: extension of `/slo-retro` (not a separate skill), dedupe via `gh search` first, never auto-file (always confirm), reuse rate-limit discipline from `/slo-sec-libs`, fallback to local file when no tracker configured.

**Important design rule**: Issue filing is ADDITIVE — the existing lessons file write happens unchanged first; classification + filing run after. If filing fails, the lessons file is still safely on disk.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `/slo-retro`-completed lessons file at `docs/lessons/<prefix>-m<N>.md`; runbook prefix; `gh` CLI availability; user confirmation for each filing |
| Outputs | Issue(s) filed in product / upstream / slo-process repos OR rows appended to `LESSONS-BACKLOG.md`; updated lessons file with `filed_issues:` frontmatter listing the filings |
| Interfaces touched | `gh issue create` (argv-list, no `--repo`); `gh search issues`; `LESSONS-BACKLOG.md` row format |
| Files allowed to change | `skills/slo-retro/SKILL.md`, `skills/slo-retro/references/issue-filing-discipline.md` (new), `crates/sldo-install/tests/e2e_loops_m3.rs` (new) |
| Files to read before changing anything | `skills/slo-retro/SKILL.md` current behavior; `skills/slo-sast/SKILL.md` M5 argv-list discipline; `skills/slo-sec-libs/references/upstream-filing-discipline.md` (R4 — if shipped before R1; otherwise inline-author the rate-limit pattern in this milestone) |
| New files allowed | `skills/slo-retro/references/issue-filing-discipline.md`, `crates/sldo-install/tests/e2e_loops_m3.rs`, `LESSONS-BACKLOG.md` (in target repos that opt in) |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing lessons-file write unchanged; existing `/slo-retro` Outputs still produced; issue filing is additive |
| Forbidden shortcuts | auto-filing without user confirmation; shell-string interpolation of lesson body into `gh issue create`; bypassing dedupe; using `--repo` flag |
| **Data classification** | `Internal` — lesson content may include reflection on internal codepaths but no real PII; the issue filing makes this content public when the destination repo is public |
| **Proactive controls in play** | OWASP C5 (Validate All Inputs — lesson body content is fence-wrapped for argv-list); C9 (Logging and Monitoring — every filing carries `gh` author + timestamp); C7 (Access Controls — filing requires user `gh` auth; never silent) |
| **Abuse acceptance scenarios** | `tm-loops-abuse-3: lesson body splices attacker prose into issue title/body via shell interpolation` — class eliminated by argv-list discipline (inherited from `/slo-sast` M5); `tm-loops-abuse-4: confused-deputy via tampered .git/config` — eliminated by NO `--repo` flag; `tm-loops-abuse-5: filing storm via runaway loop` — mitigated by per-session 40-issues/hr cap (reuses `/slo-sec-libs` pattern) |

#### Out of Scope / Must Not Do

- Replacing the lessons-file write (additive only).
- Cross-repo issue federation beyond the three classifications.
- Auto-merging filed issues.
- Modifying [`crates/sldo-common::toolflags`](../crates/sldo-common/) skill-flag denials beyond what `/slo-retro` already declares.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read [`skills/slo-retro/SKILL.md`](../skills/slo-retro/SKILL.md) end-to-end.
3. Read `skills/slo-sast/SKILL.md` M5 (the argv-list + no-`--repo` discipline anchor).
4. Read [Issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) thread for marker decision.
5. **Spike step**: test `gh search` reliability against three marker options (title prefix `[retro]`, label `retro-derived`, body sentinel `<!-- retro-derived -->`) on a small populated set. Document the choice in the lessons file before locking it.
6. Decide explicit upstream-mapping format: `.sldo/upstream-mapping.toml` with crates.io / npm fallback resolution.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-retro/SKILL.md` | Extend Outputs section with classification + dedupe + filing flow; preserve existing lessons-file output |
| `skills/slo-retro/references/issue-filing-discipline.md` | NEW: argv-list rules, comprehensive dedupe procedure (NFKC normalization, RTL-override rejection, zero-width / homoglyph rejection, lowercase-fold + whitespace-collapse before search, three-strike search with progressively-broadened queries to catch near-duplicates), rate-limit cap (40 issues/hr per session) PLUS adaptive backoff on observed `gh` rate-limit response codes, confirmation gate with full record-preview, `LESSONS-BACKLOG.md` fallback **with comprehensive audit row schema: `\| YYYY-MM-DD HH:MM:SSZ \| classification (product/upstream-OSS/slo-process) \| skill_or_runbook_prefix \| agent_version (claude-model-id) \| originating_milestone \| dedupe_search_result (none/match-id/ambiguous) \| filed_to (repo-or-local) \| issue_url_or_local_ref \| disposition (filed/skipped-dupe/skipped-user/spilled-cap) \| body_sha256 (first 12 chars, for re-dedup across sessions) \| retry_count \| status (open/closed/transferred) \|`**, three-classification routing |
| `crates/sldo-install/tests/e2e_loops_m3.rs` | NEW: structural-contract test asserting SKILL.md changes + reference file existence + argv-list discipline mention |

#### Step-by-Step

1. Run the marker-choice spike on a populated test repo.
2. Author `references/issue-filing-discipline.md` with the marker choice locked.
3. Test stub first (`tests/e2e_loops_m3.rs`).
4. Update SKILL.md with the extension flow, citing the new reference file.
5. Validate via the structural-contract test.
6. Manual smoke: invoke `/slo-retro` against a sample lessons file; observe classify + dedupe + confirmation prompt.
7. Self-review gate.

#### BDD Acceptance Scenarios

**Feature: retro extension files lessons as issues with discipline**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Happy path: 3 lessons classified + filed | happy path | lessons file with 3 lessons (1 product, 1 upstream, 1 slo-process) | `/slo-retro` runs after milestone close | 3 issues filed with user confirmation; `filed_issues:` frontmatter in lessons file |
| Dedupe finds existing issue | happy path | similar lesson already filed | `/slo-retro` classifies + searches | dedupe surfaces the existing issue; no new filing |
| `gh` not on PATH | dependency failure | `which gh` returns nothing | `/slo-retro` runs | falls back to `LESSONS-BACKLOG.md`; lessons file written; user notified |
| `gh auth login` not configured | dependency failure | `gh auth status` returns unauth | `/slo-retro` runs | falls back to `LESSONS-BACKLOG.md`; user notified with `gh auth login` install hint |
| User declines a specific filing | invalid input / consent | user types `n` at confirmation | `/slo-retro` skips that one | other filings proceed; declined filing recorded in lessons frontmatter as `filed: false` |
| Empty lessons file (no lessons to file) | empty state | lessons file has no flagged lessons | `/slo-retro` runs | "no lessons to file" notice; no `gh` calls; exit clean |
| Lesson body contains shell-injection payload | abuse case (`tm-loops-abuse-3`) | lesson body has `; rm -rf /` | `/slo-retro` constructs `gh issue create` argv | argv-list passes the string as a single argument; `rm -rf` not executed |
| Title with homoglyph evades dedupe | abuse case (`tm-loops-abuse-3a`, paradigm: comprehensive abuse enumeration) | candidate title `[retroʿ] X` (Hebrew letter substituted for `o`) | dedupe runs three-strike search (literal, NFKC-normalized, ASCII-only-collapsed) | dedupe matches if any of the three strikes hits; refuses to file silently if homoglyph normalization would change the marker (escalates to user) |
| Title with zero-width characters | abuse case (`tm-loops-abuse-3b`) | candidate title `[retro​] X` | dedupe pre-normalizes by stripping zero-width codepoints (U+200B / U+200C / U+200D / U+FEFF) | dedupe match found in normalized form |
| Title with RTL override | abuse case (`tm-loops-abuse-3c`) | candidate title contains U+202E or U+202D | dedupe rejects RTL/LTR override codepoints | refuses to file with clear stderr; never falls through to "looks fine" |
| Body with code-injection in fence | abuse case (`tm-loops-abuse-3d`) | lesson body has ```` ```bash; rm -rf /``` ```` | issue body wrapped in `~~~text` fence per `/slo-architect` user-string-fence rule (load-bearing across the pack) | fence preserved verbatim; agent reading the issue back via M4 carry-forward treats the content as literal text |
| Body > 65,536 chars (GitHub issue body cap) | dependency failure (paradigm: handle every degraded state, not just happy) | lesson body exceeds GitHub's 65,536-char issue body limit | filer truncates body with `... [truncated; full body in lessons file at <path>]` footer | filer never silently fails on the API error; preserves discoverability via the lessons file path |
| Issue body contains a markdown reference cycle | abuse case (`tm-loops-abuse-3e`) | lesson body cites `docs/lessons/<this-milestone>.md` (self-reference) | filer detects self-reference; rewrites as `<this milestone, see runbook tracker>` | no infinite recursion when M4 carry-forward reads it back |
| Filing during a `gh` rate-limit response | dependency failure | `gh issue create` returns secondary rate-limit error | filer reads `Retry-After` header, applies adaptive backoff, surfaces to user | does not retry blind; either waits + retries with user confirm, or spills to LESSONS-BACKLOG.md |
| Two retros in same session try to file same lesson | concurrency (paradigm: comprehensive coverage even when unlikely) | M3-of-runbook-A and M3-of-runbook-B both produce a similar lesson | dedupe via body_sha256 (in audit row) catches the cross-runbook duplicate | second filing surfaces the prior; user decides merge / skip / file-new |
| Tampered `.git/config` redirects origin | abuse case (`tm-loops-abuse-4`) | adversary modifies `.git/config` remote.origin.url | `/slo-retro` files | NO `--repo` flag = uses local origin; resolved to attacker repo only if attacker also controls local config (same trust class) — surfaced in confirmation gate ("filing to <origin>: confirm?") |
| Filing storm | abuse case (`tm-loops-abuse-5`) | session has filed 40 issues already | 41st filing attempted | rate-limit cap fires; remaining lessons routed to `LESSONS-BACKLOG.md` |
| Backward compat: lessons file with no `filed_issues:` frontmatter | backward compatibility | existing lessons file pre-M3 | `/slo-retro` runs again | no-op on already-closed milestone; new milestone runs clean |

#### Regression Tests

- `cargo test --workspace`.
- Existing `/slo-retro` SKILL.md install symlink test.
- M1 + M2 structural-contract tests still pass.

#### Compatibility Checklist

- [ ] Existing lessons-file write happens before any `gh` call.
- [ ] If `gh` errors, lessons file is still on disk.
- [ ] Existing `/slo-retro` install symlink unchanged.
- [ ] `/slo-execute`'s existing pre-flight Step 1 (read previous milestone's lessons) unaffected.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_loops_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_retro_skill_md_extended` | SKILL.md has the new section | grep matches "## Issue filing" or equivalent header |
| `issue_filing_discipline_reference_exists` | Reference file exists | `references/issue-filing-discipline.md` present + frontmatter valid |
| `argv_list_discipline_documented` | argv-list rule present in reference file | grep for "argv-list" in `references/issue-filing-discipline.md` |
| `no_repo_flag_documented` | NO `--repo` rule present | grep for "NO `--repo`" |
| `rate_limit_cap_documented` | 40-issues/hr cap present | grep for "40 issues" or "40 per session" |
| `lessons_backlog_fallback_documented` | Fallback present | grep for `LESSONS-BACKLOG.md` |

#### Smoke Tests

- [ ] Mock-invoke `/slo-retro` against a fixture lessons file; observe confirmation prompt.
- [ ] Decline at the confirmation prompt; verify no filing happens.
- [ ] Set `gh auth status` to unauth via env override; observe fallback.
- [ ] Test argv-list with a payload containing `;` `|` `&` — confirm no shell interpretation.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Argv-list discipline cited and tested.
- Marker choice locked in `references/issue-filing-discipline.md` with the spike data.
- Fallback works (verified manually).
- Tracker + lessons + completion files written.

#### Post-Flight

- `ARCHITECTURE.md` update: add a paragraph in the "Feedback loops" section pointing at the new filing flow.
- README.md update: not required.

#### Notes

- The marker-choice spike result is itself a lesson worth filing; M3 closing produces M3-of-this-runbook lessons that get filed via... M3's own mechanic. Self-bootstrapping; document the dogfood-smoke-test in the completion summary.

---

### Milestone 4 — `/slo-execute` pre-flight loop closure + runbook template "Carry-forward from prior retros" section

**Goal**: `/slo-execute M<N>` pre-flight queries open issues filed by `/slo-retro` for prior milestones in this runbook's prefix; surfaces them as scope candidates with a suggested lane (`micro | milestone | fresh-runbook`); user decides each milestone's bounds. Runbook template gains an optional "Carry-forward from prior retros" section so the loop is visible in the artifact, not just in the skill flow.

**Context**: [Issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) decision: "close the loop in execution flow ... runbook template should reflect this — don't bolt it onto the skill alone." [Issue #16 Q3](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) decision (folded into this runbook): the template change lives here.

**Important design rule**: Surfacing is informational, not auto-additive. The user decides each milestone's bounds; the skill never auto-extends the allow-list. The surface should reduce, not add, ceremony: at most the top 3 carry-forward items are shown inline, each with a suggested lane and a one-line why.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Runbook at `docs/RUNBOOK-<feature>.md`; current milestone N; prefix from runbook metadata; `gh` CLI |
| Outputs | Pre-flight stdout listing open prior-retro issues for this prefix plus suggested lane; updated runbook template with new section schema |
| Interfaces touched | `/slo-execute` SKILL.md pre-flight section (additive); `docs/runbook-template_v_3_template.md` (additive section with lane column) |
| Files allowed to change | `skills/slo-execute/SKILL.md`, `docs/runbook-template_v_3_template.md`, `crates/sldo-install/tests/e2e_loops_m4.rs` (new) |
| Files to read before changing anything | `skills/slo-execute/SKILL.md` current pre-flight; M3's `references/issue-filing-discipline.md` (for marker choice); `docs/runbook-template_v_3_template.md` |
| New files allowed | `crates/sldo-install/tests/e2e_loops_m4.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Existing pre-flight Step 1 (read previous milestone's lessons) unchanged; new step ADDED at Step 1.5 or as Step 2; existing runbooks without "Carry-forward" section still work |
| Forbidden shortcuts | auto-extending allow-list based on carry-forward; silently filtering issues; `--repo` flag |
| **Data classification** | `Internal` — issue titles / bodies surface as stdout; no PII expected (lesson-derived) |
| **Proactive controls in play** | C5 (Validate inputs); C9 (Logging — every pre-flight read recorded in evidence log) |
| **Abuse acceptance scenarios** | `tm-loops-abuse-6: malicious carry-forward issue body smuggles prompt-injection content into agent context` — eliminated by `~~~text` fence around any quoted issue body (matches `/slo-architect` user-string-fence rule) |

#### Out of Scope / Must Not Do

- Auto-extending the allow-list.
- Cross-runbook carry-forward (each runbook reads only its own prefix).
- Modifying the lesson-→issue filing flow (M3's domain).
- Removing existing pre-flight steps.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read M3's `skills/slo-retro/references/issue-filing-discipline.md` for the marker that the carry-forward query uses.
3. Read [`skills/slo-execute/SKILL.md`](../skills/slo-execute/SKILL.md) end-to-end.
4. Read [`docs/runbook-template_v_3_template.md`](runbook-template_v_3_template.md) end-to-end.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-execute/SKILL.md` | Add pre-flight Step 1.5: "Read open prior-retro issues filtered by this runbook's prefix; surface as scope candidates". Cite M3's marker choice. |
| `docs/runbook-template_v_3_template.md` | Add new optional section "## Carry-forward from prior retros" between "Background Context" and "BDD and Runtime Validation Rules". Schema: per-row issue # + title + suggested lane (`micro | milestone | fresh-runbook`) + suggested milestone column. |
| `crates/sldo-install/tests/e2e_loops_m4.rs` | NEW: structural-contract test asserting SKILL.md change + template change |

#### Step-by-Step

1. Test stub.
2. Update SKILL.md pre-flight prose with the additive Step 1.5 and compact top-3 carry-forward summary.
3. Update runbook template with the new section.
4. Update one existing runbook (this runbook itself, when M4 closes — dogfood) to include a "Carry-forward from prior retros" section listing M1-M3's filed issues if any.
5. Verify structural-contract test.
6. Smoke tests.
7. Self-review.

#### BDD Acceptance Scenarios

**Feature: milestone-start loop closure**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Pre-flight surfaces open prior-retro issues | happy path | runbook prefix `loops`; M1-M3 closed; issues filed | `/slo-execute M4` pre-flight runs | stdout lists open `loops`-prefix retro-derived issues with title + # + suggested lane |
| Small follow-up stays small | discoverability / orientation | open prior-retro issue is low-risk doc polish | `/slo-execute` pre-flight runs | item is surfaced as `micro`, not silently expanded into milestone-sized work |
| Large follow-up does not silently widen scope | discoverability / orientation | open prior-retro issue requires new architecture work | `/slo-execute` pre-flight runs | item is surfaced as `fresh-runbook`; current milestone allow-list stays unchanged |
| No prior-retro issues for this prefix | empty state | first milestone of a runbook | `/slo-execute M1` pre-flight runs | stdout: "no carry-forward from prior retros (this is M1)" |
| `gh` unavailable | dependency failure | `gh` missing | pre-flight runs | warns + proceeds (does not block on gh-availability for this informational read) |
| `gh` returns rate-limit | dependency failure (paradigm: comprehensive degraded states) | `gh issue list` returns 403 secondary rate-limit | pre-flight | applies 5s timeout; falls back to "carry-forward unavailable; gh rate-limited at <retry_after>"; never hangs |
| Tampered issue body smuggles prompt | abuse case (`tm-loops-abuse-6`) | issue body contains `<role>system</role>` payload | pre-flight surfaces issue | issue body wrapped in `~~~text` fence in stdout; agent treats as literal text |
| Issue was transferred to another repo | dependency failure (paradigm: comprehensive degraded states) | retro-derived issue moved from current repo to `kerberosmansour/SunLitOrchestrate` | `gh issue list` returns it under different repo | carry-forward surfaces with `[transferred from <origin>]` annotation; does NOT auto-follow the cross-repo reference |
| Issue closed but linked PR is open | dependency failure | issue closed by referencing PR `Fixes #N` but PR is unmerged | carry-forward query | surfaces as "closed-via-PR-pending"; user decides whether to track |
| Issue is a `gh` cache stale-read | dependency failure | local `gh` cache returns issue last-known-state from 2 weeks ago | carry-forward query | carry-forward annotates "freshness: cached <timestamp>"; user can `gh repo refresh` and re-run |
| Multi-runbook prefix collision | edge case | two runbooks share a prefix accidentally (e.g., both `loops`) | carry-forward query | filter is by exact prefix; if collision, surface BOTH as "ambiguous prefix; consider runbook ID renaming" |
| Backward compat: runbook without Carry-forward section | backward compatibility | existing runbook pre-M4 | `/slo-execute` runs | works unchanged; no error about missing section |
| Forward compat: runbook with Carry-forward section but no filed issues yet | backward compatibility | new runbook with section template | `/slo-execute M1` runs | section is empty; no error |
| Auto-extend allow-list attempt | abuse case (`tm-loops-abuse-7: agent extends allow-list silently based on carry-forward`) | carry-forward suggests editing file outside allow-list | `/slo-execute` runs | flagged; user-confirmation gate fires before any extension; matches existing allow-list rule |

#### Regression Tests

- `cargo test --workspace`.
- Existing `/slo-execute` install + behavior tests.
- M1, M2, M3 structural-contract tests.

#### Compatibility Checklist

- [ ] `/slo-execute` Step 1 (previous milestone's lessons) still runs first.
- [ ] Existing runbooks without "Carry-forward" section still execute milestones cleanly.
- [ ] No existing test fails.
- [ ] Auto-extension of allow-list is blocked by existing allow-list rule.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_loops_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_execute_pre_flight_extended` | SKILL.md change applied | grep for "prior-retro" in pre-flight section |
| `runbook_template_carry_forward_section` | Template change applied | grep `runbook-template_v_3_template.md` for "Carry-forward from prior retros" |
| `runbook_template_carry_forward_lane_column` | Lane schema applied | grep template for `micro | milestone | fresh-runbook` |
| `gh_issue_list_argv_list_documented` | argv-list discipline | grep for `gh issue list` argv pattern in SKILL.md |
| `no_auto_extend_allowlist_documented` | Discipline preserved | grep SKILL.md for "user decides each milestone's bounds" |

#### Smoke Tests

- [ ] Mock-invoke `/slo-execute` on a runbook with M1-M3 closed; observe carry-forward surface.
- [ ] Mock-invoke on a runbook with no prior milestones; observe empty-state message.
- [ ] Verify `gh` unavailable does not block.
- [ ] Manual: confirm template's new section renders.

#### Evidence Log

(Copy at execution time.)

#### Definition of Done

- All BDD scenarios pass.
- Pre-flight Step 1.5 added without removing Step 1.
- Runbook template gains the new optional section.
- Carry-forward section and pre-flight prose both use the `micro | milestone | fresh-runbook` lane vocabulary.
- Dogfood: this runbook itself includes a "Carry-forward from prior retros" section after M4 closes (even if empty for M1).
- Tracker + lessons + completion files written.
- ARCHITECTURE.md "Feedback loops" section updated to mention milestone-start carry-forward.

#### Post-Flight

- ARCHITECTURE.md update: "Feedback loops" section gains the milestone-start-carry-forward bullet.
- README.md update: not required.

#### Notes

- This runbook now self-tests the loop end-to-end: M4 introduces carry-forward, and M5 dogfoods the read-only orientation path that consumes it. After M5 closes there is no M6 here, but future runbooks inherit the mechanic.

---

### Milestone 5 — `/slo-resume` next-step digest + lane-aware orientation

**Goal**: Extend `/slo-resume` so it becomes the pack's canonical "what next?" entrypoint for interrupted work: it reads the runbook tracker plus the optional carry-forward section, emits exactly one next action, and classifies the safest path as `micro`, `milestone`, or `fresh-runbook` without starting anything automatically.

**Context**: The current `/slo-resume` behavior is helpful but shallow: it can name the first non-`done` milestone, but not the highest-value carry-forward item or whether the work should stay tiny, consume the current milestone, or split into a new runbook. This is the most directly applicable adoption lesson from the BMAD comparison: the pack needs one obvious orientation surface, but it should strengthen the existing `/slo-resume` verb rather than minting a parallel `/slo-help`.

**Important design rule**: `/slo-resume` stays read-only and brief. It must answer three questions in one screen: where am I, what should I do next, and what can wait. No auto-starting skills. No dumping the full carry-forward table inline. Surface at most the top 3 carry-forward items plus a count of remaining items.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Current runbook's Milestone Tracker; optional "Carry-forward from prior retros" section; latest lessons/completion summary if needed for one-line context |
| Outputs | A short orientation message: current milestone, status, recommended lane, exact next action, and top carry-forward item(s) if relevant |
| Interfaces touched | `skills/slo-resume/SKILL.md` output contract (additive only) |
| Files allowed to change | `skills/slo-resume/SKILL.md`, `crates/sldo-install/tests/e2e_loops_m5.rs` (new), `docs/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md` (tracker + carry-forward dogfood row updates only) |
| Files to read before changing anything | `skills/slo-resume/SKILL.md` current behavior; M4-updated `docs/runbook-template_v_3_template.md`; this runbook's new carry-forward section shape |
| New files allowed | `crates/sldo-install/tests/e2e_loops_m5.rs` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `/slo-resume` stays read-only; existing "multiple runbooks -> ask user which" gate preserved; existing tracker-first behavior preserved and extended, not replaced |
| Forbidden shortcuts | auto-starting the recommended skill; emitting a wall of prose; ignoring carry-forward lane; inventing a new public verb |
| **Data classification** | `Internal` — runbook tracker plus carry-forward issue summaries |
| **Proactive controls in play** | C5 (Validate inputs — runbook / carry-forward parsing is exact); C9 (Auditability — orientation output references artifact state rather than agent memory) |
| **Abuse acceptance scenarios** | `tm-loops-abuse-8: carry-forward overload turns orientation into noise` — mitigated by top-3 inline cap + remainder count; `tm-loops-abuse-9: malicious carry-forward issue body smuggles prompt-injection text into /slo-resume output` — mitigated by not inlining full bodies and fence-wrapping any quoted snippets |

#### Out of Scope / Must Not Do

- Creating a new `/slo-help` skill.
- Auto-starting `/slo-execute`, `/slo-verify`, or `/slo-ship`.
- Rewriting milestone tracker status based on inference.
- Mutating the runbook from `/slo-resume`.

#### Pre-Flight

1. Complete Global Entry Rules.
2. Read [`skills/slo-resume/SKILL.md`](../skills/slo-resume/SKILL.md) end-to-end.
3. Read the M4-updated carry-forward section schema in [`docs/runbook-template_v_3_template.md`](runbook-template_v_3_template.md).
4. Inspect at least one completed runbook that already contains a carry-forward section to confirm the parser shape remains realistic.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-resume/SKILL.md` | Extend the Method and Output sections so the skill reads carry-forward and emits one next action plus lane |
| `crates/sldo-install/tests/e2e_loops_m5.rs` | NEW: structural-contract test asserting carry-forward-aware orientation rules |

#### Step-by-Step

1. Write the structural-contract test stub first.
2. Confirm it fails for the expected reason.
3. Update `/slo-resume` so it reads the tracker first, then the carry-forward section if present.
4. Add the lane rules:
   - `micro` = safe, bounded follow-up; keep within current or immediate next milestone.
   - `milestone` = real milestone work inside the current runbook.
   - `fresh-runbook` = material scope/risk shift; do not widen the current runbook silently.
5. Keep the output compact: milestone, status, lane, next action, one-line context, top carry-forward item(s).
6. Make the structural-contract test pass.
7. Smoke test with one empty-state runbook and one runbook with carry-forward rows.
8. Self-review gate.

#### BDD Acceptance Scenarios

**Feature: one-screen orientation after interruptions**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Next action from tracker only | happy path | one runbook; M1 done, M2 not_started; no carry-forward rows | `/slo-resume` runs | output names M2 and suggests the correct next skill |
| Carry-forward item influences orientation | happy path | current milestone is in progress; carry-forward section has one `micro` item | `/slo-resume` runs | output keeps the current milestone, surfaces the `micro` item as context, and does not invent a new runbook |
| Fresh-runbook follow-up is not silently widened into current milestone | discoverability / orientation | carry-forward row is marked `fresh-runbook` | `/slo-resume` runs | output says the current runbook should not widen scope and names the follow-up as separate work |
| Empty carry-forward section | empty state | runbook has the section but no rows | `/slo-resume` runs | output still orients cleanly; no error about missing follow-ups |
| Multiple runbooks present | dependency / ambiguity | more than one `docs/RUNBOOK-*.md` exists | `/slo-resume` runs | asks the user which runbook before orienting |
| Blocked milestone | dependency failure | first non-`done` row is `blocked` | `/slo-resume` runs | output prints blocker plus the safest next human decision, not `/slo-execute` |
| Carry-forward overload | abuse case (`tm-loops-abuse-8`) | carry-forward table has 12 open items | `/slo-resume` runs | inline output shows top 3 plus "9 more" instead of dumping the whole table |
| Malicious carry-forward title/body snippet | abuse case (`tm-loops-abuse-9`) | carry-forward row references issue with prompt-injection text | `/slo-resume` runs | only short title/snippet appears, fence-wrapped if quoted; no raw full body is emitted |
| All milestones done | backward compatibility | every tracker row is `done` | `/slo-resume` runs | suggests `/slo-ship` or confirms the runbook is complete |

#### Regression Tests

- `cargo test --workspace`.
- Existing `/slo-resume` install tests still pass.
- M1-M4 structural-contract tests still pass.

#### Compatibility Checklist

- [ ] `/slo-resume` remains read-only.
- [ ] Existing tracker-only orientation still works when no carry-forward section exists.
- [ ] Multiple-runbook ambiguity still asks the user rather than guessing.
- [ ] No existing test fails.

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_loops_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `slo_resume_reads_carry_forward_section` | Carry-forward is part of orientation contract | grep `skills/slo-resume/SKILL.md` for "Carry-forward from prior retros" |
| `slo_resume_lane_vocabulary_documented` | Lane model is fixed | grep for `micro`, `milestone`, and `fresh-runbook` in the orientation logic |
| `slo_resume_output_stays_short` | Minimal visible ceremony preserved | output section explicitly says short message / one screen / top 3 carry-forward items max |
| `slo_resume_no_auto_start_preserved` | Read-only rule still intact | grep for "Do not start the next action" or equivalent |

#### Smoke Tests

- [ ] Run `/slo-resume` against a runbook with no carry-forward rows; verify the old simple orientation still works.
- [ ] Run `/slo-resume` against a runbook with `micro`, `milestone`, and `fresh-runbook` rows; verify the lane appears in output.
- [ ] Verify output stays short when the carry-forward table is long.
- [ ] `cargo test -p sldo-install` passes.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline | `cargo test --workspace` | green | | | |
| Test stub created | `tests/e2e_loops_m5.rs` | fails for missing carry-forward-aware contract | | | |
| `/slo-resume` updated | inspect file | tracker + carry-forward + lane + short output rules present | | | |
| Empty-state smoke | manual `/slo-resume` run | correct simple orientation | | | |
| Carry-forward smoke | manual `/slo-resume` run | one next action + lane + compact output | | | |
| Full tests | `cargo test --workspace` | green | | | |
| `.gitignore` review | | no new patterns needed | | | |

#### Definition of Done

- All BDD scenarios pass.
- `/slo-resume` stays read-only and compact.
- Lane vocabulary is documented and matches M4 (`micro | milestone | fresh-runbook`).
- A user can recover one next action without rereading the full runbook.
- Lessons + completion files written.
- Tracker updated.

#### Post-Flight

- README.md update: optional, if the docs index calls out `/slo-resume` as the orientation entrypoint.
- No runbook-template change required beyond M4.

#### Notes

- This is intentionally not a new `/slo-help` skill. The product lesson from BMAD is "one obvious way to ask what's next", not "more verbs".

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | .gitignore Update | Other Docs |
|---|---|---|---|---|
| 1 | Add "Feedback loops" section linking LOOPS-ENGINEERING.md | optional bullet in docs index | none | Per-skill SKILL.md cross-references |
| 2 | Update "Feedback loops" section to link LOOPS-BUSINESS.md too | optional | none | Per-biz-skill SKILL.md cross-references |
| 3 | Note `/slo-retro` issue-filing extension | optional | none | `skills/slo-retro/references/issue-filing-discipline.md` (new) |
| 4 | Note `/slo-execute` pre-flight loop closure | optional | none | `docs/runbook-template_v_3_template.md` (new section + lane column) |
| 5 | none required | optional: mention `/slo-resume` as "what next?" entrypoint | none | `skills/slo-resume/SKILL.md` |

---

## Optional Fast-Fail Review Prompt for Agents

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope, and explain how the user-facing result reduces user decisions or reviewer work.

---

## Carry-forward from prior retros

(This section is the dogfood of M4's template change. It is empty until M3 produces filings; M5 then uses the section as the read-only orientation input. The section is here so the artifact-shape is visible even before real rows accumulate.)

| Issue | Title | Suggested lane | Suggested milestone | Status |
|---|---|---|---|---|
| (none yet — M1) | | | | |

---

## Paradigm-driven enhancements (per `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`)

This runbook applies the over-engineering-for-simplicity paradigm — LLM-driven workflows can absorb more discipline than human-driven equivalents because LLMs do not pay the cognitive-load tax. Specific layers added because the LLM is the executor:

### Multi-layer dedupe defense (M3)

Where a human-driven implementation would pick ONE marker scheme (title prefix, label, or body sentinel) and rely on a single `gh search` query, this runbook layers:

1. **Three-strike dedupe**: literal search + NFKC-normalized search + ASCII-collapsed search (defends against homoglyph + zero-width + RTL-override evasions).
2. **Body-SHA-256 cross-session dedupe**: 12-char prefix in the LESSONS-BACKLOG.md row catches duplicates filed across sessions (different agents, same lesson).
3. **User-confirmation gate**: every filing surfaces the search-result disposition (`none` / `match-id` / `ambiguous`) so the user sees what dedupe found.
4. **Audit trail**: every audit row records the dedupe outcome so future debugging can trace why a duplicate slipped through.

Each layer adds zero user friction; combined, they catch the failure modes a single-layer human implementation would miss.

### Comprehensive abuse-case enumeration

M3's BDD table enumerates 9 distinct abuse / degraded-state scenarios (homoglyph, zero-width, RTL override, code-fence injection, body > GitHub cap, markdown reference cycle, rate-limit during filing, cross-session duplicate, multi-runbook prefix collision). A human-authored equivalent would typically enumerate 2-3.

### Comprehensive audit row schema

LESSONS-BACKLOG.md row carries 12 fields (date, classification, prefix, agent_version, originating_milestone, dedupe_search_result, filed_to, issue_url_or_local_ref, disposition, body_sha256, retry_count, status). A human-authored row would carry 4-5; the additional fields are no marginal cost to LLM authoring + filing, and unlock cross-session dedupe + audit replay.

### Defense-in-depth across milestones

The paradigm's "multi-layer defense without user friction" pattern:

| Concern | Layer 1 | Layer 2 | Layer 3 | Layer 4 |
|---|---|---|---|---|
| Issue body injection | argv-list at `gh issue create` | `~~~text` fence around lesson body | issue-template structured fields | M4 carry-forward also fence-wraps when surfacing |
| Confused-deputy via `.git/config` | NO `--repo` flag (M3 + M4) | confirmation gate shows resolved origin | carry-forward filter by prefix (limits cross-repo surface) | argv-list (no shell-string interpolation of remote URL) |
| Lessons drop-out | `/slo-retro` always writes lessons file FIRST | issue filing additive after | M4 surfaces at next milestone | LESSONS-BACKLOG.md fallback if `gh` unavailable |
| Marker drift | single canonical marker chosen in M3 spike | structural-contract test asserts marker-format | three-strike search defends against evasion | spike-result review at +6 months (per critique L-3) |

### One-screen orientation instead of more process

The main user-facing simplicity rule added by this revision is: a stricter loop is only a win if the operator can still answer "what next?" without rereading the whole system. M4 introduces the `micro | milestone | fresh-runbook` lane so carry-forward does not silently widen scope; M5 teaches `/slo-resume` to turn that structure into a single next-step digest. This is the practical application of the BMAD comparison: keep the internal discipline, but make recovery and orientation feel lighter.

### Anti-process-theatre check

Every added user-visible surface in this runbook must pass one question: **does it reduce user decisions or reviewer work?** If not, it belongs in a reference file, a structural-contract test, or nowhere. That rule is the counter-weight to the paradigm's "add more discipline" instinct.

### Bounded by context-window

This runbook stays inside the paradigm's "balance against context window, not attention" rule: the v3 template is the orchestrator (used by `/slo-plan`); methodology lives in `references/issue-filing-discipline.md` (consulted on-demand); the LESSONS-BACKLOG.md row schema is one line of text in the contract block; abuse-case BDD rows are individually small; `/slo-resume` compresses the result back down to one screen for the user. No file in this runbook approaches the soft 200-line cap from R2.
