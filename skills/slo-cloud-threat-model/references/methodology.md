---
name: slo-cloud-threat-model-methodology
status: stable
source_skill: skills/slo-cloud-threat-model/SKILL.md
audience: the agent running /slo-cloud-threat-model
purpose: The M1 validate → M2 author → M3 self-check method for producing one scenario threat model.
---

# Methodology — scenario threat-model authoring

This is the authoring contract behind `/slo-cloud-threat-model`. The skill is pure
Markdown: the agent authors the document; the bundled helper provides only the
deterministic scenario enumeration, schema validation, and license-boundary guard.
Read [`citation-and-licensing.md`](citation-and-licensing.md) and
[`threat-model-template.md`](threat-model-template.md) before authoring.

## M1 — validate (hard gate)

1. `python3 <skill-root>/scripts/scenario_catalog.py list` — argv-list subprocess, no
   shell string. The output is the stable declared scenario order. The declared list in
   the script is the contract; on-disk scenario files must match it exactly.
2. `python3 <skill-root>/scripts/scenario_catalog.py validate` — schema, control-ID
   shape (`FRAMEWORK:SUFFIX` or a bare Hulumi rule ID), known-framework check, and the
   verbatim-prose license-boundary deny-list. A non-zero exit is a hard stop: surface the
   failing scenario ID and the reason; do **not** author a partial document.
3. Resolve the requested scenario ID against the `list` output. Unknown or empty → print
   the list and stop. Never infer a scenario from model memory.

## M2 — author

1. `python3 <skill-root>/scripts/scenario_catalog.py show <scenario>` to print the
   resolved scenario object. Treat its fields as the only source of truth.
2. Fill [`threat-model-template.md`](threat-model-template.md):
   - `Scenario`, `Actors`, `Assets` — copy `description`, `actors[]`, `assets[]` verbatim.
   - `Threats (STRIDE)` — one row per `stride[]` entry. `Type` is the STRIDE letter;
     `Name` and `Description` are copied from the entry; `Controls` is the entry's
     `controls[]` joined with `, ` — **identifiers only**, never paraphrased or verbatim
     control wording. If `controls[]` is empty render `(none enumerated)`.
   - `Control Citations` table + frontmatter `citations[]` — for every distinct control
     ID across all STRIDE rows, resolve `framework` (the segment before the first `:`, or
     the bare ID for a Hulumi policy rule) and `url` via the map in
     [`citation-and-licensing.md`](citation-and-licensing.md). Deduplicate by ID;
     preserve first-seen order. Every URL must begin `https://`.
   - `Recommended Hulumi Components` — one bullet per `recommendedComponents[]` entry:
     `` `<name>` `` — `<availability>`. `<rationale>`. The `availability` string is
     pinned; do not promote an "evolving" / "Planned for vX" entry to "available now",
     and do not demote a "Shipped in vX" entry. This is the version-pinning discipline.
   - `Open Questions` — every entry of `requiredFrameworks[]` that resolved zero
     citations, and every control ID whose framework is absent from the map, becomes a
     bullet. Never invent an ID or URL to remove a gap.
3. Render the audit footer from the template unchanged. The document must contain zero
   verbatim licensed control prose — paraphrase is authored here, identifiers are facts.

## M3 — self-check + companion

Re-read the generated Markdown and assert before declaring done:

- frontmatter has `name`, `scenario`, `generated_at` (ISO-8601), and a non-empty
  `citations[]`; every citation `url` starts `https://`;
- every STRIDE row has a single STRIDE letter, a non-empty name and description, and a
  `Controls` cell that is IDs-only;
- at least two `Recommended Hulumi Components` bullets;
- no licensed control prose (spot-check against the deny-list classes in
  [`citation-and-licensing.md`](citation-and-licensing.md)).

Then write the `.json` companion: the resolved scenario object plus a `citations` array
of `{framework, id, url}` in the same dedup order as the table. Finally, summarize the
top STRIDE risks and the recommended components to the user in two or three sentences.

## Resource bounds

`scenario_catalog.py` is offline and bounded: stdlib-only imports (no network),
no `subprocess` / `os.system` / `bash -c`, every `--scenarios-dir` path component
checked for a symlink before resolution, and each scenario fixture capped at
**1 MiB** before it is read. Invoke it argv-list, never as a shell string.

## Idempotency

Re-running the same scenario regenerates both artifacts deterministically except the
`generated_at` timestamp and any explicitly time-derived field. Do not silently rewrite
unrelated files; the only writes are the `.md` and `.json` for the requested scenario.
