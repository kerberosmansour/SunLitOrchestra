---
name: slo-sec-libs-m2-matcher
description: Capability matching algorithm for `/slo-sec-libs` M2.
---

# /slo-sec-libs M2 Matcher Methodology

## Scope

M2 matches target runbook proactive-control rows against one or more M1 capability catalogs. It is read-only: no GitHub filing, no cache mutation, no catalog mutation, and no web lookup.

## Inputs

- A target runbook Markdown file.
- One or more M1 catalog JSON files emitted by `scripts/read-declarations.py`.

The matcher treats the catalog as the only source of library capability truth. Model memory, package names, README recollection, and web search are not evidence.

## Runbook Row Extraction

Read every Markdown table row whose first cell normalizes to:

```text
**Proactive controls in play**
```

For each matching row:

1. Preserve the full second-cell text as `control_text`.
2. Split semicolon-delimited controls into one logical row per control item.
3. Extract the leading control ID with regex `\b(?:OWASP\s*)?C([0-9]+)\b`.
4. Normalize each ID to `C<number>`.
5. Assign stable row IDs in file order: `pc-001`, `pc-002`, ...

If no rows are found, emit:

```json
{"matched": [], "unmatched": [], "diagnostics": ["no controls to match"]}
```

## Catalog Evidence

For each catalog component, construct a candidate evidence bundle from:

- `bom_ref`
- `group`
- `name`
- `version`
- `controls`
- `capabilities`
- `properties`
- component-local `claims`

Every matched entry must carry at least one `catalog_bom_ref` that exactly equals a component `bom_ref` from the catalog input. If the matcher cannot cite a catalog `bom_ref`, the row is unmatched.

## Candidate Selection

A component is a candidate when at least one condition is true:

1. The normalized proactive-control ID matches one of the component `controls` after normalizing `OWASP-C5`, `OWASP C5`, and `C5` to `C5`.
2. A non-stopword token from `control_text` appears in `capabilities`, `properties[].value`, or claim text.
3. A backticked or quoted capability phrase from `control_text` appears exactly in a catalog capability or property value.

Do not use package names alone as a match. Names can disambiguate evidence, but they do not prove a capability.

## Specificity Score

Specificity is measured from catalog evidence, not prose confidence. The rule is: more parametric evidence means a more specific capability advertisement. Score one point for each distinct parametric fact in the candidate evidence bundle:

- numeric thresholds: `>=3`, `64 MiB`, `256-bit`, `15 minutes`
- named algorithm variants: `Argon2id`, `AES-GCM`, `Ed25519`
- explicit parameter names: `iterations`, `memory`, `parallelism`, `salt`, `nonce`
- constrained modes or enforcement states: `constant-time`, `fail-closed`, `deny-by-default`
- claim evidence tied to the same `bom_ref`

Generic control labels such as `C1` or `validate inputs` are not parametric facts. A candidate with only generic labels is low confidence and must not beat a candidate with concrete parametric evidence.

## Dispositions

For each logical proactive-control row:

- No candidates -> put one entry in `unmatched` with `reason: "no-candidate-libraries"`.
- One candidate -> put one entry in `matched` with `disposition: "single-candidate"`.
- Multiple candidates, one highest specificity score -> put one entry in `matched` with `disposition: "preferred-by-specificity"`.
- Multiple candidates with equal highest specificity -> include all top candidates with `disposition: "tie"`.
- Comparable candidates with different parametric values where both satisfy the row -> prefer the stricter value with `disposition: "preferred-conservative"` and add diagnostic `conservative tiebreaker applied`.

The conservative rule only applies when both candidates are valid for the row and their evidence is comparable. If comparability is unclear, use `tie`.

## Output Shape

Emit one JSON object:

```json
{
  "matched": [
    {
      "row_id": "pc-001",
      "control_id": "C5",
      "control_text": "C5 (validate inputs with strict schema)",
      "disposition": "preferred-by-specificity",
      "selected_catalog_bom_ref": "component:slsl-strict-schema",
      "candidates": [
        {
          "catalog_bom_ref": "component:slsl-strict-schema",
          "name": "strict-schema",
          "specificity_score": 3,
          "evidence": ["controls:C5", "capability:strict schema validation"]
        }
      ]
    }
  ],
  "unmatched": [
    {
      "row_id": "pc-002",
      "control_id": "C9",
      "control_text": "C9 (append-only audit trail)",
      "reason": "no-candidate-libraries",
      "catalog_ids_considered": ["component:slsl-strict-schema"]
    }
  ],
  "diagnostics": []
}
```

`selected_catalog_bom_ref` is omitted when `disposition` is `tie`, because the point is to surface all top candidates to the user. In other words: selected_catalog_bom_ref is omitted for ties.

## Fabrication Guard

Before presenting output, build the set of allowed IDs from all input catalogs:

```text
allowed_catalog_ids = every components[].bom_ref
```

Then assert:

```text
for each matched candidate:
  candidate.catalog_bom_ref in allowed_catalog_ids
for each selected_catalog_bom_ref:
  selected_catalog_bom_ref in allowed_catalog_ids
```

If any ID is missing, discard the match and move the row to `unmatched` with `reason: "fabricated-catalog-id-refused"`. This is the M2 mitigation for `tm-slo-sec-libs-abuse-7`.

## Empty States

- Empty runbook: `matched: []`, `unmatched: []`, diagnostic `no controls to match`.
- Empty catalog: every logical proactive-control row becomes `unmatched` with `reason: "no-candidate-libraries"` and diagnostic `no candidate libraries`.

## Handoff To M3

M2 emits one unmatched record per unmatched logical control row. M3 turns those records into capability-gap issue bodies; M2 must not file anything.
