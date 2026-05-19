# SLO Threat-Model Producer (AI-First Runbook v4)

> **Purpose**: Close the loop the read-side wedge (#90, merged) deliberately left open — extend `/slo-architect` Step 3.5 to ALSO emit `docs/slo/design/<slug>-threat-model.slo.json` conforming to the now-merged `references/security/threat-model-schema.md`, so a live sprint gets the machine-readable contract automatically instead of a hand-authored fixture.
> **Audience**: AI coding agents first, humans second.
> **Core philosophy**: The schema is frozen and merged — this is a *feature addition to an already-designed system*. Producer prose is **append-only** to `/slo-architect`. Idempotent re-run = supersede-don't-renumber, never silent clobber. Provenance matches repo precedent. No new schema, no new skill directory.
> **Prerequisite reading**: [references/security/threat-model-schema.md](../references/security/threat-model-schema.md), [docs/slo/design/slo-threat-model-overview.md](slo/design/slo-threat-model-overview.md), [docs/slo/design/slo-threat-model-interfaces.md](slo/design/slo-threat-model-interfaces.md), [docs/slo/design/slo-threat-model-reversibility.md](slo/design/slo-threat-model-reversibility.md), [docs/slo/design/slo-threat-model-code-map.md](slo/design/slo-threat-model-code-map.md), [skills/slo-architect/references/threat-model-template.md](../skills/slo-architect/references/threat-model-template.md), [docs/slo/lessons/slo-threat-model-m2.md](slo/lessons/slo-threat-model-m2.md), [LESSONS-BACKLOG.md](../LESSONS-BACKLOG.md) (Row 1 = CEO-1, the scope this runbook delivers).

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `slo-threat-model-producer` |
| Project name | `SunLitOrchestra` |
| Primary stack | Markdown skill prose + JSON fixture + Rust structural-contract tests |
| Primary package/app names | `skills/slo-architect/`, `docs/slo/design/`, `xtasks/sast-verify` |
| Prefix for tests and lesson files | `slo-threat-model-producer` |
| Default unit test command | `cargo test -p sast-verify --test slo_tmp_m1_producer` |
| Default integration/BDD test command | `cargo test -p sast-verify --test slo_tmp_m1_producer --test slo_tmp_m2_rerun` |
| Default E2E/runtime validation command | `N/A - skill-prose + fixture + structural tests; no runtime service` |
| Default build/boot command | `cargo test -p sast-verify --no-run` |
| Default formatter command | `cargo fmt --all -- --check` |
| Default static analysis / lint command | `cargo clippy -p sast-verify --tests -- -D warnings` |
| Default dependency / security audit command | `N/A - no dependency changes allowed` |
| Default debugger or state-inspection tool | `rg`; `jq . <fixture>`; Rust test failure output |
| Allowed new dependencies by default | `none` (`serde_json` is already a `sast-verify` dependency) |
| Schema/config migration allowed by default | `no` — `slo_schema_version` stays `0.1.0`; the schema is frozen by #90 |
| Public interfaces stable by default | `yes` |
| Master GitHub issue | [#67](https://github.com/kerberosmansour/SunLitOrchestra/issues/67) (umbrella; the dedicated CEO-1 issue is parked in `LESSONS-BACKLOG.md` Row 1 pending a `gh` permission grant) |

### Public Interfaces That Must Remain Stable

- `references/security/threat-model-schema.md` shape and the `tm-<slug>-abuse-N` frozen-ID convention (merged in #90; this runbook only *produces* conforming files, never changes the schema).
- The existing `/slo-architect` Step 3.5 Markdown emission behavior and every substring asserted by `e2e_slo_sec_m1`, `e2e_cloud_threat_model_m1`, `e2e_fowler_ai_arch_m1`, `e2e_fowler_ai_arch_m3` — the producer prose is **append-only**.
- The merged read-side consumer contract in `/slo-critique` and `/slo-verify` (untouched here; it auto-consumes whatever the producer emits).
- Existing skill names; **no new `skills/` directory**.

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Producer emission contract in Step 3.5 | `done` | 2026-05-19 | 2026-05-19 | `docs/slo/lessons/slo-threat-model-producer-m1.md` | M1 shipped the `.slo.json` producer contract in `/slo-architect` Step 3.5 + the template's serialization mapping (append-only; four `e2e_*` guards green); SEC-1 producer-side neutralisation bound by a tight per-file regression after the loose first-pass assertion was caught by mutate-force-restore. Completion: `docs/slo/completion/slo-threat-model-producer-m1.md`. |
| 2 | Idempotent re-run: supersede-don't-renumber + live superseded fixture | `done` | 2026-05-19 | 2026-05-19 | `docs/slo/lessons/slo-threat-model-producer-m2.md` | M2 closed the wedge-retro coverage gap with a live `status: superseded` demo fixture + a structural test that owns its strict-parse (ENG-2) and proves G1–G5 mutations bite; no SKILL.md edits needed (M1's item 8 already shipped the algorithm wording — honest scope contraction). Completion: `docs/slo/completion/slo-threat-model-producer-m2.md`. |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/slo-threat-model-producer-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/slo-threat-model-producer-m<N>.md -->

---

## 3. End-To-End Architecture Diagram

```text
/slo-architect run on a feature <slug>
  | Step 3.5 (solid: emits Markdown today)
  v
docs/slo/design/<slug>-threat-model.md
  | (dashed, M1) ALSO serialize, conforming to the merged schema
  v
docs/slo/design/<slug>-threat-model.slo.json   [frozen tm-<slug>-abuse-N]
  |                                  ^
  | (solid, merged #90) read by      | (dashed, M2) re-run: diff,
  | /slo-critique + /slo-verify      | supersede-don't-renumber,
  | (loop now CLOSES automatically)  | surface diff, no silent clobber
  v                                  |
xtasks/sast-verify/tests/
  slo_tmp_m1_producer.rs (M1)  — producer prose + idempotent re-emit of the
                                 merged dogfood fixture preserves ids
  slo_tmp_m2_rerun.rs   (M2)  — supersession algorithm prose + a live
                                 status:superseded fixture row

Legend: solid = exists/merged; dashed = added by this runbook.
        Append-only edits to /slo-architect; four e2e_* tests guard the
        existing Step 3.5 substrings (regression set).
```

### Component Summary Table

| Component | Responsibility | Existing/New/Changed | Milestone | Key Interfaces |
|---|---|---|---|---|
| `skills/slo-architect/SKILL.md` Step 3.5 | Gains "also emit `<slug>-threat-model.slo.json`" contract | Changed (append-only) | M1 | Producer prose; guarded by 4 e2e_* tests |
| `skills/slo-architect/references/threat-model-template.md` | Gains the `.slo.json` serialization mapping + provenance idiom | Changed (append-only) | M1 | Serialization spec; guarded by `e2e_slo_sec_m1` |
| `docs/slo/design/slo-security-embedding-threat-model.slo.json` | Re-validated as the idempotent-emission proof (ids preserved) | Existing (merged #90) | M1 | Frozen-ID fixture |
| `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` | Asserts producer prose + idempotent re-emit preserves ids | New | M1 | `cargo test -p sast-verify` |
| Step 3.5 re-run algorithm | supersede-don't-renumber + diff-surface + no silent clobber | Changed (append-only) | M2 | Idempotency prose |
| `docs/slo/design/<demo>-threat-model.slo.json` superseded fixture | Live `status: superseded` row (closes wedge retro coverage gap) | New | M2 | Superseded-row fixture |
| `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs` | Asserts supersession prose + live superseded-row invariants | New | M2 | `cargo test -p sast-verify` |

### Data Flow Summary

| Flow | From | To | Mechanism | Bounded? | Failure Mode | Milestone |
|---|---|---|---|---|---|---|
| Producer emission | Markdown threat model + inputs | `<slug>-threat-model.slo.json` | Step 3.5 prose contract; schema-conformant | yes | non-conforming JSON; missing provenance | M1 |
| Idempotent re-emit | prior `.slo.json` | regenerated `.slo.json` | diff + supersede-don't-renumber | yes | silent renumber/clobber (the original failure) | M2 |
| Loop closure | producer output | merged `/slo-critique`+`/slo-verify` | already-merged read-side contract | yes | none new — consumer side frozen by #90 | M1 |

---

## 4. TLA+ Section

N/A — `tla_required: false`. Sequential file I/O plus deterministic structural tests. No shared mutable state, distributed ordering, leases, or recovery protocol. Supersede-don't-renumber is a deterministic emission rule, not a concurrency protocol. Same justification as the merged wedge (`docs/slo/design/slo-threat-model-overview.md`).

---

## 5. Milestone 1 — Producer Emission Contract In Step 3.5

### Goal

At the end of M1, `/slo-architect` Step 3.5 documents that it ALSO emits `docs/slo/design/<slug>-threat-model.slo.json` conforming to `references/security/threat-model-schema.md` (serializing the Markdown structure, provenance idiom, classification/sensitivity), the edits are append-only and break none of the four existing `e2e_*` guards, and a structural test proves the documented producer process re-emits the merged dogfood fixture with its frozen `tm-slo-sec-abuse-1..8` ids preserved (no renumber).

### Context

The read-side wedge (#90, merged) shipped the schema, a hand-authored dogfood fixture, and the consumer contract. No producer exists, so live sprints still get Markdown-only. This milestone adds the producer *specification* to Step 3.5. `slo-architect/SKILL.md` is NOT SHA-pinned (recon confirmed) but IS asserted by `e2e_slo_sec_m1`, `e2e_cloud_threat_model_m1`, `e2e_fowler_ai_arch_m1`, `e2e_fowler_ai_arch_m3` — every existing asserted substring must survive (append-only).

### Important Design Rule

Append-only. Do not reflow or delete any existing Step 3.5 / template prose. The producer is an *additional* emission alongside the existing Markdown one, conforming to the frozen schema — no schema change, `slo_schema_version` stays `0.1.0`.

**SEC-1 producer-side neutralisation (the write-side analogue of the merged consumer SEC-1).** The merged threat model modelled only the consumer (read) side. The producer is a new write-side surface: `/slo-architect` (an LLM) serialises user-controlled idea-doc Top-risks / abuse text into the `.slo.json`. The producer prose MUST require: (1) emit via a structural JSON serializer (or escape equivalently) so a crafted idea-doc string cannot break out of a JSON string; (2) idea-doc/user-controlled text is rendered ONLY into descriptive string fields and NEVER chooses `id`, `classification`, `accepted_residual`, or `status` — those are author-controlled. Without (2), a schema-valid but semantically-poisoned file (e.g. attacker-chosen `accepted_residual: true` suppressing a real gap) passes `deny_unknown_fields` and poisons every downstream consumer.

### Refactor Budget

No refactor permitted beyond direct implementation.

### Contract Block

| Contract Row | Value |
|---|---|
| Inputs | `references/security/threat-model-schema.md`; `skills/slo-architect/SKILL.md`; `skills/slo-architect/references/threat-model-template.md`; the merged fixture `docs/slo/design/slo-security-embedding-threat-model.slo.json`; `xtasks/sast-verify/tests/slo_tm_m1_schema.rs`; the four `e2e_*` guard tests |
| Outputs | Append-only producer prose in Step 3.5 + threat-model-template.md; `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs`; closeout artifacts |
| Interfaces touched | `/slo-architect` Step 3.5 prose (append-only); new Rust test |
| Files allowed to change | `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md`, `docs/slo/critique/slo-threat-model-producer.md`, `skills/slo-architect/SKILL.md`, `skills/slo-architect/references/threat-model-template.md`, `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs`, `docs/slo/verify/slo-threat-model-producer-m1.md`, `docs/slo/lessons/slo-threat-model-producer-m1.md`, `docs/slo/completion/slo-threat-model-producer-m1.md` |
| Files to read before changing | `skills/slo-architect/SKILL.md`, `skills/slo-architect/references/threat-model-template.md`, `references/security/threat-model-schema.md`, `crates/sldo-install/tests/e2e_slo_sec_m1.rs`, `crates/sldo-install/tests/e2e_cloud_threat_model_m1.rs`, `crates/sldo-install/tests/e2e_fowler_ai_arch_m1.rs`, `crates/sldo-install/tests/e2e_fowler_ai_arch_m3.rs`, `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` |
| New files allowed | `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs`, `docs/slo/critique/slo-threat-model-producer.md`, `docs/slo/verify/slo-threat-model-producer-m1.md`, `docs/slo/lessons/slo-threat-model-producer-m1.md`, `docs/slo/completion/slo-threat-model-producer-m1.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` (schema frozen at `0.1.0` by #90) |
| Compatibility commitments | Every substring asserted by `e2e_slo_sec_m1`, `e2e_cloud_threat_model_m1`, `e2e_fowler_ai_arch_m1/m3` survives; existing Markdown emission behavior unchanged; no `skills/` dir; no schema change |
| Exemplar code to copy | `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` for serde strict-parse + frozen-ID assertions; `crates/sldo-install/tests/e2e_slo_sec_m1.rs` for "Step 3.5 cites X" prose assertions |
| Anti-exemplar code not to copy | Reflowing existing Step 3.5 prose (breaks the four e2e_* guards); inventing a new schema/field (frozen by #90); hand-rolled JSON parsing |
| Refactoring discipline | N/A — append-only prose + new test |
| AI tolerance contract | The producer is prose `/slo-architect` (an AI agent) follows to emit JSON. Accepted variance: agent wording of the emitted Markdown may vary. Deterministic boundary: the `.slo.json` MUST conform to the frozen schema, MUST preserve existing `tm-<slug>-abuse-N` ids, MUST carry the provenance idiom. Eval evidence: `slo_tmp_m1_producer` re-validates the merged dogfood fixture against the documented process (ids preserved). Retry/fallback: none — non-conformance is a hard test failure. Must-never: never renumber an existing id; never emit without provenance. Bounded sample budget: deterministic single-run structural test. Cites `references/ai-tolerance-contract.md`. |
| Data classification | Public (skill prose + a public schema-conformant fixture about the OSS pack) |
| Proactive controls in play | OWASP C5 Validate Inputs — emitted JSON strict-validates against the merged schema; OWASP C4 Encode/Escape — **SEC-1: user-controlled idea-doc strings serialized via a structural serializer; never choose `id`/`classification`/`accepted_residual`/`status`**; OWASP C1 — provenance (producing-skill SKILL.md git sha + input git blob shas) mandatory by the producer prose, test-asserted |
| Abuse acceptance scenarios | `tm-slo-threat-model-abuse-1` (SEC-1: crafted idea-doc text poisons the emitted JSON), `tm-slo-threat-model-abuse-7` (re-emit renumbers an id), `tm-slo-threat-model-abuse-9` (emit without/with-stale provenance) — BDD rows below; merged threat-model rows this producer must honor (abuse-1 is the producer write-side extension of the consumer SEC-1) |
| Resource bounds introduced/changed | Producer prose ≤ 40 added nonblank lines in Step 3.5 + ≤ 40 in the template; emitted fixture is a bounded static file |
| Invariants/assertions required | (a) Step 3.5 / template contains the producer contract: "emit `<slug>-threat-model.slo.json`", cites `references/security/threat-model-schema.md`, states the provenance idiom and supersede-don't-renumber rule; (b) the four `e2e_*` guards still pass (asserted substrings intact); (c) **ENG-1 (no over-claim — no producer executes locally):** `slo_tmp_m1_producer` asserts the threat-model-template.md serialization-mapping section maps the Markdown `tm-<slug>-abuse-N` column 1:1 onto the JSON `id` field and explicitly forbids renumber, AND that the merged dogfood fixture still strict-conforms with ids `tm-slo-sec-abuse-1..8` present and contiguous. This is a structural proxy for idempotence, not a live re-emission proof; (d) **SEC-1 (producer-side injection):** the producer prose mandates user-controlled idea-doc strings are serialized via a structural JSON serializer (or escaped equivalently) and that idea-doc/Top-risks text NEVER chooses `id`, `classification`, `accepted_residual`, or `status` (author-controlled fields); `slo_tmp_m1_producer` asserts this clause is present |
| Debugger / inspection expectation | `rg` Step 3.5 anchors; `jq` the fixture; Rust test output |
| Static-analysis gates | `cargo fmt --all -- --check`; `cargo clippy -p sast-verify --test slo_tmp_m1_producer -- -D warnings`; `cargo test -p sast-verify --test slo_tmp_m1_producer --test slo_tm_m1_schema`; regression `cargo test -p sldo-install --test e2e_slo_sec_m1 --test e2e_cloud_threat_model_m1 --test e2e_fowler_ai_arch_m1 --test e2e_fowler_ai_arch_m3` |
| Reversibility / rollback path | Revert the appended Step 3.5/template prose + delete the M1 test; the merged schema/fixture/consumer contract are untouched and independent |
| Forbidden shortcuts | Do not reflow existing Step 3.5 prose; do not change the schema or `slo_schema_version`; do not edit the four `e2e_*` tests; do not hand-roll JSON; do not create a `skills/` dir; do not weaken the merged `slo_tm_m1_schema` test |

### Out Of Scope / Must Not Do

- Do not implement the re-run/supersession algorithm prose (that is M2).
- Do not touch `/slo-critique` or `/slo-verify` (merged consumer side is frozen).
- Do not change `references/security/threat-model-schema.md`.
- Do not file/cleanup the parked LESSONS-BACKLOG GitHub items here.

### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-architect/SKILL.md` | Append producer contract to Step 3.5 (also emit `.slo.json`, cite schema, provenance idiom) |
| `skills/slo-architect/references/threat-model-template.md` | Append the Markdown→`.slo.json` serialization mapping section |
| `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` | New: producer-prose assertions + idempotent-re-emit fixture proof |
| `docs/slo/critique/slo-threat-model-producer.md` | Critique artifact |
| `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` | M1 evidence/tracker |
| `docs/slo/verify/slo-threat-model-producer-m1.md` | M1 verification report |
| `docs/slo/lessons/slo-threat-model-producer-m1.md` | M1 lessons |
| `docs/slo/completion/slo-threat-model-producer-m1.md` | M1 completion |

### Step-By-Step

1. Read the four `e2e_*` guard tests and record every substring they assert about Step 3.5 / threat-model-template.md.
2. Add the failing M1 test (`slo_tmp_m1_producer`): asserts (a) the producer prose contract; (b) ENG-1 — the template serialization mapping maps `tm-<slug>-abuse-N`→`id` 1:1 and forbids renumber, and the merged fixture strict-conforms with `tm-slo-sec-abuse-1..8` contiguous (structural proxy, no live re-emission); (c) SEC-1 — the producer-side neutralisation clause is present.
3. Run it; confirm red (producer prose absent).
4. Append the producer contract to Step 3.5 and the serialization mapping to threat-model-template.md (append-only), INCLUDING the SEC-1 clause (structural serializer; user text never chooses `id`/`classification`/`accepted_residual`/`status`).
5. Run `slo_tmp_m1_producer` to green; run the four `e2e_*` regressions + `slo_tm_m1_schema`.
6. Scoped clippy + formatter (record unrelated drift honestly).
7. Fill the M1 Evidence Log.

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `producer_contract_documented` | happy path | Step 3.5 + template edited | `slo_tmp_m1_producer` reads them | Producer contract present: emits `.slo.json`, cites the schema, states provenance idiom + supersede-don't-renumber | `cargo test -p sast-verify --test slo_tmp_m1_producer` |
| `existing_e2e_guards_intact` | backward compat | Step 3.5 edited append-only | the four `e2e_*` tests run | All asserted substrings still present; all pass | `cargo test -p sldo-install --test e2e_slo_sec_m1 --test e2e_cloud_threat_model_m1 --test e2e_fowler_ai_arch_m1 --test e2e_fowler_ai_arch_m3` |
| `template_cites_schema` | invalid input | template missing the schema citation | `slo_tmp_m1_producer` checks the mapping section | Test fails until the template cites `references/security/threat-model-schema.md` | same M1 test |
| `producer_prose_absent` | empty state | Step 3.5 has no producer contract yet | M1 test runs pre-implementation | Fails with an explicit "producer contract missing" message (BDD-first red) | same M1 test |
| `mapping_forbids_renumber` | abuse case | The template serialization-mapping section is present | `slo_tmp_m1_producer` reads it | It maps Markdown `tm-<slug>-abuse-N` 1:1 onto the JSON `id` and explicitly forbids renumber; the merged fixture still conforms with `tm-slo-sec-abuse-1..8` contiguous — structural proxy for `tm-slo-threat-model-abuse-7` (ENG-1: no live re-emission claimed) | same M1 test |
| `provenance_idiom_mandated` | abuse case | producer prose omits provenance | M1 test scans the contract | Fails — provenance (producing-skill SKILL.md git sha + input git blob shas) is mandatory, realizing `tm-slo-threat-model-abuse-9` | same M1 test |
| `idea_doc_text_cannot_choose_control_fields` | abuse case | A crafted idea-doc Top-risks string tries to set `accepted_residual`/`classification`/`id` | `slo_tmp_m1_producer` scans the producer prose | The prose mandates structural serialization + that user text never chooses author-controlled fields — realizing `tm-slo-threat-model-abuse-1` (SEC-1 producer write-side) | same M1 test |

### Regression Tests

- `cargo test -p sldo-install --test e2e_slo_sec_m1 --test e2e_cloud_threat_model_m1 --test e2e_fowler_ai_arch_m1 --test e2e_fowler_ai_arch_m3`
- `cargo test -p sast-verify --test slo_tm_m1_schema --test slo_tm_m2_consumers`

### Compatibility Checklist

- [x] All four `e2e_*` guard tests still pass (asserted substrings intact) — 11/7/6/23, unchanged from baseline.
- [x] Existing Markdown emission behavior unchanged (append-only; item 7 + all cited substrings intact).
- [x] `references/security/threat-model-schema.md` byte-unchanged (not in the M1 footprint).
- [x] No new crate; no `skills/` directory.
- [x] Merged `slo_tm_m1_schema` (6) / `slo_tm_m2_consumers` (5) still green.

### E2E Runtime Validation

N/A — skill prose + a structural test. The "runtime" is `/slo-architect` itself, exercised structurally (the contract must be present + the merged fixture must re-validate). No deterministic local agent harness.

### Smoke Tests

- `rg -n "slo.json|threat-model-schema" skills/slo-architect/SKILL.md skills/slo-architect/references/threat-model-template.md` shows the producer contract.
- `jq . docs/slo/design/slo-security-embedding-threat-model.slo.json` still parses; ids `tm-slo-sec-abuse-1..8` unchanged.
- Read Step 3.5 and confirm the addition is appended, not interleaved into existing prose.

### Evidence Log

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene | branch before/after; dirty tree | task branch, not main | before `main` (post-#90-merge); created `slo/slo-threat-model-producer`; dirty = untracked producer runbook/critique created on main, preserved by the branch switch; no remediation needed | `pass` | One feature branch for the producer runbook |
| Guard-substring recon | Read the four `e2e_*` tests | every asserted Step 3.5 substring recorded | `e2e_slo_sec_m1` asserts (all `.contains`): `\n7. **` anchor (line 42), `references/SECURITY-md-template.md`, `references/threat-model-template.md`, `## Top risks`, STRIDE/Compliance/SOC2/ASVS/ai_component in the template — all substring, none byte/line-count → append-only safe | `pass` | M1-lesson "rg every reader" applied; baseline 11/7/6/23 green |
| New M1 test fails first | `cargo test -p sast-verify --test slo_tmp_m1_producer` | red — producer prose absent | 3 failed (producer_contract, template_mapping, sec1) + 2 passed (append_only + merged_fixture regression-guards already green) | `pass` | Red for the intended reasons |
| Producer prose appended | Edit Step 3.5 (new item 8) + template (new mapping section), append-only | contract present; existing prose intact | Item 8 appended after item 7 (item 7 `\n7. **` anchor untouched); `## SLO JSON companion serialization mapping` appended after the template's last bullet | `pass` | No existing prose reflowed |
| New M1 test passes | `cargo test -p sast-verify --test slo_tmp_m1_producer` | green | 5 passed; 0 failed | `pass` | |
| e2e_* regression | the four `e2e_*` tests | all pass (substrings intact) | `e2e_slo_sec_m1` 11, `e2e_cloud_threat_model_m1` 7, `e2e_fowler_ai_arch_m1` 6, `e2e_fowler_ai_arch_m3` 23 — unchanged from baseline | `pass` | Append-only preserved every asserted substring |
| schema/consumer regression | `cargo test -p sast-verify --test slo_tm_m1_schema --test slo_tm_m2_consumers` | pass | slo_tm_m1_schema 6; slo_tm_m2_consumers 5 | `pass` | Merged wedge contract intact |
| Scoped clippy | `cargo clippy -p sast-verify --test slo_tmp_m1_producer -- -D warnings` | passes | Failed first on a dead `SCHEMA_DOC` const; removed it; passes clean | `pass` | Fixed within the M1 allow-list |
| Formatter | `cargo fmt --all -- --check` | passes or unrelated blocker recorded | `slo_tmp_m1_producer.rs` rustfmt-clean (exit 0); edited files are Markdown (rustfmt N/A). Whole-tree check still drifts on the same ~30 pre-existing unrelated files outside the allow-list | `blocked-unrelated` | Same blocker as the merged wedge / agent-operating-contract |
| Compatibility check | `git status --short` | only M1 allow-list files changed | Footprint = `slo-architect/SKILL.md`, `slo-architect/references/threat-model-template.md`, `RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md`, `slo/critique/slo-threat-model-producer.md`, `slo_tmp_m1_producer.rs` — all M1 allow-listed | `pass` | No schema change; no new crate/skill dir |

### Definition Of Done

- [x] Step 3.5 + threat-model-template.md document the `.slo.json` producer contract (cite schema, provenance idiom, supersede-don't-renumber) — append-only.
- [x] SEC-1: producer prose mandates structural-serializer/escaped emission and that user-controlled idea-doc text never chooses `id`/`classification`/`accepted_residual`/`status`; `slo_tmp_m1_producer::producer_prose_mandates_sec1_neutralisation` asserts the clause.
- [x] The four `e2e_*` guards still pass; existing Markdown emission unchanged.
- [x] ENG-1: `slo_tmp_m1_producer` failed before (3/5), passes after (5/5); asserts the template maps `tm-<slug>-abuse-N`→`id` 1:1 and forbids renumber, and the merged fixture still conforms (`tm-slo-sec-abuse-1..8` contiguous) — a structural proxy, NOT a live-re-emission claim.
- [x] Schema byte-unchanged; no new crate/skill dir.
- [ ] Merged `slo_tm_m1_schema` / `slo_tm_m2_consumers` still green.

---

## 6. Milestone 2 — Idempotent Re-Run: Supersede-Don't-Renumber + Live Superseded Fixture

### Goal

At the end of M2, `/slo-architect` Step 3.5 documents the `.slo.json` re-run algorithm — detect existing file, diff, mark changed abuse cases `status: superseded` with `superseded_by` + `supersede_reason` (never renumber, never silently drop), surface the diff and prompt overwrite/merge/skip (mirroring the existing Markdown no-silent-clobber rule) — and a committed dogfood fixture carries a real `status: superseded` row that a structural test validates (closing the wedge-retro coverage gap where no live superseded row existed).

### Context

The merged `slo_tm_m1_schema` test already enforces *that* a `superseded` row must carry `superseded_by` + `supersede_reason`, but no committed fixture exercises it (wedge M1/M2 lessons "missing tests" + verify coverage gap). M2 documents the producer's re-run behavior and ships the first live superseded-row fixture so the invariant is exercised against real data, not just logic.

### Important Design Rule

The re-run algorithm prose is append-only to Step 3.5 and must explicitly reuse the *existing* "surface the diff, prompt overwrite/merge/skip, no silent clobber" wording pattern already used for the Markdown threat model — consistency, not a parallel mechanism.

### Refactor Budget

No refactor permitted beyond direct implementation.

### Contract Block

| Contract Row | Value |
|---|---|
| Inputs | M1 producer prose + `slo_tmp_m1_producer`; `docs/slo/lessons/slo-threat-model-producer-m1.md`; the merged `slo_tm_m1_schema` superseded-row rule; `skills/slo-architect/SKILL.md` Step 3.5 |
| Outputs | Append-only re-run/supersession prose in Step 3.5; one new `<demo>-threat-model.slo.json` fixture with a live `status: superseded` row; `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs`; closeout artifacts |
| Interfaces touched | `/slo-architect` Step 3.5 prose (append-only); new fixture; new test |
| Files allowed to change | `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md`, `docs/slo/critique/slo-threat-model-producer.md`, `skills/slo-architect/SKILL.md`, `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json`, `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs`, `docs/slo/verify/slo-threat-model-producer-m2.md`, `docs/slo/lessons/slo-threat-model-producer-m2.md`, `docs/slo/completion/slo-threat-model-producer-m2.md` |
| Files to read before changing | `skills/slo-architect/SKILL.md` (Step 3.5 + the existing Markdown no-silent-clobber wording), `references/security/threat-model-schema.md`, `xtasks/sast-verify/tests/slo_tm_m1_schema.rs`, `docs/slo/lessons/slo-threat-model-producer-m1.md`, the four `e2e_*` guard tests |
| New files allowed | `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json`, `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs`, `docs/slo/verify/slo-threat-model-producer-m2.md`, `docs/slo/lessons/slo-threat-model-producer-m2.md`, `docs/slo/completion/slo-threat-model-producer-m2.md` |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | Append-only Step 3.5; four `e2e_*` guards intact; merged `slo_tm_m1_schema` unchanged and still green (it loads only its hardcoded original fixture — ENG-2: it does NOT and is not expected to read the new demo fixture); `slo_tmp_m2_rerun` itself strict-parses the new demo fixture; no schema change |
| Exemplar code to copy | The existing Step 3.5 Markdown re-run wording ("surface the diff … overwrite, merge, or skip"); `xtasks/sast-verify/tests/slo_tm_m1_schema.rs` — **copy its serde structs + `frozen_id_invariant_holds` logic into `slo_tmp_m2_rerun` so it strict-parses the demo fixture itself** (ENG-2: the merged test only loads its own hardcoded fixture) |
| Anti-exemplar code not to copy | A parallel diff-store mechanism (the design says reuse the surface-and-prompt rule, not a `docs/slo/threat-model-diffs/` tree); renumbering on re-run |
| Refactoring discipline | N/A — append-only prose + new fixture + new test |
| AI tolerance contract | Producer re-run prose followed by `/slo-architect` (AI). Accepted variance: diff wording may vary. Deterministic boundary: a changed abuse case becomes `status: superseded` + non-empty `superseded_by` + `supersede_reason`; the original id is NEVER reused/renumbered; no silent clobber. Eval evidence: `slo_tmp_m2_rerun` validates the live superseded-row fixture against the merged schema rule + asserts the prose. Retry/fallback: none — a renumber is a hard failure. Must-never: renumber, silently drop a superseded row, or clobber without surfacing a diff. Bounded sample budget: deterministic single-run test. Cites `references/ai-tolerance-contract.md`. |
| Data classification | Public |
| Proactive controls in play | OWASP C5 — superseded-row fixture strict-validates against the merged schema; OWASP C1 — the no-silent-clobber + supersede-don't-renumber invariant is documented and test-bound |
| Abuse acceptance scenarios | `tm-slo-threat-model-abuse-7` (renumber on re-run) and `tm-slo-threat-model-abuse-8` (silent drop of a superseded case) — BDD rows below |
| Resource bounds introduced/changed | Re-run prose ≤ 30 added nonblank lines; demo fixture bounded static file |
| Invariants/assertions required | (a) Step 3.5 documents: detect existing `.slo.json`, diff, supersede-don't-renumber, `superseded_by`+`supersede_reason` required, surface diff + overwrite/merge/skip, no silent clobber; (b) the new demo fixture has ≥1 `status: superseded` row with non-empty `superseded_by`+`supersede_reason` and contiguous ids; (c) **ENG-2: `slo_tmp_m2_rerun` itself strict-parses the demo fixture with the same serde `deny_unknown_fields` structs + frozen-ID/superseded invariants (the merged `slo_tm_m1_schema` only loads its own hardcoded fixture and will not read this one)**; (d) `slo_tmp_m2_rerun` fails if the demo fixture renumbers or drops the superseded id |
| Debugger / inspection expectation | `jq '.abuse_cases[] | select(.status=="superseded")'` on the demo fixture; Rust test output |
| Static-analysis gates | `cargo fmt --all -- --check`; `cargo clippy -p sast-verify --test slo_tmp_m2_rerun -- -D warnings`; `cargo test -p sast-verify --test slo_tmp_m2_rerun --test slo_tmp_m1_producer --test slo_tm_m1_schema`; regression the four `e2e_*` tests |
| Reversibility / rollback path | Revert the appended re-run prose + delete the demo fixture + M2 test; M1 and the merged wedge are independent and untouched |
| Forbidden shortcuts | Do not introduce a separate diff-store; do not renumber; do not reflow existing Step 3.5; do not weaken `slo_tm_m1_schema`; do not edit the four `e2e_*` tests |

### Out Of Scope / Must Not Do

- Do not build SEC-2 redaction/gitignore enforcement (separate parked follow-up — `LESSONS-BACKLOG.md` Row 2).
- Do not change the merged schema or consumer contract.
- Do not add OTM/TM-BOM export.

### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-architect/SKILL.md` | Append the `.slo.json` re-run / supersession algorithm to Step 3.5 |
| `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json` | New dogfood fixture with a live `status: superseded` row |
| `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs` | New: supersession prose + live superseded-row invariants |
| `docs/slo/critique/slo-threat-model-producer.md` | M2 critique rows |
| `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` | M2 evidence/tracker |
| `docs/slo/verify/slo-threat-model-producer-m2.md` | M2 verification report |
| `docs/slo/lessons/slo-threat-model-producer-m2.md` | M2 lessons |
| `docs/slo/completion/slo-threat-model-producer-m2.md` | M2 completion |

### Step-By-Step

1. Read the existing Step 3.5 Markdown no-silent-clobber wording (to mirror it).
2. Add the failing M2 test (`slo_tmp_m2_rerun`): asserts the re-run prose + validates a live superseded-row demo fixture.
3. Run it; confirm red (prose + fixture absent).
4. Append the re-run/supersession algorithm to Step 3.5; author the demo fixture with a real `status: superseded` row.
5. Run `slo_tmp_m2_rerun` (it owns the demo-fixture strict-parse — ENG-2) + `slo_tm_m1_schema` (regression on its own original fixture only) + `slo_tmp_m1_producer` to green; run the four `e2e_*` regressions.
6. Scoped clippy + formatter (record unrelated drift honestly).
7. Fill the M2 Evidence Log.

### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Evidence |
|---|---|---|---|---|---|
| `rerun_algorithm_documented` | happy path | Step 3.5 edited | `slo_tmp_m2_rerun` reads it | Documents detect-existing + diff + supersede-don't-renumber + surface-diff + overwrite/merge/skip + no silent clobber | `cargo test -p sast-verify --test slo_tmp_m2_rerun` |
| `live_superseded_row_validates` | happy path | The demo fixture has a `status: superseded` row | `slo_tmp_m2_rerun` strict-parses it with the same `deny_unknown_fields` structs | Conforms; `superseded_by`+`supersede_reason` non-empty; ids contiguous (ENG-2: `slo_tmp_m2_rerun` owns this parse — `slo_tm_m1_schema` does not read this fixture) | `cargo test -p sast-verify --test slo_tmp_m2_rerun` |
| `rerun_prose_absent` | empty state | Step 3.5 has no re-run algorithm | M2 test runs pre-implementation | Red with explicit "re-run algorithm missing" (BDD-first) | same M2 test |
| `schema_invalid_superseded_rejected` | invalid input | demo fixture's superseded row drops `supersede_reason` | `slo_tmp_m2_rerun` strict parse + invariant check (its own structs) | Hard fail — supersede metadata mandatory | same M2 test |
| `renumber_on_rerun_fails` | abuse case | demo fixture renumbers instead of superseding | `slo_tmp_m2_rerun` id-continuity check | Fails — realizing `tm-slo-threat-model-abuse-7` | same M2 test |
| `silent_drop_of_superseded_fails` | abuse case | demo fixture omits the superseded row entirely | contiguity/coverage check | Fails — realizing `tm-slo-threat-model-abuse-8` | same M2 test |

### Regression Tests

- `cargo test -p sast-verify --test slo_tm_m1_schema --test slo_tm_m2_consumers --test slo_tmp_m1_producer`
- `cargo test -p sldo-install --test e2e_slo_sec_m1 --test e2e_cloud_threat_model_m1 --test e2e_fowler_ai_arch_m1 --test e2e_fowler_ai_arch_m3`

### Compatibility Checklist

- [x] Step 3.5 append-only; four `e2e_*` guards intact (11/7/6/23). M2 added no SKILL.md/template prose (M1's item 8 already shipped the algorithm wording — honest scope correction).
- [x] Merged `slo_tm_m1_schema` still green on its own original fixture (ENG-2: it does not read the new demo fixture — not expected to).
- [x] `slo_tmp_m2_rerun` strict-parses the new demo fixture with `deny_unknown_fields` structs and enforces the superseded-row invariants; all 5 execute-time mutations proven to bite.
- [x] No schema change; no new crate/skill dir.
- [x] M1 producer prose + test still green (`slo_tmp_m1_producer` 6, including the SEC-1 regression).

### E2E Runtime Validation

N/A — skill prose + fixture + structural test. Verified structurally + by forced-failure mutation (the renumber/drop guards must bite), same discipline as the merged wedge.

### Smoke Tests

- `jq '.abuse_cases[] | select(.status=="superseded")' docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json` returns a real row with `superseded_by` + `supersede_reason`.
- `rg -n "superseded|no silent clobber|overwrite, merge, or skip" skills/slo-architect/SKILL.md` shows the re-run algorithm appended.
- `slo_tmp_m2_rerun` strict-parses the new demo fixture green; `slo_tm_m1_schema` green on its own original fixture (ENG-2 — separate fixtures, separate tests).

### Evidence Log

| Check | Command / Action | Expected Result | Actual Result | Status | Notes |
|---|---|---|---|---|---|
| Repo hygiene | branch before/after; dirty tree | task branch, not main | Continued on `slo/slo-threat-model-producer` from M1; no remediation needed | `pass` | Single feature branch |
| Existing-wording recon | Read M1 producer prose | mirror M1 algorithm wording | **M1's item 8 ALREADY contains the full re-run algorithm** (`supersede`/`renumber`/`overwrite`/`merge`/`skip`/`no silent clobber`/`diff` + `-threat-model.slo.json` target). M2 distinct work is therefore the demo fixture + tests; no redundant new prose. The algorithm-prose test becomes a regression-guard for the M1 wording | `pass` | Honest scope correction — smallest safe change |
| New M2 test fails first | `cargo test -p sast-verify --test slo_tmp_m2_rerun` | red — demo fixture absent | 3 failed (fixture-related); 1 passed (algorithm-prose regression-guard already green because M1 shipped the wording) | `pass` | Red for the intended reason |
| Re-run prose + demo fixture added | Author the demo fixture (no SKILL.md edits needed in M2) | fixture has a live superseded row | Authored `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json`: 4 abuse cases (1..4 contiguous), abuse-2 `status: superseded` `superseded_by: tm-…-abuse-4` with a real reason. Provenance: `producer_skill_sha = e4ccfdf…408e9e`, input = runbook blob `4eb0b5d…71500c` | `pass` | |
| New M2 test passes | `cargo test -p sast-verify --test slo_tmp_m2_rerun` | green | 4 passed; 0 failed | `pass` | |
| Demo-fixture strict-parse (ENG-2) | `cargo test -p sast-verify --test slo_tmp_m2_rerun` | `slo_tmp_m2_rerun` strict-parses the demo fixture (its own structs); `slo_tm_m1_schema` regression green on its own fixture only | `slo_tmp_m2_rerun::demo_fixture_strict_parses_with_own_structs` passes; `slo_tm_m1_schema` 6 (loads its hardcoded fixture only, as ENG-2 locked) | `pass` | Critique-time ENG-2 correction honored |
| M1 + e2e_* regression | M1 producer test + the four `e2e_*` | pass | `slo_tmp_m1_producer` 6, `slo_tm_m1_schema` 6, `slo_tm_m2_consumers` 5; four `e2e_*` 4/4 ok (11/7/6/23) | `pass` | Every prior guard green |
| Mutate-force-restore at execute (M1 rule) | Mutate demo fixture / SKILL.md to force each guard; observe FAIL; restore byte-identically | every M2 invariant has a mutation that bites | G1 renumber-1→99 → contiguity FAIL; G2 drop abuse-3 → contiguity FAIL; G3 strip supersede_reason → superseded-row FAIL; G4 superseded→active → superseded-row FAIL; G5 strip "no silent clobber" → algorithm FAIL. Demo fixture + SKILL.md SHA-256 round-trip OK after each | `pass` | M1 lesson rule "design the mutation at execute time, not verify" honored end-to-end |
| Scoped clippy | `cargo clippy -p sast-verify --test slo_tmp_m2_rerun -- -D warnings` | passes | Clean | `pass` | |
| Formatter | `cargo fmt --all -- --check` | passes or unrelated blocker recorded | `slo_tmp_m2_rerun.rs` rustfmt-clean (exit 0); demo fixture is JSON (rustfmt N/A). Whole-tree still drifts on the same ~30 pre-existing unrelated files | `blocked-unrelated` | Same blocker as merged wedge / M1 |
| Compatibility check | `git status --short` | only M2 allow-list files changed | Footprint = `RUNBOOK`, new demo `.slo.json`, new `slo_tmp_m2_rerun.rs` — all M2 allow-listed. SKILL.md/template untouched in M2 (M1's prose was sufficient) | `pass` | Smallest safe change |

### Definition Of Done

- [x] Step 3.5 documents the full `.slo.json` re-run algorithm (detect/diff/supersede-don't-renumber/surface-diff/no-silent-clobber). Shipped in M1's item 8 (verified — M2 added no SKILL.md edit because M1's wording already contains every required phrase); M2's `rerun_algorithm_documented_in_step35` test stands as the regression-guard against future M1-wording weakening.
- [x] A committed demo fixture carries a live `status: superseded` row that `slo_tmp_m2_rerun` strict-parses + invariant-checks with its own `deny_unknown_fields` structs (ENG-2: not via `slo_tm_m1_schema`, which only loads its hardcoded fixture).
- [x] `slo_tmp_m2_rerun` failed before (3/4), passes after (4/4); G1–G5 forced-failure mutations all proven to bite at execute time per M1 lesson.
- [x] Four `e2e_*` guards + M1 producer test still green; schema byte-unchanged.
- [x] No new crate/skill dir.

---

## 7. Documentation Update Table

| Doc | Update | Milestone |
|---|---|---|
| `skills/slo-architect/SKILL.md`, `references/threat-model-template.md` | Producer + re-run contract (append-only) | M1, M2 |
| `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` | Evidence logs, tracker | M1, M2 |
| `docs/ARCHITECTURE.md` / `docs/skill-pack-catalog.md` | NOT updated — reality-first; this hardens an existing internal contract, adds no new public skill surface. (The wedge set this precedent.) | — |
| `docs/slo/{critique,verify,lessons,completion}/slo-threat-model-producer-*` | Closeout artifacts | M1, M2 |

## 8. Red Lines (whole-runbook)

- Schema is frozen by #90 — no field/version change.
- Append-only to `/slo-architect`; the four `e2e_*` guards must stay green.
- Supersede-don't-renumber; never silent clobber; provenance idiom matches repo precedent.
- No new skill directory, no new crate, no OTM/TM-BOM export, no SEC-2 redaction work (separate parked follow-up).
