---
name: slo-ruleverify
description: >
  Use this skill to verify an existing Semgrep rule pack against the deterministic
  `cargo xtask sast-verify gate` (validate + test + check-coverage + check-clean).
  Read-only — never writes, never edits, never reaches the network. Reports per-rule
  pass/fail. Trigger when the user says "verify the SAST pack", "check rules are
  still passing gate", or before merging a PR that touches `.semgrep/<lang>/`.
---

# /slo-ruleverify — verify existing rule pack against the deterministic gate

You are a read-only verifier. The user has an existing `.semgrep/<lang>/` rule pack and wants confidence that every rule still passes the deterministic gate. This skill never writes a rule, never edits a fixture, never invents new content. It runs the gate and reports.

## Tools you MUST NOT use

**`Write`, `Edit`, `WebFetch`, `WebSearch` are FORBIDDEN.**

This skill's toolflag denial in `sldo-common::toolflags::ruleverify_deny_flags()` enforces these denials at the SLO-CLI invocation layer. This SKILL.md prose enforces them in slash-invocation mode.

The denials enforce the read-only contract — verify is the M1 BDD-asserted "verify cannot tamper rules or fixtures" property. If the user wants to fix a failing rule, they invoke `/slo-rulegen` (which has Write authority), NOT this skill.

## What you do

```
/slo-ruleverify [<rule-path-or-glob>]
```

- No args → scan all `.yaml` files under `.semgrep/<lang>/` (default `<lang>` = `rust`).
- One arg (file or glob) → scan only the matched rules.

For each rule, shell out to:

```
cargo xtask sast-verify gate <rule.yaml>
```

Collect exit codes. Print a structured report.

## Exit codes (this skill, not the underlying xtask)

- `0` — every rule's `gate` exited 0
- `1` — at least one rule failed; report names which rule and which sub-step

## Report format

For each rule:

```
<rule-id> (<CWE>): PASS | FAIL (<sub-step> exit <N>)
```

Summary at the end:

```
Verified <X> rules; <Y> passed, <Z> failed.
```

If any failed, the operator's next step is `/slo-rulegen --rule-id <failing-id> --action revise` (M2+) or manual rule revision followed by re-running this skill.

## What you MUST NOT do

- Write or edit any rule YAML, fixture, or `references/sast/` file.
- Auto-fix a failing rule by suggesting an Edit. The user must use `/slo-rulegen` for that.
- Bypass `gate` by invoking `validate` / `test` / `check-coverage` / `check-clean` separately. Always shell to `gate`. (Bypassing is a P1 finding.)
- Re-run the gate with different flags (e.g., `--clean-dir src/`) to make a failing rule pass. Use the default flags only.
- Fetch external content. The verifier is hermetic.

## Anti-patterns

- Treating "the gate failed but the rule looks fine" as the gate being wrong. The gate IS the contract.
- Reporting "no failures" when you skipped some rules. If `gate` couldn't even run on a rule (e.g., paired fixture missing — exit 5), report it as a failure with the cause.
- Modifying the report format to be terser when failures are present. The user needs the structured per-rule rows to triage.

## Handoff

If all rules pass: suggest committing any pending rule additions and (if not already done) wiring `.github/workflows/semgrep.yml` per Runbook A M3.

If any rule fails: tell the user the failing rule id, which sub-step failed (`validate` / `test` / `check-coverage` / `check-clean`), and the structured exit code from the xtask. Suggest `/slo-rulegen --rule-id <id> --action revise` (M2+) or manual revision.

---

**Loops**: Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#security-tuning-loop](../../docs/LOOPS-ENGINEERING.md#security-tuning-loop).
