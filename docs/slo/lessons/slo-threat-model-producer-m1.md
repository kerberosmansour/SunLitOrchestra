---
filed_issues: []
note: >
  No issues filed by this retro. The single M1 lesson is intra-runbook
  process discipline (carried forward to M2 rules below), below the standalone
  tracked-issue bar; and the cumulative "loose-Markdown-assertion" pattern
  recurs across `slo-threat-model-m1/m2` lessons + this one — a candidate for
  a future skill-pack refinement, captured under "Template improvements
  suggested" rather than re-filed.
---

# Lessons Learned — slo-threat-model-producer Milestone 1

## What changed

- Appended **item 8** to `skills/slo-architect/SKILL.md` Step 3.5: the
  producer contract for also emitting `docs/slo/design/<slug>-threat-model.slo.json`
  conforming to the merged schema, with the provenance idiom and
  supersede-don't-renumber rule, and the SEC-1 producer-side neutralisation
  (structural serializer; user idea-doc text never chooses
  `id`/`classification`/`accepted_residual`/`status`).
- Appended a `## SLO JSON companion serialization mapping` section to
  `skills/slo-architect/references/threat-model-template.md` with the
  Markdown→JSON mapping table and a duplicate SEC-1 paragraph.
- New `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` (6 tests after the
  verify-time regression addition): producer-contract presence; append-only
  anchors intact; ENG-1 structural proxy (template mapping forbids renumber +
  merged fixture ids unchanged); SEC-1 (loose) presence; SEC-1
  (regression-tight, per-file regex) bound to the actual phrase.

## Design decisions and why

- **Append item 8 to the numbered Step 3.5 list rather than rewriting it.**
  Recon (M1 lesson "rg every reader") found `e2e_slo_sec_m1:42` requires the
  `\n7. **` anchor and three other substring guards across Step 3.5 +
  threat-model-template.md. Append-only is the only safe shape; the new item 8
  satisfies the producer contract without disturbing items 1–7.
- **Document SEC-1 redundantly in BOTH SKILL.md and the template.** Belt and
  braces: a partial weakening in one file doesn't fully break SEC-1 prose
  enforcement. (Tradeoff: the per-file test below is needed to catch a
  partial weakening — verify caught this.)
- **ENG-1 structural proxy, not a "proves idempotent emit" overclaim.** No
  producer code runs locally; the test asserts the template mapping is 1:1 +
  forbids renumber, and the merged fixture still conforms with ids
  `tm-slo-sec-abuse-1..8` contiguous. Honest scope.

## Mistakes made

- **My SEC-1 assertion was too loose.** The first version checked
  `lc.contains("never") && lc.contains("idea-doc")` — both extremely common
  words in the SKILL.md, so a real weakening of the SEC-1 phrase
  (`**never** chooses` → `MAYBE chooses`) did NOT fail the test. Caught by
  the mutate-force-restore verification (G3). Recurrence of the prior-lesson
  pattern "a guard must bind the invariant, not the prose around it."
- **First regression attempt also wrong.** The tightened regex initially
  checked the *union* `skill+template` — and since SEC-1 is duplicated in the
  template, a SKILL.md-only weakening still matched in the template. Had to
  iterate to a **per-file** check.

## Root causes

- Markdown structural-test assertions are recurringly tempting to write as
  loose substring conjunctions because they're easy. The discipline that
  catches this is: **for every prose assertion, design the mutation that
  should break it, and verify it actually does** (run the mutation, observe
  failure). The wedge M1 verify pattern (mutate-force-restore) is the right
  discipline; M1 producer execute should have applied it inside execute too,
  not waited for verify.

## What was harder than expected

- The first-iteration regression test (union-of-files) was a *false fix* that
  passed the good state but did not bite the actual weakening. Two iterations
  to get the test correct. Cost: one extra mutate-restore cycle.

## Naming conventions established

- Step 3.5 producer-contract item: numbered `8.` (one above the existing `7.`
  re-run item), append-only, never renumber items 1–7.
- Template serialization-mapping section: `## SLO JSON companion serialization mapping`
  (Markdown→JSON mapping table + a SEC-1 paragraph).
- Producer-runbook test prefix: `slo_tmp_m<N>_<feature>` — distinct from the
  wedge's `slo_tm_m<N>_<feature>` (collision-free).
- SEC-1 regression idiom: per-file regex
  `\bnever\b[^\n]{0,40}\bchooses\b[^\n]{0,120}\b(id|classification|accepted_residual|status)\b`,
  iterated over each file independently.

## Test patterns that worked well

- **Per-file iteration of the SEC-1 regex** caught the partial weakening that
  the union check missed. Reusable shape for any future redundantly-stated
  invariant.
- **The mutate-force-restore verification pattern from the wedge** caught my
  loose assertion. Continuing it pays dividends.
- **Coarse + tight assertions side-by-side** (the loose
  `producer_prose_mandates_sec1_neutralisation` stayed; the tight
  `sec1_clause_is_specifically_bound` was added) — additive, double-net.

## Missing tests that should exist now

- The supersession algorithm (re-run: detect-existing + diff +
  supersede-don't-renumber + no silent clobber) is not asserted by M1; that
  is M2's whole purpose. Not an M1 gap.
- A live agent-runtime emission test (a real `/slo-architect` run producing
  a fresh `.slo.json` from inputs) does not exist — same accepted limitation
  as the merged wedge (no deterministic local agent harness). Not an M1 gap.

## Rules for the next milestone (M2)

- **For every new prose assertion in `slo_tmp_m2_rerun`, design the mutation
  that should break it and confirm at execute time (not verify time) that the
  mutation fails the assertion.** Apply the mutate-force-restore pattern
  inside execute's BDD-first step, not only in verify.
- **Per-file binding by default** when an invariant is documented in more
  than one file; never assert on the union without a per-file follow-up.
- **ENG-2 is critical for M2:** `slo_tmp_m2_rerun` MUST own the demo-fixture
  strict-parse with its own serde structs — `slo_tm_m1_schema` only loads
  its hardcoded original fixture (verified at recon time, locked in the
  runbook). The demo fixture's superseded-row invariants must be checked by
  this milestone's test, not by the merged wedge test.
- Continue append-only edits to `slo-architect/SKILL.md`; preserve the M1
  item-8 anchor and the four `e2e_*` guard substrings.

## Template improvements suggested

- A reusable helper for structural-test assertions over Markdown prose: e.g.,
  a `assert_phrase_in_each(files, regex, label)` utility that iterates a list
  of files and asserts the regex matches in each, with a clean failure
  message. Would have made the M1 SEC-1 regression a one-liner instead of an
  iterate-and-fix cycle. Below the standalone tracked-issue bar; recorded as
  a candidate refinement.
