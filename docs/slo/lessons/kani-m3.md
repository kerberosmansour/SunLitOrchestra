# Lessons Learned — kani Milestone 3

## What changed
- `kani_required` frontmatter key + Step 5.5 in `/slo-architect`; §5.8 Kani proof-obligation sub-block in the v4 runbook template; Kani hooks in `/slo-plan` (step 5), `/slo-execute` (§8.5), `/slo-verify` (Kani-obligation verification), `/slo-retro` (blank-Kani-evidence refusal + scope recording); `kani_m3_integration.rs` (6 assertions); interfaces.md §5.8 marked stable.

## Design decisions and why
- Numbered the architect step **5.5** (not renumbering 5/6) so the `tla_required` step and its tests stay put — additive, not disruptive.
- Made §5.8 the Kani sub-block **after** §5.7, appended — TLA+ §5.1–5.7 prose is byte-unchanged, satisfying the "additive to §5" contract.
- The additivity test picks a *real* pre-existing overview (`sast-rulegen-skill-pack-overview.md`) and asserts it parses AND lacks the key — a behavioral additivity proof, not just a prose check.

## Mistakes made
- None this milestone. The kani-m2 lesson (lowercase the haystack) was applied up front to all six prose-gate assertions, so no capitalization gotcha recurred.

## Root causes
- N/A — clean run.

## What was harder than expected
- The riskiest part was editing six *installed* skills without breaking the `slo_tm_m2_consumers` (slo-verify phrase-presence) and `sap_imp_m5_agents` (slo-critique SHA-256) baselines. Confirmed safe by checking those tests assert *presence*/SHA on files I either appended-to (slo-verify) or didn't touch (slo-critique) before editing.

## Naming conventions established
- Integration-seam assertions live in a dedicated `kani_m3_integration.rs` sibling — never an edit to an existing baseline test (kani-m1 rule held).
- Sub-block numbering: append (`§5.8`), don't renumber, when extending a template section that other tests/skills reference.

## Test patterns that worked well
- Reading the SHA/phrase-pinning tests (`slo_tm_m2_consumers.rs`) BEFORE editing the skills they pin — turned a risky six-file edit into a confident additive one.
- The additivity test reads a real legacy artifact rather than a synthetic fixture.

## Missing tests that should exist now
- None for M3. M4 exercises the seams behaviorally against real Kani.

## Rules for the next milestone
- **M4 needs the Kani toolchain.** Check `cargo kani --version` against the pin first; if absent, the ENG-1 path fires (loud documented skip) and the demo runs on a machine that has it. The demo crate is a SEPARATE repo — do NOT add it as a workspace member (couples the toolchain to this repo's baseline).
- Each K-series (K1–K4) must show its red BEFORE its green in the scope report (naive-first).
- Continue lowercasing prose-gate haystacks; continue scoping clippy to new code.

## Template improvements suggested
- The v4 template now has a worked §5.8 example via this runbook's own §5.2 — future Rust runbooks can copy it.

## filed_issues
- none — forward-rules captured here; read by `/slo-execute M4`.
