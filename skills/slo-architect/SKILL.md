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

- `docs/slo/idea/<slug>.md`
- `docs/slo/research/<slug>/dossier.md` and `synthesis.md`
- The target repo (run `git ls-files` or inspect `package.json` / `Cargo.toml` / `go.mod` to detect stack if brownfield).

## Outputs

Seven files (creating or updating as appropriate):

1. `ARCHITECTURE.md` at the target repo root (or `docs/ARCHITECTURE.md` per repo convention) — component diagram + data flow + trust boundaries + legend.
2. `docs/slo/design/<slug>-stack-decision.md` — chosen stack, rejected alternatives with reasons.
3. `docs/slo/design/<slug>-interfaces.md` — public APIs, commands, events, persisted-state shapes that downstream milestones must keep stable.
4. `SECURITY.md` at the target repo root — project-wide security rules, generated from `references/SECURITY-md-template.md`. Emitted when `security_libs_required: true` OR the idea doc's `## Top risks` block is non-trivial. This file is read by every downstream agent before generating code (the "project-wide security defaults" contract).
5. `docs/slo/design/<slug>-threat-model.md` — STRIDE per component + abuse cases + compliance mapping, generated from `references/threat-model-template.md`. Always emitted (even for small systems — `N/A — <reason>` rows are valid).
6. `docs/slo/design/<slug>-reversibility.md` — hard-to-change decisions, why each is hard to reverse, the reversibility tactic, rollback / migration path, and proof required.
7. `docs/slo/design/<slug>-code-map.md` — for a non-empty brownfield repo, a four-object summary, exemplar code to copy, anti-exemplar code not to copy, and dangerous seams. Greenfield projects write `N/A — greenfield; no existing codebase to map`.

Set four frontmatter keys in `docs/slo/design/<slug>-overview.md`:

- `tla_required: <bool>` — how `/slo-tla` knows whether to run (see Step 5).
- `security_libs_required: <bool>` — how `/slo-sec-libs` (Phase 4 of the security-embedding work) knows whether to recommend Hulumi / SunLitSecurityLibraries components. Default when absent: `false`.
- `ai_component: <bool>` — `true` when the target system invokes or embeds an LLM / AI agent. Gates the MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad in the threat model. Default when absent: `false`.
- `compliance: [<list>]` — framework columns for the threat model. Default when absent: `[soc2, asvs]`. Allowed values: `soc2`, `asvs`, `gdpr`, `hipaa`, `pci-dss`, `nist-800-53`, `iso-27001`. Unknown values are rejected by the frontmatter-type-check documented below.

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

### Step 3.5 — STRIDE sweep + emit SECURITY.md + emit threat-model.md

Before locking interfaces, produce the security artifacts. This is the 80/20 burden rule: the architect generates; the user reviews.

1. **Read the idea doc's `## Top risks` block.** If missing, the prior `/slo-ideate` was run without Q7 — note the gap; do not refuse. Proceed with an explicit "top risks not provided by ideate; using conservative defaults" remark in the threat model.
2. **Walk every component in the diagram** and ask the STRIDE questions per component:
   - **Spoofing** — can a caller impersonate another principal?
   - **Tampering** — can data be modified in transit or at rest undetectably?
   - **Repudiation** — can an actor deny having done an action?
   - **Information disclosure** — can a principal see data they shouldn't?
   - **Denial of service** — can an attacker saturate a bounded resource?
   - **Elevation of privilege** — can a low-privilege path reach a high-privilege operation?
   For each cell, write one of: `eliminated by <control>`, `mitigated by <control>`, `N/A — <reason>`, or `residual risk — <path>`. Class-elimination framing (not bug-instance) is the standard `/slo-critique` consumes downstream.
3. **Generate three abuse cases per new surface** (endpoint, IPC handler, file path written, outbound request, subprocess invocation). Each abuse case has an attacker, an attack step, a desired outcome, a control, and a stable id (`tm-<slug>-abuse-N`) that `/slo-plan` M2+ cites in milestone BDD scenarios.
4. **Emit `SECURITY.md`** at the target repo root using `references/SECURITY-md-template.md`. Fill every `{{PLACEHOLDER}}` — never leave blank; use `N/A — <reason>` when a section does not apply. **User-provided strings from the idea doc (Top risks, etc.) are always wrapped in a `~~~text` fence** so Markdown / HTML / YAML metacharacters are literal, not interpretable. This rule is non-negotiable — it is the defense against template-placeholder injection.
5. **Emit `docs/slo/design/<slug>-threat-model.md`** using `references/threat-model-template.md`. Same `~~~text` fence rule for user strings. Populate the AI-specific section only when `ai_component: true`. GDPR gets both a column and a section when `gdpr` is in the `compliance:` list.
6. **Set the frontmatter keys** in `<slug>-overview.md`: `security_libs_required`, `ai_component`, `compliance`. Types: bool, bool, list-of-allowed-strings. Values outside the allowed set are a user error — surface the problem, do not coerce.
7. **Re-run behavior (idempotency).** If `SECURITY.md` or `<slug>-threat-model.md` already exists (i.e., `/slo-architect` has run before), do NOT silently clobber. Detect the existing file, diff against what would be regenerated, surface the diff to the user, and prompt: **overwrite** (apply regeneration), **merge** (preserve user edits where possible; regenerate only untouched sections), or **skip** (leave the file alone). Default on missing user input: prompt again; never overwrite by default.

After Step 3.5, the threat model is the artifact `/slo-plan`, `/slo-critique`, and `/slo-verify` all cite.

### Step 3.6 — Emit reversibility matrix + brownfield code map

Write `docs/slo/design/<slug>-reversibility.md` with a table covering each hard-to-change decision: decision, why hard to change, reversibility tactic, rollback / migration path, and proof required. If there are no hard-to-change decisions, write an explicit `N/A — <reason>` row rather than omitting the artifact.

Write `docs/slo/design/<slug>-code-map.md` after direct repo inspection. For non-empty brownfield repos, include:

- Four-object summary — the main objects/modules/artifacts and how they relate.
- Exemplar code to copy — concrete file paths and the shape worth following.
- Anti-exemplar code not to copy — concrete file paths or patterns that are legacy, risky, or misleading.
- Dangerous seams — boundaries where agents should inspect before editing.

For greenfield projects, the code map is `N/A — greenfield; no existing codebase to map`. On re-run, apply the same idempotency rule as Step 3.5: detect existing reversibility/code-map files, diff proposed changes, and prompt for overwrite, merge, or skip. Never silently clobber existing design artifacts.

### 4. Lock down interfaces

Write `docs/slo/design/interfaces.md`. List every interface downstream milestones must not rename or reshape without explicit migration work:

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
- **Leaving threat-model sections blank with "fill this in" prompts.** The value is that the architect *generates* — the user reviews. An empty threat model is worse than none: it signals effort without delivering signal.
- **Skipping the `~~~text` fence rule for user-provided strings in the generated SECURITY.md / threat-model.md.** Without the fence, an attacker or unwary user can smuggle prompt content through an idea doc into the project's security defaults. The fence is load-bearing; do not "clean up formatting" by removing it.
- **Silently overwriting existing SECURITY.md or threat-model.md on re-run.** The idempotency rule (overwrite / merge / skip prompt) is what protects user edits. Always diff; always prompt.

---

**Loops**: Sprint loop, Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
