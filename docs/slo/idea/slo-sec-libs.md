---
name: slo-sec-libs
created: 2026-04-27
status: ideation (preempted — issue #4 already contains the bulk of the framing; this idea doc compresses it)
tla_required: false
---

# /slo-sec-libs — stack-aware library recommender + upstream feedback loop

## The pain

[Phase 1 of the security-embedding program (`slo-security-embedding`)](https://github.com/kerberosmansour/SunLitOrchestrate/pull/3) made `/slo-ideate`, `/slo-architect`, `/slo-plan`, `/slo-critique`, and `/slo-verify` security-aware: every milestone's Contract Block now declares Data classification + Proactive controls + Abuse acceptance scenarios. The threat model and SECURITY.md are dogfooded.

The gap: when a milestone declares "this surface needs an Argon2id password hasher", the agent has no source-of-truth for *which* library covers that requirement at the runbook's pinned version. It defaults to whatever lib the model recalls — exactly the failure mode `/slo-research` exists to prevent.

Hulumi (Pulumi components for AWS) and SunLitSecurityLibraries (Rust security crates) are the public declaration-source repos for this workflow. SunLitSecurityLibraries already advertises capabilities via CycloneDX 1.6+ `declarations`; Hulumi still needs its declaration artifact landed on `main`; no SLO skill reads either source yet.

Concrete failure: a runbook with `security_libs_required: true` and a "secure JSON deserialization" requirement gets a paraphrased `serde_json` recommendation when SunLitSecurityLibraries' `secure_boundary::SecureJson` (which advertises `C5` proactive control with parametric configuration) would be the right call.

A secondary failure: when the recommender finds **no** library that covers a requirement, today the gap dies as a comment in a lessons file. A capability-gap issue should be filed against the library owner's intake repo so the gap is tracked as work, not lost as text.

## Five capabilities the user described without realizing

- A `/slo-sec-libs` skill that reads `ARCHITECTURE.md` + `stack-decision.md` + the runbook's per-milestone proactive-control requirements.
- A CycloneDX 1.6+ `declarations` reader (Python jsonschema subprocess path, since Rust ecosystem has no 1.6+ declarations support per the Phase 1 research synthesis Q1 findings).
- A capability-matcher that returns either (a) a specific library component covering the requirement, or (b) "no match" with a structured capability-gap record.
- An SLO-owned intake repo (`kerberosmansour/slo-security-intake`) as the default capability-gap filing destination.
- A gated upstream-filing path: `--file-upstream` flag + client-side cap of 40 issues/hr per session (defensive against GitHub's undocumented per-endpoint secondary-rate-limit point cost — see [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4) for the research-Q5 anchor).

## Top risks

- **Breach**: medium-low — the upstream filing path exfiltrates capability-gap content to a remote (GitHub). Surface: `gh pr create` argv-list discipline (already standard); confused-deputy via tampered `.git/config` mitigated by no `--repo` flag (per issue #4 SEC-8). Rate-limit cap protects against runaway filing storms.
- **Compliance fine**: not applicable — no personal data flows; capability-gap records describe library capability deltas, not user content.
- **Prolonged outage**: low — `/slo-sec-libs` failures degrade gracefully: if the declarations file is missing, the recommender returns "no match"; if `gh` is unavailable, the file falls back to local `LESSONS-BACKLOG.md` (matches R1's lessons-fallback pattern).

## Approach A — conservative (recommended)

- **Effort**: 8 person-days (the declarations reader is well-defined per Phase 1 synthesis; the matcher is the bulk; the filer reuses #4's argv-list discipline).
- **Wedge**: M1 = declarations reader (Python jsonschema subprocess), produces a structured catalog of advertised capabilities. Second wedge M2 = matcher (compares runbook proactive-control requirements against the catalog). Both M1 and M2 are pure-read operations — no GitHub side effects.
- **Risks**: SecOpsTM install footprint for the (parked) `/slo-threat-model` skill is too heavy; that's why `/slo-sec-libs` is reader-side only — no Rust CycloneDX emitter work in this phase, no SecOpsTM dependency.

## Approach B — cloud / SaaS

Not applicable for v1; potential future SaaS distribution of declarations index is out of scope.

## Approach C — local / desktop

Not applicable.

## Recommendation

Approach A. 5 milestones. M1 declarations reader; M2 capability matcher; M3 SLO-intake filer (default); M4 third-party filing gate + rate limit; M5 dogfood against this SLO repo (re-critique an existing milestone using `/slo-sec-libs` to recommend libraries for its proactive-controls row). Detailed milestone breakdown in [`docs/slo/future/RUNBOOK-SLO-SEC-LIBS.md`](../future/RUNBOOK-SLO-SEC-LIBS.md).

The runbook depends on three one-time pre-requisites (per [issue #4](https://github.com/kerberosmansour/SunLitOrchestrate/issues/4)):

1. Create `kerberosmansour/slo-security-intake` repo with `ISSUE_TEMPLATE` populated.
2. Add CycloneDX 1.6 `declarations` JSON to `kerberosmansour/hulumi`; `kerberosmansour/SunLitSecurityLibraries` already has `declarations/cyclonedx-1.6-capabilities.json` on `main`.
3. Confirm `gh` CLI scopes (`repo` or `public_repo`) on contributor machines.

These prereqs are out-of-band of this runbook — flagged in the runbook's Background Context section so the author can confirm them before M1.

## Open questions for /slo-research

(Most resolved by [`docs/slo/research/slo-security-embedding/`](../research/slo-security-embedding/) Phase 1 synthesis; flagged for completeness.)

1. CycloneDX 1.6 `declarations` schema — current spec at https://cyclonedx.org/docs/1.6/json/. Verify against the pinned schema URL at runbook-author time and capture the SHA.
2. CycloneDX Property Taxonomy contribution path — the project's vendored `cdx:sunlit:crypto:*` namespace per Phase 1 synthesis. Open-source contribution channel: https://github.com/CycloneDX/cyclonedx-property-taxonomy. Verify upstream maintainership cadence.
3. GitHub secondary-rate-limit point costs per endpoint — undocumented; the 40 issues/hr defensive cap derives from public Octokit benchmarks. Cite the specific public commentary at runbook-author time.
4. `gh search issues` reliability for de-dupe (matches R1's same question) — does title-prefix matching, label matching, or body-sentinel matching produce the lowest false-positive rate? Spot-check before M3.
