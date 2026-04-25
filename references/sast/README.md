# `references/sast/` — shared scaffolding for the SAST rule-gen skill pack

Sibling of `skills/`, NOT walked by `crates/sldo-install/src/install.rs::discover_skills()`. Same installer-bypass pattern as `references/biz/`.

Consumed by `/slo-rulegen` (bootstrap + extend modes), `/slo-ruleverify`, and the `cargo xtask sast-verify` binary at `xtasks/sast-verify/`.

## Directory layout

| File | Purpose |
|---|---|
| `README.md` | This file. |
| `AUTHORING.md` | Trail of Bits AGPL clean-room re-authoring policy + rule-style guide. |
| `cwe-map-rust.md` | Top-10 Rust CWE ranking with provenance from RustSec / GHSA / OSV. |
| `semgrep-rust-syntax.md` | Which Semgrep primitives work for Rust in 2026; M1 smoke-test results. |
| `manifest-schema.md` | Rule YAML metadata block schema (consumed by `check-coverage`). |
| `MIN-SEMGREP-VERSION.md` | Minimum required `semgrep --version` value. |
| `variations/cwe-<NNN>.md` | Per-CWE variation template; declares `minimum_pattern_either_arms` in frontmatter. |
| `prompts/bootstrap.md` | Prompt body `/slo-rulegen` reads in bootstrap mode. |
| `prompts/extend.md` | Prompt body `/slo-rulegen --extend` reads (M2 fills the body). |

## Stability

- Path layout is **stable** — skills and the xtask hard-code these paths.
- Content within files is **evolving** — adding a CWE entry, refining a variation, updating the Semgrep cheat-sheet are all routine edits.
- Frontmatter keys (`minimum_pattern_either_arms`, `cwe`, `sink_shapes`) are **stable** — schema changes require a fresh `/slo-architect` pass.

## Residual risk acknowledgment

Per [SECURITY.md](../../SECURITY.md) "Residual risk — `references/<pack>/` files are NOT SHA-pinned by `sldo-install`": this directory is **not** covered by `sldo-install`'s install-time SHA verification. PR review is the only barrier to malicious modification. A Phase-3 hardening runbook is open to extend the manifest walker.
