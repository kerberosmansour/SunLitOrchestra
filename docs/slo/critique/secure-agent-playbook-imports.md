# Critique — Secure Agent Playbook Imports

> **Target runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Reviewed**: 2026-05-01
> **Personas run**: CEO, eng-lead, security; design = N/A (no UI surface)
> **Skill**: `/slo-critique` (executed manually — skill not registered with the Skill tool)
> **Confidence gate**: every accepted finding rated ≥8/10. Lower-confidence concerns dropped per skill anti-pattern "Writing findings the user will immediately waive."
> **Resolution status (2026-05-01)**: User decision was "accept SEC + ENG, defer F-CEO-1". 13 findings applied inline to the runbook; 1 deferred. See Resolution Log below.

## Threat-model gap (process)

The runbook does not have a corresponding `docs/slo/design/secure-agent-playbook-imports-threat-model.md` produced by `/slo-architect` Step 3.5. Abuse cases (`tm-sap-imp-abuse-1` through `tm-sap-imp-abuse-15`) are embedded inline in milestone Contract Blocks. Per the security persona's procedure, this is a process gap — surfaced below as `F-SEC-1`. Security findings cite the embedded abuse-case ids as proxy.

## Findings summary table

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| F-CEO-1 | ceo | ask, reduce-scope | M5 (line 1376) | M5's deferred branch ships ~5% of planned output (lessons + handoff only) when M4's matrix forbids agents — a wasted milestone slot | M4 publishes matrix saying GitHub Copilot doesn't support host-native agents; M5 closes producing only a lessons file; the runbook has 5 milestones marked done but only 4 produced new artifacts | Drop M5 from this runbook. Decide whether to author RUNBOOK-AGENT-ROLES *after* M4 closes and the matrix is published. Cost: one more `/slo-plan` cycle if agents do happen. Benefit: this runbook closes cleanly at 4 milestones with 100% artifact production |
| F-ENG-1 | eng | ask | M1 (line 562) | M1's structural-contract test asserts "≥ 1 Markdown link to either security template" but doesn't specify whether the parser is AST-based (pulldown-cmark) or regex-based, nor whether code-fence content is excluded from citation counting | A future skill rewrite places the citation inside a fenced code block as part of an example — regex-based parser counts it; AST parser excludes it; behavior depends on implementation choice not specified in the runbook | Pin parser strategy in M1's Contract Block: use `pulldown-cmark` (AST-based) to walk parsed events and count `Event::Start(Tag::Link(...))` only. Excludes code-fence content. Add the parser-choice rationale to the lessons-file template |
| F-ENG-2 | eng | ask | M2 (line 757) — Invariants row, `every_abbreviates_ref_resolves` test | The `abbreviates:` frontmatter field accepts "an existing path or known skill name" but "known skill name" is fuzzy — no canonical list specified | A contributor sets `abbreviates: slo-foo` (typo for `slo-foo-bar`); test passes if "known skill name" is regex `^slo-[a-z-]+$`; fails if "known skill name" walks `skills/` directory at HEAD | M2's test resolves `abbreviates:` by either (a) walking `skills/<name>/SKILL.md` at HEAD and accepting any matching frontmatter `name`, or (b) treating the value as a literal path and `Path::exists()`. Pin choice (a) — it catches typos and rename drift |
| F-ENG-3 | eng | ask | M2 (line 757) — Out of Scope row | Runbook says "Do not consume `examples/` from any skill" but no test enforces it | A future PR adds `cite examples/security-finding.md` from a SKILL.md as a "see what good looks like" reference; passes M1 (cites a real template too); silently violates M2's Out-of-Scope rule and creates a dependency from skills onto the gallery | Add structural-contract assertion in `sap_imp_m2_examples.rs`: walk `skills/<name>/SKILL.md`; assert no Markdown link to `examples/` |
| F-ENG-4 | eng | ask | M3 (line 954) — Invariants row (e), threshold-rule | M3's high/critical mandatory-CWE rule is enforced *only* against `examples/security-finding.md` (one fixture). The rule does not enforce against real `docs/slo/critique/<slug>.md` outputs that future runs produce | A future critique pass writes a high-severity finding without CWE; M3's test passes (it's not configured to walk live critique outputs); the threshold rule is documented but not enforced where it matters | M3's test should walk `docs/slo/critique/*.md` AND `docs/slo/verify/*.md` for severity-tagged rows; assert any row with `severity: high` or `severity: critical` has CWE column populated. Vacuous-pass when those directories are empty |
| F-ENG-5 | eng | ask | M4 (line 1157) — `every_workflow_uses_is_sha_pinned` test | M4's SHA-pin test glob is implicit (`.github/workflows/*.yml`). Composite actions in `.github/actions/` and `.yaml`-extension workflows are not covered. Trigger-acceptable-set is also implicit (`tags:` good, `push:` bad — but `release:`, `workflow_dispatch:` not enumerated) | A future PR adds `.github/actions/release-helper/action.yml` containing `uses: foo/bar@v1`; test glob misses it; supply-chain risk re-enters via composite action | M4's test glob: `.github/{workflows,actions}/**/*.{yml,yaml}`. Trigger-acceptable-set: `tags: ['v*']`, `release:`, `workflow_dispatch:`, `schedule:`. Forbidden: `push:` to default branches. Encode both in M4's Contract Block |
| F-ENG-6 | eng | ask | M5 (line 1376) — `slo_critique_skill_md_unchanged` test | The "byte-identical /slo-critique SKILL.md" assertion uses "a hash recorded in the test or `git show HEAD:skills/slo-critique/SKILL.md`" — but `git show HEAD:` references the *current* commit (which moves during M5 development) | During M5 implementation, the test's HEAD-relative comparison passes against itself trivially; the canonical-portable-path preservation is not actually checked against M4 baseline | Pin a stable baseline: record SHA-256 of `skills/slo-critique/SKILL.md` at the *runbook authoring time* (or M4 close time) as a const in `sap_imp_m5_agents.rs`; test asserts the file's hash matches that recorded constant. Update the constant only via a runbook amendment |
| F-ENG-7 | eng | ask | M5 (line 1376) — BDD scenarios | No BDD scenario covers "agent invocation produces a `docs/slo/critique/<slug>.md` whose format diverges from `/slo-critique`'s artifact format (missing required sections)" | M5 ships green-lit; lead agent's manual invocation produces a critique file with the runbook reference but no per-finding rows; the runbook's compatibility commitment ("agents and persona rotation produce the same shape") silently breaks; downstream `/slo-execute` step 1.5 carry-forward query expects rows and finds none | Add BDD row to M5: "agent produces critique artifact whose format diverges from `/slo-critique`'s schema → assertion fails citing missing required sections". Implement as schema-validation step in `sap_imp_m5_agents.rs` against an artifact-format reference (extract format from current `/slo-critique` SKILL.md or one example critique) |
| F-SEC-1 | security | ask, defer | Prerequisite reading (line 7) | No `docs/slo/design/secure-agent-playbook-imports-threat-model.md` exists. Embedded abuse cases (`tm-sap-imp-abuse-1` through `-15`) substitute, but the canonical `/slo-architect` Step 3.5 artifact is absent. Class-elimination findings cannot reference STRIDE-shaped row-ids; future critique passes have no canonical table to consult | A second critique reviewer in 6 months looks for `tm-sap-imp-*` rows in the design directory, finds none, opens the runbook, sees inline rows; reviewer cannot easily cross-reference what the threat model claimed was eliminated vs. residual | Either (a) author `docs/slo/design/secure-agent-playbook-imports-threat-model.md` consolidating embedded abuse cases into a STRIDE-shaped file, or (b) add a Notes section to the runbook explicitly recording the inline-abuse-case decision and citing precedent (Markdown-only feature; no runtime trust boundary). Bug class: process gap, not a code class. Variant analysis: `find docs/slo/design -name '*-threat-model.md'` to confirm precedent |
| F-SEC-2 | security | ask, defer | M2 (line 757) — PII regex set | M2's PII regex set covers email + UK NI + UK sort code only. US, EU, German, French national-ID formats are not covered. Bug class: V8 PII at rest. Threat-model row: `tm-sap-imp-abuse-3` (says "real PII" without enumerating locale). Class state: mitigated, with locale gap. Variant analysis: zero current variants (gallery doesn't exist yet); search post-M2 would be `rg -n '\d{3}-\d{2}-\d{4}' examples/` (US SSN), `rg -n '[A-Z]{2}\d{2}[A-Z0-9]{1,30}' examples/` (EU IBAN) | Non-UK contributor anonymizes a real critique report by stripping company names but leaves their German Steuer-ID `12 345 67890` intact in a synthetic example. M2's regex doesn't match. Example commits and pushes; PII enters public git history. Class: mitigated with locale gap | Extend M2 PII regex set: US SSN (`\d{3}-\d{2}-\d{4}`), EU IBAN (`[A-Z]{2}\d{2}[A-Z0-9]{1,30}`), 11-digit numeric runs flagged with manual-review note. See appendix for expanded finding |
| F-SEC-3 | security | ask | M4 (line 1157) — `plugin_json_does_not_duplicate_skills` test | M4's test catches duplicated skill paths (`tm-sap-imp-abuse-9`) but doesn't reject `..` segments in plugin.json path values. Bug class: V4 path traversal. Threat-model row: `tm-sap-imp-abuse-9` (duplication is named; traversal is not). Class state: residual — known unmitigated; no compensating control. Variant analysis: `rg -n '\.\.' .claude-plugin/` post-M4 green-lit | Malicious PR adds `plugin.json` with `{"path": "../docs/biz/", ...}`. Claude Code resolves the path relative to `.claude-plugin/`. The repo's `docs/biz/` is currently gitignored but the path-resolution semantic is unconstrained — if a future contributor commits a confidential biz-doc by accident, the plugin manifest can install it as if it were a skill | M4's `plugin_json_does_not_duplicate_skills` test must additionally reject any `..` or absolute path in plugin.json's path-valued fields. Use `Path::components()` and assert no `Component::ParentDir`. See appendix for expanded finding |
| F-SEC-4 | security | ask | M4 (line 1157) — release-zip workflow | M4's release workflow specification doesn't require an explicit minimum-privilege `permissions:` block. Default `GITHUB_TOKEN` has `contents: write`, `actions: write`, and other ambient permissions in workflows that lack an explicit block. Bug class: V3 broken function-level authorization (GitHub Actions context). Class state: residual. Variant analysis: `rg -n '^permissions:' .github/workflows/` to confirm existing workflows already follow this discipline | A compromised contributor pushes tag `v0.1.0-evil`; the release workflow runs with default token permissions; if the workflow body includes a step that uses the token cross-repo or for issues/PRs (e.g., a logging step that opens an issue on failure), the blast radius is larger than `contents: write` | M4 must require an explicit top-level `permissions:` block with only `contents: write` (and `id-token: write` if attestation is later added). Add structural-contract test that walks `.github/workflows/*.yml` and asserts every workflow has a top-level `permissions:` block — this is wider scope than M4 strictly needs but the gate already exists. Confidence 9/10. See appendix |
| F-SEC-5 | security | ask | M4 (line 1157) — release-zip generation strategy | M4 says "the workflow generates a release zip from the existing `skills/` + `references/` tree without modifying source" but doesn't specify the zip-generation command. `tar -czf release.tar.gz .` or `cp -r .` would capture untracked files left by Actions plugins (`_temp/`, `_actions/`, npm caches). Bug class: V8 sensitive buffer leakage in release artifact. Threat-model row: not directly covered (closest is `tm-sap-imp-abuse-9` duplication but that's about plugin.json) — propose new `tm-sap-imp-abuse-13b: release zip includes untracked or gitignored files`. Class state: residual | Workflow's zip step uses `tar -czf release.tar.gz .`. After `actions/checkout` and any `setup-*` plugin, the runner working dir contains `_temp/`, `~/.npmrc` snippets, plugin caches. The zip captures these. A future addition of `setup-node@<sha>` writes an npm token into the runner; the zip artifact in GitHub Releases now exposes it | M4 must use `git archive --format=zip --prefix=sunlit-orchestra-${TAG}/ HEAD -o release.zip` (emits *only tracked files at HEAD*). Add a smoke test that unzips the release artifact and asserts file list ⊆ `git ls-files`. See appendix |
| F-SEC-6 | security | ask | M5 (line 1376) — `every_output_path_in_allowed_set` test | M5's path-set assertion checks prefix membership but doesn't reject `..` segments within those prefixes. Bug class: V4 path traversal. Threat-model row: `tm-sap-imp-abuse-13` (output-path outside allowed set is named; traversal within the prefix is not). Class state: residual | An agent declares `output-paths: [docs/slo/critique/../../../etc/passwd]`. Prefix `docs/slo/critique/` matches; M5's test passes. At runtime, an unsandboxed host writes to `/etc/passwd`. If Claude Code's host *does* sandbox writes, the runtime fails closed; if it doesn't, the agent escapes the runbook contract | M5's path-set assertion must canonicalize each declared path with `Path::components()` and reject any `Component::ParentDir`. Confidence 8/10 |
| F-DESIGN-1 | design | n/a | — | No UI surface in this runbook (Markdown skill changes + structural-contract tests + optional GitHub Actions + agent files). Per persona, design pass skipped | — | — |

## Appendices — expanded security findings

### Appendix A — F-SEC-3: M4 plugin.json path-traversal

| Field | Value |
|---|---|
| ID | `sap-imp-sec-3` |
| Source | `/slo-critique` security persona |
| Status | `open` |
| Confidence | `medium` |
| Location | `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` M4 Contract Block (line ~1218) and the test file `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs` (NEW) |
| Affected surface | `.claude-plugin/plugin.json` (NEW M4 conditional artifact) |
| Data classification | `Public` |
| Threat-model row | `tm-sap-imp-abuse-9` (duplication named; traversal not) — propose new row `tm-sap-imp-abuse-9b` for traversal |
| Bug class / CWE | V4 Path Traversal / `CWE-22` (Improper Limitation of a Pathname to a Restricted Directory) |
| Standards mapping | OWASP Top-10 A01 (Broken Access Control); ASVS V12.3 (File Path) |

#### Concrete scenario

Given a malicious or compromised PR that adds `.claude-plugin/plugin.json` with `{"skills": [{"name": "slo-foo", "path": "../docs/biz/"}]}`, when Claude Code reads the manifest and resolves the path relative to `.claude-plugin/`, then it could install `docs/biz/` as a skill set. The repo's `docs/biz/` is currently gitignored; if a future contributor accidentally commits a confidential biz-doc (which the gitignore would normally block but a `git add -f` could override), the plugin manifest provides a vector for that confidential content to be installed as user-facing skill content.

#### Evidence

- M4 Contract Block (runbook line ~1218): `tm-sap-imp-abuse-9` covers `.claude-plugin/skills/` duplication but not in-manifest path traversal
- M4 BDD scenario "Plugin.json duplicates skills" (line ~1280) explicitly tests for `.claude-plugin/skills/` prefix; doesn't test for `..`
- Existing `crates/sldo-install/src/install.rs:44-71` `discover_skills()` walks `skills/` directly; doesn't read plugin.json — so this is a Claude Code host concern, not an `sldo-install` concern. M4's structural-contract test is the only enforcement layer

#### Impact

If exploited, an installed plugin could surface confidential content as if it were a published skill, leading to (a) accidental disclosure of confidential biz-public artifacts inside a host workspace, (b) loss of the `docs/biz/` confidential tier guarantee documented in CLAUDE.md.

#### Remediation

In `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs`, when the test walks `plugin.json` path values:

```rust
for entry in plugin_manifest.skills.iter() {
    let path = std::path::PathBuf::from(&entry.path);
    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        panic!("plugin.json path '{}' contains '..' — path traversal forbidden", entry.path);
    }
    if path.is_absolute() {
        panic!("plugin.json path '{}' is absolute — relative paths only", entry.path);
    }
}
```

#### Verification

- Unit-test fixture: a plugin.json with `{"path": "../foo"}` → test fails with named error
- Unit-test fixture: a plugin.json with `{"path": "/etc/passwd"}` → test fails
- Unit-test fixture: a plugin.json with `{"path": "skills/slo-foo"}` → test passes

---

### Appendix B — F-SEC-4: M4 release workflow missing explicit `permissions:` block

| Field | Value |
|---|---|
| ID | `sap-imp-sec-4` |
| Source | `/slo-critique` security persona |
| Status | `open` |
| Confidence | `high` |
| Location | `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` M4 Contract Block (line ~1157), `.github/workflows/release-zip.yml` (NEW M4 conditional artifact) |
| Affected surface | GitHub Actions runner with default `GITHUB_TOKEN` permissions |
| Data classification | `Public` |
| Threat-model row | `tm-sap-imp-abuse-10` (workflow runs on every push named); propose new `tm-sap-imp-abuse-10b` for token-scope |
| Bug class / CWE | V3 Broken Access Control / `CWE-732` (Incorrect Permission Assignment) |
| Standards mapping | OWASP Top-10 A01 (Broken Access Control); ASVS V1.4 (Access Control Architecture); CIS GitHub Benchmark §1.4 |

#### Concrete scenario

Given a release workflow without an explicit `permissions:` block, when GitHub Actions runs the workflow, then `GITHUB_TOKEN` is created with the repository's default permissions (which can include `contents: write`, `pull-requests: write`, `issues: write`, `actions: write` — varies by repository setting). If a compromised contributor pushes a tag `v0.1.0-evil` and the workflow body includes any step that uses `gh` CLI or the GitHub API with the ambient token, the blast radius is everything those default permissions cover, not just the release-creation path.

#### Evidence

- M4 step-by-step (runbook line ~1242): "trigger on tag push only" — addresses *when* the workflow runs, not *what permissions* it has
- M4 "Forbidden shortcuts" (runbook line ~1228): names "no force-push of release tags", "no `I'll pin the SHA later`" — but does not name "missing `permissions:` block"
- M4's `release_workflow_is_tag_triggered` E2E test (runbook line ~1320) checks `on:` block — does not check `permissions:` block

#### Impact

A successful tag-push attack with a workflow lacking minimum-privilege scoping can lead to unauthorized issue creation, branch protection bypass via `actions: write`, secret exfiltration via composed environment variables, and other escalations beyond release-artifact creation.

#### Remediation

1. **In M4's Contract Block** (Forbidden shortcuts row): add "release workflow without an explicit minimum-privilege `permissions:` block".
2. **In M4's Files Allowed To Change** (release-zip.yml planned change): the workflow MUST contain a top-level `permissions:` block:
   ```yaml
   permissions:
     contents: write   # only what release-create needs
   ```
3. **In `xtasks/sast-verify/tests/sap_imp_m4_workflow_pinning.rs`**: add an assertion that walks `.github/workflows/*.yml` and asserts every workflow has a top-level `permissions:` block. Existing workflows that lack one are surfaced as a wider gap (likely already compliant per `SECURITY.md`; verify with `rg -n '^permissions:' .github/workflows/`).

#### Verification

- Add to M4 BDD: "workflow lacking `permissions:` block → structural-contract test fails citing the file"
- Local check before M4 close-out: `rg -n '^permissions:' .github/workflows/release-zip.yml` returns the explicit block

---

### Appendix C — F-SEC-5: M4 release-zip strategy unspecified

| Field | Value |
|---|---|
| ID | `sap-imp-sec-5` |
| Source | `/slo-critique` security persona |
| Status | `open` |
| Confidence | `medium` |
| Location | `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` M4 Contract Block + step 5 (line ~1242) |
| Affected surface | Released zip artifact published via GitHub Releases |
| Data classification | `Public` (zip is public-facing); but the *contents* could include local-only state |
| Threat-model row | propose new `tm-sap-imp-abuse-13b` (release zip includes untracked or gitignored files exposing local-only state) |
| Bug class / CWE | V8 Sensitive Buffer Leakage / `CWE-200` (Exposure of Sensitive Information) |
| Standards mapping | OWASP Top-10 A02 (Cryptographic Failures — broader info-disclosure); ASVS V8.2 (Client-side Data Protection — analogous principle for build outputs) |

#### Concrete scenario

Given M4 ships green-lit and the release workflow uses a naive command like `tar -czf release.tar.gz .` after `actions/checkout`, when subsequent setup-* actions add files to the runner working directory (`_temp/`, `~/.npmrc`, plugin caches, `.git/`), then the released zip captures those files. If a future addition of `actions/setup-node@<sha>` writes an npm token to `~/.npmrc` during workflow execution, and the zip step runs *after* setup-node, the token lands in the public release artifact.

#### Evidence

- M4 step 5 (runbook line ~1242): "Author `.github/workflows/release-zip.yml`. SHA-pin every `uses:`. Trigger on `tags: ['v*']`. No deps beyond default GitHub Actions ecosystem."
- No specification of which command produces the zip, what it includes, or what it excludes
- M4 "Forbidden shortcuts" row (line ~1228) names workflow + plugin concerns, not zip-content concerns
- No smoke test in M4 verifies the zip's contents

#### Impact

Local-only secrets, build artifacts, or misconfigured files leak into a public artifact. Once published to GitHub Releases, the artifact is mirrored by package proxies and cannot be reliably retracted.

#### Remediation

1. **In M4's release workflow body**: use `git archive --format=zip --prefix=sunlit-orchestra-${TAG}/ HEAD -o release.zip`. This emits *only* tracked files at the tagged SHA; untracked and gitignored content is excluded by construction.
2. **In M4's smoke tests**: unzip the locally-generated artifact (from the `gh release create --draft` dry-run) and assert `unzip -l release.zip | awk ...` ⊆ `git ls-files`.
3. **In M4's Contract Block** (Forbidden shortcuts): add "release workflow that uses `tar`/`zip` of the working directory rather than `git archive`".

#### Verification

- Smoke test asserts zip content ⊆ `git ls-files` output
- BDD row added: "release workflow zips runner working directory (untracked content)" → smoke test fails

---

## Resolution Log

User decision (2026-05-01): "accept SEC + ENG, defer F-CEO-1". 13 findings applied; 1 deferred.

| id | decision | applied at runbook section |
|---|---|---|
| F-CEO-1 | **deferred** — M5 stays in this runbook; user may revisit after M4 closes and the host capability matrix publishes | — |
| F-ENG-1 | **applied** — M1 Contract Block: pulldown-cmark AST parser pinned in Invariants row (e); regex-based citation counting added to Forbidden shortcuts | M1 Contract Block (line ~588, ~591) |
| F-ENG-2 | **applied** — M2 Invariants row (d): `abbreviates:` resolves via either skill-name walk OR filesystem path check; new BDD scenarios for both happy paths | M2 Contract Block + BDD (line ~783, ~852) |
| F-ENG-3 | **applied** — M2 Invariants row (f): no shipped SKILL.md links to `examples/` (enforced via M1's pulldown-cmark walk); new E2E test `no_skill_links_to_examples`; new BDD scenario | M2 Contract Block + BDD + E2E (line ~783, ~847, ~888) |
| F-ENG-4 | **applied** — M3 Invariants row (f): threshold-rule walks live `docs/slo/critique/*.md` AND `docs/slo/verify/*.md` (vacuous-pass when empty); new E2E test `live_critique_and_verify_findings_have_cwe` | M3 Contract Block + E2E (line ~986, ~1090) |
| F-ENG-5 | **applied** — M4 Invariants row (a) + (h): glob extended to `.github/{workflows,actions}/**/*.{yml,yaml}`; trigger-acceptable-set enumerated `{tags, release, workflow_dispatch, schedule}`; forbidden `{push to default, pull_request}`. E2E test `release_workflow_trigger_in_acceptable_set` replaces `release_workflow_is_tag_triggered` | M4 Contract Block + E2E (line ~1190, ~1296) |
| F-ENG-6 | **applied** — M5 Invariants row (f): SHA-256 hash pinned as const in test file (replaces fragile `git show HEAD:` baseline); E2E test `slo_critique_skill_md_unchanged` rewritten | M5 Contract Block + E2E (line ~1413, ~1528) |
| F-ENG-7 | **applied** — M5 Invariants row (g): agent-output schema validation against `/slo-critique` artifact contract; new E2E test `agent_output_artifact_schema_valid`; new BDD scenario "Agent-output format mismatch caught" | M5 Contract Block + E2E + BDD (line ~1413, ~1530, ~1485) |
| F-SEC-1 | **applied (option b)** — runbook §9 gains a "Threat-model artifact decision" subsection documenting the inline-abuse-cases choice with three reasons + a rule for when a separate threat-model file becomes mandatory | Section 9 (line ~360) |
| F-SEC-2 | **applied** — M2 Invariants row (c): PII regex set extended to include US SSN + EU IBAN (alongside existing email + UK NI + UK sort code); new BDD scenarios for US SSN and EU IBAN match-detection; E2E test `examples_pii_pattern_scan_clean` updated | M2 Contract Block + BDD + E2E (line ~783, ~847, ~886) |
| F-SEC-3 | **applied** — M4 Invariants row (b): plugin.json path-valued fields canonicalized via `Path::components()`; rejects `Component::ParentDir` and absolute paths; new E2E test `plugin_json_paths_are_safe`; new Forbidden shortcut + Files Allowed To Change row update | M4 Contract Block + E2E (line ~1190, ~1224, ~1300) |
| F-SEC-4 | **applied (≥9/10 confidence)** — M4 Invariants row (f): every `.github/workflows/*.yml` MUST have a top-level explicit `permissions:` block; release workflow scoped to `contents: write` only; new E2E test `every_workflow_has_explicit_permissions_block`; new Forbidden shortcut + Files Allowed To Change row update | M4 Contract Block + E2E (line ~1190, ~1224, ~1297) |
| F-SEC-5 | **applied** — M4 Invariants row (g): release zip MUST be generated via `git archive --format=zip --prefix=sunlit-orchestra-${TAG}/ HEAD -o release.zip` (not `tar`/`cp`/`zip` of working dir); new E2E tests `release_workflow_uses_git_archive` + `release_zip_content_subset_of_git_ls_files`; new Forbidden shortcut + Files Allowed To Change row update | M4 Contract Block + E2E (line ~1190, ~1224, ~1303) |
| F-SEC-6 | **applied** — M5 Invariants row (c): output-path entries canonicalized via `Path::components()`; rejects `Component::ParentDir` and absolute paths; new BDD scenarios "Path-traversal in output path caught" + "Absolute output path caught"; E2E test `every_output_path_in_allowed_set` rewritten | M5 Contract Block + BDD + E2E (line ~1413, ~1485, ~1525) |

Each `applied` finding is linked to the runbook line where the change landed. The runbook's pre-flight (Section 7) and Carmack reliability rules (Section 4) are unchanged — only milestone-scoped Contract Blocks, BDD scenarios, and E2E test entries received edits.

The runbook is now unblocked for `/slo-execute M1`.

### F-CEO-1 deferral note

F-CEO-1 was deferred — M5 remains in this runbook. The user may revisit the scope question after M4 closes and the host capability matrix publishes. If the matrix says agents are not supported on GitHub Copilot and the deferred branch fires, M5 will close with only a lessons file, and a follow-up runbook decision can be made at that point.

## Notes

- F-DESIGN-1 is N/A by design (no UI surface in this runbook).
- F-CEO-1 is the only scope-shaping ask. The other 13 findings are about test specificity and security hardening — they tighten the runbook without expanding scope.
- F-SEC-2, F-SEC-3, F-SEC-4, F-SEC-5, F-SEC-6 cluster around M4 and M2 and would benefit from being addressed together if you accept them — they share the structural-contract test surface in `xtasks/sast-verify/`.
- No findings on M3 beyond F-ENG-4 (threshold-rule fixture-only). M3's design appears solid against both eng and security lenses.
