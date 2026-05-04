# Contributing to SunLit Orchestra

Thanks for your interest. SunLit Orchestra is a workflow tool that values explicit contracts over heroic prompting; the same discipline applies to contributions.

## Quick links

- [README](README.md) — project overview and install path
- [docs/getting-started.md](docs/getting-started.md) — first-run guide
- [SECURITY.md](SECURITY.md) — vulnerability disclosure + security defaults
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — Contributor Covenant 2.1

## Recommended workflow

1. **Fork + clone.** `git clone <your-fork>` and `cd SunLitOrchestra`.
2. **Open an issue first** for non-trivial work. The runbook discipline only pays off when scope is agreed before code is written. Tiny fixes (typos, broken links, one-line bug fixes) can skip this step.
3. **Use the skills on yourself.** For larger work, run `/slo-ideate` → `/slo-research` → `/slo-architect` → `/slo-plan` to produce a runbook the maintainers can review *before* any code lands. This is the lowest-friction path to merging — reviewers can sign off on the plan and trust the execution.
4. **Pass the baseline.** `cargo test -p sldo-common -p sldo-install -p sldo-research` must be green.
5. **Open a PR** following the [PR template](.github/PULL_REQUEST_TEMPLATE.md). Link to the runbook + closed-milestone summary if there is one.

## What's currently most welcome

- **Real-world false-positive shakedown of the SAST rule pack** against your own Rust codebases, with PRs tightening any over-broad rules (`pattern-not-inside` carve-outs).
- **Cross-platform ports** of the installer (macOS-only today; Linux and Windows readiness is open work).
- **Fixture additions** to `references/biz/judgment-fixtures/` covering new marginal cases for the advisor skills.
- **New variation templates** at `references/sast/variations/cwe-<NNN>.md` extending the rule pack to additional CWE classes.
- **Documentation polish** — typos, broken links, clearer examples, screenshots, asciinema demos.

## What is out of scope

- The parked Tauri desktop UI (`crates/sldo-tauri/`) — will resume only when there's a concrete user pulling for it.
- Non-UK jurisdiction support in the biz pack — v1 is UK-only by design; non-UK is a fresh `/slo-architect` pass, not a patch.

## Project conventions

### Runbook templates live in two places

`runbook-template_v_4_template.md` (and v3) lives in **two** places by design:

- `skills/slo-plan/references/runbook-template_v_4_template.md` — the skill-local copy that travels with the skill via `sldo-install`'s symlink. This is what `/slo-plan` reads at runtime, in any project.
- `docs/slo/templates/runbook-template_v_4_template.md` — the human-browsable mirror visible in this repo on GitHub.

These two files **must be byte-identical**. A CI test (`crates/sldo-install/tests/e2e_v4_template.rs::v4_skill_local_copy_matches_docs_mirror`) fails loudly if they drift. If you intentionally edit one, copy it over the other and re-run.

### Commit messages

Follow the existing style — see `git log` on `main`. Conventional-style prefixes (`feat:`, `fix:`, `docs:`, `refactor:`) are common but not strictly required.

### One-PR rule

Prefer one PR per runbook milestone. The runbook discipline already chunks work into reviewable units; resist the temptation to bundle.

### No agentic shortcuts

The pack's value is in explicit guardrails. Do not skip:

- Threat-model integration (SECURITY.md is the source of truth)
- BDD scenarios in runbooks
- Adversarial critique passes (`/slo-critique`) before execution begins
- Verification (`/slo-verify`) before retro

## Getting help

- **Bugs / feature requests**: file a [GitHub issue](https://github.com/kerberosmansour/SunLitOrchestra/issues/new/choose).
- **Security vulnerabilities**: do **not** open a public issue — see [SECURITY.md](SECURITY.md) for the private disclosure path.
- **Questions about the workflow**: open a discussion or an issue with the `question` label.

## License

By contributing, you agree your contribution is dual-licensed under Apache-2.0 OR MIT, matching the project (see [LICENSE](LICENSE)).

## Sign-off — Developer Certificate of Origin

Every commit must carry a `Signed-off-by:` trailer asserting the [Developer Certificate of Origin 1.1](https://developercertificate.org/). This is a lightweight attestation that you have the right to contribute the work under the project's licence; it is not a Contributor Licence Agreement and does not transfer copyright.

Add the trailer automatically with `git commit -s` (or `git commit --signoff`). The trailer looks like:

```
Signed-off-by: Your Name <your.email@example.com>
```

The name must be your real name (no anonymous / pseudonymous contributions) and the email must match the email on the commit. If you have multiple commits in a PR, each commit needs its own sign-off — `git rebase --signoff main` adds the trailer to existing commits in your branch.

Why DCO and not a CLA: the DCO keeps the contribution barrier low (no agreement to e-sign, no separate database to maintain) while still establishing a clear chain of provenance. It does not grant the project relicensing rights — your contributions remain Apache-2.0 OR MIT in perpetuity, matching the project licence.

## New source files — copyright and SPDX header

For **new** source files you create, add this header at the top:

```rust
// Copyright 2026 Sherif Mansour and SunLit Orchestra contributors.
// SPDX-License-Identifier: Apache-2.0 OR MIT
```

(Adjust comment syntax for the language.) Existing files do not need to be retroactively headered — the project-level [NOTICE](NOTICE) and [LICENSE](LICENSE) cover the repo as a whole. The per-file header is belt-and-braces for new code, not a requirement to bulk-edit existing code.

## Trade-marks

The Apache-2.0 / MIT dual licence grants no rights in the project name or logo. See [TRADEMARKS.md](TRADEMARKS.md) for what permission you do and do not need before using the name in a fork, derivative, or downstream product.
