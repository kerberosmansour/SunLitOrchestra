---
name: slo-sast-methodology-m2-stack-detect
source_skill: skills/slo-sast/SKILL.md
stage: M2
status: stable-reference
---

# /slo-sast Methodology M2 — Stack Detection + Registry Fetch + Rule Filter

## Outputs (M2 current; M1 was a parser-only stdout list)

A single JSON object envelope printed to stdout:

```json
{
  "cwes_extracted": ["CWE-77", "CWE-78", "CWE-89"],
  "detected_stack": ["python", "rust"],
  "selected_rules": [
    {
      "path": "/Users/.../sldo/semgrep-rules/<SHA>/python/django/security/injection/sql/sql-injection-using-raw.yaml",
      "rule_id": "python.django.security.injection.sql.sql-injection-using-raw",
      "source_sha": "<rule-file-blob-SHA>",
      "metadata_cwe": ["CWE-89"],
      "metadata_technology": ["django", "python"]
    }
  ],
  "selection_strategy": "threat-model-cwe"
}
```

Deduplicated, with `cwes_extracted` and `detected_stack` sorted ascending. Empty arrays where applicable. Empty `detected_stack` triggers `selection_strategy: "default-fallback"` (rule filter falls back to language-agnostic rules — see `references/sast/stack-detection-contract.md`). Stderr carries operational notes.

Future milestones extend this:

- **M3** emits files into the target repo: `.semgrep/rules/<rule-id>.yaml` (copies from cache), `.semgrep.yml`, `.github/workflows/sast.yml`.
- **M4** adds `.semgrep/manifest.json` (audit-defense schema v1.0) plus first-install preview-mode UX.
- **M5** detects re-derivation triggers and surfaces drift as a GitHub PR.

### Coverage-gap reporting

When a `/slo-sast` run produces a non-empty `unmatched_cwes` set or a stack-detection mismatch, the run MAY surface a coverage-gap summary using the shared assessment-summary template at [`../../references/security/security-assessment-summary-template.md`](../../../references/security/security-assessment-summary-template.md). Individual high-severity gaps (e.g., a CWE in the threat model with zero matching rules) MAY be expanded into a per-finding entry using [`../../references/security/security-finding-template.md`](../../../references/security/security-finding-template.md) when the compact summary cell would hide standards mapping, evidence, or remediation detail.

**Standards mapping** — coverage-gap rows have `CWE claimed vs covered` as the **required** mapping per the per-output-type tier matrix at [`../../references/security/standards-mapping.md`](../../../references/security/standards-mapping.md). OWASP / ASVS rationale is **optional**. Live OpenCRE lookup is explicitly out of scope; consult the curated table for OpenCRE ids when available.

## Method (M2 — stack detection + registry fetch + rule filter)

### Stack detection

Per [`references/sast/stack-detection-contract.md`](../../../references/sast/stack-detection-contract.md), inspect target-repo manifest files in priority order:

1. `Cargo.toml` → `rust`.
2. `package.json` → `javascript`, plus `typescript` if `tsconfig.json` present.
3. `requirements.txt` / `pyproject.toml` / `Pipfile` → `python`; framework hints from declared deps.
4. `go.mod` → `go`.
5. `pom.xml` / `build.gradle(.kts)` → `java`.
6. `Gemfile` → `ruby`.
7. `composer.json` → `php`.
8. `Package.swift` / `*.xcodeproj` → `swift`.

Polyglot repos emit ALL detected tags. Empty detection (no manifest match) → `detected_stack: []` and `selection_strategy: "default-fallback"`.

### Registry fetch

Read the pinned SHA from [`references/sast/scanner-orch-pinned-rules-sha.md`](../../../references/sast/scanner-orch-pinned-rules-sha.md). The pinned value MUST match regex `^[0-9a-f]{40}$` — if it's a tag, branch, short SHA, empty, or the all-zero placeholder, exit non-zero with a clear stderr message. **All subprocess invocations are argv-list form** (e.g., `git`, `clone`, `--depth=1`, `<url>`, `<dir>` as separate args — never spliced into a `bash -c` shell string). This defends against `tm-scanner-orchestration-abuse-2 / SEC-6`.

Cache layout: `~/.cache/sldo/semgrep-rules/<SHA>/` (or `$XDG_CACHE_HOME/sldo/semgrep-rules/<SHA>/`). On cache miss, `git clone` into the SHA-suffixed directory then `git rev-parse HEAD` to verify the resulting checkout matches the pinned SHA — wipe and refuse if mismatched (defends against in-flight tag-rewriting). On cache hit, skip `git clone` (a defense-in-depth `git rev-parse HEAD` for cache integrity verification IS allowed and expected).

### Rule filter

Walk the cached `semgrep-rules/<SHA>/` tree. For each `*.yaml` rule file:

1. Parse with `serde_yaml_ng` default settings — **no entity expansion / no anchor recursion**, defending against billion-laughs (`tm-scanner-orchestration-abuse-2 / SEC-2`). Reject any individual YAML file > 1 MiB before parse.
2. Read `metadata.cwe` (a list of long-form `"CWE-N: ..."` strings) and `metadata.technology` (a list of stack tags, possibly absent for legacy rules).
3. Filter: include the rule iff `metadata.cwe[*]` prefix-matches some `cwe ∈ cwes_extracted` AND (`metadata.technology[*]` intersects `detected_stack` OR `metadata.technology` absent or empty — language-agnostic rules included for any stack).
4. For each selected rule, capture: `path`, `rule_id` (from the YAML's top-level `rules[0].id` or a derived path-based id), `source_sha` (the rule file's git-blob SHA), `metadata_cwe` (the rule's CWE list, integer-prefix form), `metadata_technology` (the rule's tech list).

Output the JSON envelope shape above. Sort `selected_rules[]` by `rule_id` ascending for determinism.

### Anti-patterns (M2 specific)

- **Tag/branch references in the pinned SHA** — refuse on non-40-char input.
- **Shell-string subprocess invocation** — argv-list form only, always.
- **Caching parsed YAML data across invocations** — re-parse per call to keep the memory footprint deterministic and avoid stale-cache bugs.
- **Vendor SaaS API fallbacks** — Semgrep AppSec was rejected in stack-decision; do not invoke it.
- **Autofix in any path** — defense against compromised-rule autofix backdoors.
