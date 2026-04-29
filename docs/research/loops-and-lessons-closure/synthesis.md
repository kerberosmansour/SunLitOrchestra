---
name: loops-and-lessons-closure
researched: 2026-04-27
incomplete: false
note: |
  Skill-pack improvement runbook — research is largely the existing skill-pack review +
  GitHub issues #16, #17, #18 + project owner's chat-elicited decisions on those issues.
  This file is the synthesis only; the equivalent of `dossier.md` is the issue threads.
---

# Synthesis — loops + lessons closure

## What the design must handle (and why)

### 1. The lessons loop must be the FIRST loop documented

The design must handle the lessons-loop as the foundational loop because **R2 (engineering) and R3 (business) will produce lessons that need filing as issues**. Without R1 closing the lessons loop first, R2 and R3's lessons drop into `docs/lessons/<prefix>-m<N>.md` and never get filed upstream — defeating the project owner's stated motivation in [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16): *"Lessons only matter if they change behavior. Today they sit in docs/lessons/ and rarely get re-read."*

### 2. `gh search` is the de-dupe mechanism — but its reliability depends on the marker

The design must handle marker selection (title prefix vs label vs body sentinel) because [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) explicitly leaves it as an open question: *"Marker for retro-originated issues — title prefix, label, or body sentinel? Whichever is most reliable for `gh search`."* The runbook M3 must include a spike step that tests each option against a populated issue set.

### 3. Engineering loops + business loops must be split docs, not blended

The design must handle this split because [issue #17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17) and [issue #18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) explicitly stated the decision: *"Split from business loops — separate doc, separate concerns."* Blending them is a known anti-pattern.

### 4. Loop docs must cross-link from ARCHITECTURE.md and SKILL.md, not from a separate index

The design must handle bidirectional cross-linking because both [issue #17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17) and [issue #18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) state: *"Cross-link from ARCHITECTURE.md and each implicated SKILL.md."* A standalone index without cross-links is a dead-link risk.

### 5. The fallback for repos without an issue tracker is a local file

The design must handle the no-tracker case because [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) explicitly states: *"Fallback when no issue tracker is configured: local LESSONS-BACKLOG.md in the target repo."* Without this, the retro skill becomes a hard dependency on `gh` availability — which violates the principle of graceful degradation observed across the rest of the pack.

### 6. Loop closure at milestone start (not at retro) is the value-creating move

The design must handle this because [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) says: *"Close the loop in execution flow. /slo-execute M<N> (or the runbook template itself) checks open issues from prior milestones at start of each milestone, surfaces them as scope candidates."* The retro filing alone produces issues but doesn't change behavior; the milestone-start surfacing is what closes the loop.

### 7. The runbook template change is in scope

The design must handle this because [issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) flags it as an open question (*"Where does the runbook template change live — its own milestone in a future runbook, or fold into this issue's scope?"*) — folding into this runbook is the cleaner choice, since the template change IS the artifact-level visibility of the loop closure.

## Open questions that research did not answer

These propagate to the runbook as M3-or-later spike steps:

1. **Best marker for retro-originated issues**: needs hands-on `gh search` testing on a populated set.
2. **Mapping upstream tools to issue-tracker repos**: explicit `.sldo/upstream-mapping.toml` vs crates.io / npm registry resolution. Lean: explicit mapping with registry fallback (per the idea doc).
3. **Loop diagram format** ([#17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17), [#18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) Q2): ASCII vs Mermaid. Lean: match the closest existing design doc — likely `docs/design/scanner-orchestration-overview.md` — at runbook-author time.

## Source pointers

- [Issue #16](https://github.com/kerberosmansour/SunLitOrchestrate/issues/16) — `/slo-retro` extension to file lessons + close loop at milestone start
- [Issue #17](https://github.com/kerberosmansour/SunLitOrchestrate/issues/17) — `LOOPS-ENGINEERING.md`
- [Issue #18](https://github.com/kerberosmansour/SunLitOrchestrate/issues/18) — `LOOPS-BUSINESS.md`
- [`skills/slo-retro/SKILL.md`](../../../skills/slo-retro/SKILL.md) — current behavior to extend
- [`skills/slo-execute/SKILL.md`](../../../skills/slo-execute/SKILL.md) — current pre-flight to extend
- [`docs/runbook-template_v_3_template.md`](../../runbook-template_v_3_template.md) — template needing the new "Carry-forward from prior retros" section

## Note on chub / get-api-docs

Not applicable — this work doesn't introduce new API integrations; `gh` CLI is the only external tool and it's already documented in the existing skill prose.
