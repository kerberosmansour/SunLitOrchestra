# Verification Report — slo-threat-model Milestone 2

Milestone: M2 — Consumer read-side wiring + F-ENG-6 amendment.
Date: 2026-05-19. Verifier: /slo-verify. Branch: `slo/slo-threat-model`.

M2 is a backend/contract milestone (SKILL.md prose + one pinned constant +
one Rust structural test). No UI surface → Playwright cascade skipped.
Runtime verification = forcing the F-ENG-6 lockstep and the ENG-1 additive
guard to bite via controlled mutation, then restoring byte-identically.

**Live dogfood note:** this `/slo-verify` run executed the very read-side
contract M2 added to `slo-verify/SKILL.md`. Slug `slo-threat-model` has no
`docs/slo/design/slo-threat-model-threat-model.slo.json` (producer deferred —
CEO-1), so per the contract this run proceeded in the documented **degraded
mode** (warn, no abuse-ID-stability claim, do not block) rather than
hard-halting. The contract correctly governed itself.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `both_skills_carry_read_side_contract` | happy path | `cargo test -p sast-verify --test slo_tm_m2_consumers` | pass | 5/5 green; both SKILL.md carry read-frozen-json + halt + re-derive + accepted_residual + missing-coverage |
| `feng6_constant_in_lockstep` | invalid input | Appended a drift comment to `slo-critique/SKILL.md` *without* updating the constant; re-ran | pass | `sap_imp_m5_agents::slo_critique_skill_md_unchanged` FAILED (and the M2 `feng6_sha_constant_in_lockstep` mirror) — the lockstep guard **bites**; restored byte-identically |
| `missing_json_uses_documented_degraded_mode` | empty state | Live: slug `slo-threat-model` has no `.slo.json`; observed this verify run's own behavior | pass | Verify proceeded in degraded mode per the contract text — did not block, made no ID-stability claim (self-dogfood) |
| `invalid_json_hard_halts` | dependency failure | Cross-referenced M1's `slo_tm_m1_schema` (an invalid `.slo.json` is a hard `serde_json` parse error) + the M2 contract text mandating hard halt | pass | M1 verify already proved an invalid fixture hard-fails; M2 SKILL.md prescribes hard halt, no silent re-derive fallback |
| `instruction_shaped_field_not_executed` | abuse case | Confirmed the M1 fixture carries instruction-shaped strings (`tm-slo-sec-abuse-3` attacker text) and the SEC-1 fence rule is specified + test-asserted in both SKILL.md | pass | `both_skills_specify_the_fence_rule` green; `~~~text` literal-fence rule mandated, "never instruction/prompt" stated; no deterministic agent-exec harness exists (structural per skill guidance) |
| `critique_edit_is_additive_only` | backward compat | Removed `## Rotation order` from a `slo-critique/SKILL.md` copy; re-ran | pass | `slo_tm_m2_consumers::critique_edit_is_additive_only` FAILED — the ENG-1 additive guard **bites**; restored byte-identically |

Restore integrity: `slo-critique/SKILL.md` SHA-256 after all mutations ==
pinned `9e31b7dd…f26f8`; `sap_imp_m5_agents.rs` `diff -q` clean. No residue.

Regression (no milestone left behind): `slo_tm_m1_schema` 6/6,
`sap_imp_m5_agents` 7/7 (new constant), `sap_imp_m1_citations` 5/5,
`e2e_sec_exec_m3` 3/3 (the recon-caught `slo-verify` reader — append-only
edit preserved its required substrings).

## Pass 4 — Security

| Check | Stack | Result | Evidence |
|---|---|---|---|
| `cargo audit` | Rust | pass | exit 0; 118 deps; M2 changed no dependency |
| `cargo deny check advisories` | Rust | pass | `advisories ok`, exit 0 |
| `semgrep` / `ast-grep` | Rust | N/A | M2 surface is SKILL.md prose + one test + one constant — no code sink introduced |
| DAST | — | N/A | no compiled artifact / no smoke service (contract + prose milestone) |
| Biz-pack PII scan | — | N/A | `docs/biz-public/` not present |
| Read-side contract (self-dogfood) | — | pass | slug `slo-threat-model` has no `.slo.json` → degraded mode exercised live; the contract did not block its own verify and made no false ID-stability claim |

Pass 4 threat-model linkage: `tm-slo-threat-model-abuse-1` (instruction-shaped
field) — control is the SEC-1 `~~~text` fence rule, now specified in both
consumer SKILL.md and asserted by `both_skills_specify_the_fence_rule`
(mitigated→eliminated-by-construction for the documented surface, per the
critique SEC-1 resolution). No Pass 4 finding; no regression test required.

## Pass 5 — AI tolerance

M2's Contract Block AI-tolerance row is a full contract (the `.slo.json` is
untrusted input to agent-run consumers; `ai_component: true`) — Pass 5 RUNS.

- **Accepted variance**: agent finding-wording may vary; not sampled (no agent
  run in this structural milestone) — the contract's value is the
  deterministic boundary, verified below.
- **Deterministic boundary**: the read-vs-re-derive decision is deterministic
  (present+valid ⇒ read; absent ⇒ degraded mode; present+invalid ⇒ hard
  halt). Schema/interface did not drift — `slo_tm_m1_schema` 6/6 green.
- **Eval evidence**: `slo_tm_m2_consumers` (deterministic, single run)
  asserts the halt-not-re-derive + SEC-1 fence + degraded/hard-halt language
  in both SKILL.md. Bounded; no sampling.
- **Retry / fallback**: no silent re-derive fallback exists in the contract
  text; the only non-halt path is the explicitly documented degraded mode.
- **Must-never outcomes**: (a) never renumber a frozen id — enforced by M1's
  `frozen_id_invariant_holds`, proven to bite in M1 verify; (b) never execute
  instruction-shaped `.slo.json` text — enforced by the SEC-1 fence rule,
  specified + asserted. Neither occurred.
- **Sample budget**: deterministic structural tests, single run — bounded.
  No unbounded sampling. **pass**.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none | — | no bugs found across Passes 1–5 |

## Environment

- OS: macOS (Darwin 25.4.0), arm64.
- Toolchain: cargo/rustc workspace; packages `sast-verify`, `sldo-install`.
- cargo-audit 0.22.1, cargo-deny 0.19.0.
- No browser/Node needed (no UI surface).

## Coverage gaps

- Agent-runtime execution of the read-side contract (an actual `/slo-critique`
  / `/slo-verify` session honoring the fence rule against a hostile
  `.slo.json`) is not deterministically reproducible from local CI — verified
  structurally (contract present + test-asserted) per the skill's own
  guidance, plus the live self-dogfood degraded-mode observation. Recorded;
  not an M2 gap (no local harness exists, by design).
