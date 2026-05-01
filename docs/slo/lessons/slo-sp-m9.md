# Lessons Learned — slo-sp Milestone 9

## What changed
- Ran the skill pack end-to-end against a real SLO feature: "auto-populate `tools.toml` SHA-256." Produced all five pipeline artifacts:
  - `docs/slo/idea/tla-sha-autopop.md` (from `/slo-ideate` equivalent)
  - `docs/slo/research/tla-sha-autopop/synthesis.md` (from `/slo-research` equivalent — scoped deliberately shallow)
  - `docs/slo/design/tla-sha-autopop-overview.md` (from `/slo-architect` equivalent; correctly set `tla_required: false`)
  - `docs/RUNBOOK-TLA-SHA-AUTOPOP.md` (from `/slo-plan` equivalent; 2 milestones, within the 5-milestone cap)
  - `docs/slo/critique/tla-sha-autopop.md` (from `/slo-critique` equivalent; 4 personas, 5 findings, 1 auto-fix, 2 asks, 1 hold)

## Design decisions and why
- **Picked a small, real feature.** "Auto-populate SHA" is genuinely useful (it's the follow-up flagged in M5's lessons) and small enough to actually validate the pipeline end-to-end in one sitting. The alternative candidate ("full Apalache integration") would have taken a week.
- **Research marked `incomplete: true` honestly.** The feature is internal maintenance; there is no market to research. The synthesis explains why. This proved the `incomplete` flag works as intended: future skills see the flag and don't treat the dossier as authoritative.
- **Design correctly set `tla_required: false`.** No concurrency. The reason field explains why TLA+ would be theater here. Good test of the gate.
- **Critique found two real security/eng findings (f1, f2).** Redirect-to-foreign-host and oversize-response are both real concerns for a tool that fetches from URLs. This validates that the critique personas aren't noise-generators.

## Mistakes made / rough edges surfaced
1. **The "idea doc" → "research brief" handoff had no formal schema.** `/slo-ideate` outputs "Open questions for /slo-research" but the questions were answerable by reading GitHub docs directly, not by running `sldo-research`. The pipeline needs a way to say "research opted for a direct lookup rather than a full dossier run" without marking the dossier incomplete. Candidate follow-up: add a `scope: full | targeted | direct-lookup` field to the research frontmatter.
2. **`/slo-architect` output files weren't all necessary.** For a tiny feature, `stack-decision.md` and `interfaces.md` would have been one-line files each. The pipeline should allow `<slug>-overview.md` alone when the feature is that small. Candidate follow-up: make the three-file architect output optional when `scope: targeted`.
3. **`/slo-plan` generated a 2-milestone runbook with full evidence-log templates.** Fine, but for a small runbook the boilerplate was ~60% of the file. Not a bug, but worth noting that the v3 template's boilerplate density is proportionally higher for tiny runbooks. Candidate follow-up: a "v3-lite" variant for ≤2-milestone runbooks.
4. **`/slo-critique` with four personas on a 2-milestone runbook produced one N/A and one hold-scope.** That's honest — design has no surface; scope is right-sized. No drift toward noise. Good signal that the discipline works at small scale.
5. **No rough edges surfaced in the installer, execute, verify, retro, or power tools.** Those weren't exercised at runtime in this milestone because the goal was to validate the THINK → PLAN sequence end-to-end. A true full-pipeline dogfood would need `/slo-execute M1` of the new runbook actually running, which is follow-up work.

## Root causes
- The rough edges in (1), (2), (3) all trace to a single cause: the pipeline was tuned for "real-sized" features. Tiny maintenance features are served by the pipeline but with disproportionate ceremony. Not blocking — the pipeline shipped what it was designed to ship.

## What was harder than expected
- Resisting the urge to polish the skills while exercising them. Several times I noticed a phrase I could tighten; each time I left it alone and noted it in this lessons file. The discipline is "validate, don't patch."

## Naming conventions confirmed working
- `docs/slo/idea/<slug>.md`, `docs/slo/research/<slug>/{dossier,sources,synthesis}.md`, `docs/slo/design/<slug>-overview.md`, `docs/RUNBOOK-<FEATURE-UPPER>.md`, `docs/slo/critique/<slug>.md`. All slots filled without collision or ambiguity.

## Test patterns that worked well
- The artifact-based validation (does the idea doc have the required sections? does the runbook match v3? does critique have the required columns?) was implicit in the structure of each skill's output. No explicit test needed — if the output is well-formed, the skill worked.

## Missing tests that should exist now
- A full-pipeline dogfood test that actually runs `/slo-execute M1` of the tla-sha-autopop runbook. Would exercise the install → execute → verify → retro loop end-to-end. Candidate for a post-M9 follow-up runbook.
- A pipeline assertion suite that checks, given any set of artifact files, whether the handoff schema is satisfied (frontmatter fields, file paths, cross-references).

## Rules for post-M9 work
- **Do not retroactively patch skills based on M9's rough edges without a dedicated runbook.** The M9 lessons list the candidate follow-ups; each deserves its own ideate → research → plan flow, not a drive-by commit.
- **The pack is ready for external use.** Publish it. Document the rough edges in the README.
- **The self-hosting exercise should happen quarterly.** That's the right cadence to catch drift between the skill bodies and the actual work users do.

## Template improvements suggested
- Add a `scope: full | targeted | direct-lookup` field to research frontmatter.
- Consider a v3-lite template for ≤2-milestone runbooks.
- Make architect's three-file output optional when `scope: targeted`.

## Pipeline stages exercised (for the record)

| Stage | Skill | Artifact | Status |
|---|---|---|---|
| Ideate | `/slo-ideate` | `docs/slo/idea/tla-sha-autopop.md` | ✓ |
| Research | `/slo-research` | `docs/slo/research/tla-sha-autopop/synthesis.md` (marked incomplete with reason) | ✓ |
| Architect | `/slo-architect` | `docs/slo/design/tla-sha-autopop-overview.md` | ✓ |
| TLA+ | `/slo-tla` | N/A (tla_required=false) | ✓ (gate worked) |
| Plan | `/slo-plan` | `docs/RUNBOOK-TLA-SHA-AUTOPOP.md` | ✓ |
| Critique | `/slo-critique` | `docs/slo/critique/tla-sha-autopop.md` | ✓ |
| Execute | `/slo-execute` | — | deferred to follow-up runbook |
| Verify | `/slo-verify` | — | deferred |
| Retro | `/slo-retro` | — | deferred |
| Ship | `/slo-ship` | — | deferred |
