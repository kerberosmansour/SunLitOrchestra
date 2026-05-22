---
name: kani-verification
created: 2026-05-22
status: design-locked
tla_required: false
kani_required: true
security_libs_required: false
ai_component: true
compliance: [soc2, asvs]
---

# Design overview — `/slo-kani` Rust formal-verification skill

Single source of truth for downstream skills (`/slo-tla` reads `tla_required`, `/slo-kani` reads `kani_required`, `/slo-plan` reads scope, `/slo-critique` reads compliance and `ai_component`, `/slo-verify` cites the threat-model row IDs).

## What this is

A new **host-neutral** skill `skills/slo-kani/` that brings the [Kani Rust model checker](https://model-checking.github.io/kani/) into the SLO engineering loop as a **code-level** peer to the **design-level** `/slo-tla`. Kani is a bit-precise model checker that proves bounded safety/correctness properties of small Rust kernels via `#[kani::proof]` harnesses. `/slo-kani` owns the methodology: candidate scoring, harness generation, the run-and-triage failure ladder (unwind → solver → stubs → contracts), and the honesty gates that keep "verified" meaning "proved within stated harness, assumptions, and bounds" — never "whole system proved."

The feature is **self-referential**: it is itself a Rust-adjacent addition to a Rust workspace, and the failure-bar demonstration (M4) is a separate seeded-bug crate where the full skill flow catches deliberately-planted bugs, remediates them, and re-verifies green.

> **M4 demo repo (delivered):** `kerberosmansour/sunlit-kani-demo` @ commit `959b23e36a4d6a2e66ed36d07f85884490ec27f6`, verified against `kani-verifier 0.67.0` (the pinned `tools.toml` version). Red→green evidence for K1–K4 is recorded in [docs/slo/verify/kani-verification-kani.md](../verify/kani-verification-kani.md). (Public push pending one terminal authorization — see that report's "Demo repo status".)

Wedge ships as **one runbook, ≤5 milestones**:

- **M1** — `/slo-kani` skill skeleton + tool prereq cascade (pinned `kani-verifier` + `cargo kani setup`) + candidate-scoring rubric + structural-contract test.
- **M2** — harness-generation + run/triage methodology reference files + honesty/scope gates.
- **M3** — integration seams: `kani_required` in `/slo-architect`, §5 Kani sub-block in the v4 runbook template + `/slo-plan`, execute/verify/retro hooks.
- **M4** — separate test repo (seeded-bug crate) + the documented catch→remediate→green failure-bar demonstration.
- **M5** — TLA+ pairing (action→fn→harness refinement map) + CI split (quick PR proofs vs. nightly deep proofs).

## Frontmatter rationale

- `tla_required: false` — the skill itself is offline, single-process, batch. No concurrent shared state, no consensus, no leases, no cross-process ordering. Nothing to model-check at the design level. (Kani and TLA+ *pair* in M5, but that is about the **target** product, not this skill's own implementation.)
- `kani_required: true` — **this is the key new bit.** The skill ships a Rust seeded-bug crate (M4) whose kernels (off-by-one bounded loop, one-past-the-end unsafe pointer read, arithmetic overflow, a contract-modularized recursive helper) are exactly the candidate classes the research report scores highest. Those kernels MUST be Kani-verified as the dogfood of the skill being built. The skill is "complete" only when its own demo crate exhibits the catch→remediate→green loop.
- `security_libs_required: false` — no service surface, no auth, no crypto-confidentiality requirement. Local CLI + skill pack + a subprocess (`cargo kani`). SunLitSecurityLibraries / Hulumi components do not apply.
- `ai_component: true` — the skill drives an LLM agent (Claude/Codex/Copilot) to *generate proof harnesses and interpret counterexamples*. The agent could hallucinate a vacuous harness or overclaim scope — MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad applies. Specific surfaces in the threat model (harness hallucination, scope overclaim, unsound-stub injection).
- `compliance: [soc2, asvs]` — defaults. No personal data processing; no GDPR.

## The TLA+ pairing (refinement relationship)

TLA+ proves the **protocol** correct by treating actions as atomic. Kani proves those **atomic actions are panic-free and invariant-preserving in real Rust**. The two layers reinforce instead of duplicating:

```
TLA+ action  Send(msg)   →   Rust fn  encode_packet()   →   Kani harness  check_encode_packet
```

When both `tla_required` and `kani_required` are true, the runbook (and the M5 pairing doc) maps each TLA+ action to its Rust kernel and its Kani harness. Kani explicitly **cannot** prove interleavings / concurrency / data-race freedom (the research report is firm on this; `await`/atomics are out of scope) — that is precisely TLA+'s job. So the pairing is complementary, never redundant.

## Inputs

- `~/Downloads/deep-research-report-Kani.md` (local, not repo-tracked; sha256 `8c2c5ef0…873f`) — treated as the research dossier per the user's short-circuit decision. Synthesizes the official Kani book, repo, blog, and verify-rust-std challenge material. Establishes: selective targeting, the candidate-scoring rubric, the run-and-triage ladder, engineering thresholds, and the hard concurrency-out-of-scope limitation.
- This repo's HEAD state — `skills/slo-tla/` (structural sibling to mirror), `skills/slo-architect/`, `skills/slo-plan/`, `skills/slo-execute/`, `skills/slo-verify/`, `skills/slo-retro/`, `docs/slo/templates/runbook-template_v_4_template.md`, `crates/sldo-install/src/install.rs` (`discover_skills`), `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (structural-contract test exemplar), `SECURITY.md`, `docs/ARCHITECTURE.md`.

## Outputs (this skill, this run)

- [docs/ARCHITECTURE.md](../../ARCHITECTURE.md) — new "`/slo-kani` Rust formal-verification skill" subsection under Skill pack with dashed (planned) components.
- [docs/slo/design/kani-verification-stack-decision.md](kani-verification-stack-decision.md)
- [docs/slo/design/kani-verification-interfaces.md](kani-verification-interfaces.md)
- [SECURITY.md](../../SECURITY.md) — merged "`/slo-kani` Rust verification skill — additional rules" section (existing sections preserved).
- [docs/slo/design/kani-verification-threat-model.md](kani-verification-threat-model.md)
- [docs/slo/design/kani-verification-threat-model.slo.json](kani-verification-threat-model.slo.json)
- [docs/slo/design/kani-verification-reversibility.md](kani-verification-reversibility.md)
- [docs/slo/design/kani-verification-code-map.md](kani-verification-code-map.md)
- This file.

## Handoff

Next: `/slo-plan kani-verification` (no TLA — `tla_required: false`).
