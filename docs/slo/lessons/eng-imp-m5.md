---
runbook: engineering-skill-improvements
prefix: eng-imp
milestone: M5
created: 2026-05-04
status: done
---

# Lessons - eng-imp M5

## What Landed

M5 added the documented expectation layer and the opt-in freeze hook without
changing the installer surface.

| Surface | Result |
|---|---|
| `skills/<skill>/evals/happy-path.md` | Added one synthetic happy-path eval for each of the 16 high-risk skills named in the contract. |
| `.claude/settings.json` | Added a project-local `PreToolUse` hook for `Edit`, `Write`, and `NotebookEdit` that reads `~/.sldo/freeze-scope.txt`. |
| `references/freeze/hook-setup.md` | Documented opt-in setup, additive mutation, and the "not a security boundary" residual risk. |
| `references/biz/consent-script-uk.md` | Added a short UK GDPR legitimate-interest interview consent script for `/slo-talk-to-users`. |
| Six polish targets | Tightened `/slo-freeze`, `/slo-second-opinion`, `get-api-docs`, `/slo-talk-to-users`, and `/slo-verify`; `/slo-research` already carried the required `sldo-research --help` and tool-safety text. |

## Implementation Lessons

- Hook commands must preserve hook stdin. The first implementation used a
  heredoc (`python3 - <<'PY'`), which consumed stdin for the script body and
  prevented the Claude hook payload from reaching `json.load(sys.stdin)`.
  The smoke test caught this; the final hook uses `python3 -c ...` so stdin
  remains available for the hook payload.
- The runbook references an existing `update-config` skill, but no such skill is
  present in `skills/` in this repo/session. The setup doc still names
  `update-config` as the canonical mutation surface when available, and the
  committed `.claude/settings.json` is project-local and additive.
- The M5 contract has two scales in tension: a later note imagines all skills
  across all eval categories, while the acceptance gate and allow-list name 16
  high-risk skills with at least one eval case. This milestone implemented the
  acceptance gate. The larger eval matrix should be its own runbook once the
  runtime harness exists.

## Evidence

| Check | Actual Result |
|---|---|
| Repo hygiene | Branch before edits: `slo/eng-imp-m5`; dirty tree before edits: clean; remediation needed: none. |
| Prior-retro carry-forward | `gh issue list --label retro-derived --search "eng-imp" --state open --json number,title,body,url` returned `[]`. |
| Baseline before M5 edits | `cargo test --workspace` passed before M5 edits. |
| Red-first M5 test | `cargo test -p sldo-install --test e2e_eng_imp_m5` failed for expected missing eval/hook/consent/polish artifacts. |
| M5 structural test | `cargo test -p sldo-install --test e2e_eng_imp_m5` passed after implementation. |
| Hook smoke | Temp-`HOME` simulation allowed in-scope target, blocked out-of-scope target with exit 2, and treated a missing scope file as no active freeze. |
| Compatibility sentinels | `cargo test -p sldo-install --test e2e_eng_imp_m4` passed; `cargo test -p sldo-install --test e2e_biz_b1_m1 --test e2e_slo_sp_m5 --test e2e_eng_imp_m3` passed. |
| Package test suite | `cargo test -p sldo-install` passed with a pre-existing unrelated warning in `e2e_biz_followup_m5.rs`. |
| Workspace test suite | `cargo test --workspace` passed with pre-existing `sast-verify` warnings and the same unrelated warning. |
| Workspace build | `cargo build --workspace` passed with pre-existing `sast-verify` warnings. |
| Formatting | `rustfmt --edition 2021 --check crates/sldo-install/tests/e2e_eng_imp_m5.rs` passed; `cargo fmt --check -p sldo-install` remains blocked by pre-existing unrelated drift in `e2e_biz_imp_m1.rs` and `e2e_biz_imp_m2.rs`. |
| Diff hygiene | `git diff --check` passed. |

## Follow-Ups

- File a dedicated eval-runtime-harness runbook before expanding the eval matrix
  beyond one synthetic case per high-risk skill.
- If `update-config` is added later, teach it to append the freeze hook without
  replacing unrelated project hooks.
- Consider a future test that executes the hook command with sample Claude hook
  payloads so stdin regressions are caught by CI, not only manual smoke.
