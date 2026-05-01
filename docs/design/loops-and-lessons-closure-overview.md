---
name: loops-and-lessons-closure
created: 2026-04-27
status: design lock-in
tla_required: false
security_libs_required: false
ai_component: false
compliance: [soc2]
---

# Design overview — loops + lessons closure

## System goal

Make the SLO skill pack's cyclic feedback structures (engineering loops, business loops, lessons loop) **first-class artifacts** that:

1. New contributors and freshly-loaded Claude instances can read in 90 seconds.
2. Are referenced by their constituent skills (cross-link from each implicated SKILL.md).
3. Drive automated lesson-→issue filing via `/slo-retro` extension.
4. Close the loop at milestone start via `/slo-execute` pre-flight pulling open prior-retro issues as scope candidates.

## Stack decision

This runbook is documentation + skill-prose extension + minor `gh` CLI wiring. Stack is unchanged from project-wide:

- Markdown for all loop docs and runbook artifacts
- `gh` CLI for issue search + creation (existing dependency)
- Rust (`crates/sldo-install`) for any structural-contract test additions

No new runtime dependencies. No schema migrations.

## Components

| Component | Responsibility | Milestone introduced/changed | Key interfaces |
|---|---|---|---|
| `docs/LOOPS-ENGINEERING.md` | First-class engineering-loops doc | M1 | Cross-linked from `ARCHITECTURE.md`, every implicated engineering SKILL.md |
| `docs/LOOPS-BUSINESS.md` | First-class business-loops doc | M2 | Cross-linked from each implicated biz SKILL.md |
| `/slo-retro` extension | Classifies each lesson + dedupes via `gh search` + files issue with confirmation | M3 | `gh issue create` (argv-list); fallback `LESSONS-BACKLOG.md` |
| `/slo-execute` pre-flight extension | Reads open prior-retro issues at milestone start; surfaces as scope candidates | M4 | `gh issue list` (argv-list); reads from runbook's `<prefix>` for filtering |
| `docs/templates/runbook-template_v_3_template.md` | Adds "Carry-forward from prior retros" section | M4 | Read by `/slo-plan` and `/slo-resume` |

## Data flow

```
/slo-retro M<N>
  ┌─────────────────────────────────┐
  │ Write docs/lessons/<prefix>-m<N>.md │
  └──────────────┬──────────────────┘
                 │
                 ▼
        ┌────────────────┐
        │ Classify each   │     ┌─ product       ──→ current repo
        │ lesson          │ ──→ ┼─ upstream-OSS  ──→ resolved upstream repo
        │ (LLM judgment)  │     └─ slo-process   ──→ kerberosmansour/SunLitOrchestrate
        └────────┬───────┘
                 │
                 ▼
        ┌────────────────┐
        │ gh search issues │ — dedupe
        └────────┬───────┘
                 │
                 ▼
        ┌────────────────┐
        │ Confirm with    │ — never auto-file (issue creation is publicly visible)
        │ user            │
        └────────┬───────┘
                 │
                 ▼
        ┌────────────────┐
        │ gh issue create │ OR LESSONS-BACKLOG.md fallback
        └────────────────┘

/slo-execute M<N+k>
  ┌────────────────────────────────────────┐
  │ Pre-flight: query open issues filed   │
  │ by /slo-retro for prior milestones    │
  │ in this runbook's prefix              │
  └─────────────────┬──────────────────────┘
                    │
                    ▼
           ┌─────────────────┐
           │ Surface as scope │
           │ candidates       │
           └─────────────────┘
```

## Trust boundaries

- `gh` CLI invocation runs with the user's GitHub auth — same trust posture as existing `/slo-ship`.
- Issue body content originates from `docs/lessons/<prefix>-m<N>.md` — author-controlled, not user-input. Low injection risk; argv-list discipline is the standard defense.
- `LESSONS-BACKLOG.md` fallback is local-only — no remote.

## Interfaces locked

| Interface | Stability | Notes |
|---|---|---|
| `docs/LOOPS-*.md` file path + per-loop section schema | `stable` | Cross-referenced from SKILL.md files |
| Issue title prefix (TBD in M3 spike) | `stable` once chosen | `gh search` reliability anchor |
| `LESSONS-BACKLOG.md` row format | `stable` | Local fallback consumers |
| Runbook template's "Carry-forward from prior retros" section | `stable` | Read by `/slo-resume` and downstream tooling |
| `/slo-retro` and `/slo-execute` SKILL.md prose updates | `evolving` | Iterative; lessons file improvements expected |

## TLA+ section

Not required (`tla_required: false`). No concurrent actors, no distributed state, no ordering guarantees, no resource ownership / leases / locks, no failure recovery protocols. The "loop closure" is sequential — retro writes → issue files → next milestone reads — with no race surface.

## STRIDE sweep (per Step 3.5)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | EoP |
|---|---|---|---|---|---|---|
| `/slo-retro` issue filing | N/A — uses user's `gh` auth | mitigated by argv-list discipline (no shell interpolation of lesson body) | mitigated — every filing carries author + timestamp via `gh` | residual — lesson content goes to remote; mitigated by user-confirmation gate before `gh issue create` | mitigated by client-side cap (reuse `/slo-sec-libs` rate limit pattern) | N/A — no privilege escalation surface |
| `/slo-execute` pre-flight issue read | N/A | N/A — read-only `gh issue list` | N/A | N/A — only reads issues already public in the repo | mitigated — single `gh issue list` per milestone start | N/A |
| `LESSONS-BACKLOG.md` fallback | N/A | low — local file, repo `.gitignore`-controlled | N/A | residual — file is local; if remote pushed, lesson content is exposed (author intent) | N/A | N/A |

No new abuse cases beyond `tm-loops-abuse-1: prompt injection via lesson body content` — mitigated by `~~~text` user-string fence rule already in `/slo-architect`'s SECURITY.md template (load-bearing for any template-authoring skill).

## Compatibility commitments

- No breaking changes to `docs/RUNBOOK-*.md` files: existing runbooks without "Carry-forward from prior retros" section remain valid; the new section is optional in M4's template change.
- `/slo-retro` extension preserves the existing lessons-file template; the issue-filing is additive after the file is written.
- `/slo-execute` extension adds a step to pre-flight; existing milestones without retro-originated issues see "no carry-forward" and proceed unchanged.

## Out-of-scope

- Substantive retro content quality (this runbook ships the loop mechanism; lesson quality is the responsibility of the agent invoking `/slo-retro`).
- Cross-repo issue federation (R4 `/slo-sec-libs` covers third-party filing for capability gaps; this runbook's upstream-OSS classification reuses `/slo-sec-libs` rate-limit pattern but is independent).
- Visualization / dashboard for the loops (loops are markdown docs in v1).
