---
name: slo-sast-methodology-m3-emission
source_skill: skills/slo-sast/SKILL.md
stage: M3
status: stable-reference
---

# /slo-sast Methodology M3 — Emission

## Method (M3 — emit `.semgrep/rules/`, `.semgrep.yml`, `.github/workflows/sast.yml`)

### Emission flow

1. **Symlink-traversal defense (SEC-1).** Before any write into `.semgrep/` or `.github/workflows/`, verify each path component is a directory, not a symlink. If any component is a symlink, exit non-zero with a clear stderr message naming the violation (e.g., `".semgrep/rules is a symlink — refusing to write"`). Use `O_NOFOLLOW`-style file creation. Defends against an attacker pre-creating `.semgrep/rules` as a symlink to `/etc/cron.d/` to redirect the skill's writes.
2. **Copy selected rule files.** For each `selected_rules[]` entry from M2, copy the source file at `path` to `.semgrep/rules/<rule-id>.yaml` in the target repo. Byte-identical copy (preserves upstream `source_sha` for the M4 manifest). Skip if `<rule-id>.yaml` already exists with the same content (idempotency).
3. **Emit `.semgrep.yml`.** Write a fixed shape that references `./.semgrep/rules/`. The file content is determined ONLY by the directory structure, not by the CWE list — same `.semgrep.yml` for any project.
4. **Emit `.github/workflows/sast.yml`.** Render [`references/sast/scanner-orch-workflow-template.yml`](../../../references/sast/scanner-orch-workflow-template.yml) with action-SHA substitution from [`references/sast/scanner-orch-action-shas.md`](../../../references/sast/scanner-orch-action-shas.md). NO other substitution. NO content from the threat model, the CWE list, or any user-provided string flows into this YAML — that's the load-bearing defense against `tm-scanner-orchestration-abuse-3`.
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
