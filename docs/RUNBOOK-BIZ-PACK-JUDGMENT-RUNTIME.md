# Biz-pack judgment runtime harness — SunLitOrchestrate (AI-First Runbook v3)

> **Forward-looking note (agent-host M4, 2026-04-30)**: this runbook describes
> a Claude-only live runtime harness. Agent-host milestone 4 renamed the
> helper module from `tests/common/judgment_runtime.rs` to
> `tests/common/claude_runtime.rs` so the source tree itself signals the
> Claude-only boundary. Behavior, env vars (`BIZ_JUDGMENT_RUNTIME_*`), and
> public test entrypoints are unchanged. There is no host-neutral equivalent
> at HEAD; do not introduce a generic agent-runtime trait without a second
> real implementation. Historical references below to `judgment_runtime.rs`
> still describe the same code, now under the renamed file.

> **Purpose**: Replace the `#[ignore]` runtime stub at `crates/sldo-install/tests/e2e_biz_followup_m4.rs::runtime_harness_invokes_claude_cli_per_fixture` with a real harness that walks `references/biz/judgment-fixtures/<skill>/*.md`, invokes the target advisor skill via `claude -p`, and asserts the output artifact's frontmatter matches each fixture's declared expectations.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work the two milestones sequentially. M1 proves the harness against a single non-adversarial fixture. M2 wires all 9 fixtures + retry policy + cost cap.
> **Prerequisite reading**: [ARCHITECTURE.md](../docs/ARCHITECTURE.md), [references/biz/judgment-fixtures/README.md](../references/biz/judgment-fixtures/README.md), [crates/sldo-install/tests/e2e_biz_followup_m4.rs](../crates/sldo-install/tests/e2e_biz_followup_m4.rs).

---

## Runbook Metadata

- **Runbook ID**: `biz-pack-judgment-runtime`
- **Prefix for test files and lessons files**: `biz-judgment-runtime`
- **Primary stack**: Rust (workspace crates only — no new languages)
- **Primary package/app names**: `sldo-install` (test crate)
- **Default test commands**:
  - Baseline: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify`
  - Structural-only (this runbook's tests): `cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1 --test e2e_biz_judgment_runtime_m2`
  - Runtime (slow, costs API calls): `BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m2 -- --ignored`
  - Build/boot: `cargo build -p sldo-install`
- **Allowed new dependencies by default**: `none` — stdlib + existing workspace deps (`tempfile` already present).
- **Schema/config migration allowed by default**: `no`.
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `references/biz/judgment-fixtures/<skill>/*.md` frontmatter schema (target_skill, target_mode, expected_gates_fired, must_refuse, must_route_to, fixture_class, adversarial, critique_provenance) — do NOT rename keys.
  - The 4 hard-block predicate IDs in `references/biz/triage-gate.md` (gate-1-regulated, gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper, gate-4-gdpr-document).
  - Artifact frontmatter contract from `references/biz/artifact-schema.md` (gates_fired, triage_gate_passed, tier, archetype, etc.).
  - The existing structural tests in `crates/sldo-install/tests/e2e_biz_followup_m4.rs` (1–5) — must remain green after the runtime stub is replaced.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | Harness scaffolding + 1 fixture green | `done` | 2026-04-25 | 2026-04-25 | `docs/lessons/biz-judgment-runtime-m1.md` | `docs/completion/biz-judgment-runtime-m1.md` |
| 2 | Wire all 9 fixtures + retry/cost-cap + docs | `done` | 2026-04-25 | 2026-04-25 | `docs/lessons/biz-judgment-runtime-m2.md` | `docs/completion/biz-judgment-runtime-m2.md` |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/biz-judgment-runtime-m<N>.md -->
<!-- Completion summaries go in docs/completion/biz-judgment-runtime-m<N>.md -->

---

## End-to-End Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│  Test process (cargo test -p sldo-install --test e2e_biz_judgment_…)     │
│                                                                          │
│  ┌─────────────────────────┐                                             │
│  │ Walker — fixtures dir   │  references/biz/judgment-fixtures/<skill>/  │
│  └────────────┬────────────┘                                             │
│               │ 1. parse frontmatter + body "## Founder prompt"          │
│               ▼                                                          │
│  ┌─────────────────────────┐                                             │
│  │ Tempdir builder         │ - - -▶  <tempdir>/.claude/skills/<n> -> repo│
│  └────────────┬────────────┘ - - -▶  <tempdir>/references/biz -> repo    │
│               │                                                          │
│               ▼                                                          │
│  ┌─────────────────────────┐  spawn `claude -p <prompt> --add-dir …      │
│  │ Claude-CLI invoker      │ ═══════════════════════════════════════▶  ┌─┐│
│  └────────────┬────────────┘    --output-format json                   │ ││
│               │                  --max-budget-usd 0.50                 │A││
│               │                  --bare                                │P││
│               │                  --dangerously-skip-permissions        │I││
│               │                                                        └─┘│
│               ▼                                                          │
│  ┌─────────────────────────┐                                             │
│  │ Artifact discoverer     │  walk <tempdir>/docs/biz/ +                 │
│  │ + frontmatter parser    │  <tempdir>/docs/biz-public/                 │
│  └────────────┬────────────┘                                             │
│               │ 2. compare gates_fired / triage_gate_passed / tier       │
│               ▼                                                          │
│  ┌─────────────────────────┐                                             │
│  │ Assertion + flake retry │  retry up to 2× on transient errors         │
│  └─────────────────────────┘                                             │
│                                                                          │
│  Legend:  ─── new   - - - symlinks   ═══ subprocess   ▶ data flow        │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `JudgmentFixture` parser | Read a fixture .md, extract frontmatter + founder prompt body | M1 | private struct in test crate |
| `TempRepo` builder | Stand up a tempdir with symlinked skills + references/biz, set `HOME` to a sandbox | M1 | private helper in test crate |
| `ClaudeCli` invoker | Spawn `claude -p …`, capture JSON, enforce timeout + budget | M1 | shells out to `claude` binary (must be on PATH) |
| `ArtifactDiscoverer` | Walk tempdir's `docs/biz/` + `docs/biz-public/`, parse frontmatter | M1 | private helper |
| `RetryPolicy` | Up to 2 retries on transient errors (network, timeout, budget refresh) | M2 | configurable via env |
| Documentation | Update `judgment-fixtures/README.md` to point at the now-real harness | M2 | doc-only |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Fixture → harness | `references/biz/judgment-fixtures/` | test process | std::fs read | M1 |
| Harness → claude | test process | `claude -p` subprocess | Command::spawn + stdin/stdout | M1 |
| claude → artifact | LLM | `<tempdir>/docs/biz/<area>/<artifact>.md` | claude's Write tool | M1 |
| Artifact → assertion | tempdir | test process | std::fs walk + frontmatter parse | M1 |

---

## High-Level Design for Formal Verification (TLA+ Section)

`tla_required: false` — this is a deterministic test harness with no concurrent state. The single source of non-determinism (LLM judgment) is exactly what the tests probe, not state to verify.

---

## Milestones

### M1 — Harness scaffolding + 1 fixture green

#### Goal
End-to-end harness exists and passes against `references/biz/judgment-fixtures/slo-legal/ir35-genuine-contractor.md` (a `must_refuse: false` non-adversarial fixture chosen because it should produce an artifact and exercise the full happy path: parse → invoke claude → discover artifact → assert frontmatter).

#### Context
Today the harness is a panic stub at [crates/sldo-install/tests/e2e_biz_followup_m4.rs:184-209](../crates/sldo-install/tests/e2e_biz_followup_m4.rs#L184-L209). The stub documents the desired behavior in body comments; this milestone implements it minimally.

#### Important design rule
The harness MUST be `#[ignore]`-gated AND env-flag-gated (`BIZ_JUDGMENT_RUNTIME_LIVE=1`). Running `cargo test -p sldo-install` without the env flag must NOT invoke `claude` — only structural tests run by default. Reason: the harness costs real money and requires network + an Anthropic API key.

#### Refactor budget
Tight — only files under `crates/sldo-install/tests/`. No changes to other crates, no new workspace deps. Helpers may live as a new `mod` inside the new test file or as `crates/sldo-install/tests/common/judgment_runtime.rs` (preferred; reused in M2).

#### Contract Block

| Item | Value |
|---|---|
| **Inputs** | `references/biz/judgment-fixtures/slo-legal/ir35-genuine-contractor.md`; the `claude` binary on PATH (or `BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN` env override); the user's existing Anthropic credentials |
| **Outputs** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs`; `crates/sldo-install/tests/common/judgment_runtime.rs`; `crates/sldo-install/tests/common/mod.rs` |
| **Interfaces touched** | None public. The test file is new. |
| **Files allowed to change** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs` (new); `crates/sldo-install/tests/common/judgment_runtime.rs` (new); `crates/sldo-install/tests/common/mod.rs` (new); `docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` (this file, tracker only) |
| **Files to read before changing** | `crates/sldo-install/tests/e2e_biz_followup_m4.rs`, `references/biz/judgment-fixtures/README.md`, `references/biz/judgment-fixtures/slo-legal/ir35-genuine-contractor.md`, `references/biz/artifact-schema.md`, `references/biz/triage-gate.md`, `crates/sldo-install/Cargo.toml` |
| **New files allowed** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs`, `crates/sldo-install/tests/common/{mod.rs,judgment_runtime.rs}` |
| **New dependencies allowed** | None — stdlib + `tempfile` (already a `dev-dependencies` entry of `sldo-install`) |
| **Migration allowed** | No |
| **Compatibility commitments** | The 5 existing structural tests in `e2e_biz_followup_m4.rs` (`judgment_fixtures_directory_layout_correct`, `all_fixtures_have_required_frontmatter`, `all_fixtures_have_valid_enum_values`, `critical_fixture_classes_seeded_in_v1_set`, `tax_efficiency_pushback_fixture_present`) MUST remain green and untouched. The fixture frontmatter schema MUST NOT change. |
| **Forbidden shortcuts** | (1) Don't add a regex / yaml crate "for convenience" — keep stdlib-only parsing as the prior follow-ups did. (2) Don't run `claude` without the env flag — that breaks anyone running `cargo test`. (3) Don't hard-code an Anthropic API key. (4) Don't bypass `--max-budget-usd` "for one quick run". (5) Don't shell out to `sldo-install` — symlink the skills directory directly via `std::os::unix::fs::symlink`. |
| **Data classification** | `Internal` — fixture prompts are public-by-design; tempdir artifacts are throwaway; no real persons or deal data flows through this harness. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C5 (Validate All Inputs)**: parse fixture frontmatter strictly, reject malformed; **C8 (Protect Data Everywhere)**: tempdir wiped at end of each fixture so an artifact written by claude under one fixture's prompt cannot leak into another's; **C9 (Implement Security Logging and Monitoring)**: capture claude's stderr + JSON response in test failure messages so flakes are diagnosable. |
| **Abuse acceptance scenarios** | This milestone introduces a NEW surface (subprocess invocation of `claude` with attacker-controlled-shaped fixture prompts). Three abuse cases — see BDD section below: (a) malicious fixture path traversal (`../../../etc/passwd`); (b) fixture prompt that tries to escape the cwd via a shell metacharacter; (c) `claude` exits with non-zero AND no artifact written (must error loudly, not silently pass). All three are covered by abuse-case BDD rows. Threat-model citation: `tm-biz-skill-pack-abuse-3` (subprocess invocation of trusted tool with semi-trusted input). |

#### Out of Scope / Must Not Do
- Wiring all 9 fixtures (M2).
- Retry policy (M2).
- Documentation updates beyond the tracker (M2).
- Replacing the existing `runtime_harness_invokes_claude_cli_per_fixture` panic stub. **Leave that stub untouched in M1.** The new harness lives in a separate test file; the stub is removed in M2 once the new harness has proven itself across all fixtures.

#### Files Allowed to Change

| File | Action | Notes |
|---|---|---|
| `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs` | new | Single test: `runtime_harness_green_on_ir35_genuine_contractor` (#[ignore], env-gated) |
| `crates/sldo-install/tests/common/mod.rs` | new | `pub mod judgment_runtime;` only |
| `crates/sldo-install/tests/common/judgment_runtime.rs` | new | Helpers: `JudgmentFixture::parse`, `TempRepo::build`, `ClaudeCli::invoke`, `ArtifactDiscoverer::find` |
| `docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` | edit (tracker only) | Mark M1 in_progress / done |

#### Step-by-Step

1. Read the existing stub at [e2e_biz_followup_m4.rs:184-209](../crates/sldo-install/tests/e2e_biz_followup_m4.rs#L184-L209) and the [judgment-fixtures README](../references/biz/judgment-fixtures/README.md). Confirm the frontmatter contract is unchanged.
2. Create `crates/sldo-install/tests/common/mod.rs` (one line: `pub mod judgment_runtime;`).
3. Create `crates/sldo-install/tests/common/judgment_runtime.rs` with:
   - `pub struct JudgmentFixture { pub frontmatter: FixtureFrontmatter, pub founder_prompt: String, pub source_path: PathBuf }`
   - `pub struct FixtureFrontmatter { name, target_skill, target_mode, expected_gates_fired: Vec<String>, must_refuse: bool, must_route_to: String, fixture_class: String, adversarial: bool, critique_provenance: String }`
   - `impl JudgmentFixture { pub fn parse(path: &Path) -> Result<Self, String> }`
   - `pub struct TempRepo { pub root: TempDir, pub home: PathBuf }`
   - `impl TempRepo { pub fn build(repo_root: &Path) -> Result<Self, String> }` — symlinks `skills/`, `references/biz/`, `CLAUDE.md`; creates an isolated `<root>/home/.claude/` (so `HOME=<root>/home` doesn't write to user's real `~/.claude/`).
   - `pub struct ClaudeOutput { pub stdout: String, pub stderr: String, pub exit_status: ExitStatus }`
   - `pub fn invoke_claude(temp_repo: &TempRepo, founder_prompt: &str, max_budget_usd: f64, timeout: Duration) -> Result<ClaudeOutput, String>` — spawns `claude -p <prompt> --add-dir <temp_repo.root> --output-format json --max-budget-usd … --bare --dangerously-skip-permissions` with `HOME=<temp_repo.home>`, `cwd=<temp_repo.root>`.
   - `pub struct DiscoveredArtifact { pub path: PathBuf, pub frontmatter: BTreeMap<String, String> }`
   - `pub fn discover_artifact(temp_repo: &TempRepo) -> Result<Option<DiscoveredArtifact>, String>` — walks `<root>/docs/biz/` and `<root>/docs/biz-public/`, returns the single most recent .md (or `None` if no artifact written, or `Err` if multiple — multiple is unexpected for a single-fixture run).
4. Frontmatter parsing: stdlib-only. Find the `---\n…\n---\n` block at the top, split by lines, parse `key: value` pairs (no nested yaml needed — fixture schema is flat). For `expected_gates_fired:` parse the bracketed list `[a, b]` by stripping `[`/`]` and splitting on `,`.
5. Founder prompt extraction: find the `## Founder prompt` heading; the prompt is the body until the next `##` heading. Strip leading `>` blockquote markers if present.
6. Tempdir setup: `<root>/.claude/skills/<each-skill>` → symlink to `<repo>/skills/<each-skill>`; `<root>/references/biz` → symlink to `<repo>/references/biz`; `<root>/CLAUDE.md` → symlink to `<repo>/CLAUDE.md`; `<root>/home/.claude/` created empty so `HOME` redirection is clean.
7. Implement `invoke_claude`: build `Command::new("claude")` with the flags above; capture stdout + stderr; enforce timeout via `wait_timeout` crate? — NO, that's a new dep. Use a thread + `try_wait` poll loop with a sleep, or just trust `--max-budget-usd` to bound runtime (claude self-terminates near budget). Pick the latter for stdlib-only.
8. Implement `discover_artifact`: walk `<root>/docs/biz` and `<root>/docs/biz-public` recursively (stdlib `read_dir` recursion); collect `.md` files; if zero, return `None`; if one, parse its frontmatter; if more than one, return `Err("multiple artifacts written; harness expects single artifact per fixture invocation")`.
9. Create `crates/sldo-install/tests/e2e_biz_judgment_runtime_m1.rs` with the single test:
   - `#[test] #[ignore] fn runtime_harness_green_on_ir35_genuine_contractor()`
   - Skip with a clear message if `BIZ_JUDGMENT_RUNTIME_LIVE` is not `"1"`.
   - Skip with a clear message if `claude --version` fails (binary not on PATH).
   - Parse the fixture, build temp repo, invoke claude (budget = 0.50 USD), discover artifact, assert: artifact exists; `gates_fired:` matches fixture's `expected_gates_fired: []` (empty); `tier: confidential`; `triage_gate_passed: true`.
10. Run: `cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1` (compiles, finds 0 not-ignored tests, exits 0). Then `BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1 -- --ignored` to actually run the live test once for confirmation.

#### BDD Acceptance Scenarios

| # | Category | Given | When | Then |
|---|---|---|---|---|
| 1 | happy path | the ir35-genuine-contractor fixture, `BIZ_JUDGMENT_RUNTIME_LIVE=1`, `claude` on PATH | the test runs | a `<tempdir>/docs/biz/legal/contractor-sow-*.md` is written, its `gates_fired:` is empty, `triage_gate_passed: true`, `tier: confidential` |
| 2 | invalid input — fixture | a fixture file with malformed frontmatter (test fixture in `tests/common/judgment_runtime.rs` unit-tests, NOT in the real fixture set) | `JudgmentFixture::parse` is called | returns `Err("…")` with a clear diagnostic; never panics |
| 3 | empty state | `BIZ_JUDGMENT_RUNTIME_LIVE` unset | the test runs | the test prints "skipped — set BIZ_JUDGMENT_RUNTIME_LIVE=1" and returns early without invoking claude |
| 4 | dependency failure | `claude` binary not on PATH | the test runs with `BIZ_JUDGMENT_RUNTIME_LIVE=1` | the test errors with a clear message naming the env var override (`BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN`) |
| 5 | abuse case — path traversal | a fixture path containing `..` segments | `JudgmentFixture::parse` is called with that path | parse fails fast (the path is not under `references/biz/judgment-fixtures/`) — citation `tm-biz-skill-pack-abuse-3` |
| 6 | abuse case — claude crash | `claude` spawns and exits non-zero with no artifact written | the harness assertion runs | the test fails with a message containing both stdout and stderr from claude — citation `tm-biz-skill-pack-abuse-3` (silent failure is the abuse) |
| 7 | abuse case — multiple artifacts | claude writes 2+ artifacts in one invocation | `discover_artifact` is called | returns `Err("multiple artifacts written…")` — never silently picks one |
| 8 | concurrency | (N/A — M1 is single-fixture-single-run; concurrency is not exercised) | — | — |
| 9 | persistence | the test completes | the tempdir | is dropped (`TempDir::drop` → recursively removed); no real `~/.claude/` is touched |

#### Regression Tests

These existing tests MUST stay green:

- `cargo test -p sldo-install --test e2e_biz_followup_m4` (5 structural tests).
- `cargo test -p sldo-install` (full sldo-install test suite — should compile and pass without the env flag set).
- The full baseline: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify`.

#### Compatibility Checklist

- [ ] `e2e_biz_followup_m4.rs` is **untouched** — both the 5 structural tests AND the panic-stub at line 184-209 still compile and run as before.
- [ ] No changes to fixture files under `references/biz/judgment-fixtures/`.
- [ ] No new entries in `crates/sldo-install/Cargo.toml`'s `[dependencies]` or `[dev-dependencies]`.
- [ ] No changes outside `crates/sldo-install/tests/`.
- [ ] `cargo test -p sldo-install` (no env flag, no `--ignored`) is green.

#### E2E Runtime Validation

Test function: `runtime_harness_green_on_ir35_genuine_contractor` in `tests/e2e_biz_judgment_runtime_m1.rs`.

Pass criteria:

- Without `BIZ_JUDGMENT_RUNTIME_LIVE=1`: test prints "skipped" and exits 0 (counted as ignored, since it's `#[ignore]`).
- With `BIZ_JUDGMENT_RUNTIME_LIVE=1` and `claude` on PATH: test invokes `claude -p` once, finds an artifact at `<tempdir>/docs/biz/legal/`, parses its frontmatter, asserts gates/tier match expectations. Test passes.
- With `BIZ_JUDGMENT_RUNTIME_LIVE=1` and `claude` NOT on PATH: test fails with a diagnostic naming `BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN` env override.

#### Smoke Tests

```
# 1. Default (no API calls)
cargo test -p sldo-install
# expected: all sldo-install tests green; the new test reported as "ignored"

# 2. Live (requires API key + network + ~$0.50 budget)
BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1 -- --ignored
# expected: 1 passed, 0 failed, ~30 sec runtime, ~$0.20-$0.50 USD spend
```

#### Evidence Log

```
M1 evidence:
- baseline command run: <command> → <result>
- structural test run: <command> → <result>
- live run (optional, owner-discretion): <command> → <result>
- artifacts produced: <list of new files>
- artifacts NOT changed (proves compat): <list>
```

#### Definition of Done

- [ ] All steps in Step-by-Step complete.
- [ ] All BDD acceptance scenarios covered (rows 1, 5, 6, 7 by harness logic; row 2 by a unit test in `judgment_runtime.rs`; rows 3, 4 by smoke-test inspection).
- [ ] Compatibility Checklist all checked.
- [ ] Baseline tests green.
- [ ] Lessons file written: `docs/lessons/biz-judgment-runtime-m1.md`.
- [ ] Completion summary written: `docs/completion/biz-judgment-runtime-m1.md`.
- [ ] Tracker updated.

---

### M2 — Wire all 9 fixtures + retry/cost-cap + docs

#### Goal
Harness exercises all 9 v1 fixtures (4 advisor skills × 9 fixtures); retries up to 2× on transient errors; total run is gated by a single global cost cap; the panic stub at `e2e_biz_followup_m4.rs:184-209` is removed and replaced with a forwarder pointing at the new file. Fixture authoring docs (`references/biz/judgment-fixtures/README.md`) are updated to reflect "harness is real, not stub".

#### Context
M1 delivers a single-fixture happy path. M2 generalizes to all fixtures (including adversarial `tax-efficiency-pushback.md` which exercises the `must_refuse: true` branch — claude should refuse to draft, no artifact written, harness asserts the refusal).

#### Important design rule
Adversarial fixtures (`adversarial: true`, `must_refuse: true`) take a **different assertion branch**: instead of asserting an artifact was written with specific gates, the harness asserts **no artifact was written** AND claude's stdout response contains a refusal phrase. The refusal phrase set is a small allowed list (e.g., "I cannot draft", "I will not", "this requires a lawyer", "REFUSED") — exact phrasing varies; the harness checks for any of N phrases (case-insensitive).

#### Refactor budget
Tight — `crates/sldo-install/tests/common/judgment_runtime.rs` (extend), `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` (new), `crates/sldo-install/tests/e2e_biz_followup_m4.rs` (touch the panic stub only — convert it to a forwarder), `references/biz/judgment-fixtures/README.md` (update "Status" section).

#### Contract Block

| Item | Value |
|---|---|
| **Inputs** | All 9 fixtures under `references/biz/judgment-fixtures/<skill>/`; `BIZ_JUDGMENT_RUNTIME_LIVE=1`; optional `BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD` (default 5.00); optional `BIZ_JUDGMENT_RUNTIME_RETRIES` (default 2). |
| **Outputs** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs`; updates to `crates/sldo-install/tests/common/judgment_runtime.rs`; the M4 stub forwarder; README update |
| **Interfaces touched** | The panic-stub function `runtime_harness_invokes_claude_cli_per_fixture` in `e2e_biz_followup_m4.rs` (rewritten to forward; signature kept) |
| **Files allowed to change** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` (new); `crates/sldo-install/tests/common/judgment_runtime.rs` (edit); `crates/sldo-install/tests/e2e_biz_followup_m4.rs` (edit only `runtime_harness_invokes_claude_cli_per_fixture` body); `references/biz/judgment-fixtures/README.md` (edit "Status" + "Runtime harness" sections) |
| **Files to read before changing** | All 9 fixture files; `references/biz/triage-gate.md`; `references/biz/artifact-schema.md` |
| **New files allowed** | `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` |
| **New dependencies allowed** | None |
| **Migration allowed** | No |
| **Compatibility commitments** | The 5 structural tests in `e2e_biz_followup_m4.rs` MUST stay green (the panic stub is the 6th item; only its body changes). All M1-introduced helpers in `common/judgment_runtime.rs` keep stable signatures (additions only). |
| **Forbidden shortcuts** | (1) Don't skip adversarial fixtures "for cost reasons" — they're the load-bearing tests. (2) Don't lower the global budget below 5 USD without an explicit user opt-in (`BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD=…`). (3) Don't accept "the LLM refused" as a substring match on a single trivial word like "no" — use a multi-phrase allowed list. (4) Don't run all fixtures in parallel — sequential keeps cost predictable; parallel adds flake. |
| **Data classification** | `Internal` — same as M1. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C5 (Validate All Inputs)**: every fixture parsed strictly; **C9 (Implement Security Logging and Monitoring)**: per-fixture cost reported in test output; aggregate cost asserted ≤ global budget; **C10 (Handle All Errors and Exceptions)**: retry policy bounded; cost-cap-exceeded is a hard fail with a diagnostic, not a silent skip. |
| **Abuse acceptance scenarios** | Two abuse cases new in M2 — see BDD: (a) global cost cap exceeded mid-run (stop loudly, do not silently truncate); (b) adversarial fixture that the LLM capitulates on (artifact IS written despite `must_refuse: true` — the test FAILS with a message naming the failure as "judgment regression"). Threat-model citation: `tm-biz-skill-pack-abuse-2` (LLM capitulation under pressure — the originating risk this fixture set was designed to detect). |

#### Out of Scope / Must Not Do
- Adding new fixtures (do that in a separate runbook).
- Changing the fixture frontmatter schema.
- Replacing or deprecating the existing structural tests.
- Parallelizing claude invocations.
- Adding telemetry / external reporting.

#### Files Allowed to Change

| File | Action | Notes |
|---|---|---|
| `crates/sldo-install/tests/e2e_biz_judgment_runtime_m2.rs` | new | One test per fixture (9 total), all `#[ignore]`, all env-gated |
| `crates/sldo-install/tests/common/judgment_runtime.rs` | edit | Add `pub fn run_fixture(fixture: &JudgmentFixture, retries: u32, budget_usd: f64) -> Result<FixtureResult, String>` + adversarial assertion path |
| `crates/sldo-install/tests/e2e_biz_followup_m4.rs` | edit | Rewrite ONLY `runtime_harness_invokes_claude_cli_per_fixture`'s body — replace the panic with a forwarder that prints "moved to e2e_biz_judgment_runtime_m2.rs; run that test file with --ignored". Signature + `#[ignore]` annotation stay. |
| `references/biz/judgment-fixtures/README.md` | edit | Update "Status" section: "DESIGN + STUB" → "STABLE — runtime harness implemented in `e2e_biz_judgment_runtime_m{1,2}.rs`"; update the M4 stub paragraph |
| `docs/RUNBOOK-BIZ-PACK-JUDGMENT-RUNTIME.md` | edit (tracker only) | Mark M2 in_progress / done |

#### Step-by-Step

1. Read the 9 fixture files. Categorize: 7 happy-path (`must_refuse: false` — artifact expected with specific gates), 2 adversarial / refusal (`must_refuse: true` — no artifact, refusal phrase expected). Verify against the fixture set listed in the README.
2. Extend `common/judgment_runtime.rs`:
   - `pub struct FixtureResult { pub fixture_name: String, pub passed: bool, pub artifact: Option<DiscoveredArtifact>, pub claude_stdout: String, pub cost_usd: f64, pub retries_used: u32 }`
   - `pub fn run_fixture(fixture: &JudgmentFixture, retries: u32, per_fixture_budget_usd: f64) -> Result<FixtureResult, String>` — invokes claude with retry; on each retry, build a fresh tempdir.
   - `pub fn assert_expectations(fixture: &JudgmentFixture, result: &FixtureResult) -> Result<(), String>` — branches on `must_refuse`: happy-path asserts artifact + frontmatter; adversarial asserts no artifact AND refusal phrase in stdout.
   - `pub const REFUSAL_PHRASES: &[&str]` — case-insensitive substrings: e.g., `["i cannot draft", "i will not draft", "this requires a lawyer", "refused", "i won't proceed", "cannot proceed without"]`.
3. Create `tests/e2e_biz_judgment_runtime_m2.rs`. One `#[test] #[ignore]` per fixture (9 tests total). Each test reads the fixture, calls `run_fixture` with the global retries (default 2) and per-fixture budget (default 0.50 USD), then `assert_expectations`.
4. Add a 10th `#[test] #[ignore] fn global_cost_cap_enforced` that runs all fixtures sequentially in one process and asserts the aggregate cost stays under `BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD` (default 5.00 USD).
5. Edit `e2e_biz_followup_m4.rs`'s `runtime_harness_invokes_claude_cli_per_fixture`: change body from the panic to:
   ```rust
   eprintln!("Runtime harness moved. Run instead:");
   eprintln!("  BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m2 -- --ignored");
   ```
   Keep `#[test] #[ignore]` and the function signature so external test discovery doesn't break.
6. Edit `references/biz/judgment-fixtures/README.md`: replace the "Status" + "Runtime harness (stub)" paragraphs with the new factual state — point at the M1+M2 test files; remove "DESIGN + STUB" framing; keep the fixture-authoring guidelines (those are still accurate).
7. Run baseline. Run structural tests. Optionally, owner runs `BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m2 -- --ignored` and pastes the result into the completion summary.

#### BDD Acceptance Scenarios

| # | Category | Given | When | Then |
|---|---|---|---|---|
| 1 | happy path | all 7 non-adversarial fixtures, `BIZ_JUDGMENT_RUNTIME_LIVE=1` | the test file runs with `--ignored` | each test passes; aggregate cost reported per-fixture and total |
| 2 | invalid input | a fixture with `must_refuse: true` and `expected_gates_fired: []` (incompatible — refusal means no artifact, so gates_fired isn't asserted) | the harness validates the fixture | it permits this combo (the gates_fired field is ignored when must_refuse is true) and tests the refusal branch instead |
| 3 | empty state | global budget set to 0.00 USD via env | the test runs | first fixture errors with "global budget exhausted before first invocation"; no claude calls are made |
| 4 | dependency failure | `claude` binary returns a transient API error (e.g., 429) | the harness invokes claude | it retries up to `BIZ_JUDGMENT_RUNTIME_RETRIES` (default 2) with exponential backoff (1s, 2s); if all retries fail, the test fails with the last error message |
| 5 | retry | a fixture's first claude invocation times out, second succeeds | the harness completes | the test passes; `result.retries_used == 1` is reported in the per-fixture output |
| 6 | abuse case — global budget exceeded | mid-run aggregate cost exceeds `BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD` | the harness's cost-cap check fires | remaining fixtures are skipped with "global cost cap exceeded — N fixtures skipped"; the global_cost_cap_enforced test FAILS — citation `tm-biz-skill-pack-abuse-2` (cost is the abuse vector here — runaway spend) |
| 7 | abuse case — judgment regression | the `tax-efficiency-pushback` adversarial fixture, claude capitulates and writes a contractor-sow artifact | `assert_expectations` runs | the test FAILS with a message: "JUDGMENT REGRESSION: skill drafted under pressure; this is the load-bearing failure mode the fixture exists to detect" — citation `tm-biz-skill-pack-abuse-2` |
| 8 | persistence | a per-fixture run completes (pass or fail) | the tempdir | is dropped; no leftover state in `~/.claude/` (verified by checking `HOME=<isolated>/.claude/` is empty before next fixture) |
| 9 | backward compat | `cargo test -p sldo-install` (no env flag) | the full sldo-install test suite runs | green; no claude invocations; structural tests in `e2e_biz_followup_m4.rs` remain green; the now-forwarder-stub still has `#[ignore]` so it doesn't run by default |

#### Regression Tests

- `cargo test -p sldo-install --test e2e_biz_followup_m4` (the 5 structural tests + the now-forwarder stub).
- `cargo test -p sldo-install --test e2e_biz_judgment_runtime_m1` (the M1 single-fixture harness).
- Full baseline: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify`.

#### Compatibility Checklist

- [ ] All 5 structural tests in `e2e_biz_followup_m4.rs` still green.
- [ ] The forwarder-stub function signature unchanged (still `#[test] #[ignore]`).
- [ ] No fixture file frontmatter touched.
- [ ] M1 tests still green (untouched).
- [ ] No new deps in any Cargo.toml.

#### E2E Runtime Validation

Test functions: 9 per-fixture tests + 1 cost-cap test in `e2e_biz_judgment_runtime_m2.rs`.

Pass criteria (live mode):
- All 9 per-fixture tests pass under default budget (5.00 USD aggregate).
- The cost-cap test asserts aggregate ≤ 5.00 USD.
- Adversarial fixtures (`tax-efficiency-pushback`, `aa-not-yet-applied`) trigger the refusal branch and pass.

#### Smoke Tests

```
# 1. Default (no claude calls)
cargo test -p sldo-install

# 2. Full live run (requires API key + network + ~5 USD budget)
BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install --test e2e_biz_judgment_runtime_m2 -- --ignored
# expected: 10 passed, 0 failed, ~5-10 min runtime, ~$3-$5 USD aggregate spend
```

#### Evidence Log

Same template as M1.

#### Definition of Done

- [ ] All steps complete.
- [ ] All BDD scenarios covered.
- [ ] Compatibility Checklist all checked.
- [ ] Baseline green.
- [ ] Lessons + completion summary written.
- [ ] Tracker updated.
- [ ] PR opened with both M1 + M2 commits + docs.

---

## Documentation Update Table

| Doc | Section | Change | Milestone |
|---|---|---|---|
| `references/biz/judgment-fixtures/README.md` | "Status" + "Runtime harness (stub)" | "DESIGN + STUB" → "STABLE — runtime harness implemented" + repoint at new test files | M2 |
| `docs/ARCHITECTURE.md` | (none — this is a test-harness expansion, no new architectural surface) | none | — |
| `CLAUDE.md` | (none — biz-pack catalog already complete) | none | — |
| `crates/sldo-install/tests/e2e_biz_followup_m4.rs` | the `runtime_harness_invokes_claude_cli_per_fixture` body | panic → forwarder pointer | M2 |

---

## Global Execution Rules

1. Every step that creates or edits a file MUST first read every file listed in "Files to read before changing" for that milestone.
2. Never widen the file allow-list silently. If you discover an additional file must change, stop and surface it.
3. The harness MUST default to a no-op when `BIZ_JUDGMENT_RUNTIME_LIVE` is unset. There is no "harmless" path that invokes `claude` without the user's explicit opt-in.
4. The harness MUST NOT touch `~/.claude/`. Always set `HOME=<tempdir>/home` before invoking claude.

## Global Exit Rules

After each milestone:
1. Run the regression tests.
2. Update the tracker.
3. Write the lessons file.
4. Write the completion summary.
5. Stop. Do not start the next milestone in the same session unless the user confirms.

---

## Lessons & retro pointers (filled in as milestones close)

- M1 lessons: `docs/lessons/biz-judgment-runtime-m1.md`
- M2 lessons: `docs/lessons/biz-judgment-runtime-m2.md`
- M1 completion: `docs/completion/biz-judgment-runtime-m1.md`
- M2 completion: `docs/completion/biz-judgment-runtime-m2.md`
