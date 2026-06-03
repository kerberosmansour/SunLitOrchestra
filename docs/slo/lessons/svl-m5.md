# Lessons Learned — svl Milestone 5

## What changed
- `docs/LOOPS-ENGINEERING.md`: added the "Secure Value Loop overlay" subsection to the Sprint loop — a per-stage security-output table + honest exit states + anti-drift review cadence.
- `docs/LOOPS-BUSINESS.md`: added a "Security-visible proof of safety" cross-ref so product/value review includes safety.
- `skills/slo-ship/SKILL.md`: added the secure-release checklist + closed `ship_state` enum + conditional SBOM/provenance.
- Dogfooded a FILLED §5B Secure Value & Security Contract into this runbook (Value Wedge, Operator Readiness, Threat Model Summary citing the frozen abuse IDs, Bundle-A Security Test Plan, a real Detected Work Ledger with DW-001..DW-005).
- Added `xtasks/sast-verify/tests/svl_m5.rs` (5 assertions).
- **Detected-work fixes (caught during full-workspace verify):** DW-004 completed the M3 version bump (dep strings + `PUBLISH_READY_VERSION` → 0.1.3); DW-005 fixed two structural caps (`/slo-plan` decomposed to `references/secure-value-contract.md`; `/slo-verify` soft-cap-exception).

## Results vs thesis
- N/A — not value-bearing. But the dogfood is the adoption proof the proposal §11 asks for: the §5B contract was fillable on a real runbook, and the Detected Work Ledger captured 5 real items with dispositions rather than prose.

## Design decisions and why
- **Overlay, not a new loop** — the security envelope wraps the existing Sprint loop, so it lives as a subsection of the Sprint loop in LOOPS-ENGINEERING, not a competing top-level loop.
- **`/slo-ship` SBOM stays conditional** — making it a hard gate would block every markdown runbook; the checklist resolves it to `not_applicable` unless a released artifact is built (the M5 design rule, asserted by `svl_m5`).
- **Dogfood in this runbook itself** — the smallest real surface; it makes the contract demonstrably fillable and turned the abstract ledger discipline into 5 concrete disposed rows.

## Mistakes made
- **The biggest lesson: M3's version bump was incomplete.** M3's lessons claimed "no ripple because `^0.1.2` accepts 0.1.3" — true for `cargo build`, but a pre-existing `publish_prep` lockstep test enforces exact-string parity across the root version, the two internal dep requirements, and `PUBLISH_READY_VERSION`. The per-milestone `cargo test -p sast-verify` baseline did NOT include `sldo-install`, so the regression hid until M5's `cargo test --workspace`.
- Skill-prose additions silently approached two structural caps (slo-plan 80 hard, slo-verify 200 soft); not checked until the full-workspace run.

## Root causes
- **Baseline scope too narrow.** The runbook's declared baseline (`cargo test -p sast-verify`) excluded `sldo-install`/`sldo-research`, so cross-crate regressions (version lockstep, SKILL.md caps) were invisible per-milestone. The fix worked because a *full-workspace* run was done at M5 — but it should have been the baseline from M3 onward (the moment a crate was touched).

## What was harder than expected
- The version bump's true blast radius (3 extra files + a test constant) only became visible under `cargo test --workspace`.

## Naming conventions established
- Reference extraction file `skills/<skill>/references/<topic>.md` with `name:` + `source_skill:` frontmatter (matches `methodology-milestone-authoring.md`).
- `# soft-cap-exception: <reason>` as a frontmatter YAML comment for legitimately-large skills (matches `slo-execute`/`slo-retro`).

## Test patterns that worked well
- Running `cargo test --workspace` (not just the narrow baseline) at the final milestone caught both cross-crate regressions. Pre-existing structural tests (`publish_prep`, the cap guards) did their job.

## Missing tests that should exist now
- None new — the existing lockstep + cap tests already cover the regressions; they just need to be in the baseline.

## Rules for the next milestone / runbook
- **When a runbook touches a workspace crate, the baseline test command MUST be `cargo test --workspace`** (or at least include every dependent crate), not a single `-p` package — otherwise cross-crate structural tests (version lockstep, SKILL.md caps, citation-path checks in `sldo-install`) hide regressions until ship.
- A workspace version bump is a 4-touch change: root `[workspace.package] version`, each internal dep `version =` string, and `PUBLISH_READY_VERSION`.

## Template improvements suggested
- The v4 metadata "Default baseline test command" guidance could note: for SLO-pack-internal runbooks, prefer `cargo test --workspace` because `sldo-install` carries cross-cutting structural tests.

## filed_issues
- none auto-filed (confirmation-gated). DW-001 (cargo deny licenses) and DW-002 (GitHub labels) carried to ship for user confirmation. DW-004/DW-005 fixed in-milestone (`fix_now`).
