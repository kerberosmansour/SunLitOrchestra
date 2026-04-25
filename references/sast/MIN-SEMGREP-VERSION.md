---
min_semgrep_version: "1.50.0"
---

# Minimum required Semgrep CLI version

The `cargo xtask sast-verify` binary asserts `semgrep --version` ≥ this value before any subcommand runs.

`1.50.0` is chosen because:

- Rust frontend GA was declared by the 1.49.x release line per Semgrep's own release notes.
- `--validate` exit codes 5 / 7 / 4 stable since 1.40+.
- `--test` exit code 1 on assertion failure (PR #6070) merged before 1.50.
- `--json` structured output stable across all 1.x.

If your local `semgrep --version` is older, upgrade via `brew install semgrep` (macOS) or `pip install --upgrade semgrep`. The xtask refuses to run on older versions because the `--validate` exit-code semantics or `--json` schema may differ.

`semgrep --version` output format expected: `<major>.<minor>.<patch>` on a single line (possibly preceded by an upgrade-available banner). The version-parser tolerates the banner.
