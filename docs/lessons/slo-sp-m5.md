# Lessons Learned — slo-sp Milestone 5

## What changed
- `/slo-tla` skill authored with the full JVM-detect → jar-fetch-and-verify cascade.
- `skills/slo-tla/tools.toml` pins TLC 1.8.0 and Apalache 0.44.11 with `sha256 = "UNSET"` placeholders (see below).
- `skills/slo-tla/templates/basic-state-machine.tla.tmpl` — starter template for the "grow the spec incrementally" method.
- `skills/slo-tla/counterexample-translator.md` — methodology the skill follows when turning TLC traces into plain-English findings.
- 11 E2E contract tests covering the JVM cascade, checksum step, bounds-required, fairness-required, Apalache hint, cache path, tools.toml shape, template shape, counterexample-translator doc.

## Design decisions and why
- **SHA-256 ships as `UNSET`.** Deliberate. We do not commit a hash we have not computed ourselves. The first maintainer to use the skill must `curl -fL <url> | shasum -a 256`, cross-check against upstream's `.sha256` sibling file if any, and replace the placeholder in a dedicated commit. The tools.toml includes a comment explaining this. Rationale: committing a hash found elsewhere encodes trust in a chain we didn't verify; committing `UNSET` makes the gap visible.
- **No bundled jar.** Per catalog Q4. Download on first use into `~/.sldo/tla/`.
- **TLC-first, Apalache lazy.** The skill doesn't check Apalache at startup. It only surfaces Apalache as a hint when TLC reports state explosion. Rationale: most users never need Apalache; pre-fetching it would be dead weight.
- **Counterexample translation is its own file.** Moved the methodology out of the SKILL.md so the skill body stays focused. Keeps the SKILL.md at ~200 lines instead of 350. Skill body references the methodology file; Claude reads both when the skill is invoked.
- **Template is a starter, not a generator.** The template is a textbook example (bounded counters moving from "start" to "done"). The skill explicitly tells Claude to grow real specs incrementally, not fill in the template. Rationale: templates that pretend to be generators produce specs that match the template but not the real design.

## Mistakes made
- First draft of the JVM cascade tried to install Java via brew as a fallback. Removed — per the `preflight` pattern, we tell the user and exit. Installing someone's JVM for them is too invasive.
- Initial `tools.toml` shipped with a plausible-looking SHA copied from a release note. Reverted to `UNSET` with a comment explaining why. A SHA you haven't verified is worse than no SHA.

## Root causes
- Optimizing for convenience over safety. Reminded myself: the cost of a wrong SHA or a silent-install is much higher than the cost of one extra manual step.

## What was harder than expected
- Specifying the counterexample translator in a way that doesn't degenerate into "be smart about it." Landed on the four-step method (annotate → find fork → name broken assumption → propose design-level fix) with explicit anti-patterns. The anti-patterns section is where the real constraints live.
- The fairness-assumption forcing function. Users conflate "eventually" with fairness. The skill body now forces an explicit weak/strong + action choice before writing the liveness property.

## Naming conventions established
- Spec artifacts: `specs/<name>.tla`, `specs/<name>.cfg`, `specs/<name>.trace.md`.
- Verified design doc: `docs/design/<name>-verified.md`.
- Cache: `~/.sldo/tla/tla2tools.jar` + `~/.sldo/tla/VERSION` + `~/.sldo/tla/tlc` shim.

## Test patterns that worked well
- Tests that grep for "refuse when bounds are missing" language enforce a discipline that would otherwise only exist in prose.
- Testing the template file's structure (MODULE, VARIABLES, Init, Next) catches accidental corruption without executing TLC.

## Missing tests that should exist now
- A TLC smoke test against the basic-state-machine template that runs when Java + tla2tools.jar are both present. Gated behind `#[ignore]` currently (not yet written — should add before M9 self-hosting).
- A checksum-mismatch simulation test. Writing one requires mocking curl/sha256 — deferred.
- An Apalache state-explosion simulation. Requires a spec that explodes predictably. Deferred.

## Rules for the next milestone (M6 — /slo-critique)
- `/slo-critique` is four sub-personas. Keep them as separate SKILL.md files under `skills/slo-critique/`? Or one parent skill with a methodology-switching mechanism? Either works; lean toward separate sub-skills so each persona's prompt can evolve independently.
- Each finding from every sub-persona must include a concrete exploit/failure scenario. Theoretical findings should be rejected. The tests should grep for this discipline.
- Do NOT modify `/slo-tla` in M6 unless critique finds a structural issue. If findings are "eh, this section could be clearer," park them in the lessons file.

## Template improvements suggested
- Consider a standard section for "Cache and artifacts" in the skill-authoring template. `/slo-tla` has a well-defined cache; other skills with filesystem state would benefit from the same structure.
- The "prereq cascade" pattern (which X → fail loud → try Y → if Y works, download/install Z) should be extractable into a reusable skill-authoring primitive. Future skill maintainers shouldn't have to re-invent it.
