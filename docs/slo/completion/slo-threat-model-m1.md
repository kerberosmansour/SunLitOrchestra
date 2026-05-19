# Completion Summary — slo-threat-model Milestone 1

## Goal completed

The SLO-owned threat-model JSON contract now exists and is machine-enforced: a
schema reference doc, one real dogfood model serialized losslessly to
`.slo.json` with its abuse-case IDs frozen, and a Rust structural-contract
test that fails on a renumbered/reused ID, a silently dropped (gap) ID, a
missing `classification`, an unknown field, or malformed JSON. The wedge's
core guarantee — abuse-case IDs cannot silently drift — is now a
machine-checked invariant, not prose.

## Files changed

- `references/security/threat-model-schema.md` (new) — SLO-owned schema reference.
- `docs/slo/design/slo-security-embedding-threat-model.slo.json` (new) — dogfood fixture, IDs `tm-slo-sec-abuse-1..8` preserved.
- `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` (new) — structural-contract test.
- `docs/RUNBOOK-SLO-THREAT-MODEL.md` — M1 evidence log, DoD, compatibility, tracker.
- `docs/slo/critique/slo-threat-model.md` — critique disposition (created in the critique phase; M1 added no further edits).

All changed files were on the M1 allow-list. No out-of-scope edits. No
`skills/**`, no `sap_imp_*` test, no `ARCHITECTURE.md`/`SECURITY.md`, no new
crate/dependency, no `skills/slo-threat-model/` directory.

## Tests added

- `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` — 6 tests:
  `schema_doc_has_required_sections`, `fixture_parses_and_conforms`,
  `abuse_cases_are_non_empty`, `frozen_id_invariant_holds`,
  `every_abuse_and_residual_carries_classification`,
  `strict_parse_rejects_unknown_and_malformed`.

## Runtime validations added

- `docs/slo/verify/slo-threat-model-m1.md` — every BDD scenario exercised at
  runtime; all negative scenarios forced by controlled fixture mutation with a
  SHA-256 restore-integrity check; Pass 4 supply-chain clean
  (`cargo audit`/`cargo deny` exit 0); Pass 5 N/A (no AI component in M1).

## Compatibility checks performed

- `cargo test -p sast-verify --test sap_imp_m1_citations` — 5 passed.
- `cargo test -p sast-verify --test sap_imp_m5_agents` — 7 passed (F-ENG-6 pin intact; M1 touched no SKILL.md).
- `git status` — M1 footprint = exactly the allow-listed files.

## Documentation updated

- `references/security/threat-model-schema.md` is the canonical schema
  reference. `docs/ARCHITECTURE.md` and `docs/skill-pack-catalog.md`
  deliberately NOT updated (reality-first; producer deferred — recorded in the
  runbook Documentation Update Table).

## .gitignore changes

- None. The dogfood fixture is `sensitivity: internal` and committed by design
  (it describes the OSS pack's own controls); the two-tier gitignore
  discipline is documented in the schema for downstream
  `confidential`/`restricted` artifacts (no redaction engine in the wedge —
  accepted residual, SEC-2).

## Test artifact cleanup verified

- `git status` in the repo tree shows only intended files. The verification
  fixture backup lived in `/tmp/stm_m1_fixture.bak` (outside the repo) and the
  fixture was restored byte-identically (SHA-256 round-trip verified). No
  stray artifacts.

## Deferred follow-ups

- Producer runbook (`/slo-architect` Step 3.5 → `.slo.json`) — CEO-1, explicit
  M2-retro filing obligation.
- Public-repo redaction/gitignore enforcement — SEC-2, explicit M2-retro
  filing obligation.
- A live `status: superseded` fixture row — for M2 / producer work.

## Known non-blocking limitations

- The fixture is one hand-authored model; no producer emits `.slo.json` yet
  (deliberate — the wedge proves the consumer contract first; CEO-1 reconfirmed
  deferral).
- `cargo fmt --all -- --check` is `blocked-unrelated` by ~30 pre-existing
  drift files outside the M1 allow-list (same blocker the
  agent-operating-contract runbook recorded); the new test file is
  rustfmt-clean.
