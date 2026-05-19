# Verification Report — slo-threat-model-producer Milestone 1

Milestone: M1 — Producer emission contract in `/slo-architect` Step 3.5.
Date: 2026-05-19. Verifier: /slo-verify. Branch: `slo/slo-threat-model-producer`.

Backend/contract milestone (skill prose + one Rust structural test). No UI →
Playwright cascade skipped. The merged read-side contract running inside this
`/slo-verify` itself dogfoods correctly: slug `slo-threat-model-producer` has
no `.slo.json` → **degraded mode** (proceed, no abuse-ID-stability claim,
don't block). Cited threat model = the merged `docs/slo/design/slo-threat-model-threat-model.md`
(same feature family).

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| `producer_contract_documented` | happy path | `cargo test -p sast-verify --test slo_tmp_m1_producer` | pass | 6/6 (after the regression test was added) |
| `template_cites_schema` | invalid input | Removed `references/security/threat-model-schema.md` from a copy of the template + SKILL.md; re-ran | pass | `producer_contract_is_documented_in_step_3_5` FAILED on the mutated state — guard bites; restored byte-identically |
| `existing_e2e_guards_intact` | backward compat | Ran the four `e2e_*` tests on the post-edit tree | pass | `e2e_slo_sec_m1` 11, `e2e_cloud_threat_model_m1` 7, `e2e_fowler_ai_arch_m1` 6, `e2e_fowler_ai_arch_m3` 23 — unchanged from baseline |
| `producer_prose_absent` | empty state | Already proven in execution (BDD-first 3 fail / 2 pass before implementation) | pass | execute Evidence Log |
| `mapping_forbids_renumber` | abuse case (ENG-1) | Mutated merged fixture: renumbered `tm-slo-sec-abuse-3` → `…-99`; re-ran | pass | `merged_fixture_ids_unchanged_and_contiguous` FAILED — ENG-1 structural-proxy guard bites; fixture restored byte-identically |
| `idea_doc_text_cannot_choose_control_fields` | abuse case (SEC-1) | Mutated SKILL.md item 8: weakened `**never** chooses` → `MAYBE chooses`; re-ran | **bug then fixed** | First mutation **did not** fail the loose `producer_prose_mandates_sec1_neutralisation` test — bug recorded; added the tight `sec1_clause_is_specifically_bound` regression (per-file, regex `\bnever\b.*\bchooses\b.*\b(id\|classification\|accepted_residual\|status)\b`). The tight test PASSES on the good state and FAILS on the same SKILL.md-only weakening → guard now bites; restored byte-identically |

Restore integrity (post-Pass-2/3): all three mutated files SHA-256 == backup
SHA-256 (`shasum` round-trip). No residue.

## Pass 4 — Security

| Check | Stack | Result | Evidence |
|---|---|---|---|
| `cargo audit` | Rust | pass | exit 0; 118 deps; M1 added no dependency |
| `cargo deny check advisories` | Rust | pass | `advisories ok`, exit 0 |
| `semgrep` / `ast-grep` | Rust | N/A | M1 surface is skill prose + a Rust test — no code sink introduced |
| DAST | — | N/A | No compiled artifact / no smoke service |
| Biz-pack PII scan | — | N/A | `docs/biz-public/` not present |
| Read-side contract (self-dogfood) | — | pass | Producer slug has no `.slo.json` → contract proceeds in degraded mode (the wedge-merged contract correctly governs this very verify run) |

Threat-model linkage: M1 honored the merged threat-model rows
`tm-slo-threat-model-abuse-7` (renumber — guard G2 proved it bites) and the
producer write-side extension of `tm-slo-threat-model-abuse-1` (SEC-1 — after
the regression-test fix, guard G3 proves it bites). No Pass 4 finding; no
external regression test needed beyond the SEC-1 one already added.

## Pass 5 — AI tolerance

M1's Contract Block AI-tolerance row is a full contract (producer is prose
`/slo-architect` follows). Pass 5 RUNS.

- **Accepted variance**: agent wording of emitted JSON may vary; not sampled
  (no local agent harness) — the deterministic boundary below is what matters.
- **Deterministic boundary**: `.slo.json` MUST conform to the merged frozen
  schema; ids MUST stay 1:1 with the Markdown `tm-<slug>-abuse-N`; provenance
  idiom mandatory. Schema/interface didn't drift — `slo_tm_m1_schema` 6,
  `slo_tm_m2_consumers` 5 still green; merged fixture preserved.
- **Eval evidence**: `slo_tmp_m1_producer` 6/6 (deterministic, single run);
  the four `e2e_*` regression guards 11/7/6/23.
- **Retry / fallback**: none — non-conformance is a hard test failure.
- **Must-never outcomes**: (a) renumber a frozen id — enforced by
  `merged_fixture_ids_unchanged_and_contiguous`, **proven to bite** (G2);
  (b) emit without schema citation — enforced by
  `producer_contract_is_documented_in_step_3_5`, **proven to bite** (G1);
  (c) let user-controlled idea-doc text choose author-controlled fields —
  enforced by `sec1_clause_is_specifically_bound`, **proven to bite** (G3
  after the regression test was added).
- **Sample budget**: deterministic single-run structural tests. **pass**.

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| VERIFY-M1-1 | medium | `idea_doc_text_cannot_choose_control_fields` — the loose `producer_prose_mandates_sec1_neutralisation` assertion was satisfiable by unrelated "never" usages elsewhere in the SKILL.md, so a real weakening of the SEC-1 phrase did not fail the test. Prior lesson recurrence: "a guard must bind the invariant, not the prose around it." | `sec1_clause_is_specifically_bound` added in `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` — per-file regex requiring `never` … `chooses` … `<author-controlled field>` within a sentence | **closed** — bug WAS the too-loose test; the regression test IS the fix. No separate code/prose fix needed (the SKILL.md/template SEC-1 clauses are correct as written). Re-verified: tight test PASSES on good state and FAILS on the SKILL.md-only weakening (G3) |

## Environment

- OS: macOS (Darwin 25.4.0), arm64.
- Toolchain: cargo/rustc workspace; packages `sast-verify`, `sldo-install`.
- cargo-audit 0.22.1, cargo-deny 0.19.0.
- No browser/Node needed.

## Coverage gaps

- Live agent-runtime emission of `.slo.json` by a real `/slo-architect` session
  is not deterministically reproducible from local CI — verified structurally
  (the producer prose is present + binds the invariants + the merged fixture
  conforms with frozen ids) per the skill-pack's nature. Same accepted
  property as the merged wedge.
- The supersession algorithm prose + a live `status: superseded` fixture row
  are M2 work, not an M1 gap.
