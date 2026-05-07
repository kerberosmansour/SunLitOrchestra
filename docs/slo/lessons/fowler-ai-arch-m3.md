# Lessons Learned — fowler-ai-arch Milestone 3

## What changed

- Added `skills/slo-plan/references/ai-tolerance-contract.md` with required nondeterminism-tolerance fields.
- `/slo-plan` now requires an AI tolerance Contract Block row for AI/LLM behavior and an explicit `N/A — no AI component` path for non-AI milestones.
- The milestone-authoring methodology and both v4 template mirrors include the AI tolerance row.
- `/slo-architect` now links `ai_component: true` to downstream AI tolerance contracts.
- `/slo-verify` now documents a gated AI tolerance pass after the normal runtime/security passes.
- `crates/sldo-install/tests/e2e_fowler_ai_arch_m3.rs` locks the M3 contract.

## Design decisions and why

- Keep AI tolerance gated instead of global. Non-AI milestones should not pay the prompt tax or invent fake eval evidence.
- Put sample budget in the required fields because unbounded retries and "try until good" loops are a reliability and cost failure mode.
- Add the verification pass after Pass 4 so AI behavior evidence does not replace normal runtime, degraded-state, boundary, or security checks.

## Mistakes made

- The first formatter check found two rustfmt line-wraps in the new M3 test. Fixed only the new file and left unrelated formatter drift untouched.

## Root causes

- The existing workspace has rustfmt drift outside this runbook, so every new Rust test must be checked against the global diff and cleaned locally.

## What was harder than expected

- Keeping the contract concise while still distinguishing acceptable variance from deterministic boundaries. The pair is the useful bit: output can vary, but schemas, safety rules, persisted data, and interfaces cannot silently drift.

## Naming conventions established

- AI tolerance reference path: `skills/slo-plan/references/ai-tolerance-contract.md`.
- Verification pass name: `Pass 5. AI tolerance (gated)`.
- M3 verification report: `docs/slo/verify/fowler-ai-arch-m3.md`.

## Test patterns that worked well

- Structural tests inspect the skill-local and docs-template mirrors together.
- Ordering checks on `/slo-verify` make sure AI tolerance is additive after existing Passes 1-4.

## Missing tests that should exist now

- A future harness should run a real AI-feature runbook through `/slo-plan` and assert that the generated Contract Block fills all six AI tolerance fields.
- A future `/slo-verify` harness should exercise a bounded golden/scenario sample set and fail on omitted sample budgets.

## Rules for the next milestone

- M4 should check that architecture-coherence critique cites the code map, reversibility, exemplar, and AI tolerance contracts together rather than treating them as separate paperwork.
- Preserve the existing four-persona critique shape; M4 should strengthen engineering review, not add a new persona.

## Template improvements suggested

- Consider a compact example row for AI tolerance in a future template appendix once a real AI milestone produces a good exemplar.
