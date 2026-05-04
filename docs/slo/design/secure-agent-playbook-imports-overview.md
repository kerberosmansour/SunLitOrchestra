# Secure Agent Playbook Imports - Adaptation Overview

> **Purpose**: Capture the larger project ideas worth borrowing from OWASP Secure Agent Playbook, and explain how they should be adapted to SunLitOrchestra without changing SLO's identity.
> **Source reviewed**: `/Users/sherifmansour/Dev/GitHub/secure-agent-playbook` project layout, README, `agents/`, `skills/`, `plays/`, `templates/`, `examples/`, plugin metadata, and release workflow.
> **Related tracker**: [docs/slo/future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../future/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md).

## Executive summary

Secure Agent Playbook is strongest as a security-assessment playbook: it separates specialist agents, short skills, detailed plays, reusable templates, standards data, and example outputs. SunLitOrchestra is strongest as a runbook-driven delivery workflow: it turns an idea into scoped, reviewed, verified, shippable work with security woven through every stage.

The useful borrowing path is therefore not "make SLO into Secure Agent Playbook." The useful path is to import the structural ideas that make security work repeatable:

- Standardized evidence-rich security findings.
- Thin skill entrypoints backed by deeper methodology references.
- Synthetic example outputs that show what good artifacts look like.
- Traceability from findings to threat-model rows, CWE, OWASP, ASVS, and optionally OpenCRE.
- Optional specialist roles where the host can support them cleanly.
- Optional plugin packaging as an additional distribution channel, not a replacement for `sldo-install`.

The first item has already landed as `references/security/` and is wired into `/slo-critique` and `/slo-verify`.

## Borrowing principles

Borrow structure, not content. Secure Agent Playbook's security procedures are authored for assessment tasks. SLO should re-author any imported pattern in SLO language: runbooks, milestones, evidence logs, critique findings, verification reports, and ship-ready PRs.

Keep SLO's product center. SLO is a delivery workflow with embedded security, not a standalone security review toolkit. Security assessment should remain a disciplined thread through the sprint loop rather than becoming the whole product.

Prefer install-neutral artifacts. SLO supports Claude Code and GitHub Copilot through the `SKILL.md` contract and Rust installer. Any borrowed plugin or agent packaging must be optional until host support is explicit.

Keep deterministic checks deterministic. Where Secure Agent Playbook relies on agent procedure, SLO should enforce machine-checkable parts with tests, templates, or Rust tooling when the behavior affects safety.

## Idea 1: Standard security finding and report templates

### What Secure Agent Playbook does well

It gives every security finding a predictable shape: severity, CWE, OpenCRE, OWASP reference, location, impact, evidence, remediation, and confidence. This makes outputs easier to compare across tools and reviewers.

### Why SLO should borrow it

SLO already requires concrete scenarios in `/slo-critique` and runtime evidence in `/slo-verify`, but compact table rows can hide the reasoning behind a serious security issue. Expanded findings are especially useful when a finding crosses multiple surfaces: threat model, runbook milestone, source file, dependency scan, and verification command.

### SLO adaptation

Landed:

- `references/security/security-finding-template.md`
- `references/security/security-assessment-summary-template.md`
- `/slo-critique` points security persona findings at the expanded template when a row is not enough.
- `/slo-verify` Pass 4 points scanner findings at the same template.

Next useful extensions:

- `/slo-sast` should use the summary template when reporting coverage gaps.
- `/slo-ruleverify` should use the finding template when a generated rule fails clean-tree or coverage gates.
- PR bodies from `/slo-ship` could link to security assessment summaries when a runbook introduced a new public surface.

### Guardrails

Do not require every minor row to use the long template. The compact table stays useful as an index. Use the expanded template when evidence, standards mapping, remediation, or residual risk would otherwise be squeezed into one unreadable cell.

## Idea 2: Thin skills backed by deeper play/procedure files

### What Secure Agent Playbook does well

Its skills are short activation wrappers, while detailed procedures live in `plays/`. This keeps the installable skill readable and lets contributors edit the methodology without bloating the entrypoint.

### Why SLO should borrow it

Some SLO skills, especially `/slo-sast`, `/slo-tla`, and `/slo-plan`, are long because they carry both activation logic and full methodology. This is good for precision but bad for scanability and future maintenance.

### SLO adaptation

Continue the existing future runbook:

- [RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md](../future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md)

The likely target structure:

```text
skills/slo-sast/
  SKILL.md                         # short entrypoint, preflight, output contract
  references/
    methodology-parser.md
    methodology-stack-detection.md
    methodology-rule-selection.md
    methodology-emission.md
    methodology-rederivation.md

skills/slo-plan/
  SKILL.md
  references/
    methodology-milestone-authoring.md
    runbook-template_v_4_template.md
```

### Guardrails

Do not create a new top-level `plays/` directory unless it clearly beats the existing `skills/<skill>/references/` pattern. SLO's current convention is skill-local references for skill-specific procedure and root `references/` for shared policy.

## Idea 3: Example output gallery

### What Secure Agent Playbook does well

It has `examples/` to show realistic outputs. This reduces ambiguity for users and contributors: they can see what "good" looks like before running the tool.

### Why SLO should borrow it

SLO asks users to trust a chain of artifacts: idea docs, research dossiers, architecture docs, threat models, runbooks, critique reports, verification reports, retros, and PR bodies. The README now describes those artifacts, but examples would make the contract tangible.

### SLO adaptation

Add an `examples/` directory with synthetic outputs:

```text
examples/
  README.md
  runbook-excerpt.md
  critique-report.md
  verification-report.md
  security-finding.md
  sast-manifest.json
  biz-public-artifact.md
```

Each example should be:

- Synthetic.
- Small enough to read in a few minutes.
- Explicitly non-normative when it abbreviates a real template.
- Free of personal data, secrets, real customer names, and confidential `docs/biz/` content.

### Priority

High. This is one of the highest-leverage improvements because it helps both users and agents calibrate expected output quality.

## Idea 4: Standards traceability

### What Secure Agent Playbook does well

It systematically maps security findings to CWE, OWASP references, ASVS, and OpenCRE. That gives readers a way to connect a concrete bug to a broader requirement family.

### Why SLO should borrow it

SLO already cites threat-model rows and CWE classes in SAST contexts. But the traceability is uneven across `/slo-critique`, `/slo-verify`, `/slo-sast`, and `/slo-rulegen`. A consistent minimum standard would make security findings easier to review and turn into follow-up rules.

### SLO adaptation

Use a tiered requirement:

| Output | Required mapping | Optional mapping |
|---|---|---|
| `/slo-critique` security row | threat-model row or explicit `N/A`, local bug class | CWE, OWASP, ASVS, OpenCRE |
| `/slo-verify` Pass 4 scanner finding | tool finding id / package / rule id, evidence | CWE, CVE, GHSA, OWASP, OpenCRE |
| `/slo-sast` coverage gap | CWE claimed vs covered | OWASP / ASVS rationale |
| `/slo-rulegen` generated rule | CWE and variation family | OpenCRE and ASVS where available |

### Guardrails

Do not make online OpenCRE lookup mandatory in normal skill execution until offline behavior and source freshness are designed. A missing OpenCRE mapping should be a coverage gap, not a blocker.

## Idea 5: Specialist agents and team lead

### What Secure Agent Playbook does well

It defines specialist agents such as code security reviewer, dependency auditor, API reviewer, AI assessor, and a team lead that scopes, dispatches, deduplicates, and consolidates findings.

### Why SLO should be careful

SLO already has persona rotation inside `/slo-critique`: CEO, engineering lead, security, and design. That keeps the review portable across hosts. Native `agents/` could be useful, but only if the host can install and invoke them consistently.

### SLO adaptation

Near term:

- Keep `/slo-critique` persona rotation as the canonical portable path.
- Improve persona outputs using shared templates and examples.

Later experiment:

```text
agents/
  slo-runbook-review-lead.md
  slo-security-reviewer.md
  slo-design-reviewer.md
  slo-verification-lead.md
```

The lead agent would scope a runbook, dispatch specialists, dedupe findings, and write one `docs/slo/critique/<slug>.md`.

### Guardrails

Do not add `agents/` until:

- The install path is explicit for at least Claude Code.
- GitHub Copilot behavior is documented as supported, unsupported, or N/A.
- Agent outputs still land in the same durable `docs/slo/` artifacts.
- No hidden agent-only workflow bypasses the runbook contract.

## Idea 6: Optional plugin packaging and release zip

### What Secure Agent Playbook does well

It ships `.claude-plugin/plugin.json` and a release workflow that packages the plugin directory correctly for Claude Code organization installation.

### Why SLO might borrow it

SLO currently has a stronger local installer story through `sldo-install`, including Claude Code and GitHub Copilot. But a Claude plugin zip could lower friction for Claude-only users and organizations.

### SLO adaptation

Treat plugin packaging as an additional distribution channel:

- Add `.claude-plugin/plugin.json` only if it can point at the existing `skills/` tree without duplicating source.
- Add a release workflow only after applying SLO's action SHA-pinning discipline.
- Keep `sldo-install` as the canonical installer for multi-host support.
- Make README wording explicit: "Claude plugin zip is optional; Rust installer remains canonical."

### Guardrails

Do not introduce an unpinned GitHub Actions workflow. Do not make Claude plugin packaging the source of truth for skill inventory. `docs/skill-pack-catalog.md` remains canonical.

## Idea 7: Large standards data directories

### What Secure Agent Playbook does well

It vendors many standards references into `data/`, making plays more self-contained.

### Why SLO should not rush this

SLO already has carefully scoped reference directories with retrieval-date discipline. Vendoring broad standards data creates maintenance burden, freshness risk, and licensing review work. It is only worth doing when a skill consumes the data directly.

### SLO adaptation

Prefer:

- Small curated references under `references/security/`, `references/sast/`, or skill-local `references/`.
- Retrieval dates on time-sensitive references.
- Source hierarchy rules for security-engineering claims.

Defer:

- Bulk ASVS / WSTG / SAMM / OpenCRE mirrors.
- Auto-generated standards data unless a parser or report generator consumes it.

## Idea 8: "Use existing tools" as a stronger project norm

### What Secure Agent Playbook does well

It explicitly tells contributors to prefer proven tools such as Semgrep, Trivy, OSV Scanner, and TruffleHog over reimplementing detection.

### Why SLO should borrow it

SLO already uses Semgrep and `cargo audit` / `cargo deny` guidance. The norm is present, but it could be more explicit across future security skills.

### SLO adaptation

Add this to future skill-authoring guidance:

- Use established scanners for detection.
- Use SLO skills to orchestrate, scope, gate, and interpret.
- Write custom rules only when the existing tools cannot express the project-specific invariant.

This fits `/slo-rulegen`: it does not replace Semgrep; it authors rules and gates them.

## Suggested implementation order

1. **Finish shared security reporting integration.** Extend `references/security/` usage to `/slo-sast`, `/slo-rulegen`, and `/slo-ruleverify`.
2. **Add example output gallery.** Make the expected artifact chain visible to users and future agents.
3. **Continue skill decomposition.** Thin long skills into entrypoints plus methodology references.
4. **Design standards traceability.** Decide which mappings are required per output type and how offline lookup works.
5. **Assess optional Claude plugin packaging.** Keep it additive to `sldo-install`.
6. **Experiment with host-native agents.** Only after install semantics and output contracts are clear.
7. **Consider standards data vendoring.** Only if a concrete skill needs it.

## Non-goals

- Do not turn SLO into a general-purpose security assessment pack.
- Do not duplicate Secure Agent Playbook's plays.
- Do not introduce a Claude-only path that bypasses the current multi-host story.
- Do not make external standards lookup mandatory in offline workflows.
- Do not add broad standards data without a consuming skill and maintenance plan.

## Open questions

- Should expanded security findings be mandatory for all `high` and `critical` findings?
- Should `/slo-ship` include a security summary section when a milestone introduced new public surface?
- Should example outputs live at repo root `examples/` or under `docs/examples/`?
- Should OpenCRE mapping be captured manually, generated from a local curated table, or fetched live when available?
- Would Claude plugin packaging help enough to justify maintaining two install paths?
- Can host-native agents be represented in a way that does not make GitHub Copilot support second-class?

