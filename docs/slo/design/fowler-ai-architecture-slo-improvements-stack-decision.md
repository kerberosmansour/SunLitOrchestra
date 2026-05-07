# Stack Decision — Fowler AI Architecture SLO Improvements

## Chosen Stack

- Markdown skill contracts under `skills/`.
- Markdown shared references under `skills/<skill>/references/` and `references/templates/`.
- Human-browsable mirrors under `docs/slo/templates/`.
- Rust structural-contract tests under `crates/sldo-install/tests/`.

## Reason

The existing SLO product is a host-neutral Markdown skill pack installed by `sldo-install`; the research synthesis says these improvements should become explicit contracts inside the current SLO flow rather than a separate runtime service. Keeping the Markdown + Rust-test stack preserves existing Codex, Copilot, and Claude Code install semantics while allowing deterministic tests to catch missing rows and contract drift.

## Rejected Alternatives

- **New CLI/runtime harness** — rejected because Codex and GitHub Copilot are explicitly interactive hosts today, not supported headless runtime targets.
- **ADR-only documentation** — rejected because ADRs would record decisions but would not force `/slo-plan`, `/slo-execute`, `/slo-verify`, or ticket flow to consume them.
- **Template-only update** — rejected because installed skills often load skill-local references; changing only `docs/slo/templates/` would leave skill behavior stale.

## Non-Negotiables

- Existing skill directories and `SKILL.md` names remain stable.
- No new runtime dependencies.
- No host-specific runtime promise beyond current host-capability docs.
- Structural tests must cover every new required Contract Block row or new output artifact.
