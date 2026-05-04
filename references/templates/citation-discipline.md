---
name: citation-discipline
status: stable-template
created: 2026-05-04
audience: every skill that makes factual, security-engineering, legal, financial, tool, or numeric claims
purpose: Source hierarchy and evidence rules for claims that must not rely on model memory.
---

# Citation Discipline Template

This template defines when a skill must consult an authority file or external source before making a claim. It is strict because source drift is one of the easiest ways for a skill pack to become confidently wrong.

## Locked Source Hierarchy

Use this hierarchy in order. Do not skip to a weaker source while a stronger source is available.

1. Tool vendor official documentation at a pinned version or dated page.
2. Tool repository README, CHANGELOG, or release notes at a pinned commit.
3. Upstream advisory database, standards body, or regulator documentation.
4. Named academic paper or conference talk with author and year.
5. Vendor blog post or maintainer discussion as secondary context only.
6. Never Stack Overflow, random forums, unsourced commentary, or model memory.

## Rule

Unverifiable claims are removed, not weakened. Do not write "approximately", "usually", "believed to", or "please verify" as a substitute for evidence. If a claim matters and no acceptable source exists, surface the gap and route to research or human review.

## Required Fields

When a claim depends on source material, record:

- `source_url:` for public sources, or `source_file:` for repo-local authority files.
- `retrieved:` or `last_checked:` as `YYYY-MM-DD`.
- `source_tier:` as one of `1` through `6`.
- `claim_status:` as `verified`, `conflict`, `stale`, or `gap`.
- `conflict_note:` when sources disagree.

## Quote And Paraphrase Discipline

Use short quotes only when wording is load-bearing. Prefer paraphrase plus a precise pointer. For legal, security, and tool-safety predicates, quote the minimal authoritative phrase and then explain how the skill uses it.

## Conflict Resolution

When sources conflict, prefer the higher-tier source. If two same-tier sources conflict, use the newer dated source and document the conflict. Do not silently blend conflicting claims.

## Skill Integration

SKILL.md files should point here rather than repeating the source hierarchy. Domain-specific references may add stricter rules, but they cannot weaken this hierarchy.
