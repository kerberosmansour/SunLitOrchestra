# Verification Report — experiment-rigor Milestone 1

Protocol Freeze in `/slo-precision`. This milestone changes Markdown skill contracts and uses a Rust cross-artifact structural test as its executable gate; it does not add a runtime service or UI.

## Pass 0 — Outcome Contract

| Outcome slice | Exercise | Result | Evidence |
|---|---|---|---|
| Complete freeze is reviewable end to end | `protocol_freeze_contract_is_complete_across_all_artifacts` reads the skill, template, authoritative spec, and interface | pass | all four artifacts require the version/date, hypothesis, baseline, candidate interventions, benchmark arms/split IDs, metrics, scoring, repetition/stability, accept/kill, and resource/risk fields |
| Post-result edits cannot masquerade as the original protocol | `amendments_are_append_only_and_invalidate_validation` | pass | append-only old/new/reason/impact record plus stale-validation and rerun language is present in every contract copy |
| Invalid and legacy states are honest | `incomplete_and_legacy_protocols_do_not_become_confirmation` | pass | incomplete freezes block validation; legacy Books are readable but degraded/unconfirmed |
| User-authored protocol text remains data | `raw_protocol_source_text_is_literal_data` | pass | `~~~text` literal-data fences are required and source strings cannot select control fields |
| Confirmation work is bounded | `protocol_bounds_are_finite` | pass | finite arms, repetitions/sample budget, time/cost bounds; open-ended “run until good” is forbidden |
| Existing Book spine remains stable | `experiment_book_section_order_is_unchanged` | pass | §0–§11 order and §6 identity are unchanged |

The structural tests prove that the authoring contract is present and synchronized; they do not prove that an arbitrary live model will obey the prose. No live model is available or required for this deterministic milestone. The filled gallery Book in M4 is the planned behavioral dogfood proxy.

## Pass 1 — Happy Path

`cargo test -p sast-verify --test innovation_loop_rigor_m1_protocol_freeze` passed all 6 tests. Manual inspection of template §6 found the freeze and amendment path readable without consulting the skill prose.

## Pass 2 — Invalid, Empty, And Compatibility Paths

- Missing freeze fields block a Validation Record rather than fabricating confirmation.
- No surviving candidate creates no fake freeze.
- Existing v1 Books without the additive fields remain readable and are explicitly legacy/degraded.
- Raw source statements containing instruction-like text stay fenced as literal evidence.

Result: pass through targeted structural assertions and contract inspection.

## Pass 3 — Partial Failure, Bounds, And Recovery

- An amendment is append-only, makes the current validation stale, and requires a rerun.
- The protocol declares finite arms, repetition/sample budget, and time/cost/resource bounds.
- The contract forbids an open-ended “run until good” loop.

Result: pass.

## Pass 4 — Security

| Check | Result | Note |
|---|---|---|
| Bundle A — docs/contract assessment | pass | first-party Markdown contracts; user-supplied source strings are fenced literal data and cannot select version, IDs, thresholds, confidence, status, or route fields |
| Existing security/regression suite | pass | full `cargo test -p sast-verify --tests` is green |
| Secret/artifact hygiene | pass | `git diff --check` is clean; `git status --short` contains only allow-listed M1 files and administrative artifacts |
| SCA | N/A | no dependency or lockfile changes |
| DAST/authn/authz/IaC/container | N/A | no service, identity, cloud, or image surface |

## Pass 5 — AI Tolerance

Accepted authoring variance is limited to the content of hypotheses, metrics, and evidence. Deterministic boundaries are the required freeze field set, append-only amendment/rerun rule, literal-data boundary, and frozen route/status vocabularies. The targeted test is the eval; incomplete input has a fail-closed fallback. Result: pass.

## Pass 6 — Measurement And Telemetry

No runtime telemetry context exists. Immediate measurement is the synchronized four-artifact contract and the green six-test targeted suite; the lagging manual three-Book review remains scheduled after the full runbook. Result: skipped with documented manual follow-up.

## Regression Evidence

| Command/check | Result |
|---|---|
| `cargo test -p sast-verify --test innovation_loop_rigor_m1_protocol_freeze` | pass — 6/6 |
| `cargo test -p sast-verify --test innovation_loop_m3_converge` | pass — 4/4 |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy -p sast-verify --test innovation_loop_rigor_m1_protocol_freeze -- -D warnings` | pass |
| `cargo test -p sast-verify --tests` | pass — full suite |
| `git diff --check` | pass |

The package-wide clippy form remains non-gating because of pre-existing unrelated warning debt; the newly added M1 target is warning-clean under `-D warnings`.

## Bugs Found

None in M1. Pre-flight exposed a pre-existing brittle catalog-count assertion pinned to 49 while HEAD contains 51 shipped skills. `DW-001` was fixed in scope by reconciling the catalog headline with discovered `skills/*/SKILL.md` directories; targeted and full tests are green.

## Coverage Gaps

- Structural assertions cannot attest that a live LLM followed the contract.
- No domain experiment is run in M1; the synthetic end-to-end gallery dogfood is intentionally deferred to M4.

Both gaps are explicit in the runbook and do not weaken the M1 contract claim.

## Disposition

M1 verified. The Protocol Freeze and amendment semantics are synchronized across the skill, template, authoritative spec, and interface; all targeted and full regression gates pass. Ready for `/slo-retro M1`.
