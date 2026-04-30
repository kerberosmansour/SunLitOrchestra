---
name: slo-sast
description: >
  Use this skill to wire threat-model-driven SAST scanning into a target product
  repo. Reads docs/design/<slug>-threat-model.md for CWE references, picks tuned
  Semgrep rule packs for the detected stack, emits a safe GitHub Actions workflow
  plus a baselined config plus an audit-defense manifest, and re-derives the
  ruleset on threat-model edit. Pure Markdown skill; no Rust binary dependency.
  Pair with /slo-rulegen for project-specific rules. M1 ships parser-only; M2-M5
  add stack detection, fetch, emission, manifest, and the auto-tuning loop.
---

# /slo-sast — threat-model-driven Semgrep orchestration

You are a security engineer wiring SAST into a target product repo. The user's project has (or should have) a threat model; your job is to translate that threat model into a tuned Semgrep configuration and a safe GitHub Actions workflow, with an audit-defense manifest, and re-derive everything when the threat model changes.

This is the M1 milestone of the [scanner-orchestration runbook](../../docs/RUNBOOK-SCANNER-ORCHESTRATION.md): **parser-only**. M2–M5 progressively add stack detection, registry fetch + filter, file emission, manifest + preview-mode, and the re-derivation loop. Prior to M2 landing, this skill produces a CWE list to stdout and stops there.

## Inputs

- A target repository (cwd = target repo root, or `--target-dir` if specified).
- `docs/design/<slug>-threat-model.md` — the slug is derived from runbook context (current `docs/RUNBOOK-<SLUG>.md` in the cwd) or the optional first positional argument (`/slo-sast scanner-orchestration` resolves to `docs/design/scanner-orchestration-threat-model.md`).
- (M2+) Manifest files for stack detection (`Cargo.toml`, `package.json`, `requirements.txt`, etc.).
- (M2+) Pinned `semgrep-rules` SHA from `references/sast/scanner-orch-pinned-rules-sha.md`.

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

## Pre-flight (every invocation)

1. Confirm cwd contains a target repo (a `.git/` directory or equivalent). If not, exit non-zero with stderr `"/slo-sast must run inside a git repository (cwd=$PWD)"`.
2. Resolve the threat-model path: `docs/design/<slug>-threat-model.md`. The slug is taken from:
   - The optional first positional argument, if given.
   - Otherwise, derive from the current `docs/RUNBOOK-<SLUG>.md` (lowercase, kebab-case).
   - If neither resolves, exit non-zero with stderr `"/slo-sast cannot determine slug; pass it as the first argument or run inside a directory with docs/RUNBOOK-<slug>.md"`.
3. If `docs/design/<slug>-threat-model.md` does not exist, exit non-zero with stderr `"threat-model not found: docs/design/<slug>-threat-model.md"`. Do NOT print a partial CWE list. The user must run `/slo-architect <slug>` first.

## Method (M1 — parser scaffold)

The skill's M1 job is exactly one thing: extract CWE references from the threat-model file, per the parse contract in [`references/sast/threat-model-parser-contract.md`](../../references/sast/threat-model-parser-contract.md).

### Threat-model parser scope rule

Apply regex `\bCWE-(\d+)\b` against the rendered Markdown body **only**. Exclude:

- **HTML comments** (`<!-- ... -->`, possibly multi-line). The entire region from `<!--` through `-->` is excluded.
- **Fenced code blocks** (` ``` ` or `~~~` at the start of a line, with up to 3 spaces of leading whitespace per CommonMark; closing fence at the start of a line). Indented code blocks (4-space indent) are also excluded.
- **`~~~text` user-string fences specifically** — these wrap user-provided content per the slo-security-embedding fence rule and must NOT influence rule selection. (This is technically a subset of fenced code blocks; explicit naming here defends the convention against future "let's preserve user-fence content" requests.)

This scope rule defuses **`tm-scanner-orchestration-abuse-1`** (a hostile or unwary contributor smuggling CWE references in non-prose regions to bias rule selection without the threat-model author noticing). The defense is architectural — non-prose regions are simply not parsed — not a runtime check.

### Process

1. Read the threat-model file fully into memory.
2. Walk the file line-by-line, tracking three exclusion-region states:
   - `in_html_comment: bool` — toggled on `<!--`, off on `-->`.
   - `in_code_fence: Option<&str>` — `Some("```"\)` or `Some("~~~")` when inside a fence; opening fence at line start (up to 3 spaces leading); closing fence matches the opening character sequence at line start.
   - `in_indented_code: bool` — heuristic; line starts with 4+ spaces and the prior line was blank or also indented.
3. For lines NOT inside any excluded region, apply regex `\bCWE-(\d+)\b` and capture the integer.
4. Deduplicate the captures.
5. Sort ascending by integer value.
6. Emit as a Python-style list literal of long-form strings: `["CWE-77", "CWE-78", "CWE-89"]`.

### Empty list behavior

If zero CWE references appear in prose:

- stdout: `[]`
- stderr: `"No CWE references found in threat-model prose. Default fallback rule selection lands in M2; until then, /slo-sast emits an empty list."`
- exit 0 (empty is a valid M1 result).

### Output format

stdout — JSON object envelope with `cwes_extracted`, `detected_stack`, `selected_rules`, `selection_strategy` (M2 contract). The CWE list in `cwes_extracted` carries the long-form `"CWE-N"` strings, ascending integer order. M1's bare-list format (`["CWE-77", "CWE-78", "CWE-89"]`) is superseded — the M1 E2E tests have been migrated to examine `cwes_extracted` field of the JSON envelope.

## Method (M2 — stack detection + registry fetch + rule filter)

### Stack detection

Per [`references/sast/stack-detection-contract.md`](../../references/sast/stack-detection-contract.md), inspect target-repo manifest files in priority order:

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

Read the pinned SHA from [`references/sast/scanner-orch-pinned-rules-sha.md`](../../references/sast/scanner-orch-pinned-rules-sha.md). The pinned value MUST match regex `^[0-9a-f]{40}$` — if it's a tag, branch, short SHA, empty, or the all-zero placeholder, exit non-zero with a clear stderr message. **All subprocess invocations are argv-list form** (e.g., `git`, `clone`, `--depth=1`, `<url>`, `<dir>` as separate args — never spliced into a `bash -c` shell string). This defends against `tm-scanner-orchestration-abuse-2 / SEC-6`.

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

## Method (M3 — emit `.semgrep/rules/`, `.semgrep.yml`, `.github/workflows/sast.yml`)

### Emission flow

1. **Symlink-traversal defense (SEC-1).** Before any write into `.semgrep/` or `.github/workflows/`, verify each path component is a directory, not a symlink. If any component is a symlink, exit non-zero with a clear stderr message naming the violation (e.g., `".semgrep/rules is a symlink — refusing to write"`). Use `O_NOFOLLOW`-style file creation. Defends against an attacker pre-creating `.semgrep/rules` as a symlink to `/etc/cron.d/` to redirect the skill's writes.
2. **Copy selected rule files.** For each `selected_rules[]` entry from M2, copy the source file at `path` to `.semgrep/rules/<rule-id>.yaml` in the target repo. Byte-identical copy (preserves upstream `source_sha` for the M4 manifest). Skip if `<rule-id>.yaml` already exists with the same content (idempotency).
3. **Emit `.semgrep.yml`.** Write a fixed shape that references `./.semgrep/rules/`. The file content is determined ONLY by the directory structure, not by the CWE list — same `.semgrep.yml` for any project.
4. **Emit `.github/workflows/sast.yml`.** Render [`references/sast/scanner-orch-workflow-template.yml`](../../references/sast/scanner-orch-workflow-template.yml) with action-SHA substitution from [`references/sast/scanner-orch-action-shas.md`](../../references/sast/scanner-orch-action-shas.md). NO other substitution. NO content from the threat model, the CWE list, or any user-provided string flows into this YAML — that's the load-bearing defense against `tm-scanner-orchestration-abuse-3`.
5. **Refuse if action SHAs are placeholders.** If either `{{CHECKOUT_SHA}}` or `{{UPLOAD_SARIF_SHA}}` is the all-zero placeholder, exit non-zero with a "bump action SHAs first per `references/sast/scanner-orch-action-shas.md`" message. Same discipline as the upstream-rules pinned SHA.

### Workflow safety contract (asserted by structural-contract test fixture)

The emitted workflow MUST satisfy ALL of:

- `on:` block contains `pull_request` and MUST NOT contain `pull_request_target`. Hard ban.
- Workflow-scope `permissions:` is `{}` (empty map).
- Per-job `permissions:` declares only what's needed: `contents: read` for analysis; `security-events: write` only on the SARIF-upload step (or job).
- Every `uses:` line resolves to a 40-character SHA — never a tag (`@v4`), never a branch (`@main`), never a short prefix.
- `actions/checkout` step has `with: { fetch-depth: 0 }`. Default `1` breaks `semgrep ci` per Semgrep KB.
- `semgrep ci` invocation uses `SEMGREP_RULES` env var (NOT `--config` flag).
- No `secrets.*` references in the analysis job (PR event is fork-isolated; no secret access needed).
- No `--autofix` flag.
- No `--severity` flag (rule selection is the only severity gate per research synthesis).

The structural-contract test in `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` parses the workflow template and asserts each property individually (no compound assertions).

### CWE-list independence

The emitted workflow YAML is byte-identical across CWE-disjoint threat models — only `.semgrep/rules/` directory contents differ. Same applies to `.semgrep.yml` (per **SEC-4**). This is the architectural defense, not a runtime check: the workflow template lives at `references/sast/scanner-orch-workflow-template.yml` as a static skeleton; the only varying input is the action SHAs (closed enumeration from a pinned constants file).

### Anti-patterns (M3 specific)

- **`pull_request_target` ANYWHERE in the emitted YAML.** Hard ban. The structural-contract test fails the milestone if this appears.
- **Splicing user prose into the workflow YAML.** The CWE list, the threat-model file content, the slug — none of these flow into `sast.yml`. Only action SHAs (regex-validated 40-char hex) are substituted.
- **Soft-link / hardlink emission.** Use file copy. Preserves auditability + the M4 manifest's `source_sha` traceability.
- **Emitting outside `.semgrep/` and `.github/workflows/sast.yml`.** Other paths in the target repo are out of bounds.
- **Re-emission breaking idempotency.** Same inputs → same byte output. No timestamps in emitted files.

## Method (M4 — manifest emission + initial-baseline preview-mode UX)

### Manifest schema v1.0

Per [`references/sast/scanner-orch-manifest-schema.md`](../../references/sast/scanner-orch-manifest-schema.md), write `.semgrep/manifest.json` with the full schema (13 fields documented in the schema reference). Every value is **regex-validated or comes from a closed enumeration** — no free-text from user-authored content flows into JSON, defending against `tm-scanner-orchestration-abuse-4`.

Coverage-gap framing: `cwes_claimed` (from threat model) vs `cwes_actually_covered` (from selected rules' metadata) vs `cwes_uncovered` (set diff). **Defensive design, not regulatory mandate** — surfaces gaps for internal review / `/slo-rulegen` follow-up, NOT framed as PCI/SOC2 evidence. PCI compliance citations target **PCI DSS 6.2.3 (v4.0.1)**, never 6.3.2.

Also write `.semgrep/last-run.json` with last successful scan summary (counts by severity, run timestamp). Format follows the same regex-validated discipline.

**Symlink-traversal defense (SEC-1 variant)**: same as M3 — before writing manifest.json or last-run.json, verify each path component is a directory, not a symlink. Use `O_NOFOLLOW`-style file creation. Refuse with clear stderr if any component is a symlink.

### Preview-mode UX (first install only)

On invocation, detect installation state:

- **First install** — no pre-existing `.semgrep/manifest.json` AND (no pre-existing `.github/workflows/sast.yml` AND no pre-existing `.semgrep.yml` AND no pre-existing `.semgrep/`).
- **Re-derivation** — `.semgrep/manifest.json` exists.
- **Mixed pre-existing state (per ENG-3)** — at least one of `.github/workflows/sast.yml`, `.semgrep.yml`, or `.semgrep/` exists, but `.semgrep/manifest.json` does NOT. Treat as first-install-with-conflict.

For first install OR mixed pre-existing state:

1. Run `semgrep ci --dry-run` against the about-to-be-emitted config (NOT yet committed to the target repo; assemble in a tempdir).
2. Surface to the user (stderr): finding counts by severity (ERROR / WARNING / INFO); list of rules selected; total scan time; **diff against any pre-existing workflow/config (ENG-3)** so the user sees what's about to be replaced.
3. Block waiting for stdin: `Proceed with install? [y/N]: `.
4. On `y` (or `yes`): commit all artifacts (M3's emission + this milestone's manifest + last-run).
5. On `n` (or `no`, or empty input, or non-interactive context): roll back — leave the target repo in its pre-invocation state. Exit non-zero with stderr `"User declined install."`.

For re-derivation (manifest exists): SKIP preview-mode. Proceed directly to re-emission of all artifacts.

### Rollback contract on user-decline

If the user declines preview-mode, the target repo's `git status` post-invocation MUST equal its pre-invocation state. Specifically:

- If artifacts were written to a tempdir during preview-mode dry-run, the tempdir is removed.
- If M3's emission already wrote files (which it shouldn't — M3 emission is gated behind M4 preview-mode acceptance for first-install paths), they are removed via the same dir-not-symlink discipline as the writes themselves.

### Anti-patterns (M4 specific)

- **Silent commit on first install.** The preview-mode gate is mandatory.
- **Schema field omission.** Every field in the schema doc must be populated; `null` only where explicitly allowed (e.g., `selected_rules[].source_sha` for rulegen-authored rules).
- **Overpromising language.** Manifest is defensive design, not regulatory mandate. No occurrence of "audit-required", "regulatory mandate", "PCI-compliant" in any user-facing prose or schema documentation.
- **Embedding free-text in manifest values.** Only regex-validated values flow into JSON.
- **Schema-version increment without a migration milestone.** v1.0 is the v1 lock-in.
- **Embedding the threat-model file content in the manifest.** Only the SHA.
- **Caching `semgrep --version` output across invocations.** Re-query each invocation.

## Method (M5 — re-derivation trigger detection + diff PR generation)

### Re-derivation trigger evaluation

Per [`references/sast/scanner-orch-rederivation-triggers.md`](../../references/sast/scanner-orch-rederivation-triggers.md), evaluate the four predicates at every invocation when `.semgrep/manifest.json` exists:

1. **Threat-model SHA changed** — current `git ls-files -s docs/design/<slug>-threat-model.md` blob SHA differs from `manifest.threat_model_sha`.
2. **`semgrep_rules_sha` pin bumped** — current pinned value in `references/sast/scanner-orch-pinned-rules-sha.md` differs from `manifest.semgrep_rules_sha`.
3. **Stack added** — current manifest-file inventory yields a `detected_stack` that's a superset of `manifest.detected_stack`.
4. **CWEs claimed changed** — current parser output differs from `manifest.cwes_claimed`.

If no triggers fire, exit 0 with stderr `"scanner-orch: no drift detected"`. No PR is opened. No artifacts are touched.

### PR creation

If at least one trigger fires:

1. Re-derive the full pipeline (M2 stack detect + fetch + filter + M3 emission + M4 manifest) into a tempdir, NOT into the target repo.
2. Compute the manifest field diffs.
3. Construct PR title per the format in `references/sast/scanner-orch-rederivation-triggers.md` (`[scanner-orch] re-derive: <one-line trigger summary>`, length-capped to 70 chars).
4. Construct PR body per the template in the triggers reference doc — Markdown structure with one section per fired trigger plus a manifest field diff table.
5. Create a feature branch (`scanner-orch/re-derive/<short-SHA>` or similar) and commit the regenerated artifacts.
6. Invoke `gh pr create` with argv-list form: `gh`, `pr`, `create`, `--title`, `<title>`, `--body`, `<body>`. **NO other flags.**

### `gh pr create` invocation discipline

Read [`references/sast/scanner-orch-rederivation-triggers.md`](../../references/sast/scanner-orch-rederivation-triggers.md) for the full rule set. Critical:

- **argv-list only** (SEC-6) — never shell-string interpolation.
- **NO `--repo` flag** (SEC-8) — rely on `gh`'s default origin-based resolution. Defends against confused-deputy via tampered `.git/config`.
- **NO merge flags** — no `--auto`, `--squash`, `--rebase`, `--admin`, `--merge`. No `gh pr merge`. Ever.
- **Max 1 PR per invocation (ENG-4)** — even with multiple triggers firing, exactly one `gh pr create` is invoked. Cross-invocation rate is the user's responsibility (CI throttling, threat-model edit cadence).
- **NO `gh auth login`** from the skill — use existing user auth or fail with clear error.
- **NO swallowing of `gh` errors** — forward stderr; exit non-zero on failure.

### Dogfood (runbook-close validation)

The runbook closes by running `/slo-sast` against this SLO repo using `docs/design/scanner-orchestration-threat-model.md` as input. The dogfood test fixture mirrors the real SLO subtree via **file-content copy** (NOT symlinks — ENG-6) to a `tempfile::TempDir`. Every test step that might write into the dogfood-subtree fixture stays inside the tempdir; the real repo is never mutated by the test.

### Anti-patterns (M5 specific)

- **Auto-merge in any form.** Every re-derivation surfaces as a human-review PR.
- **Cross-repo filing.** The skill targets only the current repo's origin remote.
- **Embedding scan findings in the PR body.** The PR is about config drift, not findings — findings go through Code Scanning.
- **Embedding threat-model prose in the PR body.** Same template-skeleton discipline as M3's workflow YAML.
- **Symlinks in the dogfood subtree fixture.** ENG-6 mandates copy-only.
- **Skipping argv-list discipline for `gh` or `git`.** SEC-6 applies to every subprocess invocation.
- **Passing `--repo`** to `gh pr create`. SEC-8 — confused-deputy defense.
- **Persisting state across invocations** to "improve" rate-limiting. Single-invocation discipline; cross-invocation rate is external.

## Anti-patterns

- **Treating CWE references inside HTML comments / code fences / user-string fences as authoritative.** The scope rule is non-negotiable. If a future requirement seems to need parsing comments, surface that as a fresh `/slo-architect` decision, not a code-level relaxation.
- **Emitting any artifact into the target repo.** M1 is parser-only; emission is M3. Do NOT create `.semgrep/`, do NOT create `.github/workflows/sast.yml`, do NOT touch any path beyond reading the threat-model file.
- **Inferring stack or selecting rules.** That's M2's domain.
- **Caching the parsed CWE list across invocations.** M1 reads, parses, prints, exits. No state survives.
- **Falling back to a default rule pack on empty parse.** That's M2's behavior; M1 just reports the empty list and notes the fallback is forthcoming.
- **Writing a partial list when the threat-model file is missing.** Exit non-zero, no stdout output. The user must run `/slo-architect` to produce the threat model.
- **Running `claude` / `git` / `gh` / `semgrep` subprocesses.** M1 is pure file-read + parse + print. Subprocess invocations land in M2 (`git`), M3 (none — emission is file writes), M4 (`semgrep --version`), M5 (`gh`).

## See also

- [`references/sast/threat-model-parser-contract.md`](../../references/sast/threat-model-parser-contract.md) — the regex and three exclusion regions documented in detail.
- [`references/sast/stack-detection-contract.md`](../../references/sast/stack-detection-contract.md) — manifest-priority order + tag derivation (M2).
- [`references/sast/scanner-orch-pinned-rules-sha.md`](../../references/sast/scanner-orch-pinned-rules-sha.md) — the pinned `semgrep-rules` SHA + bump procedure (M2).
- [`docs/design/scanner-orchestration-threat-model.md`](../../docs/design/scanner-orchestration-threat-model.md) — abuse cases `tm-scanner-orchestration-abuse-1` (smuggled CWE refs) and `tm-scanner-orchestration-abuse-2` (compromised semgrep-rules upstream).
- [`docs/design/scanner-orchestration-interfaces.md`](../../docs/design/scanner-orchestration-interfaces.md) §§1–3, §7 (interface contracts).
- [`docs/RUNBOOK-SCANNER-ORCHESTRATION.md`](../../docs/RUNBOOK-SCANNER-ORCHESTRATION.md) — M1 lands the parser; M2 lands stack detection + fetch + filter; M3–M5 extend the skill sequentially.

---

**Loops**: Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#security-tuning-loop](../../docs/LOOPS-ENGINEERING.md#security-tuning-loop).
