# Engineering loops — SunLitOrchestrate

> **Purpose**: name the cyclic feedback structures that move work through the engineering side of the skill pack, so a newcomer (human or freshly-loaded Claude instance) can answer "which loop am I in, and what do I run next?" in 90 seconds.
>
> **Companion doc**: business loops live at [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md). Static structure is in [docs/ARCHITECTURE.md](ARCHITECTURE.md). This doc is the cyclic complement.

---

## Start here

Pick the row that matches the question you have right now. The "First skill" column is what to run; the "Loop" column is the section below that explains why.

| Your question | First skill | Loop | Expected artifact |
|---|---|---|---|
| "I have an idea — is it worth building?" | `/slo-ideate` | [Sprint loop](#sprint-loop) | `docs/idea/<slug>.md` |
| "I'm starting a new feature, what do I do?" | `/slo-ideate` then `/slo-research` | [Sprint loop](#sprint-loop) | `docs/RUNBOOK-<feature>.md` once `/slo-plan` completes |
| "I have a repeated regression — where do I start?" | `/slo-resume` (orient) then check prior `docs/lessons/` | [Lessons loop](#lessons-loop) | A scope candidate at the next milestone's pre-flight |
| "Findings keep coming back from SAST — how do I tune?" | `/slo-rulegen --extend` | [Security-tuning loop](#security-tuning-loop) | A new rule pack rev under `.semgrep/<lang>/` |
| "An upstream tool has a gap — what now?" | `/slo-sec-libs` (when shipped) | [Library-feedback loop](#library-feedback-loop) | An issue in the upstream repo |
| "I stepped away — where was I?" | `/slo-resume` | (any) | A one-screen orientation message |

Each loop below documents **user-visible outcome**, **trigger**, **steps**, **exit condition**, **artifacts**, **skills involved**, and a **diagram**.

---

## Sprint loop

> **User-visible outcome**: a runbook closes with all milestones `done`, a PR is open, and the lessons file teaches the next sprint.

**Trigger**: a new feature or non-trivial change is about to start.

**Steps**:

1. `/slo-ideate` — interrogate the idea, produce `docs/idea/<slug>.md`.
2. `/slo-research` — sourced dossier under `docs/research/<slug>/`.
3. `/slo-architect` — `ARCHITECTURE.md` updates plus stack lock-in, sets `tla_required`.
4. `/slo-tla` — only when `tla_required: true`; verify the design.
5. `/slo-plan` — author `docs/RUNBOOK-<feature>.md` interactively, one milestone at a time.
6. `/slo-critique` — adversarial four-pass review BEFORE any milestone executes.
7. Per milestone: `/slo-execute M<N>` → `/slo-verify M<N>` → `/slo-retro M<N>`.
8. `/slo-ship` — open the PR with a runbook-aware description.

**Exit condition**: every milestone tracker row is `done`, every Evidence Log row has an Actual Result, the PR is open, and a completion summary plus lessons file is written.

**Artifacts**: `docs/idea/<slug>.md`, `docs/research/<slug>/`, `docs/RUNBOOK-<feature>.md`, `docs/lessons/<prefix>-m<N>.md`, `docs/completion/<prefix>-m<N>.md`, the PR.

**Skills involved**: `/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-plan`, `/slo-critique`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship`.

```
   /slo-ideate ──► /slo-research ──► /slo-architect ──► /slo-plan
        ▲                                               │
        │                                               ▼
        │                                          /slo-critique
        │                                               │
        │                                               ▼
   /slo-retro ◄── /slo-verify ◄── /slo-execute ─────────┘
        │                                               ▲
        │                                               │
        └───────────► next milestone ──────────────────┘
                              │
                              ▼
                          /slo-ship
```

---

## Security-tuning loop

> **User-visible outcome**: SAST signal stays sharp — false positives drop, real findings keep landing, and every fix produces a regression rule that catches the next variant.

**Trigger**: a SAST finding (true positive or false positive) lands, OR a new threat-model row introduces a CWE the current rule pack does not cover, OR `/slo-architect` sets `security_libs_required: true` and the existing pack does not yet have rules for the named capability.

**Steps**:

1. `/slo-architect` — confirm or update the threat model row (CWE references, abuse cases) that the rule should defend.
2. `/slo-sast` — emit or refresh the workflow plus baselined `.semgrep.yml` so the new rule lands in CI.
3. `/slo-rulegen --extend` — generate 3-5 variation rules from the agent-found bug summary plus fix diff. New rules are appended ONLY after `cargo xtask sast-verify gate` passes for every rule.
4. `/slo-ruleverify` — re-run the deterministic gate (`validate + test + check-coverage + check-clean`) to confirm the pack still passes end-to-end.
5. `/slo-verify` — runtime QA against the BDD scenarios that introduced the finding.
6. `/slo-critique` — security pass surfaces residual risk and abuse-case coverage gaps.

**Exit condition**: `cargo xtask sast-verify gate` is green, the new rule(s) detect the original variant plus 2-3 reasonable evasions, and the threat model row that motivated the rule references it.

**Artifacts**: `.semgrep/<lang>/<rule>.yml`, paired test corpora, `.semgrep/manifest.json`, the threat-model row update.

**Skills involved**: `/slo-architect`, `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-verify`, `/slo-critique`.

```
   threat-model row (CWE)
            │
            ▼
       /slo-sast ─────► .github/workflows/sast.yml
            │
            ▼
   /slo-rulegen --extend ──► .semgrep/<lang>/*.yml
            │
            ▼
   /slo-ruleverify ─► cargo xtask sast-verify gate
            │
            ▼          (gate green = rule lands)
       /slo-verify ──► /slo-critique (security pass)
            │
            └──► next finding feeds back into the threat model row
```

---

## Lessons loop

> **User-visible outcome**: a lesson learned at milestone M<N> is visible at milestone M<N+k>'s pre-flight, NOT just buried in a markdown file. The same regression does not get re-introduced two milestones later.

**Trigger**: `/slo-retro` runs at the close of any milestone.

**Steps**:

1. `/slo-retro` writes `docs/lessons/<prefix>-m<N>.md` (always — discipline rule, even if `gh` is unavailable).
2. `/slo-retro` classifies each lesson as `product`, `upstream-OSS`, or `slo-process`.
3. `/slo-retro` dedupes via `gh search issues` (three-strike: literal + NFKC-normalized + ASCII-collapsed).
4. `/slo-retro` files each lesson as a tracked issue with **explicit user confirmation** — never auto-files.
5. Fallback: when `gh` is unavailable, the lesson is appended to `LESSONS-BACKLOG.md` with a 12-field audit row (date, classification, prefix, agent_version, originating_milestone, dedupe_search_result, filed_to, issue_url_or_local_ref, disposition, body_sha256, retry_count, status).
6. At the next milestone, `/slo-execute` pre-flight queries open prior-retro issues filtered by the runbook's prefix and surfaces them as scope candidates with a suggested lane (`micro | milestone | fresh-runbook`).
7. `/slo-resume` reads the runbook tracker plus the optional "Carry-forward from prior retros" section to emit one next action and lane.

**Exit condition**: every lesson worth filing is either filed (with confirmation) or recorded in `LESSONS-BACKLOG.md`; the next milestone's pre-flight surfaces the open ones.

**Artifacts**: `docs/lessons/<prefix>-m<N>.md`, `docs/completion/<prefix>-m<N>.md`, GitHub issues with `retro-derived` marker (locked in `skills/slo-retro/references/issue-filing-discipline.md`), optional `LESSONS-BACKLOG.md` rows, the runbook's "Carry-forward from prior retros" section.

**Skills involved**: `/slo-retro`, `/slo-execute`, `/slo-resume`.

```
   /slo-retro M<N>
        │
        ├── writes docs/lessons/<prefix>-m<N>.md (always)
        │
        ├── classify each lesson ─► product / upstream-OSS / slo-process
        │
        ├── dedupe via gh search (three-strike)
        │
        ├── confirm with user ──► gh issue create (argv-list, no --repo)
        │                          OR LESSONS-BACKLOG.md (gh unavailable)
        ▼
   /slo-execute M<N+k> pre-flight
        │
        ├── gh issue list --label retro-derived --search prefix
        │
        └── surface as scope candidates
                │
                ▼
        /slo-resume reads tracker + carry-forward
                │
                └── emits one next action + lane (micro | milestone | fresh-runbook)
```

---

## Library-feedback loop

> **User-visible outcome**: when SLO discovers a capability gap or bug in an upstream tool (Semgrep, Playwright, `cargo audit`, etc.), the lesson does not die in a local markdown file — it gets filed against the upstream repo and re-checked when the upstream improves.

> **Status**: the dedicated upstream-filing surface ships in **Runbook 4** (`/slo-sec-libs`). Until then, upstream-OSS classified lessons go through the [Lessons loop](#lessons-loop) and rely on `/slo-retro`'s issue-filing flow with the upstream-OSS classification (resolved via `.sldo/upstream-mapping.toml`). This section is here so the loop is documented up-front; the dedicated skill is the next iteration, not a removed feature.

**Trigger**: `/slo-execute` (or another skill) discovers a bug or capability gap in an upstream tool while pursuing the current milestone's contract.

**Steps** (target shape, ships with R4):

1. Capture the gap as a lesson during `/slo-retro` with classification `upstream-OSS`.
2. Resolve the upstream repo via `.sldo/upstream-mapping.toml` (with crates.io / npm fallback resolution).
3. `/slo-sec-libs` files an issue against the resolved upstream repo via `gh issue create` (argv-list discipline, NO `--repo` flag, rate-limit cap of 40 issues/hour per session, body wrapped in `~~~text` fence per `/slo-architect` template).
4. The local milestone proceeds with whatever workaround the runbook's allow-list permits.
5. When the upstream issue is closed and a release ships, the next sprint loop iteration re-checks against the new upstream version; if the gap is fixed, the workaround is removed and a regression test pinned to the new upstream version is added.

**Exit condition** (target): every `upstream-OSS` lesson is either filed against an upstream repo or recorded in `LESSONS-BACKLOG.md` with the `filed_to: <upstream>` audit row, and the next sprint that touches that subsystem re-checks for upstream resolution.

**Artifacts**: filed upstream issues; `.sldo/upstream-mapping.toml`; `LESSONS-BACKLOG.md` rows for unfileable items.

**Skills involved**: `/slo-sec-libs` (Runbook 4), `/slo-retro`, `/slo-execute`.

```
   /slo-execute M<N>
        │
        │ (discovers upstream gap)
        ▼
   /slo-retro classifies as `upstream-OSS`
        │
        ▼
   resolve upstream via .sldo/upstream-mapping.toml
        │
        ▼
   /slo-sec-libs ──► gh issue create (argv-list, no --repo)
                       │
                       ▼
              upstream repo: <owner>/<project>
                       │
                       ▼
              upstream fix lands in a release
                       │
                       ▼
   next sprint re-checks; remove workaround; add pinned regression test
```

---

## Anti-process-theatre check

Every loop here exists because it produces a user-visible outcome the static architecture doc cannot make visible. Loop diagrams are kept short on purpose: the artifact tells you the answer; the loop only names the cycle.

If a future addition to this doc cannot point at a concrete user-visible outcome that an existing loop already produces, that addition belongs in a skill's reference file, not in this doc.

---

## See also

- [docs/ARCHITECTURE.md](ARCHITECTURE.md) — static structure of the skill pack at HEAD.
- [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md) — business-side loops (user-interview, GTM, pricing, founder-check).
- [docs/runbook-template_v_4_template.md](runbook-template_v_4_template.md) — the canonical planning artifact whose "Carry-forward from prior retros" section is the lessons loop's read-back. (The earlier [v3 template](runbook-template_v_3_template.md) remains in place for runbooks already authored against it.)
- [skills/slo-retro/SKILL.md](../skills/slo-retro/SKILL.md) — the writer end of the lessons loop.
- [skills/slo-execute/SKILL.md](../skills/slo-execute/SKILL.md) — the reader end of the lessons loop (pre-flight carry-forward).
- [skills/slo-resume/SKILL.md](../skills/slo-resume/SKILL.md) — one-screen orientation across loops.
