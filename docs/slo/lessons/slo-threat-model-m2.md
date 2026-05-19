---
filed_issues:
  - title: "Producer: /slo-architect Step 3.5 emits <slug>-threat-model.slo.json (CEO-1)"
    classification: slo-process
    destination: kerberosmansour/SunLitOrchestra
    dedupe: none
    disposition: spilled-harness-permission-block
    backlog_row: "LESSONS-BACKLOG.md Row 1"
  - title: "Enforce gitignore/redaction for confidential threat-model .slo.json (SEC-2 residual)"
    classification: slo-process
    destination: kerberosmansour/SunLitOrchestra
    dedupe: none
    disposition: spilled-harness-permission-block
    backlog_row: "LESSONS-BACKLOG.md Row 2"
  - target: "issue #67 progress comment"
    disposition: spilled-harness-permission-block
    backlog_row: "LESSONS-BACKLOG.md Row 3"
note: >
  Filing was user-approved at retro but the Claude Code harness permission
  classifier blocked the gh external write. Durable record is LESSONS-BACKLOG.md;
  issue filing is strictly additive and the on-disk closeout artifacts are
  complete.
---

# Lessons Learned — slo-threat-model Milestone 2

## What changed

- `skills/slo-verify/SKILL.md` and `skills/slo-critique/SKILL.md` each gained
  an additive `## Threat-model read-side contract` section: read the frozen
  `<slug>-threat-model.slo.json`, halt rather than re-derive `tm-<slug>-abuse-N`
  IDs, `accepted_residual` ≠ missing coverage, the SEC-1 `~~~text`
  literal-fence rule, and the degraded-mode-vs-hard-halt boundary.
- `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` `CRITIQUE_SKILL_SHA256`
  updated in lockstep with the edit, with the F-ENG-6 amendment recorded in
  the constant's doc-comment and runbook §6.
- New `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs` (5 tests) binding the
  read-side contract, the SEC-1 fence rule, the ENG-1 additive anchors, and
  the F-ENG-6 lockstep from the M2 side.

## Design decisions and why

- **Append-only edits to both SKILL.md.** The canonical portable
  critique/verify path must not be reflowed (ENG-1); appending a single new
  section before the Loops footer preserved every existing anchor and every
  substring `e2e_sec_exec_m3` asserts.
- **A second, M2-side F-ENG-6 lockstep test** (`feng6_sha_constant_in_lockstep`)
  in addition to `sap_imp_m5`'s own check — so the lockstep fails from *this*
  runbook's test surface too, not only the legacy one. Belt and braces on the
  highest-risk seam.
- **The read-side contract governs the dogfood case correctly**: slug
  `slo-threat-model` has no `.slo.json` (producer deferred), and the live
  `/slo-verify` run proceeded in degraded mode rather than blocking — the
  contract is internally coherent.

## Mistakes made

- None of consequence. The one risk (editing a file another structural test
  reads) was caught in pre-flight recon, not after the fact.

## Root causes

- N/A — the M1 "rg for other readers before editing" rule was followed and
  pre-empted the only latent failure (the `e2e_sec_exec_m3` slo-verify
  reader).

## What was harder than expected

- Nothing materially. The single-feature-branch rename (`-m1` →
  `slo/slo-threat-model`) was a one-liner; doing it at M2 pre-flight kept the
  eventual PR to one branch.

## Naming conventions established

- Consumer read-side contract section heading: `## Threat-model read-side
  contract (slo-threat-model M2)` — identical core block in both consumer
  SKILL.md files.
- M2 test: `xtasks/sast-verify/tests/slo_tm_m2_consumers.rs`.
- F-ENG-6 amendment idiom: update the constant + record prior baseline +
  amendment rationale in the constant's doc-comment AND the runbook section.

## Test patterns that worked well

- A test that **parses the pinned constant out of the other test file**
  (`sap_imp_m5_agents.rs`) by regex and compares it to the live SHA-256 —
  makes the lockstep self-checking from the new runbook's surface.
- Runtime verification by mutating tracked files in place, observing the guard
  fail, then restoring from a `/tmp` backup and asserting SHA-256 / `diff -q`
  equality. Same pattern as M1; reusable.

## Missing tests that should exist now

- An end-to-end agent-runtime test that a hostile `.slo.json` field is
  actually rendered inside a `~~~text` fence by a live `/slo-critique` /
  `/slo-verify` session. No deterministic local harness exists; covered
  structurally + by the live degraded-mode self-dogfood. This is the
  producer-runbook's natural home.

## Rules for the next milestone

- This is the final milestone of the runbook. The next step is `/slo-ship`,
  not `/slo-execute M3`.
- The producer runbook (CEO-1) and the redaction-enforcement residual (SEC-2)
  are filed as tracked follow-ups by this retro (see `filed_issues`); the
  next runbook that picks them up should start from `/slo-architect` (producer
  has real new surface) and treat SEC-2 as a security milestone within it.

## Template improvements suggested

- `/slo-execute` pre-flight could add an explicit "grep every test that
  `read()`s a file you're about to edit" sub-step — the M1→M2 lesson proved
  its value (caught `e2e_sec_exec_m3`). Captured here as a process nuance;
  filed as a `slo-process` candidate only if it recurs (single occurrence is
  below the standalone-issue bar; the rule is already written into this
  runbook's M1 lessons).
