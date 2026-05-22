# Interfaces — kani-verification

Interfaces downstream milestones must not rename or reshape without explicit migration work. Stability levels: `stable` (frozen), `evolving` (may change with migration), `internal` (fair game).

## Skill identity & layout

| Interface | Description | Stability |
|---|---|---|
| `skills/slo-kani/SKILL.md` | Skill entry point; `name: slo-kani`. Discovered by `discover_skills()` (`crates/sldo-install/src/install.rs`) and installed by `sldo-install`. | stable |
| `skills/slo-kani/tools.toml` | Pinned `kani-verifier` version + acquisition commands. Same role as `slo-tla/tools.toml`. | stable |
| `skills/slo-kani/references/*.md` | Method-dispatch files (candidate-scoring, harness-generation, run/triage ladder, fallback strategies, verified-scope writeup). Loaded per-phase. | evolving |
| `skills/slo-kani/evals/*.md` | Eval scenarios (happy-path, adversarial, ambiguous-input, missing-context, tool-failure, high-risk-case, outdated-information) — same seven-eval shape as `/slo-tla`. | evolving |

## Frontmatter contract (new key)

| Interface | Description | Stability |
|---|---|---|
| `kani_required: <bool>` in `docs/slo/design/<slug>-overview.md` | How `/slo-kani` knows whether to run. Set `true` by `/slo-architect` when stack is Rust **and** design has unsafe code, raw pointers, arithmetic/boundary logic, parsers/state machines, or representation invariants. Default-when-absent: `false` (existing docs stay valid). One-line justification required in frontmatter, same as `tla_required`. | stable |

## Verified-scope artifacts (skill outputs)

| Interface | Description | Stability |
|---|---|---|
| `<crate>/src/**` `#[cfg(kani)]` harness modules | Where `/slo-kani` writes `#[kani::proof]` harnesses — gated so they never compile into release/test builds. | stable |
| `docs/slo/verify/<slug>-kani.md` | The verified-scope report: properties proved, assumptions, bounds, unwind values, stubs/contracts used, solver, what remains unproved, catch→remediate→green evidence. Cited by `/slo-verify` and `/slo-retro`. | stable |
| `docs/slo/design/<slug>-kani-pairing.md` | (When both `tla_required` & `kani_required`) the TLA+ action → Rust fn → Kani harness refinement map. M5 deliverable. | evolving |

## v4 runbook template seam

| Interface | Description | Stability |
|---|---|---|
| §5.8 "Kani proof obligations" sub-block (in `runbook-template_v_4_template.md`) | Per-function: target, property, bound, assumptions, expected pre-fix/post-fix. Authored by `/slo-plan`, executed by `/slo-execute`. Additive to the existing TLA+ content in §5 — does not replace it. **Landed M3.** | stable |
| Milestone Evidence Log → Kani-obligation rows | `/slo-execute` fills "Actual Result" with the `cargo kani` outcome; `/slo-retro` refuses to close on blank rows (existing rule, extended to Kani rows). | stable |

## Cross-skill handoff contract

| From → To | Contract | Stability |
|---|---|---|
| `/slo-architect` → `/slo-kani` | `kani_required: true` ⇒ architect emits a candidate-module shortlist in the overview; `/slo-kani` consumes it. | evolving |
| `/slo-plan` → `/slo-execute` | §5 Kani obligations become per-milestone Evidence-Log rows. | stable |
| `/slo-execute` → `/slo-verify` | `/slo-verify` confirms harnesses ran green at stated bounds and scope claims are honest (no concurrency overclaim). | stable |
| `/slo-verify` → `/slo-retro` | `/slo-retro` records proved properties / assumptions / bounds / residual-unproved into the lessons file. | stable |
| `/slo-tla` ↔ `/slo-kani` | Refinement map (M5): TLA+ actions correspond 1:1 to Kani-verified Rust kernels. Neither claims the other's scope. | evolving |
