# Verification Report — slo-threat-model-producer Milestone 2

Milestone: M2 — Idempotent re-run: supersede-don't-renumber + live superseded fixture.
Date: 2026-05-19. Verifier: /slo-verify. Branch: `slo/slo-threat-model-producer`.

Backend/contract milestone (one new dogfood JSON fixture + one new Rust
structural-contract test; SKILL.md/template untouched in M2 — M1 had already
shipped the algorithm prose). No UI → Playwright cascade skipped. The
merged read-side contract still dogfoods correctly here: slug
`slo-threat-model-producer` has no `.slo.json` → **degraded mode** (proceed,
no abuse-ID-stability claim, don't block).

Execute applied the M1 lesson rule (design+exercise every mutation at execute
time, not verify): G1–G5 forced-failure mutations all bit at execute time
with byte-identical restore. Verify re-confirms a representative subset
independently below.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `rerun_algorithm_documented` | happy path | `cargo test -p sast-verify --test slo_tmp_m2_rerun` | pass | 4/4 green; M1 item 8 already contains the full algorithm wording (the test is the regression-guard) |
| `live_superseded_row_validates` | happy path | Same M2 test parses the demo fixture with its own `deny_unknown_fields` structs (ENG-2) | pass | `demo_fixture_strict_parses_with_own_structs` + `demo_fixture_has_live_superseded_row` green; `superseded_by`=`tm-…-abuse-4` with non-empty reason |
| `rerun_prose_absent` | empty state | Already proven during execute BDD-first (3/4 red until demo fixture present) | pass | execute Evidence Log |
| `schema_invalid_superseded_rejected` | invalid input (G3) | **Verify-side re-run:** mutated demo fixture's superseded row to drop `supersede_reason`; observed `demo_fixture_has_live_superseded_row` FAIL; restored byte-identically | pass | guard bites independently of execute's confirmation |
| `renumber_on_rerun_fails` | abuse case (G1) | **Verify-side re-run:** renumbered `tm-…-abuse-1` → `…-99`; observed `demo_fixture_ids_match_demo_slug_and_are_contiguous` FAIL; restored byte-identically | pass | the wedge's whole thesis enforced for the producer too |
| `silent_drop_of_superseded_fails` | abuse case (G2) | Proven at execute (drop `abuse_cases[2]` → contiguity FAIL); verify re-confirms the same guard via G1 above | pass | execute Evidence Log |

Restore integrity (verify-side): demo fixture + SKILL.md SHA-256 round-trip
both matched the backups. No residue.

## Pass 4 — Security

| Check | Stack | Result | Evidence |
|---|---|---|---|
| `cargo audit` | Rust | pass | exit 0; 118 deps; M2 added no dependency |
| `cargo deny check advisories` | Rust | pass | `advisories ok`, exit 0 |
| `semgrep` / `ast-grep` | Rust | N/A | M2 surface is a JSON fixture + a Rust test — no code sink |
| DAST | — | N/A | No compiled artifact / no smoke service |
| Biz-pack PII scan | — | N/A | `docs/biz-public/` not present |
| Read-side contract (self-dogfood) | — | pass | Producer slug has no `.slo.json` → contract proceeds in degraded mode (the merged contract correctly governs this verify run) |

Threat-model linkage: `tm-slo-threat-model-abuse-7` (re-emit renumber) +
`tm-slo-threat-model-abuse-8` (silent drop of superseded) are both enforced
by the demo-fixture contiguity guard, proven to bite at both execute and
verify. No Pass 4 finding; no regression test required.

## Pass 5 — AI tolerance

M2's Contract Block AI-tolerance row is a full contract (producer prose is
followed by `/slo-architect`, an AI agent). Pass 5 RUNS.

- **Accepted variance**: agent wording of an emitted `.slo.json`'s
  `supersede_reason` text may vary; not sampled (no local agent harness).
- **Deterministic boundary**: a changed abuse case MUST become
  `status: superseded` + non-empty `superseded_by` + `supersede_reason`;
  ids never renumber; no silent clobber. Schema/interface didn't drift —
  `slo_tm_m1_schema` 6, `slo_tm_m2_consumers` 5, `slo_tmp_m1_producer` 6,
  `slo_tmp_m2_rerun` 4 all green.
- **Eval evidence**: M2 test (deterministic, single run) + execute-time
  mutate-force-restore covering G1–G5 + verify-side independent re-confirm
  of G1/G3/G5. Bounded.
- **Retry / fallback**: none — non-conformance is a hard test failure.
- **Must-never outcomes**: (a) renumber on re-run — `demo_fixture_ids_match…contiguous`
  bites (G1 ✓); (b) silently drop a superseded row — same guard bites (G2 ✓
  at execute); (c) supersede without `superseded_by`+`supersede_reason` —
  `demo_fixture_has_live_superseded_row` bites (G3 ✓); (d) emit a fixture
  with no superseded row (when the M2 purpose is to demonstrate one) — same
  guard bites (G4 ✓ at execute); (e) weaken the no-silent-clobber algorithm
  prose — `rerun_algorithm_documented_in_step35` bites (G5 ✓).
- **Sample budget**: deterministic single-run structural tests. **pass**.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| — | — | none | — | no bugs found across Passes 1–5 (execute's execute-time mutate-force-restore caught everything that could be caught locally; verify independently re-confirmed) |

## Environment

- OS: macOS (Darwin 25.4.0), arm64.
- Toolchain: cargo/rustc workspace; packages `sast-verify`, `sldo-install`.
- cargo-audit 0.22.1, cargo-deny 0.19.0.
- No browser/Node needed.

## Coverage gaps

- Live agent-runtime emission of an actual `<slug>-threat-model.slo.json`
  with a real supersession event is not deterministically reproducible from
  local CI (same accepted property as the merged wedge + M1 producer).
  Verified structurally: the algorithm prose is present + binds, and a
  hand-authored demo fixture exercises the superseded-row branch end to end.
- No new milestone follow-up. SEC-2 (redaction enforcement) remains the
  wedge-retro parked carry-forward, still pending.
