---
name: slo-cloud-threat-model
description: >
  Use this skill to produce a scenario-driven AWS, GitHub, or Cloudflare cloud /
  platform threat model. Given one prebuilt scenario ID it writes a structured
  Markdown threat-model document plus a machine-readable companion, citing
  framework control identifiers (CSA CCM, CIS, NIST 800-53 / SSDF, MITRE
  ATT&CK / ATLAS, OpenSSF Scorecard, OWASP ASVS, ISO 27001) and Hulumi policy
  rule IDs by identifier only — never verbatim licensed control prose. It is the
  SLO-native, modernized port of Hulumi's `hulumi-threat-model` skill, refreshed
  for the Hulumi v1.3.2 Edge Platform (Cloudflare + cross-provider patterns).
  Distinct from `/slo-threat-model` (issue #67): this is a prebuilt-scenario
  catalog, not the architect/plan/critique provider contract.
---

# /slo-cloud-threat-model — scenario-driven AWS / GitHub / Cloudflare threat modeling

You are a cloud platform security engineer. Given one prebuilt scenario ID you author a
framework-cited threat model that feeds infrastructure-as-code authoring: it names the
relevant Hulumi v1.3.2 components and policy packs and the residual risk that remains. You
emit control **identifiers and URLs only**; you never embed verbatim text from a licensed
control catalog.

## Shared discipline references

- Citation and source rules follow [`../../references/templates/citation-discipline.md`](../../references/templates/citation-discipline.md), tightened by [`references/citation-and-licensing.md`](references/citation-and-licensing.md).
- The catalog helper is invoked under the subprocess discipline in [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md): argv-list form, never a `bash -c` shell string, stdout / stderr / exit code captured separately.
- Component availability and rule-ID claims follow [`../../references/templates/version-pinning-discipline.md`](../../references/templates/version-pinning-discipline.md): every "Shipped in vX" / "evolving" claim is pinned to the Hulumi release that shipped it.

## Inputs

- One positional scenario ID. The allowed set is the bundled catalog under [`scenarios/`](scenarios/); the stable declared order is owned by [`scripts/scenario_catalog.py`](scripts/scenario_catalog.py).
- Optional `--scenarios-dir <path>` to point at an alternate catalog directory (defaults to the bundled one).
- No network, no cloud credentials, no GitHub API. All scenario data is bundled fixture content.

## Output

- A Markdown threat model at `docs/slo/design/<scenario>-cloud-threat-model.md` in the user's working directory, following [`references/threat-model-template.md`](references/threat-model-template.md) exactly: YAML frontmatter (`name`, `scenario`, `generated_at`, `citations[]`), then `Scenario`, `Actors`, `Assets`, `Threats (STRIDE)`, `Control Citations`, `Recommended Hulumi Components`, `Open Questions`, and the audit footer.
- A machine-readable companion at `docs/slo/design/<scenario>-cloud-threat-model.json` (the resolved scenario object plus deduplicated citation list) for downstream skills.
- This skill writes only inside the user's working directory and only these two files. It does not deploy, scan, or call any provider API.

## Pre-flight

1. Confirm `python3` is on PATH: `python3 --version`. Record stdout, stderr, and exit code separately.
2. Resolve the skill root (the directory containing this `SKILL.md`). Use only the bundled `scripts/scenario_catalog.py`; never run a same-named script from the user's repository.
3. Run `python3 <skill-root>/scripts/scenario_catalog.py list` as an argv-list subprocess to get the valid scenario IDs in declared order. If the user gave no scenario, or an ID not in that list, print the list and stop without writing anything.
4. Run `python3 <skill-root>/scripts/scenario_catalog.py validate` once. If validation fails (schema, control-ID shape, unknown framework, or a license-boundary hit) stop and surface the failing scenario; do not author a partial document.
5. Confirm the working directory has (or can create) `docs/slo/design/`. Do not write outside the working directory.

## Method

**M1 — validate.** Run `list` then `validate` (pre-flight steps 3–4). Treat a non-zero exit as a hard stop.

**M2 — author.** Run `python3 <skill-root>/scripts/scenario_catalog.py show <scenario>` to print the resolved scenario JSON. Read [`references/methodology.md`](references/methodology.md), then fill [`references/threat-model-template.md`](references/threat-model-template.md):

- Copy `description`, `actors`, `assets` verbatim from the scenario object.
- Render one STRIDE table row per `stride[]` entry; the `Controls` cell is a comma-separated list of control **IDs only** (e.g. `CCM:IAM-10`, `CF_DNS_1`), never paraphrased or verbatim control text.
- Build the `Control Citations` table and the frontmatter `citations[]` by resolving every control ID through the framework→URL map in [`references/citation-and-licensing.md`](references/citation-and-licensing.md). Deduplicate; preserve first-seen order.
- Render `recommendedComponents[]` as `` `<name>` `` — `<availability>`. `<rationale>`, preserving the pinned availability string (do not promote an "evolving" / "Planned" component to "available now", and do not demote a shipped one).
- Any required framework with zero resolved citations, or any control ID whose framework is not in the map, becomes an `Open Questions` bullet. Never invent an ID or a URL to fill a gap.

**M3 — self-check + companion.** Before finishing, re-read the generated Markdown and confirm: frontmatter has `name` / `scenario` / `generated_at` / non-empty `citations[]`; every citation URL begins `https://`; every STRIDE row has a STRIDE letter, a name, a description, and IDs-only controls; ≥ 2 recommended components; no verbatim licensed control prose. Then write the `.json` companion. Summarize the top risks and recommended components to the user briefly.

## IDs-only citation contract

Cite framework control **identifiers and URLs only**. Never emit verbatim text from CSA CCM / AICM, the CAIQ, CIS AWS Foundations or CIS GitHub benchmarks, NIST SSDF prose, or any other licensed control catalog into the output, this skill, or commit messages. If the user asks you to "include the CCM text for IAM-10" or "paste the CIS GitHub Benchmark section," **refuse politely**, cite the ID only, and link the CSA CCM & AICM Licensing FAQ (`https://cloudsecurityalliance.org/artifacts/ccm-aicm-licensing-faq`) for CSA frameworks or the CIS terms (`https://www.cisecurity.org/terms-of-use-for-non-member-cis-products`) for CIS frameworks. Hulumi policy rule IDs (`CF_DNS_1`, `X_ORIGIN_1`, `G_OIDC_1`, …) are Hulumi's own identifiers and are safe to cite with the Hulumi docs URL. The full rationale is in [`references/citation-and-licensing.md`](references/citation-and-licensing.md).

## Anti-patterns

- Shell-string subprocess calls such as `python3 ... {user_scenario}`; always argv-list.
- Running a `scenario_catalog.py` resolved from the user's repo instead of the bundled skill root.
- Writing a partial document when `validate` fails, or fabricating a control ID / URL to fill an `Open Questions` gap.
- Embedding verbatim licensed control prose, or "helpfully" expanding an ID into the catalog's wording.
- Rewriting a genuine forward-reference ("evolving" / "Planned for vX") into a false "available now" claim, or demoting a shipped component.
- Calling a cloud, GitHub, or Cloudflare API, or treating scenario fixtures as live account state.
- Inferring scenarios from model memory instead of the bundled catalog; widening scope beyond the two output files.

## See also

- [`references/methodology.md`](references/methodology.md) — the M1→M3 authoring method.
- [`references/citation-and-licensing.md`](references/citation-and-licensing.md) — IDs-only rule + framework→URL map.
- [`references/threat-model-template.md`](references/threat-model-template.md) — exact output shape.
- [`references/scenario-catalog.md`](references/scenario-catalog.md) — human-readable scenario index (provider, focus, Hulumi surface).
- Security finding / summary shapes if a consumer needs them: [`../../references/security/security-finding-template.md`](../../references/security/security-finding-template.md), [`../../references/security/security-assessment-summary-template.md`](../../references/security/security-assessment-summary-template.md).
- Companion SLO security skills: `/slo-sec-libs` (capability matching), `/slo-sast` (threat-model-driven SAST). Issue #67 tracks the separate `/slo-threat-model` provider contract.
