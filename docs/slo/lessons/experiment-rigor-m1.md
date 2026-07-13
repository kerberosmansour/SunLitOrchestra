# Lessons Learned — experiment-rigor Milestone 1

## What Changed

- `/slo-precision` now emits a versioned `ProtocolFreeze` with a complete comparison contract and an append-only `ProtocolAmendmentLog`.
- Experiment Book §6, the authoritative Book specification, and the interfaces contract carry the same fields and invalidation semantics.
- Raw protocol/source statements are fenced as literal data and cannot select control fields.
- Legacy Books remain readable but are explicitly degraded/unconfirmed; incomplete or amended protocols cannot proceed as valid confirmation.
- A six-test Rust contract locks the skill, template, specification, and interface together.

## Outcome Versus Promise

The M1 promise is met: before confirmation begins, a reviewer can see the hypothesis, baseline, candidate interventions, benchmark arms/split IDs, metrics, scoring method, repetition/stability rule, accept/kill rules, and resource/risk envelope. A later change is visible as an amendment and makes prior validation stale until rerun. The creative front half and the frozen Experiment Book §0–§11 spine are unchanged.

The evidence is deliberately contract-level. It proves synchronized authoring requirements, not live-model compliance or empirical validity. M4 remains responsible for the filled end-to-end dogfood example.

## Design Decisions And Why

- **Additive v1 fields, not a Book v2.** Existing Books stay readable and existing skill invocations remain stable.
- **Amend rather than rewrite.** An append-only old/new/reason/impact record makes post-result rule changes reviewable and forces honest reruns.
- **Fail closed for confirmation.** Missing fields produce an incomplete freeze; legacy content is never upgraded to confirmed evidence by inference.
- **Fence source statements at the trust boundary.** Instruction-like text is evidence data, never authority over version, thresholds, confidence, status, or route.
- **Finite budgets are part of the protocol.** Confirmation cannot quietly continue until a favorable result appears.

## Mistakes And Corrections

The required baseline was initially red before M1 because `outcome_first_m5_principle.rs` pinned the catalog count to 49 while HEAD ships 51 skills. The test was brittle rather than the catalog being wrong. `DW-001` replaced the literal with a reconciliation between the catalog headline and discovered `skills/*/SKILL.md` directories; the baseline was rerun green before implementation continued.

The runbook's shared exit sentence still named the known-red package-wide clippy form after the critique had made targeted clippy the gate. It was corrected during closeout so the shared exit protocol and milestone contracts agree.

## Test Patterns That Worked

- Start with a compiling semantic-red test, keeping section-order compatibility as an independently green assertion.
- Require the same sentinel groups across every authoritative contract copy instead of checking only the template.
- Give invalid, legacy, amendment, literal-data, and finite-budget behaviors their own assertions so a happy-path edit cannot mask a safety regression.
- Reconcile mutable catalog counts from repository structure rather than pinning a number in an unrelated feature test.

## Rules For M2

1. Treat `ProtocolFreeze` as an input contract; do not redefine its fields in `/slo-spike`.
2. Keep `DiscoveryRecord` and `ValidationRecord` visibly distinct in the skill, template, specification, and interface in the same milestone.
3. A Validation Record must name the active protocol version, use frozen/held-out arms with no tuning, and stop when an amendment makes the protocol stale.
4. Keep source/evidence strings inside `~~~text` fences and prevent them from choosing verdict, confidence, or route.
5. Preserve scratch isolation, finite budgets, and the no-production-promotion gate.
6. Keep tests honest about their proof boundary: contract presence is not proof of an empirical run.

## Detected Work Ledger Disposition

- `DW-001` — `fix_now`, completed in M1. The catalog-count test now reconciles the catalog headline with shipped skill directories and the full suite is green.
- No GitHub issue filing requested or required.
