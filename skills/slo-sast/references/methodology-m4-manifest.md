---
name: slo-sast-methodology-m4-manifest
source_skill: skills/slo-sast/SKILL.md
stage: M4
status: stable-reference
---

# /slo-sast Methodology M4 — Manifest + Preview-mode UX

## Method (M4 — manifest emission + initial-baseline preview-mode UX)

### Manifest schema v1.0

Per [`references/sast/scanner-orch-manifest-schema.md`](../../../references/sast/scanner-orch-manifest-schema.md), write `.semgrep/manifest.json` with the full schema (13 fields documented in the schema reference). Every value is **regex-validated or comes from a closed enumeration** — no free-text from user-authored content flows into JSON, defending against `tm-scanner-orchestration-abuse-4`.

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
