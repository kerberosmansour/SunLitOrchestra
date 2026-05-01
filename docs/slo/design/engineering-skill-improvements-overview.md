---
name: engineering-skill-improvements
created: 2026-04-27
status: design lock-in
tla_required: false
security_libs_required: false
ai_component: false
compliance: [soc2, asvs]
---

# Design overview — engineering skill improvements

## System goal

Close the structural gaps the 2026-04-27 skill-pack review identified across the engineering-side skills: (a) decompose the three monolithic SKILL.md files (`/slo-sast`, `/slo-tla`, `/slo-plan`) into thin orchestrators + per-stage methodology references; (b) seed a `references/templates/` shared library so cross-skill drift on common patterns is minimized; (c) hard-enforce `/slo-freeze` and (optionally) `/slo-execute` allow-list via `.claude/settings.json` PreToolUse hook; (d) add per-skill `evals/` directories with the seven canonical case shapes; (e) gate every security-engineering claim through a research-validation source hierarchy.

## Stack decision

Existing project stack — no changes:

- Markdown for SKILL.md, references, methodology docs, eval cases
- Rust (`crates/sldo-install`) for structural-contract tests + `cargo test --workspace` baseline
- `.claude/settings.json` PreToolUse hook (uses `update-config` skill for mutation)
- `gh` CLI for any future issue filing (out-of-scope for this runbook; that's R1)

No new runtime dependencies. No new crates. The PreToolUse hook is a small shell script invoked by Claude Code; argv-list discipline applies if it shells out.

## Components

| Component | Responsibility | Milestone introduced/changed | Key interfaces |
|---|---|---|---|
| `references/templates/` | Shared library: intake-checklist, restate-and-confirm, citation-discipline, tool-safety-section, output-frontmatter, escalation, eval-cases, heuristic-numbers-discipline | M1 | Cited from every SKILL.md update |
| `references/templates/citation-discipline.md` | Source hierarchy + research-validation rule for security-engineering claims | M1 | Cited by `/slo-sast`, `/slo-tla`, `/slo-rulegen`, `/slo-verify`, `/slo-research` |
| `skills/slo-sast/SKILL.md` | Reduced to ~100 lines (pre-flight + dispatch + anti-patterns common across milestones) | M2 | Symlink-installed unchanged |
| `skills/slo-sast/references/methodology-m1-parser.md` ... `m5-pr-creation.md` | Per-stage methodology | M2 | Cited from SKILL.md by relative path |
| `skills/slo-tla/SKILL.md` | Reduced to ~100 lines (prereq + suitability gate + handoff) | M3 | Symlink-installed unchanged |
| `skills/slo-tla/references/methodology-*.md` | Per-stage methodology (elicitation, abstraction, counterexample, verified-design) | M3 | Cited from SKILL.md |
| `skills/slo-tla/tools.toml` | Apalache version + SHA pin added | M3 | Read by SKILL.md prereq cascade |
| `skills/slo-plan/references/methodology-milestone-authoring.md` | Per-milestone authoring sub-procedure extracted | M4 | Cited from SKILL.md |
| `crates/sldo-install/tests/<test>` | Soft line-cap structural-contract test | M4 | Asserts every SKILL.md ≤ 200 lines OR carries `# soft-cap-exception:` |
| `skills/<skill>/evals/<case>.md` | Per-skill eval cases (7 categories) | M5 | Documented expectations; runtime harness consumes later |
| `.claude/settings.json` PreToolUse hook | Hard-enforces `/slo-freeze` scope lock | M5 | Reuses `update-config` skill for mutation |

## Data flow

```
SKILL invocation
  ┌──────────────────────────────────────┐
  │ SKILL.md read (≤ 200 lines, lean)    │
  │   - pre-flight                        │
  │   - mode dispatch                     │
  │   - anti-patterns                     │
  └────────────┬──────────────────────────┘
               │ if methodology needed:
               ▼
    ┌──────────────────────────┐
    │ references/methodology-* │  per-stage detail loaded on demand
    └──────────────────────────┘
               │ if claim needs source-of-truth:
               ▼
    ┌──────────────────────────┐
    │ references/templates/    │  shared discipline (intake, citation, etc.)
    │   citation-discipline    │
    │   intake-checklist       │
    │   ...                    │
    └──────────────────────────┘
               │ at write time:
               ▼
    ┌──────────────────────────┐
    │ .claude/settings.json    │  PreToolUse hook: enforce /slo-freeze scope
    │   PreToolUse hook        │
    └──────────────────────────┘
```

## Trust boundaries

- SKILL.md prose is repo-tracked; install symlinks under `~/.claude/skills/<name>/` are the only runtime surface. Decomposition doesn't change this trust boundary.
- The PreToolUse hook is local; it reads session state from `~/.sldo/freeze-scope.txt` (gitignored). Compromise of the local user account compromises both the hook and the SKILL.md prose simultaneously — same trust class.
- Eval cases are documented expectations only; no runtime trust boundary in this milestone.

## Interfaces locked

| Interface | Stability | Notes |
|---|---|---|
| `references/templates/<name>.md` file paths | `stable` | Cited from SKILL.md across multiple skills |
| `skills/<skill>/references/methodology-*.md` per-skill convention | `stable` | Sibling-to-SKILL.md location is the install-compatible pattern |
| `evals/<case>.md` file shape (frontmatter: skill, case-name, category, expected-behavior; body: input + expected) | `stable` | Forward-compatible with future runtime harness |
| `# soft-cap-exception: <reason>` SKILL.md frontmatter pragma | `stable` | Read by structural-contract test |
| Source hierarchy in `references/templates/citation-discipline.md` | `stable` | Six-tier; lock-in required to prevent drift |
| `.claude/settings.json` PreToolUse hook contract | `evolving` | Project-level; per-project freeze scope |

## TLA+ section

Not required (`tla_required: false`). No concurrent actors, no distributed state. Decomposition is purely textual refactoring; PreToolUse hook is single-session, single-actor.

## STRIDE sweep (per Step 3.5)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | EoP |
|---|---|---|---|---|---|---|
| SKILL.md decomposition | N/A | mitigated — git-tracked, install symlinks SHA-verified by `sldo-install verify` (deferred follow-up) | N/A — git history is the audit trail | N/A — public skill content | N/A | N/A |
| `references/templates/` shared library | N/A | mitigated — git-tracked | N/A | N/A | N/A | N/A |
| PreToolUse hook for `/slo-freeze` | residual — local user account compromise impacts hook integrity (same trust class as the SKILL.md itself) | mitigated — `update-config` skill is the canonical mutation surface; argv-list if hook shells out | residual — local hook execution is not centrally logged; mitigated by Claude Code's own tool-use telemetry | N/A — hook reads session state, doesn't emit | mitigated — hook execution is bounded per tool-call | mitigated — hook is execution-control, not privilege-grant |
| Per-skill `evals/` | N/A | mitigated — git-tracked | N/A — eval content is documentation | N/A | N/A | N/A |

New abuse cases:

- `tm-eng-skill-improvements-abuse-1: SKILL.md decomposition breaks install-symlink chain` — class eliminated by structural-contract test that asserts every decomposed SKILL.md still references its methodology files (cargo test --workspace is the baseline gate).
- `tm-eng-skill-improvements-abuse-2: PreToolUse hook bypassed via session-state file deletion` — residual; documented as "this is not a security boundary" disclaimer in `/slo-freeze` SKILL.md description; the hook is a discipline-enforcer, not a security primitive.
- `tm-eng-skill-improvements-abuse-3: Soft line-cap exception abused to bypass decomposition` — mitigated by requiring `# soft-cap-exception: <reason>` to be reviewable in the structural-contract test output (CI flags exceptions for human review).

## Compatibility commitments

- Every decomposed SKILL.md still passes `cargo test --workspace` install-symlink tests.
- Every existing `references/sast/`, `references/biz/`, etc., reference file is preserved unchanged (decomposition adds new `methodology-*` files; doesn't replace existing per-stage references).
- The PreToolUse hook is opt-in (per-project `.claude/settings.json`); existing repos without it continue to operate with prose-level discipline.
- The soft line-cap structural-contract test is opt-in via runbook authoring (the test asserts it; existing SKILL.md files that exceed the cap get the `# soft-cap-exception:` pragma in M4 as part of the migration).

## Out-of-scope

- Building the runtime Claude Code harness for executable evals (deferred-follow-up in [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)).
- Substantive changes to skill behavior beyond decomposition + polish.
- Extending the `update-config` skill (use as-is).
- Moving any work currently in [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4) (`/slo-sec-libs` Phase 4) into this runbook.

## Research-validation discipline (load-bearing)

Every claim in this runbook that touches security-engineering content must be source-verified before SKILL.md edits land:

| Claim category | Primary source | Acceptance |
|---|---|---|
| Semgrep CLI behavior at pinned version | https://semgrep.dev/docs/ at `MIN-SEMGREP-VERSION.md` pin | quote + `last_checked:` |
| `cargo audit` exit-code semantics | https://github.com/rustsec/rustsec/tree/main/cargo-audit at pinned commit | quote + commit SHA |
| TLA+ behavior + Apalache pinning | https://lamport.azurewebsites.net/tla/tools.html + https://github.com/apalache-mc/apalache/releases at pinned version | quote + version + SHA-256 |
| GitHub Actions / `pull_request_target` ban | GitHub Docs at https://docs.github.com/en/actions + Trail of Bits' `https://github.com/trailofbits/audit-action` rationale | quote + retrieval date |
| OWASP Proactive Controls v3 categories | OWASP project at https://owasp.org/www-project-proactive-controls/ | quote + retrieval date |
| ASVS chapters | OWASP ASVS at https://owasp.org/www-project-application-security-verification-standard/ | quote + version + retrieval date |

**Bright-line**: unverifiable claims removed, not weakened. The bar is set in [`references/templates/citation-discipline.md`](../../references/templates/citation-discipline.md) (M1).
