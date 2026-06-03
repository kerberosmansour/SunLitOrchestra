# Stack Decision — secure-value-loop

## Chosen stack

- **Markdown skill-contract + v4-template edits + LOOPS docs**, enforced by
  **Rust structural-contract tests** in the existing `xtasks/sast-verify` crate.
- **No NEW crate. No new runtime. No new machine schema. No new taxonomy.**
- **One targeted edit to an EXISTING crate** (M3): extend
  `sldo-common::runbook::MilestoneStatus` to be total over the additive status
  set (F-ENG-1, surfaced by `/slo-critique`). This is additive + a crates.io
  semver bump, not a new crate — see [interfaces §2](secure-value-loop-interfaces.md).

## Reason

The research synthesis is unambiguous: the design must be ruthlessly subtractive
because ~80% of the proposal's security *machinery* already ships, so the work is
**contract-discipline, not capability**. The three net-new disciplines (Operator
Readiness Gate, Detected Work Ledger, honest exit states) are LLM-native
reasoning expressed as Markdown contracts — exactly the shape the
measurement-loop precedent proved works (inline sections + one structural test,
no crate). The only deterministic obligation is *structural* (sections/rows
present, dual-template byte-identity, additive enum), which the existing
`xtasks/sast-verify/tests/` harness already covers. The design must reuse the
`/slo-retro` lane vocabulary because the synthesis shows shipping a third
disposition taxonomy would fracture an already-tested filing flow.

## Rejected alternatives

- **Approach A — docs + template only (no skill enforcement)** — rejected: the
  contract would be *declared* but nothing would *check* it, so milestones keep
  behaving as today and the adoption criteria in proposal §11 ("every milestone
  has an Operator Readiness state", "every discovered issue has a disposition")
  can't be met.
- **Approach C — Rust-backed ledger/readiness validator (`sldo-secure-value`
  crate)** — rejected: forces LLM-native judgement (is this finding `fix_now` or
  `fresh-runbook`? is this wedge "too small"? is the operator "ready"?) into Rust
  where it is strictly worse, and adds a maintained binary for a markdown
  discipline. Over-engineered; same anti-pattern the `slo-security-embedding`
  design already rejected for `/slo-sec-libs`.
- **A new disposition taxonomy distinct from `/slo-retro` lanes** — rejected:
  duplicates an already-tested vocabulary and creates two competing
  classifications for the same finding. The ledger's five dispositions are mapped
  onto the existing lanes instead (see interfaces).
- **A `.slo.json` machine companion for the ledger (mirroring the threat-model
  companion)** — rejected for v1: deferred until real ledger fixtures exist, same
  low-regret ordering the measurement-loop design used for its telemetry schema.

## Non-negotiables (downstream cannot change these without migration)

1. **Additive-only / backward compatible.** Legacy runbooks without the Secure
   Value & Security Contract section remain valid. New milestone-status values
   *extend* the enum; the old four (`not_started | in_progress | blocked | done`)
   must still parse. `/slo-resume`, `/slo-execute` Step 1.5, and the Milestone
   Tracker parser must work unchanged on legacy runbooks. (Same posture as §5A /
   §10.)
2. **Dual-template byte-identity.** The two v4-template copies
   (`docs/slo/templates/...` mirror and `skills/slo-plan/references/...` primary)
   stay byte-for-byte identical; a structural test asserts it.
3. **One disposition vocabulary.** The Detected Work Ledger reuses `/slo-retro`'s
   lanes; no third taxonomy ships.
4. **OWASP controls cited by name + edition (2024), never bare number.**
5. **SBOM/provenance conditional.** Ship checklist resolves SBOM/provenance to
   `not_applicable` unless the milestone builds a released artifact.
6. **No new external dependency.** No vendor SDK, no new scanner; the envelope
   *routes to* existing skills (`/slo-dast-tuner`, `/slo-sast`, `/slo-sec-libs`,
   `/slo-cloud-threat-model`) rather than adding tools.
7. **Structural test is the only gate.** Enforcement is PR-time structural
   assertions in `xtasks/sast-verify`, not a runtime validator.
