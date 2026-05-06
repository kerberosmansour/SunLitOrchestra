# `docs/slo/` — SunLitOrchestrate runbooks and milestone artifacts

Everything under this directory is **work / task information** produced by the `/slo-*` skill pack: runbooks, milestone outputs, design notes, research dossiers, and templates. Code-level documentation (`docs/ARCHITECTURE.md`, `docs/getting-started.md`, `docs/skill-pack-catalog.md`, `docs/LOOPS-*.md`, `docs/PARADIGM-*.md`) lives one level up at `docs/`.

Historical artifacts in `completed/`, `completion/`, and `lessons/` may retain the former secure-libraries repo spelling used at the time they were written. Live docs should use the canonical public repo: `kerberosmansour/SunLitSecurityLibraries`.

## Layout

| Path | What lives here |
|---|---|
| [`current/`](current/) | Runbooks for work currently in progress (one milestone tagged `in_progress`). |
| [`completed/`](completed/) | Runbooks whose every milestone is `done`. Move a runbook here once `/slo-retro` closes the last milestone. |
| [`future/`](future/) | Runbooks queued up but not yet started (every milestone `not_started`). Drop them into `current/` when work begins. |
| [`templates/`](templates/) | Runbook templates (v3, v4) and supporting reference templates. |
| [`tickets/`](tickets/) | Ticket-sized SLO contracts created from GitHub Issues by `/slo-ticket-plan`. |
| [`idea/`](idea/) | `/slo-ideate` outputs — the YC-style interrogation that precedes every runbook. |
| [`research/`](research/) | `/slo-research` dossiers (one subdirectory per slug). |
| [`design/`](design/) | `/slo-architect` outputs — overview, interfaces, threat model, stack decision per slug. |
| [`critique/`](critique/) | `/slo-critique` four-persona adversarial reviews. |
| [`completion/`](completion/) | Per-milestone completion summaries written by `/slo-retro`. |
| [`lessons/`](lessons/) | Per-milestone lessons-learned files written by `/slo-retro`. |
| [`verify/`](verify/) | `/slo-verify` smoke and runtime QA reports. |

## Runbook lifecycle

```
future/  →  current/  →  completed/
   ▲           │             │
   │           │             │
ideate/    plan +         retro M_last
research/  execute +      moves it here
architect/ verify
critique/
```

A runbook moves between `future/`, `current/`, `completed/` based on its **Milestone Tracker**: the moment the first milestone flips from `not_started` to `in_progress`, move the file from `future/` → `current/`; the moment the last milestone flips to `done`, move it from `current/` → `completed/`.

The supporting subdirectories (`design/`, `idea/`, `research/`, etc.) stay flat — they are indexed by slug, not by lifecycle phase.

Ticket-sized issue work does not move through `future/` → `current/` → `completed/`. It lands in `tickets/` as `ticket-<issue>-<slug>.md` and carries its own validation and closure summary. If a ticket fails the sizing gate, promote it to the normal runbook lifecycle.
