# Lessons Learned — svl Milestone 1

## What changed
- Added `docs/SECURE-VALUE-LOOP.md` — the canonical Secure Value Loop envelope definition (operating rule, per-stage security outputs, Operator Readiness Gate, Detected Work Ledger with the ledger↔`/slo-retro`-lane mapping, additive status vocabulary + unknown→blocked fail-safe, Bundle A–F table cited by name+edition, the `~~~text` fence rule scoped to named generation surfaces, the one-page agent prompt, adoption criteria).
- Inserted v4-template **§5B Secure Value and Security Contract** (Value Wedge, Security Definition of Ready, Threat Model Summary, Security Test Plan, Detected Work Ledger) into BOTH template copies, byte-identical.
- Tightened the Contract Block proactive-controls row from bare numbers ("e.g., C1, C5, C9") to OWASP Proactive Controls **2024 by name**.
- Added `xtasks/sast-verify/tests/svl_m1.rs` (8 structural assertions).

## Results vs thesis
- N/A — not a value-bearing milestone (tooling/process; §5A is `N/A`). Adoption is measured against the proposal §11 criteria, tracked across milestones.

## Design decisions and why
- **Letter-suffixed section (`## 5B.`) between §5A and §6** — avoids renumbering §6/§10/§17, which the existing `mloop_m3_plan.rs` and `kani_m3_integration.rs` tests pin. The §5A Measurement Contract insertion was the proven precedent.
- **Edit primary copy, then `cp` to mirror** — the most reliable way to guarantee byte-identity; a manual second edit risks invisible whitespace drift. `diff` is the confirmation.
- **Cite OWASP controls by name + edition** — the research dossier found OWASP renumbered C1–C10 between 2018 (the proposal's list) and 2024; bare numbers silently change meaning across editions.
- **Fence rule scoped to named generation surfaces** (F-SEC-3) — §5B fields are inert author prose; the injection surface that matters is where a skill *generates* an artifact from them (`/slo-resume`, `/slo-ship`), so the rule names those rather than over-claiming.

## Mistakes made
- None material. One process note: the primary template copy had not been Read in this session, so the first Edit was rejected ("File has not been read yet") — had to Read the exact region first. Cheap, but a reminder that the harness tracks per-file read state even for a dual-copy file.

## Root causes
- The Read-before-Edit rejection: I had Read the mirror earlier but edited the primary; read-state is per-path.

## What was harder than expected
- Nothing. The measurement-loop precedent made this nearly mechanical.

## Naming conventions established
- Canonical doc at repo root: `docs/SECURE-VALUE-LOOP.md` (UPPER-KEBAB, peer to `docs/LOOPS-*.md`).
- Template section id `## 5B.` (letter suffix for additive sections between numbered ones).
- Structural test file `xtasks/sast-verify/tests/svl_m<N>.rs`; helper shape copied from `mloop_m3_plan.rs` (`workspace_root` / `read`).

## Test patterns that worked well
- Reusing the `mloop_m3_plan.rs` `workspace_root()`/`read()` helpers verbatim — consistent with the existing suite and robust to cwd.
- Asserting byte-identity AND content presence in separate tests, so a failure localises (drift vs missing-section).

## Missing tests that should exist now
- None for M1. (M3 will need a Rust round-trip test for `MilestoneStatus`; M4 a "no new lane verb" assertion.)

## Rules for the next milestone
- **M2**: `/slo-plan` SKILL.md must require §5B for **value-bearing OR security-relevant** milestones, **forward-looking** (flag a missing contract; never invalidate legacy runbooks — copy the Measurement-Contract requirement wording shape). The generated §5B must carry the **inert-window note** (F-ENG-3): "Operator Readiness Gate enforced by `/slo-execute` from the M3 release onward."
- If `/slo-plan` is pinned by a structural test SHA baseline, update it in the same milestone.
- Keep using edit-primary-then-`cp`-mirror for any further template edits, and keep `mloop_m3_plan.rs` green.

## Template improvements suggested
- None this milestone. The §5B section is now part of the v4 template itself.

## filed_issues
- none — M1 surfaced no detected-work items; the Detected Work Ledger discipline itself ships in M4.
