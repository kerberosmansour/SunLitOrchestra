# Interfaces — scanner-orchestration

Public surfaces downstream milestones (v2 `/slo-dast`, the `/slo-rulegen` integration, future audit-coverage skills) MUST keep stable. Each entry has a stability level: `stable` (frozen, breaking change requires a fresh `/slo-architect` run), `evolving` (may change with explicit migration in a future runbook), `internal` (fair game, not consumed across skill boundaries).

## 1. Skill invocation surface — `/slo-sast`

**Stability:** `stable`.

- **Command form:** `/slo-sast` (no args — operates on current target repo, derives slug from runbook context).
- **Optional arg:** `/slo-sast <slug>` — explicit threat-model slug; resolves to `docs/slo/design/<slug>-threat-model.md`.
- **SKILL.md frontmatter** keys read by Claude Code:
  - `name: slo-sast` (`stable`)
  - `description:` — single-line, ≤ 200 chars (`stable`)
  - `mode_arg:` — N/A (single-mode skill in v1)

## 2. Threat-model parse contract

**Stability:** `stable`.

The skill extracts CWE references from `docs/slo/design/<slug>-threat-model.md` using:

- Regex `\bCWE-(\d+)\b` against the rendered Markdown body (NOT inside HTML comments, NOT inside fenced code blocks, NOT inside `~~~text` user-string fences — those are user content that must not influence rule selection).
- Deduplication on the integer CWE id.
- Long-form lookup against the cached `semgrep-rules/<SHA>/` registry: `metadata.cwe` field starts with `"CWE-NN:"` (NN matching the extracted integer).

If the threat model contains zero CWE references, the skill SHALL fall back to a documented default set (initially: top-25 from `p/cwe-top-25` filtered by detected stack); this fallback is logged in the manifest's `selection_strategy: default` field.

## 3. Stack detection contract

**Stability:** `stable`.

The skill detects stack tags from these manifest files in this priority order:

| File | Detected technologies |
|---|---|
| `Cargo.toml` | `rust` |
| `package.json` | `javascript`, `typescript` (latter when `tsconfig.json` also present); framework hints: `react`, `nextjs`, `express`, `nestjs` from dependencies |
| `requirements.txt` / `pyproject.toml` / `Pipfile` | `python`; framework hints: `django`, `flask`, `fastapi` from declared deps |
| `go.mod` | `go` |
| `pom.xml` / `build.gradle(.kts)` | `java`; framework hints: `spring` from declared deps |
| `Gemfile` | `ruby`; `rails` from declared deps |
| `composer.json` | `php`; framework hints: `laravel`, `symfony` |
| `Package.swift` / `*.xcodeproj` | `swift` |

Multiple manifests detected → emit all detected tags (polyglot repos legitimately produce multi-stack rule selection). Empty detection → manifest records `detected_stack: []` and the skill falls back to language-agnostic registry rules tagged with the relevant CWE.

## 4. Emitted artifact paths

**Stability:** `stable`.

| Path | Purpose | Format |
|---|---|---|
| `.semgrep/rules/<rule-id>.yaml` | Selected registry rules, copied verbatim from the pinned semgrep-rules clone | Semgrep YAML |
| `.semgrep.yml` | Project-level Semgrep config; `rules:` references `./.semgrep/rules/` | Semgrep YAML |
| `.github/workflows/sast.yml` | Safe-template GitHub Actions workflow | GitHub Actions YAML |
| `.semgrep/manifest.json` | Audit-defense + reproducibility manifest | JSON (schema below) |
| `.semgrep/last-run.json` | Last successful scan summary (counts by severity, run timestamp) | JSON |

The skill MUST NOT write outside `.semgrep/` and `.github/workflows/sast.yml`. Other paths in the target repo are out of bounds without a fresh `/slo-architect` decision.

## 5. Manifest schema — `.semgrep/manifest.json`

**Stability:** `stable` for the listed fields; `evolving` for additive fields (downstream MUST tolerate unknown fields per the JSON-extensibility convention).

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
      "source_sha": "<rule-file-SHA-in-semgrep-rules-clone>",
      "metadata_cwe": ["CWE-89"],
      "metadata_technology": ["django"]
    }
  ]
}
```

`cwes_claimed` MUST equal the deduplicated CWE list extracted from the threat model. `cwes_actually_covered` MUST equal the union of `metadata.cwe` integers from `selected_rules`. `cwes_uncovered = cwes_claimed \ cwes_actually_covered`. The divergence between claimed and covered is the **defensive design** signal the synthesis flagged — it surfaces gaps so the user can route them to manual review or to `/slo-rulegen`.

## 6. Workflow YAML safety contract — `.github/workflows/sast.yml`

**Stability:** `stable` (the safety properties are non-negotiable; layout details may evolve).

The emitted workflow MUST satisfy all of:

- `on:` block contains `pull_request` and `schedule` (weekly cron); MUST NOT contain `pull_request_target`.
- `permissions:` at workflow scope is `{}` (empty map).
- Per-job `permissions:` block declares only what that job needs:
  - Analysis job: `contents: read`.
  - SARIF upload job/step: adds `security-events: write`.
- Every `uses:` line resolves to a 40-character SHA (no `@v4`, no `@main`).
- The `actions/checkout` step has `with: { fetch-depth: 0 }`.
- The Semgrep invocation uses `SEMGREP_RULES` env var (NOT `--config` flag); the value points at `./.semgrep.yml`.
- No `secrets.*` references in the analysis job (it runs on PR events; secrets are not exposed by `pull_request`).

A structural-contract test fixture asserts these properties on every emitted workflow.

## 7. Cache layout — `~/.cache/sldo/semgrep-rules/<SHA>/`

**Stability:** `evolving` (path may move under XDG_CACHE_HOME variations; the SHA-suffixed subdir layout is `stable`).

- `<SHA>` is the 40-character commit SHA of the cloned `semgrep-rules` snapshot.
- Multiple SHAs coexist (one per pinned version the skill has seen).
- Cache pruning is the user's responsibility for v1; pruning skill is out of scope.

## 8. Re-derivation trigger contract

**Stability:** `evolving`.

The skill considers the workflow stale and surfaces a diff PR when any of these hold:

- The threat-model file's git-blob SHA differs from the `threat_model_sha` recorded in `.semgrep/manifest.json`.
- The pinned `semgrep_rules_sha` differs from the SHA the skill would currently fetch.
- A new manifest file appears in the target repo (e.g., new `package.json` indicates added stack).
- The `cwes_claimed` extracted now differs from the one recorded.

Trigger detection runs at every `/slo-sast` invocation. Automated triggers (file-watcher, CI-side scheduled invocation) are out of v1 scope.

## 9. `/slo-rulegen` integration contract

**Stability:** `stable`.

- `/slo-rulegen` writes project-specific Semgrep rules to `.semgrep/rules/<rule-id>.yaml` using the same metadata schema (must include `metadata.cwe` long-form, `metadata.technology`).
- `/slo-sast` consumes any file under `.semgrep/rules/` regardless of authoring source — registry-copied vs `/slo-rulegen`-authored — and treats them uniformly.
- The manifest's `selected_rules[].source_sha` is `null` for `/slo-rulegen`-authored rules (no upstream pin); the manifest's `selected_rules[].source` field distinguishes `registry` vs `rulegen`.

This is the integration boundary the idea doc's Q4 second-product answer locks in.

## 10. Out-of-bounds (explicitly NOT interfaces)

These are `internal` and may change without notice:

- The exact regex used to extract CWE references from threat-model prose (the `\bCWE-(\d+)\b` shape is in the contract above, but the surrounding parsing logic — comment skipping, fence skipping — is internal).
- The cache pruning algorithm (none in v1).
- The PR title / body format the skill uses when opening a re-derivation PR.
- The structural-contract test fixture file paths.
