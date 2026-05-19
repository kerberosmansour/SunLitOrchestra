# Interfaces — slo-threat-model

Every interface downstream milestones must not rename or reshape without
explicit migration work. Stability levels: `stable` (frozen), `evolving`
(may change with a migration note), `internal` (fair game).

## Persisted-artifact shape: `docs/slo/design/<slug>-threat-model.slo.json`

`stable`. Minimal SLO-owned schema. Serializes the existing Markdown
threat-model template structure. Top-level object:

| Key | Type | Notes | Stability |
|---|---|---|---|
| `otm_compatible` | bool | informational; this is *not* OTM, but records whether a lossless OTM export is believed possible | `evolving` |
| `slo_schema_version` | string (semver) | schema version; `0.1.0` for the wedge | `stable` |
| `slug` | string | the feature slug; matches the doc family | `stable` |
| `sensitivity` | enum `public` \| `internal` \| `confidential` \| `restricted` | top-level publication marker; gates the two-tier gitignore discipline | `stable` |
| `provenance` | object | see below | `stable` |
| `stride` | array of STRIDE-cell objects | one per component×class that applies | `stable` |
| `abuse_cases` | array of abuse-case objects | the frozen-ID surface | `stable` |
| `residual_risks` | array of residual-risk objects | accepted-residual surface | `stable` |
| `compliance` | array of compliance-mapping objects | SOC2/ASVS default + opt-ins | `evolving` |

### `provenance` object

`stable`. Matches the `references/sast/manifest-schema.md` idiom.

| Key | Type | Notes |
|---|---|---|
| `produced_by` | string | producing skill name (wedge: `slo-architect`) |
| `producer_skill_sha` | string | git SHA of the producing `SKILL.md` at emit |
| `inputs` | array of `{path, sha}` | idea/research/architecture inputs + their git SHAs so staleness is detectable |
| `generated_at` | string (ISO-8601 date) | emit date |

### `abuse_cases[]` object — the frozen-ID surface

`stable`. **This is the contract the wedge exists to harden.**

| Key | Type | Notes |
|---|---|---|
| `id` | string matching `^tm-<slug>-abuse-\d+$` | **frozen.** Assigned once; never renumbered. |
| `surface` | string | new endpoint/handler/file-write/outbound/subprocess |
| `attacker` | string | rendered as literal data, never instructions |
| `attack_step` | string | literal data |
| `attacker_outcome` | string | literal data |
| `control` | string | the eliminating/mitigating control |
| `status` | enum `active` \| `superseded` | supersede-don't-renumber |
| `superseded_by` | string \| null | replacement `id` when `status: superseded` |
| `supersede_reason` | string \| null | required when `status: superseded` |
| `classification` | enum `public` \| `internal` \| `confidential` \| `restricted` | per-entry recon-exposure marker |

Frozen-ID invariant (test-enforced): for any `id` present in a prior committed
`.slo.json`, a regenerated file must keep the same `id` bound to a row whose
`status` is `active` or `superseded`; an `id` may never be reused for a
different abuse case. Re-derivation that renumbers is a hard test failure.

### `residual_risks[]` object — accepted-residual surface

`stable`. Lets `/slo-critique` and `/slo-verify` distinguish *accepted risk*
from *missing coverage*.

| Key | Type | Notes |
|---|---|---|
| `risk` | string | literal data |
| `exploit_path` | string | literal data |
| `compensating_control` | string | |
| `accepted_residual` | bool | `true` ⇒ knowingly accepted, not a gap |
| `owner` | string | |
| `review_by` | string (ISO-8601 date) | residual cannot sit forever |
| `classification` | enum (as above) | recon-exposure marker |

### `stride[]` / `compliance[]` objects

`stride[]` `stable`: `{component, class ∈ STRIDE, state ∈ {eliminated,
mitigated, na, residual}, control_or_reason}`. `compliance[]` `evolving`:
`{control, soc2, asvs, <opt-in framework>...}` — opt-in columns may grow.

## Consumer read-side contract (SKILL.md language)

`stable`. Added to `skills/slo-critique/SKILL.md` and
`skills/slo-verify/SKILL.md`:

- When a `<slug>-threat-model.slo.json` exists for the slug under review, the
  skill **reads abuse-case IDs from it** and **must not re-derive or
  renumber** them. If the file is absent or fails schema validation, the skill
  **halts with an explicit message** rather than silently re-deriving.
- `/slo-critique` treats `accepted_residual: true` rows as *not* findings;
  treats abuse cases with no covering control as missing-coverage findings.
- `/slo-verify` Pass 4 scopes runtime checks to `abuse_cases[].status ==
  active`; explains N/A/skipped rows by reference to `accepted_residual`.

## Structural-contract test

`internal` (implementation), but the *assertions* are `stable`: schema-doc
required sections present; fixture parses and validates; frozen-ID invariant
holds across the committed fixture; every `abuse_cases[]`/`residual_risks[]`
entry carries `classification`; both consumer SKILL.md files contain the
read-side halt language. Test file: `xtasks/sast-verify/tests/<prefix>_m<N>_
threat_model_contract.rs`.

## Out-of-scope interfaces (not built in the wedge)

- Producer skill API (`skills/slo-threat-model/SKILL.md`) — `evolving`,
  future runbook.
- OTM / TM-BOM export shape — `evolving`, future, additive only.
