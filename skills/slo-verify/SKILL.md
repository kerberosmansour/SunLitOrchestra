---
name: slo-verify
description: >
  Use this skill after /slo-execute finishes a milestone, before /slo-retro
  closes it. Invoke with the milestone number — "/slo-verify M3". Exercises
  BDD scenarios at runtime, including UI paths via Playwright if the milestone
  has a UI surface. For every bug found, writes a regression test FIRST, then
  hands the fix back to /slo-execute, then re-verifies. Do not use for static
  code review — that is /slo-critique.
---

# /slo-verify — runtime QA on a completed milestone

You are the QA lead. A milestone just finished implementation. Compilation and unit tests passed. Your job is to prove the thing actually works at runtime, including states the happy path never hits.

## Inputs

- A runbook at `docs/RUNBOOK-<feature>.md` with milestone N in `in_progress` state.
- The milestone's BDD Acceptance Scenarios and E2E Runtime Validation sections.
- The evidence log (you will add rows).

## Output

- `docs/verify/<prefix>-m<N>.md` — verification report.
- Regression tests for every bug you find (committed BEFORE the fix).
- Evidence log rows filled with runtime-check results.

## Prereq cascade (if UI)

If the milestone touches a UI surface:

1. `which node` — install hint if missing.
2. `npm ls playwright` in the target project — if absent, run `npx playwright install` and record it.
3. `which chromium` — Playwright ships its own Chromium; but confirm the binary is reachable.

If it's a pure backend / CLI milestone, skip the UI cascade and stick to runtime E2E.

## Method — three passes

### Pass 1. Happy path

Run every happy-path scenario from the BDD table at runtime. If the target is a UI, drive it with Playwright. If it's a CLI, exec the binary with realistic inputs. If it's an IPC/API, issue real calls.

Record: what you did, what you observed, pass/fail.

### Pass 2. Empty and degraded states

From the BDD table, run the empty-state, invalid-input, and dependency-failure scenarios. This is where most bugs live.

### Pass 3. Partial failures and boundary conditions

For any scenario that has a "partial failure" category, construct the failure (pull the plug, kill the dep, starve the queue). Observe what the system does. Every unexpected observation is a candidate bug.

### Pass 4. Security (supply-chain + variant-analysis + conditional DAST)

Pass 4 is additive: it runs after Passes 1–3 and never replaces them. It catches classes of problems that behavioral testing alone misses: vulnerable dependencies, known-bad code patterns lurking in files the milestone didn't edit, and (when applicable) runtime exposure via DAST.

**Stack detection** — inspect the target repo's manifests: `Cargo.toml` → Rust; `package.json` → Node / TypeScript; `pyproject.toml` or `requirements.txt` → Python; `go.mod` → Go. When multiple are present (**polyglot** targets: the real common case for many projects), Pass 4 runs **all applicable command sets**, and each stack gets its own row in the Pass 4 section of the verification report. No arbitrary tiebreaker; one row per stack.

**Tool-optional rule** — if a named command (`cargo audit`, `cargo deny`, `semgrep`, `ast-grep`, `npm audit`, `govulncheck`, `pip-audit`, ZAP, Dastardly) is not on PATH, Pass 4 emits an explicit `skipped — <tool> not installed (see <install-hint>)` row and moves on. Missing tools do not fail Pass 4.

**Tool-error vs. finding** — each command in [`references/security-pass-commands.md`](references/security-pass-commands.md) documents its exit-code semantics. Exit 0 is clean; exit 1 is a finding; **exit ≥ 2 is tool error / advisory DB unreachable / network failure — always mapped to a `skipped` row, never to a finding**. This is load-bearing: offline / air-gapped / flaky-network sessions must not auto-generate phantom regression tests for transient `cargo audit` DB fetch failures.

**DAST conditional on smoke-service presence** — DAST (OWASP ZAP or Dastardly) runs only when the target has a runnable smoke / reference service with an OpenAPI spec or a `docker-compose.yml` exposing a service. On markdown-only / library-only targets, DAST is explicitly `N/A — no compiled artifacts / no smoke service` with the reason recorded. This prevents DAST runs on pure docs/skill-pack milestones from being noise.

**Command reference** — the full command catalog (Rust, Node, Python, Go, DAST) lives in [`references/security-pass-commands.md`](references/security-pass-commands.md). Each command documents its exit-code contract, install hint, and interactive-budget expectation. Pass 4 targets ≤ 2 min total on a small milestone; commands that exceed that budget are deferred to a nightly cadence.

**Bug-found flow — reuse the existing one.** When Pass 4 surfaces a finding (not a tool-error), apply the same flow Passes 1–3 use: STOP; write the regression test first; hand the fix back to `/slo-execute`; re-run Pass 4 to confirm green; re-run Passes 1–3 to confirm no regression. Pass 4 does not invent a new flow.

**Anti-pattern** — running DAST on a markdown-only or library-only target. DAST needs a service to scan; running it against a docs repo produces noise. The smoke-service-presence gate is the whole defense.

**Biz-pack PII-pattern scan** (added Runbook B1 M1) — when the target repo contains a `docs/biz-public/` directory, Pass 4 ALSO runs a PII-pattern scan over every artifact in that subtree. This is the runtime enforcement for the biz-pack two-tier output convention (`docs/biz/` confidential / `docs/biz-public/` placeholder-only — see [`references/biz/artifact-schema.md`](../../references/biz/artifact-schema.md)). The scan flags artifacts that match any of these regex patterns:

- **Email addresses** — RFC 5321 simplified: `[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}`.
- **UK National Insurance numbers** — `\b[A-CEGHJ-PR-TW-Z][A-CEGHJ-NPR-TW-Z]\d{6}[A-D]\b` (HMRC format).
- **UK sort codes** — `\b\d{2}-\d{2}-\d{2}\b` paired with the literal token "sort code" within ±3 lines.
- **Capitalised-bigram named-person heuristic** — lines beginning with `name:` (case-insensitive) followed by a `[A-Z][a-z]+ [A-Z][a-z]+` pattern. False-positive tolerance is HIGH; this catches the common interview-transcript leak pattern.

**Override mechanism** — an artifact MAY include `pii_scan_override: true` and `tier_override_reason: <one-line rationale>` in its frontmatter. The scan reads the override + reason and EMITS the override decision in the Pass 4 report (so it's auditable) but does NOT fail the milestone. Without the override, a match fails Pass 4 with the same regression-test flow as other findings: STOP → write a regression test → fix the file (move to `docs/biz/` or anonymise) → re-run.

**Scan scope** — `docs/biz-public/` only. The `docs/biz/` subtree is NOT scanned (those artifacts are confidential by design and contain real PII). The founder's repo `.gitignore` excludes `docs/biz/` (skill prose enforces this; SKILL.md prose for every biz-pack skill includes a write-time warning). Pass 4 PII-scan is the second-line defense after the gitignore + write-time-warning first-line.

**Threat-model rows** — this scan addresses `tm-biz-abuse-1` (founder repo leak) and `tm-biz-abuse-6` (founder pastes PII into generator). See [`docs/design/biz-skill-pack-threat-model.md`](../../docs/design/biz-skill-pack-threat-model.md).

## When you find a bug

1. **STOP** and write a regression test that reproduces it. The test should fail today.
2. Commit the regression test on its own — do not bundle with the fix.
3. Hand the bug back to `/slo-execute` or a human to fix (do not fix it yourself in this skill — separation of concerns).
4. Once fixed, re-run the regression test; it should now pass.
5. Re-run the full milestone verification to confirm no regression in other scenarios.

## Gates — do not mark verified when

- Any BDD scenario is untested at runtime (including empty-state).
- A regression test was added without a fix being applied in the same branch.
- The milestone's Evidence Log still has blank runtime rows.
- Playwright traces / screenshots from failing scenarios weren't captured.

## Verification report shape

```markdown
# Verification Report — <prefix> Milestone <N>

## What was exercised
| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|

## Bugs found
| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|

## Environment
- OS, browser, Node version, platform.

## Coverage gaps
- <scenarios NOT exercised, with reason>
```

## Anti-patterns

- Re-running unit tests and calling it "verification." Unit tests are the input; runtime is what this skill checks.
- Finding a bug, fixing it inline, and never adding a regression test.
- Skipping empty-state because "it's just a screenshot" — empty states are where AI-slop lives.
- Batching multiple bugs into one "fix and re-verify" cycle. Do one at a time so the regression test per bug is clean.

## Handoff

When every BDD scenario has a runtime row with a `pass` result, suggest `/slo-retro M<N>` to close out the milestone. If bugs were found and fixed, the retro should mention them as "missing coverage" in the lessons file.

---

**Loops**: Sprint loop, Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
