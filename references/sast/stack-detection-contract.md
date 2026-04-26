# Stack detection contract

> The contract `/slo-sast` follows for detecting target-repo stack from manifest files. Locked in M2 of the [scanner-orchestration runbook](../../docs/RUNBOOK-SCANNER-ORCHESTRATION.md). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md). The detected stack tags feed the rule-filter intersection (`metadata.cwe ∋ CWE-N ∧ metadata.technology ∋ <stack>`).

## Manifest priority order

The skill inspects these files at the target-repo root (cwd), in priority order. Multiple matches produce a polyglot stack tag set — every detected language flows into the filter, not just the first match.

| Priority | File | Detected technologies |
|---|---|---|
| 1 | `Cargo.toml` | `rust` |
| 2 | `package.json` | `javascript`, plus `typescript` if `tsconfig.json` is also present at root or `package.json` declares `"types"`/`"typescript"`. Framework hints from declared deps in `dependencies` / `devDependencies`: `react`, `nextjs`, `express`, `nestjs`, `vue`, `angular`. |
| 3 | `requirements.txt` / `pyproject.toml` / `Pipfile` (any of these) | `python`. Framework hints from declared deps: `django`, `flask`, `fastapi`, `pyramid`. |
| 4 | `go.mod` | `go`. Framework hints from declared modules: `gin`, `echo`, `fiber`. |
| 5 | `pom.xml` / `build.gradle` / `build.gradle.kts` (any) | `java`. Framework hints from declared deps: `spring`, `springboot`. |
| 6 | `Gemfile` | `ruby`. Framework hint: `rails` if `gem 'rails'` declared. |
| 7 | `composer.json` | `php`. Framework hints: `laravel`, `symfony`. |
| 8 | `Package.swift` / `*.xcodeproj` | `swift` (no further hints in v1). |

The priority order matters only for tie-breaking when two manifests describe overlapping content; in normal polyglot repos all matches contribute.

## Tag derivation rules

For each detected manifest:

1. Parse the file (TOML, JSON, YAML as appropriate). YAML parsing uses `serde_yaml_ng` with default settings (no entity expansion, no anchor recursion — defends against billion-laughs / `tm-scanner-orchestration-abuse-2` / SEC-2).
2. Emit the base language tag (e.g., `rust` for `Cargo.toml`).
3. Emit framework hints if the relevant declared dependencies are present. Match against the closed enumeration above; unknown frameworks are NOT emitted (no string literals from manifests escape into the tag set — input validation per OWASP C7).
4. Deduplicate tags across manifests.
5. Sort the tag set lexicographically before output (deterministic order for downstream consumers + manifest reproducibility).

## Empty / no-stack-detected behavior

If zero manifest files match (e.g., a docs-only repo, a repo with custom build infra not in the priority list):

- `detected_stack: []` in the JSON output.
- The rule filter falls back to **language-agnostic mode**: only rules with `metadata.cwe ∋ <CWE>` AND `metadata.technology` absent (or empty) are selected.
- The manifest records `selection_strategy: "default-fallback"` (not the standard `"threat-model-cwe"` strategy).

## Polyglot behavior

Repos with multiple manifests (e.g., a Rust backend + JavaScript frontend) emit both tags:

```json
{"detected_stack": ["javascript", "rust"]}
```

The rule filter then matches against ANY of the detected tags (`metadata.technology ∋ rust` OR `metadata.technology ∋ javascript`). This produces a larger but more complete rule set; the user's threat model + CWE choices remain the primary scoping mechanism.

## Out of scope for v2 → only handle in v2

- Submodule-aware stack detection (a Rust repo with a Python submodule).
- Custom-tag injection (user wants to force `metadata.technology: my-custom-framework`).
- Heuristic stack detection from source files (`*.rs` count, etc.) when no manifest is present — risky / fragile.
- Build-system tags beyond the 8 above (e.g., `bazel`, `bun`, `deno`, `make` — possibly v2).

## Stability

This contract is `stable` per [`docs/design/scanner-orchestration-interfaces.md` §3](../../docs/design/scanner-orchestration-interfaces.md). Adding a manifest type to the priority order is `evolving` (additive, non-breaking — existing detections keep working). Removing or renaming a tag requires a fresh `/slo-architect` decision and a migration milestone.
