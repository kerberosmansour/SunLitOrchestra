# Verification Report — kani Milestone 5

TLA+↔Kani pairing refinement map + local deep-verification workflow (no CI in v1). No UI surface.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| Refinement map documented | happy path | `pairing_doc_has_refinement_map_and_invariant` | pass | pairing doc has action→fn→harness table + worked example (`check_gcd_contract`, `check_zero_prefix`) |
| Boundary invariant stated | abuse (`tm-…-abuse-2`) | same test | pass | "Kani never claims what TLA+ owns" present |
| Local tiers documented | resource bound | `local_deep_verification_documents_quick_and_deep_tiers` | pass | quick + deep tiers defined |
| Deep-before-release rule | assertion violation | same test | pass | "deep tier must run green before any release tag" |
| Local toolchain pinned | invalid input (`tm-…-abuse-4`) | `local_workflow_uses_pinned_toolchain` | pass | doc references `tools.toml` + pinned `0.67.0`, not `latest` |
| No CI added | compatibility | `no_kani_ci_workflow_added` | pass | no `.github/workflows/*` mentions Kani — v1 decision enforced |
| TLA+ reciprocal note | happy path | `slo_tla_carries_reciprocal_kani_note` | pass | slo-tla SKILL.md references `/slo-kani` |

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none | — | no bugs found |

## Pass 4 — Security

| Stack | Check | Result | Evidence |
|---|---|---|---|
| Rust | `cargo audit` | pass | no new deps |
| Rust | no-CI invariant | pass | `no_kani_ci_workflow_added` green — the v1 "no CI automation" decision is mechanically enforced, not just prose |
| Rust | slo-tla edit additive | pass | full suite green incl. `slo_tm_m2_consumers` (slo-verify phrase test) — the slo-tla reciprocal-note edit broke nothing |
| — | DAST / PII | N/A | doc + skill-prose edits; no service, no `docs/biz-public/` |

## Pass 5 — AI tolerance
N/A — doc/prose + structural assertions; no AI runtime sampling.

## Environment
- macOS; `cargo test -p sast-verify`; `cargo audit`.

## Coverage gaps
- The pairing's *behavioral* value (an actual dual TLA+ + Kani run on a concurrent Rust system) would need a target with both `tla_required` and `kani_required` true. M5 verifies the pairing template + boundary invariant exist and the local workflow is documented; the M4 demo supplies the Kani half of the worked example.

## Verdict
All M5 BDD scenarios pass at runtime; no-CI invariant enforced; Pass 4 clean; verified. **This is the final milestone — the runbook is complete.**
