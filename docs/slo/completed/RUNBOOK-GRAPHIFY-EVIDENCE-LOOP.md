# Graphify Evidence Loop — SunLit Orchestra (AI-First Runbook v4)

> **Purpose**: Add Graphify-backed investigation to SunLit Orchestra so engineers
> and AI coding agents can use one graph for knowledge discovery, fast
> troubleshooting, security triage, and QA-risk discovery.
> **Audience**: AI coding agents first, humans second.
> **Privacy rule**: private repo raw evidence never lands in git. Only
> anonymized summaries are tracked.
> **Prerequisite reading**: [docs/LOOPS-ENGINEERING.md](../../LOOPS-ENGINEERING.md),
> [docs/SECURE-VALUE-LOOP.md](../../SECURE-VALUE-LOOP.md),
> [skills/slo-graphify/SKILL.md](../../../skills/slo-graphify/SKILL.md),
> [references/graphify/provider-evidence-loop.md](../../../references/graphify/provider-evidence-loop.md).

---

## 1. Runbook Metadata

| Field | Value |
|---|---|
| Runbook ID | `graphify-evidence-loop` |
| Project name | `SunLit Orchestra` |
| Primary stack | Rust CLI + Markdown skills/references |
| Primary package/app names | `sldo-install`, `slo-graphify` |
| Prefix for tests and lesson files | `graphify-evidence` |
| Default unit test command | `cargo test -p sldo-install graphify` |
| Default integration/BDD test command | `cargo test -p sldo-install --test e2e_graphify_install` |
| Default E2E/runtime validation command | `sldo-install graphify --install-plan && sldo-install graphify` |
| Default build/boot command | `cargo build -p sldo-install` |
| Default formatter command | `cargo fmt --check` |
| Default static analysis / lint command | `cargo test -p sldo-install graphify` |
| Default dependency / security audit command | `n/a - no new dependency required` |
| Public interfaces stable by default | `yes` |

Public interfaces added:

- `/slo-graphify`
- `sldo-install graphify`
- `sldo-install graphify --install-plan`
- `references/graphify/provider-evidence-loop.md`

---

## 2. Milestone Tracker

| # | Milestone | Status | Started | Completed | Evidence |
|---|---|---|---|---|---|
| 1 | Product contract and confidentiality boundary | `done` | 2026-06-21 | 2026-06-21 | `/slo-graphify` contract and provider reference |
| 2 | Graphify installer readiness path | `done` | 2026-06-21 | 2026-06-21 | `sldo-install graphify` CLI + E2E tests |
| 3 | Real-provider value proof | `done` | 2026-06-21 | 2026-06-21 | anonymized trial metrics and limitations |
| 4 | Catalog, docs, and PR handoff | `done` | 2026-06-21 | 2026-06-21 | catalog count, README, targeted test commands |

---

## 3. End-to-End Architecture

```text
GitHub Issues / user question
        |
        v
 /slo-graphify
        |
        +--> Graphify graph -----------------------+
        |     syntax files/imports/calls/routes    |
        |                                          v
        +--> semantic providers ------------> normalized graph facts
        |     rust-analyzer, TypeScript LS          |
        |                                          v
        +--> analysis providers ------------> finding facts
              OpenGrep, Semgrep-compatible rules   |
                                                   v
                                      graph-backed query/readout
                                                   |
                                                   v
                       security ledger + troubleshooting map + QA tests

Trust boundary: private target repo raw evidence stays in ignored experiment
paths. Only anonymized summaries, counts, generic classes, and next actions are
tracked in SunLit Orchestra.
```

## 4. Outcome Validation Contract

| Outcome | Validation |
|---|---|
| An engineer can use the skill for non-security work | Skill and reference include knowledge and troubleshooting lanes, not only security. |
| An AI coding agent can reduce repo wandering | Reference starts from GitHub Issues and asks graph-first "which files should I open?" questions. |
| Private repo evidence is protected | Skill has a hard confidentiality gate and stop condition before scanning. |
| Real provider path is honest | Skill blocks claims based on stub semantic providers. |
| Graphify setup is easier | Installer prints host-aware Graphify commands and checks CLI/source readiness without requiring a `skills/` dir. |
| Security remains useful but bounded | Analysis findings map to CWE, source ranges, tests, ledger disposition, and human-review buckets. |

---

## 5. What We Learned

| Lane | What the experiment found | Why it matters |
|---|---|---|
| Knowledge | GitHub Issues can be bucketed into graph questions before opening files. An anonymized trial produced 123 knowledge signals. | The graph gives both the engineer and the agent a better first map, so they spend less time opening unrelated files. |
| Troubleshooting | The same graph can connect issue themes to workers, routes, retries, error paths, and cross-language boundaries. The trial produced 131 troubleshooting signals. | It turns "where do I even start?" into a ranked set of likely paths and tests. |
| Security | OpenGrep/Semgrep-compatible findings can be imported as analysis-provider facts and joined to semantic code paths. The trial found 35 warning-level findings and no critical/high findings in the bounded scan. | Security findings become actionable work items instead of a disconnected scanner report. |
| QA risk | Panic/error handling, integer arithmetic, input validation, retry behavior, and swallowed error paths show up as normal reliability bugs too. | This is where the product becomes broader than AppSec: it helps prevent user-visible failures. |
| AI-agent value | The evidence loop records provider kind, confidence, source ranges, and graph queries. | Agents can cite the graph instead of guessing from partial reads. |

Important caveat: the proof that matters is the real rust-analyzer and
TypeScript language-service provider path. Stub providers are useful for
fixture tests only; they are not acceptable for promotion claims.

---

## 6. Milestone Contracts

### M1 - Product Contract and Confidentiality Boundary

**Goal**: Add `/slo-graphify` as a first-class SLO skill for knowledge,
troubleshooting, security, and QA-risk work.

**Files allowed**:

- `skills/slo-graphify/SKILL.md`
- `skills/slo-graphify/README.md`
- `references/graphify/provider-evidence-loop.md`

**Definition of done**:

- The skill names all three product lanes.
- The confidentiality gate blocks raw private evidence from git.
- The method requires real provider labeling and warns against stub claims.

**Evidence**:

- Skill contract added.
- Provider reference added.
- No private repo name or raw private path is included.

### M2 - Graphify Installer Readiness Path

**Goal**: Make `sldo-install` help users validate or plan Graphify setup on
macOS, Linux, and Windows.

**Files allowed**:

- `crates/sldo-install/src/main.rs`
- `crates/sldo-install/src/graphify.rs`
- `crates/sldo-install/tests/e2e_graphify_install.rs`
- `crates/sldo-install/README.md`

**Definition of done**:

- `sldo-install graphify --install-plan` works without a `skills/` directory.
- `sldo-install graphify` detects a Graphify CLI on `PATH`.
- `sldo-install graphify --graphify-path <DIR>` accepts a local source checkout.
- Missing Graphify fails closed with actionable install guidance.

**Evidence**:

- E2E tests cover missing, CLI-present, source-checkout, and install-plan paths.
- Install plan is host-aware for Claude Code, Codex, and GitHub Copilot.

### M3 - Real-Provider Value Proof

**Goal**: Encode the learning from the experiment without leaking the private
target.

**Files allowed**:

- `references/graphify/provider-evidence-loop.md`
- `docs/slo/completed/RUNBOOK-GRAPHIFY-EVIDENCE-LOOP.md`

**Definition of done**:

- Anonymized metrics are allowed.
- Private repo names, absolute paths, raw scanner output, issue bodies, and code
  snippets are forbidden.
- Security findings are reported with severity posture and limitations.
- QA-risk classes are explicitly named.

**Evidence**:

- Anonymized calibration table records provider facts, graph size, issue signals,
  and finding classes.
- The bounded scan reports no critical/high finding claim.

### M4 - Catalog, Docs, and PR Handoff

**Goal**: Make the new capability discoverable and shippable.

**Files allowed**:

- `docs/skill-pack-catalog.md`
- `skills/README.md`
- this runbook

**Definition of done**:

- Catalog count is correct.
- `/slo-graphify` appears under power tools.
- Installer README and skill README explain the readiness path.
- Targeted Rust tests and formatter pass before PR.

**Evidence**:

- `cargo test -p sldo-install graphify`
- `cargo test -p sldo-install --test e2e_graphify_install`
- `cargo fmt --check`

---

## 7. Follow-Up

The next run should be a private beta on one or two real repos with strict
redaction. The success bar is not "found more findings." The success bar is:

| Question | Pass signal |
|---|---|
| Did it reduce file wandering? | Fewer files opened and fewer wrong assumptions for engineer and agent tasks. |
| Did it help with active GitHub Issues? | Issues map to useful graph queries and next tests. |
| Did security findings become work? | Every finding has fix, issue, human review, accepted risk, or false-positive disposition. |
| Did it catch QA bugs waiting to happen? | At least one regression test or issue is produced from reliability-style graph evidence. |
| Did confidentiality hold? | No private identifiers or raw evidence in tracked files. |
