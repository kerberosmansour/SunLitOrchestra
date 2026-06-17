# Lessons Learned — outcome-first Milestone 5

## What changed
- The **Outcome First Engineering** principle is now binding + discoverable: host-neutral section in `references/agent/operating-contract.md`, a gate note in `docs/skill-pack-catalog.md` (skill-count untouched), and a Sprint-loop **Outcome-First overlay** + inverted-authority pyramid in `docs/LOOPS-ENGINEERING.md` (Secure Value overlay preserved). Added `outcome_first_m5_principle.rs`.

## Results vs thesis
- **The runbook's §5A thesis is affirmed as far as it can be pre-real-feature-dogfood.** Leading metric (a runbook can author §5C + outcome sections, and a gate runs them): the v4 template + `/slo-plan` enforcement + `/slo-verify` Pass 0 are live, and the M3 theme-A dogfood **mechanically demonstrated the gate blocks an unproven milestone and passes a remediated one** — the strongest available evidence that the gate changes an outcome the old loop would have passed. Lagging metric (a real feature milestone blocked by the gate) awaits the next real value-bearing runbook authored against the new template.

## Outcome vs promise
- The promised *methodology* outcome — "a milestone can't close as done while the user outcome is unproven" — exists end-to-end (plan requires it → execute writes it first → verify Pass 0 runs it → retro refuses without it → principle binds it). Adjacent outcomes preserved: every prior `sast-verify` suite (34 total) is green; no existing skill behavior, template section, or pass number was removed/renumbered.

## Design decisions and why
- **Docs-only, marker-asserted, no SHA pin** — operating-contract/catalog/LOOPS are evolving docs; byte-pinning them would be over-rigid (unlike the slow-moving orchestration SKILL.md files).
- **Outcome-First overlay as a peer of the Secure Value overlay** — kept visibly distinct (security envelope vs outcome authority) so neither reads as subsuming the other.
- **Did not touch the catalog skill-count line or add a "Start here" loop row** — Outcome-First is an overlay on the Sprint loop, not a new loop.

## Mistakes made
- The host-neutral test initially scanned the whole operating-contract file, which legitimately names hosts in "Keep Host Boundaries Honest" — re-scoped to the principle section. Caught at first run.

## Root causes
- A whole-file "must not contain X" assertion is too broad when X legitimately appears elsewhere. Scope structural assertions to the section under test.

## What was harder than expected
- Nothing. Docs edits + marker assertions were straightforward.

## Naming conventions established
- Loop overlays are `### <Name> overlay` H3 subsections under the relevant `## <loop>` H2, with a stage→output table.

## Test patterns that worked well
- Section-scoped host-neutral assertion (find the heading, slice to the next `## `, assert within).

## Missing tests that should exist now
- None. The next real value-bearing runbook authored against the new v4 template will be the live end-to-end dogfood (the §5A lagging-metric read).

## Rules for the next milestone
- Runbook complete — next is `/slo-ship`. Surface DW-002 (pre-existing clippy debt) for user-confirmed filing at ship.

## Template improvements suggested
- None.

---

## Detected Work Ledger disposition (this milestone)
- **DW-002** (pre-existing clippy debt) — still open; `file_github_issue` (slo-process), surfaced for user-confirmed filing at ship. No new findings in M5.
