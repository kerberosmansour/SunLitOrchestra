# Lessons Learned — experiment-rigor Milestone 4

## What Changed

- The gallery Experiment Book now demonstrates the complete rigorous back half while labeling every number as synthetic fixture evidence.
- Its confidence is `confirmatory` only within the synthetic fixture scope and explicitly not `engineering_ready`; real-world generalization, retained-runner reproduction, user value, and independent replication remain gaps.
- Threat-model IDs 7–8 cover post-result protocol mutation and discovery/validation evidence leakage in both Markdown and JSON, with current `/slo-execute` provenance.
- README, architecture, loop docs, catalog, security, and the design overview now describe the same shipped path; the duplicate Innovation Sandbox loop section was removed.
- A seven-test end-to-end contract parses the threat artifact, checks the gallery chain/provenance, pins one canonical loop section, and preserves the human-controlled Book spine.

## Results Versus Thesis

The runbook's value hypothesis is supported at contract and synthetic-dogfood level. A reviewer can now determine what was explored, what was frozen, what was validated, what caused the result, what still failed, how to attempt reproduction, how strong the evidence is, and which human-controlled route follows—all without chat memory. The creative front half remains divergent and the existing workflow regressions remain green.

The lagging thesis is not yet proven: the next three real promotions still need manual review to see whether a downstream artifact can consume the packet without a clarification round.

## Outcome Versus Promise

The requested end-to-end methodology change is complete. Protocol Freeze, discovery/validation separation, ablation, failure taxonomy, reproducibility fields, structured reporting, and confidence-calibrated Recommendation Packets are implemented across skills, Book contracts, interfaces, a gallery, threat/security documentation, and executable structural tests.

The implementation does not claim an AGT result, a generic benchmark engine, statistical certification, or live-model compliance. Those were deliberately out of scope.

## Design Decisions And Why

- **Synthetic dogfood stays visibly synthetic.** Reusing the existing fixture numbers was acceptable only after scope, provenance, generalization, and independent-replication caveats became impossible to miss.
- **Confirmatory is scoped, not universal.** The fixture can demonstrate a clean frozen comparison while still failing engineering readiness because no real corpus, retained runner, product policy, or independent replication exists.
- **Threat provenance names the actual extender.** Rows 7–8 were produced by `/slo-execute` from this runbook; retaining architect-only provenance would make staleness invisible.
- **One public loop section.** Duplicate explanations create drift; the canonical section now links the gallery and names the rigorous back half once.
- **Design overview received a consistency-only update.** It remained the declared source of truth, so leaving “planned/dashed” language would have contradicted architecture and the catalog.

## Mistakes And Corrections

- The first gallery green-up wrapped “not independently replicated” across lines, causing the stable-sentinel test to fail. The phrase was made contiguous and rerun green.
- The first M4 format check found standard Rust layout differences; the formatter was applied and all affected gates rerun.
- Initial architecture/threat edits briefly risked stale or duplicated descriptions; the final checks assert one loop section, current provenance, and synchronized IDs.

## Test Patterns That Worked

- Parse the real machine-readable threat artifact rather than relying on visual JSON inspection.
- Keep public-document assertions narrow: the four method landmarks and a single canonical section, not byte-level prose pinning.
- Pair positive method-chain checks with honesty sentinels (`synthetic fixture evidence`, `not real-world validation`, `not independently replicated`).
- Assert producer/date/runbook input so future threat extensions cannot quietly inherit stale provenance.

## Follow-Up Rules

1. Review the next three real promoted Experiment Books against the RecommendationPacket field set.
2. Record whether the receiving idea/research/ticket/runbook artifact needs clarification on baseline, split, metric, ablation, failure family, rerun method, or confidence.
3. Do not upgrade confidence when a retained runner, evidence artifact, or independent replication is absent.
4. Evolve stable labels across skill/template/spec/interfaces/tests together; use an explicit migration for route or section changes.
5. Keep creative judgment out of `/slo-play`; rigor starts after reusable patterns emerge.

## Detected Work Ledger Disposition

- `DW-001` — completed in M1; the full suite remains green.
- No new finding requires a GitHub issue. No issue was filed without user confirmation.
