# Stack Decision — scanner-orchestration

## Chosen stack

- **Skill format**: pure Markdown `SKILL.md` under `skills/slo-sast/` (no Rust crate).
- **Runtime**: Claude Code interprets SKILL.md; Claude shells out to `git`, `gh`, and (transitively, in the emitted workflow) `semgrep`.
- **Cache layout**: `~/.cache/sldo/semgrep-rules/<SHA>/` for the upstream-rules clone, scoped by SHA so multiple pins coexist.
- **Emitted artifacts** (committed in target repo): `.semgrep/rules/<rule-files>.yaml`, `.semgrep.yml`, `.github/workflows/sast.yml`, `.semgrep/manifest.json`.
- **Pinned versions** (set at execute time, not research time): `actions/checkout@<v4.2.x SHA>`, `github/codeql-action/upload-sarif@<v3.x SHA>`, Semgrep CLI floor `≥ 1.50.0` (already established in `references/sast/MIN-SEMGREP-VERSION.md`); pin Semgrep minor in the emitted workflow and bump deliberately.

## Reason

The synthesis says *"the design must handle CWE → rule mapping by querying registry metadata at scan time (not maintaining a hand-curated lookup table)... because the registry is the canonical source per the contributing guide and the live rule example proves legacy schema gaps exist."* Querying the registry is YAML-parsing plus filtering — work Claude does well from a Markdown prompt. The synthesis also says *"the design must handle severity gating by tightening rule selection... because Semgrep's blocking-findings doc confirms standalone users have no post-scan gate"* — meaning the skill's deterministic core is "select tight rules and emit a manifest of what was selected," not heavy compute. CLAUDE.md is explicit about post-cleanup direction: *"all other Rust code... was removed in the 2026-04 cleanup — the skills are the canonical interface now."* A Markdown skill matches that direction; a new Rust crate fights it. Idea doc Approach A is locked here.

## Rejected alternatives

- **Approach B — `crates/sldo-sast-index` Rust helper** — re-introduces the `cargo install` bootstrap the 2026-04 cleanup removed; conflicts with stated direction. Determinism win is small because Semgrep itself is the heavy lift, not registry-YAML parsing.
- **Approach C — Semgrep AppSec Platform integration** — kills airgapped/self-hosted story; binds the SLO roadmap to a vendor's API stability and paid-tier surface; doesn't fit the OSS-tool framing of SLO.
- **Hand-curated CWE → rule-pack mapping table** — registry metadata is canonical and queryable per the contributing guide and `metadata-cwe.yaml` linter; maintaining a parallel table is duplicate work that drifts.
- **Two-workflow split (`pull_request` + `workflow_run` commenter) at v1** — the GitHub Security Lab canonical pattern, but v1's audience (solo OSS maintainers) gets PR feedback for free via SARIF-to-Code-Scanning. Defer to v2.

## Non-negotiables (downstream cannot change these without migration)

- **Markdown-only skill contract.** No Rust crate added by this runbook; if a deterministic helper is later needed, it goes through a fresh `/slo-architect` decision (Approach B becomes a v2 conversation).
- **`on: pull_request` only.** The emitted workflow MUST NOT use `pull_request_target` for the SAST job. Hard ban. Even with the 2025-12-08 default-branch mitigation, full secret access on PR runs is unacceptable for SAST.
- **`fetch-depth: 0`** on the emitted `actions/checkout` step. Default `1` breaks `semgrep ci` diff-aware scans (Semgrep KB's canonical pitfall).
- **`permissions: {}`** at workflow scope; minimal per-job (`contents: read` for analysis; `security-events: write` only on the SARIF-upload step).
- **SHA-pin every third-party action** in the emitted template (CVE-2025-30066 / Shai Hulud v2 are the failure-case evidence).
- **PCI DSS 6.2.3 (v4.0.1)** wherever a compliance claim is made — never `6.3.2`. v4.0.1 6.3.2 is the SBOM-inventory mandate (different scope, out of v1).
- **Manifest schema** at `.semgrep/manifest.json` — the field set is the audit-defense contract. `cwes_claimed`, `cwes_actually_covered`, `threat_model_sha`, `semgrep_rules_sha`, `semgrep_version`, `selected_rules` (list of `{path, sha, cwe[], technology[]}`), `generated_at` (ISO-8601 UTC), `generated_by_skill_version`. Adding fields is non-breaking; renaming or removing requires a migration milestone.
- **Threat-model parse contract** — only canonical Markdown is parsed for CWE references (regex `CWE-\d+` against rendered prose; HTML comments are ignored). Locked to defuse template-placeholder smuggling per the `~~~text` fence discipline.
- **`/slo-rulegen` deploy-target contract** — `/slo-sast` reads from `.semgrep/rules/` regardless of authoring source. `/slo-rulegen` writes to `.semgrep/rules/<rule-id>.yaml`; `/slo-sast` consumes those alongside registry-selected rules. The directory layout is the integration contract.
- **No `--config` flag** in the emitted `semgrep ci` invocation — use `SEMGREP_RULES` env var instead. (CLI reference says `--config` is "not supported in ci mode"; env-var path is the future-proof choice.)
- **Cache directory location** — `~/.cache/sldo/semgrep-rules/<SHA>/` per XDG. The SHA suffix is the migration knob: bumping the pinned SHA writes a sibling cache; never overwrite an existing SHA's cache.
