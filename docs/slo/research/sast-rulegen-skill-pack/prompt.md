# Research brief — sast-rulegen-skill-pack

## Wedge (one sentence)

`/slo-rulegen` v1 — a Claude Code skill that produces a Semgrep rule pack for Rust covering the top-10 CWE classes that idiomatic safe-Rust code is most susceptible to, with a corpus-driven `cargo xtask sast-verify` task gating every rule on fire-on-bad / silent-on-good before it lands in the project pack. Long-term value is the per-bug *extend* loop: when Claude Code finds a critical bug during `/slo-execute`, the skill takes `(bug_summary, fix_diff, file_paths)` and outputs 3-5 variation rules + corpus pairs that get appended to the project's existing Semgrep pack.

## Target user (one sentence)

Solo Rust founders ("vibe-coders") who are repeatedly bitten by panic-on-attacker-input bugs and webapp injection issues that Rust's "safe by default" claim does not cover, and who want their Claude Code session to compound every found bug into rules that prevent the class — and its credible variations — from ever landing again, without context-switching out of Claude.

## Research questions

### Q1 — CWE susceptibility ranking for Rust in 2026

What are the top 10 CWE classes that idiomatic safe-Rust + popular ecosystem crates (axum, tokio, serde, sqlx, reqwest, hyper, actix-web) are actually most susceptible to in production, ranked by **frequency-in-the-wild not theoretical risk**?

Authoritative sources to mine:
- RustSec advisory-db at https://github.com/RustSec/advisory-db — categorise the last 24 months of advisories by CWE ID, count occurrences.
- OSV.dev API at https://osv.dev/#use-the-api — query Rust ecosystem advisories with CWE mapping; cross-check against RustSec.
- Trail of Bits public Rust audit reports (e.g. https://github.com/trailofbits/publications), NCC Group public Rust audits, cure53 public reports — extract "what bugs they actually find vs. what the threat model said they'd find."
- cargo-audit and cargo-vet issue trackers for false-negative discussions.

Output: a ranked list of CWE IDs with frequency-in-the-wild counts, plus a one-paragraph "why Rust is susceptible to this class despite the safety claim" per CWE. The user's prior expectation is that panic-DoS classes (CWE-248, CWE-1284, CWE-770), integer-overflow-in-capacity-or-length (CWE-190, CWE-191), and unsafe-FFI-misuse (CWE-119, CWE-787) will be in the top 10 — research should confirm or move that ranking.

### Q2 — Direct competitors in AI-driven SAST rule generation

Name the 2026 commercial and open-source tools that take a found bug (and ideally its fix) and auto-generate SAST rules to prevent regression of the same class. For each: **pricing, target language coverage, whether the rule output is human-editable / inspectable or opaque, whether they verify the generated rule fires on the bad pattern before publishing it**.

Specific tools to investigate by name:
- Semgrep AI / Semgrep Pro AI rule suggestion
- Snyk DeepCode AI / Snyk Code Custom Rules
- GitHub Copilot Autofix / GitHub Advanced Security custom-pattern queries
- SonarQube AI CodeFix
- Veracode AI
- Any open-source Claude / GPT-driven rule synthesisers on GitHub (search "semgrep rule generator", "sast rule llm", "auto rule generation")
- The user's own prior project at https://github.com/kerberosmansour/SAST.GEN as a baseline — what was the precedent in 2024 that this 2026 version improves on?

Gap-vs-our-wedge framing: which of these (a) integrate into a Claude Code session as a skill (not as an external SaaS dashboard), (b) target Rust specifically with idiom-aware variation enumeration, (c) verify the generated rule with a corpus-driven fire/silent gate before write?

### Q3 — Adjacent Rust security tools (NOT rule generation)

What is the existing Rust security tooling landscape, and which tools should `/slo-rulegen` *integrate with* vs *replace* vs *ignore*?

Specific tools to map:
- cargo-audit, cargo-deny, cargo-vet, cargo-geiger
- miri, kani, prusti, creusot, MIRAI
- rust-clippy lints related to security (`clippy::unwrap_used`, `clippy::expect_used`, `clippy::indexing_slicing`, `clippy::arithmetic_side_effects`, `clippy::panic`, `clippy::integer_arithmetic`)
- RustSec's own pipeline (advisory-db → cargo-audit)
- The `cargo xtask` precedent — which large Rust projects ship security-focused xtasks?

Output: for each tool, (a) does it solve a sub-problem of `/slo-rulegen` so we should call it, (b) does it solve an adjacent problem so the user's existing project should already have it before installing our pack, or (c) is it irrelevant to the wedge.

### Q4 — Semgrep Rust frontend reality in 2026 + corpus-layout convention

(a) **What does Semgrep's Rust support actually cover in 2026?** Specifically: does `pattern-either` work the same on Rust as on Python; does taint mode have working source/sink declarations for Rust; does metavariable type filtering work on Rust generics and trait bounds; does `pattern-inside` work for `unsafe { ... }` blocks; what is the false-positive rate on macro-heavy code (axum / tokio handler macros, `tracing::instrument`); is there language-server-protocol integration for IDE-side rule firing?

Sources: the existing `rust/` rules in https://semgrep.dev/r and the semgrep-rules GitHub repo, semgrep/semgrep issue tracker filtered to "language:rust" or `[lang/rust]`, Semgrep release notes 2024-2026, any conference talks (BSidesSF, RustConf) on Semgrep's Rust frontend, return-to-corp blog posts.

(b) **What is the dominant convention for rule-and-snippet co-location?** Per-rule `.test.yaml` next to the rule, single `tests/` dir with naming convention (e.g. `tests/<rule-id>/{bad,good}.rs`), or inline `metadata: examples` in the rule YAML? Examine: semgrep/semgrep-rules itself, Trail of Bits' semgrep-rules, deepsource ruleset, returntocorp pro packs. The dominant convention decides the default schema for `references/sast/` and what `/slo-ruleverify` reads.

### Q5 — CI/dev-env wiring options + publication-risk precedent

(a) **What are the canonical CI and local-dev entry points for Semgrep on a Rust project in 2026?** CI: `returntocorp/semgrep-action`, GitLab CI templates, BuildKite plugins. Local: `pre-commit` framework, IDE plugins (VS Code Semgrep extension, IntelliJ-Rust), `cargo` subcommand wrappers (e.g. `cargo-semgrep` if it exists), on-save file watchers. Specifically: has a Rust-native equivalent to `pre-commit` emerged that handles `cargo`-shaped repos better, or is `pre-commit` (Python-based) still canonical?

(b) **Publication-risk precedent on vulnerable-snippet corpora.** Does shipping a public corpus of "deliberately vulnerable Rust snippets" trigger any practical issue under GitHub Acceptable Use Policy "Active Malware or Exploits" clause? Examine: how does semgrep-rules itself ship snippets (labelled? README disclaimer? gated branch?), has any public rule-pack repo received a takedown notice, what does GitHub's policy text actually require for educational vulnerability content. Output decides whether the snippet output dir defaults to `.gitignore`'d (most conservative), test-only-marked-but-tracked (middle), or fully public (most permissive).

## What the design must produce from this research

- A definitive top-10 CWE list for Rust that drives `references/sast/cwe-map-rust.md` and the M1 milestone scope.
- A clear competitor-gap table that justifies why `/slo-rulegen` exists vs. just buying Semgrep AI.
- An integration-vs-replace decision per adjacent Rust tool (especially: should `/slo-rulegen` consume Clippy output, or duplicate the work).
- A verified-feasible Semgrep-Rust feature set (taint mode? `pattern-inside`? macro handling?) so the rule templates in `references/sast/` only use working primitives.
- A defensible default for the corpus output dir (`.gitignore`'d vs tracked) backed by precedent, not opinion.
