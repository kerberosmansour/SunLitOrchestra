# Lessons Learned — experiment-rigor Milestone 2

## What Changed

- `/slo-spike` now emits distinct `DiscoveryRecord` and `ValidationRecord` objects without adding another skill or code phase.
- Discovery is exploratory, may refine a mechanism, and is not confirmation.
- Validation cites one complete active Protocol Freeze, uses held-out/frozen arms with no tuning, and reports per-arm results, exact commands, environment, repetitions, stability, deviations, and a separate finite budget.
- A protocol-changing deviation returns through `/slo-precision`, appends an amendment, stales the current validation, and requires rerun.
- Skill, template §7, authoritative spec, and interface carry the same fields and gates; the original spike safety suite remains green.

## Outcome Versus Promise

The M2 promise is met: a reviewer can distinguish how a mechanism was discovered from how it was cleanly evaluated, and can see which protocol version, arms, commands, environment, repetitions, failures, and budgets produced the validation result. The same evidence cannot quietly serve as both hypothesis generator and held-out proof.

This is contract-level evidence, not a claim that a real benchmark has run. The wording and tests preserve that boundary explicitly.

## Design Decisions And Why

- **Two records inside one skill.** The risk is evidence leakage, not a missing workflow stage; retaining one scratch-only code phase keeps the loop lightweight.
- **Shared envelope, different permissions.** Scratch path, safety, cleanup, and resource discipline remain common, while discovery may iterate and validation may not tune.
- **Per-arm and stability reporting over a single headline.** Failed/missing repetitions and dispersion remain visible.
- **Amend through precision.** `/slo-spike` consumes the active freeze and does not gain authority to redefine it.
- **Legacy means discovery-grade.** Old Books remain useful without being silently promoted to confirmation.

## Mistakes And Corrections

The first implementation run left one interface sentinel line-wrapped between “may” and “refine”. The contract meaning was present, but the deterministic cross-artifact test correctly exposed a brittle textual mismatch. The phrase was normalized and all targeted/full gates rerun green.

This reinforces the M1 lesson: structural tests are useful for keeping multiple Markdown contract copies synchronized, but labels intended as stable interface language should be rendered consistently and not depend on wrapping.

## Test Patterns That Worked

- Keep the original safety contract as its own green assertion in the new semantic-red test; this shows the change is additive.
- Assert each evidence permission across skill, template, spec, and interface, not only that both object names exist.
- Test invalid and recovery paths—legacy evidence, amendment/stale/rerun, and literal output—separately from the happy comparison path.
- Run the divergent-front-half test alongside the spike test to show confirmatory rigor did not leak backward into play.

## Rules For M3

1. Confidence must be derived from evidence class and completeness: `exploratory | confirmatory | engineering_ready`.
2. `promote_to_ticket` and `promote_to_runbook` require a current Validation Record plus ablation evidence; a confidence word alone is insufficient.
3. Idea/research routes may remain exploratory, but must state the confirmation gaps honestly.
4. Recommendation Packets must carry the active protocol version, baseline/candidates, arms/split IDs, metrics, ablation, failure taxonomy, replication context, limitations, confidence, and the exact engineering question.
5. Preserve exactly-one disposition, frozen routes/destinations, seed tables, and suggestion-only human control.
6. Evidence excerpts remain `~~~text` literal data and never select disposition, confidence, or route.

## Detected Work Ledger Disposition

- `DW-001` remains completed from M1.
- No new finding requires a ledger row or GitHub issue.
