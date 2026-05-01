# Lessons Learned — slo-sec Milestone 4

## What changed

- `skills/slo-verify/SKILL.md` — Pass 4 section (`### Pass 4. Security`) inserted between Pass 3 and the `## When you find a bug` heading. Documents stack detection (Cargo.toml / package.json / pyproject.toml / go.mod), polyglot rule, tool-optional rule, exit-code semantics (tool error → skipped, not finding), DAST conditional on smoke-service presence, markdown-only N/A path, and bug-found flow reuse.
- `skills/slo-verify/references/security-pass-commands.md` (NEW) — command catalog for Rust / Node / Python / Go / DAST. Every command documents exit-code contract and install hint. Polyglot example included. Target-repo `.gitignore` snippet for Pass 4 output artifacts.
- `docs/ARCHITECTURE.md` — one-line update to the Skill Pack table row for `slo-verify` to reflect four passes.
- `crates/sldo-install/tests/e2e_slo_sec_m4.rs` (NEW) — 18 structural-contract tests including Pass 1/2/3 byte-invariants via FNV-1a-64, Pass 4 presence + positioning, stack-detection + polyglot + tool-optional + DAST-conditional + markdown-only-N/A prose checks, reference-file command-catalog content checks.

## Design decisions and why

- **Pass 4 is an H3 nested under the same H2 as Passes 1–3.** Rationale: same-section depth signals it is part of the verification flow, not a separate skill. The existing H3 headings were `### Pass 1. Happy path`, `### Pass 2. Empty and degraded states`, `### Pass 3. Partial failures and boundary conditions`; `### Pass 4. Security` fits the pattern.
- **Pass 4 boundary in tests detected at nearest heading of any level, not specifically `\n## `.** The initial boundary logic would have broken whenever a new H3 was inserted between Pass 3 and the next H2. Fix: `min(find("\n### "), find("\n## "))`. This is reusable for any future pass-ordering test.
- **Tool-error = skipped, never finding.** The load-bearing invariant (critique f6) for offline / air-gapped / flaky-network sessions. Without it, a `cargo audit` timeout generates a phantom CVE regression test and poisons the dev flow. Documented in both SKILL.md and security-pass-commands.md.
- **DAST conditional on smoke service** (critique part of f4). DAST on markdown-only targets is noise; on library-only targets DAST has nothing to scan. The conditional is the whole defense. Documented tests ensure the gate is prose-visible.
- **`.gitignore` patterns for Pass 4 outputs live in the target-repo snippet inside the reference file, not in SLO's own `.gitignore`** (critique auto-fix f9). Pass 4 runs in target repos, not SLO. SLO's `.gitignore` remains unchanged.
- **No new Rust dependencies.** The FNV-1a-64 pattern (M2) is reused inline; no `sha2` / `blake3` / `sha256sum` subprocess. 18-line Rust inline implementation proves sufficient.

## Mistakes made

- Computed Pass 1/2/3 FNV-1a hashes using Python's `re.search(..., decoded_string).start()` — which returns **character offsets**, not byte offsets. The file has non-ASCII (em-dashes `\xe2\x80\x94`), so char offset ≠ byte offset. Rust `str::find` returns byte offsets. Result: first test run had 18/18 failures with wrong hashes. Root cause: the fixture capture step computed offsets on the decoded string but indexed into raw bytes. Fixed by re-capturing with `bytes.find(needle.encode())` on raw bytes.
- Pass 3 boundary logic used `\n## ` as the end-of-section marker. Correct until Pass 4 was inserted as an H3 inside the same H2 — then Pass 3's "end" jumped forward to include Pass 4 content, breaking the invariant. Fix: `min(find("\n### "), find("\n## "))`.
- Neither of these mistakes were scope violations — both were test-design errors caught at runtime. Net effect: two fix rounds, all tests now green.

## Root causes

- **Byte vs. character offsets in hash fixtures**: Python `re` works on decoded strings by default. When computing byte-offset fixtures for a Rust test that uses byte-offset APIs (`str::find` returns byte index), the Python script must index into raw bytes. **Rule for future milestones: always compute hash fixtures on raw bytes (`open(..., 'rb')` + `bytes.find(needle.encode())`), never on decoded strings — even for files that "look ASCII."**
- **Pass boundary too narrow**: I assumed the next H2 heading was the always-correct end-of-subsection marker. True pre-M4 (when Pass 3 was the last H3 before the next H2). Broken post-M4 (Pass 4 is another H3 at the same level). **Rule for future boundary invariants: match the nearest heading of any level (H2 or H3) from above the lower bound, not just H2.**

## What was harder than expected

- Getting the `extract_pass_subsections` function boundary logic right across both pre-M4 and post-M4 file shapes. The final version uses `min(next_h3, next_h2)` with option handling, which reads slightly clunkier but is correct for both cases.
- Balancing Pass 4 prose brevity against completeness. The final ~26-line insertion covers stack detection, polyglot, tool-optional, tool-error, DAST conditional, markdown-only N/A, bug-found reuse, and one anti-pattern. Could have been longer; chose to delegate specifics to the reference file.

## Naming conventions established

- Reference file naming continues the `references/*-commands.md` and `references/*-vocabulary.md` and `references/*-catalog.md` and `references/*-playbook.md` patterns — single-topic Markdown files under `skills/<skill>/references/`.
- FNV-1a-64 byte-invariant naming continues the `EXPECTED_<THING>_FNV1A_64` + `EXPECTED_<THING>_BYTE_LEN` pattern.
- Heading level for numbered passes inside `/slo-verify` is H3 (`### Pass N.`); the enclosing section is H2 (`## Method — three passes` pre-M4, conceptually four post-M4 but the H2 heading wasn't renamed to keep the invariant simple).

## Test patterns that worked well

- **Pass-subsection byte invariant via boundary-aware slicing** (once byte vs char bug fixed). Catches any accidental edit to Passes 1–3, including whitespace and newline changes.
- **Three-pass ordering assertion** via `body.find(...)` index comparisons. Cheap and readable; doesn't depend on FNV hashes, so it survives intentional content edits within a pass (with a separate hash check for that concern).
- **Command-catalog content tests that look for the tool name + its key flag** (e.g. `semgrep` + `--sarif`) instead of exact command string. Lets the reference file's command wording evolve without breaking tests.

## Missing tests that should exist now

- Runtime Pass 4 invocation against a real target with known CVEs. Deferred to a future `/slo-security-test` skill (Phase 3 of the security-embedding program).
- A CI job that validates all Pass 4 tool binaries are installable on a fresh runner. Would catch drift when a tool version goes EOL.
- A property-based fuzzer for the `extract_pass_subsections` boundary logic. Edge cases: file with no H3 after Pass 3; file with nested H4 inside Pass 3; file with `### Pass 5.` added later. Deferred.

## Rules for the next milestone

There is no M5 in this runbook — M4 is the last milestone. For the follow-on **Phase 2 runbook** (`/slo-threat-model` dedicated skill):

- The threat-model-template already written in M1 (`skills/slo-architect/references/threat-model-template.md`) is the output contract for `/slo-threat-model`. Don't duplicate it.
- Phase 2 introduces Python subprocess dependency (SecOpsTM v1.1.0). The `/slo-execute` allow-list rule will catch attempts to add Python deps outside that runbook's explicit scope.
- The placeholder-expansion `~~~text` fence rule is load-bearing and applies to every template the new skill emits — re-assert this in the new skill's SKILL.md.
- The FNV-1a-64 byte-invariant pattern is now proven across three runbook milestones (M2 template, M3 personas, M4 passes). Use it for any new byte-stable contract.

For the follow-on **Phase 3 runbook** (`/slo-security-test` skill):

- Pass 4 as documented in this M4 is the *contract*; Phase 3 ships the *harness* that invokes it. The reference file `security-pass-commands.md` is the canonical command catalog — Phase 3 wraps those commands with result-aggregation into a single SARIF / Markdown report.
- Polyglot rule is first-class; test against a fixture repo with Rust + Node simultaneously.
- Tool-optional rule must be end-to-end: if `cargo audit` fails to install, the skill prints an install hint and continues to the next tool. Test by uninstalling each tool one by one.

For the follow-on **Phase 4 runbook** (`/slo-sec-libs` with upstream issue filing):

- SLO-owned intake repo is the default filing channel. Third-party filing (Hulumi / SunLitSecureLibraries) needs `--file-upstream` flag + 40/hr client-side cap.
- CycloneDX 1.6 declarations reader is the core parser; Hulumi and SunLitSecureLibraries emit; SLO reads. Start with Python jsonschema; graduate to a Rust crate only if parsing complexity exceeds ~200 lines of Python.

## Template improvements suggested

- `docs/slo/templates/runbook-template_v_3_template.md` Evidence Log row set could include a "Baseline regression re-check" row to reinforce the M3 lesson (always run the full baseline after any edit). Deferred.
- The BDD and Runtime Validation Rules section could name "abuse case" as a required category when the milestone introduces a new surface. Today it's added by the `/slo-plan` SKILL.md edit in M2; the template lags. Deferred.
