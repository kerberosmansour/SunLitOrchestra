# SunLit Orchestra

> A contract-driven workflow for AI-assisted software delivery. SunLit Orchestra turns "build this" into scoped, reviewable, testable work, with security considerations and durable guardrails wired in at every stage: idea → research → architecture → plan → critique → execute → verify → ship → reflect.

Three contributions make this pack distinct: **(1) a runbook artifact that bakes best practices into every milestone**, **(2) security wired into every step rather than bolted on at the end**, and **(3) an opt-in TLA+ formal-specification stage for designs that need it**. See [What makes SunLit Orchestra different](#what-makes-sunlit-orchestra-different) for what each one actually buys you.

**License:** [Apache-2.0 OR MIT](LICENSE) (dual; pick either) — explicitly NOT AGPL. **Status:** active development. The skill pack and Rust CLIs are stable.

## Table of contents

- [At a glance](#at-a-glance)
- [The problem](#the-problem)
- [What SunLit Orchestra is](#what-sunlit-orchestra-is)
- [What makes SunLit Orchestra different](#what-makes-sunlit-orchestra-different)
- [How it works](#how-it-works)
- [What the output looks like](#what-the-output-looks-like)
- [When NOT to use it](#when-not-to-use-it)
- [Quick start](#quick-start)
- [Architecture at a glance](#architecture-at-a-glance)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)

## At a glance

SunLit Orchestra is a portable `/slo-*` skill pack for disciplined AI-assisted delivery. It gives a host agent a sequence of reviewable contracts instead of one loose prompt.

| If you have... | Start with... | You get... |
|---|---|---|
| A weird technical hunch that is not yet a feature | `/slo-experiment <slug>` | An Innovation Sandbox Experiment Book: playful probes first, then a Protocol Freeze, separate Discovery/Validation Records, and a confidence-calibrated RecommendationPacket or reusable dead end |
| A new product or feature idea | `/slo-ideate` | Problem framing, risks, and a path into research/design |
| A scoped GitHub issue | `/slo-ticket-pick #<issue>` | A compact ticket contract, evidence log, and PR handoff |
| A full feature that needs planning | `/slo-plan` after ideate/research/architecture | A v4 runbook with milestones, BDD, abuse cases, and gates |
| A security-only need (code) | `/slo-sast` · `/slo-rulegen` | Threat-model-driven Semgrep + a custom rule pack and a deterministic gate |
| A security-only need (running app) | `/slo-dast-tuner` | Tuned, authenticated ZAP scan + a [SAST→DAST bridge](skills/slo-dast-tuner/README.md) that turns code findings into targeted web tests |
| A UK founder/business artifact | The relevant `/slo-*` biz skill | Drafts or decision artifacts with hard-block gates |

Supported interactive hosts:

| Host | Install target | Overlay |
|---|---|---|
| Claude Code | `~/.claude/skills/` | [CLAUDE.md](CLAUDE.md) |
| GitHub Copilot | `~/.copilot/skills/` | [copilot-instructions.md](copilot-instructions.md) |
| Codex | `~/.codex/skills/` | [AGENTS.md](AGENTS.md) |

These install targets are `sldo-install` compatibility root paths. Current host-native project skill roots may differ: GitHub Copilot documents `.github/skills` and `.agents/skills`, and Codex documents `.agents/skills`. SunLit keeps `~/.copilot/skills`, `./.copilot/skills`, `~/.codex/skills`, and `./.codex/skills` stable until an explicit installer migration exists.

## The problem

LLM-assisted development is fast at producing code and slow at producing the things that make code useful afterwards: a clear scope, a recorded rationale, an honest threat model, an executable verification plan, and lessons that survive the next conversation. The default failure modes are easy to recognise:

- **Silent scope drift** — the model "helpfully" rewrites adjacent files; the diff balloons; review becomes a coin flip.
- **Lost rationale** — the *why* lives in a chat transcript that no one reads three weeks later.
- **Verification theatre** — "the tests pass" without a written contract for what the tests are supposed to prove.
- **Security as an afterthought** — threat modelling, SAST, and abuse cases happen (if at all) after the code is already merged.
- **Lessons that die in chat** — the same mistake gets re-made next sprint because nothing was captured outside the session.

The bet behind SunLit Orchestra is that these failures are not LLM limitations — they are *workflow* limitations. The model can produce strong work when the surrounding workflow forces it to commit to scope, rationale, and verification before code is written.

## What SunLit Orchestra is

A **skill pack** - a set of `/slo-*` slash commands installed into a host AI coding agent (Claude Code by default; GitHub Copilot and Codex also supported) - plus a small Rust toolchain that installs the pack, provides an optional batch backend for `/slo-research`, and runs a Semgrep rule gate (`cargo xtask sast-verify`) wired to the SAST skills.

The pack is organised around four workflows that compose:

1. **Core sprint flow** — `/slo-ideate → /slo-research → /slo-architect → /slo-plan → /slo-critique → /slo-execute → /slo-verify → /slo-retro → /slo-ship`. Each stage produces a versioned artifact under `docs/` that the next stage (and the next reviewer) can rely on. Optional `/slo-tla` adds a TLA+ model-check step for designs with real concurrency, ordering, or protocol risk.
2. **Ticket-sized SLO flow** — `/slo-ticket-pick → /slo-ticket-plan → /slo-ticket-execute → /slo-ticket-verify → /slo-ticket-close`. This is the GitHub Issues-first path for bite-sized work: one issue, one compact v4-derived ticket contract, one branch, one PR, and one evidence trail.
3. **Security: SAST + DAST** — `/slo-rulegen`, `/slo-ruleverify`, `/slo-sast` (read the source for bugs), and `/slo-dast-tuner` (test the *running* app like an attacker). A 10/10 CWE Semgrep rule pack is included and CI-gated; a **SAST→DAST bridge** lets the code-scanner findings drive targeted, authenticated web tests. Plain-language intros: [`/slo-sast`](skills/slo-sast/README.md) · [`/slo-dast-tuner`](skills/slo-dast-tuner/README.md).
4. **UK biz pack (v1)** — 15 founder, GTM, pricing, legal, accounting, equity, and hiring artifact skills, with mandatory hard-block gates (regulated work, large counterparties, GDPR, IR35) and confidential-vs-public output tiers.

The raw `SKILL.md` contract is agent-neutral, so the pack is portable between hosts. Canonical list: [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md).

## What makes SunLit Orchestra different

Three contributions distinguish this pack from other AI-coding workflows. Everything else in the project exists to support them.

### 1. The runbook artifact — a template that bakes in best practices

Every feature is bound to a `docs/RUNBOOK-<feature>.md` produced by `/slo-plan` against the canonical [v4 runbook template](docs/slo/templates/runbook-template_v_4_template.md). The template *is* the contribution — it forces every milestone, before any code is written, to declare:

- **Scope** — exactly which files are allowed and which shortcuts are forbidden, so out-of-scope edits become contract violations rather than judgment calls the model gets to make on its own.
- **Behaviour** — BDD scenarios for the golden path, *plus* explicit abuse cases for the failure modes an attacker or careless caller would hit.
- **Verification** — the regression tests that close the loop, the interface contracts the milestone must respect, and the gates `/slo-verify` will run.
- **Reliability** — Carmack-style controls layered on top of v3: debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, "make invalid states unrepresentable".
- **Carry-forward** — lessons and threat-model deltas pulled from prior retros, so each runbook *learns* from the last one rather than starting blank.

The runbook outlives the LLM session that produced it. It is what the next reviewer reads, what the next runbook inherits from, and what the project's institutional memory is recorded in. Best practices stop being something you have to remember to apply — they are baked into the artifact you cannot avoid producing.

### 2. Security on every step, not a separate phase

Most AI-coding workflows treat security as a closing checklist or a separate audit pass. SunLit Orchestra wires it into the sprint flow from the very first stage, and threads it forward through every subsequent one:

- **`/slo-ideate`** — before any architecture exists, ideation asks: *"what is the worst day this system causes?"* The output must name three concrete failure outcomes — a breach (which data leaves the trust boundary, to whom), a compliance fine (which regulation, what scale), or a prolonged outage (who notices, how long until the user defects). Vague answers like "security matters" or "downtime is bad" are explicitly rejected. Those named risks are what `/slo-architect` builds the threat model from — security is on the table before code exists as a concept.
- **`/slo-architect`** — produces a STRIDE threat model and an abuse-case set on *every* pass, not optionally and not later. The threat-model rows become the seed for the abuse cases the next stage carries forward, and the architecture doc records the security best practices the build is committed to (proactive-controls vocabulary, named secure libraries, data-classification posture).
- **`/slo-plan`** — the v4 runbook template has three mandatory security rows in the Contract Block of every milestone: **data classification** (`Public | Internal | Confidential | Restricted`), **proactive controls in play** (OWASP C1–C10 and named libraries from the SunLitSecurityLibraries / Hulumi vocabularies), and **abuse acceptance scenarios** (concrete attacker-role + step + outcome, each citing a threat-model row). Silent omission is forbidden — the only acceptable empty answer is `N/A — no new surface introduced` with a reason. BDD Acceptance Scenarios additionally require abuse-case rows whenever a new surface is introduced. The plan template *bakes in* secure coding and testing — you cannot ship a runbook that doesn't account for them.
- **`/slo-execute`** — writes BDD tests first, *including the abuse-case rows*. Security tests are part of the red-green TDD cycle from the start, not bolted on after the feature works. The skill is also bound to the runbook's allow-list, so it cannot silently widen scope into adjacent files — closing one of the most common attack surfaces against the workflow itself. Each security consideration declared in the plan becomes a concrete test the milestone has to pass.
- **`/slo-verify`** — runs a PII-pattern scan over public output paths (Pass 4), so confidential drafts cannot accidentally leak through the public tier; runtime QA closes the loop on the abuse-case scenarios.
- **`/slo-rulegen` + `/slo-ruleverify` + `/slo-sast`** — maintain and CI-gate a 10/10 CWE-class Semgrep rule pack against the codebase, so the static-analysis layer is also kept honest as the project evolves.
- **`/slo-dast-tuner` + the SAST→DAST bridge** — closes the loop at runtime: it reads the codebase to turn a code-scanner finding ("file X, line 42") into a concrete authenticated web test ("attack `GET /research`, log in first"), then proves it against the live app. On a deliberately-vulnerable practice app this turned a blind default scan's **0 confirmed** into **4 confirmed** real bugs. It is honest about coverage: an unauthenticated scan of a login-gated app is reported as a *coverage failure*, never "clean".
- **The UK biz pack** — adds hard-block gates (regulated work, counterparties >£5,000, GDPR-scoped documents, IR35-triggering hires) that refuse to draft until the right human is in the loop.

The consequence: there is no "security review" stage at the end, because there is no stage *without* security in it. From the first ideation question — *"what's the worst that can happen?"* — through threat model, plan template, BDD test scaffolding, runtime verification, and CI rule gate, security is something the workflow *forces* you to commit to and execute on, not something you might remember to do.

### 3. TLA+ formal specification for designs that earn it

Almost no AI-coding workflow offers a formal-methods step at all. SunLit Orchestra adds `/slo-tla` as an explicit stage between architecture and plan, for designs where concurrency, ordering, or protocol risk is real. The skill drives a TLC model-check of the design *as a specification* before any code is written — so the class of bug that would only surface under contention, partial failure, or unusual interleavings in production gets caught while the design is still text.

The decision of whether a runbook needs TLA+ is set during `/slo-architect` via the `tla_required: true | false` field, so it is visible, recorded, and reviewable rather than implicit. When the flag is true, the runbook cannot proceed past plan without a passing model-check; when false, the reasoning for skipping it is recorded in the architecture doc. Either way, the choice is on the record.

The result: well-designed solutions are the default, not an artifact of "the engineer happened to think about it carefully this time".

## How it works

A normal sprint runs the sprint-flow skills in order. Each stage produces a checked-in artifact under `docs/` that the next stage (and the next reviewer) can rely on — the workflow is the chain of artifacts, not a chain of prompts.

```text
/slo-ideate          # interrogate the problem; Q7 forces "what's the worst day this system causes?"
/slo-research        # produce a sourced dossier with named adversaries, named regulations, named UX failures
/slo-architect       # commit to a stack, build a STRIDE threat model, record security best practices
/slo-tla             # optional: model-check the design when concurrency / ordering / protocol risk is real
/slo-plan            # write the v4 runbook one milestone at a time (data classification + proactive controls + abuse cases mandatory)
/slo-critique        # four-persona adversarial review (CEO, eng, security, design) before any code is written
/slo-execute M1      # drive one milestone within the allow-list — BDD tests first, including abuse-case rows
/slo-verify M1       # runtime QA + Playwright (if UI) + PII scan over public outputs
/slo-retro M1        # lessons + completion summary + tracker update
# ... repeat /slo-execute / /slo-verify / /slo-retro per milestone ...
/slo-ship            # open a runbook-aware PR linking to the artifacts above
```

Two re-entry paths matter:

- **`/slo-resume`** — the pack's read-only orientation path. If you step away mid-runbook, it reads the tracker and suggests the next action without starting work for you.
- **Security-only adoption** — `/slo-rulegen` and `/slo-ruleverify` can run standalone against any Rust codebase to maintain the CWE Semgrep rule pack, without running the rest of the sprint flow. See [Quick start → Security-only quick path](#security-only-quick-path).

For small tracker-driven work, use the ticket-sized flow instead of minting a full runbook:

```text
/slo-ticket-pick #123       # claim/normalize one GitHub issue and create the issue workpad
/slo-ticket-plan #123       # write docs/slo/tickets/ticket-123-<slug>.md from the compact v4-derived template
/slo-ticket-execute ...     # BDD-first implementation inside the ticket allow-list
/slo-ticket-verify ...      # runtime/static/security evidence against the ticket contract
/slo-ticket-close ...       # PR handoff; no auto-merge or silent issue close
```

The artifacts produced by each stage live under `docs/slo/` (idea, research dossier, architecture, threat model, runbook, retro, completion summary). They are the project's institutional memory and the input to the *next* runbook's carry-forward.

## What the output looks like

A normal run leaves a reviewable paper trail rather than a chat-only rationale:

| Stage | Primary artifact | What reviewers get |
|---|---|---|
| `/slo-ideate` | `docs/slo/idea/<slug>.md` | Problem framing, non-goals, and worst-day failure outcomes |
| `/slo-research` | `docs/slo/research/<slug>/` | Sourced evidence, assumptions, and open questions |
| `/slo-architect` | `docs/slo/design/<slug>-overview.md` + threat model | Stack decision, interfaces, STRIDE rows, abuse cases, security posture |
| `/slo-tla` | TLA+ spec + TLC output when required | Model-check evidence for concurrency, ordering, or protocol risk |
| `/slo-plan` | `docs/RUNBOOK-<feature>.md` | Milestone scope, allowed files, BDD scenarios, abuse cases, verification gates |
| `/slo-execute` + `/slo-verify` | Code, tests, evidence log | Implementation constrained to the runbook, with proof that gates ran |
| `/slo-retro` + `/slo-ship` | Lessons, completion summary, PR body | Carry-forward lessons and links reviewers can audit without reading the whole chat |
| `/slo-ticket-*` | `docs/slo/tickets/ticket-<issue>-<slug>.md`, issue workpad, PR body | GitHub issue execution with a compact v4-derived contract and evidence trail |

## When NOT to use it

- **Throwaway scripts and one-shot prototypes.** The runbook discipline doesn't pay back if the artifact won't be read twice.
- **Teams that want a low-friction "vibe-code with the LLM" loop.** SunLit Orchestra is intentionally heavier on contracts than on autonomy. If you don't want the contracts, the rest of the pack will feel like ceremony.
- **Non-UK jurisdictions for the biz pack.** v1 is UK-only by design; non-UK is a fresh `/slo-architect` pass, not a flag.
- **Headless / CI-only automation as the primary path.** Most skills are interactive today. See [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) for the exact boundary.

## Where to go next

If this is your first time here, start with [docs/getting-started.md](docs/getting-started.md). Unfamiliar acronyms (TLA+, BDD, CWE, SEIS, IR35, CAC, NDR, …) are defined in [docs/GLOSSARY.md](docs/GLOSSARY.md).

## What ships here

| Pack | What it is for | Main entrypoints |
|---|---|---|
| Core sprint flow | Turning an idea or change request into a runbook-driven delivery loop | `/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-plan`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship` |
| Ticket-sized SLO flow | Turning a GitHub issue into one bounded, reviewable PR with v4-style evidence | `/slo-ticket-pick`, `/slo-ticket-plan`, `/slo-ticket-execute`, `/slo-ticket-verify`, `/slo-ticket-close` |
| Security + SAST | Threat-model-by-default design and Semgrep rule-pack generation | `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify` |
| UK biz pack (v1) | Founder, GTM, pricing, legal, accounting, equity, and hiring artifacts for UK-only workflows | `/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`, `/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`, `/slo-launch`, `/slo-sales-funnel`, `/slo-pricing`, `/slo-metrics`, `/slo-cofounder`, `/slo-hire`, `/slo-founder-check` |

The raw `SKILL.md` contract is agent-neutral. The canonical skill list lives in [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md). Host-specific overlays live in [CLAUDE.md](CLAUDE.md), [copilot-instructions.md](copilot-instructions.md), and [AGENTS.md](AGENTS.md).

The workflow is intentionally more technical-contract-driven than a typical prompt stack. The v4 runbook contract at [docs/slo/templates/runbook-template_v_4_template.md](docs/slo/templates/runbook-template_v_4_template.md) gives every milestone explicit scope, interfaces, abuse cases, compatibility expectations, verification gates, plus Carmack-style reliability controls (debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, "make invalid states unrepresentable"). For designs with real protocol complexity, `/slo-tla` adds a formal-spec step so the design can be checked with TLA+ before implementation. The earlier [v3 template](docs/slo/templates/runbook-template_v_3_template.md) remains in place as a historical artifact for runbooks already authored against it.

## Pick a starting point

- **New feature or product idea**: run `/slo-ideate` and expect `docs/slo/idea/<slug>.md` as the first artifact.
- **Small GitHub issue or ticket**: run `/slo-ticket-pick #<issue>` and expect `docs/slo/tickets/ticket-<issue>-<slug>.md` after `/slo-ticket-plan`.
- **Existing idea doc or known problem**: start at `/slo-research` or `/slo-architect`, depending on whether you still need external evidence.
- **Interrupted runbook**: run `/slo-resume`. It reads the tracker and suggests the next move without starting work for you.
- **Security-only adoption**: jump straight to `/slo-rulegen` or `/slo-sast`.
- **Business-only adoption**: use the UK biz-pack skills without adopting the entire sprint-flow path.

## Quick start

If you want the step-by-step first-run path, read [docs/getting-started.md](docs/getting-started.md). The short version is below.

### Prerequisites

- **Rust toolchain** (stable). `rustup install stable` if you don't have one.
- **A supported host agent**: Claude Code, GitHub Copilot, or Codex. Claude Code remains the default target if you do not pass `--host`.
- **Semgrep** (`brew install semgrep` or `pip install semgrep`). Required only for the SAST rule-pack path.

### Install the skill pack

There are two install paths. Pick whichever fits.

**Path A — `cargo install` from crates.io** (no local clone needed):

```bash
cargo install sldo-install

# Claude Code (default host)
sldo-install
sldo-install status
sldo-install verify

# GitHub Copilot / Codex
sldo-install --host github-copilot
sldo-install --host codex
```

**Path B — Build from source** (needed when iterating on the skill pack itself):

```bash
git clone https://github.com/kerberosmansour/SunLitOrchestra.git
cd SunLitOrchestra

cargo build -p sldo-install --release

# Claude Code (default host)
./target/release/sldo-install
./target/release/sldo-install status
./target/release/sldo-install verify

# GitHub Copilot
./target/release/sldo-install --host github-copilot
./target/release/sldo-install --host github-copilot status
./target/release/sldo-install --host github-copilot verify

# Codex
./target/release/sldo-install --host codex
./target/release/sldo-install --host codex status
./target/release/sldo-install --host codex verify

# Project-local installs
./target/release/sldo-install --local
./target/release/sldo-install --host github-copilot --local
./target/release/sldo-install --host codex --local
```

What success looks like:

- `sldo-install` prints the selected target root.
- `status` lists the installed skills for the host you chose.
- Global installs land in `~/.claude/skills/`, `~/.copilot/skills/`, or `~/.codex/skills/`.
- Local installs land in `./.claude/skills/`, `./.copilot/skills/`, or `./.codex/skills/` if you add `--local`.
- These paths are SLO installer compatibility root paths. They intentionally remain documented even where the host's official project-skill root is now `.github/skills` or `.agents/skills`.
- On Windows PowerShell, use `.\target\release\sldo-install.exe` with the same flags. Native Windows shells can rely on `%USERPROFILE%` when `HOME` is not set.
- Linux and macOS installs use directory symlinks. Windows tries directory symlinks first, then falls back to directory junctions if symlink privileges are unavailable.

`/slo-research` now uses host-native research first in Claude Code, GitHub Copilot, and Codex. `sldo-research` remains an optional Claude batch backend when you explicitly want that automation path.

For the canonical sprint sequence (ideate → ship), see [How it works](#how-it-works) above.

### Install via Claude plugin (optional, additive)

Claude Code organizational installs may prefer a one-zip distribution over cloning the repo. A `.claude-plugin/plugin.json` is published; tagged releases also produce a downloadable zip via the SHA-pinned [release-zip workflow](.github/workflows/release-zip.yml).

**The Rust installer remains canonical.** `sldo-install` is the supported install path for Claude Code, GitHub Copilot, and Codex; the plugin distribution is additive and Claude-only. GitHub Copilot users continue to use `sldo-install --host github-copilot`; Codex users use `sldo-install --host codex`. Choosing the plugin path on Claude Code does not bypass `sldo-install`'s manifest at `~/.sldo/install.toml` - both paths point at the same `skills/` tree.

### Examples

The [examples/](examples/) directory contains a synthetic, non-normative gallery showing what shipped SLO outputs look like — runbook excerpts, critique reports, verification reports, security findings, SAST manifests, and biz-public artifacts. Read these to calibrate quality before running a skill. Examples are deliberately small (≤ 10 KB each), deliberately synthetic (no real PII), and deliberately non-canonical — read the templates and SKILL.md files for the real contracts.

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

### Formal verification demo (Kani)

`/slo-kani` brings the [Kani](https://model-checking.github.io/kani/) Rust model checker into the loop as a code-level peer to `/slo-tla`. The seeded-bug demo crate [`kerberosmansour/sunlit-kani-demo`](https://github.com/kerberosmansour/sunlit-kani-demo) proves the catch→remediate→green failure bar (off-by-one, unsafe one-past-end read, overflow, gcd contract). Red→green evidence: [`docs/slo/verify/kani-verification-kani.md`](docs/slo/verify/kani-verification-kani.md).

## Host reality

- Claude Code, GitHub Copilot, and Codex can install and use the `SKILL.md` pack interactively.
- Claude Code remains the default host if you omit `--host`.
- Headless runtime automation is still host-specific today.
- `sldo-research` as a batch backend and the live business judgment runtime harness are still Claude-only today.
- For exact boundaries, read [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md).

## Architecture at a glance

SunLit Orchestra is intentionally split into artifacts that can be inspected independently:

- **`skills/`** — the host-facing `/slo-*` procedures. These drive the workflow and define what each stage may read or write.
- **`references/`** — shared policy, reporting templates, and scaffolding for the security, SAST, and biz-pack flows.
- **`crates/`** — small Rust CLIs for installation, optional research batching, and deterministic Semgrep rule gating.
- **`docs/`** — generated and hand-authored project memory: ideas, research, architecture, threat models, runbooks, completions, and lessons.
- **`.semgrep/` + `xtasks/`** — the rule pack and local verification harness that keep SAST changes testable.

Skills orchestrate the work, references keep policy consistent, Rust tools enforce the parts that should be deterministic, and docs carry the context forward.

## Full skill map

The README is the orientation page. For the full host-neutral skill list, output paths, and current support boundaries, use:

- [docs/skill-pack-catalog.md](docs/skill-pack-catalog.md) — canonical living catalog of shipped skills
- [docs/getting-started.md](docs/getting-started.md) — first-run path with exact commands and expected results
- [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) — what works in Claude Code, GitHub Copilot, Codex, or host-specific automation
- [CLAUDE.md](CLAUDE.md) — Claude Code overlay
- [copilot-instructions.md](copilot-instructions.md) — GitHub Copilot overlay
- [AGENTS.md](AGENTS.md) — Codex overlay
- [docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md](docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md) — design philosophy: more internal discipline, less user-visible ceremony
- [skills/get-api-docs/UPSTREAM.md](skills/get-api-docs/UPSTREAM.md) — attribution for the vendored `/get-api-docs` helper

## Project structure

```
.
├── skills/                       # Skill pack (slo-* + get-api-docs)
├── crates/
│   ├── sldo-common/              # Shared library (used by the remaining Rust tooling)
│   ├── sldo-research/            # Optional Claude batch backend for /slo-research
│   └── sldo-install/             # Skill installer (managed-links skills/* into the selected host root)
├── xtasks/sast-verify/           # cargo xtask sast-verify (Semgrep rule gate; driven by /slo-rulegen + /slo-ruleverify)
├── .semgrep/rust/                # 10/10 CWE rule pack (M1 + M1.5 + M1.6)
├── references/                   # Shared scaffolding read by skills (biz/, sast/)
├── docs/                         # Runbooks, design docs, lessons, completions
├── CLAUDE.md                     # Claude Code overlay for the canonical skill catalog
├── copilot-instructions.md       # GitHub Copilot overlay for the same skill pack
├── AGENTS.md                     # Codex overlay for the same skill pack
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
- [docs/slo/experiments/example-context-validator/EXPERIMENT.md](docs/slo/experiments/example-context-validator/EXPERIMENT.md) — synthetic Innovation Sandbox gallery: creative discovery → Protocol Freeze → held-out Validation Record → RecommendationPacket
- [skills/slo-sast/README.md](skills/slo-sast/README.md) — plain-language intro to the code-scanning skill (what SAST is, why, quick start)
- [skills/slo-dast-tuner/README.md](skills/slo-dast-tuner/README.md) — plain-language intro to the running-app scanning skill, the SAST→DAST bridge, and the 12-framework adapter catalog
- [docs/slo/templates/runbook-template_v_4_template.md](docs/slo/templates/runbook-template_v_4_template.md) — the canonical v4 runbook contract `/slo-plan` produces (Carmack-style reliability controls on top of v3)
- [docs/slo/templates/ticket-contract-template_v_1.md](docs/slo/templates/ticket-contract-template_v_1.md) — compact v4-derived contract for bite-sized GitHub issue work
- [docs/slo/templates/runbook-template_v_3_template.md](docs/slo/templates/runbook-template_v_3_template.md) — historical v3 template for runbooks already authored against it
- [references/templates/](references/templates/) — shared citation, intake, tool-safety, fallback, eval, and pinning discipline used by engineering skills
- `skills/<skill>/evals/` — documented behavioral expectations for high-risk skills; case shape lives in [references/templates/eval-cases.md](references/templates/eval-cases.md)
- [docs/LOOPS-ENGINEERING.md](docs/LOOPS-ENGINEERING.md) — engineering feedback loops (sprint, ticket, security-tuning, lessons, library-feedback)
- [docs/LOOPS-BUSINESS.md](docs/LOOPS-BUSINESS.md) — business feedback loops (user-interview, GTM, pricing, founder-check)
- [docs/slo/design/agent-host-capabilities.md](docs/slo/design/agent-host-capabilities.md) — capability matrix for install, interactive use, and headless automation
- [docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md](docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md) — why the pack prefers more internal discipline with less user-visible ceremony
- [SECURITY.md](SECURITY.md) — project-wide security defaults
- [CLAUDE.md](CLAUDE.md) — Claude Code overlay
- [copilot-instructions.md](copilot-instructions.md) — GitHub Copilot overlay
- [AGENTS.md](AGENTS.md) — Codex overlay

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

1. **Fork + clone.** `git clone <your-fork>` and `cd SunLitOrchestra`.
2. **Open an issue first** for non-trivial work. Use the ticket-sized SLO flow for small changes and the full v4 runbook flow for larger ones.
3. **Use the skills on yourself.** `/slo-ideate` -> `/slo-research` -> `/slo-architect` -> `/slo-plan` produces a runbook the maintainers can review without any code yet. For a small issue, use `/slo-ticket-pick` -> `/slo-ticket-plan`.
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

Copyright 2026 Sherif Mansour. An open-source project by Sherif Mansour.

Dual-licensed under either of:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

at your option (canonical texts: [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT); the dual-license arrangement is described in [LICENSE](LICENSE); the project-level copyright notice is in [NOTICE](NOTICE)). Pick whichever fits your project; you do not need to comply with both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions. Contributions require a Developer Certificate of Origin sign-off — see [CONTRIBUTING.md](CONTRIBUTING.md#sign-off--developer-certificate-of-origin).

## Trade-marks

**SunLit Orchestra** and the associated logo are unregistered trade-marks of Sherif Mansour. The Apache-2.0 / MIT licences grant rights in the code, not in the name or logo — see [TRADEMARKS.md](TRADEMARKS.md) for what permission you do and do not need.

## Acknowledgements

- Trail of Bits' [`semgrep-rules`](https://github.com/trailofbits/semgrep-rules) (AGPL) for the structural shape inspiration on the panic-DoS / CWE-755 rule. The SunLit Orchestra rule pack is independently re-authored from variation templates per the AGPL clean-room policy in [references/sast/AUTHORING.md](references/sast/AUTHORING.md).
- The [BMad Method](https://github.com/bmad-code-org/BMAD-METHOD) for helping popularize structured, lifecycle-aware AI-assisted development. SunLit Orchestra arrives at a more contract-heavy, security-first shape, but it shares the belief that better outcomes come from explicit workflow rather than one-shot prompting.
- Andrej Karpathy's four-rule CLAUDE.md framing, as summarized in [this r/AIAgentsInAction post](https://www.reddit.com/r/AIAgentsInAction/comments/1tgnulq/karpathys_4_rules_for_claudemd_was_1_on_github/): ask rather than assume, prefer the simplest working solution, leave unrelated code alone, and flag uncertainty explicitly. SunLit generalizes those rules into the shared [agent operating contract](references/agent/operating-contract.md) used by Claude Code, GitHub Copilot, and Codex overlays.
- [TLA+](https://github.com/tlaplus/tlaplus), and the broader formal-methods tradition around it, for reinforcing the idea that some designs should be challenged as specifications before they are implemented as code. SunLit carries that idea through `/slo-tla` when concurrency, ordering, or protocol risk is real.
- Jim Manico's talk [*Securing Claude Code: Guardrails for AI-Assisted Development*](https://youtu.be/thsdAsgIsFc?si=FvxYtdHyus7DQTe7) for sharpening the guardrail-first mindset behind the project's threat-modeling, verification, and "no agentic shortcuts" posture.
- John Carmack's [*Best programming setup and IDE*](https://youtu.be/tzr7hRXcwkw?si=SeeakVCVpqWatOUl) clip from the Lex Fridman Podcast for influencing the v4 runbook template's Carmack-style reliability controls — debugger-first inspection, mandatory static analysis, assertion-driven invariants, bounded resource design, and "make invalid states unrepresentable".
- Martin Fowler's writing on software architecture, refactoring, and AI-assisted engineering for shaping the runbook disciplines around reversibility, exemplar code, true behavior-preserving refactoring, and explicit AI tolerance contracts.
- The [oneNDA](https://www.onenda.org/) consortium for the canonical UK NDA template the biz-pack `/slo-legal draft nda` flow points users to. Licensed under [CC BY-ND 4.0](https://creativecommons.org/licenses/by-nd/4.0/); the canonical `.docx` is fetched manually by the user from onenda.org and is never copied, modified, rendered, or redistributed by this repo. The skill produces only a separate Markdown cover artifact for company-specific fields — assembly happens on the user's machine, against the user's locally-downloaded canonical `.docx`.
- The [SeedLegals](https://seedlegals.com/) public pricing page as the v1 cost baseline anchor for biz-pack ROI claims, alongside JPP Law's fixed-fee public pricing.
- Garry Tan's [gstack](https://github.com/garrytan/gstack) for the skill-pack-as-workflow pattern that shaped the overall structure of SunLit Orchestra's `/slo-*` skills — the idea that a stage-aware collection of opinionated skills can carry a project from ideation to ship more reliably than ad-hoc prompting.
- OpenAI's [Symphony](https://github.com/openai/symphony) for influencing how SunLit Orchestra thinks about multi-agent orchestration and the seams between specialist roles in a development workflow.
- OWASP's [Secure Agent Playbook](https://github.com/OWASP/secure-agent-playbook) for clear README patterns around playbooks, skill catalogues, example outputs, and standards traceability. SunLit Orchestra borrows those presentation ideas for a runbook-driven delivery workflow rather than a standalone security assessment playbook.
- Sam Stepanyan and the [OWASP Nettacker](https://github.com/OWASP/Nettacker) project for the authorized assessment workflow surface that `/slo-nettacker` wraps with scope, rate-limit, evidence, and confidentiality gates.
