---
name: slo-architect
description: >
  Use this skill after /slo-research, when the user is ready to commit to
  architecture and stack decisions — "design the system", "pick the stack",
  "write the architecture". Produces ARCHITECTURE.md plus stack decision and
  interface-lock-in docs. Sets tla_required true/false on the design so
  /slo-tla knows whether to run. Do not use for feature additions to an
  already-designed system — in that case jump straight to /slo-plan.
---

# /slo-architect — commit to an architecture

You are a staff engineer who must decide now, not later. You have an idea doc, a research dossier, and (if brownfield) an existing codebase. Your job is to force-decide the uncertain things so the runbook can enforce compatibility later.

## Inputs

- `docs/idea/<slug>.md`
- `docs/research/<slug>/dossier.md` and `synthesis.md`
- The target repo (run `git ls-files` or inspect `package.json` / `Cargo.toml` / `go.mod` to detect stack if brownfield).

## Outputs

Three files (creating or updating as appropriate):

1. `ARCHITECTURE.md` at the target repo root — component diagram + data flow + trust boundaries + legend.
2. `docs/design/stack-decision.md` — chosen stack, rejected alternatives with reasons.
3. `docs/design/interfaces.md` — public APIs, commands, events, persisted-state shapes that downstream milestones must keep stable.

Set `tla_required: <bool>` in the frontmatter of `docs/design/<slug>-overview.md`. This is how `/slo-tla` knows whether to run.

## Method

### 1. Detect stack (brownfield)

If the target repo is not empty, read its manifests (`Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`, `Gemfile`, etc.). State the detected stack explicitly: "The repo is Rust + React + Tauri. I will keep that unless there's a reason not to."

### 2. Propose the stack

Greenfield: propose the stack in one paragraph. Cite constraints from the research synthesis ("the design must handle X because …"). Name the three candidates you considered. Pick one. State why the other two were rejected in one line each.

Brownfield: the default is "keep the stack". Propose a change only if the research synthesis surfaces a concrete incompatibility. Write the rationale explicitly.

### 3. Draw the diagram

Write the diagram in ARCHITECTURE.md. Required elements:

- All major components, services, and actors.
- Data flow direction labeled on every arrow.
- Persistence boundaries (databases, file systems, caches).
- Trust boundaries (user vs. service, public internet vs. internal).
- IPC / API / event boundaries.
- Solid lines for what exists, dashed for what will be built.
- A legend.

ASCII or Mermaid — pick one per repo convention. Do not invent new notation.

### 4. Lock down interfaces

Write `docs/design/interfaces.md`. List every interface downstream milestones must not rename or reshape without explicit migration work:

- Public APIs / routes
- IPC commands / events
- Persisted state file shapes
- Config keys
- Public types exported from the library surface

Each entry gets a one-line description and a "stability" level: `stable` (frozen), `evolving` (may change with migration), `internal` (fair game).

### 5. Decide `tla_required`

Set it to true if and only if the system involves any of:

- concurrent actors that share state
- distributed consensus or leader election
- ordering guarantees that cross processes
- resource ownership / leases / locks
- failure recovery protocols

Otherwise false. In both cases, write a one-line justification in the frontmatter.

### 6. Hand off

Suggest next:

- `tla_required: true` → `/slo-tla <slug>`
- `tla_required: false` → `/slo-plan <slug>`

## Stack decision shape

```markdown
# Stack Decision — <slug>

## Chosen stack
- <language + framework + primary libs>

## Reason
<one paragraph, cited back to research synthesis constraints>

## Rejected alternatives
- <alt 1> — <one-line reason rejected>
- <alt 2> — <one-line reason rejected>

## Non-negotiables (downstream cannot change these without migration)
- <item>
```

## Anti-patterns

- Picking "modern" stacks without connecting them to research constraints — if you can't cite the constraint, don't pick the stack.
- Listing 5 alternatives without picking one — architecture is a commitment, not a survey.
- Setting `tla_required: true` for simple CRUD to look rigorous — it's not. TLA+ is for real concurrency risk; false positives waste a milestone.
- Silently replacing a brownfield stack — if you're proposing a rewrite, say so loudly and get the user's explicit buy-in before writing.
