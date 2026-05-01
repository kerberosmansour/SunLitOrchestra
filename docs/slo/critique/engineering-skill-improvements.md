---
name: engineering-skill-improvements
runbook: docs/slo/future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md
critiqued: 2026-04-28
personas_run: [ceo, eng, security]
design_persona_skipped: yes — no UI surface
---

# Critique — engineering-skill-improvements

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| E-1 | ceo | reduce-scope | M5 / per-skill evals | M5 specifies eval cases for 16 skills × 7 categories = 112 eval files minimum. Realistic implementation drag. | Agent starts M5; writes evals for 5 advisor skills (35 files); context window saturates; remaining 11 skills get stub evals or none. | Cut to highest-risk skills only in M5 (advisors + sast + tla + execute + verify = 9 skills). Lower-risk skills get a 1-line "evals deferred to follow-up" rationale. **ask** the user to confirm the cut. |
| E-2 | ceo | hold-scope | M1 source-verification spike | M1 spike says "5 representative claims". Real verification of `/slo-sast` + `/slo-verify` security claims is 20-30 claims minimum. | Spike marks 5 claims verified; M2 decomposition lands; M3 review surfaces 18 unverified claims still in `methodology-m2-stack-detect.md`. | Reframe: "spike = 5 claims to validate the discipline; full verification of the ~20-30 claims is folded into M2/M3/M4 per-milestone evidence-log work, not a single M1 batch." **auto-fix** applied to M1 pre-flight. |
| E-3 | ceo | ask | M5 PreToolUse hook | "This is not a security boundary" framing is correct but counterintuitive — why ship the hook at all? Confidence question. | Project owner reviews M5; asks "if the hook isn't a security boundary, why bother?" — answer must be in the doc. | **ask** user to confirm framing: hook is a discipline-enforcer that catches honest mistakes (agent under context pressure), not adversarial bypass. Surface this explicitly in `/slo-freeze` SKILL.md description. |
| E-4 | eng | auto-fix | M2 / M3 cut plans | Cut plans for `/slo-sast` (M2) and `/slo-tla` (M3) are described as "documented in lessons" but the lessons template doesn't have a "Cut plan" section. | M2 author writes the cut plan in the wrong template field; M3 author can't find M2's cut plan; M4 author has no precedent. | Add "Cut plan applied" as a documented lessons-file section in M2 + M3 + M4. **auto-fix** applied to milestone Notes. |
| E-5 | eng | ask | Soft line-cap = 200 lines | `/slo-architect` is currently 144 lines — passes cap. But `/slo-architect`'s Step 3.5 STRIDE-sweep + SECURITY.md + threat-model emit is dense and could grow. | Future security-related extension to `/slo-architect` pushes to 220 lines; soft-cap test fails; team adds `# soft-cap-exception:` pragma rather than decomposing. | Verify `/slo-architect` post-Phase-1 line count; if close to 200, include in M2 decomposition wave. Otherwise accept. **ask** user — include `/slo-architect` in decomposition? |
| E-6 | eng | auto-fix | M5 PreToolUse hook | Hook script is described as "argv-list discipline applies if it shells out" — but no structural-contract test asserts the hook script's argv-list pattern. | Hook script ships shell-string `bash -c "test -f $HOME/.sldo/freeze-scope.txt"`; argv-list discipline silently violated. | Add hook-script structural-contract test in M5 BDD: "hook script grep for shell-string patterns; FAIL if found." **auto-fix** applied to M5 BDD. |
| E-7 | eng | hold-scope | M2 prose preservation rule | "Diff between old SKILL.md and (new SKILL.md ∪ all 5 methodology files) MUST be empty for prose content" — strict, but minor formatting changes (heading-level shifts, link-text tweaks for new context) ARE legitimate. | Decomposition agent reformats a heading from `### Method` to `## Method` (it's now a top-level section in the methodology file); strict diff fails. | Refine the rule: "diff must show no semantic prose loss; heading-level shifts and link-text tweaks for new context are permitted, called out in the cut plan." **hold-scope** — keep strict-shaped rule but with documented permitted variations. |
| S-1 | security | hold-scope | M1 citation-discipline | Source hierarchy ranks Stack Overflow as "never authoritative" — but agents ROUTINELY consult SO during research. The discipline is "never CITE as authoritative" not "never read". | Agent reads SO post for context, then writes a SKILL.md claim citing the underlying tool docs (correct). Some other agent reads SO post and cites it directly (wrong). | Confirm wording: "Stack Overflow may inform reading; never appears as a citation source." **hold-scope** — already correct in design overview prose. |
| S-2 | security | auto-fix | M2 / M3 prose preservation BDD | "Security discipline preserved" BDD rows enumerate specific disciplines (argv-list, no `pull_request_target`, action-SHA pin, etc.) — but the list is hand-written and could drift from the actual disciplines in the SKILL.md. | M2 decomposition drops the action-SHA-pin discipline; the BDD checks for "argv-list" and passes; the missing discipline is undetected. | Generate the discipline list programmatically: M2 test extracts every "MUST" rule from old SKILL.md and asserts each appears in the post-decomposition tree. **auto-fix** applied to M2 + M3 E2E test descriptions. |
| S-3 | security | ask | M5 PreToolUse hook bypass via session-state deletion | `tm-eng-skill-improvements-abuse-2: PreToolUse hook bypassed via session-state file deletion` — labeled "residual" + "not a security boundary" disclaimer. | Adversary deletes `~/.sldo/freeze-scope.txt`; hook treats missing file as "no freeze active"; Edit/Write proceeds anywhere. | Already documented as residual + disclaimer. **ask** — is residual acceptable, or should the hook FAIL CLOSED (block all edits) when session-state is unexpectedly missing during an active session? Lean: fail-closed only inside an active `/slo-freeze` invocation; cleaner to fail-open by default. |
| S-4 | security | auto-fix | M5 settings.json mutation surface | Hook mutation via `update-config` skill is correct; but the runbook doesn't enumerate WHICH `update-config` invocation patterns are forbidden (e.g., disabling existing PreToolUse hooks while adding the new one). | Future M5 implementation `update-config`s settings.json by overwriting `PreToolUse` array entirely, dropping pre-existing hooks. | Specify: "extend `PreToolUse` array additively; never overwrite. Reject any `update-config` invocation that removes existing PreToolUse entries." **auto-fix** applied to M5 contract. |

## Auto-fix corrections applied

- E-2: M1 pre-flight reframed — spike validates discipline; full verification per-milestone in M2/M3/M4.
- E-4: cut-plan section added to milestone Notes for M2 + M3 + M4.
- E-6: M5 BDD adds hook-script argv-list test.
- S-2: M2 + M3 E2E test descriptions reframed to "extract every MUST rule programmatically".
- S-4: M5 contract specifies additive-only `PreToolUse` array extension.

## Asks for project-owner decision

- **E-1**: cut M5 evals to highest-risk-only (9 skills)?
- **E-3**: PreToolUse hook framing — "discipline-enforcer for honest mistakes" — confirm and surface in `/slo-freeze` SKILL.md description?
- **E-5**: include `/slo-architect` in M2 decomposition wave (if line count is approaching 200)?
- **S-3**: hook fail-closed when session-state file unexpectedly missing during active freeze?

## Final disposition

**Accept with minor edits + asks**. No critical findings. M5's eval scope (E-1) is the largest open question.
