---
name: slo-sec-libs
researched: 2026-04-27
incomplete: false
note: |
  Most of the research for this runbook was completed in Phase 1 of the
  slo-security-embedding program. See `docs/research/slo-security-embedding/dossier.md`
  and `docs/research/slo-security-embedding/synthesis.md` for the upstream research
  (Q1-Q5 findings on CycloneDX 1.6, Hulumi capability advertising, GitHub rate-limit
  point costs, etc.). This synthesis compresses the research-Q findings into the
  design constraints for /slo-sec-libs.
---

# Synthesis — /slo-sec-libs

## What the design must handle (and why)

### 1. CycloneDX 1.6+ `declarations` is the capability-advertising contract

The design must handle CycloneDX 1.6 declarations as the input format because the upstream Phase 1 research (Q1) confirmed it as the canonical vendor-neutral capability-advertising spec. URL: https://cyclonedx.org/docs/1.6/json/. The schema is JSON, validation is jsonschema-shaped.

### 2. Rust ecosystem has no 1.6+ declarations support — Python subprocess is the path

The design must handle the Python-jsonschema subprocess path because the upstream Phase 1 research (Q2) confirmed `cyclonedx-bom 0.8.1` is spec-1.5 only (no 1.6+ `declarations` reader). Rather than emit Rust CycloneDX work in this phase, /slo-sec-libs reads through a Python subprocess — matches the Phase 2 precedent for SecOpsTM integration.

### 3. SLO-owned intake repo as the default filing destination

The design must handle the default-channel decision because filing capability gaps to an SLO-owned repo (`kerberosmansour/slo-security-intake`) is the path of least resistance: same-owner authentication, no cross-org permissions to manage, and the user controls the issue-template shape. Third-party filing is gated explicitly — `--file-upstream` flag + per-session 40-issues/hr cap.

### 4. GitHub secondary rate-limit point costs are undocumented — defensive cap is the answer

The design must handle the rate-limit cap because the upstream Phase 1 research (Q5) found GitHub's per-endpoint secondary-rate-limit point cost is undocumented and varies. The 40-issues/hr cap is a defensive posture borrowed from public Octokit benchmarks; conservative enough that even if a single endpoint costs 100 points per call, a session stays under the 1000 points/min visible ceiling.

### 5. Crypto-primitive parametric claims ride in `properties` namespace

The design must handle the namespace decision because the upstream Phase 1 research showed CycloneDX Property Taxonomy doesn't yet have crypto-primitive parametrics (Argon2id iterations / memory / parallelism). Vendored namespace `cdx:sunlit:crypto:*` lives in `properties` until upstream contribution lands. M5 of this runbook flags the upstream contribution as a deferred follow-up.

### 6. argv-list discipline applies to every subprocess

The design must handle this — same as `/slo-sast` M5's `gh pr create` rule, same as Phase 1's argv-list-only contract. Never shell-string interpolation. Defends against `tm-scanner-orchestration-abuse-2 / SEC-6` (argv injection via tampered `.git/config`).

### 7. NO `--repo` flag on `gh pr create` (confused-deputy defense)

The design must handle this per [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4) `SEC-8`. Same pattern as `/slo-sast` M5. The skill targets only the current repo's origin remote.

### 8. Capability-gap record schema must be regex-validated, not free-text

The design must handle this because gap records flow into the upstream intake repo as issue-bodies that downstream consumers (humans + future tooling) parse. Free-text gap records become unparseable; regex-validated fields make the gap a structured artifact.

### 9. Pre-requisite repos and CycloneDX declarations must exist before runbook starts

The design must handle the pre-requisite gates explicitly because `/slo-sec-libs` is unusable until:

- `kerberosmansour/slo-security-intake` exists with `ISSUE_TEMPLATE` populated.
- `kerberosmansour/hulumi` and `SunLitSecureLibraries` repos publish CycloneDX 1.6 `declarations` JSON files.
- Contributor `gh` CLI is authenticated with the right scopes.

These are out-of-band of the runbook (per [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)) — the runbook's Background Context section flags them as required pre-flight before M1.

## Open questions that research did not answer

1. **`gh search issues` reliability for de-dupe** — same question as R1 (loops), needs hands-on testing on a populated set. Spike step in M3.
2. **Capability-matcher conflict resolution** — when two candidate libraries both claim to cover a requirement, what's the tiebreaker? Lean: prefer the one with more parametric claims (more specific advertising). Confirm in M2.
3. **Stale-declarations warning** — when a target repo's CycloneDX file is > N months old, warn or refuse? Lean: warn at 6 months, refuse at 12 — matches the cost-baseline pattern.
4. **Apalache-style SHA pin for CycloneDX schema** — should /slo-sec-libs pin a specific schema version SHA, or float to whatever 1.6+ is current? Lean: pin per runbook, document the bump procedure. Confirm in M1.

## Source pointers

- [Issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4) — Phase 4 runbook entry point + deferred follow-ups from Phase 1
- [`docs/idea/slo-security-embedding.md`](../../idea/slo-security-embedding.md) — original Phase 1 idea doc with "The next product" section that promoted /slo-sec-libs from stretch to core
- [`docs/research/slo-security-embedding/dossier.md`](../slo-security-embedding/dossier.md) and [`docs/research/slo-security-embedding/synthesis.md`](../slo-security-embedding/synthesis.md) — upstream Q1-Q5 research findings
- [`docs/design/slo-security-embedding-threat-model.md`](../../design/slo-security-embedding-threat-model.md) — threat model for the upstream program; this runbook adds the `/slo-sec-libs` surface
- [`SECURITY.md`](../../../SECURITY.md) — argv-list discipline, no-`--repo` rule, etc.

## Note on chub / get-api-docs

Use [`/get-api-docs`](../../../skills/get-api-docs/SKILL.md) for any third-party API reference encountered during execution (e.g., GitHub API specifics for `gh search` queries, Octokit secondary-rate-limit commentary). The skill itself is `WebFetch`-denied like the rest of the security skills; external claims captured at runbook-author time only, with `last_checked:` dates.
