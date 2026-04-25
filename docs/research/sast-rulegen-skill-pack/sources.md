---
name: sast-rulegen-skill-pack
accessed: 2026-04-25
---

# Sources

URLs were accessed by `sldo-research` on 2026-04-25 during a 3-iteration / 5-search deepening pass. Citation maps to the dossier and synthesis.

## CWE susceptibility data (RustSec / OSV / CVE)

- RustSec Advisory Database (categories): https://rustsec.org/categories/
- RustSec Advisory Database (advisory list): https://rustsec.org/advisories/
- RustSec Advisory Database (about): https://rustsec.org/
- RustSec Reporting Vulnerabilities: https://rustsec.org/contributing.html
- rustsec/advisory-db GitHub repo: https://github.com/rustsec/advisory-db
- rustsec/advisory-db README (raw): https://raw.githubusercontent.com/rustsec/advisory-db/main/README.md
- RUSTSEC-2024-0421 advisory: https://rustsec.org/advisories/RUSTSEC-2024-0421.html
- RUSTSEC-2026-0070 advisory: https://rustsec.org/advisories/RUSTSEC-2026-0070.html
- OSV — Data sources: https://google.github.io/osv.dev/data/
- google/osv.dev issue #3245 (CWE/CVSS/EPSS for advisories): https://github.com/google/osv.dev/issues/3245
- OSV vuln record — RUSTSEC-2025-0046: https://osv.dev/vulnerability/RUSTSEC-2025-0046
- OSV vuln record — RUSTSEC-2024-0421: https://osv.dev/vulnerability/RUSTSEC-2024-0421
- OSV API — RUSTSEC-2024-0421: https://api.osv.dev/v1/vulns/RUSTSEC-2024-0421
- OSV API — GHSA-h97m-ww89-6jmq (idna): https://api.osv.dev/v1/vulns/GHSA-h97m-ww89-6jmq
- OSV API — GHSA-x9xc-63hg-vcfq: https://api.osv.dev/v1/vulns/GHSA-x9xc-63hg-vcfq
- Wiz — RUSTSEC-2025-0112: https://www.wiz.io/vulnerability-database/cve/rustsec-2025-0112
- Wiz — RUSTSEC-2024-0423: https://www.wiz.io/vulnerability-database/cve/rustsec-2024-0423
- Wiz — RUSTSEC-2025-0028: https://www.wiz.io/vulnerability-database/cve/rustsec-2025-0028
- cvedetails — Rust-lang Rust: https://www.cvedetails.com/vulnerability-list/vendor_id-19029/product_id-48677/Rust-lang-Rust.html
- MITRE CVE search keyword=rust: https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword=rust
- RustXec dataset (Virginia Tech 2026): https://people.cs.vt.edu/xinw/publications/RustXec26-B38KjKAe.pdf
- Penligent — CVE-2025-68260 (first Rust kernel CVE): https://www.penligent.ai/hackinglabs/rusts-first-breach-cve-2025-68260-marks-the-first-rust-vulnerability-in-the-linux-kernel/
- CVE-2024-24576 — std::process::Command Windows BatBadBut: https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html
- CVE-2024-43402 — std::process::Command Windows hardening: https://blog.rust-lang.org/2024/09/04/cve-2024-43402.html
- CERT-EU 2024-035 advisory (Rust on Windows): https://cert.europa.eu/publications/security-advisories/2024-035/

## CWE Top 25 / Top 10 KEV (cross-language baseline)

- 2025 CWE Top 25 (MITRE): https://cwe.mitre.org/top25/archive/2025/2025_cwe_top25.html
- 2025 CWE Top 25 (CISA alert): https://www.cisa.gov/news-events/alerts/2025/12/11/2025-cwe-top-25-most-dangerous-software-weaknesses
- 2025 CWE Top 10 KEV Insights: https://cwe.mitre.org/top25/archive/2025/2025_kev_insights.html
- CWE-1435 (2025 Top 25 view): https://cwe.mitre.org/data/definitions/1435.html
- CWE root: https://cwe.mitre.org/

## Semgrep Rust frontend + rule syntax

- Semgrep — Supported languages: https://semgrep.dev/docs/supported-languages
- Semgrep blog 2026 — Rust SCA reachability: https://semgrep.dev/blog/2026/semgrep-supply-chain-extends-reachability-coverage-to-rust/
- Semgrep — Taint analysis overview: https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/overview
- Semgrep — Taint analysis (root): https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/
- Semgrep — Advanced taint techniques: https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/advanced
- Semgrep blog 2022 — Demystifying Taint Mode: https://semgrep.dev/blog/2022/demystifying-taint-mode/
- Semgrep blog 2021 — Taint mode beta: https://semgrep.dev/blog/2021/taint-mode-is-now-in-beta/
- Semgrep release notes Feb 2023 — Rust beta: https://semgrep.dev/docs/release-notes/february-2023
- Semgrep release notes Jul 2023 — Rust macro taint propagation: https://semgrep.dev/docs/release-notes/july-2023
- Semgrep v1.49.0 release: https://github.com/semgrep/semgrep/releases/tag/v1.49.0
- Kudelski Security — Advancing Rust Support in Semgrep (2021): https://kudelskisecurity.com/research/advancing-rust-support-in-semgrep
- Semgrep — Test rules: https://semgrep.dev/docs/writing-rules/testing-rules
- Semgrep — Rule structure syntax: https://semgrep.dev/docs/writing-rules/rule-syntax
- Semgrep — Run rules: https://semgrep.dev/docs/running-rules
- Semgrep — Rules overview: https://semgrep.dev/docs/writing-rules/overview
- Semgrep — Pattern syntax (experimental): https://semgrep.dev/docs/writing-rules/experiments/pattern-syntax
- Semgrep — metavariable-type docs: https://semgrep.dev/docs/writing-rules/experiments/metavariable-type
- Semgrep — VS Code rule schema: https://semgrep.dev/docs/kb/rules/using-semgrep-rule-schema-in-vscode
- Semgrep — Contribute rules: https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository
- Semgrep blog — Writing rules methodology: https://semgrep.dev/blog/2020/writing-semgrep-rules-a-methodology/

## Public rule packs (corpus convention + prior art)

- semgrep/semgrep-rules: https://github.com/semgrep/semgrep-rules
- semgrep/semgrep-rules — develop tree (recursive API): https://api.github.com/repos/semgrep/semgrep-rules/git/trees/develop?recursive=1
- semgrep/semgrep-rules — rust/lang/security/unsafe-usage.yml: https://raw.githubusercontent.com/semgrep/semgrep-rules/develop/rust/lang/security/unsafe-usage.yml
- semgrep/semgrep-rules — rust/lang/security/rustls-dangerous.yml: https://raw.githubusercontent.com/semgrep/semgrep-rules/develop/rust/lang/security/rustls-dangerous.yml
- trailofbits/semgrep-rules: https://github.com/trailofbits/semgrep-rules
- trailofbits/semgrep-rules — /rs directory: https://github.com/trailofbits/semgrep-rules/tree/main/rs
- trailofbits/semgrep-rules — panic-in-function-returning-result.yaml (raw): https://raw.githubusercontent.com/trailofbits/semgrep-rules/main/rs/panic-in-function-returning-result.yaml
- 0xdea/semgrep-rules: https://github.com/0xdea/semgrep-rules
- semgrep/semgrep issue #2799 — `semgrep --test` a yaml file: https://github.com/returntocorp/semgrep/issues/2799
- semgrep/semgrep-rules issue #1228 — Invalid schema on yaml/* unit test files: https://github.com/semgrep/semgrep-rules/issues/1228

## Competitor product pages

- Semgrep Assistant overview: https://semgrep.dev/docs/semgrep-assistant/overview
- Semgrep Assistant product page: https://semgrep.dev/products/semgrep-code/assistant/
- Semgrep blog — tech behind Assistant (2024): https://semgrep.dev/blog/2024/the-tech-behind-semgrep-assistant/
- Semgrep blog — Using AI to write secure code (2023): https://semgrep.dev/blog/2023/using-ai-to-write-secure-code-with-semgrep/
- Semgrep blog — Three months of Assistant (2023): https://semgrep.dev/blog/2023/assistant-public-beta/
- Semgrep blog — Assistant GA launch (2024): https://semgrep.dev/blog/2024/assistant-ga-launch/
- Snyk vs Semgrep (Konvu): https://konvu.com/compare/snyk-vs-semgrep
- Semgrep vs Snyk Code (AppSecSanta): https://appsecsanta.com/sast-tools/semgrep-vs-snyk-code
- Snyk Code Review 2026 (AppSecSanta): https://appsecsanta.com/snyk-code
- Best AI Code Security Tools 2025 (sanj.dev): https://sanj.dev/post/ai-code-security-tools-comparison
- Snyk vs Semgrep SCA vs Custom SAST Rules 2026 (DEV): https://dev.to/rahulxsingh/snyk-vs-semgrep-sca-platform-vs-custom-sast-rules-in-2026-3047
- 7 Best Snyk Alternatives 2026 (DeepSource): https://deepsource.com/resources/snyk-alternatives
- Best SAST Tools for AI-Generated Code (Vibe-Eval): https://vibe-eval.com/testing/sast-tools-ai-code
- Snyk vs Semgrep vs Corgea: https://corgea.com/blog/compare/snyk-vs-semgrep/
- Snyk vs Semgrep (Aikido): https://www.aikido.dev/blog/snyk-vs-semgrep
- Semgrep Pricing in 2026 (DEV): https://dev.to/rahulxsingh/semgrep-pricing-in-2026-open-source-vs-team-vs-enterprise-costs-3dic

## Clippy security restriction lints

- Clippy Lints index: https://rust-lang.github.io/rust-clippy/master/index.html
- Clippy Lint Configuration: https://doc.rust-lang.org/clippy/lint_configuration.html
- rust-clippy — clippy_lints/src/indexing_slicing.rs: https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/indexing_slicing.rs
- rust-clippy issue #6636 — forbid all expect/unwrap: https://github.com/rust-lang/rust-clippy/issues/6636
- rust-clippy CHANGELOG: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md
- "Unleashing the Power of Clippy" (arxiv): https://arxiv.org/pdf/2310.11738

## Pre-commit / dev-loop runners

- semgrep/pre-commit hook repo: https://github.com/semgrep/pre-commit
- Customize Semgrep in pre-commit (KB): https://semgrep.dev/docs/kb/integrations/customize-semgrep-precommit
- j178/prek (GitHub): https://github.com/j178/prek
- prek documentation: https://prek.j178.dev/
- HN — Better pre-commit re-engineered in Rust: https://news.ycombinator.com/item?id=45931273
- HN — Prek drop-in replacement: https://news.ycombinator.com/item?id=46873138
- Comparing Code Quality Meta Tools (House Absolute): https://blog.urth.org/2020/05/08/comparing-code-quality-meta-tools/
- Git hooks management with pre-commit and lefthook: https://0xdc.me/blog/git-hooks-management-with-pre-commit-and-lefthook/
- dotdc/test-lefthook-and-pre-commit: https://github.com/dotdc/test-lefthook-and-pre-commit

## cargo xtask precedent

- matklad/cargo-xtask: https://github.com/matklad/cargo-xtask
- matklad/cargo-xtask README: https://github.com/matklad/cargo-xtask/blob/master/README.md
- matklad/cargo-xtask issue #8 — workspaces recommended: https://github.com/matklad/cargo-xtask/issues/8
- matklad — Large Rust Workspaces (2021): https://matklad.github.io/2021/08/22/large-rust-workspaces.html
- rust-analyzer — xtask docs: https://rust-lang.github.io/rust-analyzer/xtask/index.html
- OpenVMM Guide — cargo xtask: https://openvmm.dev/guide/dev_guide/dev_tools/xtask.html
- nickgerace/cargo-xtask-example: https://github.com/nickgerace/cargo-xtask-example
- jondot/xtaskops: https://github.com/jondot/xtaskops
- sebastienrousseau/xtasks: https://github.com/sebastienrousseau/xtasks
- tracel-xtask on crates.io: https://crates.io/crates/tracel-xtask
- bryantluk.com — cargo-xtask blog: https://blog.bryantluk.com/cargo-xtask/
- lib.rs — cargo-xtask: https://lib.rs/crates/cargo-xtask
- docs.rs — xtasks: https://docs.rs/xtasks

## Regulatory / legal

- GitHub AUP — Active Malware or Exploits: https://docs.github.com/en/site-policy/acceptable-use-policies/github-active-malware-or-exploits

## Rust security tooling index

- rust-secure-code/projects: https://github.com/rust-secure-code/projects
