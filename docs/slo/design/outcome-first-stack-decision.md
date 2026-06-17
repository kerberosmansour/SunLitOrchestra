# Stack Decision — outcome-first

## Chosen stack

No new stack. This change is delivered with the **existing SLO authoring stack**:

- **Markdown `SKILL.md` edits** to `slo-plan`, `slo-execute`, `slo-verify`,
  `slo-retro`, `slo-critique` (host-neutral skill prose).
- **Markdown template edits** to
  `docs/slo/templates/runbook-template_v_4_template.md` (new §5C + §17
  sub-sections + §11 layer additions). The v3 template is untouched (historical
  artifact).
- **Markdown doc edits** to `docs/LOOPS-ENGINEERING.md`,
  `docs/skill-pack-catalog.md`, `references/agent/operating-contract.md`.
- **One Rust structural-contract test** in `xtasks/sast-verify/tests/` (the only
  code), asserting: the new template sections exist; the edited SKILL.md files
  match their post-change SHA-256 baselines; `/slo-verify` Pass 0 is named
  "Outcome Validation". Same pattern as
  `xtasks/sast-verify/tests/sap_imp_m5_agents.rs`.

## Reason

The founder's proposal is a *methodology* change — it adds discipline, contract
sections, and gate semantics, not a runtime service. The SLO skill pack is
already a Markdown-skill + Rust-structural-test system
(`crates/sldo-install/src/install.rs` installs any `skills/<name>/SKILL.md`;
`xtasks/sast-verify` gates structural contracts). Introducing any new
language/framework would add reversibility debt with zero functional benefit.
The decision recorded in [overview](outcome-first-overview.md) — **elevate
Outcome Validation inside `/slo-verify` rather than ship a new `/slo-outcome`
skill** — further means there is no new skill directory to install, so even the
`discover_skills()` surface is untouched.

## Rejected alternatives

- **New installable `/slo-outcome` skill (new Gate 6 binary).** — Rejected:
  duplicates `/slo-verify`'s existing runtime-BDD/Playwright ownership (process
  theatre) or forces a high-churn migration of Passes 1–3; higher reversibility
  cost. Decision deferred to the founder, who chose elevate-in-place.
- **A `<slug>-outcome.slo.json` machine-readable outcome/journey schema
  companion.** — Rejected for v1: no real outcome-test fixtures exist yet;
  freezing a schema now is stable-interface debt with no consumer (same
  reasoning that deferred the measurement `.slo.json`). Promotion trigger: a
  dogfooded runbook with emitted outcome-journey fixtures.
- **Rewriting the v4 template into a "v5" with mandatory Outcome sections.** —
  Rejected: a hard break invalidates every in-flight v4 runbook. Additive
  optional sections (the §5A/§5B precedent) deliver the same rigor without the
  migration.

## Non-negotiables (downstream cannot change these without migration)

- **Additive-only template edits.** No v4 section is renumbered or removed; §5C
  is inserted after §5B, §17 gains sub-sections, §11 gains a layer row.
- **Optional-for-legacy, required-for-value-bearing.** Mirrors §5A/§5B.
- **No new dependency.** The structural test uses the crates already in
  `xtasks/sast-verify` (`serde_yaml_ng`, `sha2`, std).
- **Every edited SKILL.md keeps `cargo xtask sast-verify gate` green**, with SHA
  baselines updated in the same milestone as the edit (never waived).
