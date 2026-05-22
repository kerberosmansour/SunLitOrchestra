# Verification Report — kani Milestone 1

`/slo-kani` skill skeleton + pinned `tools.toml` + prereq cascade + candidate-scoring rubric + structural-contract test. No UI surface (Markdown skill + Rust structural test) — UI cascade skipped.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| Skill is discovered | happy path | `cargo run -p sldo-install -- --dry-run` | pass | `+ …/.claude/skills/slo-kani -> …/skills/slo-kani` in the install plan |
| Frontmatter complete | happy path | `slo_kani_frontmatter_complete` | pass | `name: slo-kani` parsed from YAML frontmatter |
| Honesty contract present | assertion violation | `slo_kani_honesty_and_concurrency_gates_present` | pass | test FAILED earlier when the phrase was absent (fail-closed proven), passes now |
| Concurrency-refusal gate | invalid input | same test | pass | "concurrency is out of scope" present; test caught its earlier absence |
| Pinned toolchain | resource bound | `slo_kani_toolchain_pinned` | pass | `kani-verifier` `version = "0.56.0"`; `latest`/floating rejected by `pins_concrete_version` |
| Output-path allow-list documented | abuse case (`tm-kani-verification-abuse-5`) | `slo_kani_output_paths_constrained` | pass | `docs/slo/verify/` + "target crate" clauses present in SKILL.md |
| Candidate-scoring reference present | happy path | `slo_kani_candidate_scoring_reference_present` | pass | rubric > 400 bytes, names unsafe/score signals |
| Baseline untouched | compatibility | full `cargo test -p sast-verify` | pass | 13 test-files green; `sap_imp_m5_agents` SHA baseline unchanged |

Negative-path note: the structural test's fail-closed behavior was directly observed during execution — it failed (5/5) when the skill files were absent, then again on the missing concurrency phrase, then went green once the contract was satisfied. The gate catches a malformed skill rather than passing vacuously.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none | — | no bugs found |

One in-flight issue was caught and fixed *during* execution (not a post-implementation bug): the concurrency-refusal phrase was initially non-contiguous, the structural test failed, and the SKILL.md gate sentence was corrected before the milestone closed. The test is the regression guard.

## Pass 4 — Security

| Stack | Check | Result | Evidence |
|---|---|---|---|
| Rust | `cargo audit` | pass | exit 0; 118 crate deps scanned, 0 advisories. M1 added no new deps. |
| Rust | dependency-graph delta | pass | no `Cargo.toml`/`Cargo.lock` dependency change in M1 |
| — | DAST | N/A | markdown skill + library structural test; no smoke service / compiled artifact (smoke-service gate) |
| — | biz-public PII scan | N/A | M1 touched no `docs/biz-public/` artifacts |

Threat-model read-side (`kani-verification-threat-model.slo.json`, schema-valid; abuse IDs frozen 1–5): M1's active abuse coverage is `tm-kani-verification-abuse-4` (pinned-toolchain control — asserted by `slo_kani_toolchain_pinned`) and `tm-kani-verification-abuse-5` (output-path allow-list documented — asserted by `slo_kani_output_paths_constrained`). Both controls present and runtime-asserted. The two `accepted_residual: true` rows (bounded-proofs, references-not-pinned) are knowingly accepted — not findings.

## Pass 5 — AI tolerance

N/A — no AI component exercised at runtime in M1. The milestone ships skill *prose* + a deterministic structural test; the LLM-driven harness-authoring behavior the prose governs is exercised in M4. AI tolerance contract row for M1 = deterministic test, no sampling.

## Environment

- macOS (Darwin 25.4.0), Rust workspace, `cargo test -p sast-verify`, `sldo-install --dry-run`.
- `cargo-audit` present; advisory DB reachable.

## Coverage gaps

- The prereq cascade's *runtime* behavior (refusing a missing/mismatched Kani toolchain) is documented in SKILL.md but not executed here — it is exercised for real in M4 (ENG-1 "toolchain absent" scenario). M1 verifies the cascade is *documented and asserted to exist*, not that it fires against a live toolchain. Recorded as intended scope, not a gap to fix.

## Verdict

All M1 BDD scenarios pass at runtime; no bugs found; Pass 4 clean; verified.
