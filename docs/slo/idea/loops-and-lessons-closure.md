---
name: loops-and-lessons-closure
created: 2026-04-27
status: ideation (preempted — skill-pack improvement, not greenfield)
tla_required: false
---

# Loops + lessons closure

## The pain

A developer (any future contributor or the project owner himself) lands on the SLO repo, sees ARCHITECTURE.md (static structure) and 32 SKILL.md files (individual moves), but cannot answer the question "**how does this skill pack improve itself?**" — because the cyclic feedback structures are implicit. Two concrete symptoms:

- **Forward dropout in lessons**: Lessons land in `docs/slo/lessons/<prefix>-m<N>.md` after every milestone, but the next milestone rarely re-reads the prior lessons. The same mistakes repeat. Last week, the project owner manually re-read the M3 lessons before kicking off M4 and caught a naming-convention drift the agent would have repeated. That manual step shouldn't be necessary.
- **Lateral dropout in upstream signal**: When `/slo-execute` discovers a bug in an upstream tool (Semgrep, Playwright, `cargo audit`), the lesson sits in markdown. It never gets filed upstream. A library-feedback loop that *would* improve the tool dies in a local file.

## Five capabilities the user described without realizing

- A first-class document per loop (engineering / business) that newcomers and a freshly-loaded Claude can read in 90 seconds.
- A `/slo-retro` extension that, after writing the lessons file, classifies each lesson (product / upstream-OSS / SLO-process) and files an issue in the appropriate destination repo with confirmation.
- A `/slo-execute` pre-flight step that surfaces open prior-retro issues as scope candidates for the current milestone.
- A v3 runbook template addition: a "Carry-forward from prior retros" section, so the loop is visible in the runbook artifact, not just in the skill flow.
- A `LESSONS-BACKLOG.md` fallback for repos without a configured issue tracker — the loop must work locally too.

## Top risks

- **Breach**: low — this work doesn't introduce any new attack surface beyond filing issues to GitHub via `gh`. Risk: an upstream `gh search` query that exposes the founder's PII via a tampered git config. Surface: confused-deputy via `.git/config` — already covered by [`SECURITY.md`](../../SECURITY.md) "argv-list discipline" and [issue #4's `--repo` ban](https://github.com/kerberosmansour/SunLitOrchestra/issues/4).
- **Compliance fine**: not applicable — no personal data flows through this work; lesson content is engineering reflection, not founder PII.
- **Prolonged outage**: low — `/slo-retro` has no runtime dependency on issue filing; if `gh` is unavailable, the skill falls back to `LESSONS-BACKLOG.md` and the loop closes locally. The retro skill itself never fails because of issue-filing trouble.

## Approach A — conservative

- **Effort**: 3 person-days (mostly markdown).
- **Wedge**: M1 = `LOOPS-ENGINEERING.md`. Author the doc, cross-link from `ARCHITECTURE.md` and the implicated SKILL.md files. The loops doc is itself the artifact people read; everything downstream depends on it being readable.
- **Risks**: documentation-only milestone might feel like cosmetic; resist the temptation to over-elaborate.

## Approach B — cloud / SaaS

Not applicable — this is documentation + skill extension; no SaaS/cloud component.

## Approach C — local / desktop

Not applicable — no UI.

## Recommendation

Approach A. The runbook is small (4 milestones), mostly markdown, and serves as the **vehicle for the lessons-closure mechanic** that R2 and R3 will rely on. Ship this first. M1 LOOPS-ENGINEERING.md, M2 LOOPS-BUSINESS.md, M3 `/slo-retro` extension to file issues, M4 `/slo-execute` pre-flight loop closure + runbook template update.

## Open questions for /slo-research

1. Marker for retro-originated issues — title prefix vs label vs body sentinel? Check `gh search` reliability against each option.
2. Fallback file format — single `LESSONS-BACKLOG.md` table, or one file per lesson? Lean: single file with append-only rows.
3. Should the runbook template's "Carry-forward from prior retros" section be required or optional? Lean: optional, with prose guidance; required-fields drive false-positive `N/A` rows.
4. Mapping upstream tools to their issue-tracker repo: parse `Cargo.toml` / `package.json` and resolve via crates.io / npm registry, or require explicit per-repo mapping? Lean: explicit mapping in `.sldo/upstream-mapping.toml` with crates.io fallback.
