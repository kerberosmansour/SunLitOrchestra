# Lessons Backlog

Carry-forward follow-ups that could not be filed as GitHub issues at retro
time. Filing was **user-approved** (slo-threat-model M2 `/slo-retro`) but the
Claude Code harness permission classifier blocked the `gh issue create`
external write. These rows are the durable record until filed by hand or with
a Bash permission rule for `gh issue create` / `gh issue comment`.

Skill discipline note: issue filing is strictly additive — the M2 lessons,
completion, verification report, and runbook tracker are all already on disk
and complete. Only the GitHub mirror is pending.

---

## Row 1 — CEO-1 producer runbook

| Field | Value |
|---|---|
| source | slo-threat-model M2 `/slo-retro` |
| classification | slo-process |
| destination repo | `kerberosmansour/SunLitOrchestra` (origin-resolved; NO `--repo`) |
| label | `retro-derived` |
| dedupe disposition | none (3-strike `gh search` clean 2026-05-19) |
| disposition | `spilled-harness-permission-block` |
| status | pending — user to file or grant `gh issue create` permission |

**Candidate title:** Producer: /slo-architect Step 3.5 emits
`<slug>-threat-model.slo.json` (closes the slo-threat-model loop, CEO-1)

**Candidate body:**

~~~text
The slo-threat-model runbook shipped the read-side wedge:
- M1: references/security/threat-model-schema.md + a lossless dogfood fixture
  (docs/slo/design/slo-security-embedding-threat-model.slo.json, frozen
  tm-slo-sec-abuse-1..8) + xtasks/sast-verify/tests/slo_tm_m1_schema.rs.
- M2: read-side halt-not-re-derive contract wired into /slo-critique and
  /slo-verify SKILL.md (SEC-1 fence rule + degraded/hard-halt boundary),
  F-ENG-6 lockstep honored via recorded amendment, +
  xtasks/sast-verify/tests/slo_tm_m2_consumers.rs.

The CONSUMER contract is proven, but NO producer emits a .slo.json yet, so a
live sprint still gets Markdown-only from /slo-architect Step 3.5 and the
abuse-ID-drift failure stays possible until this lands. CEO-1 reconfirmed the
deferral; this issue is the durable tracker.

Scope of the producer runbook:
- Extend /slo-architect Step 3.5 to ALSO emit
  docs/slo/design/<slug>-threat-model.slo.json conforming to
  references/security/threat-model-schema.md.
- Preserve frozen tm-<slug>-abuse-N ids; supersede-don't-renumber on re-run.
- Provenance idiom = producing-skill SKILL.md git sha + input git blob shas.
- Add a live status: superseded fixture row (M1/M2 coverage note).
- Start from /slo-architect (real new producer surface), not /slo-plan.

Refs: docs/RUNBOOK-SLO-THREAT-MODEL.md, docs/slo/critique/slo-threat-model.md
(CEO-1), docs/slo/design/slo-threat-model-overview.md.
~~~

---

## Row 2 — SEC-2 redaction residual

| Field | Value |
|---|---|
| source | slo-threat-model M2 `/slo-retro` |
| classification | slo-process |
| destination repo | `kerberosmansour/SunLitOrchestra` (origin-resolved; NO `--repo`) |
| label | `retro-derived` |
| dedupe disposition | none (3-strike `gh search` clean 2026-05-19) |
| disposition | `spilled-harness-permission-block` |
| status | pending — user to file or grant `gh issue create` permission |

**Candidate title:** Enforce gitignore/redaction for confidential
threat-model `.slo.json` (SEC-2 residual)

**Candidate body:**

~~~text
Bug class V8 Data Protection (sensitive information exposure); CWE-200; ASVS
V8.3. Threat-model rows tm-slo-threat-model-abuse-4/5/6 (public-repo recon).

The read-side wedge MANDATES a per-entry classification + top-level
sensitivity field and DOCUMENTS the two-tier gitignore discipline, but it
does NOT ENFORCE that a confidential/restricted *-threat-model.slo.json is
uncommitted. A machine-readable file enumerating accepted_residual:true rows
in a public repo is a scrapeable known-unfixed-weakness list. This is an
explicitly design-accepted residual (slo-threat-model-overview.md residual #2,
reversibility doc) — not enforced in the wedge by deliberate scope choice.

The producer/redaction runbook must close it: either enforce a gitignored
tier for confidential/restricted artifacts (cf. biz-pack docs/biz vs
docs/biz-public precedent) OR emit a redacted public companion. A structural
test should fail a confidential-classified .slo.json that is git-tracked
without a redacted form.

Refs: docs/slo/design/slo-threat-model-overview.md (residual #2),
docs/slo/design/slo-threat-model-reversibility.md,
docs/slo/critique/slo-threat-model.md (SEC-2),
references/security/threat-model-schema.md.
~~~

---

## Row 3 — issue #67 progress comment (not a new issue)

| Field | Value |
|---|---|
| target | existing master issue #67 (OPEN) — "Feature: /slo-threat-model as shared threat-modeling skill" |
| action | `gh issue comment 67` |
| disposition | `spilled-harness-permission-block` |
| status | pending — user to post or grant `gh issue comment` permission |

**Candidate comment:**

~~~text
slo-threat-model read-side wedge shipped (branch slo/slo-threat-model).

- M1 done: SLO-owned threat-model JSON schema
  (references/security/threat-model-schema.md), lossless dogfood fixture with
  frozen tm-slo-sec-abuse-1..8, structural-contract test proven to bite at
  runtime.
- M2 done: read-side halt-not-re-derive contract wired into /slo-critique and
  /slo-verify (SEC-1 ~~~text fence rule, degraded/hard-halt boundary),
  F-ENG-6 lockstep honored via a recorded amendment.
- Verify M1 + M2: every BDD scenario exercised at runtime; guards forced to
  bite then restored byte-identically; Pass 4 clean; Pass 5 AI-tolerance pass;
  0 bugs.

Deferred (CEO-1 reconfirmed): the .slo.json PRODUCER (/slo-architect Step 3.5)
and SEC-2 redaction/gitignore enforcement — tracked as separate retro-derived
follow-ups. #67 stays OPEN as the umbrella for the producer slice.
~~~
