# Lessons Learned — experiment-rigor Milestone 3

## What Changed

- `/slo-curate` now derives one confidence value—`exploratory`, `confirmatory`, or `engineering_ready`—from explicit evidence prerequisites before selecting one frozen disposition.
- Ticket/runbook routes require a current Validation Record, ablation evidence, structured failure taxonomy, replication context, limitations, and a bounded engineering question.
- Idea/research routes can remain exploratory when confirmation gaps and the decision to unblock are explicit.
- `/slo-demo` emits a method-complete `RecommendationPacket`; the legacy `PromotionPacket` remains a compatible subset.
- Skill, template §§8–10, authoritative spec, and interfaces share the route gate, ablation/failure shapes, packet fields, literal-evidence boundary, and legacy downgrade behavior.

## Outcome Versus Promise

The M3 promise is met: an evidence-strength word cannot authorize itself, engineering routes fail closed when method or residual-risk evidence is missing, and the receiving skill can reconstruct the comparison and next decision without chat memory. The original eight states, four typed destinations, exactly-one disposition, compost behavior, and human-controlled suggestion gate remain intact.

The evidence remains contract-level. No real promotion or empirical claim was performed in M3.

## Design Decisions And Why

- **Confidence is a derived enum, not a narrative adjective.** Each tier has evidence prerequisites and missing evidence downgrades rather than being inferred.
- **Ablation and failure taxonomy are first-class tables.** This makes mechanism attribution and residual risk harder to bury in prose.
- **Engineering rigor does not block early discovery handoffs.** Idea/research can receive exploratory packets, preserving the loop's creative usefulness while exposing gaps.
- **RecommendationPacket evolves PromotionPacket additively.** Existing Books and seed consumers remain readable; incomplete legacy data cannot unlock engineering routes.
- **Suggestion-only remains load-bearing.** More rigorous evidence does not grant the agent authority to invoke downstream work.

## Mistakes And Corrections

The first verification pass found standard Rust formatting in the new M3 test. The formatter was applied and the format/target gates rerun green. No semantic or compatibility bug was found.

The repeated stable labels across five Markdown surfaces are intentionally verbose. The cross-artifact test showed that this duplication is manageable only when the rendered labels remain consistent; future changes should evolve all five together or introduce an explicit migration rather than paraphrasing one copy.

## Test Patterns That Worked

- Keep frozen-route/human-control compatibility green in the initial semantic-red suite.
- Require full packet fields across curation as well as demo; this prevents packaging from discovering missing method context too late.
- Test evidence prerequisites and “cannot self-upgrade” alongside the confidence enum so a label-only implementation cannot pass.
- Assert structured ablation/failure columns, not just the presence of the table names.

## Rules For M4

1. The gallery must label every numeric result as synthetic fixture evidence and avoid implying a real benchmark or independent replication.
2. Fill the complete chain: Protocol Freeze, separate Discovery/Validation Records, ablation, failure taxonomy, confidence, RecommendationPacket, and typed seed.
3. Calibrate the gallery confidence to what its synthetic evidence actually earns; disclose generalization and real-world confirmation gaps.
4. Add threat rows for post-result protocol mutation and discovery/validation leakage without renumbering existing IDs; keep Markdown/JSON in lockstep with honest producer provenance.
5. Update only directly touched public surfaces and remove the exact duplicate Innovation Sandbox loop block.
6. Preserve every frozen skill name, section, route, destination, and suggestion-only gate.

## Detected Work Ledger Disposition

- `DW-001` remains completed from M1.
- No new finding requires a ledger row or GitHub issue.
