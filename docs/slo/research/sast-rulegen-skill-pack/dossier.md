---
name: sast-rulegen-skill-pack
researched: 2026-04-25
incomplete: false
---

# Research Dossier — SAST rule-generation skill pack

## Market

The 2026 AI-SAST market is dominated by **AI-triage** (Semgrep Assistant claims 96% triage alignment) and **AI-autofix** (Copilot Autofix, SonarQube AI CodeFix, Veracode Fix produce fix patches). **Per-bug rule synthesis with corpus-driven verification, callable inside a developer's editor session, is unfilled.** Closest precedent (Semgrep Assistant) takes user-supplied corpus pairs in its SaaS UI; nothing surveyed takes a Claude-found bug + fix-diff and emits Rust-idiomatic Semgrep rules with auto-derived corpus and a fire/silent gate, in-session.

Proxy spend the wedge displaces: Semgrep Code seat ($30/contributor/month), Snyk Code Custom Rules ($50/dev/month, Enterprise-only Early Access). For a Rust solo founder, the wedge replaces "buy a SaaS seat to write the rule for me" with "extend my pack now from inside Claude Code."

## Direct competitors

| Name | Price | Key feature | Gap vs our wedge |
|---|---|---|---|
| Semgrep Assistant | $30/contributor/mo (Semgrep Code) | Internal fire-on-bad/silent-on-good rule-gen gate; `(description, 1 bad, 1 good) → 1 rule` | No auto-derivation from a found bug + fix-diff; SaaS UI only, not callable as a Claude Code skill or via MCP; 2026 product focus is finding triage, not rule synthesis |
| Snyk Code (DeepCode AI) Custom Rules | ~$50/dev/mo Snyk Advanced; Custom Rules Enterprise-only Early Access | Symbolic-AI engine; proprietary Datalog DSL against an internal "event graph" | Rules are opaque (not human-editable in the Semgrep YAML sense); no Claude Code integration; closed engine; not corpus-verified externally |
| GitHub Copilot Autofix + Code Scanning | Bundled with GHAS | AI-generated fix *patches* on existing Code Scanning findings | Generates fixes, not new queries; CodeQL ≠ Semgrep; no "bug → query" auto-generator in GHAS as of 2026 |
| SonarQube AI CodeFix | SonarQube Server commercial tiers | Fix-suggestion AI inline in PRs | Generates fixes, not rules; no rule-synthesis surface |
| Veracode Fix | Veracode platform | AI-driven fix recommendations for Veracode findings | Fix-suggestion AI, not rule-generation |

`kerberosmansour/SAST.GEN` (2024 prior art, the user's own) was flagged as a baseline; specific implementation details were not extracted in this research run and remain a residual open question.

## Adjacent tools

| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|
| `rustsec/advisory-db` | Curates Rust advisories; each entry links to fix PR (= ground-truth bug+fix) | No — they curate, do not synthesize rules. **Integrate as primary corpus source.** |
| `cargo-audit` 0.22.1 (Apache/MIT, 2026-02) | Emits findings against `Cargo.lock` in JSON + SARIF | No — Cargo.lock-scoped, not source-AST. **Integrate as CI trigger for the per-bug extend loop.** |
| `cargo-deny` 0.19.4 (Apache, 2026-04) | `--format=json/sarif` for advisories channel | Same as cargo-audit — integrate. |
| `cargo-vet` 0.10.0 | Trust attestation, no fix-diff stream | Ignore. |
| `cargo-geiger` 0.13.0 | Counts `unsafe` *presence*, not bug patterns | Ignore for findings; useful as a prioritisation signal only. |
| Clippy security restriction lints (`unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`, `panic`) | Stable; restriction group; default `Allow`; no CWE tags; fires on every occurrence with no taint reasoning | **Compose, do not replace.** Layer Semgrep taint/context-aware rules on top. |
| `miri` | Detects UB at runtime via interpreter | Backlog — output is JSON-parseable but requires triggering test + diagnostic-JSON parsing. |
| `kani` 0.67.0 (2026-01) | CBMC-based bounded model checker | Ignore for v1 — heavyweight; harness authoring required. |
| `prusti` / `creusot` / MIRAI / `rudra` | Verification (prusti/creusot LGPL caution); MIRAI archived 2024-08-22 | Ignore. |

## Technical prior art

- **Trail of Bits `semgrep-rules/rs/panic-in-function-returning-result.yaml` (AGPL-3.0)** — the only public production-grade Rust panic-DoS rule. Demonstrates variation-enumeration shape: one rule, four `pattern-either` arms (`Result<T1,T2>`, `Result<T>`, type-alias variants), three `pattern-not-inside #[cfg(test)] mod tests` exclusions. Tags as **CWE-755 (Improper Handling of Exceptional Conditions)**, not CWE-248 — establishes the canonical CWE for panic-DoS. https://github.com/trailofbits/semgrep-rules/tree/main/rs
- **`semgrep/semgrep-rules` Rust pack** (Semgrep Rules License) — 10 production rules covering crypto/TLS/process-arg classes (`args-os`, `args`, `current-exe`, `insecure-hashes`, `reqwest-accept-invalid`, `reqwest-set-sensitive`, `rustls-dangerous`, `ssl-verify-none`, `temp-dir`, `unsafe-usage`). Each is a `.yml` + `.rs` pair co-located in the same directory — defines the corpus layout convention. **Zero CWE-class overlap with the wedge** — content is genuinely net-new. https://github.com/semgrep/semgrep-rules
- **`0xdea/semgrep-rules`** — strongest documented `// ruleid:` convention: "Each rule is accompanied by an actual vulnerable source code that was targeted by an exploit, with vulnerable lines marked with `// ruleid: ...`." https://github.com/0xdea/semgrep-rules
- **`matklad/cargo-xtask`** — canonical xtask layout (sibling `xtask/` workspace member, `[alias] xtask = "run --package xtask --"` in `.cargo/config.toml`). rust-analyzer, Cargo itself, Tokio, ripgrep, clap, wasmtime, bevy, OpenVMM all use the pattern. Single-binary-with-subcommands is the dominant convention. https://github.com/matklad/cargo-xtask
- **OpenVMM `cargo xtask` precedent** — closest documented "security-themed xtask" example (`fuzz` subcommand). https://openvmm.dev/guide/dev_guide/dev_tools/xtask.html
- **`prek` 0.3.10 (MIT, 2026-04-21)** — Rust-native drop-in `pre-commit` replacement; reads `.pre-commit-config.yaml` unchanged; production users include CPython and Apache Airflow. https://github.com/j178/prek
- **RustXec dataset (Virginia Tech, 2026)** — analyses 515 RustSec advisories Jan 2021 – Apr 2025; reports memory-corruption (31 cases) and DoS (28 cases) as leading classes; top specific CWEs are CWE-787 (9), CWE-415 (8), CWE-400 (7). https://people.cs.vt.edu/xinw/publications/RustXec26-B38KjKAe.pdf

## Regulatory / legal

- **GitHub Acceptable Use Policy explicitly carves out educational vulnerability content.** Quote: *"GitHub allows dual-use content and supports the posting of content that is used for research into vulnerabilities, malware, or exploits, as the publication and distribution of such content has educational value."* No takedown notices surfaced against semgrep-rules-style repos for deliberately-vulnerable test snippets. **The idea-doc compliance-fine framing was over-cautious for the rule-pack repo itself; correct default is tracked-and-labelled.** https://docs.github.com/en/site-policy/acceptable-use-policies/github-active-malware-or-exploits
- **Trail of Bits `semgrep-rules` is AGPL-3.0** — copy-pasting YAML wholesale would inherit AGPL. Re-author rules independently, treating ToB precedent as structural inspiration only. Pattern shapes are likely uncopyrightable (functional content) but a clean re-implementation removes ambiguity.
- **Creusot is LGPL-2.1** — vendoring caution. (Decision: ignore creusot for v1 anyway.)
- **US EAR 5D002 ("intrusion software") enforcement on snippet collections is essentially zero** — Metasploit, Nuclei, Semgrep itself are publicly distributed.
- **Two-tier corpus convention** is defensible: snippets in the **rule-pack repo** track-and-label (upstream convention); snippets in a **user's application repo** default to `.gitignore`'d (the user's repo is not a security-tooling repo, tracked vuln snippets there would be a compliance finding even if portable).
- **No `LICENSE` file present in the SunLitOrchestrate repo root** — license obligations cannot be confirmed from the source tree. A LICENSE file should be added before any rule-pack is released externally.

## Open questions that research did not answer

1. **`pattern-inside: unsafe { ... }` for Rust** — no public example in `semgrep/semgrep-rules`; not documented as restricted but not exercised. A 5-line smoke rule + fixture is the only way to confirm before templating any unsafe-FFI rule (CWE-119/787/416) into `references/sast/`. Smoke-test required as part of M1.
2. **Variation-template content for CWE-416 / CWE-190 / CWE-787 / CWE-125 / CWE-697** is not yet sourced. Trail of Bits gives the CWE-755 shape only. Concrete fix-diff sources to mine in M1: `cassandra-rs` UAF (RUSTSEC-2024-0017), `mio` named-pipe-token UAF (RUSTSEC-2024-0019), `wasmtime::Linker::clone` UAF (RUSTSEC-2026-0090), `hpke-rs` underflow (RUSTSEC-2026-0070), `ruzstd` OOB-read (RUSTSEC-2024-0400), `idna` Punycode comparison (RUSTSEC-2024-0421).
3. **Full RustSec→GHSA→OSV CWE-join coverage % over a 24-month window is unmeasured.** Sample (n=8) shows 100% GHSA-side CWE coverage; bottleneck is the GHSA-alias-presence rate among RustSec advisories. Needs a one-shot script over `rustsec/advisory-db` 2024-04 → 2026-04 to report join hit-rate and the size of the residual category-only fallback.
4. **CWE-89 / CWE-79 / CWE-918 in a Rust top-10 lacks frequency support.** No major framework-level advisories surfaced for axum/sqlx SQLi/XSS/SSRF in 2024–2025. Inclusion is justifiable on threat-class grounds (axum apps absolutely can be vulnerable when devs hand-roll SQL or HTML), not frequency. **Hybrid ranking decision falls to /slo-architect.**
5. **Clippy ⇄ CWE-755/CWE-190 sink-shape overlap %** is unmeasured. The "compose-not-replace" decision stands either way, but the exact division of labour between Clippy and Semgrep is not pinned.
6. **`metavariable-type` on Rust generics + trait bounds — actual behaviour is unverified.** Open issues #10380 and #11150 suggest partial support. Recommendation is to fall back to `metavariable-pattern` + regex; smoke-test required if used.
7. **Semgrep macro/false-positive rate on heavy macro code (axum handlers, `tracing::instrument`, tokio `select!`) is not quantified.** Open issues #10471, #10362, #3600, #5221 confirm the gap qualitatively but no FP rate is published.
8. **`semgrep --test` exit-code semantics in latest 2026 CLI.** Issue #10319 (returns 0 on invalid rule) was open at deepening time; confirm fix status before relying on `--validate` + `--test` as the v1 gate.
9. **`kerberosmansour/SAST.GEN` 2024 prior art** — specific implementation, gating logic, and what 2026 should improve on were not extracted.
10. **OSS LLM rule synthesisers on GitHub** — searches did not surface any project that integrates as a Claude Code skill with corpus-gating; absence is not the same as "does not exist."
