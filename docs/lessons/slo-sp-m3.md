# Lessons Learned — slo-sp Milestone 3

## What changed
- `/slo-research` skill authored.
- 4 new E2E tests that check the skill's static contract: frontmatter valid, references backend, has preflight cascade, delegates API-doc lookups to chub.

## Design decisions and why
- **Shell-out only; no logic in the skill.** The skill frames the prompt and gates the output. All actual research goes through `sldo-research`. Rationale: avoid two implementations of the research pipeline. The Rust crate already handles M6/M7 synthesis + plan-readiness; duplicating that in prose in a SKILL.md would drift.
- **Quality gates on the dossier.** ≥3 sourced competitor comparisons, ≥1 technical prior art, every claim cited. If the pipeline can't produce these, the dossier is marked `incomplete: true` instead of passing silently. Rationale: downstream skills (`/slo-architect`, `/slo-plan`) use the dossier as input — they need a reliable signal that "we don't know enough yet."
- **Explicit non-overlap with get-api-docs.** The skill body says: this does market/competitor research, not third-party library API reference. Rationale: without this, Claude Code would plausibly use `/slo-research` to fetch OpenAI SDK docs, which is the wrong tool.

## Mistakes made
- None — the skill composition with `sldo-research` was straightforward.

## Root causes
- N/A.

## What was harder than expected
- Writing the "gates" section without making it sound like a checklist the skill mechanically ticks off. The point is to refuse to hand off a low-quality dossier, not to produce exactly three competitors regardless of what the market looks like. Landed on "if any of these is missing, set `incomplete: true`" — honest signaling.

## Naming conventions established
- Research outputs live at `docs/research/<slug>/{dossier.md, sources.md, synthesis.md}`. Fixed three-file shape, not one-file.
- Synthesis file's last sentence of every paragraph: "the design must handle X because [source]." If that sentence can't be written, the finding belongs in open-questions instead.

## Test patterns that worked well
- Static-contract tests (grep the skill body for required phrases) are a cheap way to prevent regressions when someone rewrites a skill body. They're not substitutes for runtime tests, but they catch "someone removed the chub delegation section" type bugs.

## Missing tests that should exist now
- Runtime: given an idea doc, does the skill produce a dossier that meets the quality gates? Blocked on Claude-Code-non-interactive harness.
- A dry-run that lets the user preview the shell command that will be dispatched to `sldo-research` before running it.

## Rules for the next milestone (M4 — /slo-architect + /slo-plan)
- `/slo-architect` must set `tla_required: <bool>` in the design-overview frontmatter. `/slo-plan` then reads that flag to decide whether to route through `/slo-tla`. Don't duplicate the decision in both skills.
- `/slo-plan` MUST refuse to one-shot the whole runbook. This is the core discipline. The skill body is explicit; the tests (M4) should assert the refusal is in the skill body.
- Budget: M4 is two skills. Time-box each at ~150 lines of SKILL.md. If the body is growing longer, cut to the gate/method/anti-patterns shape and move examples to an `examples/` subfolder.

## Template improvements suggested
- The pattern of "refuse on gate failure; don't paper over" emerged again here. Consider making it a first-class block in the skill-authoring template: **Gates — refuse to proceed when**.
