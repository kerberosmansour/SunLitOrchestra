# Lessons Learned — slo-sp Milestone 6

## What changed
- `/slo-critique` orchestrator skill authored at `skills/slo-critique/SKILL.md`.
- Four persona methodology files under `skills/slo-critique/personas/` — `ceo.md`, `eng.md`, `security.md`, `design.md`.
- 8 E2E contract tests.

## Design decisions and why
- **One parent skill + four persona files.** Considered four sub-skills (`slo-critique-ceo`, etc.) but rejected: the personas always run together and the orchestration is cohesive. Users should invoke `/slo-critique`, not four separate things.
- **Rotation, not blend.** Each persona reads the whole runbook with their lens uncontaminated by the others. The SKILL.md enforces this explicitly.
- **Design persona auto-skips when no UI.** The skill surfaces one line saying "N/A — no UI surface" and moves on. Rationale: forcing design review on a pure-backend runbook produces noise.
- **Concrete scenarios gate every finding.** Theoretical findings are rejected. Every accepted finding has: attacker/actor, three-to-five-sentence step-by-step, and impact. Rationale: noise is the failure mode of automated review.
- **Confidence gate at 8/10.** Inspired by gstack's `/cso`. Low-confidence findings train the user to ignore the critique, which defeats it.

## Mistakes made
- First test for persona-ordering used `lower.find("security")` on the whole body. That matched "security" inside the opening paragraph's parenthetical list of personas, which placed security BEFORE "eng lead" in the description. Fixed by searching for bold-marker list entries (`**CEO**`, `**Eng lead**`, etc.) that only appear in the rotation list.

## Root causes
- Assuming "the first occurrence of `<persona name>` is in the rotation list" — but the description paragraph mentions all four personas in prose order. Grep tests need tighter anchors when the term appears in multiple contexts.

## What was harder than expected
- The security persona. It's tempting to enumerate OWASP Top 10 as a checklist. The right pattern is: for each category, is there a concrete surface in the plan that this applies to? If not, no finding. OWASP is a lens, not a checklist.
- Keeping the design persona from becoming "my taste is the standard." The final version grounds in AI-slop detection and interaction gaps — things that are actually observable — not preferences.

## Naming conventions established
- Persona methodology files: `skills/<parent-skill>/personas/<persona-name>.md`.
- Critique output: `docs/slo/critique/<runbook-slug>.md`, one row per finding, columns: `id | persona | category | runbook section | finding | concrete scenario | recommendation`.

## Test patterns that worked well
- Searching for Markdown bold markers (`**Name**`) as anchors when the same word appears in multiple contexts.
- Testing that persona-specific methodology files include their signature elements (CEO: four modes, eng: assumptions + failure modes, security: OWASP + STRIDE + exploit language, design: no-UI-skip).

## Missing tests that should exist now
- A runbook with planted bugs (theoretical race condition claim, missing empty state, unstated at-most-once assumption, inferred scope creep) that each persona should catch a specific subset of. Useful as a regression suite. Deferred until runtime harness exists.
- A test that `/slo-critique` writes a `docs/slo/critique/*.md` file with the required column headers.

## Rules for the next milestone (M7 — /slo-execute + /slo-verify)
- `/slo-execute` must enforce the milestone's file allow-list. This is the single most common failure mode of AI-driven runbook execution. Write the test that simulates an out-of-scope edit and confirms the skill body says to pause and surface it, not widen silently.
- `/slo-verify` wraps Playwright. Keep the Playwright integration as a shell-out in the skill body; do not write a Rust wrapper.
- For each bug `/slo-verify` finds, a regression test must be added in the same commit as the fix — not after. The skill body should call this out.

## Template improvements suggested
- Add a "Concrete scenario requirement" section to the skill-authoring template for any skill that produces findings. The pattern applies beyond just `/slo-critique`.
