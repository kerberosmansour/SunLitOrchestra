---
name: slo-threat-model
created: 2026-05-19
status: design-locked
tla_required: false
tla_reason: >
  Sequential file I/O plus structural-contract test execution. No concurrent
  actors sharing state, no distributed consensus, no leader election, no
  cross-process ordering guarantees, no resource leases. The frozen-ID
  invariant is a schema rule enforced by a deterministic test, not a
  concurrency protocol.
security_libs_required: false
security_libs_reason: >
  This feature is a contract artifact (a JSON schema reference + read-side
  enforcement language + one structural-contract test). It does not integrate
  Hulumi / SunLitSecurityLibraries. /slo-sec-libs is a downstream *consumer*
  of the threat model, not a dependency this feature pulls in.
ai_component: true
ai_component_reason: >
  The `<slug>-threat-model.slo.json` artifact is untrusted input consumed by
  agent-run /slo-critique and /slo-verify. A crafted threat-model JSON is a
  prompt-injection surface — the same risk the existing Markdown
  threat-model-template guards with its `~~~text` fence rule. The AI triad
  (MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF) section applies to that
  injection surface; see the threat model file.
compliance: [soc2, asvs]
compliance_reason: >
  Defaults. SLO is an OSS tool with no regulated data-processing of its own.
  The UK GDPR / DUAA false-assurance concern raised in the idea doc is a
  *downstream-user* residual risk (a stale model causing a false coverage
  attestation in someone else's regulated product), not a compliance
  obligation of this artifact — it is carried as a residual risk in the
  threat model, not as a compliance column.
---

# Design overview — /slo-threat-model (read-first contract wedge)

## System goal

Make threat-model abuse-case IDs survive across SLO skill invocations. Today
`/slo-architect` Step 3.5 emits `docs/slo/design/<slug>-threat-model.md` and
every downstream skill (`/slo-plan`, `/slo-critique`, `/slo-verify`,
`/slo-sec-libs`) re-derives the model independently, so an abuse-case ID
(`tm-<slug>-abuse-N`) can silently change meaning between `/slo-plan` and
`/slo-execute` — which already shipped a wrong test that `/slo-verify`
greenlit. This wedge introduces a single machine-readable companion artifact
with **frozen abuse-case IDs** and an explicit **accepted-residual** marker
that `/slo-critique` and `/slo-verify` *read and halt on* rather than
re-derive. It proves the consumer contract before any producer skill is built.

## Scope (the wedge — read-side only)

In:

1. `references/security/threat-model-schema.md` — the SLO-owned minimal JSON
   schema reference (mirrors the precedent of `references/sast/manifest-schema.md`).
2. One hand-authored fixture serialized from an existing dogfood threat model
   (`docs/slo/design/slo-security-embedding-threat-model.md` →
   `docs/slo/design/slo-security-embedding-threat-model.slo.json`).
3. Read-side contract language added to `skills/slo-critique/SKILL.md` and
   `skills/slo-verify/SKILL.md`: read the frozen `.slo.json`; halt rather than
   re-derive abuse-case IDs; distinguish `accepted_residual` from missing
   coverage.
4. One structural-contract test under `xtasks/sast-verify/tests/` asserting
   schema-doc structure, fixture validity, the frozen-ID invariant, the
   `classification` field, and the SKILL.md read-side language.

Out (deliberately deferred past the wedge):

- A `skills/slo-threat-model/SKILL.md` *producer* skill — **zero new skill
  directories** in this wedge.
- `/slo-architect` Step 3.5 delegation to the producer.
- OTM 0.2.0 / CycloneDX TM-BOM export adapters.
- Any redaction engine.

## Planned architecture (this feature)

Solid lines exist today. Dashed lines are added by this runbook. Per the
repo's **reality-first `docs/ARCHITECTURE.md`** non-negotiable, this diagram
lives here, not in `ARCHITECTURE.md`, until shipped.

```
┌──────────────────────────────────────────────────────────────────────────┐
│  /slo-architect Step 3.5  ──(solid)─►  docs/slo/design/<slug>-threat-model.md│
│                           ──(dashed)─►  docs/slo/design/<slug>-threat-model. │
│                                          slo.json   [frozen AC IDs]        │
└───────────────┬──────────────────────────────────────────┬────────────────┘
                │ (dashed) read frozen JSON                 │ (dashed)
                ▼                                            ▼
   ┌─────────────────────────┐               ┌─────────────────────────────┐
   │ /slo-critique           │               │ /slo-verify (Pass 4)        │
   │  read .slo.json;        │               │  read .slo.json;            │
   │  HALT not re-derive IDs;│               │  scope by accepted_residual │
   │  accepted_residual ≠    │               │  vs missing coverage;       │
   │  missing coverage       │               │  HALT not re-derive IDs     │
   └────────────┬────────────┘               └──────────────┬──────────────┘
                │                                            │
                ▼ (dashed) deterministic guard               ▼
   ┌────────────────────────────────────────────────────────────────────────┐
   │ xtasks/sast-verify/tests/<prefix>_m<N>_threat_model_contract.rs          │
   │  schema-doc structure │ fixture valid │ frozen-ID invariant │           │
   │  classification field present │ SKILL.md read-side language present     │
   └────────────────────────────────────────────────────────────────────────┘
```

### Trust boundaries

- **Threat-model JSON ↔ agent-run skill.** The `.slo.json` is untrusted input.
  `/slo-critique` and `/slo-verify` run inside an agent; a crafted file is a
  prompt-injection vector. Any user-controlled string surfaced from it must be
  treated as literal data, not instructions (mirrors the Markdown template's
  `~~~text` fence rule).
- **Public repo ↔ attacker.** `docs/slo/design/*.slo.json` is git-tracked. A
  machine-readable file enumerating `accepted_residual: true` entries is a
  scrapeable recon map of known-unfixed weaknesses. Mitigated by the per-entry
  `classification` field + the documented two-tier gitignore discipline for
  Confidential/Restricted detail (research synthesis).
- **`/slo-plan` ↔ `/slo-execute` ID continuity.** The boundary the wedge
  exists to harden: an abuse-case ID must mean the same thing on both sides.

### Data flow — new artifact

`<slug>-threat-model.slo.json` — a serialization of the *existing* Markdown
threat-model structure (STRIDE cells, `tm-<slug>-abuse-N` abuse-case rows,
compliance mapping, residual-risk table, provenance). It does not invent a new
model; it makes the existing one machine-readable and ID-stable.

## Non-negotiables (downstream cannot change these without migration)

- **Zero new skill directories in the wedge.** The producer skill is a later
  runbook with its own plan.
- **Serialize the existing structure; do not invent.** The JSON mirrors
  `skills/slo-architect/references/threat-model-template.md`, including the
  `tm-<slug>-abuse-N` ID convention. No `AC-N` parallel scheme.
- **Frozen IDs, supersede-don't-renumber.** A `tm-<slug>-abuse-N` is assigned
  once. A change supersedes (carries `superseded_by` + reason); it never
  renumbers. Enforced by the structural-contract test, not prose.
- **Provenance matches repo precedent.** Producing-skill SKILL.md git SHA +
  input-doc SHAs, strict unknown-field rejection — same idiom as
  `references/sast/manifest-schema.md`. No novel content-hash scheme.
- **Reality-first ARCHITECTURE.md.** This planned work does not enter
  `docs/ARCHITECTURE.md` until shipped.
- **No silent clobber.** A regenerated `.slo.json` surfaces a diff and a
  supersession record; it never silently overwrites frozen IDs.
- **F-ENG-6 governance on `slo-critique/SKILL.md`.** The read-side edit to the
  canonical portable critique path is permitted only via a documented runbook
  amendment that updates the pinned SHA-256 constant in
  `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` in the same milestone. This
  is a hard constraint, not a cleanup step.

## Residual risks carried into /slo-plan

1. **Prose-discipline relapse.** The wedge's value is that the frozen-ID and
   `classification` invariants are *machine-checked*. If the structural test
   asserts the prose around the rule instead of the rule itself, the original
   failure recurs. The test must bind the invariant directly.
2. **Public-repo recon exposure.** Until a producer/redaction story exists,
   the wedge mitigates by `classification` + documented gitignore discipline
   only — it does not enforce redaction. Downstream user repos with
   Confidential residual detail must follow the two-tier guidance manually.
3. **Stale-model false assurance (UK GDPR / DUAA downstream).** A frozen JSON
   that drifts from reality could let `/slo-critique` greenlight uncovered
   coverage in a regulated downstream product. Mitigated by provenance
   (input-doc SHAs) so staleness is detectable; not eliminated in the wedge.
