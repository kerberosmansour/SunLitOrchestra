# Lessons Learned — outcome-first Milestone 4

## What changed
- The three back-end consumers now enforce the outcome contract: `/slo-retro` refuses to close a value-bearing milestone with an unproven Outcome Validation row + gained a `## Outcome vs promise` lessons section; `/slo-execute` Step 1 writes Outcome Scenario + Critical User Journey tests first; `/slo-critique` eng-lead flags outcome-test theatre as `ask` and security requires security-BDD rows to cite `tm-<slug>-abuse-N`. Added `outcome_first_m4_consumers.rs` (retro+execute fresh SHA pins + the critique single-source cross-check); bumped the single `slo-critique` constant.

## Results vs thesis
- `N/A — methodology/tooling milestone`. The enforcement is now end-to-end (plan → execute → verify → retro all reference the outcome contract); the gate-fires proof was the M3 theme-A dogfood.

## Outcome vs promise
- `N/A — not a product milestone`. The promised *methodology* outcome (a milestone can't close with an unproven user outcome) is now enforced at the close gate; demonstrated non-vacuously by the M3 dogfood.

## Design decisions and why
- **DW-003 — single source of truth, not two.** The plan/critique (ENG-4) assumed `slo-critique` was pinned by two constants. Reading `slo_tm_m2_consumers.rs` showed it DERIVES the SHA by regex from `sap_imp_m5_agents.rs`'s single constant. So M4 bumped only that one constant and left `slo_tm_m2_consumers.rs` untouched. The cross-check was corrected to assert single-source consistency + that slo_tm_m2 keeps deriving. The half-update risk doesn't exist — better than feared.
- **Additive edits only** — each of the three SKILL.md edits appended to existing sections (refusal-gate list, lessons template, Step 1, persona bullets), preserving every marker the reader tests assert.

## Mistakes made
- Initial M4 test + doc comment encoded the wrong (two-constant) mental model. Caught immediately when `critique_single_source_of_truth_consistent` failed pre-edit during debugging; corrected the test + doc comment to reality.

## Root causes
- The design code-map summarized the Explore agent's read ("F-ENG-6 in both tests") without distinguishing "stores a constant" from "derives the constant." Verifying against the actual test source corrected it. **Verify the mechanism, not the summary.**

## What was harder than expected
- Nothing material once the single-source mechanism was understood. Bumping one constant made both pin tests + the derive test green at once.

## Naming conventions established
- Single-source SHA pattern: one `*_SKILL_SHA256` constant; other tests derive via regex (`CRITIQUE_SKILL_SHA256:\s*&str\s*=\s*"([0-9a-f]{64})"`).

## Test patterns that worked well
- Cross-checking "the live SHA is in the source-of-truth file AND the derived test still references it" guards the tm-outcome-first-abuse-4 weaken-the-pin surface without assuming a second constant.

## Missing tests that should exist now
- None for M4.

## Rules for the next milestone
- **M5 is docs-only** (operating-contract + catalog + LOOPS) — no SKILL.md edits, so no SHA pin; assert principle-presence markers. Do NOT touch the catalog skill-count line or add a "Start here" loop row. Keep the principle host-neutral.
- When the plan describes a pin mechanism, **read the actual test source** before relying on the description.

## Template improvements suggested
- None.

---

## Detected Work Ledger disposition (this milestone)
- **DW-003** (single-source-of-truth correction) — `fix_now`, **done in M4**.
- **DW-002** (pre-existing clippy debt) — unchanged; `file_github_issue` at ship.
