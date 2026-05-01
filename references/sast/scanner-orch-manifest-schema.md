# Scanner-orchestration manifest schema v1.0

> The audit-defense + reproducibility manifest `/slo-sast` writes at `.semgrep/manifest.json` in the target repo. Locked in M4 of the [scanner-orchestration runbook](../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md) and [`docs/slo/design/scanner-orchestration-interfaces.md` §5](../../docs/slo/design/scanner-orchestration-interfaces.md). **Defensive design, not regulatory mandate** — see Framing below.

## Schema (v1.0)

```json
{
  "schema_version": "1.0",
  "generated_at": "2026-04-26T15:30:00Z",
  "generated_by_skill_version": "0.1.0",
  "threat_model_path": "docs/slo/design/scanner-orchestration-threat-model.md",
  "threat_model_sha": "<git-blob-SHA-of-threat-model-file>",
  "semgrep_rules_sha": "<git-commit-SHA-of-semgrep-rules-clone>",
  "semgrep_version": "1.161.0",
  "detected_stack": ["rust", "javascript"],
  "selection_strategy": "threat-model-cwe",
  "cwes_claimed": ["CWE-77", "CWE-78", "CWE-89"],
  "cwes_actually_covered": ["CWE-78", "CWE-89"],
  "cwes_uncovered": ["CWE-77"],
  "selected_rules": [
    {
      "path": ".semgrep/rules/python.django.security.injection.sql.sql-injection-using-raw.yaml",
      "rule_id": "python.django.security.injection.sql.sql-injection-using-raw",
      "source_sha": "<rule-file-blob-SHA-in-cache>",
      "source": "registry",
      "metadata_cwe": ["CWE-89"],
      "metadata_technology": ["django"]
    }
  ]
}
```

## Field-by-field

| Field | Type | How populated | Why |
|---|---|---|---|
| `schema_version` | string `"1.0"` (literal) | Hardcoded at v1 | Migration knob — bumping to `"1.1"` is non-breaking (additive); `"2.0"` requires a migration milestone. |
| `generated_at` | ISO-8601 UTC timestamp | `chrono::Utc::now().to_rfc3339()` (or stable equivalent) | Human-readable timestamp for audit trail. |
| `generated_by_skill_version` | semver string | Read from a constant in SKILL.md (`0.1.0` for v1) | Identifies which skill version produced this manifest — important when the skill itself evolves and produces older / newer schema variants. |
| `threat_model_path` | relative path string | Resolved from runbook context or arg | Audit-trail breadcrumb. |
| `threat_model_sha` | git-blob SHA (40-char) | `git ls-files -s <path>` or `git hash-object <path>` | Reproducibility — any future re-derivation can compare against this. |
| `semgrep_rules_sha` | git-commit SHA (40-char) | Read from `references/sast/scanner-orch-pinned-rules-sha.md` | The trust-window anchor. |
| `semgrep_version` | semver string | `semgrep --version` output | Distinguishes which Semgrep CLI ran the scan. |
| `detected_stack` | array of strings | M2's stack-detection output, sorted ascending | Reproducibility + audit-trail. |
| `selection_strategy` | enum `"threat-model-cwe" \| "default-fallback"` | Determined by whether the threat model produced any CWE refs | Distinguishes "I selected based on the threat model" from "I fell back to language-agnostic default-pack" — important context for reviewers. |
| `cwes_claimed` | array of regex-validated `"CWE-N"` strings | Deduplicated parser output from M1 (sorted ascending by integer) | What the threat model claims to cover. |
| `cwes_actually_covered` | array of regex-validated `"CWE-N"` strings | Union of `metadata_cwe` integers across all `selected_rules[]`, deduplicated and sorted | What the rule selection actually addresses. |
| `cwes_uncovered` | array of regex-validated `"CWE-N"` strings | `cwes_claimed` set-minus `cwes_actually_covered`, sorted | The defensive-design surface — gaps the user can route to manual review or `/slo-rulegen`. |
| `selected_rules` | array of objects | M2's `selected_rules[]` envelope, sorted by `rule_id` | Per-rule reproducibility data. |
| `selected_rules[].path` | relative path string | Path in the target repo (`.semgrep/rules/<rule-id>.yaml`) | Where the rule actually lives post-M3 emission. |
| `selected_rules[].rule_id` | string | YAML's top-level `rules[0].id` or path-derived | Stable identifier across rule-file moves. |
| `selected_rules[].source_sha` | git-blob SHA (40-char) OR `null` | The rule file's git-blob SHA in the cache. **`null` for `/slo-rulegen`-authored rules** (no upstream pin). | Audit traceability. |
| `selected_rules[].source` | enum `"registry" \| "rulegen"` | `"registry"` for cache-derived rules; `"rulegen"` for `/slo-rulegen`-authored | Distinguishes integration boundary per the M5 / future `/slo-rulegen` integration contract. |
| `selected_rules[].metadata_cwe` | array of regex-validated `"CWE-N"` strings | Parsed from rule YAML's `metadata.cwe` (long-form prefix-extracted to integer-form) | Per-rule CWE coverage. |
| `selected_rules[].metadata_technology` | array of strings (possibly empty for legacy rules with absent `metadata.technology`) | Parsed from rule YAML | Per-rule stack relevance. |

## Validation rules

- Every value MUST be regex-validated or come from a closed enumeration. **No free-text from user-authored content (the threat-model file) flows into any field** — defends against `tm-scanner-orchestration-abuse-4` (manifest content injection).
- Specifically: `cwes_claimed` / `cwes_actually_covered` / `cwes_uncovered` / `metadata_cwe` are all regex-validated `^CWE-\d+$`. `*_sha` fields are regex-validated `^[0-9a-f]{40}$` (or `null` where allowed). `selection_strategy` and `source` are closed enumerations.
- `cwes_uncovered = cwes_claimed \ cwes_actually_covered` MUST hold by construction — never computed independently.
- Output JSON MUST round-trip through `serde_json::from_str` + `serde_json::to_string_pretty` byte-identically (deterministic key order via `BTreeMap` serialization).

## Framing — defensive design, not regulatory mandate

The `cwes_claimed` vs `cwes_actually_covered` divergence is a defensive-design property — it surfaces gaps so the user can route them to manual review or `/slo-rulegen`. **This is NOT framed as required-for-audit content.** Multiple research rounds did NOT surface a published QSA-firm postmortem demonstrating "mapped-but-not-scanned CWE coverage claim" as a documented audit-failure pattern. The manifest is useful, but its usefulness is internal (helping the team see gaps), not external (audit acceptance).

Wherever this manifest is described to users — in `/slo-sast` output, in any future audit-coverage-doc skill, in user-facing prose — the framing is:

- ✅ "Defensive design — surfaces gaps for follow-up."
- ✅ "Useful for internal review and `/slo-rulegen` prioritization."
- ❌ NOT "PCI DSS 6.2.3 evidence."
- ❌ NOT "SOC 2 CC7.1 mandatory artifact."
- ❌ NOT "regulatory mandate."

The same constraint applies to PCI DSS 6.2.3 references elsewhere — cite **6.2.3 (v4.0.1)**, never 6.3.2 (which is v3.2.1 numbering for code review OR v4.0.1's separate SBOM-inventory mandate).

## Stability

This schema is **`stable`** at v1.0 per [`docs/slo/design/scanner-orchestration-interfaces.md` §5](../../docs/slo/design/scanner-orchestration-interfaces.md). Adding fields is non-breaking (additive — downstream consumers MUST tolerate unknown fields). Renaming or removing fields requires a migration milestone with explicit handling for existing manifests at the prior schema_version.
