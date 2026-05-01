# SunLitOrchestrate examples

> **Synthetic, non-normative — not for direct use.**
>
> Every artifact in this directory is a small abbreviation of a real SLO output. Names, IDs, paths, and findings are invented. Do not copy these files into a real project as templates — read the canonical sources they cite.

## Why this directory exists

SLO chains many artifacts together — idea docs, research dossiers, architecture docs, threat models, runbooks, critique reports, verification reports, retrospectives, and PR bodies. The README and CLAUDE.md describe the chain; this directory *shows* it.

Each example is:

- **Synthetic.** Frontmatter declares `synthetic: true`. PII patterns (email, UK NI, sort code, US SSN, EU IBAN) are forbidden by a structural-contract test in `xtasks/sast-verify/tests/sap_imp_m2_examples.rs`.
- **Non-normative.** Frontmatter declares `non-normative: true`. The canonical contract lives in the source the example abbreviates.
- **Small.** Hard cap: 10 KB per file (enforced by the structural-contract test). The cap is the discipline — these abbreviate; they do not replicate.
- **Linked back.** Frontmatter `abbreviates:` field points at the canonical template, skill, or runbook.

## Files

| Artifact | Abbreviates |
|---|---|
| [runbook-excerpt.md](runbook-excerpt.md) | `docs/slo/templates/runbook-template_v_4_template.md` |
| [critique-report.md](critique-report.md) | `skills/slo-critique` |
| [verification-report.md](verification-report.md) | `skills/slo-verify` |
| [security-finding.md](security-finding.md) | `references/security/security-finding-template.md` |
| [sast-manifest.json](sast-manifest.json) | `skills/slo-sast` (M4 manifest schema) |
| [biz-public-artifact.md](biz-public-artifact.md) | `skills/slo-gtm` |

## What this is NOT

- **Not a template.** Use the canonical templates under `docs/slo/templates/` or `references/`.
- **Not installed.** `sldo-install` walks `skills/<name>/SKILL.md` only — `examples/` is git-tracked but not installable.
- **Not for production data.** Per the structural-contract test, no SKILL.md may link to `examples/` (Out-of-Scope rule). Skills are the contract; examples are the gallery.
