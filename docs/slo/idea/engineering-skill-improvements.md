---
name: engineering-skill-improvements
created: 2026-04-27
status: ideation (preempted — derived from 2026-04-27 skill-pack review + issues #21, #22)
tla_required: false
---

# Engineering skill improvements

## The pain

The 2026-04-27 skill-pack review identified two classes of structural issues across the engineering-side skills (`/slo-sast`, `/slo-tla`, `/slo-plan`, `/slo-execute`, `/slo-verify`, `/slo-critique`, `/slo-research`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-second-opinion`, `get-api-docs`, `/slo-architect`, `/slo-retro`, `/slo-freeze`, `/slo-resume`):

- **SKILL.md monolithic length under context pressure**: [`/slo-sast` is 296 lines covering M1-M5 in one file](../../skills/slo-sast/SKILL.md); [`/slo-tla` is 323 lines](../../skills/slo-tla/SKILL.md); [`/slo-plan` is 132 lines with a 15-step milestone authoring sub-procedure](../../skills/slo-plan/SKILL.md). Under context-window pressure (which Claude Code routinely hits mid-runbook), an agent reads less of a long SKILL.md and skips key gates. A future invocation of `/slo-sast` for an M3 emission still reads the M1 parser scaffolding + M2 stack detect + M4 manifest + M5 PR creation — all competing for attention.
- **Cross-skill drift on common patterns**: every SKILL.md repeats its own version of intake, restate-and-confirm, citation discipline, tool-safety boilerplate, output frontmatter. There is no `references/templates/` shared library, so when one skill's pattern improves, the others don't inherit. Specific symptoms: `/slo-execute`'s allow-list rule is prose-level (LLM compliance), not filesystem-enforced; `/slo-freeze`'s scope lock is the same; `/slo-second-opinion` shows raw provider output without a "neither response is verified" header; `get-api-docs` has no `chub` failure-mode handling.

The project owner explicitly flagged a third concern: **deep, source-verified citations for security-engineering content**. Any recommendation touching SAST / DAST / supply-chain / threat-model content must cite primary authoritative sources (tool docs at pinned version), not paraphrased commentary. This is the **research-validation prereq** that gates every security-touching milestone in this runbook.

## Five capabilities the user described without realizing

- A `references/templates/` shared library with the eight common patterns (intake-checklist, restate-and-confirm, citation-discipline, tool-safety-section, output-frontmatter, escalation, eval-cases, heuristic-numbers-discipline).
- `/slo-sast` decomposed into a thin orchestrator + per-stage methodology references (`methodology-m1-parser.md` through `methodology-m5-pr-creation.md`); `/slo-tla` similarly split (`methodology-elicitation.md`, `methodology-abstraction.md`, `methodology-counterexample.md`, `methodology-verified-design.md`); `/slo-plan` per-milestone authoring extracted.
- A per-skill `evals/` directory with seven canonical case shapes (happy / missing-context / ambiguous / adversarial / outdated / tool-failure / high-risk).
- A `settings.json` PreToolUse hook that hard-enforces `/slo-freeze`'s scope lock and (optionally) `/slo-execute`'s milestone allow-list at the filesystem layer, with the existing prose-level pattern as documentation.
- A research-validation discipline that, before any security-engineering claim lands, validates the citation against a strict source hierarchy (tool's own docs at pinned version → tool repo `README` / `CHANGELOG` at pinned commit → upstream advisory DB docs → conference talks / academic papers → vendor blog posts (secondary) → never Stack Overflow).

## Top risks

- **Breach**: low — decomposition is internal restructuring; no new attack surface. The PreToolUse hook for `/slo-freeze` introduces a settings.json mutation surface; risk is in the hook script itself (must use argv-list discipline if it shells out). Reuse the `update-config` skill for the mutation.
- **Compliance fine**: not applicable — no personal data, no regulatory surface.
- **Prolonged outage**: medium — if a SKILL.md decomposition breaks the install symlink chain in [`crates/sldo-install`](../../crates/sldo-install), the skill becomes unusable. Mitigation: structural-contract tests assert every SKILL.md still references its decomposed methodology files, and `cargo test --workspace` is the green-baseline gate.

## Approach A — conservative (recommended)

- **Effort**: 12 person-days (the per-skill `evals/` write-up is the long pole; decomposition is mechanical once the templates land).
- **Wedge**: M1 = research-validation prereq + `references/templates/` library seeded. Once the templates exist, every downstream milestone is "extract content from SKILL.md, refactor into the template-shaped reference" — a clean, mechanical pattern.
- **Risks**: `evals/` writing is high-bandwidth thinking work; resist the temptation to copy-paste eval cases between skills.

## Approach B — cloud / SaaS

Not applicable.

## Approach C — local / desktop

Not applicable.

## Recommendation

Approach A. 5 milestones, ordered so the foundational work (templates + research-validation discipline) lands first, decomposition lands second (sast + tla + plan), and the cross-skill polish + evals + hooks ship in M4-M5. Detailed milestone breakdown in [`docs/slo/future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md`](../future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md).

The research-validation discipline (M0-style prereq baked into M1) is **load-bearing** per project owner's explicit concern. Every claim that touches security-engineering content gets validated through the source hierarchy before SKILL.md edits land. Unverifiable claims are removed, not weakened.

## Open questions for /slo-research

(Most of these are resolved by reading the existing skill-pack review + issues #21/#22; flagged here for completeness so a future contributor can re-research if claims age.)

1. Have the existing references/sast/ and references/sast/scanner-orch-* files been source-verified against the upstream Semgrep CLI docs at the pinned version? Spot-check needed before M2 ships.
2. What is the canonical FNV-1a-64 invariant re-capture pattern? Issue #4's deferred-follow-ups list mentions automation; verify against the existing M4 lessons file before designing the soft line-cap structural-contract test.
3. Is the current `~~~text` user-string fence rule in `/slo-architect` documented as a Markdown rendering convention, or as a defense-in-depth? Citation needed for shared template's `citation-discipline.md`.
4. Apalache's pinned-SHA distribution channel — does it use the same SHA-256 manifest file pattern as `tla2tools.jar`, or release-asset checksums? Verify against [https://github.com/apalache-mc/apalache/releases/latest](https://github.com/apalache-mc/apalache/releases/latest) at runbook-author time.
