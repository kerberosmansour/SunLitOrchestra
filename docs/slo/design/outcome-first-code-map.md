# Code Map — outcome-first

Brownfield map for the Outcome First Engineering change. The change is Markdown +
one Rust test, so the "code" here is the **v4 template + loop skills + the
structural-test harness**. Mirrors
[measurement-loop-slo-improvements-code-map.md](measurement-loop-slo-improvements-code-map.md).

## Four-object summary

| Object | Path | Role in this change |
|---|---|---|
| **v4 runbook template** | `docs/slo/templates/runbook-template_v_4_template.md` | Carries most of the 11 changes: new §5C Outcome Validation Contract (after §5B at line ~406), §17 sub-sections (Outcome Scenarios / Critical User Journeys / Core Capability Regression Matrix — the milestone template starts at line ~945, BDD Acceptance Scenarios at ~1020), §11 Outcome test layer (§11.4 naming table ~709, §11.6 runtime-validation ~729), §6 "Outcome outranks unit" rule (~408). |
| **Sprint-loop skills** | `skills/slo-plan/`, `skills/slo-execute/`, `skills/slo-verify/`, `skills/slo-retro/`, `skills/slo-critique/` (each `SKILL.md`) | `/slo-plan` requires §5C for value-bearing milestones; `/slo-execute` writes outcome/journey tests first; `/slo-verify` gains Pass 0 = Outcome Validation (non-renumbering; current Passes 1–6 keep their numbers); `/slo-retro` gains the outcome refusal gate (alongside the refusals at ~lines 34–46) + `## Outcome vs promise`; `/slo-critique` reviews outcome scenarios for theatre. |
| **Loop / catalog / contract docs** | `docs/LOOPS-ENGINEERING.md` (Sprint loop ~30–87), `docs/skill-pack-catalog.md` (Sprint flow table ~18–33), `references/agent/operating-contract.md` | Outcome First Engineering principle + inverted pyramid + the "Verify Pass 0 = Outcome Validation gate" note. operating-contract is **host-neutral** (read by Copilot/Codex too). |
| **Structural-test + install harness** | `xtasks/sast-verify/tests/` (exemplar `sap_imp_m5_agents.rs`), `crates/sldo-install/src/install.rs` (`discover_skills` ~46–73) | New test asserts the new template sections exist + edited SKILL.md SHA baselines + Pass 0 naming. `discover_skills` is **untouched** (no new skill dir — elevate-in-place). |

## Exemplar code to copy

- **Optional-additive section pattern** — `docs/slo/templates/runbook-template_v_4_template.md` §5A (line 316) and §5B (line 341): the `> **Optional section.** Legacy runbooks without this section remain valid ... But /slo-plan REQUIRES it for any value-bearing ...` blockquote. **Copy this exact shape verbatim for §5C** — it is the load-bearing backward-compat contract.
- **Resolution vocabulary `pass | not_applicable | waived_with_reason` (never blank)** — §5B Security Test Plan (line 383–385) and the `.slo.json` Bundle rows. **Copy for the Core Capability Regression Matrix resolution column.**
- **Frozen ID discipline** — `tm-<slug>-abuse-N` in [outcome-first-threat-model.slo.json](outcome-first-threat-model.slo.json) + `references/security/threat-model-schema.md` (lines 76–81, supersede-don't-renumber). **Copy for `oc-<slug>-N` and `cuj-<slug>-N`.**
- **Additive verify pass without renumber-breakage** — the measurement loop added Pass 6 additively; `/slo-verify` Pass 4/5/6 content is the template for "new pass, resolution vocabulary, regression-test-first on finding". For Pass 0 we *insert at the front* without renumbering Passes 1–6 (the non-renumbering DW-001 decision).
- **Structural-contract test** — `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`: `extract_frontmatter` + `serde_yaml_ng` parse (~lines 59–66), SHA-256 byte-identical baseline constant (~lines 25–26, 318–331), required-field assertion loop (~lines 145–166), vacuous-pass for deferred branches (~lines 69–74). **Copy the SHA-baseline + section-presence assertion shape.**
- **Measurement-loop family** as the closest *whole-change* exemplar — `docs/slo/design/measurement-loop-slo-improvements-*` is a methodology change of the same shape (new optional §5A section + new verify pass + new retro section + loop doc). Follow its decomposition and interface discipline.

## Anti-exemplar code not to copy

- **Do NOT mint a "v5" template or renumber v4 sections.** The v3→v4 jump was a deliberate generational rewrite; this change is additive insertions only. A renumber breaks every in-flight v4 runbook's section references.
- **Do NOT add a `skills/slo-outcome/` directory.** The elevate-in-place decision (D1) means no new skill; adding one would change the `discover_skills` install surface and create duplicate runtime-BDD ownership with `/slo-verify` Pass 1.
- **Do NOT renumber `/slo-verify` Pass numbers.** Pass 4/5/6 are referenced by name+number across skills + the threat-model read-side contract; Pass 0 is the non-renumbering choice precisely to avoid this. Insert Outcome Validation as Pass 0 and leave Passes 1–6 untouched.
- **Do NOT edit a SKILL.md without updating its SHA baseline in the same milestone** (and vice-versa) — the `sast-verify` gate compares byte-identical hashes; a mismatch is a hard red, never waived.
- **Do NOT make the operating-contract principle host-specific.** `references/agent/operating-contract.md` is read by Copilot + Codex; keep "Outcome First Engineering" phrased host-neutrally (no Claude/Playwright-only assumptions in the principle text — those belong in the Claude overlay / skill prose).

## Dangerous seams (inspect before editing)

0. **The v4 template is TWO byte-identical copies** — `skills/slo-plan/references/runbook-template_v_4_template.md` (skill-primary, the one `/slo-plan` uses) **and** `docs/slo/templates/runbook-template_v_4_template.md` (mirror). Three existing tests enforce byte-identity: `svl_m1::template_copies_stay_byte_identical`, plus byte checks in `svl_m3` and `mloop_m3_plan`. **Every template edit must hit both copies identically** or those tests go red. (Verified 2026-06-17.)
1. **`/slo-verify` pass numbering** — high-risk seam. Resolved by the **non-renumbering Pass 0** decision (DW-001): insert Outcome Validation ahead of Pass 1 and leave Passes 1–6 numbered as-is, so the repo-wide `Pass 4/5/6` citations (and the threat-model read-side contract) are untouched. Still `rg -n "Pass [0-9]"` before editing to confirm.
2. **`xtasks/sast-verify` SHA baselines** — only **`slo-critique/SKILL.md` is currently SHA-pinned, by TWO tests**: `sap_imp_m5_agents.rs` (F-ENG-6) and `slo_tm_m2_consumers.rs` (F-ENG-6 lockstep). Editing it (M4) requires updating **both** baseline constants in lockstep AND preserving the `## Rotation order` heading + the four persona anchors (CEO / Eng lead / Security / Design) that `slo_tm_m2_consumers` asserts. `slo-plan`/`slo-execute`/`slo-verify`/`slo-retro` SKILL.md are **not** currently SHA-pinned — this runbook adds fresh pins for them (founder direction); they ARE read for behavior markers by `svl_m2/svl_m4/mloop_m3_plan/mloop_m4/kani_m3/sap_imp_m3_standards`, so additive edits must preserve existing asserted markers. (Verified 2026-06-17.)
3. **`/slo-plan` §5A/§5B requirement logic** — §5C must reuse the same "value-bearing → required, else N/A-with-reason" trigger; a divergent trigger creates confusing double-gating.
4. **`docs/LOOPS-ENGINEERING.md` Secure Value overlay** — the new Outcome-First overlay sits next to the existing Secure Value overlay (lines 70–87); keep them visibly distinct (security envelope vs outcome authority) so neither is read as subsuming the other.
5. **`/slo-retro` refusal-gate stacking** — the new outcome refusal joins existing refusals (blank Evidence-Log actuals, pending BDD, Kani row, untracked artifacts ~lines 34–46); ensure the new gate fires for *value-bearing* milestones only, matching the §5C trigger, so pure-docs runbooks still close.
