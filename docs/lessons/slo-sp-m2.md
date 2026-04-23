# Lessons Learned — slo-sp Milestone 2

## What changed
- Two new first-party skills authored: `/slo-ideate` (YC-office-hours interrogation) and `/slo-retro` (milestone close-out).
- One example idea doc at `skills/slo-ideate/examples/briefing-app.md`, demonstrating the expected output shape.
- New test file `crates/sldo-install/tests/e2e_slo_sp_m2.rs` validating frontmatter + installer pickup.

## Design decisions and why
- **Skill body over prompt tuning.** Each skill's `SKILL.md` reads like a role brief — method, gates, anti-patterns, handoff. No implicit assumptions about how a "good" Claude session would handle the task. Rationale: when Claude Code runs the skill, the body is the entire context; anything implicit is lost.
- **Hand-off clauses in every skill.** Every skill ends with "suggest the next step: `/slo-<next>`". Rationale: pipeline discoverability. A user who runs `/slo-ideate` and then asks "what's next?" should get a concrete answer without needing to re-read the catalog.
- **`/slo-ideate` refuses to produce on a vague pitch.** Rather than generating a low-quality idea doc, it interrogates until the pain is concrete. Rationale: downstream skills trust the idea doc. GIGO kills the pipeline.
- **`/slo-retro` refuses to run on an empty evidence log.** Same reason — lessons files are the input to the next milestone's execute; a lessons file built on missing evidence corrupts the next milestone.

## Mistakes made
- None that required rework. The skills were drafted in parallel while M1 baseline compiled, which caught a few inconsistencies (e.g., "`slo-retro`" vs "`/slo-retro`" — standardized on the slash-prefix form everywhere the skill is invoked).

## Root causes
- Not applicable — no incidents this milestone.

## What was harder than expected
- Calibrating specificity. The first draft of `/slo-ideate` tried to enumerate pain scenarios; that doesn't survive the diversity of real ideas. Moved to forcing questions the user answers, rather than templates the skill pre-fills.
- Making the handoff from `/slo-retro` to the NEXT milestone feel natural. A retro is naturally backward-looking; injecting "and now start M+1" risks rushing. Landed on "suggest the next step, do not auto-start" as the rule.

## Naming conventions established
- First-party skill directories use the `slo-` prefix (e.g., `slo-ideate`). When Claude Code exposes them as slash commands, they become `/slo-ideate`. Third-party vendored skills (M10) do NOT use the prefix — they keep their upstream name.
- Example artifacts go in `skills/<skill>/examples/`. The installer symlinks the whole skill dir, so examples are available to Claude at runtime.
- Lessons files reference the milestone prefix `slo-sp-m<N>` — matches the runbook metadata.

## Test patterns that worked well
- Minimal frontmatter validator that doesn't require a YAML parser. Checks for `---\n` start, `name:` field, `description:` field, closing `---`, and a non-trivial body. Catches shape bugs without pulling in `serde_yaml`.
- Copying individual skills into a tempdir for the install test keeps the test hermetic even as more skills are added to the repo.

## Missing tests that should exist now
- A golden-output test for `/slo-ideate` — given a seed pitch, does the skill produce a doc with all required sections? Defer until we have a harness that can invoke Claude Code non-interactively.
- A test that `/slo-retro` refuses on a blank evidence log. Same blocker — requires Claude Code invocation.

## Rules for the next milestone (M3 — /slo-research)
- `/slo-research` should shell out to `sldo-research` but NOT modify the Rust binary itself. If a prompt-shape change is needed, surface it in the lessons file and propose a separate sldo-research PR — do not widen M3 to touch Rust.
- Do not treat `chub`/`get-api-docs` as a competitor for `/slo-research`. They're complementary — research is market/competitor; get-api-docs is third-party API reference. Document this clearly in the `/slo-research` SKILL.md body (already done).
- Check at the start: is `sldo-research` on PATH? Use the existing `which::which` pattern from `preflight.rs`.

## Template improvements suggested
- The v3 skill-authoring pattern would benefit from an explicit "gates" section (when to refuse). Both skills in M2 grew this section organically. Consider making it a named block in the template alongside Method, Anti-patterns, Handoff.
