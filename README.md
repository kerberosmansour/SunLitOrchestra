# SunLitOrchestrate

> An AI-first workflow that turns "build this" into scoped, reviewable, testable work. SunLitOrchestrate adds durable guardrails around LLM execution: idea → research → architecture → plan → critique → execute → verify → ship → reflect.

## Why SunLitOrchestrate

SunLitOrchestrate is for teams who like fast LLM output but dislike silent scope drift, missing rationale, and lessons that die in chat history.

- **Stop silent scope widening**: every feature lives in a `docs/RUNBOOK-<feature>.md` with allowed files, forbidden shortcuts, BDD scenarios, abuse cases, and regression tests.
- **Preserve the why**: research dossiers, threat models, runbooks, lessons, and completion summaries survive across sessions and reviewers.
- **Add formal rigor where it matters**: `/slo-tla` gives the workflow an explicit TLA+ step for designs with real concurrency, ordering, or protocol risk, so the system can be challenged as a spec before it is implemented as code.
- **Keep follow-ups alive**: `/slo-retro` captures what was learned, and `/slo-resume` is the "what next?" entrypoint when work gets interrupted.
- **Default to reviewability**: the pack prefers explicit contracts, adversarial critique, and verification over "the model will probably remember".

If this is your first time here, start with [docs/getting-started.md](docs/getting-started.md). Unfamiliar acronyms (TLA+, BDD, CWE, SEIS, IR35, CAC, NDR, …) are defined in [docs/GLOSSARY.md](docs/GLOSSARY.md).

**License:** [Apache-2.0 OR MIT](LICENSE) (dual; pick either) — explicitly NOT AGPL.

**Status:** active development. The skill pack and Rust CLIs are stable.

## What ships here

| Pack | What it is for | Main entrypoints |
|---|---|---|
| Core sprint flow | Turning an idea or change request into a runbook-driven delivery loop | `/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-plan`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship` |
| Security + SAST | Threat-model-by-default design and Semgrep rule-pack generation | `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify` |
| UK biz pack (v1) | Founder, GTM, pricing, legal, accounting, equity, and hiring artifacts for UK-only workflows | `/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`, `/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`, `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`, `/slo-cofounder`, `/slo-hire`, `/slo-founder-check` |

The raw `SKILL.md` contract is agent-neutral. The canonical skill list lives in [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md). Host-specific overlays live in [CLAUDE.md](CLAUDE.md) and [copilot-instructions.md](copilot-instructions.md).

The workflow is intentionally more technical-contract-driven than a typical prompt stack. The v4 runbook contract at [docs/slo/templates/runbook-template_v_4_template.md](docs/slo/templates/runbook-template_v_4_template.md) gives every milestone explicit scope, interfaces, abuse cases, compatibility expectations, verification gates, plus Carmack-style reliability controls (debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, "make invalid states unrepresentable"). For designs with real protocol complexity, `/slo-tla` adds a formal-spec step so the design can be checked with TLA+ before implementation. The earlier [v3 template](docs/slo/templates/runbook-template_v_3_template.md) remains in place as a historical artifact for runbooks already authored against it.

## Pick a starting point

- **New feature or product idea**: run `/slo-ideate` and expect `docs/slo/idea/<slug>.md` as the first artifact.
- **Existing idea doc or known problem**: start at `/slo-research` or `/slo-architect`, depending on whether you still need external evidence.
- **Interrupted runbook**: run `/slo-resume`. It reads the tracker and suggests the next move without starting work for you.
- **Security-only adoption**: jump straight to `/slo-rulegen` or `/slo-sast`.
- **Business-only adoption**: use the UK biz-pack skills without adopting the entire sprint-flow path.

## Quick start

If you want the step-by-step first-run path, read [docs/getting-started.md](docs/getting-started.md). The short version is below.

### Prerequisites

- **Rust toolchain** (stable). `rustup install stable` if you don't have one.
- **A supported host agent**: Claude Code or GitHub Copilot. Claude Code remains the default target if you do not pass `--host`.
- **Semgrep** (`brew install semgrep` or `pip install semgrep`). Required only for the SAST rule-pack path.

### Install the skill pack

```bash
git clone https://github.com/kerberosmansour/SunLitOrchestrate.git
cd SunLitOrchestrate

cargo build -p sldo-install --release

# Claude Code (default host)
./target/release/sldo-install
./target/release/sldo-install status
./target/release/sldo-install verify

# GitHub Copilot
./target/release/sldo-install --host github-copilot
./target/release/sldo-install --host github-copilot status
./target/release/sldo-install --host github-copilot verify

# Project-local installs
./target/release/sldo-install --local
./target/release/sldo-install --host github-copilot --local
```

What success looks like:

- `sldo-install` prints the selected target root.
- `status` lists the installed skills for the host you chose.
- Global installs land in `~/.claude/skills/` or `~/.copilot/skills/`.
- Local installs land in `./.claude/skills/` or `./.copilot/skills/` if you add `--local`.

`/slo-research` now uses host-native research first in both Claude Code and GitHub Copilot. `sldo-research` remains an optional Claude batch backend when you explicitly want that automation path.

### Typical flow

```text
/slo-ideate          # interrogate the problem before code exists
/slo-research        # produce a sourced dossier
/slo-architect       # commit to a stack + threat model
/slo-tla             # optional: model-check the design when concurrency or ordering risk is real
/slo-plan            # write the v3 runbook one milestone at a time
/slo-critique        # run the adversarial review pass
/slo-execute M1      # drive one milestone within the allow-list
/slo-verify M1       # runtime QA + Playwright if UI
/slo-retro M1        # lessons + completion + tracker update
# ... repeat /slo-execute / /slo-verify / /slo-retro per milestone ...
/slo-ship            # open a runbook-aware PR
```

If you step away mid-runbook, use `/slo-resume`. It is the pack's read-only orientation path: it reads the tracker and suggests the next action.

### Security-only quick path

```bash
# In a Rust workspace where you want the rules:
/slo-rulegen
/slo-ruleverify

# Run a specific gate locally:
cargo xtask sast-verify gate .semgrep/rust/cwe-755-panic-on-result-fn.yaml

# Extend the pack from a real bug + fix:
/slo-rulegen --extend \
  --bug-summary /tmp/bug.md \
  --fix-diff /tmp/fix.patch \
  --file-paths src/api/users.rs,src/api/auth.rs
```

CI wiring is documented in [`references/sast/CI-WIRING.md`](references/sast/CI-WIRING.md).

## Host reality

- Claude Code and GitHub Copilot can both install and use the `SKILL.md` pack interactively.
- Claude Code remains the default host if you omit `--host`.
- Headless runtime automation is still host-specific today.
- `sldo-research` as a batch backend and the live business judgment runtime harness are still Claude-only today.
- For exact boundaries, read [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md).

## Full skill map

The README is the orientation page. For the full host-neutral skill list, output paths, and current support boundaries, use:

- [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) — canonical living catalog of shipped skills
- [docs/getting-started.md](docs/getting-started.md) — first-run path with exact commands and expected results
- [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) — what works in Claude Code, GitHub Copilot, or both
- [CLAUDE.md](CLAUDE.md) — Claude Code overlay
- [copilot-instructions.md](copilot-instructions.md) — GitHub Copilot overlay
- [docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md](docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md) — design philosophy: more internal discipline, less user-visible ceremony
- [skills/get-api-docs/UPSTREAM.md](skills/get-api-docs/UPSTREAM.md) — attribution for the vendored `/get-api-docs` helper

## Project structure

```
.
├── skills/                       # Skill pack (slo-* + get-api-docs)
├── crates/
│   ├── sldo-common/              # Shared library (used by the remaining Rust tooling)
│   ├── sldo-research/            # Optional Claude batch backend for /slo-research
│   └── sldo-install/             # Skill installer (symlinks skills/* into the selected host root)
├── xtasks/sast-verify/           # cargo xtask sast-verify (Semgrep rule gate; driven by /slo-rulegen + /slo-ruleverify)
├── .semgrep/rust/                # 10/10 CWE rule pack (M1 + M1.5 + M1.6)
├── references/                   # Shared scaffolding read by skills (biz/, sast/)
├── docs/                         # Runbooks, design docs, lessons, completions
├── CLAUDE.md                     # Claude Code overlay for the canonical skill catalog
├── copilot-instructions.md       # GitHub Copilot overlay for the same skill pack
└── SECURITY.md                   # Project-wide security defaults
```

### Baseline test command

```bash
cargo test -p sldo-common -p sldo-install -p sldo-research
```

## Documentation

Docs live in-repo today.

Start here:

- [docs/getting-started.md](docs/getting-started.md) — first-run guide with exact commands and expected results
- [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) — canonical living catalog of shipped skills
- [docs/slo/templates/runbook-template_v_4_template.md](docs/slo/templates/runbook-template_v_4_template.md) — the canonical v4 runbook contract `/slo-plan` produces (Carmack-style reliability controls on top of v3)
- [docs/slo/templates/runbook-template_v_3_template.md](docs/slo/templates/runbook-template_v_3_template.md) — historical v3 template for runbooks already authored against it
- [docs/LOOPS-ENGINEERING.md](docs/LOOPS-ENGINEERING.md) — engineering feedback loops (sprint, security-tuning, lessons, library-feedback)
- [docs/LOOPS-BUSINESS.md](docs/LOOPS-BUSINESS.md) — business feedback loops (user-interview, GTM, pricing, founder-check)
- [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) — capability matrix for install, interactive use, and headless automation
- [docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md](docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md) — why the pack prefers more internal discipline with less user-visible ceremony
- [SECURITY.md](SECURITY.md) — project-wide security defaults
- [CLAUDE.md](CLAUDE.md) — Claude Code overlay
- [copilot-instructions.md](copilot-instructions.md) — GitHub Copilot overlay

Skill-pack design:

- [docs/slo/idea/biz-skill-pack.md](docs/slo/idea/biz-skill-pack.md) — biz-pack idea doc + locked decisions
- [docs/slo/design/biz-skill-pack-overview.md](docs/slo/design/biz-skill-pack-overview.md) — biz-pack design overview
- [docs/slo/design/biz-skill-pack-threat-model.md](docs/slo/design/biz-skill-pack-threat-model.md) — STRIDE × abuse cases × compliance
- [docs/slo/design/sast-rulegen-skill-pack-overview.md](docs/slo/design/sast-rulegen-skill-pack-overview.md) — SAST rule pack design

Per-runbook detail:

- [docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md](docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-A.md) — biz-pack 4 advisor skills
- [docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-B1.md](docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-B1.md), [B2](docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-B2.md), [C](docs/slo/completed/RUNBOOK-BIZ-SKILL-PACK-C.md) — biz-pack 11 generator skills
- [docs/slo/completed/RUNBOOK-SAST-RULEGEN-A.md](docs/slo/completed/RUNBOOK-SAST-RULEGEN-A.md) — SAST rule pack 10/10
- [docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md](docs/slo/completed/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md) — judgment-runtime test harness

## Contributing

Contributions are welcome. The recommended workflow:

1. **Fork + clone.** `git clone <your-fork>` and `cd SunLitOrchestrate`.
2. **Open an issue first** for non-trivial work — the v3 runbook discipline only pays off when the work is scoped before code is written.
3. **Use the skills on yourself.** `/slo-ideate` → `/slo-research` → `/slo-architect` → `/slo-plan` produces a runbook the maintainers can review without any code yet. This is the lowest-friction path to merging.
4. **Pass the baseline.** `cargo test -p sldo-common -p sldo-install -p sldo-research` must be green.
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

This project adopts the [Contributor Covenant 2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for the full text and how to report unacceptable behavior.

## License

Dual-licensed under either of:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

at your option (see [LICENSE](LICENSE) for both texts). Pick whichever fits your project; you do not need to comply with both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.

## Acknowledgements

- Trail of Bits' [`semgrep-rules`](https://github.com/trailofbits/semgrep-rules) (AGPL) for the structural shape inspiration on the panic-DoS / CWE-755 rule. The SunLitOrchestrate rule pack is independently re-authored from variation templates per the AGPL clean-room policy in [references/sast/AUTHORING.md](references/sast/AUTHORING.md).
- The [BMad Method](https://github.com/bmad-code-org/BMAD-METHOD) for helping popularize structured, lifecycle-aware AI-assisted development. SunLitOrchestrate arrives at a more contract-heavy, security-first shape, but it shares the belief that better outcomes come from explicit workflow rather than one-shot prompting.
- [TLA+](https://github.com/tlaplus/tlaplus), and the broader formal-methods tradition around it, for reinforcing the idea that some designs should be challenged as specifications before they are implemented as code. SunLit carries that idea through `/slo-tla` when concurrency, ordering, or protocol risk is real.
- Jim Manico's talk [*Securing Claude Code: Guardrails for AI-Assisted Development*](https://youtu.be/thsdAsgIsFc?si=FvxYtdHyus7DQTe7) for sharpening the guardrail-first mindset behind the project's threat-modeling, verification, and "no agentic shortcuts" posture.
- John Carmack's [*Best programming setup and IDE*](https://youtu.be/tzr7hRXcwkw?si=SeeakVCVpqWatOUl) clip from the Lex Fridman Podcast for influencing the v4 runbook template's Carmack-style reliability controls — debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, and "make invalid states unrepresentable".
- The [oneNDA](https://www.onenda.org/) consortium for the canonical UK NDA template the biz-pack `/slo-legal draft nda` flow points users to. Licensed under [CC BY-ND 4.0](https://creativecommons.org/licenses/by-nd/4.0/); the canonical `.docx` is fetched manually by the user from onenda.org and is never copied, modified, rendered, or redistributed by this repo. The skill produces only a separate Markdown cover artifact for company-specific fields — assembly happens on the user's machine, against the user's locally-downloaded canonical `.docx`.
- The [SeedLegals](https://seedlegals.com/) public pricing page as the v1 cost baseline anchor for biz-pack ROI claims, alongside JPP Law's fixed-fee public pricing.
