---
name: scanner-orchestration
created: 2026-04-26
status: ideation
tla_required: false
---

# Scanner Orchestration — `/slo-sast` wedge for threat-model-driven Semgrep

## The pain

A solo OSS maintainer had a remote-code-execution disclosed in one of their public projects — a command-injection bug that a generic Semgrep run might or might not have flagged, but which a CWE-77/78 ruleset tuned to the project's specific input-handling surfaces would have caught. The repo had no security tests in CI at all. The disclosure was personally embarrassing, the patch took a long time to land, and additional time went into reassuring the contributor community that "this will be dealt with better next time." The bug itself wasn't novel; the failure was that no scanner was running, and even if one had been, generic rules don't know which CWEs the project's threat model actually fears.

## Five capabilities the user described without realizing

- Read the project's threat model (`docs/slo/design/<slug>-threat-model.md`) and extract the CWE list it names.
- Detect the project stack (`Cargo.toml`, `package.json`, language-mix in source) and pick Semgrep rule packs that cover the CWE × stack intersection — not the whole registry.
- Emit `.semgrep.yml` and `.github/workflows/sast.yml` with safe defaults (`on: pull_request`, never `pull_request_target`; minimal `permissions:` block; no secrets exposed to PR runs).
- Establish and maintain a baseline of pre-existing findings so onboarding an existing repo doesn't jam CI on day one.
- Re-derive the ruleset when the threat model changes — surface the diff for human review before the redeploy lands. (This is the auto-tuning loop and the actual wedge.)

## Top risks

- **Breach**: `pull_request_target` footgun in the auto-emitted workflow. The skill (or a future contributor / Claude run) writes `on: pull_request_target`, granting forked PRs the upstream secret context. **Adversary**: drive-by malicious PR contributor exfiltrating `GITHUB_TOKEN` or any repo secret. **Surface**: the auto-emitted CI YAML.
- **Compliance fine**: False coverage claim against **PCI DSS 6.2.3 (v4.0.1 — bespoke and custom software code review)** or **SOC 2 CC7.1**. The skill emits a coverage doc claiming CWE-77 / CWE-78 / CWE-89 are covered because the threat model named them, but the chosen rule packs don't actually have rules for the user's specific framework. User cites the doc in audit; auditor disproves. **Data class**: whatever the regulated app processes (cardholder data for PCI, customer PII for SOC 2 CC7.1 scope). Note: the v3.2.1 numbering for code review was **6.3.2**; in v4.0.1 it renumbered to **6.2.3** and v4.0.1's 6.3.2 is now the bespoke / third-party software inventory mandate (different scope, out of v1).
- **Prolonged outage**: Baseline-day CI jam in existing repos. Skill onboards a mature project, baseline doesn't capture all 200 legacy critical findings, fail-on-new-critical degrades to fail-on-everything. **First noticer**: the next maintainer trying to merge a PR (within hours of install). **Defection**: team disables the workflow within 1–3 days; SLO skill abandoned by precisely the user who most needed it.

## Approach A — pure Markdown skill (conservative)

- **Effort**: 0.5–1 person-week.
- **Wedge**: SKILL.md prompt drives Claude to read threat-model + stack files, look up CWE → rule-pack mapping in a committed reference table, emit `.semgrep.yml` + `.github/workflows/sast.yml`, run `semgrep ci --baseline` to seed a baseline, open a PR.
- **Risks**: rule-pack selection is prompt-driven and non-deterministic across Claude runs; CWE → ruleset mapping logic lives only in prompt + reference table and is hard to unit-test exhaustively. Mitigation: structural tests assert the emitted `.semgrep.yml` references the expected packs for given threat-model fixtures.

## Approach B — hosted Semgrep AppSec Platform integration (cloud / SaaS)

- **Effort**: 3–4 person-weeks.
- **Wedge**: skill provisions a Semgrep AppSec Platform account for the user, ties the repo, configures rule policies via the platform's API, wires PR comments through the Semgrep bot, surfaces findings in the platform's dashboard.
- **Risks**: requires Semgrep paid tier (free OSS tier exists but has limits); kills airgapped / self-hosted story; introduces account provisioning + billing surface that doesn't fit the OSS-tool framing of SLO; couples SLO's roadmap to a vendor's API stability.

## Approach C — `crates/sldo-sast` Rust backend + skill driver (local)

- **Effort**: 2 person-weeks.
- **Wedge**: new Rust crate handles deterministic logic (threat-model parse → CWE extraction, CWE × stack → rule-pack selection, baseline diff, severity classification); markdown skill shells out to the binary. Mirrors the `/slo-research` + `sldo-research` precedent.
- **Risks**: more code to build and maintain; bootstrap requires `cargo install`; the heavy lift is Semgrep itself, so a Rust shim adds machinery without a proportional determinism win; slower iteration loop than prompt-only.

## Recommendation

Approach A. The skill is glue: read documents, look up a mapping, emit YAML, invoke Semgrep CLI. Semgrep does the heavy lift. A Rust backend (Approach C) is overkill when the deterministic core is "table lookup + YAML emission" — that fits a committed reference table called from a prompt. SaaS (Approach B) kills SLO's airgapped/OSS posture and binds the roadmap to a vendor. The differentiator is the auto-tuning loop — re-derive ruleset on threat-model edit, surface the diff — which doesn't exist as a feature of Semgrep AppSec or `/slo-architect`. The one-week wedge: SKILL.md + a `references/scanner-orchestration/cwe-to-semgrep-pack.md` mapping table + a `.github/workflows/sast.yml` template with safe defaults + a structural test fixture. Fail-on-new-critical only; baseline seeded on first run.

## Open questions for /slo-research

1. **Semgrep CI integration patterns** — what does the Semgrep team / community recommend for `pull_request` vs `pull_request_target` discipline, baseline-suppression mechanics for `semgrep ci` on existing repos, severity-threshold behaviour, and PR-comment posting? What are the documented common pitfalls?
2. **CWE → Semgrep rule pack mapping** — is there a maintained CWE-to-rule-pack mapping anywhere (Semgrep registry tags, `p/owasp-top-10`, `r2c-security-audit`)? Or does the skill have to author its own table? What's the canonical lookup path, and how often does the registry's CWE tagging actually align with the rule's real coverage?
3. **Auditable SAST coverage claims** — what does PCI DSS 6.3.2 / SOC 2 CC7.1 actually accept as evidence of "code review for vulnerabilities"? Schema, format, retention requirements. The v1 wedge skips the coverage doc, but the data we capture in v1 has to support emitting one later.
4. **`pull_request_target` security posture** — current GitHub-recommended hardening for SAST workflows that need to read PR diffs. Trail of Bits, Semgrep, GitHub Actions Security docs. What's the minimum-permissions stance, and how do you handle first-time-contributor PRs without exposing secrets?
5. **Prior art for threat-model-driven scanner configuration** — has anyone done this? OWASP Threat Dragon (does its export feed into anything?), NIST SSDF PW.7 implementation guidance, vendor docs (Snyk Code, CodeQL config-from-threat-model posts). Null findings (nobody's done it) is itself a valuable signal.
