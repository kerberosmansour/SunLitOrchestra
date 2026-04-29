# SunLitOrchestrate

> An AI-driven software-development workflow that adds the missing guardrails: idea → research → architecture → plan → critique → execute → verify → ship → reflect. Each step is a slash-command skill for Claude Code, backed by file-based contracts (runbooks, threat models, lessons) so the work survives across sessions and reviewers.

**License:** [Apache-2.0 OR MIT](LICENSE) (dual; pick either) — explicitly NOT AGPL.
**Status:** active development. The skill pack and Rust CLIs are stable; the Tauri desktop UI is parked.

## What problem this solves

Sit a senior engineer next to an LLM and the LLM will happily write 5,000 lines of beautiful code that solves the wrong problem, skips the important risks, and leaves no record of *why* anything happened. SunLitOrchestrate fixes that with three things:

1. **A v3 milestone-runbook contract.** Every feature lives in a `docs/RUNBOOK-<feature>.md` with explicit Contract Blocks (allowed files, forbidden shortcuts, BDD scenarios, abuse cases, regression tests). The LLM can't "silently widen scope" because the scope is checked against this file at every step.
2. **A sequence of focused skills**, each doing one thing: `/slo-ideate` interrogates the idea, `/slo-research` produces a sourced dossier, `/slo-architect` commits to a stack + emits a threat model, `/slo-plan` writes the runbook one milestone at a time, `/slo-critique` rotates four adversarial reviewers (CEO, eng-lead, security, designer), `/slo-execute M<N>` drives one milestone with allow-list enforcement, `/slo-verify M<N>` runs the runtime QA, `/slo-retro M<N>` writes lessons + completion summaries.
3. **A SAST rule pack** (`/slo-rulegen`) generating Semgrep rules for the top-10 CWE classes idiomatic Rust + popular crates are most susceptible to. Variation-template-driven, gated by `cargo xtask sast-verify`, never copies AGPL upstream YAML.

The skills can now be installed into [Claude Code](https://claude.com/claude-code) or GitHub Copilot. `sldo-install` still defaults to Claude Code for backward compatibility, and this milestone only changes installer support — broader host-specific docs and runtime capability notes are tracked separately.

## Highlights

- **Sprint-flow skill pack** — 11 first-party `/slo-*` skills covering ideate → research → architect → tla → plan → critique → execute → verify → retro → ship, plus power tools (`/slo-second-opinion`, `/slo-freeze`, `/slo-resume`).
- **UK biz-pack** — 4 advisor skills (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`) and 11 generator skills (`/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`, `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`, `/slo-cofounder`, `/slo-hire`, `/slo-founder-check`) for the company-around-the-product side. Hard-block gates for regulated domains, deals over £5,000, counterparty-with-lawyer, and GDPR documents. JPP Law / SeedLegals public pricing as the cost baseline.
- **SAST rule generator** — `/slo-rulegen` produces a 10/10 CWE-class Semgrep rule pack from variation templates; `/slo-ruleverify` re-gates the pack on demand. Trail-of-Bits-AGPL-clean-room policy is enforced by code review.
- **Threat-model-by-default** — `/slo-architect` emits a `docs/design/<slug>-threat-model.md` (STRIDE × component, abuse cases, compliance mapping) for every project. `/slo-plan` cites threat-model rows as BDD abuse-case scenarios.
- **No agentic shortcuts** — every skill *refuses* to run when its inputs aren't present (no idea doc → refuse `/slo-research`; no research dossier → refuse `/slo-architect`; non-`done` tracker rows → refuse `/slo-ship`). Slow is smooth, smooth is fast.

## Quick start

### Prerequisites

- **Rust toolchain** (stable). `rustup install stable` if you don't have one.
- **A supported host agent**: Claude Code or GitHub Copilot. Claude Code remains the default target if you do not pass `--host`.
- **Semgrep** (`brew install semgrep` or `pip install semgrep`). Required for the SAST rule pack only.

### Install the skill pack

```bash
git clone https://github.com/kerberosmansour/SunLitOrchestrate.git
cd SunLitOrchestrate

# Build the installer
cargo build -p sldo-install --release

# Claude Code is still the default host
./target/release/sldo-install

# Install into GitHub Copilot instead
./target/release/sldo-install --host github-copilot

# Project-local installs write into ./.claude/skills/ or ./.copilot/skills/
./target/release/sldo-install --local
./target/release/sldo-install --host github-copilot --local
```

After install, every `/slo-*` skill is available from the host you selected. If you omit `--host`, `sldo-install` uses `claude-code`.

### Drive a feature end-to-end

```text
/slo-ideate          # YC-style product interrogation
/slo-research        # Sourced dossier (wraps sldo-research)
/slo-architect       # Stack + ARCHITECTURE.md + threat model
/slo-plan            # v3 runbook, one milestone at a time
/slo-critique        # 4-persona adversarial review
/slo-execute M1      # Drive M1 with allow-list enforcement
/slo-verify M1       # Runtime QA + Playwright if UI
/slo-retro M1        # Lessons + completion + tracker update
# ... repeat /slo-execute / /slo-verify / /slo-retro per milestone ...
/slo-ship            # Open PR with runbook-aware description
```

### Generate a SAST rule pack

```bash
# In a Rust workspace where you want the rules:
/slo-rulegen                                    # generates 10 rule pairs at .semgrep/rust/
/slo-ruleverify                                 # confirms every rule passes gate

# Run a specific rule's gate locally:
cargo xtask sast-verify gate .semgrep/rust/cwe-755-panic-on-result-fn.yaml

# Extend the pack from a real bug + fix:
/slo-rulegen --extend \
  --bug-summary /tmp/bug.md \
  --fix-diff /tmp/fix.patch \
  --file-paths src/api/users.rs,src/api/auth.rs
```

CI wiring is documented in [`references/sast/CI-WIRING.md`](references/sast/CI-WIRING.md).

## Skill reference

### Sprint flow

| Stage | Skill | Purpose |
|---|---|---|
| Ideate | `/slo-ideate` | YC-style product interrogation before any code |
| Research | `/slo-research` | Wraps `sldo-research` Rust backend for sourced dossiers |
| Architect | `/slo-architect` | Stack + `ARCHITECTURE.md` + interfaces lock-in + `tla_required` flag + threat model |
| Verify design | `/slo-tla` | TLC model-check the design (when `tla_required: true`) |
| Plan | `/slo-plan` | Interactive v3 runbook authoring, one milestone at a time |
| Critique | `/slo-critique` | Four-persona adversarial review (CEO, eng-lead, security, designer) |
| Execute | `/slo-execute M<N>` | Per-milestone driver with allow-list enforcement |
| Verify | `/slo-verify M<N>` | Runtime QA with Playwright for UI surfaces |
| Close | `/slo-retro M<N>` | Lessons + completion + tracker update |
| Ship | `/slo-ship` | Open PR with runbook-aware description |

Power tools: `/slo-second-opinion` (cross-model disagreement surfacer), `/slo-freeze <path>` (lock edits to one directory for the session), `/slo-resume` (read tracker, suggest next step).

### Biz pack (UK only, v1)

Four advisor skills with `draft | translate | triage | prepare` modes; eleven generator skills producing one artifact each. See [docs/design/biz-skill-pack-overview.md](docs/design/biz-skill-pack-overview.md) for the full design.

| Skill | Domain |
|---|---|
| `/slo-legal` | NDA, contractor SOW, IP assignment, T&Cs |
| `/slo-accounting` | Bookkeeping, VAT, R&D credit, MTD |
| `/slo-equity` | Cofounder split, vesting, cap-table snapshot |
| `/slo-fundraise` | SAFE math, pitch narrative, term-sheet redline brief |
| `/slo-talk-to-users` | Mom-test interviews + post-call extraction |
| `/slo-gtm` | ICP / motion choice / channel strategy / KPI alignment |
| `/slo-product` | Roadmap / metrics / OKRs |
| `/slo-marketing` | B2B / B2C tactics with PECR routing |
| `/slo-launch` | 4-stage launch sequence + readiness gates |
| `/slo-sales-funnel` | Outbound funnel math + cold email templates |
| `/slo-pricing` | Value-equation pricing + 3-tier-max model |
| `/slo-metrics` | Financial KPI dashboard (consumer / B2B) |
| `/slo-cofounder` | Cofounder evaluation + 4-week paid trial framing |
| `/slo-hire` | IR35-aware hiring with mandatory CEST triage gate |
| `/slo-founder-check` | 12-question self-assessment + worst-case-runway worksheet |

### SAST pack

| Skill | Purpose |
|---|---|
| `/slo-rulegen` | Generate or extend a Semgrep rule pack from variation templates |
| `/slo-ruleverify` | Re-gate the rule pack |

The xtask `cargo xtask sast-verify` exposes `validate`, `test`, `check-coverage`, `check-clean`, `gate`, `detect-tier`, and `validate-file-paths` subcommands. `gate` is the single deterministic entry point that `/slo-rulegen` shells out to before authorising any rule write.

### Vendored

| Skill | Purpose | Prereq |
|---|---|---|
| `/get-api-docs` | Fetch current third-party API docs via `chub` | `npm install -g @aisuite/chub` |

See [skills/get-api-docs/UPSTREAM.md](skills/get-api-docs/UPSTREAM.md) for attribution.

## Project structure

```
.
├── skills/                       # Skill pack (slo-* + get-api-docs)
├── crates/
│   ├── sldo-common/              # Shared library (used by sldo-research)
│   ├── sldo-research/            # Backend driven by /slo-research skill
│   └── sldo-install/             # Skill installer (symlinks skills/* into the selected host root)
├── xtasks/sast-verify/           # cargo xtask sast-verify (Semgrep rule gate; driven by /slo-rulegen + /slo-ruleverify)
├── .semgrep/rust/                # 10/10 CWE rule pack (M1 + M1.5 + M1.6)
├── references/                   # Shared scaffolding read by skills (biz/, sast/)
├── docs/                         # Runbooks, design docs, lessons, completions
├── CLAUDE.md                     # Project guidance for Claude Code
└── SECURITY.md                   # Project-wide security defaults
```

### Baseline test command

```bash
cargo test --workspace
```

## Documentation

Start here:

- [CLAUDE.md](CLAUDE.md) — project guidance read by Claude Code on every session
- [SECURITY.md](SECURITY.md) — project-wide security defaults
- [docs/runbook-template_v_3_template.md](docs/runbook-template_v_3_template.md) — the v3 runbook contract `/slo-plan` produces

Skill-pack design:

- [docs/idea/biz-skill-pack.md](docs/idea/biz-skill-pack.md) — biz-pack idea doc + locked decisions
- [docs/design/biz-skill-pack-overview.md](docs/design/biz-skill-pack-overview.md) — biz-pack design overview
- [docs/design/biz-skill-pack-threat-model.md](docs/design/biz-skill-pack-threat-model.md) — STRIDE × abuse cases × compliance
- [docs/design/sast-rulegen-skill-pack-overview.md](docs/design/sast-rulegen-skill-pack-overview.md) — SAST rule pack design

Per-runbook detail:

- [docs/RUNBOOK-BIZ-SKILL-PACK-A.md](docs/RUNBOOK-BIZ-SKILL-PACK-A.md) — biz-pack 4 advisor skills
- [docs/RUNBOOK-BIZ-SKILL-PACK-B1.md](docs/RUNBOOK-BIZ-SKILL-PACK-B1.md), [B2](docs/RUNBOOK-BIZ-SKILL-PACK-B2.md), [C](docs/RUNBOOK-BIZ-SKILL-PACK-C.md) — biz-pack 11 generator skills
- [docs/RUNBOOK-SAST-RULEGEN-A.md](docs/RUNBOOK-SAST-RULEGEN-A.md) — SAST rule pack 10/10
- [docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md](docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md) — judgment-runtime test harness

## Contributing

Contributions are welcome. The recommended workflow:

1. **Fork + clone.** `git clone <your-fork>` and `cd SunLitOrchestrate`.
2. **Open an issue first** for non-trivial work — the v3 runbook discipline only pays off when the work is scoped before code is written.
3. **Use the skills on yourself.** `/slo-ideate` → `/slo-research` → `/slo-architect` → `/slo-plan` produces a runbook the maintainers can review without any code yet. This is the lowest-friction path to merging.
4. **Pass the baseline.** `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify` must be green.
5. **Open a PR.** The PR description should link to the runbook + the closed milestone's completion summary.

What's currently most welcome:

- **Real-world FP shakedown of the SAST rule pack** against your own Rust codebases, with PRs tightening any over-broad rules (`pattern-not-inside` carve-outs).
- **Fixture additions** to `references/biz/judgment-fixtures/` covering new marginal cases for the advisor skills.
- **New variation templates** at `references/sast/variations/cwe-<NNN>.md` extending the rule pack to additional CWE classes.
- **Documentation polish** — typos, broken links, clearer examples.

Out-of-scope:

- The parked Tauri desktop UI (`crates/sldo-tauri/`) — will resume only when there's a concrete user pulling for it.
- Non-UK jurisdiction support in the biz pack — v1 is UK-only by design; non-UK is a fresh `/slo-architect` pass.

### Code of conduct

Be excellent to each other. We follow the spirit of the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/); no separate file is shipped because the project is small enough that the spirit suffices.

## License

Dual-licensed under either of:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

at your option (see [LICENSE](LICENSE) for both texts). Pick whichever fits your project; you do not need to comply with both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.

## Acknowledgements

- Trail of Bits' [`semgrep-rules`](https://github.com/trailofbits/semgrep-rules) (AGPL) for the structural shape inspiration on the panic-DoS / CWE-755 rule. The SunLitOrchestrate rule pack is independently re-authored from variation templates per the AGPL clean-room policy in [references/sast/AUTHORING.md](references/sast/AUTHORING.md).
- The [oneNDA](https://www.onenda.org/) consortium (CC BY-ND 4.0) for the canonical UK NDA template the biz-pack `/slo-legal draft nda` flow defers to.
- The [SeedLegals](https://seedlegals.com/) public pricing page as the v1 cost baseline anchor for biz-pack ROI claims, alongside JPP Law's fixed-fee public pricing.
