# Research brief — scanner-orchestration

## Wedge (one sentence)

A pure-Markdown Claude Code skill (`/slo-sast`) that reads a project's `docs/design/<slug>-threat-model.md`, extracts the named CWE list, picks tuned Semgrep rule packs covering the CWE × stack intersection, emits a safe `.github/workflows/sast.yml` (`on: pull_request`, never `pull_request_target`) plus a baselined `.semgrep.yml`, and re-derives the ruleset whenever the threat model changes — surfacing the diff for human review before the redeploy lands.

## Target user (one sentence)

A solo or small-team OSS maintainer who has had (or fears having) a CVE disclosed against a public project that ran no SAST in CI, where the missed bug needed a CWE-tuned rule rather than a generic registry sweep, and who will abandon any tool that floods the team with legacy findings on day one.

## Five specific research questions

### Q1 — Semgrep CI integration patterns and baseline mechanics

What does the Semgrep team and community recommend for wiring Semgrep into GitHub Actions on existing repos with significant pre-existing findings?

- Look at: official Semgrep docs (`semgrep ci`, `--baseline-ref`, `--config auto`), Semgrep blog posts and conference talks (BSides, OWASP Global), real-world `.github/workflows/semgrep.yml` examples from popular OSS repos (e.g., `gitlab-org/gitlab`, `sentry/sentry`, `airbnb/javascript`, large Rust projects).
- Specific unknowns: how does `--baseline-ref` interact with `pull_request` event vs `merge_group`? What's the recommended severity-threshold mechanism (`--severity ERROR` vs config-level `severity: ERROR`)? How does Semgrep's PR-comment integration work without the AppSec Platform — is there a documented OSS-only path (e.g., `semgrep-action` or third-party comment-posters)? What are the documented common pitfalls (e.g., baseline drift on force-pushed PRs, monorepo path-filtering errors)?
- Required output: a documented "minimum-viable Semgrep CI on an existing OSS repo" recipe, with citations.

### Q2 — CWE → Semgrep rule pack mapping (canonical lookup path)

Is there a maintained CWE-to-rule-pack mapping anywhere, or does the skill have to author its own table?

- Look at: Semgrep registry tags (the `p/owasp-top-10`, `p/r2c-security-audit`, `p/cwe-top-25`, `p/security-audit` rule packs and how their metadata exposes CWE coverage), the Semgrep registry API or YAML metadata schema (each rule has a `metadata.cwe` field — is the coverage actually accurate or aspirational?), MITRE's CWE catalog cross-references, OWASP's CWE-to-rule-pack mappings (if any), academic papers on rule-pack coverage analysis.
- Specific unknowns: what fraction of Semgrep registry rules tag a CWE in their metadata? For the OWASP Top 10 2021 CWE list, how complete is the registry coverage per language (JavaScript, Python, Go, Rust, Java)? Does the registry expose a queryable index by CWE, or do you have to grep YAML?
- Required output: a verdict — "the registry's CWE tagging is canonical and queryable, use it" OR "the registry's tagging is sparse/inaccurate, the skill must maintain its own CWE → rule-pack reference table."

### Q3 — Auditable SAST coverage claims for PCI DSS 6.3.2 and SOC 2 CC7.1

What does PCI DSS 6.3.2 ("custom and bespoke software is reviewed prior to release to identify and address any potential coding vulnerability") and SOC 2 CC7.1 actually accept as evidence of SAST coverage?

- Look at: PCI DSS v4.0 official text and the PCI Council's supplemental guidance, AICPA SOC 2 Trust Services Criteria 2017 + 2022 revisions, PCI QSA blog posts and audit-prep guides (e.g., from Coalfire, Schellman, A-LIGN), SOC 2 audit checklists from Vanta / Drata / SecureFrame, NIST SSDF (SP 800-218) PW.7 implementation guidance.
- Specific unknowns: schema/format expectations (SARIF? CSV? PDF report?), retention requirements (1 year? 3 years? 7?), what counts as "addressed" for findings the team chose not to fix (risk acceptance memo? compensating control?), and whether claiming CWE coverage that's mapped-but-not-actually-scanned is a documented audit-failure pattern.
- Required output: a decision table on what data the v1 wedge must capture per scan run to support a defensible coverage claim later, even though the coverage doc itself is post-v1.

### Q4 — `pull_request_target` security posture and safe defaults

What is the current GitHub-recommended hardening posture for SAST workflows that need to read PR diffs from forks?

- Look at: GitHub Actions Security Hardening official docs, GitHub's `pull_request_target` advisory and its evolution since 2020, Trail of Bits "Untrusted input is untrusted, even if it's YAML" series, Semgrep's own blog posts on workflow security, CVE writeups for `pull_request_target` exploits in real OSS repos (e.g., `actions/runner-images`, `pytorch/pytorch`, others), GitGuardian/Snyk advisories on workflow misconfig.
- Specific unknowns: when (if ever) is `pull_request_target` actually required for SAST? What's the minimum-permissions posture (`permissions:` block contents)? How do you handle first-time contributor PRs without exposing secrets — is the "approve-and-run" gating sufficient? Are there documented patterns for "two-workflow split" (one for analysis, one for posting comments with elevated permissions)?
- Required output: a "safe-by-default workflow YAML template" with cited rationale for every `permissions:` line and event trigger choice.

### Q5 — Prior art for threat-model-driven scanner configuration

Has anyone built a system that derives scanner rule selection from a threat model, rather than running the whole registry?

- Look at: OWASP Threat Dragon (does its OTM JSON export feed into any SAST tool?), pytm (does it have CWE export hooks?), MITRE Threat Modeling Methodology, NIST SSDF PW.1 + PW.7 reference implementations, vendor-side claims (Snyk Code's "policy as code", GitHub Advanced Security's CodeQL config, Checkmarx One presets, Veracode policy framework), academic papers on "automated SAST configuration" or "threat-model-driven testing" 2022–2026, GitHub repos with manifest names like `threat-model-to-semgrep` or similar.
- Specific unknowns: is the loop (threat-model edit → ruleset re-derivation) novel, partially-implemented elsewhere, or fully-solved by an existing product? Is there a published OTM-or-similar → Semgrep config converter? Even null findings ("nobody has published this") are valuable signal.
- Required output: a competitive landscape verdict — direct competitors (if any), adjacent tools that could pivot into this, and a defensible answer to "why doesn't this already exist?"

## Out of scope for this research

- Deep API reference for the Semgrep CLI or GitHub Actions runner (use `get-api-docs` / `chub` if needed during architect/execute).
- Restating SLO's own architecture (the idea doc already has it).
- DAST tooling (`/slo-dast` is a separate runbook — Dastardly, ZAP, sitemap mechanics are out of scope here).
- The `/slo-rulegen` integration mechanics — that's a follow-on once `/slo-sast` is shipped; this research is for v1 only.
- Business model / pricing — SLO is an OSS skill pack.
