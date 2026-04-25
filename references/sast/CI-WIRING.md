# CI + dev-env wiring for the SAST rule pack

Read by: developers wiring `/slo-rulegen`-generated rule packs into a Rust project's CI and local-dev hooks.

Two surfaces are wired in M3:
1. **GitHub Actions workflow** at `.github/workflows/semgrep.yml` — PR-blocking.
2. **Local pre-commit hook** at `.pre-commit-config.yaml` — runs under both `pre-commit` (Python) and `prek` (Rust drop-in).

A third entry point — the **cargo-audit-driven extend trigger** — is documented below as **developer-initiated only**, NOT a CI auto-run.

## GitHub Actions workflow

The shipped `.github/workflows/semgrep.yml` has two jobs:

### Job 1 — `admission-control`

Runs `cargo xtask sast-verify gate` against every `.yaml` in `.semgrep/<lang>/`. Per `/slo-critique` sec-4 reframe, this is the read-only admission-control step that catches direct-edit additions of rules that bypassed the skill's write-time `gate` (threats `tm-sast-rulegen-skill-pack-abuse-7` and `-8`).

If a contributor lands a rule directly via `Write` (skipping `/slo-rulegen`), this job catches it on the next PR.

### Job 2 — `semgrep`

Runs `semgrep ci --config .semgrep/` against the user's source. Fails the PR on any finding (severity ≥ WARNING by default).

### Hard NOT-DOs (per BDD)

- **NEVER invoke `/slo-rulegen`, `--extend`, or any rule-generation path** in the workflow. Tested by BDD `workflow_does_not_invoke_extend_or_rulegen_paths`. Threat-model row `tm-sast-rulegen-skill-pack-abuse-3` is the load-bearing reason.
- **NEVER pin `returntocorp/semgrep-action` by tag**; pin by SHA. Tested by BDD `workflow_invokes_pinned_semgrep_action`.

### Pin maintenance

- `actions/checkout` SHA: pinned to `692973e3d937129bcbf40652eb9f2f61becf3332` (v4.1.7). Update via `gh api repos/actions/checkout/git/refs/tags/v4` and re-pin.
- `returntocorp/semgrep-action` SHA: pin to the latest release tag's SHA at deploy time. The workflow currently has the `uses:` line gated by `if: false` and falls back to direct `semgrep` CLI invocation while the action SHA is being maintained.
- Semgrep pinned at `>= 1.50.0` per `references/sast/MIN-SEMGREP-VERSION.md`.

## Local pre-commit hook

`.pre-commit-config.yaml` declares the Semgrep hook from `https://github.com/semgrep/pre-commit`. It works under both:

- **`pre-commit`** (https://pre-commit.com/) — Python-based, canonical. `pip install pre-commit && pre-commit install`.
- **`prek`** (https://github.com/j178/prek; v0.3.10+) — Rust-native drop-in replacement. Reads the same `.pre-commit-config.yaml` unchanged. `cargo install prek && prek install`. Used by CPython and Apache Airflow.

`lefthook` is explicitly NOT supported per the synthesis design rule "lefthook is out — adds friction without a clear win for this skill pack."

The hook excludes test fixtures (`xtasks/sast-verify/tests/fixtures/`, `.semgrep/rust/cwe-*.rs`, `tests/fixtures/`) so the deliberately-bad code there doesn't fire the hook.

## cargo-audit-driven extend trigger (developer-initiated)

When `cargo audit --json` reports a new advisory landing in the project's `Cargo.lock`:

```bash
cargo audit --json | jq '.vulnerabilities.list[]' > /tmp/advisory.json
# Read the advisory, identify the CWE class, find the fix PR linked from the advisory.
# Then in your local Claude Code session:
/slo-rulegen --extend \
  --bug-summary /tmp/advisory.json \
  --fix-diff /tmp/fix-diff.patch \
  --file-paths "$(echo /tmp/advisory.json | jq -r '.affected_paths | join(\",\")')"
```

This is **developer-initiated only** — invoked manually in a local Claude Code session. It is NOT auto-fired by CI. Auto-firing in CI is forbidden per `tm-sast-rulegen-skill-pack-abuse-3`: a malicious PR could craft a bug summary with prompt-injection that the LLM might follow if the workflow auto-invokes extend-mode on PR diffs.

The extend-mode skill itself enforces the prompt-injection-resistance posture (`~~~text` fences, toolflag denial of WebFetch/WebSearch). But the architectural separation — CI runs the existing pack, developers extend it — is the primary defense and is why the workflow YAML is BDD-asserted to never invoke `--extend`.

## Two-tier corpus rendering in CI logs

When CI runs `semgrep ci`, findings include the matching source line. For a Confidential-tier corpus (gitignore'd in user app repos), the snippets in `<rule-id>.rs` fixture files are not committed and so do not appear in PR diffs. Public-tier corpora (rule-pack repos) ship snippets explicitly per `references/sast/AUTHORING.md`.

Either way: the CI job's logs show only matches in the user's source code, not the test fixtures. Test fixtures are excluded from the scan via the `paths.exclude` block in the rule YAML (per `references/sast/manifest-schema.md`).

## Workflow-level kill switches

If a generated rule is over-noisy in CI:

1. Run `cargo xtask sast-verify gate <rule>.yaml --clean-dir <a-noisier-fixture-dir>` locally and confirm the rule fails — this is the legitimate signal that the rule is too broad.
2. Either tighten the rule (preferred) OR add a `paths.exclude` block to the rule's `paths:` field so it skips specific files.
3. NEVER set the rule's severity to `INFO` to silence it in CI without addressing the root cause. INFO rules are advisory; they exist for genuinely low-confidence patterns, not as a workaround for noisy rules.

If the entire pack is generating false positives and you need to disable it for a critical merge, the legitimate escape hatch is per [SECURITY.md](../../SECURITY.md) "Escape hatches":

```
Escape hatch: SAST rule pack disabled in CI workflow
Reason: pack is generating false positives blocking critical hotfix merges
Residual risk: known-bug-class regressions can land while the pack is disabled
Scope: <commit hash> or <PR number>
Approved by: <reviewer>
```

Document the escape hatch in the PR description AND in the milestone Evidence Log if the bypass is part of an active runbook.

## Out of scope for v1

- GitLab CI templates (BuildKite, CircleCI, etc.) — the workflow shipped is GitHub-Actions-specific. Generic instructions: install Semgrep ≥ 1.50.0, run `cargo build -p sast-verify --release`, iterate `cargo xtask sast-verify gate` over `.semgrep/<lang>/*.yaml`, then `semgrep ci --config .semgrep/`. Each CI provider needs its own translation.
- IDE integration (VS Code Semgrep extension, IntelliJ-Rust Semgrep LSP) — works out of the box with the rule pack at `.semgrep/`; document in the consumer's README, not here.
- `cargo-semgrep` subcommand wrapper — does not exist as of 2026 (per the research dossier). If/when it lands upstream, this doc should mention it as a third local entry point.
