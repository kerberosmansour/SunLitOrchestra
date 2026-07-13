# Verification Report — experiment-rigor Milestone 2

Discovery/validation separation in `/slo-spike`. This is a Markdown contract milestone with Rust structural gates; no benchmark, runtime service, or UI was added.

## Pass 0 — Outcome Contract

| Outcome slice | Exercise | Result | Evidence |
|---|---|---|---|
| Evidence classes are visibly different | `record_types_are_distinct_across_contracts` + `discovery_is_exploratory_and_may_refine_the_mechanism` | pass | skill, template, spec, and interface name `DiscoveryRecord` and `ValidationRecord`; discovery may refine and is not confirmation |
| Validation consumes the freeze rather than reinventing it | `validation_uses_the_active_freeze_without_tuning` | pass | complete active protocol version, held-out/frozen arms, baseline/candidate comparison, per-arm results, and no-tuning rule present in all four contracts |
| Method handoff is rerunnable | `validation_is_reproducible_and_reports_stability` | pass | exact commands, environment, repetitions, stability, deviations, and separate discovery/validation budgets present in all four contracts |
| Protocol changes remain honest | `amendment_stales_validation_and_routes_back_to_precision` | pass | an amendment routes through `/slo-precision`, stales validation, and requires rerun |
| Evidence cannot issue control instructions | `evidence_strings_are_literal_and_cannot_select_controls` | pass | `~~~text` literal-data boundary; evidence never selects verdict, confidence, or route |
| Compatibility is conservative | `legacy_spike_cards_are_discovery_grade_only` + existing M4 spike suite | pass | legacy generic cards remain readable but discovery-grade/not confirmed; original scratch/budget/no-production rules remain green |

These checks prove the authoring permissions and fields are synchronized. They do not attest that a benchmark was actually run or that a live model obeyed the contract; M4 dogfoods a filled synthetic Book as the available behavioral proxy.

## Pass 1 — Happy Path

The targeted suite passed 8/8. A manual side-by-side read of the template shows a lightweight exploratory record and a stricter validation record, with shared scratch/safety bounds but visibly different permissions and completion gates.

## Pass 2 — Invalid, Empty, And Compatibility Paths

- No complete active freeze or held-out arm blocks validation and preserves exploratory confidence.
- A legacy generic Spike Card is readable as discovery-grade evidence and cannot be inferred confirmed.
- Instruction-like benchmark/model output remains fenced literal data.
- The existing spike safety suite remains green, preserving invocation, scratch root, resource budget, evidence-derived verdict, delete-or-promote, and no-production-promotion behavior.

Result: pass.

## Pass 3 — Partial Failure, Bounds, And Recovery

- Protocol-changing deviations create an amendment, stale the Validation Record, and require rerun from the new active version.
- Failed/missing repetitions and dispersion belong in the stability summary rather than being hidden behind the best result.
- Discovery and Validation budgets are finite and reported separately.

Result: pass.

## Pass 4 — Security

| Check | Result | Note |
|---|---|---|
| Bundle A — contract assessment | pass | untrusted output is literal data; control fields stay agent-authorized; held-out/no-tuning rule reduces discovery/confirmation leakage |
| Existing security/regression suite | pass | full `cargo test -p sast-verify --tests` green |
| Secret/artifact hygiene | pass | `git diff --check` clean; status contains only cumulative allow-listed M1/M2 and SLO evidence files |
| SCA | N/A | no dependency or lockfile change |
| DAST/authn/authz/IaC/container | N/A | no service, identity, cloud, or image surface |

## Pass 5 — AI Tolerance

Method prose and domain metrics may vary. Deterministic boundaries are the two record types, active-freeze citation, held-out/no-tuning rule, per-arm/repetition/stability fields, exact rerun context, literal-evidence boundary, and stale-on-amendment behavior. Incomplete input fails closed by returning to `/slo-precision`. Result: pass.

## Pass 6 — Measurement And Telemetry

No runtime telemetry surface exists. Immediate measurement is the 8/8 cross-artifact contract test plus green existing spike/front-half regressions. Manual review of the next three real recommendation packets remains the lagging follow-up. Result: skipped with documented manual follow-up.

## Regression Evidence

| Command/check | Result |
|---|---|
| `cargo test -p sast-verify --test innovation_loop_rigor_m2_validation` | pass — 8/8 |
| `cargo test -p sast-verify --test innovation_loop_m4_spike` | pass — 6/6 |
| `cargo test -p sast-verify --test innovation_loop_m2_divergent` | pass — 5/5 |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy -p sast-verify --test innovation_loop_rigor_m2_validation -- -D warnings` | pass |
| `cargo test -p sast-verify --tests` | pass — full suite |
| `git diff --check` | pass |

## Bugs Found

None. The first semantic-red run had 7 expected failures and 1 green compatibility assertion. During implementation, the first green attempt identified one line-wrapped `may refine` sentinel in the interface; the prose was normalized and the full suite rerun green.

## Coverage Gaps

- No real held-out corpus or benchmark is executed in this Markdown-only milestone.
- Structural tests cannot verify live-agent compliance.

The runbook already assigns the filled synthetic path to M4 and avoids claiming empirical validation here.

## Disposition

M2 verified. Discovery and validation are separate, bounded evidence contracts; validation consumes the active freeze without tuning and remains reproducible, stale-on-amendment, and compatible with the original spike safety boundary. Ready for `/slo-retro M2`.
