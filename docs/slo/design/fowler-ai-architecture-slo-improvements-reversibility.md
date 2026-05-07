# Reversibility Matrix — Fowler AI Architecture SLO Improvements

| Decision | Why hard to change | Reversibility tactic | Rollback / migration path | Proof required |
|---|---|---|---|---|
| Add `/slo-architect` output files for reversibility and code maps | Downstream skills may start citing the paths | Additive outputs only; existing architecture outputs remain valid | If too noisy, `/slo-plan` can mark rows `N/A — no brownfield / no hard-to-change decision` while files stay optional | Structural test asserts existing output names still documented and new names are additive |
| Add new v4 Contract Block rows | Runbook authors and agents will depend on row names | Add rows additively; do not rename existing rows | Historical runbooks remain valid; new `/slo-plan` emits rows going forward | Structural test asserts old required rows still present and new rows present |
| Add AI tolerance pass to `/slo-verify` | Verification semantics change for AI components | Gate only when `ai_component: true` or a milestone declares AI behavior | Non-AI milestones emit `N/A — no AI component` | Eval/structural test covers AI and non-AI paths |
| Add critique architecture-coherence check | Review output may produce more findings | Make it part of engineering persona, not a new persona | Findings use existing `ask/defer/auto-fix` categories | Critique eval case covers missing four-object coherence |
| Add ticket-flow parity | More files touched and more tests | Isolate ticket updates to final milestone after sprint-flow rows settle | If ticket parity is too large, split M5 into follow-up runbook before execution | Ticket template test asserts parity with selected runbook rows |

## Design Rule

Every new discipline must have an explicit "N/A with reason" path. Fowler's reversibility principle is about reducing irreversible complexity, not making every simple docs-only milestone carry fake architecture work.
